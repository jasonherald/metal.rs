//! MTL4 CommandBuffer implementation.
//!
//! Corresponds to `Metal/MTL4CommandBuffer.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, msg_send_5, sel};

use super::acceleration_structure::BufferRange;

use super::CommandAllocator;
use crate::{Device, ResidencySet};

// ============================================================
// CommandBufferOptions
// ============================================================

/// Options for MTL4 command buffer creation.
///
/// C++ equivalent: `MTL4::CommandBufferOptions`
#[repr(transparent)]
pub struct CommandBufferOptions(NonNull<c_void>);

impl CommandBufferOptions {
    /// Create a CommandBufferOptions from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create new command buffer options.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4CommandBufferOptions")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the log state.
    ///
    /// C++ equivalent: `MTL::LogState* logState() const`
    pub fn log_state(&self) -> Option<crate::log_state::LogState> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(logState));
            crate::log_state::LogState::from_raw(ptr)
        }
    }

    /// Set the log state.
    ///
    /// C++ equivalent: `void setLogState(MTL::LogState*)`
    pub fn set_log_state(&self, log_state: &crate::log_state::LogState) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setLogState:), log_state.as_ptr());
        }
    }
}

impl Clone for CommandBufferOptions {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CommandBufferOptions {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for CommandBufferOptions {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CommandBufferOptions {}
unsafe impl Sync for CommandBufferOptions {}

// ============================================================
// CommandBuffer
// ============================================================

/// MTL4 command buffer for recording GPU commands.
///
/// C++ equivalent: `MTL4::CommandBuffer`
///
/// CommandBuffer in Metal 4 provides explicit control over command recording
/// with an allocator-based memory model.
#[repr(transparent)]
pub struct CommandBuffer(NonNull<c_void>);

impl CommandBuffer {
    /// Create a CommandBuffer from a raw pointer.
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
    /// C++ equivalent: `void setLabel(const NS::String* label)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Begin recording commands with an allocator.
    ///
    /// C++ equivalent: `void beginCommandBuffer(const MTL4::CommandAllocator* allocator)`
    pub fn begin_command_buffer(&self, allocator: &CommandAllocator) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(beginCommandBufferWithAllocator:),
                allocator.as_ptr(),
            );
        }
    }

    /// Begin recording commands with an allocator and options.
    ///
    /// C++ equivalent: `void beginCommandBuffer(const MTL4::CommandAllocator*, const MTL4::CommandBufferOptions*)`
    pub fn begin_command_buffer_with_options(
        &self,
        allocator: &CommandAllocator,
        options: &CommandBufferOptions,
    ) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(beginCommandBufferWithAllocator:options:),
                allocator.as_ptr(),
                options.as_ptr(),
            );
        }
    }

    /// End recording commands.
    ///
    /// C++ equivalent: `void endCommandBuffer()`
    pub fn end_command_buffer(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(endCommandBuffer));
        }
    }

    /// Use a residency set.
    ///
    /// C++ equivalent: `void useResidencySet(const MTL::ResidencySet* residencySet)`
    pub fn use_residency_set(&self, residency_set: &ResidencySet) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(useResidencySet:),
                residency_set.as_ptr(),
            );
        }
    }

    /// Use multiple residency sets.
    ///
    /// C++ equivalent: `void useResidencySets(const MTL::ResidencySet* const[], NS::UInteger count)`
    pub fn use_residency_sets(&self, residency_sets: &[&ResidencySet]) {
        let ptrs: Vec<*const c_void> = residency_sets.iter().map(|r| r.as_ptr()).collect();
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(useResidencySets:count:),
                ptrs.as_ptr(),
                ptrs.len() as UInteger,
            );
        }
    }

    /// Push a debug group.
    ///
    /// C++ equivalent: `void pushDebugGroup(const NS::String* string)`
    pub fn push_debug_group(&self, name: &str) {
        if let Some(ns_name) = metal_foundation::String::from_str(name) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(pushDebugGroup:), ns_name.as_ptr());
            }
        }
    }

    /// Pop the current debug group.
    ///
    /// C++ equivalent: `void popDebugGroup()`
    pub fn pop_debug_group(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(popDebugGroup));
        }
    }

    // =========================================================================
    // Command Encoder Creation
    // =========================================================================

    /// Create a compute command encoder.
    ///
    /// C++ equivalent: `ComputeCommandEncoder* computeCommandEncoder()`
    pub fn compute_command_encoder(&self) -> Option<super::ComputeCommandEncoder> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(computeCommandEncoder));
            if ptr.is_null() {
                None
            } else {
                super::ComputeCommandEncoder::from_raw(ptr)
            }
        }
    }

    /// Create a render command encoder with the specified render pass descriptor.
    ///
    /// C++ equivalent: `RenderCommandEncoder* renderCommandEncoder(const RenderPassDescriptor*)`
    pub fn render_command_encoder(
        &self,
        descriptor: &super::RenderPassDescriptor,
    ) -> Option<super::RenderCommandEncoder> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(renderCommandEncoderWithDescriptor:),
                descriptor.as_ptr(),
            );
            if ptr.is_null() {
                None
            } else {
                super::RenderCommandEncoder::from_raw(ptr)
            }
        }
    }

    /// Create a machine learning command encoder.
    ///
    /// C++ equivalent: `MachineLearningCommandEncoder* machineLearningCommandEncoder()`
    pub fn machine_learning_command_encoder(&self) -> Option<super::MachineLearningCommandEncoder> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(machineLearningCommandEncoder));
            if ptr.is_null() {
                None
            } else {
                super::MachineLearningCommandEncoder::from_raw(ptr)
            }
        }
    }

    // =========================================================================
    // Counter/Timestamp Methods
    // =========================================================================

    /// Resolve counter heap data into a buffer.
    ///
    /// C++ equivalent: `void resolveCounterHeap(const MTL4::CounterHeap*, NS::Range, const MTL4::BufferRange, const MTL::Fence*, const MTL::Fence*)`
    pub fn resolve_counter_heap(
        &self,
        counter_heap: *const c_void,
        range_location: UInteger,
        range_length: UInteger,
        buffer_range: BufferRange,
        fence_to_wait: *const c_void,
        fence_to_update: *const c_void,
    ) {
        unsafe {
            let range = (range_location, range_length);
            let _: () = msg_send_5(
                self.as_ptr(),
                sel!(resolveCounterHeap:withRange:intoBuffer:waitFence:updateFence:),
                counter_heap,
                range,
                buffer_range,
                fence_to_wait,
                fence_to_update,
            );
        }
    }

    /// Write a timestamp into a counter heap.
    ///
    /// C++ equivalent: `void writeTimestampIntoHeap(const MTL4::CounterHeap*, NS::UInteger)`
    pub fn write_timestamp_into_heap(&self, counter_heap: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(writeTimestampIntoHeap:atIndex:),
                counter_heap,
                index,
            );
        }
    }
}

impl Clone for CommandBuffer {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for CommandBuffer {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CommandBuffer {}
unsafe impl Sync for CommandBuffer {}

impl std::fmt::Debug for CommandBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandBuffer")
            .field("label", &self.label())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_buffer_options_size() {
        assert_eq!(
            std::mem::size_of::<CommandBufferOptions>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_command_buffer_size() {
        assert_eq!(
            std::mem::size_of::<CommandBuffer>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
