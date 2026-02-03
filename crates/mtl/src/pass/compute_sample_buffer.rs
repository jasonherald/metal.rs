//! Compute pass sample buffer attachment descriptors.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

/// A sample buffer attachment for a compute pass.
///
/// C++ equivalent: `MTL::ComputePassSampleBufferAttachmentDescriptor`
#[repr(transparent)]
pub struct ComputePassSampleBufferAttachmentDescriptor(pub(crate) NonNull<c_void>);

impl ComputePassSampleBufferAttachmentDescriptor {
    /// Create a new compute pass sample buffer attachment descriptor.
    ///
    /// C++ equivalent: `static ComputePassSampleBufferAttachmentDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLComputePassSampleBufferAttachmentDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
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
    pub fn sample_buffer(&self) -> Option<crate::CounterSampleBuffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(sampleBuffer));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr, sel!(retain));
            crate::CounterSampleBuffer::from_raw(ptr)
        }
    }

    /// Set the sample buffer.
    ///
    /// C++ equivalent: `void setSampleBuffer(const MTL::CounterSampleBuffer* sampleBuffer)`
    pub fn set_sample_buffer(&self, sample_buffer: Option<&crate::CounterSampleBuffer>) {
        unsafe {
            let ptr = sample_buffer.map_or(std::ptr::null(), |s| s.as_ptr());
            let _: () = msg_send_1(self.as_ptr(), sel!(setSampleBuffer:), ptr);
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
    /// C++ equivalent: `void setStartOfEncoderSampleIndex(NS::UInteger startOfEncoderSampleIndex)`
    #[inline]
    pub fn set_start_of_encoder_sample_index(&self, index: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setStartOfEncoderSampleIndex:), index);
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
    /// C++ equivalent: `void setEndOfEncoderSampleIndex(NS::UInteger endOfEncoderSampleIndex)`
    #[inline]
    pub fn set_end_of_encoder_sample_index(&self, index: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setEndOfEncoderSampleIndex:), index);
        }
    }
}

impl Clone for ComputePassSampleBufferAttachmentDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ComputePassSampleBufferAttachmentDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for ComputePassSampleBufferAttachmentDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ComputePassSampleBufferAttachmentDescriptor {}
unsafe impl Sync for ComputePassSampleBufferAttachmentDescriptor {}

impl std::fmt::Debug for ComputePassSampleBufferAttachmentDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComputePassSampleBufferAttachmentDescriptor")
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

/// An array of compute pass sample buffer attachment descriptors.
///
/// C++ equivalent: `MTL::ComputePassSampleBufferAttachmentDescriptorArray`
#[repr(transparent)]
pub struct ComputePassSampleBufferAttachmentDescriptorArray(pub(crate) NonNull<c_void>);

impl ComputePassSampleBufferAttachmentDescriptorArray {
    /// Create a new array.
    ///
    /// C++ equivalent: `static ComputePassSampleBufferAttachmentDescriptorArray* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class =
                mtl_sys::Class::get("MTLComputePassSampleBufferAttachmentDescriptorArray")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
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
    /// C++ equivalent: `ComputePassSampleBufferAttachmentDescriptor* object(NS::UInteger attachmentIndex)`
    pub fn object(&self, index: UInteger) -> Option<ComputePassSampleBufferAttachmentDescriptor> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(objectAtIndexedSubscript:), index);
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr, sel!(retain));
            ComputePassSampleBufferAttachmentDescriptor::from_raw(ptr)
        }
    }

    /// Set the descriptor at the specified index.
    ///
    /// C++ equivalent: `void setObject(const MTL::ComputePassSampleBufferAttachmentDescriptor* attachment, NS::UInteger attachmentIndex)`
    pub fn set_object(
        &self,
        attachment: Option<&ComputePassSampleBufferAttachmentDescriptor>,
        index: UInteger,
    ) {
        unsafe {
            let ptr = attachment.map_or(std::ptr::null(), |a| a.as_ptr());
            let _: () = mtl_sys::msg_send_2(
                self.as_ptr(),
                sel!(setObject:atIndexedSubscript:),
                ptr,
                index,
            );
        }
    }
}

impl Clone for ComputePassSampleBufferAttachmentDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ComputePassSampleBufferAttachmentDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for ComputePassSampleBufferAttachmentDescriptorArray {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ComputePassSampleBufferAttachmentDescriptorArray {}
unsafe impl Sync for ComputePassSampleBufferAttachmentDescriptorArray {}

impl std::fmt::Debug for ComputePassSampleBufferAttachmentDescriptorArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComputePassSampleBufferAttachmentDescriptorArray")
            .finish()
    }
}
