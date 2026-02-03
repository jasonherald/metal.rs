//! Array type for Foundation.
//!
//! Corresponds to `Foundation/NSArray.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! class Array : public Copying<Array> {
//! public:
//!     static Array* array();
//!     static Array* array(const Object* pObject);
//!     static Array* array(const Object* const* pObjects, UInteger count);
//!     static Array* alloc();
//!     Array*        init();
//!     Array*        init(const Object* const* pObjects, UInteger count);
//!     Array*        init(const class Coder* pCoder);
//!     template <class _Object = Object>
//!     _Object*            object(UInteger index) const;
//!     UInteger            count() const;
//!     Enumerator<Object>* objectEnumerator() const;
//! };
//! }
//! ```

use std::ffi::c_void;
use std::marker::PhantomData;
use std::ptr::NonNull;

use mtl_sys::{class, msg_send_0, msg_send_1, msg_send_2, sel};

use crate::enumerator::Enumerator;
use crate::object::{Copying, Object, Referencing};
use crate::types::UInteger;

/// An Objective-C array object.
///
/// C++ equivalent: `NS::Array`
#[repr(transparent)]
pub struct Array<T = Object> {
    inner: NonNull<c_void>,
    _marker: PhantomData<T>,
}

impl<T> Clone for Array<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Array<T> {
    /// Create an empty array.
    ///
    /// C++ equivalent: `static Array* array()`
    #[inline]
    pub fn array() -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(class!(NSArray).as_ptr(), sel!(array));
            Self::from_ptr(ptr)
        }
    }

    /// Create an array with a single object.
    ///
    /// C++ equivalent: `static Array* array(const Object* pObject)`
    #[inline]
    pub fn array_with_object(object: *const T) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(class!(NSArray).as_ptr(), sel!(arrayWithObject:), object);
            Self::from_ptr(ptr)
        }
    }

    /// Create an array with multiple objects.
    ///
    /// C++ equivalent: `static Array* array(const Object* const* pObjects, UInteger count)`
    #[inline]
    pub fn array_with_objects(objects: *const *const T, count: UInteger) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_2(
                class!(NSArray).as_ptr(),
                sel!(arrayWithObjects:count:),
                objects,
                count,
            );
            Self::from_ptr(ptr)
        }
    }

    /// Allocate a new array.
    ///
    /// C++ equivalent: `static Array* alloc()`
    #[inline]
    pub fn alloc() -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(class!(NSArray).as_ptr(), sel!(alloc));
            Self::from_ptr(ptr)
        }
    }

    /// Initialize an allocated array.
    ///
    /// C++ equivalent: `Array* init()`
    #[inline]
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with multiple objects.
    ///
    /// C++ equivalent: `Array* init(const Object* const* pObjects, UInteger count)`
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
    /// C++ equivalent: `Array* init(const class Coder* pCoder)`
    #[inline]
    pub fn init_with_coder(&self, coder: *const c_void) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithCoder:), coder);
            Self::from_ptr(ptr)
        }
    }

    /// Get the object at the specified index.
    ///
    /// C++ equivalent: `template <class _Object = Object> _Object* object(UInteger index) const`
    #[inline]
    pub fn object(&self, index: UInteger) -> *mut T {
        unsafe { msg_send_1(self.as_ptr(), sel!(objectAtIndex:), index) }
    }

    /// Get the count of objects in the array.
    ///
    /// C++ equivalent: `UInteger count() const`
    #[inline]
    pub fn count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(count)) }
    }

    /// Get an enumerator for the objects in the array.
    ///
    /// C++ equivalent: `Enumerator<Object>* objectEnumerator() const`
    #[inline]
    pub fn object_enumerator(&self) -> Option<Enumerator<T>> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(objectEnumerator));
            Enumerator::from_ptr(ptr)
        }
    }

    /// Create an Array from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C NSArray object.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(|inner| Self {
            inner,
            _marker: PhantomData,
        })
    }
}

impl<T> Referencing for Array<T> {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.inner.as_ptr()
    }
}

impl<T> Copying for Array<T> {
    #[inline]
    fn copy(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_ptr(ptr)
        }
    }
}

// SAFETY: NSArray is thread-safe for reference counting
unsafe impl<T: Send> Send for Array<T> {}
unsafe impl<T: Sync> Sync for Array<T> {}

impl<T> std::fmt::Debug for Array<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Array")
            .field("ptr", &self.inner)
            .field("count", &self.count())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_size() {
        // Array should be pointer-sized
        assert_eq!(
            std::mem::size_of::<Array<Object>>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
