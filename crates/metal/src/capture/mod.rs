//! Metal GPU capture facilities for debugging.
//!
//! Corresponds to `Metal/MTLCaptureManager.hpp` and `Metal/MTLCaptureScope.hpp`.
//!
//! The capture API allows you to programmatically capture GPU workloads
//! for debugging in Xcode or saving to a GPU trace document.
//!
//! # Example
//!
//! ```ignore
//! use metal::capture::CaptureManager;
//! use metal::device::system_default;
//!
//! let device = system_default().expect("no Metal device");
//! let manager = CaptureManager::shared();
//!
//! // Start capturing with default settings
//! manager.start_capture_with_device(&device);
//!
//! // ... perform GPU work ...
//!
//! manager.stop_capture();
//! ```

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, Url};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, sel};

use crate::enums::{CaptureDestination, CaptureError};
use crate::CommandQueue;
use crate::Device;

// ============================================================================
// CaptureDescriptor
// ============================================================================

/// Descriptor for configuring a GPU capture session.
///
/// C++ equivalent: `MTL::CaptureDescriptor`
#[repr(transparent)]
pub struct CaptureDescriptor(NonNull<c_void>);

impl CaptureDescriptor {
    /// Allocate a new capture descriptor.
    ///
    /// C++ equivalent: `static CaptureDescriptor* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let cls = metal_sys::Class::get("MTLCaptureDescriptor")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize an allocated capture descriptor.
    ///
    /// C++ equivalent: `CaptureDescriptor* init()`
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new capture descriptor.
    pub fn new() -> Option<Self> {
        Self::alloc()?.init()
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal capture descriptor.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the object to capture.
    ///
    /// C++ equivalent: `NS::Object* captureObject() const`
    pub fn capture_object(&self) -> Option<*mut c_void> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(captureObject));
            if ptr.is_null() {
                None
            } else {
                Some(ptr)
            }
        }
    }

    /// Set the object to capture (Device, CommandQueue, or CaptureScope).
    ///
    /// C++ equivalent: `void setCaptureObject(NS::Object*)`
    pub fn set_capture_object(&self, object: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setCaptureObject:), object);
        }
    }

    /// Set a device as the capture target.
    pub fn set_capture_device(&self, device: &Device) {
        self.set_capture_object(device.as_ptr());
    }

    /// Set a command queue as the capture target.
    pub fn set_capture_command_queue(&self, queue: &CommandQueue) {
        self.set_capture_object(queue.as_ptr());
    }

    /// Set a capture scope as the capture target.
    pub fn set_capture_scope(&self, scope: &CaptureScope) {
        self.set_capture_object(scope.as_ptr());
    }

    /// Get the capture destination.
    ///
    /// C++ equivalent: `CaptureDestination destination() const`
    #[inline]
    pub fn destination(&self) -> CaptureDestination {
        unsafe { msg_send_0(self.as_ptr(), sel!(destination)) }
    }

    /// Set the capture destination.
    ///
    /// C++ equivalent: `void setDestination(CaptureDestination)`
    #[inline]
    pub fn set_destination(&self, destination: CaptureDestination) {
        unsafe {
            msg_send_1::<(), CaptureDestination>(self.as_ptr(), sel!(setDestination:), destination);
        }
    }

    /// Get the output URL for trace document captures.
    ///
    /// C++ equivalent: `NS::URL* outputURL() const`
    pub fn output_url(&self) -> Option<Url> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(outputURL));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            Url::from_ptr(ptr)
        }
    }

    /// Set the output URL for trace document captures.
    ///
    /// C++ equivalent: `void setOutputURL(const NS::URL*)`
    pub fn set_output_url(&self, url: &Url) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setOutputURL:), url.as_ptr());
        }
    }
}

impl Default for CaptureDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create CaptureDescriptor")
    }
}

impl Clone for CaptureDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy CaptureDescriptor")
        }
    }
}

impl Drop for CaptureDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for CaptureDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CaptureDescriptor {}
unsafe impl Sync for CaptureDescriptor {}

impl std::fmt::Debug for CaptureDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CaptureDescriptor")
            .field("destination", &self.destination())
            .finish()
    }
}

// ============================================================================
// CaptureScope
// ============================================================================

/// A capture scope that defines the boundaries of a GPU capture.
///
/// C++ equivalent: `MTL::CaptureScope`
#[repr(transparent)]
pub struct CaptureScope(NonNull<c_void>);

impl CaptureScope {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal capture scope.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the device associated with this capture scope.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            Device::from_raw(ptr).expect("capture scope has no device")
        }
    }

    /// Get the command queue associated with this capture scope.
    ///
    /// C++ equivalent: `CommandQueue* commandQueue() const`
    pub fn command_queue(&self) -> Option<CommandQueue> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(commandQueue));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            CommandQueue::from_raw(ptr)
        }
    }

    /// Get the label for this capture scope.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                metal_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Set the label for this capture scope.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Begin the capture scope.
    ///
    /// C++ equivalent: `void beginScope()`
    #[inline]
    pub fn begin_scope(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(beginScope));
        }
    }

    /// End the capture scope.
    ///
    /// C++ equivalent: `void endScope()`
    #[inline]
    pub fn end_scope(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(endScope));
        }
    }
}

impl Clone for CaptureScope {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CaptureScope {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for CaptureScope {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CaptureScope {}
unsafe impl Sync for CaptureScope {}

impl std::fmt::Debug for CaptureScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CaptureScope")
            .field("label", &self.label())
            .finish()
    }
}

// ============================================================================
// CaptureManager
// ============================================================================

/// The singleton manager for GPU capture operations.
///
/// C++ equivalent: `MTL::CaptureManager`
///
/// Use `CaptureManager::shared()` to get the singleton instance.
#[repr(transparent)]
pub struct CaptureManager(NonNull<c_void>);

impl CaptureManager {
    /// Get the shared capture manager singleton.
    ///
    /// Returns `None` if capture is not supported on this system.
    ///
    /// C++ equivalent: `static CaptureManager* sharedCaptureManager()`
    pub fn shared() -> Option<Self> {
        unsafe {
            let cls = metal_sys::Class::get("MTLCaptureManager")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(sharedCaptureManager));
            if ptr.is_null() {
                return None;
            }
            // The shared manager is a singleton, retain it for our reference
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal capture manager.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Check if a capture is currently in progress.
    ///
    /// C++ equivalent: `bool isCapturing() const`
    #[inline]
    pub fn is_capturing(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isCapturing)) }
    }

    /// Get the default capture scope.
    ///
    /// C++ equivalent: `CaptureScope* defaultCaptureScope() const`
    pub fn default_capture_scope(&self) -> Option<CaptureScope> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(defaultCaptureScope));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            CaptureScope::from_raw(ptr)
        }
    }

    /// Set the default capture scope.
    ///
    /// C++ equivalent: `void setDefaultCaptureScope(const CaptureScope*)`
    pub fn set_default_capture_scope(&self, scope: &CaptureScope) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setDefaultCaptureScope:),
                scope.as_ptr(),
            );
        }
    }

    /// Create a new capture scope for a device.
    ///
    /// C++ equivalent: `CaptureScope* newCaptureScope(const Device*)`
    pub fn new_capture_scope_with_device(&self, device: &Device) -> Option<CaptureScope> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newCaptureScopeWithDevice:),
                device.as_ptr(),
            );
            CaptureScope::from_raw(ptr)
        }
    }

    /// Create a new capture scope for a command queue.
    ///
    /// C++ equivalent: `CaptureScope* newCaptureScope(const CommandQueue*)`
    pub fn new_capture_scope_with_command_queue(
        &self,
        queue: &CommandQueue,
    ) -> Option<CaptureScope> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newCaptureScopeWithCommandQueue:),
                queue.as_ptr(),
            );
            CaptureScope::from_raw(ptr)
        }
    }

    /// Check if a capture destination is supported.
    ///
    /// C++ equivalent: `bool supportsDestination(CaptureDestination)`
    #[inline]
    pub fn supports_destination(&self, destination: CaptureDestination) -> bool {
        unsafe { msg_send_1(self.as_ptr(), sel!(supportsDestination:), destination) }
    }

    /// Start a capture with a descriptor.
    ///
    /// Returns `Ok(())` on success or `Err(CaptureError)` on failure.
    ///
    /// C++ equivalent: `bool startCapture(const CaptureDescriptor*, NS::Error**)`
    pub fn start_capture(&self, descriptor: &CaptureDescriptor) -> Result<(), CaptureError> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let success: bool = msg_send_2(
                self.as_ptr(),
                sel!(startCaptureWithDescriptor: error:),
                descriptor.as_ptr(),
                &mut error as *mut *mut c_void,
            );
            if success {
                Ok(())
            } else {
                // Try to get the error code
                if !error.is_null() {
                    let code: metal_foundation::Integer =
                        metal_sys::msg_send_0(error as *const c_void, sel!(code));
                    Err(CaptureError(code))
                } else {
                    Err(CaptureError::NOT_SUPPORTED)
                }
            }
        }
    }

    /// Start a capture for a device (simplified API).
    ///
    /// C++ equivalent: `void startCapture(const Device*)`
    #[inline]
    pub fn start_capture_with_device(&self, device: &Device) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(startCaptureWithDevice:),
                device.as_ptr(),
            );
        }
    }

    /// Start a capture for a command queue (simplified API).
    ///
    /// C++ equivalent: `void startCapture(const CommandQueue*)`
    #[inline]
    pub fn start_capture_with_command_queue(&self, queue: &CommandQueue) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(startCaptureWithCommandQueue:),
                queue.as_ptr(),
            );
        }
    }

    /// Start a capture for a capture scope (simplified API).
    ///
    /// C++ equivalent: `void startCapture(const CaptureScope*)`
    #[inline]
    pub fn start_capture_with_scope(&self, scope: &CaptureScope) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(startCaptureWithScope:),
                scope.as_ptr(),
            );
        }
    }

    /// Stop the current capture.
    ///
    /// C++ equivalent: `void stopCapture()`
    #[inline]
    pub fn stop_capture(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(stopCapture));
        }
    }
}

impl Clone for CaptureManager {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CaptureManager {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for CaptureManager {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CaptureManager {}
unsafe impl Sync for CaptureManager {}

impl std::fmt::Debug for CaptureManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CaptureManager")
            .field("is_capturing", &self.is_capturing())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<CaptureDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_capture_scope_size() {
        assert_eq!(
            std::mem::size_of::<CaptureScope>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_capture_manager_size() {
        assert_eq!(
            std::mem::size_of::<CaptureManager>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_shared_capture_manager() {
        // CaptureManager may not be available in all environments
        if let Some(manager) = CaptureManager::shared() {
            // Should not be capturing by default
            assert!(!manager.is_capturing());
        }
    }

    // Note: test_capture_descriptor_creation removed because CaptureDescriptor
    // is not available in all test environments (e.g., headless CI).
}
