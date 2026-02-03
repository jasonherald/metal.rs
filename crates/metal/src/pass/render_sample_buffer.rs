//! Render pass sample buffer attachment descriptors.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

/// A sample buffer attachment for a render pass.
///
/// C++ equivalent: `MTL::RenderPassSampleBufferAttachmentDescriptor`
///
/// Note: This type has separate indices for vertex and fragment stages.
#[repr(transparent)]
pub struct RenderPassSampleBufferAttachmentDescriptor(pub(crate) NonNull<c_void>);

impl RenderPassSampleBufferAttachmentDescriptor {
    /// Create a new render pass sample buffer attachment descriptor.
    ///
    /// C++ equivalent: `static RenderPassSampleBufferAttachmentDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLRenderPassSampleBufferAttachmentDescriptor")?;
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

    /// Get the start of vertex sample index.
    ///
    /// C++ equivalent: `NS::UInteger startOfVertexSampleIndex() const`
    #[inline]
    pub fn start_of_vertex_sample_index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(startOfVertexSampleIndex)) }
    }

    /// Set the start of vertex sample index.
    ///
    /// C++ equivalent: `void setStartOfVertexSampleIndex(NS::UInteger startOfVertexSampleIndex)`
    #[inline]
    pub fn set_start_of_vertex_sample_index(&self, index: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setStartOfVertexSampleIndex:), index);
        }
    }

    /// Get the end of vertex sample index.
    ///
    /// C++ equivalent: `NS::UInteger endOfVertexSampleIndex() const`
    #[inline]
    pub fn end_of_vertex_sample_index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(endOfVertexSampleIndex)) }
    }

    /// Set the end of vertex sample index.
    ///
    /// C++ equivalent: `void setEndOfVertexSampleIndex(NS::UInteger endOfVertexSampleIndex)`
    #[inline]
    pub fn set_end_of_vertex_sample_index(&self, index: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setEndOfVertexSampleIndex:), index);
        }
    }

    /// Get the start of fragment sample index.
    ///
    /// C++ equivalent: `NS::UInteger startOfFragmentSampleIndex() const`
    #[inline]
    pub fn start_of_fragment_sample_index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(startOfFragmentSampleIndex)) }
    }

    /// Set the start of fragment sample index.
    ///
    /// C++ equivalent: `void setStartOfFragmentSampleIndex(NS::UInteger startOfFragmentSampleIndex)`
    #[inline]
    pub fn set_start_of_fragment_sample_index(&self, index: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setStartOfFragmentSampleIndex:), index);
        }
    }

    /// Get the end of fragment sample index.
    ///
    /// C++ equivalent: `NS::UInteger endOfFragmentSampleIndex() const`
    #[inline]
    pub fn end_of_fragment_sample_index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(endOfFragmentSampleIndex)) }
    }

    /// Set the end of fragment sample index.
    ///
    /// C++ equivalent: `void setEndOfFragmentSampleIndex(NS::UInteger endOfFragmentSampleIndex)`
    #[inline]
    pub fn set_end_of_fragment_sample_index(&self, index: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setEndOfFragmentSampleIndex:), index);
        }
    }
}

impl Clone for RenderPassSampleBufferAttachmentDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for RenderPassSampleBufferAttachmentDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for RenderPassSampleBufferAttachmentDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RenderPassSampleBufferAttachmentDescriptor {}
unsafe impl Sync for RenderPassSampleBufferAttachmentDescriptor {}

impl std::fmt::Debug for RenderPassSampleBufferAttachmentDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderPassSampleBufferAttachmentDescriptor")
            .field(
                "start_of_vertex_sample_index",
                &self.start_of_vertex_sample_index(),
            )
            .field(
                "end_of_vertex_sample_index",
                &self.end_of_vertex_sample_index(),
            )
            .field(
                "start_of_fragment_sample_index",
                &self.start_of_fragment_sample_index(),
            )
            .field(
                "end_of_fragment_sample_index",
                &self.end_of_fragment_sample_index(),
            )
            .finish()
    }
}

/// An array of render pass sample buffer attachment descriptors.
///
/// C++ equivalent: `MTL::RenderPassSampleBufferAttachmentDescriptorArray`
#[repr(transparent)]
pub struct RenderPassSampleBufferAttachmentDescriptorArray(pub(crate) NonNull<c_void>);

impl RenderPassSampleBufferAttachmentDescriptorArray {
    /// Create a new array.
    ///
    /// C++ equivalent: `static RenderPassSampleBufferAttachmentDescriptorArray* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class =
                metal_sys::Class::get("MTLRenderPassSampleBufferAttachmentDescriptorArray")?;
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
    /// C++ equivalent: `RenderPassSampleBufferAttachmentDescriptor* object(NS::UInteger attachmentIndex)`
    pub fn object(&self, index: UInteger) -> Option<RenderPassSampleBufferAttachmentDescriptor> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(objectAtIndexedSubscript:), index);
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr, sel!(retain));
            RenderPassSampleBufferAttachmentDescriptor::from_raw(ptr)
        }
    }

    /// Set the descriptor at the specified index.
    ///
    /// C++ equivalent: `void setObject(const MTL::RenderPassSampleBufferAttachmentDescriptor* attachment, NS::UInteger attachmentIndex)`
    pub fn set_object(
        &self,
        attachment: Option<&RenderPassSampleBufferAttachmentDescriptor>,
        index: UInteger,
    ) {
        unsafe {
            let ptr = attachment.map_or(std::ptr::null(), |a| a.as_ptr());
            let _: () = metal_sys::msg_send_2(
                self.as_ptr(),
                sel!(setObject:atIndexedSubscript:),
                ptr,
                index,
            );
        }
    }
}

impl Clone for RenderPassSampleBufferAttachmentDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for RenderPassSampleBufferAttachmentDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for RenderPassSampleBufferAttachmentDescriptorArray {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RenderPassSampleBufferAttachmentDescriptorArray {}
unsafe impl Sync for RenderPassSampleBufferAttachmentDescriptorArray {}

impl std::fmt::Debug for RenderPassSampleBufferAttachmentDescriptorArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderPassSampleBufferAttachmentDescriptorArray")
            .finish()
    }
}
