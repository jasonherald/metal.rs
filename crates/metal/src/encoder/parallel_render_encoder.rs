//! Parallel render command encoder.
//!
//! Corresponds to `Metal/MTLParallelRenderCommandEncoder.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{StoreAction, StoreActionOptions};

/// A command encoder that creates multiple render command encoders that render in parallel.
///
/// C++ equivalent: `MTL::ParallelRenderCommandEncoder`
///
/// Parallel render encoders allow you to split rendering work across multiple
/// render command encoders that can execute concurrently on the GPU.
#[repr(transparent)]
pub struct ParallelRenderCommandEncoder(pub(crate) NonNull<c_void>);

impl ParallelRenderCommandEncoder {
    /// Create a ParallelRenderCommandEncoder from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal parallel render command encoder object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the encoder.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // CommandEncoder base methods
    // =========================================================================

    /// Get the device that created this encoder.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("encoder has no device")
        }
    }

    /// Get the command buffer that this encoder is encoding into.
    ///
    /// C++ equivalent: `CommandBuffer* commandBuffer() const`
    pub fn command_buffer(&self) -> crate::CommandBuffer {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(commandBuffer));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::CommandBuffer::from_raw(ptr).expect("encoder has no command buffer")
        }
    }

    /// Get the label for this encoder.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                metal_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Set the label for this encoder.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// End encoding commands with this encoder.
    ///
    /// C++ equivalent: `void endEncoding()`
    #[inline]
    pub fn end_encoding(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(endEncoding));
        }
    }

    /// Insert a debug signpost.
    ///
    /// C++ equivalent: `void insertDebugSignpost(const NS::String*)`
    pub fn insert_debug_signpost(&self, string: &str) {
        if let Some(ns_string) = metal_foundation::String::from_str(string) {
            unsafe {
                msg_send_1::<(), *const c_void>(
                    self.as_ptr(),
                    sel!(insertDebugSignpost:),
                    ns_string.as_ptr(),
                );
            }
        }
    }

    /// Push a debug group.
    ///
    /// C++ equivalent: `void pushDebugGroup(const NS::String*)`
    pub fn push_debug_group(&self, string: &str) {
        if let Some(ns_string) = metal_foundation::String::from_str(string) {
            unsafe {
                msg_send_1::<(), *const c_void>(
                    self.as_ptr(),
                    sel!(pushDebugGroup:),
                    ns_string.as_ptr(),
                );
            }
        }
    }

    /// Pop the current debug group.
    ///
    /// C++ equivalent: `void popDebugGroup()`
    #[inline]
    pub fn pop_debug_group(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(popDebugGroup));
        }
    }

    /// Insert a barrier to synchronize queue stages.
    ///
    /// C++ equivalent: `void barrierAfterQueueStages(Stages, Stages)`
    #[inline]
    pub fn barrier_after_queue_stages(
        &self,
        after_stages: crate::enums::Stages,
        before_stages: crate::enums::Stages,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), crate::enums::Stages, crate::enums::Stages>(
                self.as_ptr(),
                sel!(barrierAfterQueueStages:beforeQueueStages:),
                after_stages,
                before_stages,
            );
        }
    }

    // =========================================================================
    // Parallel Render Encoder Methods
    // =========================================================================

    /// Create a new render command encoder that renders in parallel.
    ///
    /// C++ equivalent: `RenderCommandEncoder* renderCommandEncoder()`
    ///
    /// Each call creates a new render encoder that shares the render pass
    /// with other encoders created from this parallel encoder.
    #[inline]
    pub fn render_command_encoder(&self) -> Option<crate::RenderCommandEncoder> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(renderCommandEncoder));
            if ptr.is_null() {
                None
            } else {
                // The returned encoder is autoreleased, so we retain it
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
                crate::encoder::RenderCommandEncoder::from_raw(ptr)
            }
        }
    }

    /// Set the store action for a color attachment.
    ///
    /// C++ equivalent: `void setColorStoreAction(StoreAction, NS::UInteger)`
    #[inline]
    pub fn set_color_store_action(
        &self,
        store_action: StoreAction,
        color_attachment_index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), StoreAction, UInteger>(
                self.as_ptr(),
                sel!(setColorStoreAction: atIndex:),
                store_action,
                color_attachment_index,
            );
        }
    }

    /// Set the store action options for a color attachment.
    ///
    /// C++ equivalent: `void setColorStoreActionOptions(StoreActionOptions, NS::UInteger)`
    #[inline]
    pub fn set_color_store_action_options(
        &self,
        store_action_options: StoreActionOptions,
        color_attachment_index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), StoreActionOptions, UInteger>(
                self.as_ptr(),
                sel!(setColorStoreActionOptions: atIndex:),
                store_action_options,
                color_attachment_index,
            );
        }
    }

    /// Set the store action for the depth attachment.
    ///
    /// C++ equivalent: `void setDepthStoreAction(StoreAction)`
    #[inline]
    pub fn set_depth_store_action(&self, store_action: StoreAction) {
        unsafe {
            msg_send_1::<(), StoreAction>(self.as_ptr(), sel!(setDepthStoreAction:), store_action);
        }
    }

    /// Set the store action options for the depth attachment.
    ///
    /// C++ equivalent: `void setDepthStoreActionOptions(StoreActionOptions)`
    #[inline]
    pub fn set_depth_store_action_options(&self, store_action_options: StoreActionOptions) {
        unsafe {
            msg_send_1::<(), StoreActionOptions>(
                self.as_ptr(),
                sel!(setDepthStoreActionOptions:),
                store_action_options,
            );
        }
    }

    /// Set the store action for the stencil attachment.
    ///
    /// C++ equivalent: `void setStencilStoreAction(StoreAction)`
    #[inline]
    pub fn set_stencil_store_action(&self, store_action: StoreAction) {
        unsafe {
            msg_send_1::<(), StoreAction>(
                self.as_ptr(),
                sel!(setStencilStoreAction:),
                store_action,
            );
        }
    }

    /// Set the store action options for the stencil attachment.
    ///
    /// C++ equivalent: `void setStencilStoreActionOptions(StoreActionOptions)`
    #[inline]
    pub fn set_stencil_store_action_options(&self, store_action_options: StoreActionOptions) {
        unsafe {
            msg_send_1::<(), StoreActionOptions>(
                self.as_ptr(),
                sel!(setStencilStoreActionOptions:),
                store_action_options,
            );
        }
    }
}

impl Clone for ParallelRenderCommandEncoder {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ParallelRenderCommandEncoder {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for ParallelRenderCommandEncoder {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ParallelRenderCommandEncoder {}
unsafe impl Sync for ParallelRenderCommandEncoder {}

impl std::fmt::Debug for ParallelRenderCommandEncoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParallelRenderCommandEncoder")
            .field("label", &self.label())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_render_encoder_size() {
        assert_eq!(
            std::mem::size_of::<ParallelRenderCommandEncoder>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
