//! Data type for Foundation.
//!
//! Corresponds to `Foundation/NSData.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! class Data : public Copying<Data> {
//! public:
//!     void*    mutableBytes() const;
//!     UInteger length() const;
//! };
//! }
//! ```

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_sys::{msg_send_0, sel};

use crate::object::{Copying, Referencing};
use crate::types::UInteger;

/// An Objective-C data object.
///
/// C++ equivalent: `NS::Data`
#[repr(transparent)]
#[derive(Clone)]
pub struct Data(NonNull<c_void>);

impl Data {
    /// Get a mutable pointer to the data's bytes.
    ///
    /// C++ equivalent: `void* mutableBytes() const`
    #[inline]
    pub fn mutable_bytes(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(mutableBytes)) }
    }

    /// Get the length of the data in bytes.
    ///
    /// C++ equivalent: `UInteger length() const`
    #[inline]
    pub fn length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(length)) }
    }

    /// Create a Data from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C NSData object.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }
}

impl Referencing for Data {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

impl Copying for Data {
    #[inline]
    fn copy(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_ptr(ptr)
        }
    }
}

// SAFETY: NSData is thread-safe for reference counting
unsafe impl Send for Data {}
unsafe impl Sync for Data {}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Data")
            .field("ptr", &self.0)
            .field("length", &self.length())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_size() {
        // Data should be pointer-sized
        assert_eq!(
            std::mem::size_of::<Data>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
