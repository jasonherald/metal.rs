//! Compute command encoder.
//!
//! Corresponds to `Metal/MTLComputeCommandEncoder.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_0, msg_send_1, sel};

mod acceleration;
mod binding;
mod dispatch;
mod indirect;
mod memory;
mod pipeline;

/// Indirect arguments for dispatching threadgroups.
///
/// C++ equivalent: `MTL::DispatchThreadgroupsIndirectArguments`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DispatchThreadgroupsIndirectArguments {
    pub threadgroups_per_grid: [u32; 3],
}

/// Indirect arguments for dispatching threads.
///
/// C++ equivalent: `MTL::DispatchThreadsIndirectArguments`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DispatchThreadsIndirectArguments {
    pub threads_per_grid: [u32; 3],
    pub threads_per_threadgroup: [u32; 3],
}

/// Indirect arguments for stage-in region.
///
/// C++ equivalent: `MTL::StageInRegionIndirectArguments`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StageInRegionIndirectArguments {
    pub stage_in_origin: [u32; 3],
    pub stage_in_size: [u32; 3],
}

/// A command encoder for compute operations.
///
/// C++ equivalent: `MTL::ComputeCommandEncoder`
///
/// Compute encoders are used to dispatch compute shaders and manage
/// resources for compute operations.
#[repr(transparent)]
pub struct ComputeCommandEncoder(pub(crate) NonNull<c_void>);

impl ComputeCommandEncoder {
    /// Create a ComputeCommandEncoder from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal compute command encoder object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the encoder.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // CommandEncoder base methods
    // =========================================================================

    /// Get the device that created this encoder.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("encoder has no device")
        }
    }

    /// Get the command buffer that this encoder is encoding into.
    ///
    /// C++ equivalent: `CommandBuffer* commandBuffer() const`
    pub fn command_buffer(&self) -> crate::CommandBuffer {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(commandBuffer));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::CommandBuffer::from_raw(ptr).expect("encoder has no command buffer")
        }
    }

    /// Get the label for this encoder.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                mtl_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Set the label for this encoder.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = mtl_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// End encoding commands with this encoder.
    ///
    /// C++ equivalent: `void endEncoding()`
    #[inline]
    pub fn end_encoding(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(endEncoding));
        }
    }

    /// Insert a debug signpost.
    ///
    /// C++ equivalent: `void insertDebugSignpost(const NS::String*)`
    pub fn insert_debug_signpost(&self, string: &str) {
        if let Some(ns_string) = mtl_foundation::String::from_str(string) {
            unsafe {
                msg_send_1::<(), *const c_void>(
                    self.as_ptr(),
                    sel!(insertDebugSignpost:),
                    ns_string.as_ptr(),
                );
            }
        }
    }

    /// Push a debug group.
    ///
    /// C++ equivalent: `void pushDebugGroup(const NS::String*)`
    pub fn push_debug_group(&self, string: &str) {
        if let Some(ns_string) = mtl_foundation::String::from_str(string) {
            unsafe {
                msg_send_1::<(), *const c_void>(
                    self.as_ptr(),
                    sel!(pushDebugGroup:),
                    ns_string.as_ptr(),
                );
            }
        }
    }

    /// Pop the current debug group.
    ///
    /// C++ equivalent: `void popDebugGroup()`
    #[inline]
    pub fn pop_debug_group(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(popDebugGroup));
        }
    }

    /// Insert a barrier to synchronize queue stages.
    ///
    /// C++ equivalent: `void barrierAfterQueueStages(Stages, Stages)`
    #[inline]
    pub fn barrier_after_queue_stages(
        &self,
        after_stages: crate::enums::Stages,
        before_stages: crate::enums::Stages,
    ) {
        unsafe {
            mtl_sys::msg_send_2::<(), crate::enums::Stages, crate::enums::Stages>(
                self.as_ptr(),
                sel!(barrierAfterQueueStages:beforeQueueStages:),
                after_stages,
                before_stages,
            );
        }
    }
}

impl Clone for ComputeCommandEncoder {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ComputeCommandEncoder {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for ComputeCommandEncoder {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ComputeCommandEncoder {}
unsafe impl Sync for ComputeCommandEncoder {}

impl std::fmt::Debug for ComputeCommandEncoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComputeCommandEncoder")
            .field("label", &self.label())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_encoder_size() {
        assert_eq!(
            std::mem::size_of::<ComputeCommandEncoder>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_dispatch_threadgroups_indirect_arguments_size() {
        assert_eq!(
            std::mem::size_of::<DispatchThreadgroupsIndirectArguments>(),
            12
        ); // 3 * 4 bytes
    }

    #[test]
    fn test_dispatch_threads_indirect_arguments_size() {
        assert_eq!(std::mem::size_of::<DispatchThreadsIndirectArguments>(), 24); // 6 * 4 bytes
    }

    #[test]
    fn test_stage_in_region_indirect_arguments_size() {
        assert_eq!(std::mem::size_of::<StageInRegionIndirectArguments>(), 24); // 6 * 4 bytes
    }
}
