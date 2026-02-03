//! Device creation and enumeration.
//!
//! Corresponds to the free functions in `Metal/MTLDevice.hpp`:
//! - `MTLCreateSystemDefaultDevice()`
//! - `MTLCopyAllDevices()` (macOS only)
//! - `MTLCopyAllDevicesWithObserver()` (macOS only)
//! - `MTLRemoveDeviceObserver()` (macOS only)

use std::ffi::c_void;

use mtl_foundation::Referencing;
use mtl_sys::{DeviceObserver as SysDeviceObserver, create_system_default_device};

#[cfg(target_os = "macos")]
use mtl_sys::copy_all_devices as sys_copy_all_devices;

use super::Device;

/// Timestamp type for CPU/GPU timestamp queries.
///
/// C++ equivalent: `using Timestamp = std::uint64_t`
pub type Timestamp = u64;

/// Create the system default Metal device.
///
/// Returns the default GPU for this system. On systems with multiple GPUs,
/// this typically returns the most capable GPU.
///
/// C++ equivalent: `MTL::Device* MTL::CreateSystemDefaultDevice()`
///
/// # Example
///
/// ```ignore
/// use mtl::device;
///
/// let device = device::system_default().expect("no Metal device available");
/// println!("GPU: {}", device.name());
/// ```
#[inline]
pub fn system_default() -> Option<Device> {
    unsafe {
        let ptr = create_system_default_device()?;
        // The returned pointer is autoreleased, so we need to retain it
        let _: *mut c_void = mtl_sys::msg_send_0(ptr, mtl_sys::sel!(retain));
        Device::from_raw(ptr)
    }
}

/// Copy all available Metal devices (macOS only).
///
/// Returns an array of all GPUs available on this system.
///
/// C++ equivalent: `NS::Array* MTL::CopyAllDevices()`
///
/// # Example
///
/// ```ignore
/// use mtl::device;
///
/// let devices = device::copy_all_devices();
/// for device in devices {
///     println!("Found GPU: {}", device.name());
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn copy_all_devices() -> Vec<Device> {
    unsafe {
        let array_ptr = match sys_copy_all_devices() {
            Some(ptr) => ptr,
            None => return Vec::new(),
        };

        // Get the count
        let count: usize = mtl_sys::msg_send_0(array_ptr, mtl_sys::sel!(count));

        let mut devices = Vec::with_capacity(count);
        for i in 0..count {
            let device_ptr: *mut c_void =
                mtl_sys::msg_send_1(array_ptr, mtl_sys::sel!(objectAtIndex:), i);
            if let Some(device) = Device::from_raw(device_ptr) {
                // Retain since objectAtIndex: returns an autoreleased reference
                device.retain();
                devices.push(device);
            }
        }

        // Release the array (CopyAllDevices transfers ownership)
        mtl_sys::msg_send_0::<()>(array_ptr, mtl_sys::sel!(release));

        devices
    }
}

/// Device observer handle.
///
/// Used to receive notifications about device addition/removal.
/// Call [`remove_device_observer`] when done observing.
#[cfg(target_os = "macos")]
pub struct DeviceObserver(SysDeviceObserver);

#[cfg(target_os = "macos")]
impl DeviceObserver {
    /// Create from the raw observer handle.
    ///
    /// # Safety
    ///
    /// The handle must be a valid device observer.
    pub unsafe fn from_raw(observer: SysDeviceObserver) -> Self {
        Self(observer)
    }

    /// Get the raw observer handle.
    pub fn as_raw(&self) -> SysDeviceObserver {
        self.0
    }
}

/// Copy all devices with an observer for hot-plug notifications (macOS only).
///
/// This function returns all available devices and sets up an observer
/// that will be notified when devices are added or removed.
///
/// C++ equivalent: `NS::Array* MTL::CopyAllDevicesWithObserver(NS::Object**, handler)`
///
/// # Example
///
/// ```ignore
/// use mtl::device;
///
/// let (devices, observer) = device::copy_all_devices_with_observer(|device, notification| {
///     println!("Device event: {:?}", notification);
/// });
///
/// // ... use devices ...
///
/// // When done observing:
/// device::remove_device_observer(observer);
/// ```
#[cfg(target_os = "macos")]
pub fn copy_all_devices_with_observer<F>(handler: F) -> (Vec<Device>, DeviceObserver)
where
    F: Fn(&Device, DeviceNotificationName) + Send + 'static,
{
    use mtl_sys::MTLCopyAllDevicesWithObserver;

    // Create the block that wraps the handler
    let block = mtl_sys::TwoArgBlock::from_fn(
        move |device_ptr: *mut c_void, notification_name_ptr: *mut c_void| {
            unsafe {
                if let Some(device) = Device::from_raw(device_ptr) {
                    // Parse the notification name from the NSString
                    let notification = parse_notification_name(notification_name_ptr);
                    handler(&device, notification);
                    // Don't drop - Metal owns this reference
                    std::mem::forget(device);
                }
            }
        },
    );

    let mut observer: SysDeviceObserver = std::ptr::null_mut();

    let array_ptr =
        unsafe { MTLCopyAllDevicesWithObserver(&mut observer as *mut _, block.as_ptr()) };

    // Transfer block ownership to Metal
    std::mem::forget(block);

    // Parse the devices array
    let devices = if array_ptr.is_null() {
        Vec::new()
    } else {
        unsafe {
            let count: usize = mtl_sys::msg_send_0(array_ptr, mtl_sys::sel!(count));
            let mut devices = Vec::with_capacity(count);
            for i in 0..count {
                let device_ptr: *mut c_void =
                    mtl_sys::msg_send_1(array_ptr, mtl_sys::sel!(objectAtIndex:), i);
                if let Some(device) = Device::from_raw(device_ptr) {
                    // Retain since objectAtIndex: returns an autoreleased reference
                    device.retain();
                    devices.push(device);
                }
            }
            // Release the array (CopyAllDevicesWithObserver transfers ownership)
            mtl_sys::msg_send_0::<()>(array_ptr, mtl_sys::sel!(release));
            devices
        }
    };

    (devices, unsafe { DeviceObserver::from_raw(observer) })
}

/// Parse a notification name NSString into our enum.
#[cfg(target_os = "macos")]
fn parse_notification_name(ns_string: *mut c_void) -> DeviceNotificationName {
    if ns_string.is_null() {
        return DeviceNotificationName::WasRemoved; // Default fallback
    }

    unsafe {
        let c_str: *const i8 = mtl_sys::msg_send_0(ns_string, mtl_sys::sel!(UTF8String));
        if c_str.is_null() {
            return DeviceNotificationName::WasRemoved;
        }

        let name = std::ffi::CStr::from_ptr(c_str).to_string_lossy();

        // These are the standard Metal notification names
        if name.contains("WasAdded") {
            DeviceNotificationName::WasAdded
        } else if name.contains("RemovalRequested") {
            DeviceNotificationName::RemovalRequested
        } else if name.contains("WasRemoved") {
            DeviceNotificationName::WasRemoved
        } else {
            // Unknown notification, default to WasRemoved
            DeviceNotificationName::WasRemoved
        }
    }
}

/// Device notification name.
#[cfg(target_os = "macos")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceNotificationName {
    /// A device was added to the system.
    WasAdded,
    /// A device removal was requested.
    RemovalRequested,
    /// A device was removed from the system.
    WasRemoved,
}

/// Remove a device observer (macOS only).
///
/// Call this function when you no longer need to receive device notifications.
///
/// C++ equivalent: `void MTL::RemoveDeviceObserver(const NS::Object* pObserver)`
#[cfg(target_os = "macos")]
pub fn remove_device_observer(observer: DeviceObserver) {
    use mtl_sys::MTLRemoveDeviceObserver;
    unsafe {
        MTLRemoveDeviceObserver(observer.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_default() {
        let device = system_default();
        assert!(device.is_some(), "should have at least one Metal device");

        let device = device.unwrap();
        assert!(!device.name().is_empty(), "device should have a name");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_copy_all_devices() {
        let devices = copy_all_devices();
        assert!(!devices.is_empty(), "should have at least one device");
    }
}
