//! IO scratch buffer for Metal.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, sel};

use crate::buffer::Buffer;

/// Scratch buffer for IO operations.
///
/// C++ equivalent: `MTL::IOScratchBuffer`
#[repr(transparent)]
pub struct IOScratchBuffer(pub(crate) NonNull<c_void>);

impl IOScratchBuffer {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal IO scratch buffer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the underlying buffer.
    ///
    /// C++ equivalent: `Buffer* buffer() const`
    pub fn buffer(&self) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(buffer));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Buffer::from_raw(ptr)
        }
    }
}

impl Clone for IOScratchBuffer {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for IOScratchBuffer {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for IOScratchBuffer {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for IOScratchBuffer {}
unsafe impl Sync for IOScratchBuffer {}
