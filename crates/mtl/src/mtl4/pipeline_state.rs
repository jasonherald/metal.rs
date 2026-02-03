//! MTL4 PipelineState implementation.
//!
//! Corresponds to `Metal/MTL4PipelineState.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_0, msg_send_1, sel};

use super::enums::ShaderReflection;
use crate::ShaderValidation;

// ============================================================
// PipelineOptions
// ============================================================

/// Options for MTL4 pipeline creation.
///
/// C++ equivalent: `MTL4::PipelineOptions`
///
/// PipelineOptions controls shader reflection and validation
/// settings for pipeline compilation.
#[repr(transparent)]
pub struct PipelineOptions(NonNull<c_void>);

impl PipelineOptions {
    /// Create a PipelineOptions from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create new pipeline options.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTL4PipelineOptions")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the shader reflection options.
    ///
    /// C++ equivalent: `ShaderReflection shaderReflection() const`
    pub fn shader_reflection(&self) -> ShaderReflection {
        unsafe { msg_send_0(self.as_ptr(), sel!(shaderReflection)) }
    }

    /// Set the shader reflection options.
    ///
    /// C++ equivalent: `void setShaderReflection(MTL4::ShaderReflection)`
    pub fn set_shader_reflection(&self, reflection: ShaderReflection) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setShaderReflection:), reflection);
        }
    }

    /// Get the shader validation setting.
    ///
    /// C++ equivalent: `MTL::ShaderValidation shaderValidation() const`
    pub fn shader_validation(&self) -> ShaderValidation {
        unsafe { msg_send_0(self.as_ptr(), sel!(shaderValidation)) }
    }

    /// Set the shader validation setting.
    ///
    /// C++ equivalent: `void setShaderValidation(MTL::ShaderValidation)`
    pub fn set_shader_validation(&self, validation: ShaderValidation) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setShaderValidation:), validation);
        }
    }
}

impl Clone for PipelineOptions {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for PipelineOptions {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for PipelineOptions {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for PipelineOptions {}
unsafe impl Sync for PipelineOptions {}

impl std::fmt::Debug for PipelineOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipelineOptions")
            .field("shader_reflection", &self.shader_reflection())
            .finish()
    }
}

// ============================================================
// PipelineDescriptor
// ============================================================

/// Base descriptor for MTL4 pipelines.
///
/// C++ equivalent: `MTL4::PipelineDescriptor`
///
/// PipelineDescriptor is the base class for all MTL4 pipeline descriptors.
#[repr(transparent)]
pub struct PipelineDescriptor(NonNull<c_void>);

impl PipelineDescriptor {
    /// Create a PipelineDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new pipeline descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTL4PipelineDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
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
}

impl Clone for PipelineDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for PipelineDescriptor {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for PipelineDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for PipelineDescriptor {}
unsafe impl Sync for PipelineDescriptor {}

impl std::fmt::Debug for PipelineDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipelineDescriptor")
            .field("label", &self.label())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_options_size() {
        assert_eq!(
            std::mem::size_of::<PipelineOptions>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_pipeline_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<PipelineDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
