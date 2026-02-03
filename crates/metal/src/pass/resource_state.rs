//! Resource state pass descriptors.
//!
//! Corresponds to `Metal/MTLResourceStatePass.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{Class, msg_send_0, msg_send_1, msg_send_2, sel};

use crate::counter::CounterSampleBuffer;

/// A descriptor for a sample buffer attachment in a resource state pass.
///
/// C++ equivalent: `MTL::ResourceStatePassSampleBufferAttachmentDescriptor`
#[repr(transparent)]
pub struct ResourceStatePassSampleBufferAttachmentDescriptor(pub(crate) NonNull<c_void>);

impl ResourceStatePassSampleBufferAttachmentDescriptor {
    /// Create a new sample buffer attachment descriptor.
    ///
    /// C++ equivalent: `alloc()->init()`
    pub fn new() -> Option<Self> {
        let class = Class::get("MTLResourceStatePassSampleBufferAttachmentDescriptor")?;
        unsafe {
            let obj: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if obj.is_null() {
                return None;
            }
            let obj: *mut c_void = msg_send_0(obj, sel!(init));
            Self::from_raw(obj)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the sample buffer.
    ///
    /// C++ equivalent: `CounterSampleBuffer* sampleBuffer() const`
    pub fn sample_buffer(&self) -> Option<CounterSampleBuffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(sampleBuffer));
            if ptr.is_null() {
                None
            } else {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
                CounterSampleBuffer::from_raw(ptr)
            }
        }
    }

    /// Set the sample buffer.
    ///
    /// C++ equivalent: `void setSampleBuffer(const CounterSampleBuffer*)`
    pub fn set_sample_buffer(&self, sample_buffer: Option<&CounterSampleBuffer>) {
        unsafe {
            let ptr = sample_buffer.map_or(std::ptr::null(), |b| b.as_ptr());
            let _: () = msg_send_1(self.as_ptr(), sel!(setSampleBuffer:), ptr);
        }
    }

    /// Get the start of encoder sample index.
    ///
    /// C++ equivalent: `NS::UInteger startOfEncoderSampleIndex() const`
    pub fn start_of_encoder_sample_index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(startOfEncoderSampleIndex)) }
    }

    /// Set the start of encoder sample index.
    ///
    /// C++ equivalent: `void setStartOfEncoderSampleIndex(NS::UInteger)`
    pub fn set_start_of_encoder_sample_index(&self, index: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setStartOfEncoderSampleIndex:), index);
        }
    }

    /// Get the end of encoder sample index.
    ///
    /// C++ equivalent: `NS::UInteger endOfEncoderSampleIndex() const`
    pub fn end_of_encoder_sample_index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(endOfEncoderSampleIndex)) }
    }

    /// Set the end of encoder sample index.
    ///
    /// C++ equivalent: `void setEndOfEncoderSampleIndex(NS::UInteger)`
    pub fn set_end_of_encoder_sample_index(&self, index: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setEndOfEncoderSampleIndex:), index);
        }
    }
}

impl Default for ResourceStatePassSampleBufferAttachmentDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create ResourceStatePassSampleBufferAttachmentDescriptor")
    }
}

impl Clone for ResourceStatePassSampleBufferAttachmentDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ResourceStatePassSampleBufferAttachmentDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for ResourceStatePassSampleBufferAttachmentDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ResourceStatePassSampleBufferAttachmentDescriptor {}
unsafe impl Sync for ResourceStatePassSampleBufferAttachmentDescriptor {}

impl std::fmt::Debug for ResourceStatePassSampleBufferAttachmentDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResourceStatePassSampleBufferAttachmentDescriptor")
            .field(
                "start_of_encoder_sample_index",
                &self.start_of_encoder_sample_index(),
            )
            .field(
                "end_of_encoder_sample_index",
                &self.end_of_encoder_sample_index(),
            )
            .finish()
    }
}

/// An array of sample buffer attachment descriptors for resource state passes.
///
/// C++ equivalent: `MTL::ResourceStatePassSampleBufferAttachmentDescriptorArray`
#[repr(transparent)]
pub struct ResourceStatePassSampleBufferAttachmentDescriptorArray(pub(crate) NonNull<c_void>);

impl ResourceStatePassSampleBufferAttachmentDescriptorArray {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid descriptor array object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the descriptor at the specified index.
    ///
    /// C++ equivalent: `ResourceStatePassSampleBufferAttachmentDescriptor* object(NS::UInteger)`
    pub fn object(
        &self,
        index: UInteger,
    ) -> Option<ResourceStatePassSampleBufferAttachmentDescriptor> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(objectAtIndexedSubscript:), index);
            if ptr.is_null() {
                None
            } else {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
                ResourceStatePassSampleBufferAttachmentDescriptor::from_raw(ptr)
            }
        }
    }

    /// Set the descriptor at the specified index.
    ///
    /// C++ equivalent: `void setObject(const ResourceStatePassSampleBufferAttachmentDescriptor*, NS::UInteger)`
    pub fn set_object(
        &self,
        descriptor: Option<&ResourceStatePassSampleBufferAttachmentDescriptor>,
        index: UInteger,
    ) {
        unsafe {
            let ptr = descriptor.map_or(std::ptr::null(), |d| d.as_ptr());
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setObject: atIndexedSubscript:),
                ptr,
                index,
            );
        }
    }
}

impl Clone for ResourceStatePassSampleBufferAttachmentDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ResourceStatePassSampleBufferAttachmentDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for ResourceStatePassSampleBufferAttachmentDescriptorArray {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ResourceStatePassSampleBufferAttachmentDescriptorArray {}
unsafe impl Sync for ResourceStatePassSampleBufferAttachmentDescriptorArray {}

impl std::fmt::Debug for ResourceStatePassSampleBufferAttachmentDescriptorArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResourceStatePassSampleBufferAttachmentDescriptorArray")
            .finish()
    }
}

/// A descriptor for a resource state pass.
///
/// C++ equivalent: `MTL::ResourceStatePassDescriptor`
#[repr(transparent)]
pub struct ResourceStatePassDescriptor(pub(crate) NonNull<c_void>);

impl ResourceStatePassDescriptor {
    /// Create a new resource state pass descriptor.
    ///
    /// C++ equivalent: `resourceStatePassDescriptor()`
    pub fn new() -> Option<Self> {
        let class = Class::get("MTLResourceStatePassDescriptor")?;
        unsafe {
            let obj: *mut c_void = msg_send_0(class.as_ptr(), sel!(resourceStatePassDescriptor));
            if obj.is_null() {
                None
            } else {
                let _: *mut c_void = msg_send_0(obj, sel!(retain));
                Self::from_raw(obj)
            }
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the sample buffer attachments array.
    ///
    /// C++ equivalent: `ResourceStatePassSampleBufferAttachmentDescriptorArray* sampleBufferAttachments() const`
    pub fn sample_buffer_attachments(
        &self,
    ) -> Option<ResourceStatePassSampleBufferAttachmentDescriptorArray> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(sampleBufferAttachments));
            if ptr.is_null() {
                None
            } else {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
                ResourceStatePassSampleBufferAttachmentDescriptorArray::from_raw(ptr)
            }
        }
    }
}

impl Default for ResourceStatePassDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create ResourceStatePassDescriptor")
    }
}

impl Clone for ResourceStatePassDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ResourceStatePassDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for ResourceStatePassDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ResourceStatePassDescriptor {}
unsafe impl Sync for ResourceStatePassDescriptor {}

impl std::fmt::Debug for ResourceStatePassDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResourceStatePassDescriptor").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_state_pass_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<ResourceStatePassDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_resource_state_pass_sample_buffer_attachment_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<ResourceStatePassSampleBufferAttachmentDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
