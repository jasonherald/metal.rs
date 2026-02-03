//! MTL4 ComputePipeline implementation.
//!
//! Corresponds to `Metal/MTL4ComputePipeline.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use super::enums::IndirectCommandBufferSupportState;
use super::{FunctionDescriptor, PipelineOptions, StaticLinkingDescriptor};
use crate::Size;

// ============================================================
// ComputePipelineDescriptor
// ============================================================

/// Descriptor for MTL4 compute pipelines.
///
/// C++ equivalent: `MTL4::ComputePipelineDescriptor`
///
/// ComputePipelineDescriptor configures the compute function and
/// threadgroup settings for a compute pipeline.
#[repr(transparent)]
pub struct ComputePipelineDescriptor(NonNull<c_void>);

impl ComputePipelineDescriptor {
    /// Create a ComputePipelineDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new compute pipeline descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTL4ComputePipelineDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    // ========== Base Pipeline Properties ==========

    /// Get the label.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ns_string: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ns_string.is_null() {
                return None;
            }
            let c_str: *const i8 = msg_send_0(ns_string, sel!(UTF8String));
            if c_str.is_null() {
                return None;
            }
            Some(
                std::ffi::CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    }

    /// Set the label.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = mtl_foundation::String::from_str(label) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the pipeline options.
    ///
    /// C++ equivalent: `PipelineOptions* options() const`
    pub fn options(&self) -> Option<PipelineOptions> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(options));
            PipelineOptions::from_raw(ptr)
        }
    }

    /// Set the pipeline options.
    ///
    /// C++ equivalent: `void setOptions(const MTL4::PipelineOptions*)`
    pub fn set_options(&self, options: &PipelineOptions) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOptions:), options.as_ptr());
        }
    }

    // ========== Compute-Specific Properties ==========

    /// Get the compute function descriptor.
    ///
    /// C++ equivalent: `FunctionDescriptor* computeFunctionDescriptor() const`
    pub fn compute_function_descriptor(&self) -> Option<FunctionDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(computeFunctionDescriptor));
            FunctionDescriptor::from_raw(ptr)
        }
    }

    /// Set the compute function descriptor.
    ///
    /// C++ equivalent: `void setComputeFunctionDescriptor(const MTL4::FunctionDescriptor*)`
    pub fn set_compute_function_descriptor(&self, descriptor: &FunctionDescriptor) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setComputeFunctionDescriptor:),
                descriptor.as_ptr(),
            );
        }
    }

    /// Get the maximum total threads per threadgroup.
    ///
    /// C++ equivalent: `NS::UInteger maxTotalThreadsPerThreadgroup() const`
    pub fn max_total_threads_per_threadgroup(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxTotalThreadsPerThreadgroup)) }
    }

    /// Set the maximum total threads per threadgroup.
    ///
    /// C++ equivalent: `void setMaxTotalThreadsPerThreadgroup(NS::UInteger)`
    pub fn set_max_total_threads_per_threadgroup(&self, max_threads: UInteger) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setMaxTotalThreadsPerThreadgroup:),
                max_threads,
            );
        }
    }

    /// Get the required threads per threadgroup.
    ///
    /// C++ equivalent: `MTL::Size requiredThreadsPerThreadgroup() const`
    pub fn required_threads_per_threadgroup(&self) -> Size {
        unsafe { msg_send_0(self.as_ptr(), sel!(requiredThreadsPerThreadgroup)) }
    }

    /// Set the required threads per threadgroup.
    ///
    /// C++ equivalent: `void setRequiredThreadsPerThreadgroup(MTL::Size)`
    pub fn set_required_threads_per_threadgroup(&self, size: Size) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setRequiredThreadsPerThreadgroup:), size);
        }
    }

    /// Get the static linking descriptor.
    ///
    /// C++ equivalent: `StaticLinkingDescriptor* staticLinkingDescriptor() const`
    pub fn static_linking_descriptor(&self) -> Option<StaticLinkingDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(staticLinkingDescriptor));
            StaticLinkingDescriptor::from_raw(ptr)
        }
    }

    /// Set the static linking descriptor.
    ///
    /// C++ equivalent: `void setStaticLinkingDescriptor(const MTL4::StaticLinkingDescriptor*)`
    pub fn set_static_linking_descriptor(&self, descriptor: &StaticLinkingDescriptor) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setStaticLinkingDescriptor:),
                descriptor.as_ptr(),
            );
        }
    }

    /// Get whether binary linking is supported.
    ///
    /// C++ equivalent: `bool supportBinaryLinking() const`
    pub fn support_binary_linking(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportBinaryLinking)) }
    }

    /// Set whether binary linking is supported.
    ///
    /// C++ equivalent: `void setSupportBinaryLinking(bool)`
    pub fn set_support_binary_linking(&self, support: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setSupportBinaryLinking:), support);
        }
    }

    /// Get the indirect command buffer support state.
    ///
    /// C++ equivalent: `IndirectCommandBufferSupportState supportIndirectCommandBuffers() const`
    pub fn support_indirect_command_buffers(&self) -> IndirectCommandBufferSupportState {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportIndirectCommandBuffers)) }
    }

    /// Set the indirect command buffer support state.
    ///
    /// C++ equivalent: `void setSupportIndirectCommandBuffers(MTL4::IndirectCommandBufferSupportState)`
    pub fn set_support_indirect_command_buffers(&self, state: IndirectCommandBufferSupportState) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setSupportIndirectCommandBuffers:),
                state,
            );
        }
    }

    /// Get whether threadgroup size is a multiple of thread execution width.
    ///
    /// C++ equivalent: `bool threadGroupSizeIsMultipleOfThreadExecutionWidth() const`
    pub fn thread_group_size_is_multiple_of_thread_execution_width(&self) -> bool {
        unsafe {
            msg_send_0(
                self.as_ptr(),
                sel!(threadGroupSizeIsMultipleOfThreadExecutionWidth),
            )
        }
    }

    /// Set whether threadgroup size is a multiple of thread execution width.
    ///
    /// C++ equivalent: `void setThreadGroupSizeIsMultipleOfThreadExecutionWidth(bool)`
    pub fn set_thread_group_size_is_multiple_of_thread_execution_width(&self, value: bool) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setThreadGroupSizeIsMultipleOfThreadExecutionWidth:),
                value,
            );
        }
    }

    /// Reset the descriptor to its default state.
    ///
    /// C++ equivalent: `void reset()`
    pub fn reset(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(reset));
        }
    }
}

impl Clone for ComputePipelineDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ComputePipelineDescriptor {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for ComputePipelineDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ComputePipelineDescriptor {}
unsafe impl Sync for ComputePipelineDescriptor {}

impl std::fmt::Debug for ComputePipelineDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComputePipelineDescriptor")
            .field("label", &self.label())
            .field(
                "max_total_threads_per_threadgroup",
                &self.max_total_threads_per_threadgroup(),
            )
            .field(
                "required_threads_per_threadgroup",
                &self.required_threads_per_threadgroup(),
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_pipeline_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<ComputePipelineDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
