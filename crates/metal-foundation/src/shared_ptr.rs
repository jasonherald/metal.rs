//! Smart pointer for Objective-C objects.
//!
//! Corresponds to `Foundation/NSSharedPtr.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! template <class _Class>
//! class SharedPtr {
//! public:
//!     SharedPtr();
//!     ~SharedPtr();
//!     SharedPtr(std::nullptr_t) noexcept;
//!     SharedPtr(const SharedPtr<_Class>& other) noexcept;
//!     SharedPtr(SharedPtr<_Class>&& other) noexcept;
//!     SharedPtr& operator=(const SharedPtr<_Class>& other);
//!     SharedPtr& operator=(SharedPtr<_Class>&& other);
//!     _Class* get() const;
//!     _Class* operator->() const;
//!     explicit operator bool() const;
//!     void reset();
//!     void detach();
//! private:
//!     _Class* m_pObject;
//! };
//!
//! template <class _Class>
//! SharedPtr<_Class> RetainPtr(_Class* pObject);
//!
//! template <class _Class>
//! SharedPtr<_Class> TransferPtr(_Class* pObject);
//! }
//! ```

use std::ffi::c_void;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

use metal_sys::{msg_send_0, sel};

/// A smart pointer that automatically manages the reference count of an Objective-C object.
///
/// C++ equivalent: `NS::SharedPtr<_Class>`
///
/// # Ownership
///
/// `SharedPtr` owns a reference to the underlying Objective-C object. When the `SharedPtr`
/// is dropped, it will release the reference. When cloned, it will retain a new reference.
pub struct SharedPtr<T> {
    /// The raw pointer to the Objective-C object.
    ptr: NonNull<c_void>,
    /// Phantom data to track the type.
    _marker: PhantomData<T>,
}

impl<T> SharedPtr<T> {
    /// Create a null SharedPtr.
    ///
    /// Note: Unlike C++, Rust's NonNull cannot be null, so this returns an Option.
    /// For null representation, use `Option<SharedPtr<T>>`.
    ///
    /// C++ equivalent: `SharedPtr()`
    #[inline]
    pub fn null() -> Option<Self> {
        None
    }

    /// Create a SharedPtr from a raw pointer, taking ownership.
    ///
    /// # Safety
    ///
    /// - The pointer must be a valid Objective-C object.
    /// - The caller must ensure the object has a retain count of at least 1.
    /// - Ownership is transferred to the SharedPtr (no additional retain).
    ///
    /// C++ equivalent: Used internally by `TransferPtr`
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self {
            ptr,
            _marker: PhantomData,
        })
    }

    /// Get the raw pointer to the Objective-C object.
    ///
    /// C++ equivalent: `_Class* get() const`
    #[inline]
    pub fn get(&self) -> *mut c_void {
        self.ptr.as_ptr()
    }

    /// Get the raw pointer (const version for API compatibility).
    #[inline]
    pub fn as_ptr(&self) -> *const c_void {
        self.ptr.as_ptr()
    }

    /// Reset this SharedPtr to null, releasing the reference.
    ///
    /// C++ equivalent: `void reset()`
    #[inline]
    pub fn reset(&mut self) {
        unsafe {
            msg_send_0::<()>(self.ptr.as_ptr(), sel!(release));
        }
        // Note: In Rust, we can't actually make NonNull null,
        // so after reset, the SharedPtr should not be used.
        // The caller should replace it or drop it.
    }

    /// Detach the SharedPtr from the pointee without releasing.
    ///
    /// Returns the raw pointer and prevents the destructor from releasing.
    ///
    /// C++ equivalent: `void detach()`
    #[inline]
    pub fn detach(self) -> *mut c_void {
        let ptr = self.ptr.as_ptr();
        std::mem::forget(self);
        ptr
    }

    /// Convert to a pointer to a different type.
    ///
    /// # Safety
    ///
    /// The types must be compatible.
    #[inline]
    pub unsafe fn cast<U>(self) -> SharedPtr<U> {
        let ptr = self.ptr;
        std::mem::forget(self);
        SharedPtr {
            ptr,
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for SharedPtr<T> {
    /// Clone this SharedPtr, retaining the object.
    ///
    /// C++ equivalent: `SharedPtr(const SharedPtr<_Class>& other)`
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let _: *mut c_void = msg_send_0(self.ptr.as_ptr(), sel!(retain));
        }
        Self {
            ptr: self.ptr,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for SharedPtr<T> {
    /// Release the reference when dropped.
    ///
    /// C++ equivalent: `~SharedPtr()`
    #[inline]
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.ptr.as_ptr(), sel!(release));
        }
    }
}

impl<T> Deref for SharedPtr<T> {
    type Target = *mut c_void;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // Return a reference to the pointer itself
        // This is a bit awkward, but matches the C++ operator-> semantics
        unsafe { &*((&self.ptr) as *const NonNull<c_void> as *const *mut c_void) }
    }
}

impl<T> PartialEq for SharedPtr<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl<T> Eq for SharedPtr<T> {}

impl<T> std::fmt::Debug for SharedPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedPtr").field("ptr", &self.ptr).finish()
    }
}

// SAFETY: Objective-C objects with proper reference counting are thread-safe
unsafe impl<T: Send> Send for SharedPtr<T> {}
unsafe impl<T: Sync> Sync for SharedPtr<T> {}

/// Create a SharedPtr by retaining an existing raw pointer.
///
/// Increases the reference count of the passed-in object.
///
/// C++ equivalent: `NS::SharedPtr<_Class> RetainPtr(_Class* pObject)`
///
/// # Safety
///
/// The pointer must be a valid Objective-C object with a retain count >= 1.
#[inline]
pub unsafe fn retain_ptr<T>(ptr: *mut c_void) -> Option<SharedPtr<T>> {
    if ptr.is_null() {
        return None;
    }

    // Retain the object
    let _: *mut c_void = unsafe { msg_send_0(ptr, sel!(retain)) };

    unsafe { SharedPtr::from_raw(ptr) }
}

/// Create a SharedPtr by transferring ownership of an existing raw pointer.
///
/// Does not increase the reference count - ownership is transferred to SharedPtr.
///
/// C++ equivalent: `NS::SharedPtr<_Class> TransferPtr(_Class* pObject)`
///
/// # Safety
///
/// - The pointer must be a valid Objective-C object.
/// - The object must have a retain count >= 1.
/// - The caller transfers ownership and must not release the object.
#[inline]
pub unsafe fn transfer_ptr<T>(ptr: *mut c_void) -> Option<SharedPtr<T>> {
    unsafe { SharedPtr::from_raw(ptr) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_ptr_size() {
        // SharedPtr should be pointer-sized (no vtable, just NonNull + PhantomData)
        assert_eq!(
            std::mem::size_of::<SharedPtr<()>>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
