//! Device properties.
//!
//! Corresponds to property getters in `MTL::Device` from `Metal/MTLDevice.hpp`.

use std::ffi::c_void;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_2, sel};

use super::{Architecture, Device};
use crate::enums::{ArgumentBuffersTier, DeviceLocation, ReadWriteTextureTier};
use crate::types::SamplePosition;

impl Device {
    /// Get the device name.
    ///
    /// C++ equivalent: `NS::String* name() const`
    ///
    /// # Example
    ///
    /// ```ignore
    /// let device = device::system_default().unwrap();
    /// println!("GPU: {}", device.name());
    /// ```
    #[inline]
    pub fn name(&self) -> &str {
        unsafe {
            let ns_string: *mut c_void = msg_send_0(self.as_ptr(), sel!(name));
            if ns_string.is_null() {
                return "";
            }
            let c_str: *const std::ffi::c_char = msg_send_0(ns_string, sel!(UTF8String));
            if c_str.is_null() {
                return "";
            }
            std::ffi::CStr::from_ptr(c_str).to_str().unwrap_or("")
        }
    }

    /// Get the device architecture.
    ///
    /// C++ equivalent: `Architecture* architecture() const`
    #[inline]
    pub fn architecture(&self) -> Option<Architecture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(architecture));
            if ptr.is_null() {
                return None;
            }
            // Property getter returns autoreleased, so retain it
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Architecture::from_raw(ptr)
        }
    }

    /// Get the device registry ID.
    ///
    /// This is a unique identifier for the device that persists across reboots.
    ///
    /// C++ equivalent: `uint64_t registryID() const`
    #[inline]
    pub fn registry_id(&self) -> u64 {
        unsafe { msg_send_0(self.as_ptr(), sel!(registryID)) }
    }

    /// Get the device location.
    ///
    /// C++ equivalent: `DeviceLocation location() const`
    #[inline]
    pub fn location(&self) -> DeviceLocation {
        unsafe { msg_send_0(self.as_ptr(), sel!(location)) }
    }

    /// Get the device location number.
    ///
    /// C++ equivalent: `NS::UInteger locationNumber() const`
    #[inline]
    pub fn location_number(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(locationNumber)) }
    }

    /// Check if the device has unified memory.
    ///
    /// Returns true if the CPU and GPU share the same memory.
    ///
    /// C++ equivalent: `bool hasUnifiedMemory() const`
    #[inline]
    pub fn has_unified_memory(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(hasUnifiedMemory)) }
    }

    /// Check if the device is headless (has no display attached).
    ///
    /// C++ equivalent: `bool isHeadless() const`
    #[inline]
    pub fn is_headless(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isHeadless)) }
    }

    /// Check if the device is low-power.
    ///
    /// C++ equivalent: `bool isLowPower() const`
    #[inline]
    pub fn is_low_power(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isLowPower)) }
    }

    /// Check if the device is removable.
    ///
    /// C++ equivalent: `bool isRemovable() const`
    #[inline]
    pub fn is_removable(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isRemovable)) }
    }

    /// Get the current allocated memory size in bytes.
    ///
    /// C++ equivalent: `NS::UInteger currentAllocatedSize() const`
    #[inline]
    pub fn current_allocated_size(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(currentAllocatedSize)) }
    }

    /// Get the recommended maximum working set size.
    ///
    /// C++ equivalent: `uint64_t recommendedMaxWorkingSetSize() const`
    #[inline]
    pub fn recommended_max_working_set_size(&self) -> u64 {
        unsafe { msg_send_0(self.as_ptr(), sel!(recommendedMaxWorkingSetSize)) }
    }

    /// Get the read-write texture tier supported by this device.
    ///
    /// C++ equivalent: `ReadWriteTextureTier readWriteTextureSupport() const`
    #[inline]
    pub fn read_write_texture_support(&self) -> ReadWriteTextureTier {
        unsafe { msg_send_0(self.as_ptr(), sel!(readWriteTextureSupport)) }
    }

    /// Get the argument buffers tier supported by this device.
    ///
    /// C++ equivalent: `ArgumentBuffersTier argumentBuffersSupport() const`
    #[inline]
    pub fn argument_buffers_support(&self) -> ArgumentBuffersTier {
        unsafe { msg_send_0(self.as_ptr(), sel!(argumentBuffersSupport)) }
    }

    /// Get the peer group ID for multi-GPU configurations.
    ///
    /// C++ equivalent: `uint64_t peerGroupID() const`
    #[inline]
    pub fn peer_group_id(&self) -> u64 {
        unsafe { msg_send_0(self.as_ptr(), sel!(peerGroupID)) }
    }

    /// Get the peer index within a multi-GPU configuration.
    ///
    /// C++ equivalent: `uint32_t peerIndex() const`
    #[inline]
    pub fn peer_index(&self) -> u32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(peerIndex)) }
    }

    /// Get the number of peer devices.
    ///
    /// C++ equivalent: `uint32_t peerCount() const`
    #[inline]
    pub fn peer_count(&self) -> u32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(peerCount)) }
    }

    /// Get the maximum data transfer rate in bytes per second.
    ///
    /// C++ equivalent: `uint64_t maxTransferRate() const`
    #[inline]
    pub fn max_transfer_rate(&self) -> u64 {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxTransferRate)) }
    }

    /// Get whether to maximize concurrent compilation.
    ///
    /// C++ equivalent: `bool shouldMaximizeConcurrentCompilation() const`
    #[inline]
    pub fn should_maximize_concurrent_compilation(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(shouldMaximizeConcurrentCompilation)) }
    }

    /// Set whether to maximize concurrent compilation.
    ///
    /// C++ equivalent: `void setShouldMaximizeConcurrentCompilation(bool)`
    #[inline]
    pub fn set_should_maximize_concurrent_compilation(&self, value: bool) {
        unsafe {
            metal_sys::msg_send_1::<(), bool>(
                self.as_ptr(),
                sel!(setShouldMaximizeConcurrentCompilation:),
                value,
            );
        }
    }

    /// Get the maximum number of concurrent compilation tasks.
    ///
    /// C++ equivalent: `NS::UInteger maximumConcurrentCompilationTaskCount() const`
    #[inline]
    pub fn maximum_concurrent_compilation_task_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maximumConcurrentCompilationTaskCount)) }
    }

    /// Get the GPU timestamp frequency in Hz.
    ///
    /// C++ equivalent: `uint64_t queryTimestampFrequency()`
    #[inline]
    pub fn query_timestamp_frequency(&self) -> u64 {
        unsafe { msg_send_0(self.as_ptr(), sel!(queryTimestampFrequency)) }
    }

    /// Sample CPU and GPU timestamps simultaneously.
    ///
    /// C++ equivalent: `void sampleTimestamps(MTL::Timestamp*, MTL::Timestamp*)`
    #[inline]
    pub fn sample_timestamps(&self) -> (u64, u64) {
        let mut cpu_timestamp: u64 = 0;
        let mut gpu_timestamp: u64 = 0;
        unsafe {
            msg_send_2::<(), *mut u64, *mut u64>(
                self.as_ptr(),
                sel!(sampleTimestamps: gpuTimestamp:),
                &mut cpu_timestamp as *mut u64,
                &mut gpu_timestamp as *mut u64,
            );
        }
        (cpu_timestamp, gpu_timestamp)
    }

    /// Get default sample positions for a given sample count.
    ///
    /// C++ equivalent: `void getDefaultSamplePositions(MTL::SamplePosition*, NS::UInteger)`
    #[inline]
    pub fn get_default_sample_positions(&self, positions: &mut [SamplePosition]) {
        unsafe {
            msg_send_2::<(), *mut SamplePosition, usize>(
                self.as_ptr(),
                sel!(getDefaultSamplePositions: count:),
                positions.as_mut_ptr(),
                positions.len(),
            );
        }
    }

    // =========================================================================
    // Deprecated properties (included for 1:1 API compatibility)
    // =========================================================================

    /// Check if barycentric coordinates are supported.
    ///
    /// C++ equivalent: `bool barycentricCoordsSupported() const`
    #[deprecated(note = "Use are_barycentric_coords_supported() instead")]
    #[inline]
    pub fn barycentric_coords_supported(&self) -> bool {
        self.are_barycentric_coords_supported()
    }

    /// Check if programmable sample positions are supported.
    ///
    /// C++ equivalent: `bool programmableSamplePositionsSupported() const`
    #[deprecated(note = "Use are_programmable_sample_positions_supported() instead")]
    #[inline]
    pub fn programmable_sample_positions_supported(&self) -> bool {
        self.are_programmable_sample_positions_supported()
    }

    /// Check if raster order groups are supported.
    ///
    /// C++ equivalent: `bool rasterOrderGroupsSupported() const`
    #[deprecated(note = "Use are_raster_order_groups_supported() instead")]
    #[inline]
    pub fn raster_order_groups_supported(&self) -> bool {
        self.are_raster_order_groups_supported()
    }

    /// Check if the device is headless.
    ///
    /// C++ equivalent: `bool headless() const`
    #[deprecated(note = "Use is_headless() instead")]
    #[inline]
    pub fn headless(&self) -> bool {
        self.is_headless()
    }

    /// Check if the device is low-power.
    ///
    /// C++ equivalent: `bool lowPower() const`
    #[deprecated(note = "Use is_low_power() instead")]
    #[inline]
    pub fn low_power(&self) -> bool {
        self.is_low_power()
    }

    /// Check if the device is removable.
    ///
    /// C++ equivalent: `bool removable() const`
    #[deprecated(note = "Use is_removable() instead")]
    #[inline]
    pub fn removable(&self) -> bool {
        self.is_removable()
    }

    /// Check if depth24 stencil8 pixel format is supported.
    ///
    /// C++ equivalent: `bool depth24Stencil8PixelFormatSupported() const`
    #[deprecated(note = "Use is_depth24_stencil8_pixel_format_supported() instead")]
    #[inline]
    pub fn depth24_stencil8_pixel_format_supported(&self) -> bool {
        self.is_depth24_stencil8_pixel_format_supported()
    }
}

#[cfg(test)]
mod tests {
    use crate::device::system_default;

    #[test]
    fn test_device_properties() {
        let device = system_default().expect("no Metal device");

        // Name should not be empty
        assert!(!device.name().is_empty());

        // Registry ID should be non-zero
        assert!(device.registry_id() != 0);

        // Architecture should be available
        let arch = device.architecture();
        assert!(arch.is_some());
    }

    #[test]
    fn test_device_timestamps() {
        let device = system_default().expect("no Metal device");

        let (cpu, gpu) = device.sample_timestamps();
        // Both timestamps should be non-zero
        assert!(cpu > 0);
        assert!(gpu > 0);
    }
}
