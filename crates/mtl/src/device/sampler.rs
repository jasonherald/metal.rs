//! Device sampler state creation methods.
//!
//! Corresponds to sampler state creation methods in `Metal/MTLDevice.hpp`.

use std::ffi::c_void;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_1, sel};

use super::Device;
use crate::error::ValidationError;
use crate::sampler::{SamplerDescriptor, SamplerState};

impl Device {
    // =========================================================================
    // Sampler State Creation
    // =========================================================================

    /// Create a new sampler state with the specified descriptor.
    ///
    /// C++ equivalent: `SamplerState* newSamplerState(const SamplerDescriptor*)`
    pub fn new_sampler_state(&self, descriptor: &SamplerDescriptor) -> Option<SamplerState> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newSamplerStateWithDescriptor:),
                descriptor.as_ptr(),
            );
            SamplerState::from_raw(ptr)
        }
    }

    /// Create a sampler state with validation.
    ///
    /// This safe method validates the descriptor before calling Metal APIs:
    /// - Ensures LOD min clamp is <= LOD max clamp
    /// - Ensures max anisotropy is >= 1 and a power of 2
    ///
    /// # Example
    ///
    /// ```ignore
    /// let desc = SamplerDescriptor::new().unwrap();
    /// desc.set_min_filter(SamplerMinMagFilter::LINEAR);
    /// desc.set_mag_filter(SamplerMinMagFilter::LINEAR);
    ///
    /// match device.new_sampler_state_validated(&desc) {
    ///     Ok(sampler) => { /* use sampler */ }
    ///     Err(ValidationError::InvalidLodRange { .. }) => { /* handle error */ }
    ///     Err(e) => { /* handle other errors */ }
    /// }
    /// ```
    pub fn new_sampler_state_validated(
        &self,
        descriptor: &SamplerDescriptor,
    ) -> Result<SamplerState, ValidationError> {
        // Validate LOD range
        let min_lod = descriptor.lod_min_clamp();
        let max_lod = descriptor.lod_max_clamp();
        if min_lod > max_lod {
            return Err(ValidationError::InvalidLodRange {
                min: min_lod,
                max: max_lod,
            });
        }

        // Validate anisotropy
        let anisotropy = descriptor.max_anisotropy();
        if anisotropy < 1 || !anisotropy.is_power_of_two() {
            return Err(ValidationError::InvalidAnisotropy(anisotropy));
        }

        // Call existing safe implementation
        self.new_sampler_state(descriptor)
            .ok_or(ValidationError::CreationFailed(None))
    }

    /// Create a new sampler state with a raw descriptor pointer.
    ///
    /// C++ equivalent: `SamplerState* newSamplerState(const SamplerDescriptor*)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn new_sampler_state_with_ptr(
        &self,
        descriptor: *const c_void,
    ) -> Option<SamplerState> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newSamplerStateWithDescriptor:),
                descriptor,
            );
            SamplerState::from_raw(ptr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::system_default;
    use crate::enums::{SamplerMinMagFilter, SamplerMipFilter};

    #[test]
    fn test_new_sampler_state() {
        let device = system_default().expect("no Metal device");
        let descriptor = SamplerDescriptor::new().expect("failed to create descriptor");

        descriptor.set_min_filter(SamplerMinMagFilter::LINEAR);
        descriptor.set_mag_filter(SamplerMinMagFilter::LINEAR);
        descriptor.set_mip_filter(SamplerMipFilter::LINEAR);

        let sampler = device.new_sampler_state(&descriptor);
        assert!(sampler.is_some());
    }
}
