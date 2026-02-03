//! Residency set-related Device methods.
//!
//! Corresponds to residency set methods in `Metal/MTLDevice.hpp`.

use std::ffi::c_void;

use metal_foundation::Referencing;
use metal_sys::{msg_send_2, sel};

use super::Device;
use crate::residency_set::{ResidencySet, ResidencySetDescriptor};

impl Device {
    /// Create a new residency set.
    ///
    /// C++ equivalent: `ResidencySet* newResidencySet(const MTL::ResidencySetDescriptor*, NS::Error**)`
    pub fn new_residency_set(
        &self,
        descriptor: &ResidencySetDescriptor,
    ) -> Result<ResidencySet, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newResidencySetWithDescriptor:error:),
                descriptor.as_ptr(),
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    return Err(
                        metal_foundation::Error::from_ptr(error).expect("error should be valid")
                    );
                }
                return Err(metal_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error"));
            }

            Ok(ResidencySet::from_raw(ptr).expect("failed to create residency set"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::system_default;

    #[test]
    fn test_new_residency_set() {
        let device = system_default().expect("no Metal device");
        let descriptor = ResidencySetDescriptor::new().expect("failed to create descriptor");

        // Creating a residency set should succeed
        let result = device.new_residency_set(&descriptor);
        assert!(result.is_ok());
    }
}
