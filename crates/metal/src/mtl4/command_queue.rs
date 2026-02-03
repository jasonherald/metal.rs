//! MTL4 CommandQueue implementation.
//!
//! Corresponds to `Metal/MTL4CommandQueue.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, msg_send_3, msg_send_4, sel};

use crate::{Device, Drawable, Event, ResidencySet};
use super::{CommandBuffer, CommitFeedback};

/// Dispatch queue type (opaque).
pub type DispatchQueue = *mut c_void;

// ============================================================
// CommitOptions
// ============================================================

/// Options for committing command buffers.
///
/// C++ equivalent: `MTL4::CommitOptions`
#[repr(transparent)]
pub struct CommitOptions(NonNull<c_void>);

impl CommitOptions {
    /// Create a CommitOptions from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create new commit options.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4CommitOptions")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Add a feedback handler to be called when the commit completes.
    ///
    /// C++ equivalent: `void addFeedbackHandler(void (^)(MTL4::CommitFeedback*))`
    ///
    /// The handler is called with the commit feedback when the commit completes.
    pub fn add_feedback_handler<F>(&self, handler: F)
    where
        F: Fn(&CommitFeedback) + Send + 'static,
    {
        let block = metal_sys::OneArgBlock::from_fn(move |feedback_ptr: *mut c_void| {
            unsafe {
                if let Some(feedback) = CommitFeedback::from_raw(feedback_ptr) {
                    handler(&feedback);
                    // Don't drop - Metal owns this reference
                    std::mem::forget(feedback);
                }
            }
        });

        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(addFeedbackHandler:),
                block.as_ptr(),
            );
        }

        // The block is retained by Metal
        std::mem::forget(block);
    }
}

impl Clone for CommitOptions {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CommitOptions {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for CommitOptions {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CommitOptions {}
unsafe impl Sync for CommitOptions {}

// ============================================================
// CommandQueueDescriptor
// ============================================================

/// Descriptor for creating a MTL4 command queue.
///
/// C++ equivalent: `MTL4::CommandQueueDescriptor`
#[repr(transparent)]
pub struct CommandQueueDescriptor(NonNull<c_void>);

impl CommandQueueDescriptor {
    /// Create a CommandQueueDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new command queue descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4CommandQueueDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the label.
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
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the feedback queue.
    ///
    /// C++ equivalent: `dispatch_queue_t feedbackQueue() const`
    pub fn feedback_queue(&self) -> DispatchQueue {
        unsafe { msg_send_0(self.as_ptr(), sel!(feedbackQueue)) }
    }

    /// Set the feedback queue.
    ///
    /// C++ equivalent: `void setFeedbackQueue(const dispatch_queue_t)`
    pub fn set_feedback_queue(&self, queue: DispatchQueue) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setFeedbackQueue:), queue);
        }
    }
}

impl Clone for CommandQueueDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CommandQueueDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for CommandQueueDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CommandQueueDescriptor {}
unsafe impl Sync for CommandQueueDescriptor {}

// ============================================================
// CommandQueue
// ============================================================

/// MTL4 command queue with explicit residency management.
///
/// C++ equivalent: `MTL4::CommandQueue`
///
/// CommandQueue in Metal 4 provides explicit control over residency sets
/// and command buffer submission.
#[repr(transparent)]
pub struct CommandQueue(NonNull<c_void>);

impl CommandQueue {
    /// Create a CommandQueue from a raw pointer.
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

    // ========== Residency Management ==========

    /// Add a residency set.
    ///
    /// C++ equivalent: `void addResidencySet(const MTL::ResidencySet*)`
    pub fn add_residency_set(&self, residency_set: &ResidencySet) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(addResidencySet:),
                residency_set.as_ptr(),
            );
        }
    }

    /// Add multiple residency sets.
    ///
    /// C++ equivalent: `void addResidencySets(const MTL::ResidencySet* const[], NS::UInteger count)`
    pub fn add_residency_sets(&self, residency_sets: &[&ResidencySet]) {
        let ptrs: Vec<*const c_void> = residency_sets.iter().map(|r| r.as_ptr()).collect();
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(addResidencySets:count:),
                ptrs.as_ptr(),
                ptrs.len() as UInteger,
            );
        }
    }

    /// Remove a residency set.
    ///
    /// C++ equivalent: `void removeResidencySet(const MTL::ResidencySet*)`
    pub fn remove_residency_set(&self, residency_set: &ResidencySet) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(removeResidencySet:),
                residency_set.as_ptr(),
            );
        }
    }

    /// Remove multiple residency sets.
    ///
    /// C++ equivalent: `void removeResidencySets(const MTL::ResidencySet* const[], NS::UInteger count)`
    pub fn remove_residency_sets(&self, residency_sets: &[&ResidencySet]) {
        let ptrs: Vec<*const c_void> = residency_sets.iter().map(|r| r.as_ptr()).collect();
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(removeResidencySets:count:),
                ptrs.as_ptr(),
                ptrs.len() as UInteger,
            );
        }
    }

    // ========== Command Buffer Creation ==========

    /// Create a new MTL4 command buffer.
    ///
    /// C++ equivalent: `MTL4::CommandBuffer* newCommandBuffer()`
    ///
    /// The returned command buffer must be used with `begin_command_buffer()`
    /// and `end_command_buffer()` to record commands.
    pub fn new_command_buffer(&self) -> Option<CommandBuffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(newCommandBuffer));
            if ptr.is_null() {
                None
            } else {
                CommandBuffer::from_raw(ptr)
            }
        }
    }

    // ========== Commit ==========

    /// Commit command buffers for execution.
    ///
    /// C++ equivalent: `void commit(const MTL4::CommandBuffer* const[], NS::UInteger count)`
    pub fn commit(&self, command_buffers: &[&CommandBuffer]) {
        let ptrs: Vec<*const c_void> = command_buffers.iter().map(|c| c.as_ptr()).collect();
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(commit:count:),
                ptrs.as_ptr(),
                ptrs.len() as UInteger,
            );
        }
    }

    /// Commit command buffers with options.
    ///
    /// C++ equivalent: `void commit(const MTL4::CommandBuffer* const[], NS::UInteger, const MTL4::CommitOptions*)`
    pub fn commit_with_options(
        &self,
        command_buffers: &[&CommandBuffer],
        options: &CommitOptions,
    ) {
        let ptrs: Vec<*const c_void> = command_buffers.iter().map(|c| c.as_ptr()).collect();
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(commit:count:options:),
                ptrs.as_ptr(),
                ptrs.len() as UInteger,
                options.as_ptr(),
            );
        }
    }

    // ========== Synchronization ==========

    /// Signal an event with a value.
    ///
    /// C++ equivalent: `void signalEvent(const MTL::Event*, uint64_t value)`
    pub fn signal_event(&self, event: &Event, value: u64) {
        unsafe {
            let _: () = msg_send_2(self.as_ptr(), sel!(signalEvent:value:), event.as_ptr(), value);
        }
    }

    /// Wait for an event to reach a value.
    ///
    /// C++ equivalent: `void wait(const MTL::Event*, uint64_t value)`
    pub fn wait_for_event(&self, event: &Event, value: u64) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(waitForEvent:value:),
                event.as_ptr(),
                value,
            );
        }
    }

    /// Signal a drawable for presentation.
    ///
    /// C++ equivalent: `void signalDrawable(const MTL::Drawable*)`
    pub fn signal_drawable(&self, drawable: &Drawable) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(signalDrawable:), drawable.as_ptr());
        }
    }

    /// Wait for a drawable.
    ///
    /// C++ equivalent: `void wait(const MTL::Drawable*)`
    pub fn wait_for_drawable(&self, drawable: &Drawable) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(waitForDrawable:), drawable.as_ptr());
        }
    }

    // ========== Sparse Resource Mapping ==========

    /// Copy buffer mappings from one buffer to another.
    ///
    /// C++ equivalent: `void copyBufferMappingsFromBuffer(const MTL::Buffer*, const MTL::Buffer*, const MTL4::CopySparseBufferMappingOperation*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The operations pointer must be valid and point to `count` operations.
    pub unsafe fn copy_buffer_mappings_from_buffer(
        &self,
        source_buffer: *const c_void,
        destination_buffer: *const c_void,
        operations: *const c_void,
        count: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_4(
                self.as_ptr(),
                sel!(copyBufferMappingsFromBuffer:toBuffer:operations:count:),
                source_buffer,
                destination_buffer,
                operations,
                count,
            );
        }
    }

    /// Copy texture mappings from one texture to another.
    ///
    /// C++ equivalent: `void copyTextureMappingsFromTexture(const MTL::Texture*, const MTL::Texture*, const MTL4::CopySparseTextureMappingOperation*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The operations pointer must be valid and point to `count` operations.
    pub unsafe fn copy_texture_mappings_from_texture(
        &self,
        source_texture: *const c_void,
        destination_texture: *const c_void,
        operations: *const c_void,
        count: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_4(
                self.as_ptr(),
                sel!(copyTextureMappingsFromTexture:toTexture:operations:count:),
                source_texture,
                destination_texture,
                operations,
                count,
            );
        }
    }

    /// Update buffer mappings for a sparse buffer.
    ///
    /// C++ equivalent: `void updateBufferMappings(const MTL::Buffer*, const MTL::Heap*, const MTL4::UpdateSparseBufferMappingOperation*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The operations pointer must be valid and point to `count` operations.
    pub unsafe fn update_buffer_mappings(
        &self,
        buffer: *const c_void,
        heap: *const c_void,
        operations: *const c_void,
        count: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_4(
                self.as_ptr(),
                sel!(updateBufferMappings:heap:operations:count:),
                buffer,
                heap,
                operations,
                count,
            );
        }
    }

    /// Update texture mappings for a sparse texture.
    ///
    /// C++ equivalent: `void updateTextureMappings(const MTL::Texture*, const MTL::Heap*, const MTL4::UpdateSparseTextureMappingOperation*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The operations pointer must be valid and point to `count` operations.
    pub unsafe fn update_texture_mappings(
        &self,
        texture: *const c_void,
        heap: *const c_void,
        operations: *const c_void,
        count: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_4(
                self.as_ptr(),
                sel!(updateTextureMappings:heap:operations:count:),
                texture,
                heap,
                operations,
                count,
            );
        }
    }
}

impl Clone for CommandQueue {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CommandQueue {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for CommandQueue {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CommandQueue {}
unsafe impl Sync for CommandQueue {}

impl std::fmt::Debug for CommandQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandQueue")
            .field("label", &self.label())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_options_size() {
        assert_eq!(
            std::mem::size_of::<CommitOptions>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_command_queue_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<CommandQueueDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_command_queue_size() {
        assert_eq!(
            std::mem::size_of::<CommandQueue>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
