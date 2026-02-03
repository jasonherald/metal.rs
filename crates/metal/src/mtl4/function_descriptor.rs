//! MTL4 FunctionDescriptor implementation.
//!
//! Corresponds to `Metal/MTL4FunctionDescriptor.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, sel};

// ============================================================
// FunctionDescriptor
// ============================================================

/// Descriptor for MTL4 functions.
///
/// C++ equivalent: `MTL4::FunctionDescriptor`
///
/// FunctionDescriptor is used to specify a function for pipeline creation
/// in Metal 4.
#[repr(transparent)]
pub struct FunctionDescriptor(NonNull<c_void>);

impl FunctionDescriptor {
    /// Create a FunctionDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new function descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4FunctionDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }
}

impl Clone for FunctionDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for FunctionDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for FunctionDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for FunctionDescriptor {}
unsafe impl Sync for FunctionDescriptor {}

impl std::fmt::Debug for FunctionDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionDescriptor").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<FunctionDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
