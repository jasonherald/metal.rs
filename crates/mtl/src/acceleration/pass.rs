//! Acceleration structure pass descriptors.
//!
//! Contains pass-related types for acceleration structure operations.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, msg_send_2, sel};

pub struct AccelerationStructurePassDescriptor(pub(crate) NonNull<c_void>);

impl AccelerationStructurePassDescriptor {
    /// Create a new acceleration structure pass descriptor.
    ///
    /// C++ equivalent: `static AccelerationStructurePassDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLAccelerationStructurePassDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new acceleration structure pass descriptor using the class method.
    ///
    /// C++ equivalent: `static AccelerationStructurePassDescriptor* accelerationStructurePassDescriptor()`
    pub fn descriptor() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLAccelerationStructurePassDescriptor")?;
            let ptr: *mut c_void =
                msg_send_0(class.as_ptr(), sel!(accelerationStructurePassDescriptor));
            if ptr.is_null() {
                return None;
            }
            // This returns an autoreleased object, so retain it
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal acceleration structure pass descriptor.
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
    /// C++ equivalent: `AccelerationStructurePassSampleBufferAttachmentDescriptorArray* sampleBufferAttachments() const`
    pub fn sample_buffer_attachments(
        &self,
    ) -> Option<AccelerationStructurePassSampleBufferAttachmentDescriptorArray> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(sampleBufferAttachments));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            AccelerationStructurePassSampleBufferAttachmentDescriptorArray::from_raw(ptr)
        }
    }
}

impl Default for AccelerationStructurePassDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create acceleration structure pass descriptor")
    }
}

impl Clone for AccelerationStructurePassDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy acceleration structure pass descriptor")
        }
    }
}

impl Drop for AccelerationStructurePassDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for AccelerationStructurePassDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for AccelerationStructurePassDescriptor {}
unsafe impl Sync for AccelerationStructurePassDescriptor {}

// ============================================================================
// AccelerationStructurePassSampleBufferAttachmentDescriptor
// ============================================================================

/// Descriptor for sample buffer attachments in an acceleration structure pass.
///
/// C++ equivalent: `MTL::AccelerationStructurePassSampleBufferAttachmentDescriptor`
#[repr(transparent)]
pub struct AccelerationStructurePassSampleBufferAttachmentDescriptor(pub(crate) NonNull<c_void>);

impl AccelerationStructurePassSampleBufferAttachmentDescriptor {
    /// Create a new sample buffer attachment descriptor.
    ///
    /// C++ equivalent: `static AccelerationStructurePassSampleBufferAttachmentDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get(
                "MTLAccelerationStructurePassSampleBufferAttachmentDescriptor",
            )?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal sample buffer attachment descriptor.
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
    /// # Safety
    ///
    /// Returns raw pointer to CounterSampleBuffer.
    ///
    /// C++ equivalent: `CounterSampleBuffer* sampleBuffer() const`
    pub fn sample_buffer_ptr(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(sampleBuffer)) }
    }

    /// Set the sample buffer.
    ///
    /// # Safety
    ///
    /// The sample_buffer pointer must be valid or null.
    ///
    /// C++ equivalent: `void setSampleBuffer(CounterSampleBuffer*)`
    pub unsafe fn set_sample_buffer_ptr(&self, sample_buffer: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setSampleBuffer:), sample_buffer);
        }
    }

    /// Get the start of encoder sample index.
    ///
    /// C++ equivalent: `NS::UInteger startOfEncoderSampleIndex() const`
    #[inline]
    pub fn start_of_encoder_sample_index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(startOfEncoderSampleIndex)) }
    }

    /// Set the start of encoder sample index.
    ///
    /// C++ equivalent: `void setStartOfEncoderSampleIndex(NS::UInteger)`
    #[inline]
    pub fn set_start_of_encoder_sample_index(&self, index: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setStartOfEncoderSampleIndex:), index);
        }
    }

    /// Get the end of encoder sample index.
    ///
    /// C++ equivalent: `NS::UInteger endOfEncoderSampleIndex() const`
    #[inline]
    pub fn end_of_encoder_sample_index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(endOfEncoderSampleIndex)) }
    }

    /// Set the end of encoder sample index.
    ///
    /// C++ equivalent: `void setEndOfEncoderSampleIndex(NS::UInteger)`
    #[inline]
    pub fn set_end_of_encoder_sample_index(&self, index: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setEndOfEncoderSampleIndex:), index);
        }
    }
}

impl Default for AccelerationStructurePassSampleBufferAttachmentDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create sample buffer attachment descriptor")
    }
}

impl Clone for AccelerationStructurePassSampleBufferAttachmentDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy sample buffer attachment descriptor")
        }
    }
}

impl Drop for AccelerationStructurePassSampleBufferAttachmentDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for AccelerationStructurePassSampleBufferAttachmentDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for AccelerationStructurePassSampleBufferAttachmentDescriptor {}
unsafe impl Sync for AccelerationStructurePassSampleBufferAttachmentDescriptor {}

// ============================================================================
// AccelerationStructurePassSampleBufferAttachmentDescriptorArray
// ============================================================================

/// Array of sample buffer attachment descriptors for an acceleration structure pass.
///
/// C++ equivalent: `MTL::AccelerationStructurePassSampleBufferAttachmentDescriptorArray`
#[repr(transparent)]
pub struct AccelerationStructurePassSampleBufferAttachmentDescriptorArray(
    pub(crate) NonNull<c_void>,
);

impl AccelerationStructurePassSampleBufferAttachmentDescriptorArray {
    /// Create a new sample buffer attachment descriptor array.
    ///
    /// C++ equivalent: `static AccelerationStructurePassSampleBufferAttachmentDescriptorArray* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get(
                "MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray",
            )?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal sample buffer attachment descriptor array.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the attachment at the specified index.
    ///
    /// C++ equivalent: `AccelerationStructurePassSampleBufferAttachmentDescriptor* object(NS::UInteger)`
    pub fn object(
        &self,
        index: UInteger,
    ) -> Option<AccelerationStructurePassSampleBufferAttachmentDescriptor> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(objectAtIndexedSubscript:), index);
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            AccelerationStructurePassSampleBufferAttachmentDescriptor::from_raw(ptr)
        }
    }

    /// Set the attachment at the specified index.
    ///
    /// C++ equivalent: `void setObject(AccelerationStructurePassSampleBufferAttachmentDescriptor*, NS::UInteger)`
    pub fn set_object(
        &self,
        attachment: Option<&AccelerationStructurePassSampleBufferAttachmentDescriptor>,
        index: UInteger,
    ) {
        unsafe {
            msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setObject:atIndexedSubscript:),
                attachment.map_or(std::ptr::null(), |a| a.as_ptr()),
                index,
            );
        }
    }
}

impl Default for AccelerationStructurePassSampleBufferAttachmentDescriptorArray {
    fn default() -> Self {
        Self::new().expect("failed to create sample buffer attachment descriptor array")
    }
}

impl Clone for AccelerationStructurePassSampleBufferAttachmentDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for AccelerationStructurePassSampleBufferAttachmentDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for AccelerationStructurePassSampleBufferAttachmentDescriptorArray {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for AccelerationStructurePassSampleBufferAttachmentDescriptorArray {}
unsafe impl Sync for AccelerationStructurePassSampleBufferAttachmentDescriptorArray {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceleration_structure_pass_descriptor_creation() {
        let desc = AccelerationStructurePassDescriptor::new();
        assert!(desc.is_some());
    }

    #[test]
    fn test_acceleration_structure_pass_descriptor_class_method() {
        let desc = AccelerationStructurePassDescriptor::descriptor();
        assert!(desc.is_some());
    }

    #[test]
    fn test_sample_buffer_attachment_descriptor() {
        let desc = AccelerationStructurePassSampleBufferAttachmentDescriptor::new();
        assert!(desc.is_some());
    }

    #[test]
    fn test_sample_buffer_attachment_descriptor_array() {
        let arr = AccelerationStructurePassSampleBufferAttachmentDescriptorArray::new();
        assert!(arr.is_some());
    }
}
