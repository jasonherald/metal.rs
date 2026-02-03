//! Set type for Foundation.
//!
//! Corresponds to `Foundation/NSSet.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! class Set : public NS::Copying<Set> {
//! public:
//!     UInteger count() const;
//!     Enumerator<Object>* objectEnumerator() const;
//!     static Set* alloc();
//!     Set* init();
//!     Set* init(const Object* const* pObjects, UInteger count);
//!     Set* init(const class Coder* pCoder);
//! };
//! }
//! ```

use std::ffi::c_void;
use std::marker::PhantomData;
use std::ptr::NonNull;

use metal_sys::{class, msg_send_0, msg_send_1, msg_send_2, sel};

use crate::enumerator::Enumerator;
use crate::object::{Copying, Object, Referencing};
use crate::types::UInteger;

/// An Objective-C set object.
///
/// C++ equivalent: `NS::Set`
#[repr(transparent)]
pub struct Set<T = Object> {
    inner: NonNull<c_void>,
    _marker: PhantomData<T>,
}

impl<T> Clone for Set<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Set<T> {
    /// Get the count of objects in the set.
    ///
    /// C++ equivalent: `UInteger count() const`
    #[inline]
    pub fn count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(count)) }
    }

    /// Get an enumerator for the objects in the set.
    ///
    /// C++ equivalent: `Enumerator<Object>* objectEnumerator() const`
    #[inline]
    pub fn object_enumerator(&self) -> Option<Enumerator<T>> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(objectEnumerator));
            Enumerator::from_ptr(ptr)
        }
    }

    /// Allocate a new set.
    ///
    /// C++ equivalent: `static Set* alloc()`
    #[inline]
    pub fn alloc() -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(class!(NSSet).as_ptr(), sel!(alloc));
            Self::from_ptr(ptr)
        }
    }

    /// Initialize an allocated set.
    ///
    /// C++ equivalent: `Set* init()`
    #[inline]
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with multiple objects.
    ///
    /// C++ equivalent: `Set* init(const Object* const* pObjects, UInteger count)`
    #[inline]
    pub fn init_with_objects(&self, objects: *const *const T, count: UInteger) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_2(self.as_ptr(), sel!(initWithObjects:count:), objects, count);
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with a coder.
    ///
    /// C++ equivalent: `Set* init(const class Coder* pCoder)`
    #[inline]
    pub fn init_with_coder(&self, coder: *const c_void) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithCoder:), coder);
            Self::from_ptr(ptr)
        }
    }

    /// Create a Set from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C NSSet object.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(|inner| Self {
            inner,
            _marker: PhantomData,
        })
    }
}

impl<T> Referencing for Set<T> {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.inner.as_ptr()
    }
}

impl<T> Copying for Set<T> {
    #[inline]
    fn copy(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_ptr(ptr)
        }
    }
}

unsafe impl<T: Send> Send for Set<T> {}
unsafe impl<T: Sync> Sync for Set<T> {}

impl<T> std::fmt::Debug for Set<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Set")
            .field("ptr", &self.inner)
            .field("count", &self.count())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_size() {
        assert_eq!(
            std::mem::size_of::<Set<Object>>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
