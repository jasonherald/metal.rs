//! MTL4 LibraryDescriptor implementation.
//!
//! Corresponds to `Metal/MTL4LibraryDescriptor.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::CompileOptions;

// ============================================================
// LibraryDescriptor
// ============================================================

/// Descriptor for creating a Metal library.
///
/// C++ equivalent: `MTL4::LibraryDescriptor`
///
/// LibraryDescriptor specifies the source code, name, and compile
/// options for creating a Metal shader library.
#[repr(transparent)]
pub struct LibraryDescriptor(NonNull<c_void>);

impl LibraryDescriptor {
    /// Create a LibraryDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new library descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4LibraryDescriptor")?;
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

    /// Get the source code.
    ///
    /// C++ equivalent: `NS::String* source() const`
    pub fn source(&self) -> Option<String> {
        unsafe {
            let ns_string: *mut c_void = msg_send_0(self.as_ptr(), sel!(source));
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

    /// Set the source code.
    ///
    /// C++ equivalent: `void setSource(const NS::String*)`
    pub fn set_source(&self, source: &str) {
        if let Some(ns_source) = metal_foundation::String::from_str(source) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setSource:), ns_source.as_ptr());
            }
        }
    }

    /// Get the compile options.
    ///
    /// C++ equivalent: `MTL::CompileOptions* options() const`
    pub fn options(&self) -> Option<CompileOptions> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(options));
            CompileOptions::from_raw(ptr)
        }
    }

    /// Set the compile options.
    ///
    /// C++ equivalent: `void setOptions(const MTL::CompileOptions*)`
    pub fn set_options(&self, options: &CompileOptions) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOptions:), options.as_ptr());
        }
    }
}

impl Clone for LibraryDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for LibraryDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for LibraryDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for LibraryDescriptor {}
unsafe impl Sync for LibraryDescriptor {}

impl std::fmt::Debug for LibraryDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LibraryDescriptor")
            .field("name", &self.name())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<LibraryDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
