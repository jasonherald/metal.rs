//! Error Path Tests
//!
//! These tests verify that error conditions are handled correctly:
//! - Invalid shader compilation returns proper errors
//! - Invalid pipeline configurations fail gracefully
//! - Resource creation failures are reported correctly
//! - Command buffer errors are captured properly

use metal::{
    device, PixelFormat, ResourceOptions, StorageMode, TextureDescriptor, TextureUsage,
    ValidationError,
};
use metal_foundation::Referencing;

/// Get the default Metal device or skip the test.
fn get_device() -> metal::Device {
    device::system_default().expect("No Metal device available")
}

// =============================================================================
// Shader Compilation Errors
// =============================================================================

#[test]
fn test_invalid_shader_syntax_error() {
    let device = get_device();

    // Shader with syntax error - missing semicolon
    let bad_shader = r#"
        #include <metal_stdlib>
        using namespace metal;

        kernel void broken_kernel() {
            int x = 5  // Missing semicolon
        }
    "#;

    let result = device.new_library_with_source(bad_shader, None);
    assert!(result.is_err(), "Expected shader compilation to fail");

    if let Err(error) = result {
        // Error should contain useful information
        let code = error.code();
        println!("Shader compilation error code: {}", code);
        // Metal shader compilation errors typically have non-zero error codes
        assert_ne!(code, 0, "Error code should be non-zero for compilation failure");
    }
}

#[test]
fn test_invalid_shader_unknown_type() {
    let device = get_device();

    // Shader with unknown type
    let bad_shader = r#"
        #include <metal_stdlib>
        using namespace metal;

        kernel void broken_kernel() {
            NonExistentType x;
        }
    "#;

    let result = device.new_library_with_source(bad_shader, None);
    assert!(result.is_err(), "Expected shader compilation to fail");
}

#[test]
fn test_invalid_shader_missing_function() {
    let device = get_device();

    // Valid shader but try to get non-existent function
    let shader = r#"
        #include <metal_stdlib>
        using namespace metal;

        kernel void real_kernel() {
        }
    "#;

    let library = device
        .new_library_with_source(shader, None)
        .expect("Shader should compile");

    let result = library.new_function_with_name("non_existent_function");
    assert!(result.is_none(), "Expected function lookup to fail");
}

#[test]
fn test_empty_shader_source() {
    let device = get_device();

    let result = device.new_library_with_source("", None);
    // Empty shader should either fail or produce a library with no functions
    if let Ok(library) = result {
        let names = library.function_names();
        assert!(names.is_empty() || names.len() == 0, "Empty shader should have no functions");
    }
    // If it fails, that's also acceptable behavior
}

#[test]
fn test_shader_with_only_comments() {
    let device = get_device();

    let shader = r#"
        // This is just a comment
        /* And a block comment */
    "#;

    let result = device.new_library_with_source(shader, None);
    // Should either fail or produce empty library
    if let Ok(library) = result {
        let names = library.function_names();
        assert!(names.is_empty(), "Comment-only shader should have no functions");
    }
}

// =============================================================================
// Pipeline Creation Errors
// =============================================================================

#[test]
fn test_compute_pipeline_with_non_kernel_function() {
    let device = get_device();

    // Create a vertex function (not a kernel)
    let shader = r#"
        #include <metal_stdlib>
        using namespace metal;

        vertex float4 vertex_func(uint vid [[vertex_id]]) {
            return float4(0.0);
        }
    "#;

    let library = device
        .new_library_with_source(shader, None)
        .expect("Shader should compile");

    let function = library
        .new_function_with_name("vertex_func")
        .expect("Function should exist");

    // Try to create a compute pipeline with a vertex function
    let result = device.new_compute_pipeline_state_with_function(&function);

    // This should fail because vertex functions can't be used for compute
    assert!(
        result.is_err(),
        "Expected compute pipeline creation to fail with vertex function"
    );
}

#[test]
fn test_render_pipeline_missing_vertex_function() {
    let device = get_device();

    // Create a render pipeline descriptor without setting vertex function
    let desc = metal::RenderPipelineDescriptor::new().expect("Failed to create descriptor");

    // Don't set vertex function - leave it as None
    desc.set_vertex_function(None);

    // Set a pixel format for the color attachment
    let attachments = desc.color_attachments();
    if let Some(attachment) = attachments.object(0) {
        attachment.set_pixel_format(PixelFormat::BGRA8_UNORM);
    }

    // Use the safe validated method - should return Err, not abort
    let result = device.new_render_pipeline_state_with_descriptor(&desc);

    // Should fail with MissingVertexFunction error
    assert!(result.is_err(), "Expected render pipeline creation to fail without vertex function");
    assert!(
        matches!(result.unwrap_err(), ValidationError::MissingVertexFunction),
        "Expected MissingVertexFunction error"
    );
}

#[test]
#[ignore = "Metal validation layer aborts process instead of returning error"]
fn test_render_pipeline_missing_vertex_function_unsafe() {
    // This test documents that the unsafe method WILL abort the process.
    // Use new_render_pipeline_state_with_descriptor() instead for safe behavior.
    let device = get_device();
    let desc = metal::RenderPipelineDescriptor::new().expect("Failed to create descriptor");
    desc.set_vertex_function(None);
    let result = unsafe { device.new_render_pipeline_state(desc.as_ptr()) };
    assert!(result.is_err());
}

// =============================================================================
// Resource Creation Edge Cases
// =============================================================================

#[test]
fn test_buffer_zero_length() {
    let device = get_device();

    // Creating a zero-length buffer might succeed or fail depending on implementation
    let result = device.new_buffer(0, ResourceOptions::STORAGE_MODE_SHARED);

    // Document the behavior - zero-length buffers are technically valid in Metal
    if let Some(buffer) = result {
        assert_eq!(buffer.length(), 0, "Zero-length buffer should have length 0");
    }
    // If it fails, that's also acceptable
}

#[test]
fn test_texture_invalid_dimensions() {
    let device = get_device();

    // Try to create a texture with zero dimensions
    let desc = TextureDescriptor::texture_2d_descriptor(PixelFormat::BGRA8_UNORM, 0, 0, false);

    if let Some(desc) = desc {
        // Use the safe validated method - should return Err, not abort
        let result = device.new_texture_with_descriptor(&desc);

        // Should fail with InvalidTextureDimensions error
        assert!(result.is_err(), "Expected texture creation to fail with zero dimensions");
        assert!(
            matches!(result.unwrap_err(), ValidationError::InvalidTextureDimensions { .. }),
            "Expected InvalidTextureDimensions error"
        );
    }
}

#[test]
#[ignore = "Metal validation layer aborts process for zero dimensions"]
fn test_texture_invalid_dimensions_unsafe() {
    // This test documents that the unsafe method WILL abort the process.
    // Use new_texture_with_descriptor() instead for safe behavior.
    let device = get_device();
    let desc = TextureDescriptor::texture_2d_descriptor(PixelFormat::BGRA8_UNORM, 0, 0, false);
    if let Some(desc) = desc {
        let result = unsafe { device.new_texture(desc.as_ptr()) };
        assert!(result.is_none());
    }
}

#[test]
fn test_texture_unsupported_format_for_render_target() {
    let device = get_device();

    // Create a texture with a format that might not support render target usage
    // R8Uint is typically not supported as a render target
    let desc =
        TextureDescriptor::texture_2d_descriptor(PixelFormat::R8_UINT, 64, 64, false);

    if let Some(desc) = desc {
        // Try to use it as a render target
        desc.set_usage(TextureUsage::RENDER_TARGET);
        desc.set_storage_mode(StorageMode::PRIVATE);

        let result = unsafe { device.new_texture(desc.as_ptr()) };
        // This might succeed or fail depending on the device capabilities
        // The important thing is it doesn't crash
        if let Some(texture) = result {
            println!(
                "Texture created with format {:?}, usage {:?}",
                texture.pixel_format(),
                texture.usage()
            );
        }
    }
}

// =============================================================================
// Command Buffer Error States
// =============================================================================

#[test]
fn test_command_buffer_status_transitions() {
    let device = get_device();
    let queue = device.new_command_queue().expect("Failed to create queue");
    let cmd_buffer = queue.command_buffer().expect("Failed to create command buffer");

    // Initial status should be NotEnqueued
    let status = cmd_buffer.status();
    assert_eq!(
        status,
        metal::CommandBufferStatus::NOT_ENQUEUED,
        "Initial status should be NotEnqueued"
    );

    // Commit the buffer
    cmd_buffer.commit();

    // After commit, status should progress through Enqueued, Committed, Scheduled, Completed
    cmd_buffer.wait_until_completed();

    let final_status = cmd_buffer.status();
    assert_eq!(
        final_status,
        metal::CommandBufferStatus::COMPLETED,
        "Final status should be Completed"
    );

    // Error should be None for successful execution
    assert!(
        cmd_buffer.error().is_none(),
        "Successful command buffer should have no error"
    );
}

#[test]
fn test_command_buffer_error_after_completion() {
    let device = get_device();
    let queue = device.new_command_queue().expect("Failed to create queue");
    let cmd_buffer = queue.command_buffer().expect("Failed to create command buffer");

    // Execute an empty command buffer (should succeed)
    cmd_buffer.commit();
    cmd_buffer.wait_until_completed();

    // Check error state
    let error = cmd_buffer.error();
    assert!(error.is_none(), "Empty command buffer should complete without error");
}

// =============================================================================
// Device Query Edge Cases
// =============================================================================

#[test]
fn test_device_supports_nonexistent_sample_count() {
    let device = get_device();

    // Test an absurdly high sample count that no device supports
    let supports = device.supports_texture_sample_count(1024);
    assert!(
        !supports,
        "No device should support 1024x MSAA"
    );

    // Sample count 1 should always be supported
    let supports_1 = device.supports_texture_sample_count(1);
    assert!(supports_1, "Sample count 1 should always be supported");
}

#[test]
fn test_device_alignment_queries() {
    let device = get_device();

    // Query alignment for various pixel formats
    let formats = [
        PixelFormat::BGRA8_UNORM,
        PixelFormat::RGBA16_FLOAT,
        PixelFormat::RGBA32_FLOAT,
        PixelFormat::R8_UNORM,
    ];

    for format in formats {
        let alignment = device.minimum_linear_texture_alignment_for_pixel_format(format);
        // Alignment should be a power of 2 and non-zero
        assert!(alignment > 0, "Alignment should be positive for {:?}", format);
        assert!(
            alignment.is_power_of_two(),
            "Alignment should be power of 2 for {:?}",
            format
        );
    }
}

// =============================================================================
// Library Function Enumeration
// =============================================================================

#[test]
fn test_library_function_names() {
    let device = get_device();

    let shader = r#"
        #include <metal_stdlib>
        using namespace metal;

        kernel void kernel1() {}
        kernel void kernel2() {}
        vertex float4 vertex1(uint vid [[vertex_id]]) { return float4(0); }
        fragment float4 fragment1() { return float4(0); }
    "#;

    let library = device
        .new_library_with_source(shader, None)
        .expect("Shader should compile");

    let names = library.function_names();

    // Should have 4 functions
    assert_eq!(names.len(), 4, "Should have 4 functions");

    // Verify we can get each function
    for name in &names {
        let func = library.new_function_with_name(name);
        assert!(func.is_some(), "Should be able to get function '{}'", name);
    }
}

// =============================================================================
// Heap Allocation Errors
// =============================================================================

#[test]
fn test_heap_insufficient_size() {
    let device = get_device();

    // Create a small heap
    let heap_desc = metal::HeapDescriptor::new().expect("Failed to create heap descriptor");
    heap_desc.set_size(1024); // Very small heap
    heap_desc.set_storage_mode(StorageMode::PRIVATE);

    let heap = device.new_heap(&heap_desc);

    if let Some(heap) = heap {
        // Try to allocate a buffer larger than the heap
        let large_buffer = heap.new_buffer(1024 * 1024, ResourceOptions::STORAGE_MODE_PRIVATE);

        // Should fail due to insufficient space
        assert!(
            large_buffer.is_none(),
            "Should not be able to allocate buffer larger than heap"
        );
    }
}

// =============================================================================
// Encoder State Validation
// =============================================================================

#[test]
fn test_compute_encoder_without_pipeline() {
    let device = get_device();
    let queue = device.new_command_queue().expect("Failed to create queue");
    let cmd_buffer = queue.command_buffer().expect("Failed to create command buffer");

    let encoder_ptr = cmd_buffer.compute_command_encoder();
    let encoder =
        unsafe { metal::ComputeCommandEncoder::from_raw(encoder_ptr) }.expect("Failed to create encoder");

    // Don't set a pipeline state
    // Just end encoding - this should be fine, we just can't dispatch
    encoder.end_encoding();

    // Commit and wait - should succeed (empty encoder)
    cmd_buffer.commit();
    cmd_buffer.wait_until_completed();

    assert_eq!(
        cmd_buffer.status(),
        metal::CommandBufferStatus::COMPLETED
    );
}

// =============================================================================
// Blit Encoder Edge Cases
// =============================================================================

#[test]
fn test_fill_buffer_with_various_values() {
    let device = get_device();
    let queue = device.new_command_queue().expect("Failed to create queue");

    let buffer = device
        .new_buffer(256, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // Test filling with different byte values
    let test_values: [u8; 4] = [0x00, 0xFF, 0xAB, 0x55];

    for value in test_values {
        let cmd_buffer = queue.command_buffer().expect("Failed to create command buffer");
        let encoder_ptr = cmd_buffer.blit_command_encoder();
        let encoder =
            unsafe { metal::BlitCommandEncoder::from_raw(encoder_ptr) }.expect("Failed to create encoder");

        encoder.fill_buffer(&buffer, 0, 256, value);
        encoder.end_encoding();
        cmd_buffer.commit();
        cmd_buffer.wait_until_completed();

        // Verify all bytes are the expected value
        let contents = buffer.contents().expect("Buffer contents is null") as *const u8;
        let data = unsafe { std::slice::from_raw_parts(contents, 256) };

        for (i, &byte) in data.iter().enumerate() {
            assert_eq!(
                byte, value,
                "Byte {} should be 0x{:02X}, got 0x{:02X}",
                i, value, byte
            );
        }
    }
}

// =============================================================================
// Texture Descriptor Validation
// =============================================================================

#[test]
fn test_texture_descriptor_setters() {
    let desc = TextureDescriptor::texture_2d_descriptor(PixelFormat::BGRA8_UNORM, 256, 256, false)
        .expect("Failed to create descriptor");

    // Test various setter combinations
    desc.set_usage(TextureUsage::SHADER_READ | TextureUsage::SHADER_WRITE);
    desc.set_storage_mode(StorageMode::SHARED);

    // Verify getters return expected values
    assert_eq!(desc.width(), 256);
    assert_eq!(desc.height(), 256);
    assert_eq!(desc.pixel_format(), PixelFormat::BGRA8_UNORM);

    let usage = desc.usage();
    assert!(
        (usage.bits() & TextureUsage::SHADER_READ.bits()) != 0,
        "SHADER_READ should be set"
    );
    assert!(
        (usage.bits() & TextureUsage::SHADER_WRITE.bits()) != 0,
        "SHADER_WRITE should be set"
    );
}

// =============================================================================
// Multiple Command Buffers
// =============================================================================

#[test]
fn test_multiple_command_buffers_sequential() {
    let device = get_device();
    let queue = device.new_command_queue().expect("Failed to create queue");

    // Create and execute multiple command buffers sequentially
    for i in 0..10 {
        let cmd_buffer = queue.command_buffer().expect("Failed to create command buffer");
        cmd_buffer.set_label(&format!("Command Buffer {}", i));
        cmd_buffer.commit();
        cmd_buffer.wait_until_completed();

        assert_eq!(
            cmd_buffer.status(),
            metal::CommandBufferStatus::COMPLETED,
            "Command buffer {} should complete successfully",
            i
        );
    }
}

#[test]
fn test_multiple_command_buffers_batch_commit() {
    let device = get_device();
    let queue = device.new_command_queue().expect("Failed to create queue");

    // Create multiple command buffers
    let mut buffers = Vec::new();
    for i in 0..5 {
        let cmd_buffer = queue.command_buffer().expect("Failed to create command buffer");
        cmd_buffer.set_label(&format!("Batch Buffer {}", i));
        buffers.push(cmd_buffer);
    }

    // Commit all
    for buffer in &buffers {
        buffer.commit();
    }

    // Wait for all to complete
    for (i, buffer) in buffers.iter().enumerate() {
        buffer.wait_until_completed();
        assert_eq!(
            buffer.status(),
            metal::CommandBufferStatus::COMPLETED,
            "Batch buffer {} should complete successfully",
            i
        );
    }
}

// =============================================================================
// Validation Error Tests (Safe Methods)
// =============================================================================

#[test]
fn test_texture_invalid_mipmap_count() {
    let device = get_device();

    // Create a 64x64 texture - max mipmaps = 1 + log2(64) = 7
    let desc = TextureDescriptor::texture_2d_descriptor(PixelFormat::BGRA8_UNORM, 64, 64, false);

    if let Some(desc) = desc {
        // Try to set more mipmaps than allowed
        desc.set_mipmap_level_count(10); // Max would be 7

        let result = device.new_texture_with_descriptor(&desc);

        assert!(result.is_err(), "Expected texture creation to fail with too many mipmaps");
        assert!(
            matches!(result.unwrap_err(), ValidationError::InvalidMipmapCount { .. }),
            "Expected InvalidMipmapCount error"
        );
    }
}

#[test]
fn test_sampler_invalid_lod_range() {
    let device = get_device();

    let desc = metal::SamplerDescriptor::new().expect("Failed to create descriptor");

    // Set invalid LOD range (min > max)
    desc.set_lod_min_clamp(10.0);
    desc.set_lod_max_clamp(5.0);

    let result = device.new_sampler_state_validated(&desc);

    assert!(result.is_err(), "Expected sampler creation to fail with invalid LOD range");
    assert!(
        matches!(result.unwrap_err(), ValidationError::InvalidLodRange { .. }),
        "Expected InvalidLodRange error"
    );
}

#[test]
fn test_sampler_invalid_anisotropy() {
    let device = get_device();

    let desc = metal::SamplerDescriptor::new().expect("Failed to create descriptor");

    // Set invalid anisotropy (not a power of 2)
    desc.set_max_anisotropy(3);

    let result = device.new_sampler_state_validated(&desc);

    assert!(result.is_err(), "Expected sampler creation to fail with invalid anisotropy");
    assert!(
        matches!(result.unwrap_err(), ValidationError::InvalidAnisotropy(_)),
        "Expected InvalidAnisotropy error"
    );
}

#[test]
fn test_heap_zero_size() {
    let device = get_device();

    let desc = metal::HeapDescriptor::new().expect("Failed to create descriptor");
    desc.set_size(0);
    desc.set_storage_mode(StorageMode::PRIVATE);

    let result = device.new_heap_validated(&desc);

    assert!(result.is_err(), "Expected heap creation to fail with zero size");
    assert!(
        matches!(result.unwrap_err(), ValidationError::InvalidHeapSize),
        "Expected InvalidHeapSize error"
    );
}

#[test]
fn test_valid_texture_creation() {
    let device = get_device();

    // Create a valid texture
    let desc = TextureDescriptor::texture_2d_descriptor(PixelFormat::BGRA8_UNORM, 256, 256, false);

    if let Some(desc) = desc {
        let result = device.new_texture_with_descriptor(&desc);
        assert!(result.is_ok(), "Valid texture should be created successfully");

        let texture = result.unwrap();
        assert_eq!(texture.width(), 256);
        assert_eq!(texture.height(), 256);
    }
}

#[test]
fn test_valid_sampler_creation() {
    let device = get_device();

    let desc = metal::SamplerDescriptor::new().expect("Failed to create descriptor");
    desc.set_lod_min_clamp(0.0);
    desc.set_lod_max_clamp(1000.0);
    desc.set_max_anisotropy(16); // Power of 2

    let result = device.new_sampler_state_validated(&desc);
    assert!(result.is_ok(), "Valid sampler should be created successfully");
}

#[test]
fn test_valid_heap_creation() {
    let device = get_device();

    let desc = metal::HeapDescriptor::new().expect("Failed to create descriptor");
    desc.set_size(1024 * 1024); // 1 MB
    desc.set_storage_mode(StorageMode::PRIVATE);

    let result = device.new_heap_validated(&desc);
    assert!(result.is_ok(), "Valid heap should be created successfully");

    let heap = result.unwrap();
    assert!(heap.size() >= 1024 * 1024);
}
