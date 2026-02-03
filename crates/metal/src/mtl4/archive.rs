//! MTL4 Archive implementation.
//!
//! Corresponds to `Metal/MTL4Archive.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, msg_send_3, sel};

use super::{
    BinaryFunction, BinaryFunctionDescriptor, ComputePipelineDescriptor, PipelineDescriptor,
    PipelineStageDynamicLinkingDescriptor, RenderPipelineDynamicLinkingDescriptor,
};
use crate::{ComputePipelineState, RenderPipelineState};

/// Helper to create a generic error.
fn generic_error() -> metal_foundation::Error {
    metal_foundation::Error::error(std::ptr::null_mut(), -1, std::ptr::null_mut())
        .expect("failed to create error object")
}

// ============================================================
// Archive
// ============================================================

/// MTL4 pipeline archive.
///
/// C++ equivalent: `MTL4::Archive`
///
/// Archive provides access to pre-compiled pipelines and binary functions,
/// allowing for faster pipeline creation without runtime compilation.
#[repr(transparent)]
pub struct Archive(NonNull<c_void>);

impl Archive {
    /// Create an Archive from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

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
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    // ========== Binary Function Creation ==========

    /// Create a new binary function from the archive.
    ///
    /// C++ equivalent: `BinaryFunction* newBinaryFunction(const MTL4::BinaryFunctionDescriptor*, NS::Error**)`
    pub fn new_binary_function(
        &self,
        descriptor: &BinaryFunctionDescriptor,
    ) -> Result<BinaryFunction, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newBinaryFunctionWithDescriptor:error:),
                descriptor.as_ptr(),
                &mut error as *mut _,
            );
            if !error.is_null() {
                if let Some(err) = metal_foundation::Error::from_ptr(error) {
                    return Err(err);
                }
            }
            BinaryFunction::from_raw(ptr).ok_or_else(generic_error)
        }
    }

    // ========== Compute Pipeline Creation ==========

    /// Create a new compute pipeline state from the archive.
    ///
    /// C++ equivalent: `MTL::ComputePipelineState* newComputePipelineState(const MTL4::ComputePipelineDescriptor*, NS::Error**)`
    pub fn new_compute_pipeline_state(
        &self,
        descriptor: &ComputePipelineDescriptor,
    ) -> Result<ComputePipelineState, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newComputePipelineStateWithDescriptor:error:),
                descriptor.as_ptr(),
                &mut error as *mut _,
            );
            if !error.is_null() {
                if let Some(err) = metal_foundation::Error::from_ptr(error) {
                    return Err(err);
                }
            }
            ComputePipelineState::from_raw(ptr).ok_or_else(generic_error)
        }
    }

    /// Create a new compute pipeline state with dynamic linking from the archive.
    ///
    /// C++ equivalent: `MTL::ComputePipelineState* newComputePipelineState(..., dynamicLinkingDescriptor, ...)`
    pub fn new_compute_pipeline_state_with_dynamic_linking(
        &self,
        descriptor: &ComputePipelineDescriptor,
        dynamic_linking: &PipelineStageDynamicLinkingDescriptor,
    ) -> Result<ComputePipelineState, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newComputePipelineStateWithDescriptor:dynamicLinkingDescriptor:error:),
                descriptor.as_ptr(),
                dynamic_linking.as_ptr(),
                &mut error as *mut _,
            );
            if !error.is_null() {
                if let Some(err) = metal_foundation::Error::from_ptr(error) {
                    return Err(err);
                }
            }
            ComputePipelineState::from_raw(ptr).ok_or_else(generic_error)
        }
    }

    // ========== Render Pipeline Creation ==========

    /// Create a new render pipeline state from the archive.
    ///
    /// C++ equivalent: `MTL::RenderPipelineState* newRenderPipelineState(const MTL4::PipelineDescriptor*, NS::Error**)`
    pub fn new_render_pipeline_state(
        &self,
        descriptor: &PipelineDescriptor,
    ) -> Result<RenderPipelineState, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newRenderPipelineStateWithDescriptor:error:),
                descriptor.as_ptr(),
                &mut error as *mut _,
            );
            if !error.is_null() {
                if let Some(err) = metal_foundation::Error::from_ptr(error) {
                    return Err(err);
                }
            }
            RenderPipelineState::from_raw(ptr).ok_or_else(generic_error)
        }
    }

    /// Create a new render pipeline state with dynamic linking from the archive.
    ///
    /// C++ equivalent: `MTL::RenderPipelineState* newRenderPipelineState(..., dynamicLinkingDescriptor, ...)`
    pub fn new_render_pipeline_state_with_dynamic_linking(
        &self,
        descriptor: &PipelineDescriptor,
        dynamic_linking: &RenderPipelineDynamicLinkingDescriptor,
    ) -> Result<RenderPipelineState, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newRenderPipelineStateWithDescriptor:dynamicLinkingDescriptor:error:),
                descriptor.as_ptr(),
                dynamic_linking.as_ptr(),
                &mut error as *mut _,
            );
            if !error.is_null() {
                if let Some(err) = metal_foundation::Error::from_ptr(error) {
                    return Err(err);
                }
            }
            RenderPipelineState::from_raw(ptr).ok_or_else(generic_error)
        }
    }
}

impl Clone for Archive {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for Archive {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for Archive {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for Archive {}
unsafe impl Sync for Archive {}

impl std::fmt::Debug for Archive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Archive")
            .field("label", &self.label())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archive_size() {
        assert_eq!(
            std::mem::size_of::<Archive>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
