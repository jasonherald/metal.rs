//! Metal 4 Machine Learning Pipeline Example
//!
//! This example demonstrates the MTL4 machine learning APIs for GPU inference:
//!
//! 1. **MachineLearningPipelineDescriptor** - Configures ML pipeline with:
//!    - Function descriptor pointing to a compiled neural network
//!    - Input dimensions (tensor shapes)
//!
//! 2. **MachineLearningPipelineState** - Compiled pipeline providing:
//!    - Intermediates heap size (for layer activations)
//!    - Pipeline reflection for introspection
//!
//! 3. **ArgumentTable** - Binds resources to the pipeline:
//!    - Input buffers (model weights, input tensors)
//!    - Output buffers (inference results)
//!    - GPU addresses for bindless access
//!
//! 4. **MachineLearningCommandEncoder** - Dispatches inference:
//!    - Sets pipeline state
//!    - Sets argument table
//!    - Dispatches network with intermediates heap
//!
//! ## Metal 4 ML Pipeline Workflow
//!
//! ```text
//! [Neural Network (e.g., from MLX/CoreML)]
//!              ↓
//! [Metal Shader Library with ML kernel]
//!              ↓
//! [MTL4 Compiler] → [MachineLearningPipelineState]
//!              ↓
//! [ArgumentTable] ← [Input/Output Buffers]
//!              ↓
//! [MachineLearningCommandEncoder]
//!              ↓
//! [GPU Execution] → [Inference Results]
//! ```
//!
//! ## Key Benefits of MTL4 ML APIs
//!
//! - **Explicit Memory Management**: Control over intermediates heap sizing
//! - **ArgumentTable Binding**: Efficient resource binding without per-dispatch overhead
//! - **Async Compilation**: Pipeline compilation doesn't block the main thread
//! - **Reflection**: Introspect pipeline bindings for validation
//!
//! Run with: cargo run --example 08_mtl4_machine_learning
//!
//! Note: Metal 4 requires macOS 15+ or iOS 18+. This example demonstrates
//! the API structure without requiring an actual neural network model.

use mtl::device;

fn main() {
    println!("Metal 4 Machine Learning Pipeline Example");
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
    println!("\n--- Part 1: MTL4 Availability ---");

    let _queue = match device.new_mtl4_command_queue() {
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

    // =======================================================================
    // Part 2: MachineLearningPipelineDescriptor
    // =======================================================================
    println!("\n--- Part 2: MachineLearningPipelineDescriptor ---");

    match mtl::mtl4::MachineLearningPipelineDescriptor::new() {
        Some(ml_desc) => {
            println!("Created MachineLearningPipelineDescriptor");

            // Set a label for debugging
            ml_desc.set_label("Example ML Pipeline");
            if let Some(label) = ml_desc.label() {
                println!("  Label: {}", label);
            }

            // In a real application, you would:
            // 1. Load a Metal library with the compiled ML kernel
            // 2. Create a LibraryFunctionDescriptor pointing to the kernel
            // 3. Set it on the pipeline descriptor:
            //    ml_desc.set_machine_learning_function_descriptor(&func_desc);
            //
            // 4. Set input dimensions (tensor shapes):
            //    ml_desc.set_input_dimensions_raw(tensor_extents_ptr, buffer_index);

            println!("\n  Typical configuration:");
            println!("    1. Load Metal library with neural network kernel");
            println!("    2. Create FunctionDescriptor for the kernel");
            println!("    3. Set input tensor dimensions per buffer index");
            println!("    4. Compile with MTL4 Compiler");

            // Reset returns descriptor to default state
            ml_desc.reset();
            println!("\n  Called reset() to clear descriptor state");
        }
        None => {
            println!("MachineLearningPipelineDescriptor not available");
        }
    }

    // =======================================================================
    // Part 3: ArgumentTable for Resource Binding
    // =======================================================================
    println!("\n--- Part 3: ArgumentTable ---");

    // Create argument table descriptor
    match mtl::mtl4::ArgumentTableDescriptor::new() {
        Some(table_desc) => {
            println!("Created ArgumentTableDescriptor");

            // Configure binding capacity
            table_desc.set_max_buffer_bind_count(16);
            table_desc.set_max_texture_bind_count(8);
            table_desc.set_max_sampler_state_bind_count(4);
            table_desc.set_initialize_bindings(true);

            println!("  Configuration:");
            println!(
                "    Max buffer bindings: {}",
                table_desc.max_buffer_bind_count()
            );
            println!(
                "    Max texture bindings: {}",
                table_desc.max_texture_bind_count()
            );
            println!(
                "    Max sampler bindings: {}",
                table_desc.max_sampler_state_bind_count()
            );
            println!(
                "    Initialize bindings: {}",
                table_desc.initialize_bindings()
            );

            // Create the argument table
            match device.new_argument_table(&table_desc) {
                Ok(arg_table) => {
                    println!("\n  Created ArgumentTable");

                    // In a real application, you would bind resources:
                    //
                    // Bind input buffer at index 0:
                    //   arg_table.set_address(input_buffer.gpu_address(), 0);
                    //
                    // Bind weights buffer at index 1:
                    //   arg_table.set_address(weights_buffer.gpu_address(), 1);
                    //
                    // Bind output buffer at index 2:
                    //   arg_table.set_address(output_buffer.gpu_address(), 2);
                    //
                    // Or bind textures for image-based models:
                    //   arg_table.set_texture(texture.resource_id(), 0);

                    println!("\n  Binding methods available:");
                    println!("    set_address(gpu_address, index) - Bind buffer by GPU address");
                    println!("    set_resource(resource_id, index) - Bind buffer by resource ID");
                    println!("    set_texture(resource_id, index) - Bind texture");
                    println!("    set_sampler_state(resource_id, index) - Bind sampler");

                    drop(arg_table);
                }
                Err(e) => {
                    println!("  Failed to create ArgumentTable: {:?}", e);
                }
            }
        }
        None => {
            println!("ArgumentTableDescriptor not available");
        }
    }

    // =======================================================================
    // Part 4: MTL4 Compiler for ML Pipelines
    // =======================================================================
    println!("\n--- Part 4: Compiler for ML Pipelines ---");

    let compiler_desc = match mtl::mtl4::CompilerDescriptor::new() {
        Some(desc) => {
            desc.set_label("ML Pipeline Compiler");
            println!("Created CompilerDescriptor");
            desc
        }
        None => {
            println!("CompilerDescriptor not available");
            println!("\n=== Summary ===");
            print_summary();
            return;
        }
    };

    match device.new_compiler(&compiler_desc) {
        Ok(compiler) => {
            println!("Created MTL4 Compiler");

            // In a real application with an ML pipeline descriptor configured:
            //
            // Synchronous compilation:
            //   let pipeline_state = compiler.new_machine_learning_pipeline_state(
            //       &ml_desc,
            //       None, // CompilerTaskOptions
            //   )?;
            //
            // Async compilation (preferred for large models):
            //   let task = compiler.new_machine_learning_pipeline_state_async(
            //       &ml_desc,
            //       None,
            //       |result, error| {
            //           if let Some(pipeline) = result {
            //               // Pipeline ready for use
            //               println!("Intermediates heap size: {}", pipeline.intermediates_heap_size());
            //           }
            //       },
            //   );

            println!("\n  Compiler methods for ML:");
            println!("    new_machine_learning_pipeline_state() - Synchronous compilation");
            println!("    new_machine_learning_pipeline_state_async() - Async compilation");

            drop(compiler);
        }
        Err(e) => {
            println!("Failed to create compiler: {:?}", e);
        }
    }

    // =======================================================================
    // Part 5: Intermediates Heap for Layer Activations
    // =======================================================================
    println!("\n--- Part 5: Intermediates Heap ---");

    // Neural networks require memory for intermediate layer activations.
    // The intermediates heap size is determined by the compiled pipeline.
    //
    // Example workflow:
    //   1. Compile ML pipeline
    //   2. Query intermediates_heap_size() from the pipeline state
    //   3. Create a heap with that size
    //   4. Pass heap to dispatchNetwork()

    println!("Intermediates Heap Workflow:");
    println!("  1. let heap_size = pipeline_state.intermediates_heap_size();");
    println!("  2. Create HeapDescriptor with size = heap_size");
    println!("  3. let heap = device.new_heap(&heap_desc);");
    println!("  4. encoder.dispatch_network(&heap);");

    // Create a sample heap to show the API
    let heap_desc = mtl::HeapDescriptor::new().expect("Failed to create heap descriptor");
    heap_desc.set_size(1024 * 1024); // 1 MB placeholder
    heap_desc.set_storage_mode(mtl::enums::StorageMode::PRIVATE);

    match device.new_heap(&heap_desc) {
        Some(heap) => {
            println!("\n  Created sample heap:");
            println!("    Size: {} bytes", heap.size());
            println!("    Storage mode: Private (GPU-only, optimal for intermediates)");
        }
        None => {
            println!("  Failed to create heap");
        }
    }

    // =======================================================================
    // Part 6: MachineLearningCommandEncoder Workflow
    // =======================================================================
    println!("\n--- Part 6: MachineLearningCommandEncoder ---");

    println!("ML Command Encoder Workflow:");
    println!("  1. Get encoder from MTL4 command buffer:");
    println!("     let encoder = cmd_buffer.machine_learning_command_encoder();");
    println!();
    println!("  2. Set the compiled pipeline state:");
    println!("     encoder.set_pipeline_state(&pipeline_state);");
    println!();
    println!("  3. Set the argument table with bound resources:");
    println!("     encoder.set_argument_table(&arg_table);");
    println!();
    println!("  4. Dispatch the neural network:");
    println!("     encoder.dispatch_network(&intermediates_heap);");
    println!();
    println!("  5. End encoding:");
    println!("     encoder.end_encoding();");

    // =======================================================================
    // Part 7: Complete Inference Example (Pseudocode)
    // =======================================================================
    println!("\n--- Part 7: Complete Inference Example ---");

    println!("```rust");
    println!("// 1. Setup");
    println!("let device = mtl::device::system_default().unwrap();");
    println!("let queue = device.new_mtl4_command_queue().unwrap();");
    println!("let allocator = device.new_command_allocator().unwrap();");
    println!();
    println!("// 2. Load compiled ML model from library");
    println!("let library = device.new_library_with_source(ml_shader_source, None).unwrap();");
    println!("let func_desc = LibraryFunctionDescriptor::new().unwrap();");
    println!("func_desc.set_library(&library);");
    println!("func_desc.set_name(\"neural_network_kernel\");");
    println!();
    println!("// 3. Configure ML pipeline descriptor");
    println!("let ml_desc = MachineLearningPipelineDescriptor::new().unwrap();");
    println!("ml_desc.set_machine_learning_function_descriptor(&func_desc);");
    println!("// Set input tensor dimensions...");
    println!();
    println!("// 4. Compile pipeline");
    println!("let compiler_desc = CompilerDescriptor::new().unwrap();");
    println!("let compiler = device.new_compiler(&compiler_desc).unwrap();");
    println!(
        "let pipeline = compiler.new_machine_learning_pipeline_state(&ml_desc, None).unwrap();"
    );
    println!();
    println!("// 5. Create intermediates heap");
    println!("let heap_desc = HeapDescriptor::new().unwrap();");
    println!("heap_desc.set_size(pipeline.intermediates_heap_size());");
    println!("let heap = device.new_heap(&heap_desc).unwrap();");
    println!();
    println!("// 6. Create argument table and bind buffers");
    println!("let table_desc = ArgumentTableDescriptor::new().unwrap();");
    println!("table_desc.set_max_buffer_bind_count(4);");
    println!("let arg_table = device.new_argument_table(&table_desc).unwrap();");
    println!("arg_table.set_address(input_buffer.gpu_address(), 0);");
    println!("arg_table.set_address(output_buffer.gpu_address(), 1);");
    println!();
    println!("// 7. Record and execute");
    println!("let cmd_buffer = queue.new_command_buffer().unwrap();");
    println!("cmd_buffer.begin_command_buffer(&allocator);");
    println!();
    println!("let encoder = cmd_buffer.machine_learning_command_encoder().unwrap();");
    println!("encoder.set_pipeline_state(&pipeline);");
    println!("encoder.set_argument_table(&arg_table);");
    println!("encoder.dispatch_network(&heap);");
    println!("encoder.end_encoding();");
    println!();
    println!("cmd_buffer.end_command_buffer();");
    println!("queue.commit(&[&cmd_buffer]);");
    println!("```");

    // =======================================================================
    // Summary
    // =======================================================================
    println!("\n=== Summary ===");
    print_summary();

    println!("\nMetal 4 Machine Learning example completed!");
}

fn print_summary() {
    println!("Metal 4 ML Pipeline Components:");
    println!();
    println!("  MachineLearningPipelineDescriptor");
    println!("    - Configures neural network function");
    println!("    - Sets input tensor dimensions");
    println!("    - Compiled by MTL4 Compiler");
    println!();
    println!("  MachineLearningPipelineState");
    println!("    - Compiled, ready-to-execute pipeline");
    println!("    - Provides intermediates_heap_size()");
    println!("    - Contains reflection data");
    println!();
    println!("  ArgumentTable");
    println!("    - Binds input/output buffers");
    println!("    - GPU address-based binding");
    println!("    - Reusable across dispatches");
    println!();
    println!("  MachineLearningCommandEncoder");
    println!("    - Sets pipeline state");
    println!("    - Sets argument table");
    println!("    - Dispatches network execution");
    println!();
    println!("Key Benefits:");
    println!("  - Explicit memory management for intermediates");
    println!("  - Async compilation to avoid stalls");
    println!("  - Efficient resource binding via ArgumentTable");
    println!("  - Integration with MTL4 command buffer model");
}
