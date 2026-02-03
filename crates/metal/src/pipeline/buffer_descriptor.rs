//! Pipeline buffer descriptors.
//!
//! Corresponds to `MTL::PipelineBufferDescriptor` and `MTL::PipelineBufferDescriptorArray`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

pub struct PipelineBufferDescriptor(pub(crate) NonNull<c_void>);

impl PipelineBufferDescriptor {
    /// Create a new pipeline buffer descriptor.
    ///
    /// C++ equivalent: `static PipelineBufferDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLPipelineBufferDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a PipelineBufferDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal pipeline buffer descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the descriptor.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the mutability of the buffer.
    ///
    /// C++ equivalent: `Mutability mutability() const`
    #[inline]
    pub fn mutability(&self) -> crate::enums::Mutability {
        unsafe { msg_send_0(self.as_ptr(), sel!(mutability)) }
    }

    /// Set the mutability of the buffer.
    ///
    /// C++ equivalent: `void setMutability(MTL::Mutability mutability)`
    #[inline]
    pub fn set_mutability(&self, mutability: crate::enums::Mutability) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMutability:), mutability);
        }
    }
}

impl Clone for PipelineBufferDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for PipelineBufferDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for PipelineBufferDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for PipelineBufferDescriptor {}
unsafe impl Sync for PipelineBufferDescriptor {}

impl std::fmt::Debug for PipelineBufferDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipelineBufferDescriptor")
            .field("mutability", &self.mutability())
            .finish()
    }
}

// ============================================================================
// PipelineBufferDescriptorArray
// ============================================================================

/// An array of pipeline buffer descriptors.
///
/// C++ equivalent: `MTL::PipelineBufferDescriptorArray`
#[repr(transparent)]
pub struct PipelineBufferDescriptorArray(pub(crate) NonNull<c_void>);

impl PipelineBufferDescriptorArray {
    /// Create a new pipeline buffer descriptor array.
    ///
    /// C++ equivalent: `static PipelineBufferDescriptorArray* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLPipelineBufferDescriptorArray")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a PipelineBufferDescriptorArray from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal pipeline buffer descriptor array object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the array.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Array Access
    // =========================================================================

    /// Get the descriptor at the specified buffer index.
    ///
    /// C++ equivalent: `PipelineBufferDescriptor* object(NS::UInteger bufferIndex)`
    pub fn object(&self, buffer_index: UInteger) -> Option<PipelineBufferDescriptor> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(objectAtIndexedSubscript:), buffer_index);
            if ptr.is_null() {
                return None;
            }
            // Retain since we're returning an owned reference
            msg_send_0::<*mut c_void>(ptr, sel!(retain));
            PipelineBufferDescriptor::from_raw(ptr)
        }
    }

    /// Set the descriptor at the specified buffer index.
    ///
    /// C++ equivalent: `void setObject(const MTL::PipelineBufferDescriptor* buffer, NS::UInteger bufferIndex)`
    pub fn set_object(&self, buffer: &PipelineBufferDescriptor, buffer_index: UInteger) {
        unsafe {
            let _: () = metal_sys::msg_send_2(
                self.as_ptr(),
                sel!(setObject:atIndexedSubscript:),
                buffer.as_ptr(),
                buffer_index,
            );
        }
    }
}

impl Clone for PipelineBufferDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for PipelineBufferDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for PipelineBufferDescriptorArray {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for PipelineBufferDescriptorArray {}
unsafe impl Sync for PipelineBufferDescriptorArray {}

impl std::fmt::Debug for PipelineBufferDescriptorArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipelineBufferDescriptorArray").finish()
    }
}
