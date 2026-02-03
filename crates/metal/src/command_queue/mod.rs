//! Metal command queue.
//!
//! Corresponds to `Metal/MTLCommandQueue.hpp`.
//!
//! A command queue manages the execution of command buffers on a device.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

// ============================================================================
// CommandQueueDescriptor
// ============================================================================

/// A descriptor for configuring command queue creation.
///
/// C++ equivalent: `MTL::CommandQueueDescriptor`
#[repr(transparent)]
pub struct CommandQueueDescriptor(NonNull<c_void>);

impl CommandQueueDescriptor {
    /// Create a new CommandQueueDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal command queue descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the descriptor.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new command queue descriptor.
    ///
    /// C++ equivalent: `CommandQueueDescriptor::alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::class!(MTLCommandQueueDescriptor);
            let obj: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if obj.is_null() {
                return None;
            }
            let obj: *mut c_void = msg_send_0(obj, sel!(init));
            Self::from_raw(obj)
        }
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the maximum number of command buffers in flight.
    ///
    /// C++ equivalent: `NS::UInteger maxCommandBufferCount() const`
    #[inline]
    pub fn max_command_buffer_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxCommandBufferCount)) }
    }

    /// Set the maximum number of command buffers in flight.
    ///
    /// C++ equivalent: `void setMaxCommandBufferCount(NS::UInteger maxCommandBufferCount)`
    #[inline]
    pub fn set_max_command_buffer_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxCommandBufferCount:), count);
        }
    }

    /// Get the log state for the command queue.
    ///
    /// C++ equivalent: `LogState* logState() const`
    ///
    /// Returns a raw pointer to the log state object.
    #[inline]
    pub fn log_state(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(logState)) }
    }

    /// Set the log state for the command queue.
    ///
    /// C++ equivalent: `void setLogState(const LogState* logState)`
    ///
    /// # Safety
    ///
    /// The log_state pointer must be valid or null.
    #[inline]
    pub unsafe fn set_log_state(&self, log_state: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLogState:), log_state);
        }
    }
}

impl Default for CommandQueueDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create CommandQueueDescriptor")
    }
}

impl Clone for CommandQueueDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CommandQueueDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
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

impl std::fmt::Debug for CommandQueueDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandQueueDescriptor")
            .field("max_command_buffer_count", &self.max_command_buffer_count())
            .finish()
    }
}

// ============================================================================
// CommandQueue
// ============================================================================

/// A queue that organizes the order of command buffer execution.
///
/// C++ equivalent: `MTL::CommandQueue`
///
/// You typically create a command queue early in your app's lifecycle and reuse
/// it throughout the app. Command buffers created from the same queue are
/// guaranteed to execute in the order they are committed.
#[repr(transparent)]
pub struct CommandQueue(pub(crate) NonNull<c_void>);

impl CommandQueue {
    /// Create a CommandQueue from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal command queue object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the command queue.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Command Buffer Creation
    // =========================================================================

    /// Create a new command buffer.
    ///
    /// C++ equivalent: `CommandBuffer* commandBuffer()`
    pub fn command_buffer(&self) -> Option<crate::command_buffer::CommandBuffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(commandBuffer));
            if ptr.is_null() {
                return None;
            }
            // Retain to take ownership (Metal returns autoreleased object)
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::command_buffer::CommandBuffer::from_raw(ptr)
        }
    }

    /// Create a new command buffer with an unretained reference.
    ///
    /// The returned command buffer is autoreleased and will be deallocated unless
    /// you explicitly retain it.
    ///
    /// C++ equivalent: `CommandBuffer* commandBufferWithUnretainedReferences()`
    pub fn command_buffer_with_unretained_references(
        &self,
    ) -> Option<crate::command_buffer::CommandBuffer> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_0(self.as_ptr(), sel!(commandBufferWithUnretainedReferences));
            // Retain to take ownership
            if !ptr.is_null() {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            }
            crate::command_buffer::CommandBuffer::from_raw(ptr)
        }
    }

    /// Create a new command buffer with a descriptor (raw pointer version).
    ///
    /// C++ equivalent: `CommandBuffer* commandBuffer(const CommandBufferDescriptor*)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn command_buffer_with_descriptor_ptr(
        &self,
        descriptor: *const c_void,
    ) -> Option<crate::command_buffer::CommandBuffer> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(commandBufferWithDescriptor:), descriptor);
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::command_buffer::CommandBuffer::from_raw(ptr)
        }
    }

    /// Create a new command buffer with a typed descriptor.
    ///
    /// C++ equivalent: `CommandBuffer* commandBuffer(const CommandBufferDescriptor*)`
    pub fn command_buffer_with_descriptor(
        &self,
        descriptor: &crate::command_buffer::CommandBufferDescriptor,
    ) -> Option<crate::command_buffer::CommandBuffer> {
        unsafe { self.command_buffer_with_descriptor_ptr(descriptor.as_ptr()) }
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the label for this command queue.
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

    /// Set the label for this command queue.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the device that created this command queue.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("command queue has no device")
        }
    }

    // =========================================================================
    // Residency (macOS only)
    // =========================================================================

    /// Insert a debug capture boundary.
    ///
    /// C++ equivalent: `void insertDebugCaptureBoundary()`
    #[inline]
    pub fn insert_debug_capture_boundary(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(insertDebugCaptureBoundary));
        }
    }

    // =========================================================================
    // Residency Sets
    // =========================================================================

    /// Add a residency set to the command queue.
    ///
    /// C++ equivalent: `void addResidencySet(const ResidencySet*)`
    ///
    /// # Safety
    ///
    /// The residency_set pointer must be valid.
    pub unsafe fn add_residency_set(&self, residency_set: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(addResidencySet:), residency_set);
        }
    }

    /// Add multiple residency sets to the command queue.
    ///
    /// C++ equivalent: `void addResidencySets(const ResidencySet* const*, NS::UInteger count)`
    ///
    /// # Safety
    ///
    /// The residency_sets pointer must be valid and point to count valid pointers.
    pub unsafe fn add_residency_sets(&self, residency_sets: *const *const c_void, count: UInteger) {
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, UInteger>(
                self.as_ptr(),
                sel!(addResidencySets: count:),
                residency_sets,
                count,
            );
        }
    }

    /// Remove a residency set from the command queue.
    ///
    /// C++ equivalent: `void removeResidencySet(const ResidencySet*)`
    ///
    /// # Safety
    ///
    /// The residency_set pointer must be valid.
    pub unsafe fn remove_residency_set(&self, residency_set: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(removeResidencySet:),
                residency_set,
            );
        }
    }

    /// Remove multiple residency sets from the command queue.
    ///
    /// C++ equivalent: `void removeResidencySets(const ResidencySet* const*, NS::UInteger count)`
    ///
    /// # Safety
    ///
    /// The residency_sets pointer must be valid and point to count valid pointers.
    pub unsafe fn remove_residency_sets(
        &self,
        residency_sets: *const *const c_void,
        count: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, UInteger>(
                self.as_ptr(),
                sel!(removeResidencySets: count:),
                residency_sets,
                count,
            );
        }
    }

    // =========================================================================
    // Wait Methods
    // =========================================================================

    /// Wait for an event to reach a specific value.
    ///
    /// C++ equivalent: `void wait(const MTL::Event* event, uint64_t value)`
    pub fn wait_for_event(&self, event: &crate::Event, value: u64) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, u64>(
                self.as_ptr(),
                sel!(waitForEvent:value:),
                event.as_ptr(),
                value,
            );
        }
    }

    /// Wait for a drawable to become available.
    ///
    /// C++ equivalent: `void wait(const MTL::Drawable* drawable)`
    ///
    /// # Safety
    ///
    /// The drawable pointer must be valid.
    pub unsafe fn wait_for_drawable(&self, drawable: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(waitForDrawable:), drawable);
        }
    }
}

impl Clone for CommandQueue {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CommandQueue {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
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
    fn test_command_queue_size() {
        assert_eq!(
            std::mem::size_of::<CommandQueue>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
