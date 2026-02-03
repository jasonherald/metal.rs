//! Locking types for Foundation.
//!
//! Corresponds to `Foundation/NSLock.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! template <class _Class, class _Base = class Object>
//! class Locking : public _Base {
//! public:
//!     void lock();
//!     void unlock();
//! };
//!
//! class Condition : public Locking<Condition> {
//! public:
//!     static Condition* alloc();
//!     Condition*        init();
//!     void              wait();
//!     bool              waitUntilDate(Date* pLimit);
//!     void              signal();
//!     void              broadcast();
//! };
//! }
//! ```

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_sys::{class, msg_send_0, msg_send_1, sel};

use crate::date::Date;
use crate::object::Referencing;

/// Trait for objects that support locking.
///
/// C++ equivalent: `NS::Locking<_Class, _Base>`
pub trait Locking: Referencing {
    /// Lock the object.
    ///
    /// C++ equivalent: `void lock()`
    fn lock(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(lock));
        }
    }

    /// Unlock the object.
    ///
    /// C++ equivalent: `void unlock()`
    fn unlock(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(unlock));
        }
    }
}

/// An Objective-C condition object.
///
/// C++ equivalent: `NS::Condition`
#[repr(transparent)]
#[derive(Clone)]
pub struct Condition(NonNull<c_void>);

impl Condition {
    /// Allocate a new condition.
    ///
    /// C++ equivalent: `static Condition* alloc()`
    #[inline]
    pub fn alloc() -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(class!(NSCondition).as_ptr(), sel!(alloc));
            Self::from_ptr(ptr)
        }
    }

    /// Initialize an allocated condition.
    ///
    /// C++ equivalent: `Condition* init()`
    #[inline]
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_ptr(ptr)
        }
    }

    /// Wait on the condition.
    ///
    /// C++ equivalent: `void wait()`
    #[inline]
    pub fn wait(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(wait));
        }
    }

    /// Wait on the condition until a date.
    ///
    /// C++ equivalent: `bool waitUntilDate(Date* pLimit)`
    #[inline]
    pub fn wait_until_date(&self, limit: &Date) -> bool {
        unsafe { msg_send_1(self.as_ptr(), sel!(waitUntilDate:), limit.as_ptr()) }
    }

    /// Signal one waiting thread.
    ///
    /// C++ equivalent: `void signal()`
    #[inline]
    pub fn signal(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(signal));
        }
    }

    /// Signal all waiting threads.
    ///
    /// C++ equivalent: `void broadcast()`
    #[inline]
    pub fn broadcast(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(broadcast));
        }
    }

    /// Create a new condition (convenience method).
    #[inline]
    pub fn new() -> Option<Self> {
        Self::alloc()?.init()
    }

    /// Create a Condition from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C NSCondition object.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }
}

impl Referencing for Condition {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

impl Locking for Condition {}

unsafe impl Send for Condition {}
unsafe impl Sync for Condition {}

impl std::fmt::Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Condition").field("ptr", &self.0).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_size() {
        assert_eq!(
            std::mem::size_of::<Condition>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
