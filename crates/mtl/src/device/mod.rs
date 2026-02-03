//! Metal device interface.
//!
//! Corresponds to `Metal/MTLDevice.hpp`.
//!
//! The `Device` represents a GPU that can execute Metal commands. Use the
//! [`system_default`] function to get the default GPU, or [`copy_all_devices`]
//! (macOS only) to enumerate all available GPUs.
//!
//! # Example
//!
//! ```ignore
//! use mtl_gpu::device;
//!
//! // Get the default GPU
//! let device = device::system_default().expect("no Metal device");
//! println!("Using GPU: {}", device.name());
//! ```

mod architecture;
mod creation;
mod features;
mod limits;
mod properties;

// Resource creation modules
mod acceleration;
mod binary_archive;
mod buffer;
mod command_queue;
mod depth_stencil;
mod events;
mod heap;
mod indirect;
mod io;
mod library;
mod mtl4;
mod pipeline;
mod residency_set;
mod sampler;
mod texture;

// Re-export creation functions at module level
pub use creation::{Timestamp, system_default};

#[cfg(target_os = "macos")]
pub use creation::{
    DeviceObserver, copy_all_devices, copy_all_devices_with_observer, remove_device_observer,
};

// Re-export Architecture
pub use architecture::Architecture;

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::Referencing;

/// A Metal device representing a GPU.
///
/// C++ equivalent: `MTL::Device`
///
/// The device is the central object in Metal. It is used to create all other
/// Metal objects including command queues, buffers, textures, and pipelines.
#[repr(transparent)]
pub struct Device(pub(crate) NonNull<c_void>);

impl Device {
    /// Create a Device from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal device object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the device.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }
}

impl Clone for Device {
    fn clone(&self) -> Self {
        // Retain the underlying object
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        // Release the underlying object
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for Device {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

// SAFETY: Device is a reference-counted object that is thread-safe
unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("name", &self.name())
            .field("registry_id", &self.registry_id())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_size() {
        // Device should be pointer-sized
        assert_eq!(
            std::mem::size_of::<Device>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
