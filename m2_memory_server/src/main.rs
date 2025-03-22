use lazy_static::lazy_static;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::task;

// 32 GB shared memory buffer
const MEMORY_SIZE: usize = 32 * 1024 * 1024 * 1024;

lazy_static! {
    static ref MEMORY: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![0u8; MEMORY_SIZE]));
}

// Handle memory request from M1
async fn handle_client(mut stream: TcpStream, mem_clone: Arc<Mutex<Vec<u8>>>) {
    let mut buf = [0u8; 16]; // Buffer for offset and size
    loop {
        // Read offset and size (16 bytes: 8 for offset + 8 for size)
        match stream.read_exact(&mut buf).await {
            Ok(_) => {
                let offset = u64::from_le_bytes(buf[..8].try_into().unwrap()) as usize;
                let size = u64::from_le_bytes(buf[8..].try_into().unwrap()) as usize;

                println!("Received memory request: offset={} size={}", offset, size);

                // Validate request boundaries
                if offset + size > MEMORY_SIZE {
                    eprintln!("Invalid memory range: offset={} size={}", offset, size);
                    continue; // Ignore invalid requests
                }

                // Copy memory chunk without blocking the async context
                let data = {
                    let memory = mem_clone.lock().unwrap();
                    memory[offset..offset + size].to_vec()
                };

                // Send the requested memory chunk
                if let Err(e) = stream.write_all(&data).await {
                    eprintln!("Error sending data: {}", e);
                    break;
                }

                println!("Sent {} bytes to M1", size);
            }
            Err(e) => {
                eprintln!("Error reading request: {}", e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:5000".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind memory server");

    println!("Memory server running on {}", addr);

    let mem_clone = MEMORY.clone();
    while let Ok((stream, addr)) = listener.accept().await {
        println!("New connection from: {:?}", addr);

        let mem_clone = mem_clone.clone();
        task::spawn(async move {
            handle_client(stream, mem_clone).await;
        });
    }
}
