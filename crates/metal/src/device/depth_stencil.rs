//! Device depth/stencil state creation methods.
//!
//! Corresponds to depth/stencil state creation methods in `Metal/MTLDevice.hpp`.

use std::ffi::c_void;

use metal_foundation::Referencing;
use metal_sys::{msg_send_1, sel};

use crate::depth_stencil::{DepthStencilDescriptor, DepthStencilState};
use super::Device;

impl Device {
    // =========================================================================
    // Depth/Stencil State Creation
    // =========================================================================

    /// Create a new depth/stencil state with the specified descriptor.
    ///
    /// C++ equivalent: `DepthStencilState* newDepthStencilState(const DepthStencilDescriptor*)`
    pub fn new_depth_stencil_state(
        &self,
        descriptor: &DepthStencilDescriptor,
    ) -> Option<DepthStencilState> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newDepthStencilStateWithDescriptor:),
                descriptor.as_ptr(),
            );
            DepthStencilState::from_raw(ptr)
        }
    }

    /// Create a new depth/stencil state with a raw descriptor pointer.
    ///
    /// C++ equivalent: `DepthStencilState* newDepthStencilState(const DepthStencilDescriptor*)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn new_depth_stencil_state_with_ptr(
        &self,
        descriptor: *const c_void,
    ) -> Option<DepthStencilState> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newDepthStencilStateWithDescriptor:),
                descriptor,
            );
            DepthStencilState::from_raw(ptr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::system_default;
    use crate::enums::CompareFunction;

    #[test]
    fn test_new_depth_stencil_state() {
        let device = system_default().expect("no Metal device");
        let descriptor = DepthStencilDescriptor::new().expect("failed to create descriptor");

        descriptor.set_depth_compare_function(CompareFunction::LESS);
        descriptor.set_depth_write_enabled(true);

        let state = device.new_depth_stencil_state(&descriptor);
        assert!(state.is_some());
    }
}
