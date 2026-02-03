//! A buffer that stores GPU commands for indirect execution.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::types::ResourceID;

use super::{IndirectComputeCommand, IndirectRenderCommand};

/// A buffer that stores GPU commands for indirect execution.
///
/// C++ equivalent: `MTL::IndirectCommandBuffer`
///
/// Indirect command buffers are created from a device and can be populated
/// with commands either from the CPU or from GPU compute shaders.
#[repr(transparent)]
pub struct IndirectCommandBuffer(NonNull<c_void>);

impl IndirectCommandBuffer {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal indirect command buffer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the number of commands this buffer can hold.
    ///
    /// C++ equivalent: `NS::UInteger size() const`
    #[inline]
    pub fn size(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(size)) }
    }

    /// Get the GPU resource ID for bindless access.
    ///
    /// C++ equivalent: `ResourceID gpuResourceID() const`
    #[inline]
    pub fn gpu_resource_id(&self) -> ResourceID {
        unsafe { msg_send_0(self.as_ptr(), sel!(gpuResourceID)) }
    }

    /// Reset commands in the specified range.
    ///
    /// C++ equivalent: `void reset(NS::Range)`
    pub fn reset(&self, location: UInteger, length: UInteger) {
        unsafe {
            let range = metal_foundation::Range::new(location, length);
            msg_send_1::<(), metal_foundation::Range>(self.as_ptr(), sel!(resetWithRange:), range);
        }
    }

    /// Get an indirect render command at the specified index.
    ///
    /// C++ equivalent: `IndirectRenderCommand* indirectRenderCommand(NS::UInteger)`
    pub fn indirect_render_command(&self, index: UInteger) -> Option<IndirectRenderCommand> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(indirectRenderCommandAtIndex:), index);
            IndirectRenderCommand::from_raw(ptr)
        }
    }

    /// Get an indirect compute command at the specified index.
    ///
    /// C++ equivalent: `IndirectComputeCommand* indirectComputeCommand(NS::UInteger)`
    pub fn indirect_compute_command(&self, index: UInteger) -> Option<IndirectComputeCommand> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(indirectComputeCommandAtIndex:), index);
            IndirectComputeCommand::from_raw(ptr)
        }
    }
}

impl Clone for IndirectCommandBuffer {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for IndirectCommandBuffer {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for IndirectCommandBuffer {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for IndirectCommandBuffer {}
unsafe impl Sync for IndirectCommandBuffer {}

impl std::fmt::Debug for IndirectCommandBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndirectCommandBuffer")
            .field("size", &self.size())
            .finish()
    }
}
