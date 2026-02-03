//! Device event and fence creation methods.
//!
//! Corresponds to event/fence creation methods in `Metal/MTLDevice.hpp`.

use std::ffi::c_void;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::sync::{Event, Fence, SharedEvent, SharedEventHandle};
use super::Device;

impl Device {
    // =========================================================================
    // Event Creation
    // =========================================================================

    /// Create a new event.
    ///
    /// C++ equivalent: `Event* newEvent()`
    pub fn new_event(&self) -> Option<Event> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(newEvent));
            Event::from_raw(ptr)
        }
    }

    /// Create a new shared event.
    ///
    /// C++ equivalent: `SharedEvent* newSharedEvent()`
    pub fn new_shared_event(&self) -> Option<SharedEvent> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(newSharedEvent));
            SharedEvent::from_raw(ptr)
        }
    }

    /// Create a shared event from a handle.
    ///
    /// C++ equivalent: `SharedEvent* newSharedEvent(const SharedEventHandle*)`
    pub fn new_shared_event_with_handle(&self, handle: &SharedEventHandle) -> Option<SharedEvent> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newSharedEventWithHandle:),
                handle.as_raw(),
            );
            SharedEvent::from_raw(ptr)
        }
    }

    // =========================================================================
    // Fence Creation
    // =========================================================================

    /// Create a new fence.
    ///
    /// C++ equivalent: `Fence* newFence()`
    pub fn new_fence(&self) -> Option<Fence> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(newFence));
            Fence::from_raw(ptr)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::device::system_default;

    #[test]
    fn test_new_event() {
        let device = system_default().expect("no Metal device");
        let event = device.new_event();
        assert!(event.is_some());

        let event = event.unwrap();
        event.set_label("Test Event");
        assert_eq!(event.label(), Some("Test Event".to_string()));
    }

    #[test]
    fn test_new_shared_event() {
        let device = system_default().expect("no Metal device");
        let event = device.new_shared_event();
        assert!(event.is_some());

        let event = event.unwrap();
        assert_eq!(event.signaled_value(), 0);

        event.set_signaled_value(42);
        assert_eq!(event.signaled_value(), 42);
    }

    #[test]
    fn test_new_fence() {
        let device = system_default().expect("no Metal device");
        let fence = device.new_fence();
        assert!(fence.is_some());

        let fence = fence.unwrap();
        fence.set_label("Test Fence");
        assert_eq!(fence.label(), Some("Test Fence".to_string()));
    }
}
