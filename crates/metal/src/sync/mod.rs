//! Metal synchronization primitives.
//!
//! Corresponds to `Metal/MTLEvent.hpp` and `Metal/MTLFence.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, msg_send_1, sel};

// ============================================================================
// Event
// ============================================================================

/// A simple synchronization primitive.
///
/// C++ equivalent: `MTL::Event`
#[repr(transparent)]
pub struct Event(pub(crate) NonNull<c_void>);

impl Event {
    /// Create an Event from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal event object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the label.
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

    /// Set the label.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the device.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("event has no device")
        }
    }
}

impl Clone for Event {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for Event {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for Event {}
unsafe impl Sync for Event {}

impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Event")
            .field("label", &self.label())
            .finish()
    }
}

// ============================================================================
// SharedEvent
// ============================================================================

/// A cross-process synchronization primitive.
///
/// C++ equivalent: `MTL::SharedEvent`
#[repr(transparent)]
pub struct SharedEvent(pub(crate) NonNull<c_void>);

impl SharedEvent {
    /// Create a SharedEvent from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal shared event object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the label.
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

    /// Set the label.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the device.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("shared event has no device")
        }
    }

    /// Get the current signaled value.
    ///
    /// C++ equivalent: `uint64_t signaledValue() const`
    #[inline]
    pub fn signaled_value(&self) -> u64 {
        unsafe { msg_send_0(self.as_ptr(), sel!(signaledValue)) }
    }

    /// Set the signaled value.
    ///
    /// C++ equivalent: `void setSignaledValue(uint64_t)`
    #[inline]
    pub fn set_signaled_value(&self, value: u64) {
        unsafe {
            msg_send_1::<(), u64>(self.as_ptr(), sel!(setSignaledValue:), value);
        }
    }

    /// Create a handle for sharing across processes.
    ///
    /// C++ equivalent: `SharedEventHandle* newSharedEventHandle()`
    pub fn new_shared_event_handle(&self) -> Option<SharedEventHandle> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(newSharedEventHandle));
            SharedEventHandle::from_raw(ptr)
        }
    }

    /// Notify a listener when the event reaches a value.
    ///
    /// C++ equivalent: `void notifyListener(SharedEventListener*, uint64_t, void (^)(SharedEvent*, uint64_t))`
    ///
    /// # Safety
    ///
    /// The listener pointer must be valid.
    pub unsafe fn notify_listener<F>(
        &self,
        listener: *const c_void,
        value: u64,
        block: F,
    ) where
        F: Fn(*mut c_void, u64) + Send + 'static,
    {
        let block = metal_sys::EventBlock::from_fn(block);
        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, u64, *const c_void>(
                self.as_ptr(),
                sel!(notifyListener: atValue: block:),
                listener,
                value,
                block.as_ptr(),
            );
        }
        std::mem::forget(block);
    }
}

impl Clone for SharedEvent {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for SharedEvent {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for SharedEvent {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for SharedEvent {}
unsafe impl Sync for SharedEvent {}

impl std::fmt::Debug for SharedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedEvent")
            .field("label", &self.label())
            .field("signaled_value", &self.signaled_value())
            .finish()
    }
}

// ============================================================================
// SharedEventHandle
// ============================================================================

/// A handle for sharing events across processes.
///
/// C++ equivalent: `MTL::SharedEventHandle`
#[repr(transparent)]
pub struct SharedEventHandle(pub(crate) NonNull<c_void>);

impl SharedEventHandle {
    /// Create a SharedEventHandle from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal shared event handle object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the label.
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
}

impl Clone for SharedEventHandle {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for SharedEventHandle {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for SharedEventHandle {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for SharedEventHandle {}
unsafe impl Sync for SharedEventHandle {}

// ============================================================================
// SharedEventListener
// ============================================================================

/// A listener that can be notified when a shared event reaches a value.
///
/// C++ equivalent: `MTL::SharedEventListener`
#[repr(transparent)]
pub struct SharedEventListener(pub(crate) NonNull<c_void>);

impl SharedEventListener {
    /// Allocate a new shared event listener.
    ///
    /// C++ equivalent: `static SharedEventListener* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let cls = metal_sys::Class::get("MTLSharedEventListener")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize an allocated listener with the default dispatch queue.
    ///
    /// C++ equivalent: `SharedEventListener* init()`
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new shared event listener with the default dispatch queue.
    pub fn new() -> Option<Self> {
        Self::alloc()?.init()
    }

    /// Get the shared (global) listener singleton.
    ///
    /// C++ equivalent: `static SharedEventListener* sharedListener()`
    pub fn shared() -> Option<Self> {
        unsafe {
            let cls = metal_sys::Class::get("MTLSharedEventListener")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(sharedListener));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal shared event listener.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the dispatch queue for this listener.
    ///
    /// C++ equivalent: `dispatch_queue_t dispatchQueue() const`
    ///
    /// Returns the raw dispatch_queue_t pointer.
    #[inline]
    pub fn dispatch_queue_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(dispatchQueue)) }
    }
}

impl Clone for SharedEventListener {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for SharedEventListener {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for SharedEventListener {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for SharedEventListener {}
unsafe impl Sync for SharedEventListener {}

impl std::fmt::Debug for SharedEventListener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedEventListener").finish()
    }
}

// ============================================================================
// Fence
// ============================================================================

/// A GPU-side synchronization primitive.
///
/// C++ equivalent: `MTL::Fence`
#[repr(transparent)]
pub struct Fence(pub(crate) NonNull<c_void>);

impl Fence {
    /// Create a Fence from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal fence object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the label.
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

    /// Set the label.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the device.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("fence has no device")
        }
    }
}

impl Clone for Fence {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for Fence {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for Fence {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for Fence {}
unsafe impl Sync for Fence {}

impl std::fmt::Debug for Fence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Fence")
            .field("label", &self.label())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_size() {
        assert_eq!(
            std::mem::size_of::<Event>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_shared_event_size() {
        assert_eq!(
            std::mem::size_of::<SharedEvent>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_fence_size() {
        assert_eq!(
            std::mem::size_of::<Fence>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_shared_event_listener_size() {
        assert_eq!(
            std::mem::size_of::<SharedEventListener>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
