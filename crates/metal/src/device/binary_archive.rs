//! Binary archive-related Device methods.
//!
//! Corresponds to binary archive methods in `Metal/MTLDevice.hpp`.

use std::ffi::c_void;

use metal_foundation::Referencing;
use metal_sys::{msg_send_2, sel};

use super::Device;
use crate::binary_archive::{BinaryArchive, BinaryArchiveDescriptor};

impl Device {
    /// Create a new binary archive.
    ///
    /// C++ equivalent: `BinaryArchive* newBinaryArchive(const MTL::BinaryArchiveDescriptor*, NS::Error**)`
    pub fn new_binary_archive(
        &self,
        descriptor: &BinaryArchiveDescriptor,
    ) -> Result<BinaryArchive, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newBinaryArchiveWithDescriptor:error:),
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

            Ok(BinaryArchive::from_raw(ptr).expect("failed to create binary archive"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::system_default;

    #[test]
    fn test_new_binary_archive() {
        let device = system_default().expect("no Metal device");
        let descriptor = BinaryArchiveDescriptor::new().expect("failed to create descriptor");

        // Creating an empty binary archive should succeed
        let result = device.new_binary_archive(&descriptor);
        assert!(result.is_ok());
    }
}
