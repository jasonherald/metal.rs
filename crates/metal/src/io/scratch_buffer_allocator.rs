//! IO scratch buffer allocator for Metal.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use super::IOScratchBuffer;

/// Allocator for IO scratch buffers.
///
/// C++ equivalent: `MTL::IOScratchBufferAllocator`
#[repr(transparent)]
pub struct IOScratchBufferAllocator(pub(crate) NonNull<c_void>);

impl IOScratchBufferAllocator {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal IO scratch buffer allocator.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new scratch buffer with the specified minimum size.
    ///
    /// C++ equivalent: `IOScratchBuffer* newScratchBuffer(NS::UInteger)`
    pub fn new_scratch_buffer(&self, minimum_size: UInteger) -> Option<IOScratchBuffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newScratchBufferWithMinimumSize:),
                minimum_size,
            );
            IOScratchBuffer::from_raw(ptr)
        }
    }
}

impl Clone for IOScratchBufferAllocator {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for IOScratchBufferAllocator {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for IOScratchBufferAllocator {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for IOScratchBufferAllocator {}
unsafe impl Sync for IOScratchBufferAllocator {}
