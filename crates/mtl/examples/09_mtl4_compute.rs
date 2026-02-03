//! MTL4 Compute Pipeline Example
//!
//! This example demonstrates Metal 4 infrastructure with compute pipelines:
//! 1. Create an MTL4 command queue and allocator
//! 2. Create an MTL4 Compiler for shader compilation
//! 3. Compile a compute shader and create a pipeline
//! 4. Execute compute work
//! 5. Read the results back
//!
//! Metal 4 provides more explicit control over GPU resources, similar to
//! Vulkan or DirectX 12. Key features include:
//! - MTL4 CommandQueue with explicit residency management
//! - MTL4 Compiler for pipeline creation
//! - CommandAllocator for memory management
//!
//! Run with: cargo run --example 09_mtl4_compute
//!
//! Note: Metal 4 requires macOS 15+ or iOS 18+. This example will
//! gracefully exit on older systems.

use mtl::{ComputeCommandEncoder, ComputePipelineState, ResourceOptions, Size, device, mtl4};

/// Metal Shading Language (MSL) source code for our compute kernel.
/// This kernel adds two arrays element-wise: output[i] = a[i] + b[i]
const SHADER_SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void vector_add(
    device const float* a [[buffer(0)]],
    device const float* b [[buffer(1)]],
    device float* output [[buffer(2)]],
    uint id [[thread_position_in_grid]]
) {
    output[id] = a[id] + b[id];
}
"#;

fn main() {
    println!("Metal 4 Compute Pipeline Example");
    println!("=================================\n");

    // Step 1: Get the Metal device
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

    // Step 2: Create MTL4 command queue (this checks MTL4 availability)
    println!("\n--- Setting up MTL4 Infrastructure ---");

    let _mtl4_queue = match device.new_mtl4_command_queue() {
        Some(queue) => {
            println!("Created MTL4 command queue");
            if let Some(label) = queue.label() {
                println!("  Queue label: {}", label);
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

    // Create command allocator for recording commands
    let allocator = match device.new_command_allocator() {
        Some(alloc) => {
            println!("Created command allocator");
            println!("  Allocated size: {} bytes", alloc.allocated_size());
            alloc
        }
        None => {
            eprintln!("Error: Failed to create command allocator");
            std::process::exit(1);
        }
    };

    // Step 3: Create MTL4 Compiler
    println!("\n--- Creating MTL4 Compiler ---");

    let compiler_desc =
        mtl4::CompilerDescriptor::new().expect("Failed to create compiler descriptor");
    compiler_desc.set_label("Example Compiler");

    let compiler = match device.new_compiler(&compiler_desc) {
        Ok(c) => {
            println!("Created MTL4 compiler");
            if let Some(label) = c.label() {
                println!("  Compiler label: {}", label);
            }
            c
        }
        Err(e) => {
            eprintln!("Error: Failed to create compiler: {:?}", e);
            std::process::exit(1);
        }
    };

    // Step 4: Compile shader and create pipeline
    println!("\n--- Compiling Shader ---");

    // Compile shader source to library
    let library = match device.new_library_with_source(SHADER_SOURCE, None) {
        Ok(lib) => {
            println!("Shader compiled successfully");
            lib
        }
        Err(e) => {
            eprintln!("Error: Failed to compile shader: {:?}", e);
            std::process::exit(1);
        }
    };

    // Create MTL4 LibraryFunctionDescriptor to demonstrate MTL4 function reference
    let func_desc =
        mtl4::LibraryFunctionDescriptor::new().expect("Failed to create LibraryFunctionDescriptor");
    func_desc.set_library(&library);
    func_desc.set_name("vector_add");

    println!("Created MTL4 LibraryFunctionDescriptor");
    if let Some(name) = func_desc.name() {
        println!("  Function name: {}", name);
    }

    // Get the function for standard pipeline creation
    let function = library
        .new_function_with_name("vector_add")
        .expect("Failed to get 'vector_add' function");

    println!("Retrieved function: {:?}", function.name());

    // Create compute pipeline state
    let pipeline: ComputePipelineState = device
        .new_compute_pipeline_state_with_function(&function)
        .expect("Failed to create compute pipeline");

    println!("\nCreated compute pipeline state");
    println!(
        "  Max threads per threadgroup: {}",
        pipeline.max_total_threads_per_threadgroup()
    );
    println!(
        "  Thread execution width: {}",
        pipeline.thread_execution_width()
    );
    println!(
        "  Static threadgroup memory length: {}",
        pipeline.static_threadgroup_memory_length()
    );

    // Step 5: Create buffers with input data
    println!("\n--- Creating Buffers ---");

    let element_count: usize = 16;
    let buffer_size = element_count * std::mem::size_of::<f32>();

    // Input arrays
    let data_a: Vec<f32> = (0..element_count).map(|i| i as f32).collect();
    let data_b: Vec<f32> = (0..element_count).map(|i| (i * 2) as f32).collect();

    println!("Input A: {:?}", data_a);
    println!("Input B: {:?}", data_b);

    // Create buffers using shared storage for CPU/GPU access
    let bytes_a: &[u8] =
        unsafe { std::slice::from_raw_parts(data_a.as_ptr() as *const u8, buffer_size) };
    let bytes_b: &[u8] =
        unsafe { std::slice::from_raw_parts(data_b.as_ptr() as *const u8, buffer_size) };

    let buffer_a = device
        .new_buffer_with_bytes(bytes_a, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer A");

    let buffer_b = device
        .new_buffer_with_bytes(bytes_b, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer B");

    let buffer_output = device
        .new_buffer(buffer_size, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create output buffer");

    println!("Created 3 buffers ({} bytes each)", buffer_size);

    // Step 6: Execute compute using standard command buffer
    // (MTL4 command buffers have specific requirements; using standard path for compatibility)
    println!("\n--- Recording and Executing Commands ---");

    let command_queue = device
        .new_command_queue()
        .expect("Failed to create command queue");
    let command_buffer = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");

    // Create compute command encoder
    let encoder_ptr = command_buffer.compute_command_encoder();
    let compute_encoder =
        unsafe { ComputeCommandEncoder::from_raw(encoder_ptr) }.expect("Failed to create encoder");

    // Set pipeline and buffers
    compute_encoder.set_compute_pipeline_state(&pipeline);
    compute_encoder.set_buffer(&buffer_a, 0, 0); // buffer(0)
    compute_encoder.set_buffer(&buffer_b, 0, 1); // buffer(1)
    compute_encoder.set_buffer(&buffer_output, 0, 2); // buffer(2)

    // Calculate dispatch configuration
    let thread_execution_width = pipeline.thread_execution_width();
    let threads_per_threadgroup = Size::new(thread_execution_width, 1, 1);
    let threadgroup_count = element_count.div_ceil(thread_execution_width);
    let threadgroups_per_grid = Size::new(threadgroup_count, 1, 1);

    println!("Dispatch configuration:");
    println!("  Elements: {}", element_count);
    println!("  Threads per threadgroup: {}", thread_execution_width);
    println!("  Threadgroups: {}", threadgroup_count);

    // Dispatch and execute
    compute_encoder.dispatch_threadgroups(threadgroups_per_grid, threads_per_threadgroup);
    compute_encoder.end_encoding();
    command_buffer.commit();

    println!("Commands committed, waiting for completion...");
    command_buffer.wait_until_completed();
    println!("GPU execution completed!");

    // Step 7: Read results
    println!("\n--- Results ---");

    let result_ptr = buffer_output
        .contents()
        .expect("Output buffer contents is null") as *const f32;
    let result: Vec<f32> =
        unsafe { std::slice::from_raw_parts(result_ptr, element_count).to_vec() };

    println!("Output: {:?}", result);

    // Verify results
    let expected: Vec<f32> = data_a
        .iter()
        .zip(data_b.iter())
        .map(|(a, b)| a + b)
        .collect();

    if result == expected {
        println!("\nSuccess! Vector addition computed correctly.");
    } else {
        println!("\nError: Results don't match expected values.");
        println!("Expected: {:?}", expected);
        std::process::exit(1);
    }

    // Print timing info
    let gpu_start = command_buffer.gpu_start_time();
    let gpu_end = command_buffer.gpu_end_time();
    println!(
        "\nGPU execution time: {:.6} ms",
        (gpu_end - gpu_start) * 1000.0
    );

    // Demonstrate allocator memory management
    println!("\n--- MTL4 Allocator Memory ---");
    println!(
        "Allocated size before reset: {} bytes",
        allocator.allocated_size()
    );
    allocator.reset();
    println!(
        "Allocated size after reset: {} bytes",
        allocator.allocated_size()
    );

    // Summary of MTL4 features demonstrated
    println!("\n=== MTL4 Features Demonstrated ===");
    println!("1. MTL4::CommandQueue - Created with explicit residency control");
    println!("2. MTL4::CommandAllocator - Memory management for command recording");
    println!("3. MTL4::Compiler - Shader compilation infrastructure");
    println!("4. MTL4::LibraryFunctionDescriptor - Function reference by name");
    println!("\nNote: Full MTL4 command buffer recording requires");
    println!("additional setup; this example uses standard Metal");
    println!("command buffers for compute dispatch compatibility.");

    // Show compiler info
    println!("\n--- Compiler Info ---");
    if let Some(device) = compiler.device() {
        println!("Compiler device: {}", device.name());
    }

    println!("\nMetal 4 compute example completed successfully!");
}
