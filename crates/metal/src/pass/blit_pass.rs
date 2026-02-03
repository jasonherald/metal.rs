//! Blit pass descriptor.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, sel};

use super::BlitPassSampleBufferAttachmentDescriptorArray;

/// A blit pass descriptor that configures a blit pass.
///
/// C++ equivalent: `MTL::BlitPassDescriptor`
#[repr(transparent)]
pub struct BlitPassDescriptor(NonNull<c_void>);

impl BlitPassDescriptor {
    /// Create a BlitPassDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal blit pass descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new blit pass descriptor.
    ///
    /// C++ equivalent: `BlitPassDescriptor::blitPassDescriptor()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::class!(MTLBlitPassDescriptor);
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(blitPassDescriptor));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Self::from_raw(ptr)
        }
    }

    /// Get the sample buffer attachments array.
    ///
    /// C++ equivalent: `BlitPassSampleBufferAttachmentDescriptorArray* sampleBufferAttachments() const`
    pub fn sample_buffer_attachments(&self) -> Option<BlitPassSampleBufferAttachmentDescriptorArray> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(sampleBufferAttachments));
            BlitPassSampleBufferAttachmentDescriptorArray::from_raw(ptr)
        }
    }
}

impl Default for BlitPassDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create BlitPassDescriptor")
    }
}

impl Clone for BlitPassDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for BlitPassDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for BlitPassDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for BlitPassDescriptor {}
unsafe impl Sync for BlitPassDescriptor {}

impl std::fmt::Debug for BlitPassDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlitPassDescriptor").finish()
    }
}
