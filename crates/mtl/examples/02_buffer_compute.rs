//! Buffer Compute Example
//!
//! This example demonstrates a complete Metal compute pipeline workflow:
//! 1. Create a Metal device
//! 2. Create a buffer with data
//! 3. Compile a compute shader from source
//! 4. Create a compute pipeline
//! 5. Dispatch the compute kernel
//! 6. Read the results back
//!
//! Run with: cargo run --example 02_buffer_compute

use mtl::{ComputeCommandEncoder, ComputePipelineState, ResourceOptions, Size, device};

/// Metal Shading Language (MSL) source code for our compute kernel.
/// This kernel multiplies each element in the buffer by 2.
const SHADER_SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void multiply_by_two(
    device float* data [[buffer(0)]],
    uint id [[thread_position_in_grid]]
) {
    data[id] = data[id] * 2.0;
}
"#;

fn main() {
    println!("Metal Buffer Compute Example");
    println!("=============================\n");

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

    // Step 2: Create input data and a buffer
    let element_count: usize = 16;
    let data: Vec<f32> = (0..element_count).map(|i| i as f32).collect();
    println!("\nInput data: {:?}", data);

    // Create a buffer with our data
    // Using STORAGE_MODE_SHARED so both CPU and GPU can access it
    // Convert f32 slice to bytes
    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            data.as_ptr() as *const u8,
            data.len() * std::mem::size_of::<f32>(),
        )
    };
    let buffer = device
        .new_buffer_with_bytes(bytes, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    println!("Created buffer: {} bytes", buffer.length());

    // Step 3: Compile the shader
    println!("\nCompiling shader...");
    let library = device
        .new_library_with_source(SHADER_SOURCE, None)
        .expect("Failed to compile shader");

    // Get the function from the library
    let function = library
        .new_function_with_name("multiply_by_two")
        .expect("Function 'multiply_by_two' not found in library");

    println!("Shader compiled successfully: {:?}", function.name());

    // Step 4: Create the compute pipeline
    let pipeline: ComputePipelineState = device
        .new_compute_pipeline_state_with_function(&function)
        .expect("Failed to create compute pipeline");

    println!("Pipeline created");
    println!(
        "  Max threads per threadgroup: {}",
        pipeline.max_total_threads_per_threadgroup()
    );
    println!(
        "  Thread execution width: {}",
        pipeline.thread_execution_width()
    );

    // Step 5: Create a command queue and encode commands
    let command_queue = device
        .new_command_queue()
        .expect("Failed to create command queue");
    let command_buffer = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");

    // Get a compute command encoder
    let compute_encoder_ptr = command_buffer.compute_command_encoder();
    let compute_encoder = unsafe { ComputeCommandEncoder::from_raw(compute_encoder_ptr) }
        .expect("Failed to create compute encoder");

    // Set up the compute pass
    compute_encoder.set_compute_pipeline_state(&pipeline);
    compute_encoder.set_buffer(&buffer, 0, 0);

    // Calculate thread configuration
    let thread_execution_width = pipeline.thread_execution_width();
    let threads_per_threadgroup = Size::new(thread_execution_width, 1, 1);

    // Calculate how many threadgroups we need
    let threadgroup_count = element_count.div_ceil(thread_execution_width);
    let threadgroups_per_grid = Size::new(threadgroup_count, 1, 1);

    println!("\nDispatching compute kernel:");
    println!("  Elements: {}", element_count);
    println!("  Threads per threadgroup: {}", thread_execution_width);
    println!("  Threadgroups: {}", threadgroup_count);

    // Dispatch the compute kernel
    compute_encoder.dispatch_threadgroups(threadgroups_per_grid, threads_per_threadgroup);

    // End encoding and commit
    compute_encoder.end_encoding();
    command_buffer.commit();

    // Wait for completion
    println!("\nWaiting for GPU to complete...");
    command_buffer.wait_until_completed();

    // Step 6: Read results back
    let result_ptr = buffer.contents().expect("Buffer contents is null") as *const f32;
    let result: Vec<f32> =
        unsafe { std::slice::from_raw_parts(result_ptr, element_count).to_vec() };

    println!("\nOutput data: {:?}", result);

    // Verify the results
    let expected: Vec<f32> = (0..element_count).map(|i| (i as f32) * 2.0).collect();

    if result == expected {
        println!("\nSuccess! All values correctly multiplied by 2.");
    } else {
        println!("\nError: Results don't match expected values.");
        println!("Expected: {:?}", expected);
        std::process::exit(1);
    }

    // Print execution timing
    let gpu_start = command_buffer.gpu_start_time();
    let gpu_end = command_buffer.gpu_end_time();
    let kernel_start = command_buffer.kernel_start_time();
    let kernel_end = command_buffer.kernel_end_time();

    println!("\nTiming:");
    println!("  GPU time: {:.6} ms", (gpu_end - gpu_start) * 1000.0);
    println!(
        "  Kernel time: {:.6} ms",
        (kernel_end - kernel_start) * 1000.0
    );

    println!("\nCompute example completed successfully!");
}
