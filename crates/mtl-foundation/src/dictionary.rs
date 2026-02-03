//! Dictionary type for Foundation.
//!
//! Corresponds to `Foundation/NSDictionary.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! class Dictionary : public NS::Copying<Dictionary> {
//! public:
//!     static Dictionary* dictionary();
//!     static Dictionary* dictionary(const Object* pObject, const Object* pKey);
//!     static Dictionary* dictionary(const Object* const* pObjects, const Object* const* pKeys, UInteger count);
//!     static Dictionary* alloc();
//!     Dictionary*        init();
//!     Dictionary*        init(const Object* const* pObjects, const Object* const* pKeys, UInteger count);
//!     Dictionary*        init(const class Coder* pCoder);
//!     template <class _KeyType = Object>
//!     Enumerator<_KeyType>* keyEnumerator() const;
//!     template <class _Object = Object>
//!     _Object* object(const Object* pKey) const;
//!     UInteger count() const;
//! };
//! }
//! ```

use std::ffi::c_void;
use std::marker::PhantomData;
use std::ptr::NonNull;

use mtl_sys::{class, msg_send_0, msg_send_1, msg_send_2, msg_send_3, sel};

use crate::enumerator::Enumerator;
use crate::object::{Copying, Object, Referencing};
use crate::types::UInteger;

/// An Objective-C dictionary object.
///
/// C++ equivalent: `NS::Dictionary`
#[repr(transparent)]
pub struct Dictionary<K = Object, V = Object> {
    inner: NonNull<c_void>,
    _key_marker: PhantomData<K>,
    _value_marker: PhantomData<V>,
}

impl<K, V> Clone for Dictionary<K, V> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            _key_marker: PhantomData,
            _value_marker: PhantomData,
        }
    }
}

impl<K, V> Dictionary<K, V> {
    /// Create an empty dictionary.
    ///
    /// C++ equivalent: `static Dictionary* dictionary()`
    #[inline]
    pub fn dictionary() -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(class!(NSDictionary).as_ptr(), sel!(dictionary));
            Self::from_ptr(ptr)
        }
    }

    /// Create a dictionary with a single object and key.
    ///
    /// C++ equivalent: `static Dictionary* dictionary(const Object* pObject, const Object* pKey)`
    #[inline]
    pub fn dictionary_with_object(object: *const V, key: *const K) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_2(
                class!(NSDictionary).as_ptr(),
                sel!(dictionaryWithObject:forKey:),
                object,
                key,
            );
            Self::from_ptr(ptr)
        }
    }

    /// Create a dictionary with multiple objects and keys.
    ///
    /// C++ equivalent: `static Dictionary* dictionary(const Object* const* pObjects, const Object* const* pKeys, UInteger count)`
    #[inline]
    pub fn dictionary_with_objects(
        objects: *const *const V,
        keys: *const *const K,
        count: UInteger,
    ) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_3(
                class!(NSDictionary).as_ptr(),
                sel!(dictionaryWithObjects:forKeys:count:),
                objects,
                keys,
                count,
            );
            Self::from_ptr(ptr)
        }
    }

    /// Allocate a new dictionary.
    ///
    /// C++ equivalent: `static Dictionary* alloc()`
    #[inline]
    pub fn alloc() -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(class!(NSDictionary).as_ptr(), sel!(alloc));
            Self::from_ptr(ptr)
        }
    }

    /// Initialize an allocated dictionary.
    ///
    /// C++ equivalent: `Dictionary* init()`
    #[inline]
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with multiple objects and keys.
    ///
    /// C++ equivalent: `Dictionary* init(const Object* const* pObjects, const Object* const* pKeys, UInteger count)`
    #[inline]
    pub fn init_with_objects(
        &self,
        objects: *const *const V,
        keys: *const *const K,
        count: UInteger,
    ) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(initWithObjects:forKeys:count:),
                objects,
                keys,
                count,
            );
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with a coder.
    ///
    /// C++ equivalent: `Dictionary* init(const class Coder* pCoder)`
    #[inline]
    pub fn init_with_coder(&self, coder: *const c_void) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithCoder:), coder);
            Self::from_ptr(ptr)
        }
    }

    /// Get a key enumerator for the dictionary.
    ///
    /// C++ equivalent: `template <class _KeyType = Object> Enumerator<_KeyType>* keyEnumerator() const`
    #[inline]
    pub fn key_enumerator(&self) -> Option<Enumerator<K>> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(keyEnumerator));
            Enumerator::from_ptr(ptr)
        }
    }

    /// Get the object for the specified key.
    ///
    /// C++ equivalent: `template <class _Object = Object> _Object* object(const Object* pKey) const`
    #[inline]
    pub fn object(&self, key: *const K) -> *mut V {
        unsafe { msg_send_1(self.as_ptr(), sel!(objectForKey:), key) }
    }

    /// Get the count of key-value pairs in the dictionary.
    ///
    /// C++ equivalent: `UInteger count() const`
    #[inline]
    pub fn count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(count)) }
    }

    /// Create a Dictionary from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C NSDictionary object.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(|inner| Self {
            inner,
            _key_marker: PhantomData,
            _value_marker: PhantomData,
        })
    }
}

impl<K, V> Referencing for Dictionary<K, V> {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.inner.as_ptr()
    }
}

impl<K, V> Copying for Dictionary<K, V> {
    #[inline]
    fn copy(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_ptr(ptr)
        }
    }
}

// SAFETY: NSDictionary is thread-safe for reference counting
unsafe impl<K: Send, V: Send> Send for Dictionary<K, V> {}
unsafe impl<K: Sync, V: Sync> Sync for Dictionary<K, V> {}

impl<K, V> std::fmt::Debug for Dictionary<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dictionary")
            .field("ptr", &self.inner)
            .field("count", &self.count())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_size() {
        // Dictionary should be pointer-sized
        assert_eq!(
            std::mem::size_of::<Dictionary<Object, Object>>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
