//! Device heap creation methods.
//!
//! Corresponds to heap creation methods in `Metal/MTLDevice.hpp`.

use std::ffi::c_void;

use metal_foundation::Referencing;
use metal_sys::{msg_send_1, sel};

use crate::error::ValidationError;
use crate::heap::{Heap, HeapDescriptor};
use super::Device;

impl Device {
    // =========================================================================
    // Heap Creation
    // =========================================================================

    /// Create a new heap with the specified descriptor.
    ///
    /// C++ equivalent: `Heap* newHeap(const HeapDescriptor*)`
    pub fn new_heap(&self, descriptor: &HeapDescriptor) -> Option<Heap> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newHeapWithDescriptor:),
                descriptor.as_ptr(),
            );
            Heap::from_raw(ptr)
        }
    }

    /// Create a heap with validation.
    ///
    /// This safe method validates the descriptor before calling Metal APIs:
    /// - Ensures heap size is > 0
    ///
    /// # Example
    ///
    /// ```ignore
    /// let desc = HeapDescriptor::new().unwrap();
    /// desc.set_size(1024 * 1024); // 1 MB
    /// desc.set_storage_mode(StorageMode::PRIVATE);
    ///
    /// match device.new_heap_validated(&desc) {
    ///     Ok(heap) => { /* use heap */ }
    ///     Err(ValidationError::InvalidHeapSize) => { /* handle error */ }
    ///     Err(e) => { /* handle other errors */ }
    /// }
    /// ```
    pub fn new_heap_validated(
        &self,
        descriptor: &HeapDescriptor,
    ) -> Result<Heap, ValidationError> {
        // Validate size
        if descriptor.size() == 0 {
            return Err(ValidationError::InvalidHeapSize);
        }

        // Call existing safe implementation
        self.new_heap(descriptor)
            .ok_or(ValidationError::CreationFailed(None))
    }

    /// Create a new heap with a raw descriptor pointer.
    ///
    /// C++ equivalent: `Heap* newHeap(const HeapDescriptor*)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn new_heap_with_ptr(&self, descriptor: *const c_void) -> Option<Heap> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newHeapWithDescriptor:),
                descriptor,
            );
            Heap::from_raw(ptr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::system_default;
    use crate::enums::StorageMode;

    #[test]
    fn test_new_heap() {
        let device = system_default().expect("no Metal device");
        let descriptor = HeapDescriptor::new().expect("failed to create descriptor");

        descriptor.set_size(1024 * 1024); // 1 MB
        descriptor.set_storage_mode(StorageMode::PRIVATE);

        let heap = device.new_heap(&descriptor);
        assert!(heap.is_some());

        let heap = heap.unwrap();
        assert!(heap.size() >= 1024 * 1024);
    }
}
