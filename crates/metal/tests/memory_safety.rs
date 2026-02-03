//! Memory Safety Tests
//!
//! These tests verify that Metal object lifecycle and memory management is correct.
//! They test retain/release patterns, factory method ownership, and object cloning.

use metal::{device, ResourceOptions};

/// Get the default Metal device or skip the test.
fn get_device() -> metal::Device {
    device::system_default().expect("No Metal device available")
}

// =============================================================================
// Clone and Drop Tests
// =============================================================================

#[test]
fn test_device_clone() {
    let device1 = get_device();
    let device2 = device1.clone();

    // Both should reference the same underlying device
    assert_eq!(device1.registry_id(), device2.registry_id());
    assert_eq!(device1.name(), device2.name());

    // Dropping one shouldn't affect the other
    drop(device1);
    assert!(!device2.name().is_empty());
}

#[test]
fn test_buffer_clone() {
    let device = get_device();

    let buffer1 = device
        .new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    let buffer2 = buffer1.clone();

    // Both should reference the same underlying buffer
    assert_eq!(buffer1.length(), buffer2.length());
    assert_eq!(buffer1.gpu_address(), buffer2.gpu_address());

    // Both should have accessible contents
    let contents1 = buffer1.contents().expect("contents1");
    let contents2 = buffer2.contents().expect("contents2");

    // Both pointers should point to the same memory
    assert_eq!(contents1, contents2);

    // Modify through one, read through other
    unsafe {
        *(contents1 as *mut u8) = 123;
        assert_eq!(*(contents2 as *const u8), 123);
    }

    // Dropping one shouldn't affect the other
    drop(buffer1);
    assert_eq!(buffer2.length(), 1024);
}

#[test]
fn test_command_queue_clone() {
    let device = get_device();

    let queue1 = device.new_command_queue().expect("Failed to create queue");
    let queue2 = queue1.clone();

    // Both should reference the same device
    let device1 = queue1.device();
    let device2 = queue2.device();
    assert_eq!(device1.registry_id(), device2.registry_id());

    // Dropping one shouldn't affect the other
    drop(queue1);
    let _cmd_buffer = queue2.command_buffer().expect("Should still work");
}

// =============================================================================
// Factory Method Ownership Tests
// =============================================================================

#[test]
fn test_buffer_ownership() {
    let device = get_device();

    // Create buffer - we own it
    let buffer = device
        .new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // We can use it
    assert_eq!(buffer.length(), 1024);

    // We can clone it
    let buffer2 = buffer.clone();
    assert_eq!(buffer2.length(), 1024);

    // Original still valid after clone dropped
    drop(buffer2);
    assert_eq!(buffer.length(), 1024);
}

#[test]
fn test_command_queue_ownership() {
    let device = get_device();

    // Create queue - we own it
    let queue = device.new_command_queue().expect("Failed to create queue");

    // We can use it multiple times
    let _cmd1 = queue.command_buffer().expect("cmd1");
    let _cmd2 = queue.command_buffer().expect("cmd2");
    let _cmd3 = queue.command_buffer().expect("cmd3");
}

#[test]
fn test_command_buffer_ownership() {
    let device = get_device();
    let queue = device.new_command_queue().expect("Failed to create queue");

    // Create command buffer - we own it
    let cmd_buffer = queue.command_buffer().expect("Failed to create command buffer");

    // We can use it
    cmd_buffer.commit();
    cmd_buffer.wait_until_completed();
}

// =============================================================================
// Multiple Object Tests
// =============================================================================

#[test]
fn test_multiple_buffers() {
    let device = get_device();

    // Create many buffers
    let mut buffers = Vec::new();
    for i in 0..100 {
        let buffer = device
            .new_buffer(1024 * (i + 1), ResourceOptions::STORAGE_MODE_SHARED)
            .expect(&format!("Failed to create buffer {}", i));
        buffers.push(buffer);
    }

    // All should still be valid
    for (i, buffer) in buffers.iter().enumerate() {
        assert_eq!(buffer.length(), 1024 * (i + 1));
    }

    // Drop half of them
    buffers.truncate(50);

    // Remaining should still be valid
    for (i, buffer) in buffers.iter().enumerate() {
        assert_eq!(buffer.length(), 1024 * (i + 1));
    }
}

#[test]
fn test_multiple_command_queues() {
    let device = get_device();

    // Create multiple queues
    let mut queues = Vec::new();
    for _ in 0..10 {
        let queue = device.new_command_queue().expect("Failed to create queue");
        queues.push(queue);
    }

    // All should work
    for queue in &queues {
        let _cmd = queue.command_buffer().expect("Should work");
    }
}

#[test]
fn test_multiple_command_buffers() {
    let device = get_device();
    let queue = device.new_command_queue().expect("Failed to create queue");

    // Create many command buffers from the same queue
    let mut cmd_buffers = Vec::new();
    for _ in 0..50 {
        let cmd = queue.command_buffer().expect("Failed to create command buffer");
        cmd_buffers.push(cmd);
    }

    // Commit and wait on all
    for cmd in cmd_buffers {
        cmd.commit();
        cmd.wait_until_completed();
    }
}

// =============================================================================
// Object Hierarchy Tests
// =============================================================================

#[test]
fn test_buffer_device_reference() {
    let device = get_device();
    let device_id = device.registry_id();

    let buffer = device
        .new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // Buffer should hold a reference to device
    let buffer_device = buffer.device();
    assert_eq!(buffer_device.registry_id(), device_id);

    // Even after dropping original device reference
    drop(device);

    // Buffer's device reference should still be valid
    let buffer_device2 = buffer.device();
    assert_eq!(buffer_device2.registry_id(), device_id);
}

#[test]
fn test_queue_device_reference() {
    let device = get_device();
    let device_id = device.registry_id();

    let queue = device.new_command_queue().expect("Failed to create queue");

    // Queue should hold a reference to device
    let queue_device = queue.device();
    assert_eq!(queue_device.registry_id(), device_id);

    // Even after dropping original device reference
    drop(device);

    // Queue's device reference should still be valid
    let queue_device2 = queue.device();
    assert_eq!(queue_device2.registry_id(), device_id);
}

// =============================================================================
// Library and Function Tests
// =============================================================================

const SIMPLE_KERNEL: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void simple_kernel() {
}
"#;

#[test]
fn test_library_ownership() {
    let device = get_device();

    let library = device
        .new_library_with_source(SIMPLE_KERNEL, None)
        .expect("Failed to compile");

    // Library should be usable
    let function = library
        .new_function_with_name("simple_kernel")
        .expect("Function not found");

    assert_eq!(function.name(), Some("simple_kernel".to_string()));
}

#[test]
fn test_function_ownership() {
    let device = get_device();

    let library = device
        .new_library_with_source(SIMPLE_KERNEL, None)
        .expect("Failed to compile");

    let function = library
        .new_function_with_name("simple_kernel")
        .expect("Function not found");

    // Drop library
    drop(library);

    // Function should still be valid (it retains the library)
    assert_eq!(function.name(), Some("simple_kernel".to_string()));
}

#[test]
fn test_pipeline_ownership() {
    let device = get_device();

    let library = device
        .new_library_with_source(SIMPLE_KERNEL, None)
        .expect("Failed to compile");

    let function = library
        .new_function_with_name("simple_kernel")
        .expect("Function not found");

    let pipeline = device
        .new_compute_pipeline_state_with_function(&function)
        .expect("Failed to create pipeline");

    // Drop library and function
    drop(library);
    drop(function);

    // Pipeline should still be valid
    assert!(pipeline.max_total_threads_per_threadgroup() > 0);
}

// =============================================================================
// Stress Tests
// =============================================================================

#[test]
fn test_rapid_buffer_create_drop() {
    let device = get_device();

    // Rapidly create and drop buffers
    for _ in 0..1000 {
        let buffer = device
            .new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)
            .expect("Failed to create buffer");
        assert_eq!(buffer.length(), 1024);
        // Buffer dropped at end of iteration
    }
}

#[test]
fn test_concurrent_buffer_access() {
    let device = get_device();

    let buffer = device
        .new_buffer(4096, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // Clone buffer multiple times
    let clones: Vec<_> = (0..10).map(|_| buffer.clone()).collect();

    // All clones should have same properties
    for clone in &clones {
        assert_eq!(clone.length(), 4096);
        assert_eq!(clone.gpu_address(), buffer.gpu_address());
    }

    // Write through one
    let contents = buffer.contents().expect("contents") as *mut u8;
    unsafe {
        for i in 0..100 {
            *contents.add(i) = i as u8;
        }
    }

    // Read through all clones - should see same data
    for clone in &clones {
        let clone_contents = clone.contents().expect("contents") as *const u8;
        unsafe {
            for i in 0..100 {
                assert_eq!(*clone_contents.add(i), i as u8);
            }
        }
    }
}

// =============================================================================
// Label Memory Tests
// =============================================================================

#[test]
fn test_buffer_label_memory() {
    let device = get_device();

    let buffer = device
        .new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create buffer");

    // Set label multiple times
    for i in 0..100 {
        let label = format!("Test Buffer {}", i);
        buffer.set_label(&label);
        assert_eq!(buffer.label(), Some(label));
    }
}

#[test]
fn test_command_queue_label_memory() {
    let device = get_device();

    let queue = device.new_command_queue().expect("Failed to create queue");

    // Set label multiple times
    for i in 0..100 {
        let label = format!("Test Queue {}", i);
        queue.set_label(&label);
        assert_eq!(queue.label(), Some(label));
    }
}

// =============================================================================
// Device All Tests
// =============================================================================

#[cfg(target_os = "macos")]
#[test]
fn test_all_devices_ownership() {
    let devices = device::copy_all_devices();

    // All devices should be independently usable
    for device in &devices {
        let _buffer = device
            .new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)
            .expect("Failed to create buffer");
    }

    // Devices should still be valid after creating objects
    for device in &devices {
        assert!(!device.name().is_empty());
    }
}
