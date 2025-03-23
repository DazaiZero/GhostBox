# GhostBox

> **Reviving old machines—One unified AI powerhouse.**

GhostBox is an open-source project that creates a **Unified Virtual Machine (VM)** by pooling **RAM and GPU resources** across multiple machines on a network. It is designed to tackle **AI workloads** by combining the capabilities of older devices using **NUMA nodes**, enabling large language model (LLM) inference on distributed hardware.

## Why GhostBox?

Modern AI models demand substantial computing power, making them inaccessible for many. GhostBox changes this by allowing you to:

- **Run Large AI Models**: Execute resource-intensive LLMs by combining memory and GPU power from multiple devices.
- **Reuse Old Machines**: Leverage the hardware from older devices (e.g., laptops with GPUs, desktops with large RAM).
- **Unified AI Infrastructure**: Share RAM and GPU across the network to create a single, powerful virtual machine.
- **Cost-Effective AI**: Avoid expensive hardware upgrades by pooling your existing resources.

## How It Works

GhostBox uses a **TCP-based memory sharing system** to dynamically pool resources from multiple machines. While the project initially explored **RDMA** for better performance, the current implementation is compatible with any hardware via standard networking.

### System Overview:

1. **Machine 1 (m1_vm_host)**: Hosts the QEMU-based Linux VM and manages GPU if available.
2. **Machine 2 (m2_memory_server)**: Provides additional RAM over TCP for the unified VM.

### Resource Sharing Examples:

- **Machine 1**: 5 GB RAM + GPU (upcoming)
- **Machine 2**: 32 GB RAM

The unified Linux VM sees **37.9 GB of combined RAM** and accesses the GPU for AI model inference if available.

## Features

✅ Unified RAM sharing over TCP (NUMA-based implementation)  
✅ QEMU-based Linux VM with dynamic memory integration  
✅ Works without RDMA hardware  
✅ Open-source

## Upcoming Features
✅ Extensible for GPU sharing  
✅ Optimized for running large AI models (LLMs)  

## Getting Started

### Prerequisites

- **Machine 1**: Windows with Rust installed (for VM host)
- **Machine 2**: Windows with Rust installed (for memory server)
- **QEMU** installed on Machine 1

### Installation

Clone the GhostBox repository on both machines:

```bash
git clone https://github.com/DazaiZero/GhostBox.git
cd GhostBox
```

### Setup Instructions

1. **On Machine 2 (m2_memory_server)**:

```bash
cargo run --bin m2_memory_server
```

2. **On Machine 1 (m1_vm_host)**:

```bash
cargo run --bin m1_vm_host
```

### Configuration

Modify `config.toml` for advanced options (e.g., memory chunk size, network settings).

## Demo

Check out the RAM Sharing on Unified VM between 2 machines demo video: Uploading Soon

## Roadmap

- [x] TCP-based RAM sharing
- [ ] GPU passthrough and sharing
- [ ] Multi-machine memory pooling
- [ ] Performance optimization with zero-copy
- [ ] LLM optimization for distributed inference

## Contributing

Contributions are welcome! Feel free to open issues or submit PRs to improve GhostBox.

1. Fork the repository.
2. Create a new branch.
3. Make your changes and commit.
4. Submit a pull request.

## License

This project is licensed under the **MIT License**.

## Connect

Follow my journey as I build **GhostBox** to bring life to old machines for AI workloads!  
Let me know your thoughts and ideas—I’d love to hear them.

GitHub: [GhostBox Repository](https://github.com/DazaiZero/GhostBox)  
LinkedIn: [Aniket Vaidya](https://www.linkedin.com/in/aniket-vaidya-bb7a87160/)

