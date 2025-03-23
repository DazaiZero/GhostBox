Building a Unified VM: Harnessing Old Machines for Modern AI Workloads

Running large language models (LLMs) locally is challenging—especially when you don’t have a single high-powered machine. But what if you could combine the resources of multiple devices on your network to create a unified system?

I’m working on a project to do exactly that—building a Unified Virtual Machine (VM) that pools RAM and GPU resources across multiple machines using NUMA (Non-Uniform Memory Access) nodes.

In the first phase, I’ve successfully implemented RAM sharing between two machines over a network. While I initially explored RDMA (Remote Direct Memory Access) for faster performance, my hardware didn’t support it. So, I built a TCP-based solution to transfer memory across machines, enabling efficient resource sharing without specialized hardware.

Why am I doing this?
I have several older devices lying around—four laptops with GPUs and one desktop with 128 GB of RAM—none of which, on their own, can handle large AI models. Instead of letting them collect dust, I’m creating a unified system where these machines work together to run demanding workloads like LLM inference.

The journey has been both technical and rewarding, and I’m excited to share more as I enhance the system with GPU sharing and performance optimizations.
