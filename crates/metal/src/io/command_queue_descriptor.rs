//! IO command queue descriptor for Metal.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{IOCommandQueueType, IOPriority};

/// Descriptor for creating an IO command queue.
///
/// C++ equivalent: `MTL::IOCommandQueueDescriptor`
#[repr(transparent)]
pub struct IOCommandQueueDescriptor(pub(crate) NonNull<c_void>);

impl IOCommandQueueDescriptor {
    /// Create a new IO command queue descriptor.
    ///
    /// C++ equivalent: `static IOCommandQueueDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLIOCommandQueueDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal IO command queue descriptor.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the maximum command buffer count.
    ///
    /// C++ equivalent: `NS::UInteger maxCommandBufferCount() const`
    #[inline]
    pub fn max_command_buffer_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxCommandBufferCount)) }
    }

    /// Set the maximum command buffer count.
    ///
    /// C++ equivalent: `void setMaxCommandBufferCount(NS::UInteger)`
    #[inline]
    pub fn set_max_command_buffer_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxCommandBufferCount:), count);
        }
    }

    /// Get the maximum commands in flight.
    ///
    /// C++ equivalent: `NS::UInteger maxCommandsInFlight() const`
    #[inline]
    pub fn max_commands_in_flight(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxCommandsInFlight)) }
    }

    /// Set the maximum commands in flight.
    ///
    /// C++ equivalent: `void setMaxCommandsInFlight(NS::UInteger)`
    #[inline]
    pub fn set_max_commands_in_flight(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxCommandsInFlight:), count);
        }
    }

    /// Get the priority.
    ///
    /// C++ equivalent: `IOPriority priority() const`
    #[inline]
    pub fn priority(&self) -> IOPriority {
        unsafe { msg_send_0(self.as_ptr(), sel!(priority)) }
    }

    /// Set the priority.
    ///
    /// C++ equivalent: `void setPriority(IOPriority)`
    #[inline]
    pub fn set_priority(&self, priority: IOPriority) {
        unsafe {
            msg_send_1::<(), IOPriority>(self.as_ptr(), sel!(setPriority:), priority);
        }
    }

    /// Get the queue type.
    ///
    /// C++ equivalent: `IOCommandQueueType type() const`
    #[inline]
    pub fn queue_type(&self) -> IOCommandQueueType {
        unsafe { msg_send_0(self.as_ptr(), sel!(type)) }
    }

    /// Set the queue type.
    ///
    /// C++ equivalent: `void setType(IOCommandQueueType)`
    #[inline]
    pub fn set_queue_type(&self, queue_type: IOCommandQueueType) {
        unsafe {
            msg_send_1::<(), IOCommandQueueType>(self.as_ptr(), sel!(setType:), queue_type);
        }
    }

    /// Get the scratch buffer allocator as a raw pointer.
    ///
    /// C++ equivalent: `IOScratchBufferAllocator* scratchBufferAllocator() const`
    #[inline]
    pub fn scratch_buffer_allocator_ptr(&self) -> *const c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(scratchBufferAllocator)) }
    }

    /// Set the scratch buffer allocator from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid IOScratchBufferAllocator or null.
    ///
    /// C++ equivalent: `void setScratchBufferAllocator(const IOScratchBufferAllocator*)`
    pub unsafe fn set_scratch_buffer_allocator_ptr(&self, allocator: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setScratchBufferAllocator:),
                allocator,
            );
        }
    }
}

impl Default for IOCommandQueueDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create IO command queue descriptor")
    }
}

impl Clone for IOCommandQueueDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy IO command queue descriptor")
        }
    }
}

impl Drop for IOCommandQueueDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for IOCommandQueueDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for IOCommandQueueDescriptor {}
unsafe impl Sync for IOCommandQueueDescriptor {}
