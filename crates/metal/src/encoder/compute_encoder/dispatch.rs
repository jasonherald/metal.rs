//! Dispatch methods for ComputeCommandEncoder.

use std::ffi::c_void;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_1, sel};

use crate::types::{Region, Size};
use crate::Buffer;

use super::ComputeCommandEncoder;

impl ComputeCommandEncoder {
    // =========================================================================
    // Stage-In Region
    // =========================================================================

    /// Set the stage-in region.
    ///
    /// C++ equivalent: `void setStageInRegion(MTL::Region)`
    #[inline]
    pub fn set_stage_in_region(&self, region: Region) {
        unsafe {
            msg_send_1::<(), Region>(self.as_ptr(), sel!(setStageInRegion:), region);
        }
    }

    /// Set the stage-in region from an indirect buffer.
    ///
    /// C++ equivalent: `void setStageInRegion(const Buffer*, NS::UInteger)`
    #[inline]
    pub fn set_stage_in_region_with_indirect_buffer(
        &self,
        indirect_buffer: &Buffer,
        indirect_buffer_offset: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setStageInRegionWithIndirectBuffer: indirectBufferOffset:),
                indirect_buffer.as_ptr(),
                indirect_buffer_offset,
            );
        }
    }

    // =========================================================================
    // Dispatch
    // =========================================================================

    /// Dispatch threadgroups.
    ///
    /// C++ equivalent: `void dispatchThreadgroups(MTL::Size, MTL::Size)`
    #[inline]
    pub fn dispatch_threadgroups(
        &self,
        threadgroups_per_grid: Size,
        threads_per_threadgroup: Size,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), Size, Size>(
                self.as_ptr(),
                sel!(dispatchThreadgroups: threadsPerThreadgroup:),
                threadgroups_per_grid,
                threads_per_threadgroup,
            );
        }
    }

    /// Dispatch threadgroups with an indirect buffer.
    ///
    /// C++ equivalent: `void dispatchThreadgroups(const Buffer*, NS::UInteger, MTL::Size)`
    #[inline]
    pub fn dispatch_threadgroups_with_indirect_buffer(
        &self,
        indirect_buffer: &Buffer,
        indirect_buffer_offset: UInteger,
        threads_per_threadgroup: Size,
    ) {
        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, UInteger, Size>(
                self.as_ptr(),
                sel!(dispatchThreadgroupsWithIndirectBuffer: indirectBufferOffset: threadsPerThreadgroup:),
                indirect_buffer.as_ptr(),
                indirect_buffer_offset,
                threads_per_threadgroup,
            );
        }
    }

    /// Dispatch threads directly (non-uniform dispatch).
    ///
    /// C++ equivalent: `void dispatchThreads(MTL::Size, MTL::Size)`
    #[inline]
    pub fn dispatch_threads(&self, threads_per_grid: Size, threads_per_threadgroup: Size) {
        unsafe {
            metal_sys::msg_send_2::<(), Size, Size>(
                self.as_ptr(),
                sel!(dispatchThreads: threadsPerThreadgroup:),
                threads_per_grid,
                threads_per_threadgroup,
            );
        }
    }
}
