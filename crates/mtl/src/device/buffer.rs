//! Device buffer creation methods.
//!
//! Corresponds to buffer creation methods in `Metal/MTLDevice.hpp`.

use std::ffi::c_void;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_2, msg_send_3, msg_send_4, sel};

use super::Device;
use crate::buffer::Buffer;
use crate::enums::ResourceOptions;

impl Device {
    // =========================================================================
    // Buffer Creation
    // =========================================================================

    /// Create a new buffer with the specified length and options.
    ///
    /// C++ equivalent: `Buffer* newBuffer(NS::UInteger length, MTL::ResourceOptions options)`
    pub fn new_buffer(&self, length: UInteger, options: ResourceOptions) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newBufferWithLength: options:),
                length,
                options,
            );
            Buffer::from_raw(ptr)
        }
    }

    /// Create a new buffer initialized with the specified data.
    ///
    /// C++ equivalent: `Buffer* newBuffer(const void* pointer, NS::UInteger length, MTL::ResourceOptions options)`
    pub fn new_buffer_with_bytes(&self, bytes: &[u8], options: ResourceOptions) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newBufferWithBytes: length: options:),
                bytes.as_ptr() as *const c_void,
                bytes.len() as UInteger,
                options,
            );
            Buffer::from_raw(ptr)
        }
    }

    /// Create a new buffer initialized with the specified data (raw pointer version).
    ///
    /// C++ equivalent: `Buffer* newBuffer(const void* pointer, NS::UInteger length, MTL::ResourceOptions options)`
    ///
    /// # Safety
    ///
    /// The pointer must be valid for `length` bytes.
    pub unsafe fn new_buffer_with_bytes_ptr(
        &self,
        pointer: *const c_void,
        length: UInteger,
        options: ResourceOptions,
    ) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newBufferWithBytes: length: options:),
                pointer,
                length,
                options,
            );
            Buffer::from_raw(ptr)
        }
    }

    /// Create a new buffer that wraps existing memory without copying.
    ///
    /// The deallocator block will be called when the buffer is deallocated.
    ///
    /// C++ equivalent: `Buffer* newBuffer(void* pointer, NS::UInteger length, MTL::ResourceOptions options, void (^)(void*, NS::UInteger))`
    ///
    /// # Safety
    ///
    /// - The pointer must remain valid until the deallocator is called.
    /// - The pointer must be page-aligned.
    pub unsafe fn new_buffer_with_bytes_no_copy<F>(
        &self,
        pointer: *mut c_void,
        length: UInteger,
        options: ResourceOptions,
        deallocator: F,
    ) -> Option<Buffer>
    where
        F: Fn(*mut c_void, usize) + Send + 'static,
    {
        let block = mtl_sys::DeallocatorBlock::from_fn(deallocator);

        unsafe {
            let ptr: *mut c_void = msg_send_4(
                self.as_ptr(),
                sel!(newBufferWithBytesNoCopy: length: options: deallocator:),
                pointer,
                length,
                options,
                block.as_ptr(),
            );
            // Metal retains the block
            std::mem::forget(block);
            Buffer::from_raw(ptr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::system_default;

    #[test]
    fn test_new_buffer() {
        let device = system_default().expect("no Metal device");
        let buffer = device.new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED);
        assert!(buffer.is_some());

        let buffer = buffer.unwrap();
        assert_eq!(buffer.length(), 1024);
    }

    #[test]
    fn test_new_buffer_with_bytes() {
        let device = system_default().expect("no Metal device");
        let data = [0u8; 256];
        let buffer = device.new_buffer_with_bytes(&data, ResourceOptions::STORAGE_MODE_SHARED);
        assert!(buffer.is_some());

        let buffer = buffer.unwrap();
        assert_eq!(buffer.length(), 256);
    }

    #[test]
    fn test_buffer_contents() {
        let device = system_default().expect("no Metal device");
        let data = [1u8, 2, 3, 4, 5, 6, 7, 8];
        let buffer = device
            .new_buffer_with_bytes(&data, ResourceOptions::STORAGE_MODE_SHARED)
            .expect("failed to create buffer");

        let contents = buffer.contents();
        assert!(contents.is_some());

        // Verify the data was copied
        let ptr = contents.unwrap() as *const u8;
        unsafe {
            assert_eq!(*ptr, 1);
            assert_eq!(*ptr.add(7), 8);
        }
    }
}
