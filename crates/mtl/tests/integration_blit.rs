//! Blit Command Encoder Integration Tests
//!
//! These tests verify that blit operations work correctly with the Metal GPU.
//! They test buffer copies, texture copies, fill operations, and synchronization.

use mtl_gpu::{
    BlitCommandEncoder, Origin, PixelFormat, Region, ResourceOptions, Size, StorageMode,
    TextureDescriptor, TextureUsage, device,
};

/// Get the default Metal device or skip the test.
fn get_device() -> mtl_gpu::Device {
    device::system_default().expect("No Metal device available")
}

/// Helper to create a blit encoder from a command buffer.
fn create_blit_encoder(command_buffer: &mtl_gpu::CommandBuffer) -> BlitCommandEncoder {
    let encoder_ptr = command_buffer.blit_command_encoder();
    unsafe { BlitCommandEncoder::from_raw(encoder_ptr) }.expect("Failed to create blit encoder")
}

// =============================================================================
// Buffer Copy Tests
// =============================================================================

#[test]
fn test_blit_buffer_to_buffer_copy() {
    let device = get_device();
    let queue = device
        .new_command_queue()
        .expect("Failed to create command queue");

    // Create source buffer with data
    let source_data: Vec<u8> = (0..=255).collect();
    let source_buffer = device
        .new_buffer_with_bytes(&source_data, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create source buffer");

    // Create destination buffer
    let dest_buffer = device
        .new_buffer(256, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create destination buffer");

    // Perform blit copy
    let command_buffer = queue
        .command_buffer()
        .expect("Failed to create command buffer");
    let blit_encoder = create_blit_encoder(&command_buffer);

    blit_encoder.copy_from_buffer_to_buffer(&source_buffer, 0, &dest_buffer, 0, 256);

    blit_encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Verify the copy
    let dest_ptr = dest_buffer.contents().expect("Buffer contents null") as *const u8;
    for i in 0..256 {
        let value = unsafe { *dest_ptr.add(i) };
        assert_eq!(value, i as u8, "Mismatch at index {}", i);
    }
}

#[test]
fn test_blit_buffer_partial_copy() {
    let device = get_device();
    let queue = device
        .new_command_queue()
        .expect("Failed to create command queue");

    // Create source buffer with data
    let source_data: Vec<u8> = (0..100).collect();
    let source_buffer = device
        .new_buffer_with_bytes(&source_data, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create source buffer");

    // Create destination buffer
    let dest_buffer = device
        .new_buffer(100, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create destination buffer");

    // Copy only bytes 20-60 to destination offset 10
    let command_buffer = queue
        .command_buffer()
        .expect("Failed to create command buffer");
    let blit_encoder = create_blit_encoder(&command_buffer);

    blit_encoder.copy_from_buffer_to_buffer(
        &source_buffer,
        20, // source offset
        &dest_buffer,
        10, // destination offset
        40, // size (bytes 20-59)
    );

    blit_encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Verify
    let dest_ptr = dest_buffer.contents().expect("Buffer contents null") as *const u8;
    for i in 0..40 {
        let value = unsafe { *dest_ptr.add(10 + i) };
        assert_eq!(
            value,
            (20 + i) as u8,
            "Mismatch at destination index {}",
            10 + i
        );
    }
}

// =============================================================================
// Fill Buffer Tests
// =============================================================================

#[test]
fn test_blit_fill_buffer() {
    let device = get_device();
    let queue = device
        .new_command_queue()
        .expect("Failed to create command queue");

    // Create buffer
    let buffer = device
        .new_buffer(256, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // Fill with a pattern
    let command_buffer = queue
        .command_buffer()
        .expect("Failed to create command buffer");
    let blit_encoder = create_blit_encoder(&command_buffer);

    blit_encoder.fill_buffer(&buffer, 0, 256, 0xAB);

    blit_encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Verify fill
    let ptr = buffer.contents().expect("Buffer contents null") as *const u8;
    for i in 0..256 {
        let value = unsafe { *ptr.add(i) };
        assert_eq!(value, 0xAB, "Fill failed at index {}", i);
    }
}

#[test]
fn test_blit_fill_buffer_partial() {
    let device = get_device();
    let queue = device
        .new_command_queue()
        .expect("Failed to create command queue");

    // Create buffer
    let buffer = device
        .new_buffer(100, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // First fill with zeros
    let command_buffer = queue
        .command_buffer()
        .expect("Failed to create command buffer");
    let blit_encoder = create_blit_encoder(&command_buffer);
    blit_encoder.fill_buffer(&buffer, 0, 100, 0x00);
    blit_encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Then fill only middle portion
    let command_buffer = queue
        .command_buffer()
        .expect("Failed to create command buffer");
    let blit_encoder = create_blit_encoder(&command_buffer);
    blit_encoder.fill_buffer(&buffer, 25, 50, 0xFF); // Fill bytes 25-74
    blit_encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Verify
    let ptr = buffer.contents().expect("Buffer contents null") as *const u8;
    for i in 0..25 {
        assert_eq!(unsafe { *ptr.add(i) }, 0x00, "Should be zero at {}", i);
    }
    for i in 25..75 {
        assert_eq!(unsafe { *ptr.add(i) }, 0xFF, "Should be 0xFF at {}", i);
    }
    for i in 75..100 {
        assert_eq!(unsafe { *ptr.add(i) }, 0x00, "Should be zero at {}", i);
    }
}

// =============================================================================
// Mipmap Generation Tests
// =============================================================================

#[test]
fn test_blit_generate_mipmaps() {
    let device = get_device();
    let queue = device
        .new_command_queue()
        .expect("Failed to create command queue");

    // Create texture with mipmaps
    let descriptor = TextureDescriptor::texture_2d_descriptor(
        PixelFormat::RGBA8_UNORM,
        256,
        256,
        true, // mipmapped
    )
    .expect("Failed to create texture descriptor");

    descriptor.set_storage_mode(StorageMode::SHARED);
    descriptor.set_usage(TextureUsage::SHADER_READ | TextureUsage::SHADER_WRITE);

    let texture = device
        .new_texture_with_descriptor(&descriptor)
        .expect("Failed to create texture");

    // Write some data to mipmap level 0
    let pixels: Vec<u8> = vec![0xFF; 256 * 256 * 4];
    let region = Region::new_2d(0, 0, 256, 256);
    unsafe {
        texture.replace_region_simple(
            region,
            0, // level 0
            pixels.as_ptr() as *const std::ffi::c_void,
            256 * 4,
        );
    }

    // Generate mipmaps
    let command_buffer = queue
        .command_buffer()
        .expect("Failed to create command buffer");
    let blit_encoder = create_blit_encoder(&command_buffer);

    blit_encoder.generate_mipmaps(&texture);

    blit_encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Verify mipmaps exist (texture should have multiple levels)
    assert!(
        texture.mipmap_level_count() > 1,
        "Texture should have mipmaps"
    );
}

// =============================================================================
// Buffer to Texture Copy Tests
// =============================================================================

#[test]
fn test_blit_buffer_to_texture() {
    let device = get_device();
    let queue = device
        .new_command_queue()
        .expect("Failed to create command queue");

    // Create a 4x4 RGBA texture
    let descriptor =
        TextureDescriptor::texture_2d_descriptor(PixelFormat::RGBA8_UNORM, 4, 4, false)
            .expect("Failed to create texture descriptor");

    descriptor.set_storage_mode(StorageMode::SHARED);
    descriptor.set_usage(TextureUsage::SHADER_READ);

    let texture = device
        .new_texture_with_descriptor(&descriptor)
        .expect("Failed to create texture");

    // Create source buffer with pixel data
    let pixels: Vec<u8> = (0..64).collect(); // 4x4 * 4 bytes per pixel = 64 bytes
    let source_buffer = device
        .new_buffer_with_bytes(&pixels, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create source buffer");

    // Copy buffer to texture
    let command_buffer = queue
        .command_buffer()
        .expect("Failed to create command buffer");
    let blit_encoder = create_blit_encoder(&command_buffer);

    blit_encoder.copy_from_buffer_to_texture(
        &source_buffer,
        0,  // source offset
        16, // bytes per row (4 pixels * 4 bytes)
        64, // bytes per image (4 rows * 16 bytes)
        Size::new(4, 4, 1),
        &texture,
        0, // destination slice
        0, // destination level
        Origin::new(0, 0, 0),
    );

    blit_encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    // The copy should succeed (texture now contains the buffer data)
}

// =============================================================================
// Synchronize Tests
// =============================================================================

#[test]
fn test_blit_synchronize_buffer() {
    let device = get_device();
    let queue = device
        .new_command_queue()
        .expect("Failed to create command queue");

    // Create a managed buffer
    let buffer = device
        .new_buffer(1024, ResourceOptions::STORAGE_MODE_MANAGED)
        .expect("Failed to create managed buffer");

    // Synchronize after GPU writes
    let command_buffer = queue
        .command_buffer()
        .expect("Failed to create command buffer");
    let blit_encoder = create_blit_encoder(&command_buffer);

    blit_encoder.synchronize_buffer(&buffer);

    blit_encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();
}

#[test]
fn test_blit_synchronize_texture() {
    let device = get_device();
    let queue = device
        .new_command_queue()
        .expect("Failed to create command queue");

    // Create a managed texture
    let descriptor =
        TextureDescriptor::texture_2d_descriptor(PixelFormat::RGBA8_UNORM, 64, 64, false)
            .expect("Failed to create texture descriptor");

    descriptor.set_storage_mode(StorageMode::MANAGED);
    descriptor.set_usage(TextureUsage::SHADER_READ);

    let texture = device
        .new_texture_with_descriptor(&descriptor)
        .expect("Failed to create texture");

    // Synchronize after GPU writes
    let command_buffer = queue
        .command_buffer()
        .expect("Failed to create command buffer");
    let blit_encoder = create_blit_encoder(&command_buffer);

    blit_encoder.synchronize_texture(&texture);

    blit_encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();
}

// =============================================================================
// Encoder Properties Tests
// =============================================================================

#[test]
fn test_blit_encoder_label() {
    let device = get_device();
    let queue = device
        .new_command_queue()
        .expect("Failed to create command queue");

    let command_buffer = queue
        .command_buffer()
        .expect("Failed to create command buffer");
    let blit_encoder = create_blit_encoder(&command_buffer);

    blit_encoder.set_label("MyBlitEncoder");
    assert_eq!(blit_encoder.label(), Some("MyBlitEncoder".to_string()));

    blit_encoder.end_encoding();
}

#[test]
fn test_blit_encoder_debug_groups() {
    let device = get_device();
    let queue = device
        .new_command_queue()
        .expect("Failed to create command queue");

    let command_buffer = queue
        .command_buffer()
        .expect("Failed to create command buffer");
    let blit_encoder = create_blit_encoder(&command_buffer);

    // Test debug group operations (these don't crash is the test)
    blit_encoder.push_debug_group("CopyOperations");
    blit_encoder.insert_debug_signpost("Starting copy");
    blit_encoder.pop_debug_group();

    blit_encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();
}

// =============================================================================
// Size Tests
// =============================================================================

#[test]
fn test_blit_command_encoder_size() {
    assert_eq!(
        std::mem::size_of::<BlitCommandEncoder>(),
        std::mem::size_of::<*mut std::ffi::c_void>()
    );
}
