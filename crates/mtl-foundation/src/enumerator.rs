//! Enumerator types for Foundation.
//!
//! Corresponds to `Foundation/NSEnumerator.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! struct FastEnumerationState {
//!     unsigned long  state;
//!     Object**       itemsPtr;
//!     unsigned long* mutationsPtr;
//!     unsigned long  extra[5];
//! } _NS_PACKED;
//!
//! class FastEnumeration : public Referencing<FastEnumeration> {
//! public:
//!     NS::UInteger countByEnumerating(FastEnumerationState* pState, Object** pBuffer, NS::UInteger len);
//! };
//!
//! template <class _ObjectType>
//! class Enumerator : public Referencing<Enumerator<_ObjectType>, FastEnumeration> {
//! public:
//!     _ObjectType* nextObject();
//!     class Array* allObjects();
//! };
//! }
//! ```

use std::ffi::c_void;
use std::marker::PhantomData;
use std::ptr::NonNull;

use mtl_sys::{msg_send_0, msg_send_3, sel};

use crate::object::{Object, Referencing};
use crate::types::UInteger;

/// Fast enumeration state structure.
///
/// C++ equivalent: `NS::FastEnumerationState`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct FastEnumerationState {
    /// Enumeration state value.
    pub state: std::ffi::c_ulong,
    /// Pointer to items buffer.
    pub items_ptr: *mut *mut Object,
    /// Pointer to mutations counter.
    pub mutations_ptr: *mut std::ffi::c_ulong,
    /// Extra storage for implementation use.
    pub extra: [std::ffi::c_ulong; 5],
}

impl Default for FastEnumerationState {
    fn default() -> Self {
        Self {
            state: 0,
            items_ptr: std::ptr::null_mut(),
            mutations_ptr: std::ptr::null_mut(),
            extra: [0; 5],
        }
    }
}

/// An object that supports fast enumeration.
///
/// C++ equivalent: `NS::FastEnumeration`
#[repr(transparent)]
#[derive(Clone)]
pub struct FastEnumeration(NonNull<c_void>);

impl FastEnumeration {
    /// Count objects by enumerating.
    ///
    /// C++ equivalent: `NS::UInteger countByEnumerating(FastEnumerationState* pState, Object** pBuffer, NS::UInteger len)`
    #[inline]
    pub fn count_by_enumerating(
        &self,
        state: *mut FastEnumerationState,
        buffer: *mut *mut Object,
        len: UInteger,
    ) -> UInteger {
        unsafe {
            msg_send_3(
                self.as_ptr(),
                sel!(countByEnumeratingWithState:objects:count:),
                state,
                buffer,
                len,
            )
        }
    }

    /// Create a FastEnumeration from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C object that supports fast enumeration.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }
}

impl Referencing for FastEnumeration {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

// SAFETY: Objective-C objects are thread-safe for reference counting
unsafe impl Send for FastEnumeration {}
unsafe impl Sync for FastEnumeration {}

/// An enumerator for a collection.
///
/// C++ equivalent: `NS::Enumerator<_ObjectType>`
#[repr(transparent)]
pub struct Enumerator<T> {
    inner: NonNull<c_void>,
    _marker: PhantomData<T>,
}

impl<T> Clone for Enumerator<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Enumerator<T> {
    /// Get the next object in the enumeration.
    ///
    /// C++ equivalent: `_ObjectType* nextObject()`
    #[inline]
    pub fn next_object(&self) -> *mut T {
        unsafe { msg_send_0(self.as_ptr(), sel!(nextObject)) }
    }

    /// Get all remaining objects as an array.
    ///
    /// C++ equivalent: `class Array* allObjects()`
    #[inline]
    pub fn all_objects(&self) -> *mut c_void {
        // Returns NSArray* but we return raw pointer to avoid circular dep
        unsafe { msg_send_0(self.as_ptr(), sel!(allObjects)) }
    }

    /// Create an Enumerator from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C NSEnumerator object.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(|inner| Self {
            inner,
            _marker: PhantomData,
        })
    }
}

impl<T> Referencing for Enumerator<T> {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.inner.as_ptr()
    }
}

// SAFETY: Objective-C objects are thread-safe for reference counting
unsafe impl<T: Send> Send for Enumerator<T> {}
unsafe impl<T: Sync> Sync for Enumerator<T> {}

impl<T> std::fmt::Debug for Enumerator<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Enumerator")
            .field("ptr", &self.inner)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_enumeration_state_size() {
        // FastEnumerationState should have the expected size
        // state (8) + itemsPtr (8) + mutationsPtr (8) + extra[5] (40) = 64 bytes
        assert_eq!(
            std::mem::size_of::<FastEnumerationState>(),
            std::mem::size_of::<std::ffi::c_ulong>() * 8
        );
    }
}
