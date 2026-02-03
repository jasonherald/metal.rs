//! Device command queue creation methods.
//!
//! Corresponds to command queue creation methods in `Metal/MTLDevice.hpp`.

use std::ffi::c_void;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use super::Device;
use crate::command_queue::CommandQueue;

impl Device {
    // =========================================================================
    // Command Queue Creation
    // =========================================================================

    /// Create a new command queue.
    ///
    /// C++ equivalent: `CommandQueue* newCommandQueue()`
    pub fn new_command_queue(&self) -> Option<CommandQueue> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(newCommandQueue));
            CommandQueue::from_raw(ptr)
        }
    }

    /// Create a new command queue with a maximum command buffer count.
    ///
    /// C++ equivalent: `CommandQueue* newCommandQueue(NS::UInteger maxCommandBufferCount)`
    pub fn new_command_queue_with_max_command_buffer_count(
        &self,
        max_command_buffer_count: UInteger,
    ) -> Option<CommandQueue> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newCommandQueueWithMaxCommandBufferCount:),
                max_command_buffer_count,
            );
            CommandQueue::from_raw(ptr)
        }
    }

    /// Create a new command queue with a descriptor.
    ///
    /// C++ equivalent: `CommandQueue* newCommandQueue(const CommandQueueDescriptor*)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn new_command_queue_with_descriptor(
        &self,
        descriptor: *const c_void,
    ) -> Option<CommandQueue> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newCommandQueueWithDescriptor:),
                descriptor,
            );
            CommandQueue::from_raw(ptr)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::device::system_default;

    #[test]
    fn test_new_command_queue() {
        let device = system_default().expect("no Metal device");
        let queue = device.new_command_queue();
        assert!(queue.is_some());
    }

    #[test]
    fn test_new_command_queue_with_max() {
        let device = system_default().expect("no Metal device");
        let queue = device.new_command_queue_with_max_command_buffer_count(64);
        assert!(queue.is_some());
    }

    #[test]
    fn test_command_buffer_creation() {
        let device = system_default().expect("no Metal device");
        let queue = device.new_command_queue().expect("failed to create queue");
        let cmd_buf = queue.command_buffer();
        assert!(cmd_buf.is_some());
    }
}
