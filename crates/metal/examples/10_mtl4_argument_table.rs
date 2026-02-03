//! MTL4 ArgumentTable Buffer Binding Example
//!
//! This example demonstrates Metal 4 ArgumentTable for bindless resource management:
//! 1. Create an MTL4 ArgumentTable with capacity configuration
//! 2. Create buffers and retrieve their GPU addresses
//! 3. Bind buffers to the ArgumentTable by GPU address
//! 4. Demonstrate bindless buffer access patterns
//!
//! ArgumentTable is a key MTL4 feature for bindless GPU programming, enabling:
//! - Dynamic resource binding without pipeline recompilation
//! - GPU address-based buffer access (useful for llama.cpp-style weight access)
//! - Reduced CPU overhead for resource binding
//!
//! Run with: cargo run --example 10_mtl4_argument_table
//!
//! Note: Metal 4 requires macOS 15+ or iOS 18+. This example will
//! gracefully exit on older systems.

use metal::{device, mtl4, ResourceOptions};

fn main() {
    println!("Metal 4 ArgumentTable Example");
    println!("==============================\n");

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

    // Step 2: Verify MTL4 availability
    println!("\n--- Checking MTL4 Availability ---");

    let _mtl4_queue = match device.new_mtl4_command_queue() {
        Some(queue) => {
            println!("MTL4 is available");
            queue
        }
        None => {
            println!("MTL4 is not available on this system.");
            println!("Metal 4 requires macOS 15+ or iOS 18+.");
            println!("\nExiting gracefully.");
            std::process::exit(0);
        }
    };

    // Step 3: Create test buffers with sample data
    println!("\n--- Creating Buffers ---");

    // Simulate weight matrices (like in llama.cpp)
    let weights_data: Vec<f32> = (0..256).map(|i| (i as f32) * 0.01).collect();
    let input_data: Vec<f32> = (0..64).map(|i| i as f32).collect();
    let output_size = 64 * std::mem::size_of::<f32>();

    // Create buffers
    let weights_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            weights_data.as_ptr() as *const u8,
            weights_data.len() * std::mem::size_of::<f32>(),
        )
    };
    let input_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            input_data.as_ptr() as *const u8,
            input_data.len() * std::mem::size_of::<f32>(),
        )
    };

    let weights_buffer = device
        .new_buffer_with_bytes(weights_bytes, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create weights buffer");

    let input_buffer = device
        .new_buffer_with_bytes(input_bytes, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create input buffer");

    let output_buffer = device
        .new_buffer(output_size, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create output buffer");

    println!("Created buffers:");
    println!("  Weights: {} bytes", weights_buffer.length());
    println!("  Input:   {} bytes", input_buffer.length());
    println!("  Output:  {} bytes", output_buffer.length());

    // Step 4: Get GPU addresses from buffers
    println!("\n--- Buffer GPU Addresses ---");

    let weights_gpu_addr = weights_buffer.gpu_address();
    let input_gpu_addr = input_buffer.gpu_address();
    let output_gpu_addr = output_buffer.gpu_address();

    println!("GPU Addresses (64-bit):");
    println!("  Weights: 0x{:016X}", weights_gpu_addr);
    println!("  Input:   0x{:016X}", input_gpu_addr);
    println!("  Output:  0x{:016X}", output_gpu_addr);

    // Verify addresses are valid (non-zero)
    assert!(weights_gpu_addr != 0, "Weights GPU address should be non-zero");
    assert!(input_gpu_addr != 0, "Input GPU address should be non-zero");
    assert!(output_gpu_addr != 0, "Output GPU address should be non-zero");

    println!("\nAll GPU addresses are valid");

    // Step 5: Create ArgumentTable
    println!("\n--- Creating ArgumentTable ---");

    // Configure the argument table descriptor
    let table_desc =
        mtl4::ArgumentTableDescriptor::new().expect("Failed to create ArgumentTableDescriptor");

    table_desc.set_label("Model Weights Table");
    table_desc.set_max_buffer_bind_count(16); // Support up to 16 buffer bindings
    table_desc.set_max_texture_bind_count(0); // No textures needed
    table_desc.set_max_sampler_state_bind_count(0); // No samplers needed
    table_desc.set_initialize_bindings(true); // Initialize all bindings to null

    println!("ArgumentTableDescriptor configuration:");
    println!("  Label: {:?}", table_desc.label());
    println!("  Max buffer bindings: {}", table_desc.max_buffer_bind_count());
    println!("  Max texture bindings: {}", table_desc.max_texture_bind_count());
    println!("  Max sampler bindings: {}", table_desc.max_sampler_state_bind_count());
    println!("  Initialize bindings: {}", table_desc.initialize_bindings());

    // Create the argument table
    let arg_table = match device.new_argument_table(&table_desc) {
        Ok(table) => {
            println!("\nCreated ArgumentTable successfully");
            if let Some(label) = table.label() {
                println!("  Table label: {}", label);
            }
            if let Some(dev) = table.device() {
                println!("  Device: {}", dev.name());
            }
            table
        }
        Err(e) => {
            eprintln!("Error: Failed to create ArgumentTable: {:?}", e);
            std::process::exit(1);
        }
    };

    // Step 6: Bind buffers to the ArgumentTable by GPU address
    println!("\n--- Binding Buffers by GPU Address ---");

    // Use descriptive binding indices (like shader buffer indices)
    const WEIGHTS_INDEX: usize = 0;
    const INPUT_INDEX: usize = 1;
    const OUTPUT_INDEX: usize = 2;

    arg_table.set_address(weights_gpu_addr, WEIGHTS_INDEX);
    println!("Bound weights buffer to index {} (addr: 0x{:X})", WEIGHTS_INDEX, weights_gpu_addr);

    arg_table.set_address(input_gpu_addr, INPUT_INDEX);
    println!("Bound input buffer to index {} (addr: 0x{:X})", INPUT_INDEX, input_gpu_addr);

    arg_table.set_address(output_gpu_addr, OUTPUT_INDEX);
    println!("Bound output buffer to index {} (addr: 0x{:X})", OUTPUT_INDEX, output_gpu_addr);

    // Step 7: Demonstrate binding with stride (for strided buffer access)
    println!("\n--- Binding with Stride ---");

    // Create a buffer with interleaved data
    let interleaved_data: Vec<f32> = (0..128).map(|i| i as f32).collect();
    let interleaved_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            interleaved_data.as_ptr() as *const u8,
            interleaved_data.len() * std::mem::size_of::<f32>(),
        )
    };

    let interleaved_buffer = device
        .new_buffer_with_bytes(interleaved_bytes, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create interleaved buffer");

    let interleaved_gpu_addr = interleaved_buffer.gpu_address();
    let stride = 8 * std::mem::size_of::<f32>(); // Every 8 floats

    const INTERLEAVED_INDEX: usize = 3;

    arg_table.set_address_with_stride(interleaved_gpu_addr, stride, INTERLEAVED_INDEX);
    println!(
        "Bound interleaved buffer to index {} with stride {} bytes (addr: 0x{:X})",
        INTERLEAVED_INDEX, stride, interleaved_gpu_addr
    );

    // Step 8: Create a second argument table for comparison
    println!("\n--- Creating Second ArgumentTable ---");

    let table_desc2 =
        mtl4::ArgumentTableDescriptor::new().expect("Failed to create second ArgumentTableDescriptor");

    table_desc2.set_label("Auxiliary Table");
    table_desc2.set_max_buffer_bind_count(4);
    table_desc2.set_initialize_bindings(true);

    let arg_table2 = match device.new_argument_table(&table_desc2) {
        Ok(table) => {
            println!("Created second ArgumentTable");
            table
        }
        Err(e) => {
            eprintln!("Error: Failed to create second ArgumentTable: {:?}", e);
            std::process::exit(1);
        }
    };

    // Bind some addresses to the second table
    arg_table2.set_address(weights_gpu_addr, 0);
    println!("Second table: bound weights to index 0");

    // Step 9: Summary
    println!("\n=== ArgumentTable Summary ===");
    println!("Created 2 argument tables demonstrating:");
    println!("  1. Basic GPU address binding (set_address)");
    println!("  2. Strided buffer binding (set_address_with_stride)");
    println!("  3. Multiple tables for different resource sets");

    println!("\n--- Bindless Access Pattern ---");
    println!("In Metal shaders, you can access these buffers via:");
    println!();
    println!("  // Declare the argument buffer");
    println!("  struct Arguments {{");
    println!("      device float* weights [[id(0)]];");
    println!("      device float* input   [[id(1)]];");
    println!("      device float* output  [[id(2)]];");
    println!("  }};");
    println!();
    println!("  kernel void compute(");
    println!("      constant Arguments& args [[buffer(0)]],");
    println!("      uint tid [[thread_position_in_grid]]");
    println!("  ) {{");
    println!("      float w = args.weights[tid];");
    println!("      float i = args.input[tid];");
    println!("      args.output[tid] = w * i;");
    println!("  }}");

    println!("\n--- Benefits for LLM Inference (llama.cpp) ---");
    println!("1. Weight matrices can be pre-bound once and reused");
    println!("2. KV-cache buffers can be dynamically bound each forward pass");
    println!("3. Reduces CPU overhead compared to traditional set_buffer calls");
    println!("4. Enables flexible memory layouts with strided access");

    // Show memory addresses for verification
    println!("\n--- Final Buffer Address Summary ---");
    println!(
        "weights_buffer.gpu_address() = 0x{:016X}",
        weights_buffer.gpu_address()
    );
    println!(
        "input_buffer.gpu_address()   = 0x{:016X}",
        input_buffer.gpu_address()
    );
    println!(
        "output_buffer.gpu_address()  = 0x{:016X}",
        output_buffer.gpu_address()
    );

    println!("\nMetal 4 ArgumentTable example completed successfully!");
}
