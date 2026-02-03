//! Function log types.
//!
//! Corresponds to `Metal/MTLFunctionLog.hpp`.
//!
//! These types provide access to validation and debug information
//! from shader function execution.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, sel};

use crate::enums::FunctionLogType;
use crate::library::Function;

// ============================================================================
// LogContainer
// ============================================================================

/// A container for function logs.
///
/// C++ equivalent: `MTL::LogContainer`
///
/// This is an opaque container that holds function log entries.
/// It conforms to FastEnumeration in Objective-C, which maps to
/// iteration in Rust.
#[repr(transparent)]
pub struct LogContainer(pub(crate) NonNull<c_void>);

impl LogContainer {
    /// Create from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }
}

impl Clone for LogContainer {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for LogContainer {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for LogContainer {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for LogContainer {}
unsafe impl Sync for LogContainer {}

// ============================================================================
// FunctionLogDebugLocation
// ============================================================================

/// Debug location information for a function log entry.
///
/// C++ equivalent: `MTL::FunctionLogDebugLocation`
///
/// Contains source file location information including URL, line number,
/// column number, and function name.
#[repr(transparent)]
pub struct FunctionLogDebugLocation(pub(crate) NonNull<c_void>);

impl FunctionLogDebugLocation {
    /// Create from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the source file URL.
    ///
    /// C++ equivalent: `NS::URL* URL() const`
    pub fn url(&self) -> Option<String> {
        unsafe {
            let url_ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(URL));
            if url_ptr.is_null() {
                return None;
            }
            // Get absoluteString from NSURL
            let str_ptr: *mut c_void = msg_send_0(url_ptr, sel!(absoluteString));
            if str_ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                mtl_sys::msg_send_0(str_ptr, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Get the line number.
    ///
    /// C++ equivalent: `NS::UInteger line() const`
    #[inline]
    pub fn line(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(line)) }
    }

    /// Get the column number.
    ///
    /// C++ equivalent: `NS::UInteger column() const`
    #[inline]
    pub fn column(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(column)) }
    }

    /// Get the function name.
    ///
    /// C++ equivalent: `NS::String* functionName() const`
    pub fn function_name(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(functionName));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                mtl_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }
}

impl Clone for FunctionLogDebugLocation {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for FunctionLogDebugLocation {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for FunctionLogDebugLocation {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for FunctionLogDebugLocation {}
unsafe impl Sync for FunctionLogDebugLocation {}

// ============================================================================
// FunctionLog
// ============================================================================

/// A log entry from shader function execution.
///
/// C++ equivalent: `MTL::FunctionLog`
///
/// Contains information about validation errors or other log messages
/// generated during shader execution.
#[repr(transparent)]
pub struct FunctionLog(pub(crate) NonNull<c_void>);

impl FunctionLog {
    /// Create from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the log type.
    ///
    /// C++ equivalent: `FunctionLogType type() const`
    #[inline]
    pub fn log_type(&self) -> FunctionLogType {
        unsafe { msg_send_0(self.as_ptr(), sel!(type)) }
    }

    /// Get the debug location.
    ///
    /// C++ equivalent: `FunctionLogDebugLocation* debugLocation() const`
    pub fn debug_location(&self) -> Option<FunctionLogDebugLocation> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(debugLocation));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            FunctionLogDebugLocation::from_raw(ptr)
        }
    }

    /// Get the encoder label.
    ///
    /// C++ equivalent: `NS::String* encoderLabel() const`
    pub fn encoder_label(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(encoderLabel));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                mtl_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Get the function that generated this log entry.
    ///
    /// C++ equivalent: `Function* function() const`
    pub fn function(&self) -> Option<Function> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(function));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Function::from_raw(ptr)
        }
    }
}

impl Clone for FunctionLog {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for FunctionLog {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for FunctionLog {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for FunctionLog {}
unsafe impl Sync for FunctionLog {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_sizes() {
        assert_eq!(
            std::mem::size_of::<LogContainer>(),
            std::mem::size_of::<*mut c_void>()
        );
        assert_eq!(
            std::mem::size_of::<FunctionLogDebugLocation>(),
            std::mem::size_of::<*mut c_void>()
        );
        assert_eq!(
            std::mem::size_of::<FunctionLog>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
