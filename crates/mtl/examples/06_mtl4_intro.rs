//! Metal 4 Introduction Example
//!
//! This example demonstrates the basics of Metal 4 (MTL4) APIs:
//! 1. Creating an MTL4 command queue
//! 2. Creating a command allocator
//! 3. MTL4 command buffer lifecycle (begin/end)
//! 4. Batch commit with feedback handlers
//!
//! Metal 4 provides more explicit control over GPU resources compared to
//! standard Metal, similar to Vulkan or DirectX 12.
//!
//! Run with: cargo run --example 06_mtl4_intro
//!
//! Note: Metal 4 requires macOS 15+ or iOS 18+. This example will
//! gracefully exit on older systems.

use mtl_gpu::device;

fn main() {
    println!("Metal 4 Introduction Example");
    println!("=============================\n");

    // Get the Metal device
    let device = match device::system_default() {
        Some(device) => {
            println!("Using device: {}", device.name());
            device
        }
        None => {
            eprintln!("Error: No Metal device found.");
            std::process::exit(1);
        }
    };

    // =======================================================================
    // Part 1: Create MTL4 Command Queue
    // =======================================================================
    println!("\n--- Part 1: MTL4 Command Queue ---");

    let mtl4_queue = match device.new_mtl4_command_queue() {
        Some(queue) => {
            println!("Created MTL4 command queue");
            if let Some(label) = queue.label() {
                println!("  Queue label: {}", label);
            } else {
                println!("  Queue label: (none)");
            }
            queue
        }
        None => {
            println!("MTL4 is not available on this system.");
            println!("Metal 4 requires macOS 15+ or iOS 18+.");
            println!("\nExiting gracefully.");
            std::process::exit(0);
        }
    };

    // =======================================================================
    // Part 2: Create Command Allocator
    // =======================================================================
    println!("\n--- Part 2: Command Allocator ---");

    // Command allocators manage memory for command buffer recording
    // They can be reused across multiple command buffers
    let allocator = match device.new_command_allocator() {
        Some(alloc) => {
            println!("Created command allocator");
            println!("  Initial allocated size: {} bytes", alloc.allocated_size());
            alloc
        }
        None => {
            eprintln!("Error: Failed to create command allocator");
            std::process::exit(1);
        }
    };

    // =======================================================================
    // Part 3: Create and Use MTL4 Command Buffer
    // =======================================================================
    println!("\n--- Part 3: MTL4 Command Buffer ---");

    // In MTL4, we need to create command buffers differently
    // Using the queue's device to get a reference
    let mtl4_device = mtl4_queue.device().expect("Queue should have device");

    // Create command allocator descriptor with custom settings
    let alloc_desc = mtl_gpu::mtl4::CommandAllocatorDescriptor::new()
        .expect("Failed to create allocator descriptor");
    alloc_desc.set_label("Example Allocator");

    let custom_allocator = mtl4_device
        .new_command_allocator_with_descriptor(&alloc_desc)
        .expect("Failed to create command allocator with descriptor");

    println!("Created custom allocator with label");
    if let Some(label) = custom_allocator.label() {
        println!("  Allocator label: {}", label);
    }

    // =======================================================================
    // Part 4: MTL4 Command Queue Descriptor
    // =======================================================================
    println!("\n--- Part 4: Command Queue with Descriptor ---");

    let queue_desc =
        mtl_gpu::mtl4::CommandQueueDescriptor::new().expect("Failed to create queue descriptor");
    queue_desc.set_label("Custom MTL4 Queue");

    let custom_queue = device
        .new_mtl4_command_queue_with_descriptor(&queue_desc)
        .expect("Failed to create queue with descriptor");

    println!("Created custom MTL4 queue");
    if let Some(label) = custom_queue.label() {
        println!("  Queue label: {}", label);
    }

    // =======================================================================
    // Part 5: Allocator Memory Management
    // =======================================================================
    println!("\n--- Part 5: Allocator Memory ---");

    println!("Before reset:");
    println!("  Allocated size: {} bytes", allocator.allocated_size());

    // Reset frees all allocated memory
    allocator.reset();

    println!("After reset:");
    println!("  Allocated size: {} bytes", allocator.allocated_size());

    // =======================================================================
    // Summary
    // =======================================================================
    println!("\n=== Summary ===");
    println!("Metal 4 APIs demonstrated:");
    println!("  - MTL4::CommandQueue creation");
    println!("  - MTL4::CommandAllocator creation");
    println!("  - MTL4::CommandQueueDescriptor configuration");
    println!("  - MTL4::CommandAllocatorDescriptor configuration");
    println!("  - Allocator memory management (reset)");
    println!("\nMetal 4 provides explicit control over:");
    println!("  - Command buffer memory allocation");
    println!("  - Residency set management");
    println!("  - Batch command buffer commits");
    println!("  - Completion feedback handlers");

    println!("\nMetal 4 intro example completed!");
}
