//! Date type for Foundation.
//!
//! Corresponds to `Foundation/NSDate.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! using TimeInterval = double;
//!
//! class Date : public Copying<Date> {
//! public:
//!     static Date* dateWithTimeIntervalSinceNow(TimeInterval secs);
//! };
//! }
//! ```

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_sys::{class, msg_send_0, msg_send_1, sel};

use crate::object::{Copying, Referencing};
use crate::types::TimeInterval;

/// An Objective-C date object.
///
/// C++ equivalent: `NS::Date`
#[repr(transparent)]
#[derive(Clone)]
pub struct Date(NonNull<c_void>);

impl Date {
    /// Create a date with a time interval since now.
    ///
    /// C++ equivalent: `static Date* dateWithTimeIntervalSinceNow(TimeInterval secs)`
    #[inline]
    pub fn date_with_time_interval_since_now(secs: TimeInterval) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                class!(NSDate).as_ptr(),
                sel!(dateWithTimeIntervalSinceNow:),
                secs,
            );
            Self::from_ptr(ptr)
        }
    }

    /// Create a Date from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C NSDate object.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }
}

impl Referencing for Date {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

impl Copying for Date {
    #[inline]
    fn copy(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_ptr(ptr)
        }
    }
}

unsafe impl Send for Date {}
unsafe impl Sync for Date {}

impl std::fmt::Debug for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Date").field("ptr", &self.0).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_size() {
        assert_eq!(
            std::mem::size_of::<Date>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
