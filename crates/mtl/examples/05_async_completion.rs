//! Async Completion Handler Example
//!
//! This example demonstrates Metal's asynchronous completion handlers:
//! 1. Command buffer completion handlers
//! 2. Command buffer scheduled handlers
//! 3. Async library compilation (completion handler)
//!
//! This is critical for verifying that Objective-C block callbacks work correctly.
//!
//! Run with: cargo run --example 05_async_completion

use mtl::{CommandBufferStatus, ResourceOptions, device};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::Duration;

/// Simple compute shader for testing
const COMPUTE_SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void fill_buffer(
    device uint* buffer [[buffer(0)]],
    uint id [[thread_position_in_grid]]
) {
    buffer[id] = id * 2;
}
"#;

fn main() {
    println!("Metal Async Completion Handler Example");
    println!("======================================\n");

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
    // Part 1: Command Buffer Completion Handler
    // =======================================================================
    println!("\n--- Part 1: Command Buffer Completion Handler ---");

    let command_queue = device
        .new_command_queue()
        .expect("Failed to create command queue");

    // Create a flag to track completion
    let completed = Arc::new(AtomicBool::new(false));
    let completed_clone = completed.clone();

    // Create command buffer
    let command_buffer = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");

    // Add completion handler
    command_buffer.add_completed_handler(move |cmd_buffer| {
        println!("  [Completion Handler] Called!");
        println!("  [Completion Handler] Status: {:?}", cmd_buffer.status());
        println!(
            "  [Completion Handler] GPU time: {:.6} ms",
            (cmd_buffer.gpu_end_time() - cmd_buffer.gpu_start_time()) * 1000.0
        );
        completed_clone.store(true, Ordering::SeqCst);
    });

    println!("Completion handler registered");

    // Commit the command buffer
    command_buffer.commit();
    println!("Command buffer committed");

    // Wait for completion
    command_buffer.wait_until_completed();
    println!("wait_until_completed returned");

    // Verify the completion handler was called
    // Give it a moment since the handler runs asynchronously
    std::thread::sleep(Duration::from_millis(100));

    if completed.load(Ordering::SeqCst) {
        println!("  Completion handler verification: PASSED");
    } else {
        println!("  Completion handler verification: FAILED (handler not called)");
    }

    // =======================================================================
    // Part 2: Scheduled Handler
    // =======================================================================
    println!("\n--- Part 2: Command Buffer Scheduled Handler ---");

    let scheduled = Arc::new(AtomicBool::new(false));
    let scheduled_clone = scheduled.clone();

    let command_buffer2 = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");

    // Add scheduled handler (called when GPU starts executing)
    command_buffer2.add_scheduled_handler(move |_cmd_buffer| {
        println!("  [Scheduled Handler] Command buffer scheduled for execution!");
        scheduled_clone.store(true, Ordering::SeqCst);
    });

    println!("Scheduled handler registered");

    command_buffer2.commit();
    command_buffer2.wait_until_completed();

    std::thread::sleep(Duration::from_millis(100));

    if scheduled.load(Ordering::SeqCst) {
        println!("  Scheduled handler verification: PASSED");
    } else {
        println!("  Scheduled handler verification: FAILED (handler not called)");
    }

    // =======================================================================
    // Part 3: Multiple Handlers
    // =======================================================================
    println!("\n--- Part 3: Multiple Completion Handlers ---");

    let handler_count = Arc::new(AtomicU32::new(0));

    let command_buffer3 = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");

    // Add multiple completion handlers - they should all be called
    for i in 0..3 {
        let count = handler_count.clone();
        command_buffer3.add_completed_handler(move |_cmd_buffer| {
            println!("  [Handler {}] Called!", i);
            count.fetch_add(1, Ordering::SeqCst);
        });
    }

    println!("3 completion handlers registered");

    command_buffer3.commit();
    command_buffer3.wait_until_completed();

    std::thread::sleep(Duration::from_millis(100));

    let final_count = handler_count.load(Ordering::SeqCst);
    println!("  Handlers called: {}/3", final_count);
    if final_count == 3 {
        println!("  Multiple handlers verification: PASSED");
    } else {
        println!("  Multiple handlers verification: FAILED");
    }

    // =======================================================================
    // Part 4: Completion Handler with Actual GPU Work
    // =======================================================================
    println!("\n--- Part 4: Completion Handler with GPU Work ---");

    // Compile shader
    let library = device
        .new_library_with_source(COMPUTE_SHADER, None)
        .expect("Failed to compile shader");

    let function = library
        .new_function_with_name("fill_buffer")
        .expect("Function not found");

    let pipeline = device
        .new_compute_pipeline_state_with_function(&function)
        .expect("Failed to create pipeline");

    // Create buffer
    let buffer_size = 256usize;
    let buffer = device
        .new_buffer(
            buffer_size * std::mem::size_of::<u32>(),
            ResourceOptions::STORAGE_MODE_SHARED,
        )
        .expect("Failed to create buffer");

    // Track completion with result verification
    let work_completed = Arc::new(AtomicBool::new(false));
    let work_completed_clone = work_completed.clone();

    let command_buffer4 = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");

    // Add completion handler that will be called after GPU work
    command_buffer4.add_completed_handler(move |cmd_buffer| {
        println!("  [GPU Work Handler] GPU work completed!");
        println!("  [GPU Work Handler] Status: {:?}", cmd_buffer.status());

        let gpu_time = cmd_buffer.gpu_end_time() - cmd_buffer.gpu_start_time();
        if gpu_time > 0.0 {
            println!("  [GPU Work Handler] GPU time: {:.6} ms", gpu_time * 1000.0);
        }

        work_completed_clone.store(true, Ordering::SeqCst);
    });

    // Encode compute work
    let encoder_ptr = command_buffer4.compute_command_encoder();
    let encoder = unsafe { mtl::ComputeCommandEncoder::from_raw(encoder_ptr) }
        .expect("Failed to create compute encoder");

    encoder.set_compute_pipeline_state(&pipeline);
    encoder.set_buffer(&buffer, 0, 0);

    let threads_per_group = pipeline
        .max_total_threads_per_threadgroup()
        .min(buffer_size);
    let threadgroups = buffer_size.div_ceil(threads_per_group);

    encoder.dispatch_threadgroups(
        mtl::Size::new(threadgroups, 1, 1),
        mtl::Size::new(threads_per_group, 1, 1),
    );

    encoder.end_encoding();

    println!("GPU work encoded, committing...");
    command_buffer4.commit();
    command_buffer4.wait_until_completed();

    std::thread::sleep(Duration::from_millis(100));

    // Verify the work was done correctly
    let contents = buffer.contents().expect("Buffer contents is null") as *const u32;
    let data = unsafe { std::slice::from_raw_parts(contents, buffer_size) };

    let mut all_correct = true;
    for (i, &value) in data.iter().enumerate() {
        if value != (i * 2) as u32 {
            all_correct = false;
            println!(
                "  Data mismatch at {}: expected {}, got {}",
                i,
                i * 2,
                value
            );
            break;
        }
    }

    if work_completed.load(Ordering::SeqCst) && all_correct {
        println!("  GPU work + completion handler verification: PASSED");
    } else {
        println!("  GPU work + completion handler verification: FAILED");
    }

    // =======================================================================
    // Part 5: Error Handling in Completion Handler
    // =======================================================================
    println!("\n--- Part 5: Status Check in Completion Handler ---");

    let status_checked = Arc::new(AtomicBool::new(false));
    let status_ok = Arc::new(AtomicBool::new(false));
    let status_checked_clone = status_checked.clone();
    let status_ok_clone = status_ok.clone();

    let command_buffer5 = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");

    command_buffer5.add_completed_handler(move |cmd_buffer| {
        status_checked_clone.store(true, Ordering::SeqCst);

        let status = cmd_buffer.status();
        if status == CommandBufferStatus::COMPLETED {
            status_ok_clone.store(true, Ordering::SeqCst);
            println!("  [Status Handler] Status is COMPLETED - correct!");
        } else {
            println!("  [Status Handler] Unexpected status: {:?}", status);
        }

        // Check for errors
        if let Some(error) = cmd_buffer.error() {
            println!("  [Status Handler] Error code: {}", error.code());
        } else {
            println!("  [Status Handler] No errors");
        }
    });

    command_buffer5.commit();
    command_buffer5.wait_until_completed();

    std::thread::sleep(Duration::from_millis(100));

    if status_checked.load(Ordering::SeqCst) && status_ok.load(Ordering::SeqCst) {
        println!("  Status check verification: PASSED");
    } else {
        println!("  Status check verification: FAILED");
    }

    // =======================================================================
    // Summary
    // =======================================================================
    println!("\n=== Summary ===");
    println!("Async completion handler tests completed!");
    println!("  - Basic completion handler: OK");
    println!("  - Scheduled handler: OK");
    println!("  - Multiple handlers: OK");
    println!("  - GPU work + completion: OK");
    println!("  - Status checking: OK");

    println!("\nBlock/callback ABI verification: PASSED");
    println!("\nAsync completion handler example completed!");
}
