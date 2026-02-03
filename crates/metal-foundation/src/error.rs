//! Error type for Foundation.
//!
//! Corresponds to `Foundation/NSError.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! using ErrorDomain = class String*;
//!
//! _NS_CONST(ErrorDomain, CocoaErrorDomain);
//! _NS_CONST(ErrorDomain, POSIXErrorDomain);
//! _NS_CONST(ErrorDomain, OSStatusErrorDomain);
//! _NS_CONST(ErrorDomain, MachErrorDomain);
//!
//! using ErrorUserInfoKey = class String*;
//!
//! _NS_CONST(ErrorUserInfoKey, UnderlyingErrorKey);
//! _NS_CONST(ErrorUserInfoKey, LocalizedDescriptionKey);
//! _NS_CONST(ErrorUserInfoKey, LocalizedFailureReasonErrorKey);
//! _NS_CONST(ErrorUserInfoKey, LocalizedRecoverySuggestionErrorKey);
//! _NS_CONST(ErrorUserInfoKey, LocalizedRecoveryOptionsErrorKey);
//! _NS_CONST(ErrorUserInfoKey, RecoveryAttempterErrorKey);
//! _NS_CONST(ErrorUserInfoKey, HelpAnchorErrorKey);
//! _NS_CONST(ErrorUserInfoKey, DebugDescriptionErrorKey);
//! _NS_CONST(ErrorUserInfoKey, LocalizedFailureErrorKey);
//! _NS_CONST(ErrorUserInfoKey, StringEncodingErrorKey);
//! _NS_CONST(ErrorUserInfoKey, URLErrorKey);
//! _NS_CONST(ErrorUserInfoKey, FilePathErrorKey);
//!
//! class Error : public Copying<Error> {
//! public:
//!     static Error*     error(ErrorDomain domain, Integer code, class Dictionary* pDictionary);
//!     static Error*     alloc();
//!     Error*            init();
//!     Error*            init(ErrorDomain domain, Integer code, class Dictionary* pDictionary);
//!     Integer           code() const;
//!     ErrorDomain       domain() const;
//!     class Dictionary* userInfo() const;
//!     class String*     localizedDescription() const;
//!     class Array*      localizedRecoveryOptions() const;
//!     class String*     localizedRecoverySuggestion() const;
//!     class String*     localizedFailureReason() const;
//! };
//! }
//! ```

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_sys::{class, msg_send_0, msg_send_3, sel};

use crate::array::Array;
use crate::dictionary::Dictionary;
use crate::object::{Copying, Referencing};
use crate::string::String;
use crate::types::Integer;

/// Error domain type (alias for String pointer).
///
/// C++ equivalent: `NS::ErrorDomain`
pub type ErrorDomain = *mut String;

/// Error user info key type (alias for String pointer).
///
/// C++ equivalent: `NS::ErrorUserInfoKey`
pub type ErrorUserInfoKey = *mut String;

// Error domain constants - linked from Foundation framework
#[link(name = "Foundation", kind = "framework")]
unsafe extern "C" {
    #[link_name = "NSCocoaErrorDomain"]
    static COCOA_ERROR_DOMAIN: *mut c_void;

    #[link_name = "NSPOSIXErrorDomain"]
    static POSIX_ERROR_DOMAIN: *mut c_void;

    #[link_name = "NSOSStatusErrorDomain"]
    static OS_STATUS_ERROR_DOMAIN: *mut c_void;

    #[link_name = "NSMachErrorDomain"]
    static MACH_ERROR_DOMAIN: *mut c_void;

    #[link_name = "NSUnderlyingErrorKey"]
    static UNDERLYING_ERROR_KEY: *mut c_void;

    #[link_name = "NSLocalizedDescriptionKey"]
    static LOCALIZED_DESCRIPTION_KEY: *mut c_void;

    #[link_name = "NSLocalizedFailureReasonErrorKey"]
    static LOCALIZED_FAILURE_REASON_ERROR_KEY: *mut c_void;

    #[link_name = "NSLocalizedRecoverySuggestionErrorKey"]
    static LOCALIZED_RECOVERY_SUGGESTION_ERROR_KEY: *mut c_void;

    #[link_name = "NSLocalizedRecoveryOptionsErrorKey"]
    static LOCALIZED_RECOVERY_OPTIONS_ERROR_KEY: *mut c_void;

    #[link_name = "NSRecoveryAttempterErrorKey"]
    static RECOVERY_ATTEMPTER_ERROR_KEY: *mut c_void;

    #[link_name = "NSHelpAnchorErrorKey"]
    static HELP_ANCHOR_ERROR_KEY: *mut c_void;

    #[link_name = "NSDebugDescriptionErrorKey"]
    static DEBUG_DESCRIPTION_ERROR_KEY: *mut c_void;

    #[link_name = "NSLocalizedFailureErrorKey"]
    static LOCALIZED_FAILURE_ERROR_KEY: *mut c_void;

    #[link_name = "NSStringEncodingErrorKey"]
    static STRING_ENCODING_ERROR_KEY: *mut c_void;

    #[link_name = "NSURLErrorKey"]
    static URL_ERROR_KEY: *mut c_void;

    #[link_name = "NSFilePathErrorKey"]
    static FILE_PATH_ERROR_KEY: *mut c_void;
}

/// Get the Cocoa error domain.
///
/// C++ equivalent: `NS::CocoaErrorDomain`
#[inline]
pub fn cocoa_error_domain() -> ErrorDomain {
    unsafe { COCOA_ERROR_DOMAIN as ErrorDomain }
}

/// Get the POSIX error domain.
///
/// C++ equivalent: `NS::POSIXErrorDomain`
#[inline]
pub fn posix_error_domain() -> ErrorDomain {
    unsafe { POSIX_ERROR_DOMAIN as ErrorDomain }
}

/// Get the OSStatus error domain.
///
/// C++ equivalent: `NS::OSStatusErrorDomain`
#[inline]
pub fn os_status_error_domain() -> ErrorDomain {
    unsafe { OS_STATUS_ERROR_DOMAIN as ErrorDomain }
}

/// Get the Mach error domain.
///
/// C++ equivalent: `NS::MachErrorDomain`
#[inline]
pub fn mach_error_domain() -> ErrorDomain {
    unsafe { MACH_ERROR_DOMAIN as ErrorDomain }
}

/// Get the underlying error key.
///
/// C++ equivalent: `NS::UnderlyingErrorKey`
#[inline]
pub fn underlying_error_key() -> ErrorUserInfoKey {
    unsafe { UNDERLYING_ERROR_KEY as ErrorUserInfoKey }
}

/// Get the localized description key.
///
/// C++ equivalent: `NS::LocalizedDescriptionKey`
#[inline]
pub fn localized_description_key() -> ErrorUserInfoKey {
    unsafe { LOCALIZED_DESCRIPTION_KEY as ErrorUserInfoKey }
}

/// Get the localized failure reason error key.
///
/// C++ equivalent: `NS::LocalizedFailureReasonErrorKey`
#[inline]
pub fn localized_failure_reason_error_key() -> ErrorUserInfoKey {
    unsafe { LOCALIZED_FAILURE_REASON_ERROR_KEY as ErrorUserInfoKey }
}

/// Get the localized recovery suggestion error key.
///
/// C++ equivalent: `NS::LocalizedRecoverySuggestionErrorKey`
#[inline]
pub fn localized_recovery_suggestion_error_key() -> ErrorUserInfoKey {
    unsafe { LOCALIZED_RECOVERY_SUGGESTION_ERROR_KEY as ErrorUserInfoKey }
}

/// Get the localized recovery options error key.
///
/// C++ equivalent: `NS::LocalizedRecoveryOptionsErrorKey`
#[inline]
pub fn localized_recovery_options_error_key() -> ErrorUserInfoKey {
    unsafe { LOCALIZED_RECOVERY_OPTIONS_ERROR_KEY as ErrorUserInfoKey }
}

/// Get the recovery attempter error key.
///
/// C++ equivalent: `NS::RecoveryAttempterErrorKey`
#[inline]
pub fn recovery_attempter_error_key() -> ErrorUserInfoKey {
    unsafe { RECOVERY_ATTEMPTER_ERROR_KEY as ErrorUserInfoKey }
}

/// Get the help anchor error key.
///
/// C++ equivalent: `NS::HelpAnchorErrorKey`
#[inline]
pub fn help_anchor_error_key() -> ErrorUserInfoKey {
    unsafe { HELP_ANCHOR_ERROR_KEY as ErrorUserInfoKey }
}

/// Get the debug description error key.
///
/// C++ equivalent: `NS::DebugDescriptionErrorKey`
#[inline]
pub fn debug_description_error_key() -> ErrorUserInfoKey {
    unsafe { DEBUG_DESCRIPTION_ERROR_KEY as ErrorUserInfoKey }
}

/// Get the localized failure error key.
///
/// C++ equivalent: `NS::LocalizedFailureErrorKey`
#[inline]
pub fn localized_failure_error_key() -> ErrorUserInfoKey {
    unsafe { LOCALIZED_FAILURE_ERROR_KEY as ErrorUserInfoKey }
}

/// Get the string encoding error key.
///
/// C++ equivalent: `NS::StringEncodingErrorKey`
#[inline]
pub fn string_encoding_error_key() -> ErrorUserInfoKey {
    unsafe { STRING_ENCODING_ERROR_KEY as ErrorUserInfoKey }
}

/// Get the URL error key.
///
/// C++ equivalent: `NS::URLErrorKey`
#[inline]
pub fn url_error_key() -> ErrorUserInfoKey {
    unsafe { URL_ERROR_KEY as ErrorUserInfoKey }
}

/// Get the file path error key.
///
/// C++ equivalent: `NS::FilePathErrorKey`
#[inline]
pub fn file_path_error_key() -> ErrorUserInfoKey {
    unsafe { FILE_PATH_ERROR_KEY as ErrorUserInfoKey }
}

/// An Objective-C error object.
///
/// C++ equivalent: `NS::Error`
#[repr(transparent)]
#[derive(Clone)]
pub struct Error(NonNull<c_void>);

impl Error {
    /// Create an error with the specified domain, code, and user info.
    ///
    /// C++ equivalent: `static Error* error(ErrorDomain domain, Integer code, class Dictionary* pDictionary)`
    #[inline]
    pub fn error(
        domain: ErrorDomain,
        code: Integer,
        user_info: *mut Dictionary,
    ) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_3(
                class!(NSError).as_ptr(),
                sel!(errorWithDomain:code:userInfo:),
                domain,
                code,
                user_info,
            );
            Self::from_ptr(ptr)
        }
    }

    /// Allocate a new error.
    ///
    /// C++ equivalent: `static Error* alloc()`
    #[inline]
    pub fn alloc() -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(class!(NSError).as_ptr(), sel!(alloc));
            Self::from_ptr(ptr)
        }
    }

    /// Initialize an allocated error.
    ///
    /// C++ equivalent: `Error* init()`
    #[inline]
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with domain, code, and user info.
    ///
    /// C++ equivalent: `Error* init(ErrorDomain domain, Integer code, class Dictionary* pDictionary)`
    #[inline]
    pub fn init_with_domain(
        &self,
        domain: ErrorDomain,
        code: Integer,
        user_info: *mut Dictionary,
    ) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(initWithDomain:code:userInfo:),
                domain,
                code,
                user_info,
            );
            Self::from_ptr(ptr)
        }
    }

    /// Get the error code.
    ///
    /// C++ equivalent: `Integer code() const`
    #[inline]
    pub fn code(&self) -> Integer {
        unsafe { msg_send_0(self.as_ptr(), sel!(code)) }
    }

    /// Get the error domain.
    ///
    /// C++ equivalent: `ErrorDomain domain() const`
    #[inline]
    pub fn domain(&self) -> ErrorDomain {
        unsafe { msg_send_0(self.as_ptr(), sel!(domain)) }
    }

    /// Get the user info dictionary.
    ///
    /// C++ equivalent: `class Dictionary* userInfo() const`
    #[inline]
    pub fn user_info(&self) -> *mut Dictionary {
        unsafe { msg_send_0(self.as_ptr(), sel!(userInfo)) }
    }

    /// Get the localized description.
    ///
    /// C++ equivalent: `class String* localizedDescription() const`
    #[inline]
    pub fn localized_description(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(localizedDescription)) }
    }

    /// Get the localized recovery options.
    ///
    /// C++ equivalent: `class Array* localizedRecoveryOptions() const`
    #[inline]
    pub fn localized_recovery_options(&self) -> *mut Array<String> {
        unsafe { msg_send_0(self.as_ptr(), sel!(localizedRecoveryOptions)) }
    }

    /// Get the localized recovery suggestion.
    ///
    /// C++ equivalent: `class String* localizedRecoverySuggestion() const`
    #[inline]
    pub fn localized_recovery_suggestion(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(localizedRecoverySuggestion)) }
    }

    /// Get the localized failure reason.
    ///
    /// C++ equivalent: `class String* localizedFailureReason() const`
    #[inline]
    pub fn localized_failure_reason(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(localizedFailureReason)) }
    }

    /// Create an Error from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C NSError object.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }
}

impl Referencing for Error {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

impl Copying for Error {
    #[inline]
    fn copy(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_ptr(ptr)
        }
    }
}

// SAFETY: NSError is thread-safe for reference counting
unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Error")
            .field("ptr", &self.0)
            .field("code", &self.code())
            .finish()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc_ptr = self.localized_description();
        if !desc_ptr.is_null() {
            let desc = unsafe { &*desc_ptr };
            if let Some(s) = desc.to_string() {
                return write!(f, "{}", s);
            }
        }
        write!(f, "Error(code={})", self.code())
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_size() {
        // Error should be pointer-sized
        assert_eq!(std::mem::size_of::<Error>(), std::mem::size_of::<*mut c_void>());
    }
}
