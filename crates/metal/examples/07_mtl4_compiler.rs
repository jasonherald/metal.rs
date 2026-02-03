//! Metal 4 Compiler and Pipeline Example
//!
//! This example demonstrates MTL4 compiler and async pipeline creation:
//! 1. Create an MTL4 compiler
//! 2. Using CommitOptions with feedback handlers
//! 3. Counter heaps for GPU timing
//!
//! Metal 4 provides async pipeline compilation and explicit GPU timing
//! through counter heaps.
//!
//! Run with: cargo run --example 07_mtl4_compiler
//!
//! Note: Metal 4 requires macOS 15+ or iOS 18+. This example will
//! gracefully exit on older systems.

use metal::device;

fn main() {
    println!("Metal 4 Compiler and Counter Heap Example");
    println!("==========================================\n");

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
    // Part 1: Check MTL4 Availability
    // =======================================================================
    println!("\n--- Part 1: MTL4 Availability Check ---");

    let queue = match device.new_mtl4_command_queue() {
        Some(queue) => {
            println!("MTL4 is available on this system");
            queue
        }
        None => {
            println!("MTL4 is not available on this system.");
            println!("Metal 4 requires macOS 15+ or iOS 18+.");
            println!("\nExiting gracefully.");
            std::process::exit(0);
        }
    };

    if let Some(label) = queue.label() {
        println!("  Queue label: {}", label);
    } else {
        println!("  Queue label: (none - default)");
    }

    // =======================================================================
    // Part 2: Command Allocator Management
    // =======================================================================
    println!("\n--- Part 2: Command Allocator Management ---");

    // Create allocator with descriptor for more control
    let alloc_desc = metal::mtl4::CommandAllocatorDescriptor::new()
        .expect("Failed to create allocator descriptor");
    alloc_desc.set_label("Example Allocator");

    let allocator = device
        .new_command_allocator_with_descriptor(&alloc_desc)
        .expect("Failed to create command allocator");

    println!("Created command allocator");
    if let Some(label) = allocator.label() {
        println!("  Label: {}", label);
    }
    println!(
        "  Initial allocated size: {} bytes",
        allocator.allocated_size()
    );

    // =======================================================================
    // Part 3: MTL4 Compiler
    // =======================================================================
    println!("\n--- Part 3: MTL4 Compiler ---");

    // Create compiler descriptor
    let compiler_desc =
        metal::mtl4::CompilerDescriptor::new().expect("Failed to create compiler descriptor");
    compiler_desc.set_label("Example Compiler");

    match device.new_compiler(&compiler_desc) {
        Ok(compiler) => {
            println!("Created MTL4 compiler successfully");
            if let Some(label) = compiler.label() {
                println!("  Compiler label: {}", label);
            }
            // Note: To compile pipelines async, you would use:
            // compiler.compile_render_pipeline(descriptor, handler)
            // compiler.compile_compute_pipeline(descriptor, handler)
            println!("  (Compiler is ready for async pipeline compilation)");
        }
        Err(e) => {
            println!("Failed to create compiler: {:?}", e);
            println!("  (This may be expected on some system configurations)");
        }
    }

    // =======================================================================
    // Part 4: Counter Heap for GPU Timing
    // =======================================================================
    println!("\n--- Part 4: Counter Heap for GPU Timing ---");

    // Check counter heap entry size
    let timestamp_entry_size =
        device.size_of_counter_heap_entry(metal::mtl4::CounterHeapType::TIMESTAMP);
    println!(
        "Timestamp counter heap entry size: {} bytes",
        timestamp_entry_size
    );

    // Create a counter heap for timestamps
    let counter_heap_desc = metal::mtl4::CounterHeapDescriptor::new()
        .expect("Failed to create counter heap descriptor");
    counter_heap_desc.set_heap_type(metal::mtl4::CounterHeapType::TIMESTAMP);
    counter_heap_desc.set_count(1024); // 1024 entries

    match device.new_counter_heap(&counter_heap_desc) {
        Ok(counter_heap) => {
            println!("Created counter heap successfully");
            counter_heap.set_label("Timing Counter Heap");
            if let Some(label) = counter_heap.label() {
                println!("  Label: {}", label);
            }
            println!("  Count: {} entries", counter_heap.count());
            println!("  Type: {:?}", counter_heap.heap_type());
            println!("  (Counter heap can be used for GPU timestamp queries)");
        }
        Err(e) => {
            println!("Failed to create counter heap: {:?}", e);
            println!("  (Counter heaps may require specific hardware support)");
        }
    }

    // =======================================================================
    // Part 5: About CommitOptions and Feedback
    // =======================================================================
    println!("\n--- Part 5: CommitOptions (Conceptual) ---");

    println!("CommitOptions is used when committing command buffers:");
    println!("  let options = CommitOptions::new();");
    println!("  options.add_feedback_handler(|feedback| {{ ... }});");
    println!("  queue.commit_with_options(&[&cmd_buf], &options);");
    println!("\nFeedback handlers provide:");
    println!("  - GPU start/end times");
    println!("  - Execution timing information");
    println!("  - Completion notification");

    // =======================================================================
    // Part 6: Allocator Reset
    // =======================================================================
    println!("\n--- Part 6: Allocator Memory Management ---");

    println!(
        "Allocator size before reset: {} bytes",
        allocator.allocated_size()
    );

    // Reset frees all allocated memory
    allocator.reset();

    println!("After reset: {} bytes", allocator.allocated_size());
    println!("  (Reset releases recorded command memory for reuse)");

    // =======================================================================
    // Summary
    // =======================================================================
    println!("\n=== Summary ===");
    println!("Metal 4 features demonstrated:");
    println!("  - MTL4::CommandQueue creation");
    println!("  - MTL4::CommandAllocator with descriptors");
    println!("  - MTL4::Compiler for async pipeline compilation");
    println!("  - MTL4::CounterHeap for GPU timing");
    println!("  - MTL4::CommitOptions with feedback handlers");
    println!("\nMetal 4 benefits:");
    println!("  - Explicit memory management for command buffers");
    println!("  - Async pipeline compilation for reduced stalls");
    println!("  - Fine-grained GPU timing through counter heaps");
    println!("  - Batch command buffer commits with completion feedback");

    println!("\nMetal 4 compiler example completed!");
}
