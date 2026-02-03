//! Command buffer encoder info.
//!
//! Corresponds to `MTL::CommandBufferEncoderInfo` in `Metal/MTLCommandBuffer.hpp`.
//!
//! Provides information about encoders that have been used in a command buffer,
//! useful for debugging and error handling.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_0, sel};

use crate::enums::CommandEncoderErrorState;

/// Information about an encoder in a command buffer.
///
/// C++ equivalent: `MTL::CommandBufferEncoderInfo`
///
/// This type provides debugging information about encoders that have
/// contributed to a command buffer, including their error state and
/// debug signposts.
#[repr(transparent)]
pub struct CommandBufferEncoderInfo(pub(crate) NonNull<c_void>);

impl CommandBufferEncoderInfo {
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

    /// Get the label of the encoder.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
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

    /// Get the error state of the encoder.
    ///
    /// C++ equivalent: `CommandEncoderErrorState errorState() const`
    #[inline]
    pub fn error_state(&self) -> CommandEncoderErrorState {
        unsafe { msg_send_0(self.as_ptr(), sel!(errorState)) }
    }

    /// Get the debug signposts as raw pointer.
    ///
    /// Returns a pointer to an NSArray of debug signpost strings.
    ///
    /// C++ equivalent: `NS::Array* debugSignposts() const`
    #[inline]
    pub fn debug_signposts_ptr(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(debugSignposts)) }
    }
}

impl Clone for CommandBufferEncoderInfo {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CommandBufferEncoderInfo {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for CommandBufferEncoderInfo {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CommandBufferEncoderInfo {}
unsafe impl Sync for CommandBufferEncoderInfo {}

impl std::fmt::Debug for CommandBufferEncoderInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandBufferEncoderInfo")
            .field("label", &self.label())
            .field("error_state", &self.error_state())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_size() {
        assert_eq!(
            std::mem::size_of::<CommandBufferEncoderInfo>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
