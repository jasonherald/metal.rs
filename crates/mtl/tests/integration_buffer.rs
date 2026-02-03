//! Buffer Integration Tests
//!
//! These tests verify that buffer operations work correctly with the Metal GPU.
//! They test real GPU operations, not just struct layouts or selector existence.

use mtl_gpu::{ResourceOptions, device};

/// Get the default Metal device or skip the test.
fn get_device() -> mtl_gpu::Device {
    device::system_default().expect("No Metal device available")
}

// =============================================================================
// Buffer Creation Tests
// =============================================================================

#[test]
fn test_create_buffer_with_length() {
    let device = get_device();

    let buffer = device
        .new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    assert_eq!(buffer.length(), 1024);
}

#[test]
fn test_create_buffer_with_bytes() {
    let device = get_device();

    let data: Vec<u8> = (0u16..256).map(|x| x as u8).collect();
    let buffer = device
        .new_buffer_with_bytes(&data, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    assert_eq!(buffer.length(), 256);

    // Verify data was copied correctly
    let contents = buffer.contents().expect("Buffer contents is null");
    let result = unsafe { std::slice::from_raw_parts(contents as *const u8, 256) };
    assert_eq!(result, data.as_slice());
}

#[test]
fn test_create_buffer_various_sizes() {
    let device = get_device();

    // Test various buffer sizes
    let sizes = [1, 16, 256, 1024, 4096, 65536, 1024 * 1024];

    for size in sizes {
        let buffer = device
            .new_buffer(size, ResourceOptions::STORAGE_MODE_SHARED)
            .unwrap_or_else(|| panic!("Failed to create buffer of size {}", size));

        assert_eq!(buffer.length(), size);
    }
}

// =============================================================================
// Storage Mode Tests
// =============================================================================

#[test]
fn test_buffer_storage_mode_shared() {
    let device = get_device();

    let buffer = device
        .new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create shared buffer");

    // Shared buffers should have accessible contents
    assert!(buffer.contents().is_some());
    // Verify resource options include shared storage mode
    let options = buffer.resource_options();
    assert!(options.contains(ResourceOptions::STORAGE_MODE_SHARED));
}

#[test]
fn test_buffer_storage_mode_private() {
    let device = get_device();

    let buffer = device
        .new_buffer(1024, ResourceOptions::STORAGE_MODE_PRIVATE)
        .expect("Failed to create private buffer");

    // Private buffers may or may not have accessible contents depending on unified memory
    // Verify resource options include private storage mode
    let options = buffer.resource_options();
    assert!(options.contains(ResourceOptions::STORAGE_MODE_PRIVATE));
}

#[test]
#[cfg(target_os = "macos")]
fn test_buffer_storage_mode_managed() {
    let device = get_device();

    // Managed storage is only available on macOS with discrete GPUs
    // On unified memory devices, this might behave like shared
    let buffer = device
        .new_buffer(1024, ResourceOptions::STORAGE_MODE_MANAGED)
        .expect("Failed to create managed buffer");

    assert!(buffer.contents().is_some());
}

// =============================================================================
// Buffer Data Operations
// =============================================================================

#[test]
fn test_buffer_write_and_read_u8() {
    let device = get_device();

    let buffer = device
        .new_buffer(256, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // Write data
    let contents = buffer.contents().expect("Buffer contents is null") as *mut u8;
    let data: Vec<u8> = (0u16..256).map(|x| x as u8).collect();
    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr(), contents, 256);
    }

    // Read data back
    let result = unsafe { std::slice::from_raw_parts(contents, 256) };
    assert_eq!(result, data.as_slice());
}

#[test]
fn test_buffer_write_and_read_f32() {
    let device = get_device();

    let element_count = 64;
    let buffer_size = element_count * std::mem::size_of::<f32>();

    let buffer = device
        .new_buffer(buffer_size, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // Write f32 data
    let contents = buffer.contents().expect("Buffer contents is null") as *mut f32;
    let data: Vec<f32> = (0..element_count).map(|i| i as f32 * 1.5).collect();
    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr(), contents, element_count);
    }

    // Read data back
    let result = unsafe { std::slice::from_raw_parts(contents, element_count) };
    assert_eq!(result, data.as_slice());
}

#[test]
fn test_buffer_write_and_read_structs() {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Vertex {
        position: [f32; 3],
        normal: [f32; 3],
        uv: [f32; 2],
    }

    let device = get_device();

    let vertices = vec![
        Vertex {
            position: [0.0, 1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            uv: [0.5, 0.0],
        },
        Vertex {
            position: [-1.0, -1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            uv: [0.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            uv: [1.0, 1.0],
        },
    ];

    let buffer_size = vertices.len() * std::mem::size_of::<Vertex>();
    let buffer = device
        .new_buffer(buffer_size, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // Write vertices
    let contents = buffer.contents().expect("Buffer contents is null") as *mut Vertex;
    unsafe {
        std::ptr::copy_nonoverlapping(vertices.as_ptr(), contents, vertices.len());
    }

    // Read vertices back
    let result = unsafe { std::slice::from_raw_parts(contents, vertices.len()) };
    assert_eq!(result, vertices.as_slice());
}

// =============================================================================
// Buffer Properties Tests
// =============================================================================

#[test]
fn test_buffer_device() {
    let device = get_device();

    let buffer = device
        .new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // Buffer should reference the same device
    let buffer_device = buffer.device();
    assert_eq!(buffer_device.registry_id(), device.registry_id());
}

#[test]
fn test_buffer_label() {
    let device = get_device();

    let buffer = device
        .new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // Set and get label
    buffer.set_label("Test Buffer");
    let label = buffer.label();
    assert_eq!(label, Some("Test Buffer".to_string()));
}

#[test]
fn test_buffer_gpu_address() {
    let device = get_device();

    let buffer = device
        .new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // GPU address should be non-zero for valid buffers
    let gpu_address = buffer.gpu_address();
    assert!(gpu_address != 0, "GPU address should be non-zero");
}

#[test]
fn test_buffer_resource_options() {
    let device = get_device();

    let options =
        ResourceOptions::STORAGE_MODE_SHARED | ResourceOptions::CPU_CACHE_MODE_DEFAULT_CACHE;
    let buffer = device
        .new_buffer(1024, options)
        .expect("Failed to create buffer");

    // Verify options are retrievable
    let retrieved_options = buffer.resource_options();
    assert!(retrieved_options.contains(ResourceOptions::STORAGE_MODE_SHARED));
}

// =============================================================================
// Buffer Clone/Retain Tests
// =============================================================================

#[test]
fn test_buffer_clone() {
    let device = get_device();

    let buffer = device
        .new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // Clone should work and reference the same underlying buffer
    let buffer_clone = buffer.clone();

    // Both should have same properties
    assert_eq!(buffer.length(), buffer_clone.length());
    assert_eq!(buffer.gpu_address(), buffer_clone.gpu_address());

    // Modifying through one should be visible through the other
    let contents1 = buffer.contents().expect("contents") as *mut u8;
    let contents2 = buffer_clone.contents().expect("contents") as *const u8;

    unsafe {
        *contents1 = 42;
        assert_eq!(*contents2, 42);
    }
}

// =============================================================================
// Allocated Size Tests
// =============================================================================

#[test]
fn test_buffer_allocated_size() {
    let device = get_device();

    let buffer = device
        .new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // Allocated size should be at least the requested size
    // (may be larger due to alignment requirements)
    assert!(buffer.allocated_size() >= 1024);
}

// =============================================================================
// Zero-length Buffer Tests
// =============================================================================

#[test]
fn test_zero_length_buffer() {
    let device = get_device();

    // Zero-length buffer should either succeed or fail gracefully
    let result = device.new_buffer(0, ResourceOptions::STORAGE_MODE_SHARED);

    // Either it succeeds with length 0, or it fails
    // Both are acceptable behaviors
    if let Some(buffer) = result {
        assert_eq!(buffer.length(), 0);
    }
}

// =============================================================================
// Large Buffer Tests
// =============================================================================

#[test]
fn test_large_buffer() {
    let device = get_device();

    // Create a 16MB buffer
    let size = 16 * 1024 * 1024;
    let buffer = device
        .new_buffer(size, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create 16MB buffer");

    assert_eq!(buffer.length(), size);

    // Write to beginning, middle, and end
    let contents = buffer.contents().expect("contents") as *mut u8;
    unsafe {
        *contents = 1;
        *contents.add(size / 2) = 2;
        *contents.add(size - 1) = 3;

        assert_eq!(*contents, 1);
        assert_eq!(*contents.add(size / 2), 2);
        assert_eq!(*contents.add(size - 1), 3);
    }
}
