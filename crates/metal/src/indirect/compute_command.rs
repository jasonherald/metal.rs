//! A compute command within an indirect command buffer.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, sel};

use crate::types::{Region, Size};
use crate::{Buffer, ComputePipelineState};

/// A compute command within an indirect command buffer.
///
/// C++ equivalent: `MTL::IndirectComputeCommand`
///
/// Indirect compute commands can encode dispatch calls and state changes
/// that will be executed when the indirect command buffer is executed.
#[repr(transparent)]
pub struct IndirectComputeCommand(NonNull<c_void>);

impl IndirectComputeCommand {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal indirect compute command.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Reset this command.
    ///
    /// C++ equivalent: `void reset()`
    #[inline]
    pub fn reset(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(reset));
        }
    }

    /// Set a barrier for this command.
    ///
    /// C++ equivalent: `void setBarrier()`
    #[inline]
    pub fn set_barrier(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(setBarrier));
        }
    }

    /// Clear a barrier for this command.
    ///
    /// C++ equivalent: `void clearBarrier()`
    #[inline]
    pub fn clear_barrier(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(clearBarrier));
        }
    }

    /// Set the compute pipeline state.
    ///
    /// C++ equivalent: `void setComputePipelineState(const ComputePipelineState*)`
    pub fn set_compute_pipeline_state(&self, state: &ComputePipelineState) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setComputePipelineState:),
                state.as_ptr(),
            );
        }
    }

    /// Set a kernel buffer.
    ///
    /// C++ equivalent: `void setKernelBuffer(const Buffer*, NS::UInteger offset, NS::UInteger index)`
    pub fn set_kernel_buffer(&self, buffer: &Buffer, offset: UInteger, index: UInteger) {
        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setKernelBuffer: offset: atIndex:),
                buffer.as_ptr(),
                offset,
                index,
            );
        }
    }

    /// Set a kernel buffer with stride.
    ///
    /// C++ equivalent: `void setKernelBuffer(const Buffer*, NS::UInteger, NS::UInteger, NS::UInteger)`
    pub fn set_kernel_buffer_with_stride(
        &self,
        buffer: &Buffer,
        offset: UInteger,
        stride: UInteger,
        index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_4::<(), *const c_void, UInteger, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setKernelBuffer: offset: attributeStride: atIndex:),
                buffer.as_ptr(),
                offset,
                stride,
                index,
            );
        }
    }

    /// Set threadgroup memory length.
    ///
    /// C++ equivalent: `void setThreadgroupMemoryLength(NS::UInteger, NS::UInteger)`
    pub fn set_threadgroup_memory_length(&self, length: UInteger, index: UInteger) {
        unsafe {
            msg_send_2::<(), UInteger, UInteger>(
                self.as_ptr(),
                sel!(setThreadgroupMemoryLength: atIndex:),
                length,
                index,
            );
        }
    }

    /// Set the stage-in region.
    ///
    /// C++ equivalent: `void setStageInRegion(Region)`
    #[inline]
    pub fn set_stage_in_region(&self, region: Region) {
        unsafe {
            msg_send_1::<(), Region>(self.as_ptr(), sel!(setStageInRegion:), region);
        }
    }

    /// Set the imageblock dimensions.
    ///
    /// C++ equivalent: `void setImageblockWidth(NS::UInteger, NS::UInteger)`
    pub fn set_imageblock_width(&self, width: UInteger, height: UInteger) {
        unsafe {
            msg_send_2::<(), UInteger, UInteger>(
                self.as_ptr(),
                sel!(setImageblockWidth: height:),
                width,
                height,
            );
        }
    }

    /// Dispatch threadgroups concurrently.
    ///
    /// C++ equivalent: `void concurrentDispatchThreadgroups(...)`
    pub fn concurrent_dispatch_threadgroups(
        &self,
        threadgroups_per_grid: Size,
        threads_per_threadgroup: Size,
    ) {
        unsafe {
            msg_send_2::<(), Size, Size>(
                self.as_ptr(),
                sel!(concurrentDispatchThreadgroups: threadsPerThreadgroup:),
                threadgroups_per_grid,
                threads_per_threadgroup,
            );
        }
    }

    /// Dispatch threads concurrently.
    ///
    /// C++ equivalent: `void concurrentDispatchThreads(...)`
    pub fn concurrent_dispatch_threads(
        &self,
        threads_per_grid: Size,
        threads_per_threadgroup: Size,
    ) {
        unsafe {
            msg_send_2::<(), Size, Size>(
                self.as_ptr(),
                sel!(concurrentDispatchThreads: threadsPerThreadgroup:),
                threads_per_grid,
                threads_per_threadgroup,
            );
        }
    }
}

impl Referencing for IndirectComputeCommand {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

// Note: IndirectComputeCommand is not reference counted - it's a view into the ICB

impl std::fmt::Debug for IndirectComputeCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndirectComputeCommand").finish()
    }
}
