// Clippy allows for legacy API patterns
#![allow(clippy::self_named_constructors)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::new_without_default)]

//! Foundation framework bindings for Rust.
//!
//! This crate provides Rust bindings to Apple's Foundation framework,
//! corresponding to the C++ headers in `metal-cpp/Foundation/`.
//!
//! # Overview
//!
//! The Foundation framework provides fundamental classes for Objective-C
//! programming, including:
//!
//! - Basic types: [`Integer`], [`UInteger`], [`TimeInterval`]
//! - Objects: [`Object`], [`String`], [`Array`], [`Dictionary`], [`Data`]
//! - Numbers: [`Number`], [`Value`]
//! - Error handling: [`Error`]
//! - URL handling: [`Url`]
//! - Memory management: [`SharedPtr`], [`AutoreleasePool`]
//! - System info: [`ProcessInfo`], [`Bundle`]
//!
//! # C++ Correspondence
//!
//! This crate provides 1:1 correspondence with the metal-cpp Foundation headers:
//!
//! | C++ Header | Rust Module |
//! |------------|-------------|
//! | `NSTypes.hpp` | [`types`] |
//! | `NSObjCRuntime.hpp` | [`objc_runtime`] |
//! | `NSRange.hpp` | [`range`] |
//! | `NSObject.hpp` | [`object`] |
//! | `NSSharedPtr.hpp` | [`shared_ptr`] |
//! | `NSString.hpp` | [`string`] |
//! | `NSArray.hpp` | [`array`] |
//! | `NSDictionary.hpp` | [`dictionary`] |
//! | `NSSet.hpp` | [`set`] |
//! | `NSData.hpp` | [`data`] |
//! | `NSNumber.hpp` | [`number`] |
//! | `NSDate.hpp` | [`date`] |
//! | `NSError.hpp` | [`error`] |
//! | `NSURL.hpp` | [`url`] |
//! | `NSEnumerator.hpp` | [`enumerator`] |
//! | `NSAutoreleasePool.hpp` | [`autorelease`] |
//! | `NSNotification.hpp` | [`notification`] |
//! | `NSLock.hpp` | [`lock`] |
//! | `NSBundle.hpp` | [`bundle`] |
//! | `NSProcessInfo.hpp` | [`process_info`] |

// Modules
pub mod array;
pub mod autorelease;
pub mod bundle;
pub mod data;
pub mod date;
pub mod dictionary;
pub mod enumerator;
pub mod error;
pub mod lock;
pub mod notification;
pub mod number;
pub mod objc_runtime;
pub mod object;
pub mod process_info;
pub mod range;
pub mod set;
pub mod shared_ptr;
pub mod string;
pub mod types;
pub mod url;

// Re-export commonly used types at crate root

// Types
pub use types::{INTEGER_MAX, INTEGER_MIN, UINTEGER_MAX};
pub use types::{Integer, OperatingSystemVersion, TimeInterval, UInteger};

// ObjC Runtime
pub use objc_runtime::{ComparisonResult, NOT_FOUND};

// Range
pub use range::Range;

// Object traits
pub use object::{Copying, Object, Referencing, SecureCoding};

// SharedPtr
pub use shared_ptr::{SharedPtr, retain_ptr, transfer_ptr};

// String
pub use string::{String, StringCompareOptions, StringEncoding, Unichar};

// Array
pub use array::Array;

// Dictionary
pub use dictionary::Dictionary;

// Set
pub use set::Set;

// Data
pub use data::Data;

// Number
pub use number::{Number, Value};

// Date
pub use date::Date;

// Error
pub use error::{Error, ErrorDomain, ErrorUserInfoKey};
pub use error::{
    cocoa_error_domain, debug_description_error_key, file_path_error_key, help_anchor_error_key,
    localized_description_key, localized_failure_error_key, localized_failure_reason_error_key,
    localized_recovery_options_error_key, localized_recovery_suggestion_error_key,
    mach_error_domain, os_status_error_domain, posix_error_domain, recovery_attempter_error_key,
    string_encoding_error_key, underlying_error_key, url_error_key,
};

// Enumerator
pub use enumerator::{Enumerator, FastEnumeration, FastEnumerationState};

// URL
pub use url::Url;

// Autorelease
pub use autorelease::{AutoreleasePool, AutoreleasePoolScope};

// Notification
pub use notification::{Notification, NotificationCenter, NotificationName};

// Lock
pub use lock::{Condition, Locking};

// Bundle
pub use bundle::Bundle;
pub use bundle::{
    localized_string, localized_string_from_table, localized_string_from_table_in_bundle,
    localized_string_with_default_value,
};

// ProcessInfo
pub use process_info::{
    ActivityOptions, DeviceCertification, ProcessInfo, ProcessInfoThermalState,
    ProcessPerformanceProfile,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_imports() {
        // Verify that all re-exports compile
        let _: Integer = 0;
        let _: UInteger = 0;
        let _ = ComparisonResult::ORDERED_SAME;
        let _ = Range::new(0, 10);
    }
}
