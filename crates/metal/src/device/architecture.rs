//! GPU architecture information.
//!
//! Corresponds to `MTL::Architecture` in `Metal/MTLDevice.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, Copying};
use metal_sys::{msg_send_0, sel, class};

/// GPU architecture information.
///
/// C++ equivalent: `MTL::Architecture`
#[repr(transparent)]
pub struct Architecture(NonNull<c_void>);

impl Architecture {
    /// Create an Architecture from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid MTLArchitecture object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Allocate a new Architecture instance.
    ///
    /// C++ equivalent: `static Architecture* alloc()`
    #[inline]
    pub fn alloc() -> Option<Self> {
        unsafe {
            let cls = class!(MTLArchitecture);
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize an allocated Architecture.
    ///
    /// C++ equivalent: `Architecture* init()`
    #[inline]
    pub fn init(self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            // Consume self without releasing
            std::mem::forget(self);
            Self::from_raw(ptr)
        }
    }

    /// Get the architecture name.
    ///
    /// C++ equivalent: `NS::String* name() const`
    #[inline]
    pub fn name(&self) -> &str {
        unsafe {
            let ns_string: *mut c_void = msg_send_0(self.as_ptr(), sel!(name));
            if ns_string.is_null() {
                return "";
            }
            // Get UTF-8 bytes from NSString
            let c_str: *const std::ffi::c_char = msg_send_0(ns_string, sel!(UTF8String));
            if c_str.is_null() {
                return "";
            }
            std::ffi::CStr::from_ptr(c_str)
                .to_str()
                .unwrap_or("")
        }
    }
}

impl Clone for Architecture {
    fn clone(&self) -> Self {
        self.retain();
        Self(self.0)
    }
}

impl Referencing for Architecture {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

impl Copying for Architecture {
    fn copy(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr)
        }
    }
}

impl Drop for Architecture {
    fn drop(&mut self) {
        self.release();
    }
}

// SAFETY: Architecture is an immutable value object
unsafe impl Send for Architecture {}
unsafe impl Sync for Architecture {}

impl std::fmt::Debug for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Architecture")
            .field("name", &self.name())
            .finish()
    }
}

impl std::fmt::Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_size() {
        assert_eq!(
            std::mem::size_of::<Architecture>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
