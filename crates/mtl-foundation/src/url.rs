//! URL type for Foundation.
//!
//! Corresponds to `Foundation/NSURL.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! class URL : public Copying<URL> {
//! public:
//!     static URL* fileURLWithPath(const class String* pPath);
//!     static URL* alloc();
//!     URL*        init();
//!     URL*        init(const class String* pString);
//!     URL*        initFileURLWithPath(const class String* pPath);
//!     const char* fileSystemRepresentation() const;
//! };
//! }
//! ```

use std::ffi::{c_char, c_void};
use std::ptr::NonNull;

use mtl_sys::{class, msg_send_0, msg_send_1, sel};

use crate::object::{Copying, Referencing};
use crate::string::String;

/// An Objective-C URL object.
///
/// C++ equivalent: `NS::URL`
#[repr(transparent)]
#[derive(Clone)]
pub struct Url(NonNull<c_void>);

impl Url {
    /// Create a file URL with the specified path.
    ///
    /// C++ equivalent: `static URL* fileURLWithPath(const class String* pPath)`
    #[inline]
    pub fn file_url_with_path(path: &String) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                class!(NSURL).as_ptr(),
                sel!(fileURLWithPath:),
                path.as_ptr(),
            );
            Self::from_ptr(ptr)
        }
    }

    /// Allocate a new URL.
    ///
    /// C++ equivalent: `static URL* alloc()`
    #[inline]
    pub fn alloc() -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(class!(NSURL).as_ptr(), sel!(alloc));
            Self::from_ptr(ptr)
        }
    }

    /// Initialize an allocated URL.
    ///
    /// C++ equivalent: `URL* init()`
    #[inline]
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with a string.
    ///
    /// C++ equivalent: `URL* init(const class String* pString)`
    #[inline]
    pub fn init_with_string(&self, string: &String) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(initWithString:), string.as_ptr());
            Self::from_ptr(ptr)
        }
    }

    /// Initialize as a file URL with the specified path.
    ///
    /// C++ equivalent: `URL* initFileURLWithPath(const class String* pPath)`
    #[inline]
    pub fn init_file_url_with_path(&self, path: &String) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(initFileURLWithPath:), path.as_ptr());
            Self::from_ptr(ptr)
        }
    }

    /// Get the file system representation.
    ///
    /// C++ equivalent: `const char* fileSystemRepresentation() const`
    #[inline]
    pub fn file_system_representation(&self) -> *const c_char {
        unsafe { msg_send_0(self.as_ptr(), sel!(fileSystemRepresentation)) }
    }

    /// Create a URL from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C NSURL object.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    // Convenience methods for Rust interop

    /// Create a file URL from a Rust path.
    ///
    /// This is a convenience method for Rust users.
    #[inline]
    pub fn from_path(path: &std::path::Path) -> Option<Self> {
        let path_str = path.to_str()?;
        let ns_string = String::from_str(path_str)?;
        Self::file_url_with_path(&ns_string)
    }
}

impl Referencing for Url {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

impl Copying for Url {
    #[inline]
    fn copy(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_ptr(ptr)
        }
    }
}

// SAFETY: NSURL is thread-safe for reference counting
unsafe impl Send for Url {}
unsafe impl Sync for Url {}

impl std::fmt::Debug for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = self.file_system_representation();
        if repr.is_null() {
            f.debug_struct("Url").field("ptr", &self.0).finish()
        } else {
            let s = unsafe { std::ffi::CStr::from_ptr(repr) };
            f.debug_struct("Url")
                .field("path", &s.to_string_lossy())
                .finish()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_size() {
        // URL should be pointer-sized
        assert_eq!(
            std::mem::size_of::<Url>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
