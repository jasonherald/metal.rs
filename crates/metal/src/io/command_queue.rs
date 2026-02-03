//! IO command queue for Metal.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, msg_send_1, sel};

use super::IOCommandBuffer;

/// Command queue for IO operations.
///
/// C++ equivalent: `MTL::IOCommandQueue`
#[repr(transparent)]
pub struct IOCommandQueue(pub(crate) NonNull<c_void>);

impl IOCommandQueue {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal IO command queue.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a command buffer.
    ///
    /// C++ equivalent: `IOCommandBuffer* commandBuffer()`
    pub fn command_buffer(&self) -> Option<IOCommandBuffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(commandBuffer));
            // commandBuffer returns an autoreleased object, so retain it
            if !ptr.is_null() {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            }
            IOCommandBuffer::from_raw(ptr)
        }
    }

    /// Create a command buffer with unretained references.
    ///
    /// C++ equivalent: `IOCommandBuffer* commandBufferWithUnretainedReferences()`
    pub fn command_buffer_with_unretained_references(&self) -> Option<IOCommandBuffer> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_0(self.as_ptr(), sel!(commandBufferWithUnretainedReferences));
            if !ptr.is_null() {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            }
            IOCommandBuffer::from_raw(ptr)
        }
    }

    /// Enqueue a barrier.
    ///
    /// C++ equivalent: `void enqueueBarrier()`
    #[inline]
    pub fn enqueue_barrier(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(enqueueBarrier));
        }
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
}

impl Clone for IOCommandQueue {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for IOCommandQueue {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for IOCommandQueue {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for IOCommandQueue {}
unsafe impl Sync for IOCommandQueue {}
