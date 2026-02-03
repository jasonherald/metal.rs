//! Notification types for Foundation.
//!
//! Corresponds to `Foundation/NSNotification.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! using NotificationName = class String*;
//!
//! class Notification : public NS::Referencing<Notification> {
//! public:
//!     NS::String*     name() const;
//!     NS::Object*     object() const;
//!     NS::Dictionary* userInfo() const;
//! };
//!
//! class NotificationCenter : public NS::Referencing<NotificationCenter> {
//! public:
//!     static class NotificationCenter* defaultCenter();
//!     Object* addObserver(NotificationName name, Object* pObj, void* pQueue, ObserverBlock block);
//!     void removeObserver(Object* pObserver);
//! };
//! }
//! ```

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_sys::{class, msg_send_0, msg_send_1, sel};

use crate::dictionary::Dictionary;
use crate::object::{Object, Referencing};
use crate::string::String;

/// Notification name type (alias for String pointer).
///
/// C++ equivalent: `NS::NotificationName`
pub type NotificationName = *mut String;

/// An Objective-C notification object.
///
/// C++ equivalent: `NS::Notification`
#[repr(transparent)]
#[derive(Clone)]
pub struct Notification(NonNull<c_void>);

impl Notification {
    /// Get the notification name.
    ///
    /// C++ equivalent: `NS::String* name() const`
    #[inline]
    pub fn name(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(name)) }
    }

    /// Get the notification object.
    ///
    /// C++ equivalent: `NS::Object* object() const`
    #[inline]
    pub fn object(&self) -> *mut Object {
        unsafe { msg_send_0(self.as_ptr(), sel!(object)) }
    }

    /// Get the notification user info dictionary.
    ///
    /// C++ equivalent: `NS::Dictionary* userInfo() const`
    #[inline]
    pub fn user_info(&self) -> *mut Dictionary {
        unsafe { msg_send_0(self.as_ptr(), sel!(userInfo)) }
    }

    /// Create a Notification from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C NSNotification object.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }
}

impl Referencing for Notification {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for Notification {}
unsafe impl Sync for Notification {}

impl std::fmt::Debug for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Notification")
            .field("ptr", &self.0)
            .finish()
    }
}

/// An Objective-C notification center.
///
/// C++ equivalent: `NS::NotificationCenter`
#[repr(transparent)]
#[derive(Clone)]
pub struct NotificationCenter(NonNull<c_void>);

impl NotificationCenter {
    /// Get the default notification center.
    ///
    /// C++ equivalent: `static class NotificationCenter* defaultCenter()`
    #[inline]
    pub fn default_center() -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_0(class!(NSNotificationCenter).as_ptr(), sel!(defaultCenter));
            Self::from_ptr(ptr)
        }
    }

    /// Add an observer with a block.
    ///
    /// C++ equivalent: `Object* addObserver(NotificationName name, Object* pObj, void* pQueue, ObserverBlock block)`
    ///
    /// Note: The block parameter is a raw pointer to an Objective-C block.
    /// Use with mtl_sys block types.
    #[inline]
    pub fn add_observer(
        &self,
        name: NotificationName,
        object: *mut Object,
        queue: *mut c_void,
        block: *const c_void,
    ) -> *mut Object {
        unsafe {
            mtl_sys::msg_send_4(
                self.as_ptr(),
                sel!(addObserverForName:object:queue:usingBlock:),
                name,
                object,
                queue,
                block,
            )
        }
    }

    /// Remove an observer.
    ///
    /// C++ equivalent: `void removeObserver(Object* pObserver)`
    #[inline]
    pub fn remove_observer(&self, observer: &Object) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(removeObserver:), observer.as_ptr());
        }
    }

    /// Create a NotificationCenter from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C NSNotificationCenter object.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }
}

impl Referencing for NotificationCenter {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for NotificationCenter {}
unsafe impl Sync for NotificationCenter {}

impl std::fmt::Debug for NotificationCenter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NotificationCenter")
            .field("ptr", &self.0)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_size() {
        assert_eq!(
            std::mem::size_of::<Notification>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_notification_center_size() {
        assert_eq!(
            std::mem::size_of::<NotificationCenter>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
