//! Device acceleration structure creation methods.
//!
//! Corresponds to acceleration structure methods in `Metal/MTLDevice.hpp`.

use std::ffi::c_void;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_1, sel};

use super::Device;
use crate::acceleration::{AccelerationStructure, AccelerationStructureSizes};

impl Device {
    // =========================================================================
    // Acceleration Structure Creation
    // =========================================================================

    /// Create an acceleration structure with the given descriptor.
    ///
    /// C++ equivalent: `AccelerationStructure* newAccelerationStructure(const AccelerationStructureDescriptor*)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn new_acceleration_structure_with_descriptor(
        &self,
        descriptor: *const c_void,
    ) -> Option<AccelerationStructure> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newAccelerationStructureWithDescriptor:),
                descriptor,
            );
            AccelerationStructure::from_raw(ptr)
        }
    }

    /// Create an acceleration structure with the given size.
    ///
    /// C++ equivalent: `AccelerationStructure* newAccelerationStructure(NS::UInteger size)`
    pub fn new_acceleration_structure_with_size(
        &self,
        size: UInteger,
    ) -> Option<AccelerationStructure> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(newAccelerationStructureWithSize:), size);
            AccelerationStructure::from_raw(ptr)
        }
    }

    // =========================================================================
    // Acceleration Structure Size Queries
    // =========================================================================

    /// Get the sizes needed for building an acceleration structure.
    ///
    /// C++ equivalent: `AccelerationStructureSizes accelerationStructureSizes(const AccelerationStructureDescriptor*)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn acceleration_structure_sizes_with_descriptor(
        &self,
        descriptor: *const c_void,
    ) -> AccelerationStructureSizes {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(accelerationStructureSizesWithDescriptor:),
                descriptor,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::acceleration::PrimitiveAccelerationStructureDescriptor;
    use crate::device::system_default;

    #[test]
    fn test_supports_raytracing() {
        let device = system_default().expect("no Metal device");
        // Just verify we can call the method - result depends on hardware
        let _supports = device.supports_raytracing();
    }

    #[test]
    fn test_acceleration_structure_sizes() {
        let device = system_default().expect("no Metal device");

        if !device.supports_raytracing() {
            println!("Skipping test - device does not support ray tracing");
            return;
        }

        let desc =
            PrimitiveAccelerationStructureDescriptor::new().expect("failed to create descriptor");

        unsafe {
            let sizes = device.acceleration_structure_sizes_with_descriptor(desc.as_raw());
            // Empty descriptor should have 0 sizes
            println!(
                "Acceleration structure size: {}",
                sizes.acceleration_structure_size
            );
            println!(
                "Build scratch buffer size: {}",
                sizes.build_scratch_buffer_size
            );
            println!(
                "Refit scratch buffer size: {}",
                sizes.refit_scratch_buffer_size
            );
        }
    }

    #[test]
    fn test_new_acceleration_structure_with_size() {
        let device = system_default().expect("no Metal device");

        if !device.supports_raytracing() {
            println!("Skipping test - device does not support ray tracing");
            return;
        }

        // Create a small acceleration structure
        let accel = device.new_acceleration_structure_with_size(1024);
        assert!(accel.is_some());

        let accel = accel.unwrap();
        assert!(accel.size() >= 1024);
    }
}
