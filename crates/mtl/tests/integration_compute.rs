//! Compute Pipeline Integration Tests
//!
//! These tests verify that compute pipeline operations work correctly with the Metal GPU.
//! They test real GPU operations including shader compilation, pipeline creation, and dispatch.

use mtl_gpu::{ComputeCommandEncoder, ComputePipelineState, ResourceOptions, Size, device};

/// Get the default Metal device or skip the test.
fn get_device() -> mtl_gpu::Device {
    device::system_default().expect("No Metal device available")
}

// =============================================================================
// Shader Compilation Tests
// =============================================================================

/// Simple shader that does nothing but is valid MSL
const EMPTY_KERNEL: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void empty_kernel() {
}
"#;

/// Shader that multiplies buffer elements by 2
const MULTIPLY_KERNEL: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void multiply_by_two(
    device float* data [[buffer(0)]],
    uint id [[thread_position_in_grid]]
) {
    data[id] = data[id] * 2.0;
}
"#;

/// Shader that adds two buffers
const ADD_KERNEL: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void add_buffers(
    device const float* a [[buffer(0)]],
    device const float* b [[buffer(1)]],
    device float* result [[buffer(2)]],
    uint id [[thread_position_in_grid]]
) {
    result[id] = a[id] + b[id];
}
"#;

/// Shader using threadgroup memory (reserved for future threadgroup tests)
#[allow(dead_code)]
const REDUCTION_KERNEL: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void sum_reduce(
    device const float* input [[buffer(0)]],
    device float* output [[buffer(1)]],
    uint id [[thread_position_in_grid]],
    uint local_id [[thread_position_in_threadgroup]],
    uint group_id [[threadgroup_position_in_grid]],
    threadgroup float* shared [[threadgroup(0)]]
) {
    shared[local_id] = input[id];
    threadgroup_barrier(mem_flags::mem_threadgroup);

    // Simple reduction (for testing, not optimal)
    if (local_id == 0) {
        float sum = 0.0;
        for (uint i = 0; i < 32; i++) {
            sum += shared[i];
        }
        output[group_id] = sum;
    }
}
"#;

#[test]
fn test_compile_empty_kernel() {
    let device = get_device();

    let library = device
        .new_library_with_source(EMPTY_KERNEL, None)
        .expect("Failed to compile empty kernel");

    let function = library
        .new_function_with_name("empty_kernel")
        .expect("Function not found");

    assert_eq!(function.name(), Some("empty_kernel".to_string()));
}

#[test]
fn test_compile_multiply_kernel() {
    let device = get_device();

    let library = device
        .new_library_with_source(MULTIPLY_KERNEL, None)
        .expect("Failed to compile multiply kernel");

    let function = library
        .new_function_with_name("multiply_by_two")
        .expect("Function not found");

    assert_eq!(function.name(), Some("multiply_by_two".to_string()));
}

#[test]
fn test_compile_invalid_shader() {
    let device = get_device();

    let invalid_source = r#"
    kernel void broken( {
        // Syntax error
    }
    "#;

    let result = device.new_library_with_source(invalid_source, None);
    assert!(result.is_err(), "Invalid shader should fail to compile");
}

#[test]
fn test_function_not_found() {
    let device = get_device();

    let library = device
        .new_library_with_source(EMPTY_KERNEL, None)
        .expect("Failed to compile");

    let result = library.new_function_with_name("nonexistent_function");
    assert!(result.is_none(), "Nonexistent function should return None");
}

// =============================================================================
// Pipeline Creation Tests
// =============================================================================

#[test]
fn test_create_compute_pipeline() {
    let device = get_device();

    let library = device
        .new_library_with_source(MULTIPLY_KERNEL, None)
        .expect("Failed to compile");

    let function = library
        .new_function_with_name("multiply_by_two")
        .expect("Function not found");

    let pipeline: ComputePipelineState = device
        .new_compute_pipeline_state_with_function(&function)
        .expect("Failed to create pipeline");

    // Pipeline should have valid properties
    assert!(pipeline.max_total_threads_per_threadgroup() > 0);
    assert!(pipeline.thread_execution_width() > 0);
}

#[test]
fn test_pipeline_properties() {
    let device = get_device();

    let library = device
        .new_library_with_source(MULTIPLY_KERNEL, None)
        .expect("Failed to compile");

    let function = library
        .new_function_with_name("multiply_by_two")
        .expect("Function not found");

    let pipeline: ComputePipelineState = device
        .new_compute_pipeline_state_with_function(&function)
        .expect("Failed to create pipeline");

    // Check that pipeline reports reasonable values
    let max_threads = pipeline.max_total_threads_per_threadgroup();
    let thread_width = pipeline.thread_execution_width();

    // These should be powers of 2 and > 0
    assert!(max_threads >= 32, "Max threads should be at least 32");
    assert!(
        thread_width >= 32,
        "Thread execution width should be at least 32"
    );

    // Thread width should divide max threads
    assert_eq!(max_threads % thread_width, 0);
}

// =============================================================================
// Command Queue and Buffer Tests
// =============================================================================

#[test]
fn test_create_command_queue() {
    let device = get_device();

    let queue = device
        .new_command_queue()
        .expect("Failed to create command queue");

    // Queue should reference the same device
    let queue_device = queue.device();
    assert_eq!(queue_device.registry_id(), device.registry_id());
}

#[test]
fn test_create_command_buffer() {
    let device = get_device();

    let queue = device
        .new_command_queue()
        .expect("Failed to create command queue");
    let _command_buffer = queue
        .command_buffer()
        .expect("Failed to create command buffer");

    // Command buffer was created successfully
    // (status check would require matching enum values)
}

// =============================================================================
// Compute Dispatch Tests
// =============================================================================

#[test]
fn test_dispatch_multiply_kernel() {
    let device = get_device();
    let element_count = 64usize;

    // Create input buffer with data
    let input_data: Vec<f32> = (0..element_count).map(|i| i as f32).collect();
    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            input_data.as_ptr() as *const u8,
            input_data.len() * std::mem::size_of::<f32>(),
        )
    };
    let buffer = device
        .new_buffer_with_bytes(bytes, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // Compile shader and create pipeline
    let library = device
        .new_library_with_source(MULTIPLY_KERNEL, None)
        .expect("Failed to compile");
    let function = library
        .new_function_with_name("multiply_by_two")
        .expect("Function not found");
    let pipeline: ComputePipelineState = device
        .new_compute_pipeline_state_with_function(&function)
        .expect("Failed to create pipeline");

    // Create command queue and buffer
    let command_queue = device.new_command_queue().expect("Failed to create queue");
    let command_buffer = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");

    // Encode compute commands
    let encoder_ptr = command_buffer.compute_command_encoder();
    let encoder =
        unsafe { ComputeCommandEncoder::from_raw(encoder_ptr) }.expect("Failed to create encoder");

    encoder.set_compute_pipeline_state(&pipeline);
    encoder.set_buffer(&buffer, 0, 0);

    // Dispatch threads
    let thread_width = pipeline.thread_execution_width();
    let threadgroup_count = element_count.div_ceil(thread_width);

    encoder.dispatch_threadgroups(
        Size::new(threadgroup_count, 1, 1),
        Size::new(thread_width, 1, 1),
    );

    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Verify results
    let result_ptr = buffer.contents().expect("Buffer contents null") as *const f32;
    let results: Vec<f32> =
        unsafe { std::slice::from_raw_parts(result_ptr, element_count).to_vec() };

    let expected: Vec<f32> = (0..element_count).map(|i| (i as f32) * 2.0).collect();
    assert_eq!(results, expected);
}

#[test]
fn test_dispatch_add_buffers_kernel() {
    let device = get_device();
    let element_count = 32usize;

    // Create input buffers
    let data_a: Vec<f32> = (0..element_count).map(|i| i as f32).collect();
    let data_b: Vec<f32> = (0..element_count).map(|i| (i as f32) * 10.0).collect();

    let bytes_a: &[u8] = unsafe {
        std::slice::from_raw_parts(
            data_a.as_ptr() as *const u8,
            data_a.len() * std::mem::size_of::<f32>(),
        )
    };
    let bytes_b: &[u8] = unsafe {
        std::slice::from_raw_parts(
            data_b.as_ptr() as *const u8,
            data_b.len() * std::mem::size_of::<f32>(),
        )
    };

    let buffer_a = device
        .new_buffer_with_bytes(bytes_a, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer A");
    let buffer_b = device
        .new_buffer_with_bytes(bytes_b, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer B");
    let buffer_result = device
        .new_buffer(
            element_count * std::mem::size_of::<f32>(),
            ResourceOptions::STORAGE_MODE_SHARED,
        )
        .expect("Failed to create result buffer");

    // Compile and create pipeline
    let library = device
        .new_library_with_source(ADD_KERNEL, None)
        .expect("Failed to compile");
    let function = library
        .new_function_with_name("add_buffers")
        .expect("Function not found");
    let pipeline: ComputePipelineState = device
        .new_compute_pipeline_state_with_function(&function)
        .expect("Failed to create pipeline");

    // Dispatch
    let command_queue = device.new_command_queue().expect("Failed to create queue");
    let command_buffer = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");

    let encoder_ptr = command_buffer.compute_command_encoder();
    let encoder =
        unsafe { ComputeCommandEncoder::from_raw(encoder_ptr) }.expect("Failed to create encoder");

    encoder.set_compute_pipeline_state(&pipeline);
    encoder.set_buffer(&buffer_a, 0, 0);
    encoder.set_buffer(&buffer_b, 0, 1);
    encoder.set_buffer(&buffer_result, 0, 2);

    let thread_width = pipeline.thread_execution_width();
    let threadgroup_count = element_count.div_ceil(thread_width);

    encoder.dispatch_threadgroups(
        Size::new(threadgroup_count, 1, 1),
        Size::new(thread_width, 1, 1),
    );

    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Verify results
    let result_ptr = buffer_result.contents().expect("Buffer contents null") as *const f32;
    let results: Vec<f32> =
        unsafe { std::slice::from_raw_parts(result_ptr, element_count).to_vec() };

    let expected: Vec<f32> = (0..element_count)
        .map(|i| (i as f32) + (i as f32) * 10.0)
        .collect();
    assert_eq!(results, expected);
}

#[test]
fn test_dispatch_large_buffer() {
    let device = get_device();
    let element_count = 1024 * 64usize; // 64K elements

    // Create large input buffer
    let input_data: Vec<f32> = (0..element_count).map(|i| (i % 1000) as f32).collect();
    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            input_data.as_ptr() as *const u8,
            input_data.len() * std::mem::size_of::<f32>(),
        )
    };
    let buffer = device
        .new_buffer_with_bytes(bytes, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // Compile and create pipeline
    let library = device
        .new_library_with_source(MULTIPLY_KERNEL, None)
        .expect("Failed to compile");
    let function = library
        .new_function_with_name("multiply_by_two")
        .expect("Function not found");
    let pipeline: ComputePipelineState = device
        .new_compute_pipeline_state_with_function(&function)
        .expect("Failed to create pipeline");

    // Dispatch
    let command_queue = device.new_command_queue().expect("Failed to create queue");
    let command_buffer = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");

    let encoder_ptr = command_buffer.compute_command_encoder();
    let encoder =
        unsafe { ComputeCommandEncoder::from_raw(encoder_ptr) }.expect("Failed to create encoder");

    encoder.set_compute_pipeline_state(&pipeline);
    encoder.set_buffer(&buffer, 0, 0);

    let thread_width = pipeline.thread_execution_width();
    let threadgroup_count = element_count.div_ceil(thread_width);

    encoder.dispatch_threadgroups(
        Size::new(threadgroup_count, 1, 1),
        Size::new(thread_width, 1, 1),
    );

    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Verify results (spot check)
    let result_ptr = buffer.contents().expect("Buffer contents null") as *const f32;
    let results = unsafe { std::slice::from_raw_parts(result_ptr, element_count) };

    // Check first, middle, and last elements
    assert_eq!(results[0], 0.0);
    assert_eq!(results[1], 2.0);
    assert_eq!(
        results[element_count / 2],
        ((element_count / 2) % 1000) as f32 * 2.0
    );
    assert_eq!(
        results[element_count - 1],
        ((element_count - 1) % 1000) as f32 * 2.0
    );
}

// =============================================================================
// Multiple Dispatch Tests
// =============================================================================

#[test]
fn test_multiple_dispatches_same_buffer() {
    let device = get_device();
    let element_count = 32usize;

    // Create buffer with initial data
    let input_data: Vec<f32> = (0..element_count).map(|i| i as f32).collect();
    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            input_data.as_ptr() as *const u8,
            input_data.len() * std::mem::size_of::<f32>(),
        )
    };
    let buffer = device
        .new_buffer_with_bytes(bytes, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // Compile and create pipeline
    let library = device
        .new_library_with_source(MULTIPLY_KERNEL, None)
        .expect("Failed to compile");
    let function = library
        .new_function_with_name("multiply_by_two")
        .expect("Function not found");
    let pipeline: ComputePipelineState = device
        .new_compute_pipeline_state_with_function(&function)
        .expect("Failed to create pipeline");

    let command_queue = device.new_command_queue().expect("Failed to create queue");

    // Run the kernel 3 times (multiply by 8 total)
    for _ in 0..3 {
        let command_buffer = command_queue
            .command_buffer()
            .expect("Failed to create command buffer");

        let encoder_ptr = command_buffer.compute_command_encoder();
        let encoder = unsafe { ComputeCommandEncoder::from_raw(encoder_ptr) }
            .expect("Failed to create encoder");

        encoder.set_compute_pipeline_state(&pipeline);
        encoder.set_buffer(&buffer, 0, 0);

        let thread_width = pipeline.thread_execution_width();
        let threadgroup_count = element_count.div_ceil(thread_width);

        encoder.dispatch_threadgroups(
            Size::new(threadgroup_count, 1, 1),
            Size::new(thread_width, 1, 1),
        );

        encoder.end_encoding();
        command_buffer.commit();
        command_buffer.wait_until_completed();
    }

    // Verify results (each value multiplied by 2^3 = 8)
    let result_ptr = buffer.contents().expect("Buffer contents null") as *const f32;
    let results: Vec<f32> =
        unsafe { std::slice::from_raw_parts(result_ptr, element_count).to_vec() };

    let expected: Vec<f32> = (0..element_count).map(|i| (i as f32) * 8.0).collect();
    assert_eq!(results, expected);
}

#[test]
fn test_multiple_dispatches_single_command_buffer() {
    let device = get_device();
    let element_count = 32usize;

    // Create buffer
    let input_data: Vec<f32> = (0..element_count).map(|i| i as f32).collect();
    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            input_data.as_ptr() as *const u8,
            input_data.len() * std::mem::size_of::<f32>(),
        )
    };
    let buffer = device
        .new_buffer_with_bytes(bytes, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // Compile and create pipeline
    let library = device
        .new_library_with_source(MULTIPLY_KERNEL, None)
        .expect("Failed to compile");
    let function = library
        .new_function_with_name("multiply_by_two")
        .expect("Function not found");
    let pipeline: ComputePipelineState = device
        .new_compute_pipeline_state_with_function(&function)
        .expect("Failed to create pipeline");

    let command_queue = device.new_command_queue().expect("Failed to create queue");
    let command_buffer = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");

    // Encode 3 dispatches in the same command buffer
    for _ in 0..3 {
        let encoder_ptr = command_buffer.compute_command_encoder();
        let encoder = unsafe { ComputeCommandEncoder::from_raw(encoder_ptr) }
            .expect("Failed to create encoder");

        encoder.set_compute_pipeline_state(&pipeline);
        encoder.set_buffer(&buffer, 0, 0);

        let thread_width = pipeline.thread_execution_width();
        let threadgroup_count = element_count.div_ceil(thread_width);

        encoder.dispatch_threadgroups(
            Size::new(threadgroup_count, 1, 1),
            Size::new(thread_width, 1, 1),
        );

        encoder.end_encoding();
    }

    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Verify results (each value multiplied by 2^3 = 8)
    let result_ptr = buffer.contents().expect("Buffer contents null") as *const f32;
    let results: Vec<f32> =
        unsafe { std::slice::from_raw_parts(result_ptr, element_count).to_vec() };

    let expected: Vec<f32> = (0..element_count).map(|i| (i as f32) * 8.0).collect();
    assert_eq!(results, expected);
}

// =============================================================================
// Command Buffer Timing Tests
// =============================================================================

#[test]
fn test_command_buffer_timing() {
    let device = get_device();
    let element_count = 1024usize;

    // Create buffer
    let input_data: Vec<f32> = (0..element_count).map(|i| i as f32).collect();
    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            input_data.as_ptr() as *const u8,
            input_data.len() * std::mem::size_of::<f32>(),
        )
    };
    let buffer = device
        .new_buffer_with_bytes(bytes, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // Compile and create pipeline
    let library = device
        .new_library_with_source(MULTIPLY_KERNEL, None)
        .expect("Failed to compile");
    let function = library
        .new_function_with_name("multiply_by_two")
        .expect("Function not found");
    let pipeline: ComputePipelineState = device
        .new_compute_pipeline_state_with_function(&function)
        .expect("Failed to create pipeline");

    let command_queue = device.new_command_queue().expect("Failed to create queue");
    let command_buffer = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");

    let encoder_ptr = command_buffer.compute_command_encoder();
    let encoder =
        unsafe { ComputeCommandEncoder::from_raw(encoder_ptr) }.expect("Failed to create encoder");

    encoder.set_compute_pipeline_state(&pipeline);
    encoder.set_buffer(&buffer, 0, 0);

    let thread_width = pipeline.thread_execution_width();
    let threadgroup_count = element_count.div_ceil(thread_width);

    encoder.dispatch_threadgroups(
        Size::new(threadgroup_count, 1, 1),
        Size::new(thread_width, 1, 1),
    );

    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Check timing (should be > 0 after completion)
    let gpu_start = command_buffer.gpu_start_time();
    let gpu_end = command_buffer.gpu_end_time();
    let kernel_start = command_buffer.kernel_start_time();
    let kernel_end = command_buffer.kernel_end_time();

    // GPU times should be valid after completion
    // Note: All times should be non-negative
    assert!(gpu_start >= 0.0, "GPU start time should be >= 0");
    assert!(gpu_end >= 0.0, "GPU end time should be >= 0");
    assert!(kernel_start >= 0.0, "Kernel start time should be >= 0");
    assert!(kernel_end >= 0.0, "Kernel end time should be >= 0");

    // GPU end should be >= GPU start (if both are non-zero)
    if gpu_start > 0.0 && gpu_end > 0.0 {
        assert!(gpu_end >= gpu_start, "GPU end should be >= GPU start");
    }
}

// =============================================================================
// Pipeline Label Tests
// =============================================================================

#[test]
fn test_compute_encoder_label() {
    let device = get_device();

    let library = device
        .new_library_with_source(EMPTY_KERNEL, None)
        .expect("Failed to compile");
    let function = library
        .new_function_with_name("empty_kernel")
        .expect("Function not found");
    let pipeline: ComputePipelineState = device
        .new_compute_pipeline_state_with_function(&function)
        .expect("Failed to create pipeline");

    let command_queue = device.new_command_queue().expect("Failed to create queue");
    let command_buffer = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");

    let encoder_ptr = command_buffer.compute_command_encoder();
    let encoder =
        unsafe { ComputeCommandEncoder::from_raw(encoder_ptr) }.expect("Failed to create encoder");

    encoder.set_label("Test Compute Pass");

    encoder.set_compute_pipeline_state(&pipeline);
    encoder.dispatch_threadgroups(Size::new(1, 1, 1), Size::new(1, 1, 1));
    encoder.end_encoding();

    command_buffer.commit();
    command_buffer.wait_until_completed();
}

// =============================================================================
// Edge Case Tests
// =============================================================================

#[test]
fn test_dispatch_single_thread() {
    let device = get_device();

    // Single element buffer
    let input_data: Vec<f32> = vec![42.0];
    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            input_data.as_ptr() as *const u8,
            input_data.len() * std::mem::size_of::<f32>(),
        )
    };
    let buffer = device
        .new_buffer_with_bytes(bytes, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    let library = device
        .new_library_with_source(MULTIPLY_KERNEL, None)
        .expect("Failed to compile");
    let function = library
        .new_function_with_name("multiply_by_two")
        .expect("Function not found");
    let pipeline: ComputePipelineState = device
        .new_compute_pipeline_state_with_function(&function)
        .expect("Failed to create pipeline");

    let command_queue = device.new_command_queue().expect("Failed to create queue");
    let command_buffer = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");

    let encoder_ptr = command_buffer.compute_command_encoder();
    let encoder =
        unsafe { ComputeCommandEncoder::from_raw(encoder_ptr) }.expect("Failed to create encoder");

    encoder.set_compute_pipeline_state(&pipeline);
    encoder.set_buffer(&buffer, 0, 0);

    // Dispatch exactly 1 thread
    encoder.dispatch_threadgroups(Size::new(1, 1, 1), Size::new(1, 1, 1));

    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    let result_ptr = buffer.contents().expect("Buffer contents null") as *const f32;
    let result = unsafe { *result_ptr };
    assert_eq!(result, 84.0); // 42.0 * 2.0
}
