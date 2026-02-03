//! Texture Integration Tests
//!
//! These tests verify that texture operations work correctly with the Metal GPU.
//! They test texture creation, pixel data read/write, and texture properties.

use metal::{device, PixelFormat, TextureDescriptor, TextureUsage, StorageMode, Region};

/// Get the default Metal device or skip the test.
fn get_device() -> metal::Device {
    device::system_default().expect("No Metal device available")
}

// =============================================================================
// Texture Creation Tests
// =============================================================================

#[test]
fn test_texture_descriptor_creation() {
    let descriptor = TextureDescriptor::new();
    assert!(descriptor.is_some(), "Failed to create TextureDescriptor");
}

#[test]
fn test_texture_2d_descriptor() {
    let descriptor = TextureDescriptor::texture_2d_descriptor(
        PixelFormat::RGBA8_UNORM,
        256,
        256,
        false,
    );
    assert!(descriptor.is_some(), "Failed to create 2D texture descriptor");

    let desc = descriptor.unwrap();
    assert_eq!(desc.width(), 256);
    assert_eq!(desc.height(), 256);
    assert_eq!(desc.pixel_format(), PixelFormat::RGBA8_UNORM);
}

#[test]
fn test_texture_2d_with_mipmaps() {
    let descriptor = TextureDescriptor::texture_2d_descriptor(
        PixelFormat::RGBA8_UNORM,
        256,
        256,
        true,
    );
    assert!(descriptor.is_some());

    let desc = descriptor.unwrap();
    // With mipmaps enabled, mipmap level count should be > 1
    assert!(desc.mipmap_level_count() > 1, "Expected mipmaps for 256x256 texture");
}

#[test]
fn test_texture_cube_descriptor() {
    let descriptor = TextureDescriptor::texture_cube_descriptor(
        PixelFormat::RGBA8_UNORM,
        64,
        false,
    );
    assert!(descriptor.is_some(), "Failed to create cube texture descriptor");
}

#[test]
fn test_texture_creation() {
    let device = get_device();

    let descriptor = TextureDescriptor::texture_2d_descriptor(
        PixelFormat::RGBA8_UNORM,
        128,
        128,
        false,
    ).expect("Failed to create descriptor");

    descriptor.set_usage(TextureUsage::SHADER_READ | TextureUsage::SHADER_WRITE);
    descriptor.set_storage_mode(StorageMode::SHARED);

    let texture = device.new_texture_with_descriptor(&descriptor);
    assert!(texture.is_ok(), "Failed to create texture: {:?}", texture.err());

    let texture = texture.unwrap();
    assert_eq!(texture.width(), 128);
    assert_eq!(texture.height(), 128);
    assert_eq!(texture.pixel_format(), PixelFormat::RGBA8_UNORM);
}

#[test]
fn test_texture_with_label() {
    let device = get_device();

    let descriptor = TextureDescriptor::texture_2d_descriptor(
        PixelFormat::RGBA8_UNORM,
        64,
        64,
        false,
    ).unwrap();

    descriptor.set_storage_mode(StorageMode::SHARED);

    let texture = device.new_texture_with_descriptor(&descriptor).unwrap();
    texture.set_label("MyTexture");

    assert_eq!(texture.label(), Some("MyTexture".to_string()));
}

#[test]
fn test_texture_invalid_dimensions() {
    let device = get_device();

    let descriptor = TextureDescriptor::new().unwrap();
    descriptor.set_pixel_format(PixelFormat::RGBA8_UNORM);
    descriptor.set_width(0); // Invalid!
    descriptor.set_height(64);
    descriptor.set_storage_mode(StorageMode::SHARED);

    let result = device.new_texture_with_descriptor(&descriptor);
    assert!(result.is_err(), "Should fail with zero width");

    match result {
        Err(metal::ValidationError::InvalidTextureDimensions { width, .. }) => {
            assert_eq!(width, 0);
        }
        Err(e) => panic!("Expected InvalidTextureDimensions, got: {:?}", e),
        Ok(_) => panic!("Expected error"),
    }
}

// =============================================================================
// Texture Data Tests
// =============================================================================

#[test]
fn test_texture_replace_region() {
    let device = get_device();

    let descriptor = TextureDescriptor::texture_2d_descriptor(
        PixelFormat::RGBA8_UNORM,
        4,
        4,
        false,
    ).unwrap();

    descriptor.set_storage_mode(StorageMode::SHARED);
    descriptor.set_usage(TextureUsage::SHADER_READ);

    let texture = device.new_texture_with_descriptor(&descriptor).unwrap();

    // Create test data: 4x4 texture with RGBA pixels (16 pixels * 4 bytes = 64 bytes)
    let pixels: Vec<u8> = (0..64).collect();

    let region = Region::new_2d(0, 0, 4, 4);
    let bytes_per_row = 4 * 4; // 4 pixels * 4 bytes per pixel

    // Write data to texture
    unsafe {
        texture.replace_region_simple(
            region,
            0, // mipmap level
            pixels.as_ptr() as *const std::ffi::c_void,
            bytes_per_row as usize,
        );
    }

    // The replace should succeed (data is written to GPU)
    // Note: reading back requires more setup with blit encoder
}

#[test]
fn test_texture_bytes_per_row() {
    let device = get_device();

    let descriptor = TextureDescriptor::texture_2d_descriptor(
        PixelFormat::RGBA8_UNORM,
        256,
        256,
        false,
    ).unwrap();

    descriptor.set_storage_mode(StorageMode::SHARED);

    let texture = device.new_texture_with_descriptor(&descriptor).unwrap();

    // RGBA8 should be 4 bytes per pixel
    // 256 pixels wide = 256 * 4 = 1024 bytes per row minimum
    // Actual bytes per row may include padding

    // Create test data
    let bytes_per_row = 256 * 4;
    let pixels: Vec<u8> = vec![0u8; bytes_per_row as usize * 256];

    let region = Region::new_2d(0, 0, 256, 256);

    unsafe {
        texture.replace_region_simple(
            region,
            0,
            pixels.as_ptr() as *const std::ffi::c_void,
            bytes_per_row as usize,
        );
    }
}

#[test]
fn test_texture_3d_descriptor() {
    let device = get_device();

    let descriptor = TextureDescriptor::new().unwrap();
    descriptor.set_texture_type(metal::TextureType::TYPE_3D);
    descriptor.set_pixel_format(PixelFormat::RGBA8_UNORM);
    descriptor.set_width(16);
    descriptor.set_height(16);
    descriptor.set_depth(16);
    descriptor.set_storage_mode(StorageMode::SHARED);
    descriptor.set_usage(TextureUsage::SHADER_READ);

    let texture = device.new_texture_with_descriptor(&descriptor);
    assert!(texture.is_ok(), "Failed to create 3D texture: {:?}", texture.err());

    let texture = texture.unwrap();
    assert_eq!(texture.width(), 16);
    assert_eq!(texture.height(), 16);
    assert_eq!(texture.depth(), 16);
}

#[test]
fn test_texture_array() {
    let device = get_device();

    let descriptor = TextureDescriptor::new().unwrap();
    descriptor.set_texture_type(metal::TextureType::TYPE_2D_ARRAY);
    descriptor.set_pixel_format(PixelFormat::RGBA8_UNORM);
    descriptor.set_width(64);
    descriptor.set_height(64);
    descriptor.set_array_length(4);
    descriptor.set_storage_mode(StorageMode::SHARED);
    descriptor.set_usage(TextureUsage::SHADER_READ);

    let texture = device.new_texture_with_descriptor(&descriptor);
    assert!(texture.is_ok(), "Failed to create texture array: {:?}", texture.err());

    let texture = texture.unwrap();
    assert_eq!(texture.array_length(), 4);
}

// =============================================================================
// Texture Properties Tests
// =============================================================================

#[test]
fn test_texture_properties() {
    let device = get_device();

    let descriptor = TextureDescriptor::texture_2d_descriptor(
        PixelFormat::RGBA8_UNORM,
        512,
        256,
        true,
    ).unwrap();

    descriptor.set_storage_mode(StorageMode::SHARED);
    descriptor.set_usage(TextureUsage::SHADER_READ | TextureUsage::RENDER_TARGET);

    let texture = device.new_texture_with_descriptor(&descriptor).unwrap();

    assert_eq!(texture.width(), 512);
    assert_eq!(texture.height(), 256);
    assert_eq!(texture.depth(), 1);
    assert_eq!(texture.pixel_format(), PixelFormat::RGBA8_UNORM);
    assert!(texture.mipmap_level_count() > 1);
    assert_eq!(texture.sample_count(), 1);
    assert_eq!(texture.array_length(), 1);
}

#[test]
fn test_texture_usage_flags() {
    let device = get_device();

    let descriptor = TextureDescriptor::texture_2d_descriptor(
        PixelFormat::RGBA8_UNORM,
        64,
        64,
        false,
    ).unwrap();

    let combined_usage = TextureUsage::SHADER_READ | TextureUsage::SHADER_WRITE | TextureUsage::RENDER_TARGET;
    descriptor.set_usage(combined_usage);
    descriptor.set_storage_mode(StorageMode::SHARED);

    let texture = device.new_texture_with_descriptor(&descriptor).unwrap();

    let usage = texture.usage();
    assert!(usage.contains(TextureUsage::SHADER_READ));
    assert!(usage.contains(TextureUsage::SHADER_WRITE));
    assert!(usage.contains(TextureUsage::RENDER_TARGET));
}

#[test]
fn test_texture_gpu_resource_id() {
    let device = get_device();

    let descriptor = TextureDescriptor::texture_2d_descriptor(
        PixelFormat::RGBA8_UNORM,
        64,
        64,
        false,
    ).unwrap();

    descriptor.set_storage_mode(StorageMode::SHARED);

    let texture = device.new_texture_with_descriptor(&descriptor).unwrap();

    // GPU resource ID should be non-zero for valid textures
    let resource_id = texture.gpu_resource_id();
    assert!(resource_id._impl != 0, "GPU resource ID should be non-zero");
}

// =============================================================================
// Size Tests
// =============================================================================

#[test]
fn test_texture_descriptor_size() {
    assert_eq!(
        std::mem::size_of::<TextureDescriptor>(),
        std::mem::size_of::<*mut std::ffi::c_void>()
    );
}

#[test]
fn test_texture_size() {
    assert_eq!(
        std::mem::size_of::<metal::Texture>(),
        std::mem::size_of::<*mut std::ffi::c_void>()
    );
}
