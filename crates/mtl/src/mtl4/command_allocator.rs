//! MTL4 CommandAllocator implementation.
//!
//! Corresponds to `Metal/MTL4CommandAllocator.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::Device;

// ============================================================
// CommandAllocatorDescriptor
// ============================================================

/// Descriptor for creating a MTL4 command allocator.
///
/// C++ equivalent: `MTL4::CommandAllocatorDescriptor`
#[repr(transparent)]
pub struct CommandAllocatorDescriptor(NonNull<c_void>);

impl CommandAllocatorDescriptor {
    /// Create a CommandAllocatorDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new command allocator descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTL4CommandAllocatorDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the label.
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ns_string: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ns_string.is_null() {
                return None;
            }
            let c_str: *const i8 = msg_send_0(ns_string, sel!(UTF8String));
            if c_str.is_null() {
                return None;
            }
            Some(
                std::ffi::CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    }

    /// Set the label.
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = mtl_foundation::String::from_str(label) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }
}

impl Clone for CommandAllocatorDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CommandAllocatorDescriptor {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for CommandAllocatorDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CommandAllocatorDescriptor {}
unsafe impl Sync for CommandAllocatorDescriptor {}

// ============================================================
// CommandAllocator
// ============================================================

/// Memory allocator for MTL4 command buffer recording.
///
/// C++ equivalent: `MTL4::CommandAllocator`
///
/// CommandAllocator manages memory for command buffer recording in Metal 4.
/// It provides explicit control over memory allocation for GPU commands.
#[repr(transparent)]
pub struct CommandAllocator(NonNull<c_void>);

impl CommandAllocator {
    /// Create a CommandAllocator from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the device that created this allocator.
    ///
    /// C++ equivalent: `MTL::Device* device() const`
    pub fn device(&self) -> Option<Device> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            Device::from_raw(ptr)
        }
    }

    /// Get the label.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ns_string: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ns_string.is_null() {
                return None;
            }
            let c_str: *const i8 = msg_send_0(ns_string, sel!(UTF8String));
            if c_str.is_null() {
                return None;
            }
            Some(
                std::ffi::CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    }

    /// Get the current allocated size.
    ///
    /// C++ equivalent: `uint64_t allocatedSize()`
    pub fn allocated_size(&self) -> u64 {
        unsafe { msg_send_0(self.as_ptr(), sel!(allocatedSize)) }
    }

    /// Reset the allocator, freeing all allocated memory.
    ///
    /// C++ equivalent: `void reset()`
    pub fn reset(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(reset));
        }
    }
}

impl Clone for CommandAllocator {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CommandAllocator {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for CommandAllocator {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CommandAllocator {}
unsafe impl Sync for CommandAllocator {}

impl std::fmt::Debug for CommandAllocator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandAllocator")
            .field("allocated_size", &self.allocated_size())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_allocator_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<CommandAllocatorDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_command_allocator_size() {
        assert_eq!(
            std::mem::size_of::<CommandAllocator>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_command_allocator_descriptor_creation() {
        // May fail if MTL4 is not available on the system
        if let Some(desc) = CommandAllocatorDescriptor::new() {
            // Basic test - descriptor should exist
            drop(desc);
        }
    }
}
