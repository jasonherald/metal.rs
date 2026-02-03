//! Base object traits for Foundation.
//!
//! Corresponds to `Foundation/NSObject.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! template <class _Class, class _Base = class Object>
//! class Referencing : public _Base {
//! public:
//!     _Class*  retain();
//!     void     release();
//!     _Class*  autorelease();
//!     UInteger retainCount() const;
//! };
//!
//! template <class _Class, class _Base = class Object>
//! class Copying : public Referencing<_Class, _Base> {
//! public:
//!     _Class* copy() const;
//! };
//!
//! template <class _Class, class _Base = class Object>
//! class SecureCoding : public Referencing<_Class, _Base> {};
//!
//! class Object : public Referencing<Object, objc_object> {
//! public:
//!     UInteger      hash() const;
//!     bool          isEqual(const Object* pObject) const;
//!     class String* description() const;
//!     class String* debugDescription() const;
//! };
//! }
//! ```

use std::ffi::c_void;
use std::ptr::NonNull;

use crate::types::UInteger;
use metal_sys::{Class, Sel, msg_send_0, msg_send_1, sel};

/// Base trait for all Objective-C objects that support reference counting.
///
/// C++ equivalent: `NS::Referencing<_Class, _Base>`
pub trait Referencing: Sized {
    /// Get the raw pointer to the Objective-C object.
    fn as_ptr(&self) -> *const c_void;

    /// Get the raw mutable pointer to the Objective-C object.
    fn as_mut_ptr(&self) -> *mut c_void {
        self.as_ptr() as *mut c_void
    }

    /// Retain the object, incrementing its reference count.
    ///
    /// C++ equivalent: `_Class* retain()`
    fn retain(&self) -> Self
    where
        Self: Clone,
    {
        unsafe {
            let _: *mut c_void = msg_send_0(self.as_ptr(), sel!(retain));
        }
        self.clone()
    }

    /// Release the object, decrementing its reference count.
    ///
    /// C++ equivalent: `void release()`
    fn release(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }

    /// Autorelease the object.
    ///
    /// C++ equivalent: `_Class* autorelease()`
    fn autorelease(&self) -> Self
    where
        Self: Clone,
    {
        unsafe {
            let _: *mut c_void = msg_send_0(self.as_ptr(), sel!(autorelease));
        }
        self.clone()
    }

    /// Get the retain count of the object.
    ///
    /// C++ equivalent: `UInteger retainCount() const`
    fn retain_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(retainCount)) }
    }
}

/// Trait for objects that support copying.
///
/// C++ equivalent: `NS::Copying<_Class, _Base>`
pub trait Copying: Referencing {
    /// Create a copy of this object.
    ///
    /// C++ equivalent: `_Class* copy() const`
    fn copy(&self) -> Option<Self>
    where
        Self: Sized;
}

/// Trait for objects that support secure coding.
///
/// C++ equivalent: `NS::SecureCoding<_Class, _Base>`
pub trait SecureCoding: Referencing {}

/// The root class of most Objective-C class hierarchies.
///
/// C++ equivalent: `NS::Object`
#[repr(transparent)]
#[derive(Clone)]
pub struct Object(NonNull<c_void>);

impl Object {
    /// Create an Object from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C object.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the hash value of this object.
    ///
    /// C++ equivalent: `UInteger hash() const`
    #[inline]
    pub fn hash(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(hash)) }
    }

    /// Check if this object is equal to another.
    ///
    /// C++ equivalent: `bool isEqual(const Object* pObject) const`
    #[inline]
    pub fn is_equal(&self, other: &Object) -> bool {
        unsafe { msg_send_1(self.as_ptr(), sel!(isEqual:), other.as_ptr()) }
    }

    /// Get a string description of this object.
    ///
    /// C++ equivalent: `class String* description() const`
    #[inline]
    pub fn description(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(description)) }
    }

    /// Get a debug description of this object.
    ///
    /// C++ equivalent: `class String* debugDescription() const`
    #[inline]
    pub fn debug_description(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(debugDescription)) }
    }

    /// Allocate an instance of a class.
    ///
    /// C++ equivalent: `template <class _Class> static _Class* alloc(const void* pClass)`
    #[inline]
    pub fn alloc_with_class(cls: Class) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(alloc));
            Self::from_ptr(ptr)
        }
    }

    /// Initialize an allocated object.
    ///
    /// C++ equivalent: `template <class _Class> _Class* init()`
    #[inline]
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_ptr(ptr)
        }
    }

    /// Check if this object responds to a selector.
    #[inline]
    pub fn responds_to_selector(&self, selector: Sel) -> bool {
        unsafe { msg_send_1(self.as_ptr(), sel!(respondsToSelector:), selector) }
    }
}

impl Referencing for Object {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

// SAFETY: Objective-C objects are thread-safe for reference counting
unsafe impl Send for Object {}
unsafe impl Sync for Object {}

/// Helper function to send a message with the given selector.
///
/// This is the Rust equivalent of C++'s `Object::sendMessage<>`.
///
/// # Safety
///
/// - `obj` must be a valid Objective-C object pointer
/// - `sel` must be a valid selector for the object
/// - The return type must match the actual return type of the method
#[inline]
pub unsafe fn send_message<R>(obj: *const c_void, sel: Sel) -> R {
    unsafe { msg_send_0(obj, sel) }
}

/// Helper function to send a message with one argument.
///
/// # Safety
///
/// See `send_message` for safety requirements.
#[inline]
pub unsafe fn send_message_1<R, A>(obj: *const c_void, sel: Sel, a: A) -> R {
    unsafe { msg_send_1(obj, sel, a) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_size() {
        // Object should be pointer-sized
        assert_eq!(
            std::mem::size_of::<Object>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
