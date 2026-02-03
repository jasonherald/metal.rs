//! MTL4 LibraryFunctionDescriptor implementation.
//!
//! Corresponds to `Metal/MTL4LibraryFunctionDescriptor.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::Library;

// ============================================================
// LibraryFunctionDescriptor
// ============================================================

/// Descriptor for a function from a library.
///
/// C++ equivalent: `MTL4::LibraryFunctionDescriptor`
///
/// LibraryFunctionDescriptor extends FunctionDescriptor to reference
/// a specific function from a Metal library by name.
#[repr(transparent)]
pub struct LibraryFunctionDescriptor(NonNull<c_void>);

impl LibraryFunctionDescriptor {
    /// Create a LibraryFunctionDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new library function descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTL4LibraryFunctionDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the library.
    ///
    /// C++ equivalent: `MTL::Library* library() const`
    pub fn library(&self) -> Option<Library> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(library));
            Library::from_raw(ptr)
        }
    }

    /// Set the library.
    ///
    /// C++ equivalent: `void setLibrary(const MTL::Library*)`
    pub fn set_library(&self, library: &Library) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setLibrary:), library.as_ptr());
        }
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

    /// Set the function name.
    ///
    /// C++ equivalent: `void setName(const NS::String*)`
    pub fn set_name(&self, name: &str) {
        if let Some(ns_name) = mtl_foundation::String::from_str(name) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setName:), ns_name.as_ptr());
            }
        }
    }
}

impl Default for LibraryFunctionDescriptor {
    fn default() -> Self {
        Self::new().expect("Failed to create MTL4LibraryFunctionDescriptor")
    }
}

impl Clone for LibraryFunctionDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for LibraryFunctionDescriptor {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for LibraryFunctionDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for LibraryFunctionDescriptor {}
unsafe impl Sync for LibraryFunctionDescriptor {}

impl std::fmt::Debug for LibraryFunctionDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LibraryFunctionDescriptor")
            .field("name", &self.name())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_function_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<LibraryFunctionDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
