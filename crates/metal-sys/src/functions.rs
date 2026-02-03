//! Metal C function exports.
//!
//! These are standalone C functions exported by the Metal framework,
//! not Objective-C methods. They must be linked directly.
//!
//! # C++ Equivalent
//!
//! From metal-cpp MTLDevice.hpp:
//! ```cpp
//! Device*    CreateSystemDefaultDevice();
//! NS::Array* CopyAllDevices();
//! NS::Array* CopyAllDevicesWithObserver(NS::Object** pOutObserver, MTL::DeviceNotificationHandlerBlock handler);
//! void       RemoveDeviceObserver(const NS::Object* pObserver);
//! ```
//!
//! # Device Creation
//!
//! - `MTLCreateSystemDefaultDevice()` - Creates the system default Metal device
//! - `MTLCopyAllDevices()` - Copies all available Metal devices (macOS only)
//! - `MTLCopyAllDevicesWithObserver()` - Copies devices with hot-plug notifications
//! - `MTLRemoveDeviceObserver()` - Removes a device observer
//!
//! # IO Compression
//!
//! - `MTLIOCompressionContextDefaultChunkSize()` - Default chunk size for compression
//! - `MTLIOCreateCompressionContext()` - Creates a compression context
//! - `MTLIOCompressionContextAppendData()` - Appends data to compression context
//! - `MTLIOFlushAndDestroyCompressionContext()` - Flushes and destroys context

use std::ffi::c_void;

// =============================================================================
// Device Creation Functions
// =============================================================================

/// Opaque type for device observers (used with hot-plug notifications).
pub type DeviceObserver = *mut c_void;

// Link against the Metal framework for device creation functions
#[link(name = "Metal", kind = "framework")]
unsafe extern "C" {
    /// Create the system default Metal device.
    ///
    /// Returns a pointer to the default `MTLDevice`, or null if no Metal
    /// device is available. The returned object is autoreleased.
    ///
    /// # C++ Equivalent
    ///
    /// ```cpp
    /// MTL::Device* MTL::CreateSystemDefaultDevice();
    /// ```
    ///
    /// # Returns
    ///
    /// A pointer to the default Metal device, or null.
    pub fn MTLCreateSystemDefaultDevice() -> *mut c_void;

    /// Copy all available Metal devices.
    ///
    /// Returns an `NSArray` containing all available Metal devices.
    /// This function is only available on macOS.
    ///
    /// # C++ Equivalent
    ///
    /// ```cpp
    /// NS::Array* MTL::CopyAllDevices();
    /// ```
    ///
    /// # Returns
    ///
    /// A pointer to an `NSArray` of Metal devices. The caller owns the array.
    #[cfg(target_os = "macos")]
    pub fn MTLCopyAllDevices() -> *mut c_void;

    /// Copy all devices with a notification observer.
    ///
    /// Returns an `NSArray` containing all available Metal devices and
    /// sets up a notification observer for device hot-plug events.
    ///
    /// # C++ Equivalent
    ///
    /// ```cpp
    /// NS::Array* MTL::CopyAllDevicesWithObserver(
    ///     NS::Object** pOutObserver,
    ///     MTL::DeviceNotificationHandlerBlock handler
    /// );
    /// ```
    ///
    /// # Arguments
    ///
    /// * `observer` - Output parameter that receives the observer reference
    /// * `handler` - Block called when device changes occur
    ///
    /// # Returns
    ///
    /// A pointer to an `NSArray` of Metal devices. The caller owns the array.
    #[cfg(target_os = "macos")]
    pub fn MTLCopyAllDevicesWithObserver(
        observer: *mut DeviceObserver,
        handler: *const c_void, // Block: void(^)(MTLDevice*, MTLDeviceNotificationName)
    ) -> *mut c_void;

    /// Remove a device observer.
    ///
    /// Stops receiving device hot-plug notifications.
    ///
    /// # C++ Equivalent
    ///
    /// ```cpp
    /// void MTL::RemoveDeviceObserver(const NS::Object* pObserver);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `observer` - The observer to remove
    #[cfg(target_os = "macos")]
    pub fn MTLRemoveDeviceObserver(observer: DeviceObserver);
}

// =============================================================================
// IO Compression Types
// =============================================================================

/// IO compression method.
///
/// # C++ Equivalent
///
/// ```cpp
/// _MTL_ENUM(NS::Integer, IOCompressionMethod) {
///     IOCompressionMethodZlib = 0,
///     IOCompressionMethodLZFSE = 1,
///     IOCompressionMethodLZ4 = 2,
///     IOCompressionMethodLZMA = 3,
///     IOCompressionMethodLZBitmap = 4,
/// };
/// ```
#[repr(i64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IOCompressionMethod {
    /// Zlib compression.
    Zlib = 0,
    /// LZFSE compression (Apple's fast compression).
    LZFSE = 1,
    /// LZ4 compression (very fast).
    LZ4 = 2,
    /// LZMA compression (high ratio).
    LZMA = 3,
    /// LZ Bitstream compression.
    LZBitmap = 4,
}

/// IO compression status.
#[repr(i64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IOCompressionStatus {
    /// Compression completed successfully.
    Complete = 0,
    /// Compression encountered an error.
    Error = -1,
}

/// Opaque compression context handle.
pub type IOCompressionContext = *mut c_void;

// =============================================================================
// IO Compression Functions
// =============================================================================

#[link(name = "Metal", kind = "framework")]
unsafe extern "C" {
    /// Get the default chunk size for IO compression.
    ///
    /// # Returns
    ///
    /// The recommended chunk size in bytes.
    pub fn MTLIOCompressionContextDefaultChunkSize() -> usize;

    /// Create an IO compression context.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the output file (C string)
    /// * `compression_method` - The compression method to use
    /// * `chunk_size` - The chunk size for compression
    ///
    /// # Returns
    ///
    /// A compression context handle, or null on failure.
    pub fn MTLIOCreateCompressionContext(
        path: *const std::ffi::c_char,
        compression_method: IOCompressionMethod,
        chunk_size: usize,
    ) -> IOCompressionContext;

    /// Append data to a compression context.
    ///
    /// # Arguments
    ///
    /// * `context` - The compression context
    /// * `data` - Pointer to the data to compress
    /// * `size` - Size of the data in bytes
    pub fn MTLIOCompressionContextAppendData(
        context: IOCompressionContext,
        data: *const c_void,
        size: usize,
    );

    /// Flush and destroy a compression context.
    ///
    /// Flushes any remaining data, writes the compressed output, and
    /// destroys the context.
    ///
    /// # Arguments
    ///
    /// * `context` - The compression context to finalize
    ///
    /// # Returns
    ///
    /// The compression status.
    pub fn MTLIOFlushAndDestroyCompressionContext(
        context: IOCompressionContext,
    ) -> IOCompressionStatus;
}

// =============================================================================
// Safe Wrapper Functions
// =============================================================================

/// Safely create the system default Metal device.
///
/// # Returns
///
/// The raw pointer to the default device, or `None` if unavailable.
///
/// # Safety
///
/// The returned pointer is autoreleased. Callers should retain it if
/// they need to keep it beyond the current autorelease pool scope.
#[inline]
pub fn create_system_default_device() -> Option<*mut c_void> {
    let ptr = unsafe { MTLCreateSystemDefaultDevice() };
    if ptr.is_null() { None } else { Some(ptr) }
}

/// Safely copy all available Metal devices (macOS only).
///
/// # Returns
///
/// The raw pointer to an NSArray of devices, or `None` if unavailable.
///
/// # Safety
///
/// The caller owns the returned array and must release it.
#[cfg(target_os = "macos")]
#[inline]
pub fn copy_all_devices() -> Option<*mut c_void> {
    let ptr = unsafe { MTLCopyAllDevices() };
    if ptr.is_null() { None } else { Some(ptr) }
}

/// Get the default IO compression chunk size.
#[inline]
pub fn io_compression_default_chunk_size() -> usize {
    unsafe { MTLIOCompressionContextDefaultChunkSize() }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_system_default_device() {
        // This may return None in headless environments
        let device = create_system_default_device();
        // Just check it doesn't crash
        let _ = device;
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_copy_all_devices() {
        let devices = copy_all_devices();
        // Just check it doesn't crash
        let _ = devices;
    }

    #[test]
    fn test_compression_chunk_size() {
        let size = io_compression_default_chunk_size();
        // Should be a reasonable size (typically 64KB or larger)
        assert!(size > 0);
    }
}
