use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::process::Command;
use tokio::sync::Mutex;
use tokio::fs::File;

const M2_ADDR: &str = "192.168.1.5:5000"; // Machine 2 IP
const CHUNK_SIZE: usize = 4096; // 4 KB chunks for fast memory fetch
const PREFETCH_SIZE: usize = 256 * 1024 * 1024; // 256 MB prefetch buffer
const SHARED_MEMORY_PATH: &str = "D:/AI/VM/RSON/m1_vm_host/m2_shared_memory.img"; // Memory file

// Fetch memory chunk from Machine 2
async fn fetch_remote_memory(offset: usize, size: usize) -> Vec<u8> {
    for attempt in 1..=3 {
        match TcpStream::connect(M2_ADDR).await {
            Ok(mut stream) => {
                println!(
                    "Connected to M2 (attempt {}). Requesting: offset={} size={}...",
                    attempt, offset, size
                );

                let mut buffer = vec![0u8; size];
                let mut request = [0u8; 16];
                request[..8].copy_from_slice(&offset.to_le_bytes());
                request[8..].copy_from_slice(&size.to_le_bytes());

                if let Err(e) = stream.write_all(&request).await {
                    eprintln!("Failed to send memory request: {}", e);
                    continue;
                }

                if let Err(e) = stream.read_exact(&mut buffer).await {
                    eprintln!("Failed to receive memory: {}", e);
                    continue;
                }
                println!("Received {} bytes from M2", buffer.len());
                return buffer;
            }
            Err(e) => eprintln!("Connection failed (attempt {}): {}", attempt, e),
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    panic!("All attempts to connect to M2 failed!");
}

// Write shared memory to file for QEMU
async fn write_to_shared_file(data: &[u8]) {
    let mut file = File::create(SHARED_MEMORY_PATH).await.expect("Failed to create memory file");
    file.write_all(data).await.expect("Failed to write to memory file");
    println!("Shared memory file updated: {}", SHARED_MEMORY_PATH);
}

// Prefetch memory chunks and write to shared memory file
async fn prefetch_memory(cache: Arc<Mutex<Vec<u8>>>) {
    loop {
        let new_data = fetch_remote_memory(0, PREFETCH_SIZE).await;
        {
            let mut cache_lock = cache.lock().await;
            cache_lock.copy_from_slice(&new_data);
        }
        write_to_shared_file(&new_data).await;

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

async fn start_qemu_vm() {
    println!("Starting QEMU VM with shared RAM...");

    // Total memory = 8G (M1) + 32G (M2) = 40G
    let total_memory = "37G";
    let local_memory = "5G";
    let shared_memory = "32G";

    let shared_mem_backend = format!("memory-backend-ram,id=mem2,size={},share=on", shared_memory);
    let local_mem_backend = format!("memory-backend-ram,id=mem1,size={}", local_memory);

    let qemu_args = vec![
        "-m", total_memory,                 // Total memory (M1 + M2)
        "-cpu", "qemu64",                   // CPU type
        "-smp", "4",                        // 4 CPU cores
        "-hda", "D:/AI/VM/RSON/m1_vm_host/vm_img/ubuntu.qcow2",
        "-net", "nic",

        "-net", "user,hostfwd=tcp::2222-:22",
        //"-boot", "menu=on",
        // Define both memory regions
        "-object", &local_mem_backend,      // Local memory (M1)
        "-object", &shared_mem_backend,     // Shared memory (M2)

        // Map both memory regions to NUMA nodes
        "-numa", "node,memdev=mem1",        // Local NUMA (M1)
        "-numa", "node,memdev=mem2",        // Shared NUMA (M2)
    ];

    let output = Command::new("qemu-system-x86_64")
        .args(qemu_args)
        .output()
        .await;

    match output {
        Ok(out) => {
            println!("QEMU started successfully.");
            println!("QEMU Output: {}", String::from_utf8_lossy(&out.stdout));
            println!("QEMU Error: {}", String::from_utf8_lossy(&out.stderr));
        }
        Err(e) => panic!("Failed to start QEMU: {}", e),
    }
}


#[tokio::main]
async fn main() {
    println!("Starting optimized QEMU VM...");

    let cache = Arc::new(Mutex::new(vec![0u8; PREFETCH_SIZE]));

    let cache_clone = Arc::clone(&cache);
    tokio::spawn(async move {
        println!("Memory prefetching started...");
        prefetch_memory(cache_clone).await;
    });

    let initial_chunk = fetch_remote_memory(0, CHUNK_SIZE).await;
    write_to_shared_file(&initial_chunk).await;
    println!("Preloaded memory: {} bytes", initial_chunk.len());

    start_qemu_vm().await;
}
