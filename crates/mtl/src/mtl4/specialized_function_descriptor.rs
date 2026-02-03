//! MTL4 SpecializedFunctionDescriptor implementation.
//!
//! Corresponds to `Metal/MTL4SpecializedFunctionDescriptor.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_0, msg_send_1, sel};

use super::FunctionDescriptor;

// ============================================================
// SpecializedFunctionDescriptor
// ============================================================

/// Descriptor for specialized functions with constant values.
///
/// C++ equivalent: `MTL4::SpecializedFunctionDescriptor`
///
/// SpecializedFunctionDescriptor extends FunctionDescriptor to support
/// function specialization with constant values.
#[repr(transparent)]
pub struct SpecializedFunctionDescriptor(NonNull<c_void>);

impl SpecializedFunctionDescriptor {
    /// Create a SpecializedFunctionDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new specialized function descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTL4SpecializedFunctionDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the function descriptor.
    ///
    /// C++ equivalent: `FunctionDescriptor* functionDescriptor() const`
    pub fn function_descriptor(&self) -> Option<FunctionDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(functionDescriptor));
            FunctionDescriptor::from_raw(ptr)
        }
    }

    /// Set the function descriptor.
    ///
    /// C++ equivalent: `void setFunctionDescriptor(const MTL4::FunctionDescriptor*)`
    pub fn set_function_descriptor(&self, descriptor: &FunctionDescriptor) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setFunctionDescriptor:),
                descriptor.as_ptr(),
            );
        }
    }

    /// Get the constant values (as raw pointer).
    ///
    /// C++ equivalent: `MTL::FunctionConstantValues* constantValues() const`
    pub fn constant_values_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(constantValues)) }
    }

    /// Set the constant values (from raw pointer).
    ///
    /// C++ equivalent: `void setConstantValues(const MTL::FunctionConstantValues*)`
    pub fn set_constant_values_raw(&self, values: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setConstantValues:), values);
        }
    }

    /// Get the specialized name.
    ///
    /// C++ equivalent: `NS::String* specializedName() const`
    pub fn specialized_name(&self) -> Option<String> {
        unsafe {
            let ns_string: *mut c_void = msg_send_0(self.as_ptr(), sel!(specializedName));
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

    /// Set the specialized name.
    ///
    /// C++ equivalent: `void setSpecializedName(const NS::String*)`
    pub fn set_specialized_name(&self, name: &str) {
        if let Some(ns_name) = mtl_foundation::String::from_str(name) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setSpecializedName:), ns_name.as_ptr());
            }
        }
    }
}

impl Default for SpecializedFunctionDescriptor {
    fn default() -> Self {
        Self::new().expect("Failed to create MTL4SpecializedFunctionDescriptor")
    }
}

impl Clone for SpecializedFunctionDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for SpecializedFunctionDescriptor {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for SpecializedFunctionDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for SpecializedFunctionDescriptor {}
unsafe impl Sync for SpecializedFunctionDescriptor {}

impl std::fmt::Debug for SpecializedFunctionDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpecializedFunctionDescriptor")
            .field("specialized_name", &self.specialized_name())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specialized_function_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<SpecializedFunctionDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
