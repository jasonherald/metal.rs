//! Compute pass descriptor.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::DispatchType;

use super::ComputePassSampleBufferAttachmentDescriptorArray;

/// A compute pass descriptor that configures a compute pass.
///
/// C++ equivalent: `MTL::ComputePassDescriptor`
#[repr(transparent)]
pub struct ComputePassDescriptor(NonNull<c_void>);

impl ComputePassDescriptor {
    /// Create a ComputePassDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal compute pass descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new compute pass descriptor.
    ///
    /// C++ equivalent: `ComputePassDescriptor::computePassDescriptor()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::class!(MTLComputePassDescriptor);
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(computePassDescriptor));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Self::from_raw(ptr)
        }
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the dispatch type.
    ///
    /// C++ equivalent: `DispatchType dispatchType() const`
    #[inline]
    pub fn dispatch_type(&self) -> DispatchType {
        unsafe { msg_send_0(self.as_ptr(), sel!(dispatchType)) }
    }

    /// Set the dispatch type.
    ///
    /// C++ equivalent: `void setDispatchType(DispatchType)`
    #[inline]
    pub fn set_dispatch_type(&self, dispatch_type: DispatchType) {
        unsafe {
            msg_send_1::<(), DispatchType>(self.as_ptr(), sel!(setDispatchType:), dispatch_type);
        }
    }

    /// Get the sample buffer attachments array.
    ///
    /// C++ equivalent: `ComputePassSampleBufferAttachmentDescriptorArray* sampleBufferAttachments() const`
    pub fn sample_buffer_attachments(
        &self,
    ) -> Option<ComputePassSampleBufferAttachmentDescriptorArray> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(sampleBufferAttachments));
            ComputePassSampleBufferAttachmentDescriptorArray::from_raw(ptr)
        }
    }
}

impl Default for ComputePassDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create ComputePassDescriptor")
    }
}

impl Clone for ComputePassDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ComputePassDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for ComputePassDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ComputePassDescriptor {}
unsafe impl Sync for ComputePassDescriptor {}

impl std::fmt::Debug for ComputePassDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComputePassDescriptor")
            .field("dispatch_type", &self.dispatch_type())
            .finish()
    }
}
