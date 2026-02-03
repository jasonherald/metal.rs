//! MTL4 BinaryFunction implementation.
//!
//! Corresponds to `Metal/MTL4BinaryFunction.hpp` and `Metal/MTL4BinaryFunctionDescriptor.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::FunctionType;
use super::enums::BinaryFunctionOptions;
use super::FunctionDescriptor;

// ============================================================
// BinaryFunction
// ============================================================

/// A compiled binary function for linking.
///
/// C++ equivalent: `MTL4::BinaryFunction`
///
/// BinaryFunction represents a precompiled function that can be
/// linked with pipelines at runtime.
#[repr(transparent)]
pub struct BinaryFunction(NonNull<c_void>);

impl BinaryFunction {
    /// Create a BinaryFunction from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the function type.
    ///
    /// C++ equivalent: `MTL::FunctionType functionType() const`
    pub fn function_type(&self) -> FunctionType {
        unsafe { msg_send_0(self.as_ptr(), sel!(functionType)) }
    }

    /// Get the function name.
    ///
    /// C++ equivalent: `NS::String* name() const`
    pub fn name(&self) -> Option<String> {
        unsafe {
            let ns_string: *mut c_void = msg_send_0(self.as_ptr(), sel!(name));
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
}

impl Clone for BinaryFunction {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for BinaryFunction {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for BinaryFunction {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for BinaryFunction {}
unsafe impl Sync for BinaryFunction {}

impl std::fmt::Debug for BinaryFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryFunction")
            .field("name", &self.name())
            .field("function_type", &self.function_type())
            .finish()
    }
}

// ============================================================
// BinaryFunctionDescriptor
// ============================================================

/// Descriptor for creating a binary function.
///
/// C++ equivalent: `MTL4::BinaryFunctionDescriptor`
#[repr(transparent)]
pub struct BinaryFunctionDescriptor(NonNull<c_void>);

impl BinaryFunctionDescriptor {
    /// Create a BinaryFunctionDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new binary function descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4BinaryFunctionDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the name.
    ///
    /// C++ equivalent: `NS::String* name() const`
    pub fn name(&self) -> Option<String> {
        unsafe {
            let ns_string: *mut c_void = msg_send_0(self.as_ptr(), sel!(name));
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

    /// Set the name.
    ///
    /// C++ equivalent: `void setName(const NS::String*)`
    pub fn set_name(&self, name: &str) {
        if let Some(ns_name) = metal_foundation::String::from_str(name) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setName:), ns_name.as_ptr());
            }
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

    /// Get the options.
    ///
    /// C++ equivalent: `BinaryFunctionOptions options() const`
    pub fn options(&self) -> BinaryFunctionOptions {
        unsafe { msg_send_0(self.as_ptr(), sel!(options)) }
    }

    /// Set the options.
    ///
    /// C++ equivalent: `void setOptions(MTL4::BinaryFunctionOptions)`
    pub fn set_options(&self, options: BinaryFunctionOptions) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOptions:), options);
        }
    }
}

impl Clone for BinaryFunctionDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for BinaryFunctionDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for BinaryFunctionDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for BinaryFunctionDescriptor {}
unsafe impl Sync for BinaryFunctionDescriptor {}

impl std::fmt::Debug for BinaryFunctionDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryFunctionDescriptor")
            .field("name", &self.name())
            .field("options", &self.options())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_function_size() {
        assert_eq!(
            std::mem::size_of::<BinaryFunction>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_binary_function_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<BinaryFunctionDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
