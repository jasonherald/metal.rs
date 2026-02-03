//! Metal log state types.
//!
//! Corresponds to `Metal/MTLLogState.hpp`.
//!
//! Log states allow you to capture and process GPU shader logs for debugging
//! and validation purposes.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel, Class};

use crate::enums::LogLevel;

// ============================================================================
// LogStateDescriptor
// ============================================================================

/// Descriptor for creating a log state.
///
/// C++ equivalent: `MTL::LogStateDescriptor`
#[repr(transparent)]
pub struct LogStateDescriptor(NonNull<c_void>);

impl LogStateDescriptor {
    /// Allocate a new log state descriptor.
    ///
    /// C++ equivalent: `static LogStateDescriptor* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let cls = Class::get("MTLLogStateDescriptor")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize an allocated descriptor.
    ///
    /// C++ equivalent: `LogStateDescriptor* init()`
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new log state descriptor.
    pub fn new() -> Option<Self> {
        Self::alloc()?.init()
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal log state descriptor.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the buffer size.
    ///
    /// C++ equivalent: `NS::UInteger bufferSize() const`
    #[inline]
    pub fn buffer_size(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(bufferSize)) }
    }

    /// Set the buffer size.
    ///
    /// C++ equivalent: `void setBufferSize(NS::UInteger)`
    #[inline]
    pub fn set_buffer_size(&self, size: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setBufferSize:), size);
        }
    }

    /// Get the log level.
    ///
    /// C++ equivalent: `LogLevel level() const`
    #[inline]
    pub fn level(&self) -> LogLevel {
        unsafe { msg_send_0(self.as_ptr(), sel!(level)) }
    }

    /// Set the log level.
    ///
    /// C++ equivalent: `void setLevel(LogLevel)`
    #[inline]
    pub fn set_level(&self, level: LogLevel) {
        unsafe {
            msg_send_1::<(), LogLevel>(self.as_ptr(), sel!(setLevel:), level);
        }
    }
}

impl Default for LogStateDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create LogStateDescriptor")
    }
}

impl Clone for LogStateDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy LogStateDescriptor")
        }
    }
}

impl Drop for LogStateDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for LogStateDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for LogStateDescriptor {}
unsafe impl Sync for LogStateDescriptor {}

impl std::fmt::Debug for LogStateDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogStateDescriptor")
            .field("buffer_size", &self.buffer_size())
            .field("level", &self.level())
            .finish()
    }
}

// ============================================================================
// LogState
// ============================================================================

/// A log state for capturing GPU shader logs.
///
/// C++ equivalent: `MTL::LogState`
///
/// Log states are created from a device using a descriptor. They can be
/// attached to command buffers to capture shader logs.
#[repr(transparent)]
pub struct LogState(NonNull<c_void>);

impl LogState {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal log state.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Add a log handler (raw block pointer version).
    ///
    /// C++ equivalent: `void addLogHandler(void(^)(NS::String*, NS::String*, LogLevel, NS::String*))`
    ///
    /// # Safety
    ///
    /// The block pointer must be a valid Objective-C block.
    pub unsafe fn add_log_handler_raw(&self, block: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(addLogHandler:), block);
        }
    }

    /// Add a log handler to receive shader log messages.
    ///
    /// C++ equivalent: `void addLogHandler(void(^)(NS::String*, NS::String*, LogLevel, NS::String*))`
    ///
    /// The handler is called with:
    /// - `subsystem` - The subsystem that generated the log
    /// - `category` - The category of the log message
    /// - `level` - The severity level of the log message
    /// - `message` - The actual log message content
    pub fn add_log_handler<F>(&self, handler: F)
    where
        F: Fn(&str, &str, LogLevel, &str) + Send + 'static,
    {
        let block = metal_sys::LogHandlerBlock::from_fn(
            move |subsystem_ptr: *mut c_void,
                  category_ptr: *mut c_void,
                  level: isize,
                  message_ptr: *mut c_void| {
                unsafe {
                    // Convert NSString pointers to Rust strings
                    let subsystem = nsstring_to_str(subsystem_ptr);
                    let category = nsstring_to_str(category_ptr);
                    let message = nsstring_to_str(message_ptr);

                    // Create LogLevel from the raw integer value
                    let log_level = LogLevel(level);

                    handler(&subsystem, &category, log_level, &message);
                }
            },
        );

        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(addLogHandler:), block.as_ptr());
        }

        // Transfer ownership to Metal
        std::mem::forget(block);
    }
}

/// Convert an NSString pointer to a Rust String.
///
/// # Safety
///
/// The pointer must be a valid NSString or null.
unsafe fn nsstring_to_str(ns_string: *mut c_void) -> String {
    if ns_string.is_null() {
        return String::new();
    }

    unsafe {
        let c_str: *const i8 = msg_send_0(ns_string, sel!(UTF8String));
        if c_str.is_null() {
            return String::new();
        }

        std::ffi::CStr::from_ptr(c_str)
            .to_string_lossy()
            .into_owned()
    }
}

impl Clone for LogState {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for LogState {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for LogState {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for LogState {}
unsafe impl Sync for LogState {}

impl std::fmt::Debug for LogState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogState").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_state_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<LogStateDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_log_state_size() {
        assert_eq!(
            std::mem::size_of::<LogState>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
