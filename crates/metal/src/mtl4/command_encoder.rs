//! MTL4 CommandEncoder implementation.
//!
//! Corresponds to `Metal/MTL4CommandEncoder.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, msg_send_3, sel};

use super::enums::VisibilityOptions;
use crate::Device;

// ============================================================
// CommandEncoder
// ============================================================

/// Base command encoder for MTL4.
///
/// C++ equivalent: `MTL4::CommandEncoder`
///
/// CommandEncoder provides the base interface for all MTL4 command encoders,
/// including barrier operations, fence management, and debug groups.
#[repr(transparent)]
pub struct CommandEncoder(NonNull<c_void>);

impl CommandEncoder {
    /// Create a CommandEncoder from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the device.
    ///
    /// C++ equivalent: `MTL::Device* device() const`
    pub fn device(&self) -> Option<Device> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            Device::from_raw(ptr)
        }
    }

    /// Get the label.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ns_string: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ns_string.is_null() {
                return None;
            }
            let c_str: *const i8 = msg_send_0(ns_string, sel!(UTF8String));
            if c_str.is_null() {
                return None;
            }
            Some(
                std::ffi::CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    }

    /// Set the label.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    // ========== Barrier Methods ==========

    /// Insert a barrier for all resources.
    ///
    /// C++ equivalent: `void barrier()`
    pub fn barrier(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(barrier));
        }
    }

    /// Insert a barrier for a specific buffer.
    ///
    /// C++ equivalent: `void barrier(const MTL::Buffer*, MTL4::VisibilityOptions)`
    pub fn barrier_buffer(&self, buffer: *const c_void, visibility: VisibilityOptions) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(barrierWithBuffer:visibilityOptions:),
                buffer,
                visibility.0,
            );
        }
    }

    /// Insert a barrier for multiple buffers.
    ///
    /// C++ equivalent: `void barrier(const MTL::Buffer* const*, NS::UInteger, MTL4::VisibilityOptions)`
    pub fn barrier_buffers(
        &self,
        buffers: *const *const c_void,
        count: UInteger,
        visibility: VisibilityOptions,
    ) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(barrierWithBuffers:count:visibilityOptions:),
                buffers,
                count,
                visibility.0,
            );
        }
    }

    /// Insert a barrier for a specific texture.
    ///
    /// C++ equivalent: `void barrier(const MTL::Texture*, MTL4::VisibilityOptions)`
    pub fn barrier_texture(&self, texture: *const c_void, visibility: VisibilityOptions) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(barrierWithTexture:visibilityOptions:),
                texture,
                visibility.0,
            );
        }
    }

    /// Insert a barrier for multiple textures.
    ///
    /// C++ equivalent: `void barrier(const MTL::Texture* const*, NS::UInteger, MTL4::VisibilityOptions)`
    pub fn barrier_textures(
        &self,
        textures: *const *const c_void,
        count: UInteger,
        visibility: VisibilityOptions,
    ) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(barrierWithTextures:count:visibilityOptions:),
                textures,
                count,
                visibility.0,
            );
        }
    }

    // ========== Fence Methods ==========

    /// Update a fence.
    ///
    /// C++ equivalent: `void updateFence(const MTL::Fence*)`
    pub fn update_fence(&self, fence: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(updateFence:), fence);
        }
    }

    /// Wait for a fence.
    ///
    /// C++ equivalent: `void waitForFence(const MTL::Fence*)`
    pub fn wait_for_fence(&self, fence: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(waitForFence:), fence);
        }
    }

    // ========== Debug Methods ==========

    /// Push a debug group.
    ///
    /// C++ equivalent: `void pushDebugGroup(const NS::String*)`
    pub fn push_debug_group(&self, name: &str) {
        if let Some(ns_name) = metal_foundation::String::from_str(name) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(pushDebugGroup:), ns_name.as_ptr());
            }
        }
    }

    /// Pop a debug group.
    ///
    /// C++ equivalent: `void popDebugGroup()`
    pub fn pop_debug_group(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(popDebugGroup));
        }
    }

    /// Insert a debug signpost.
    ///
    /// C++ equivalent: `void insertDebugSignpost(const NS::String*)`
    pub fn insert_debug_signpost(&self, name: &str) {
        if let Some(ns_name) = metal_foundation::String::from_str(name) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(insertDebugSignpost:), ns_name.as_ptr());
            }
        }
    }

    // ========== Encoding ==========

    /// End encoding.
    ///
    /// C++ equivalent: `void endEncoding()`
    pub fn end_encoding(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(endEncoding));
        }
    }
}

impl Clone for CommandEncoder {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CommandEncoder {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for CommandEncoder {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CommandEncoder {}
unsafe impl Sync for CommandEncoder {}

impl std::fmt::Debug for CommandEncoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandEncoder")
            .field("label", &self.label())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_encoder_size() {
        assert_eq!(
            std::mem::size_of::<CommandEncoder>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
