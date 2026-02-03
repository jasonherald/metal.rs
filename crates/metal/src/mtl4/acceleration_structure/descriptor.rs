//! Base descriptor for creating an acceleration structure.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, sel};

/// Base descriptor for creating an acceleration structure.
///
/// C++ equivalent: `MTL4::AccelerationStructureDescriptor`
#[repr(transparent)]
pub struct AccelerationStructureDescriptor(NonNull<c_void>);

impl AccelerationStructureDescriptor {
    /// Create an AccelerationStructureDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new acceleration structure descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4AccelerationStructureDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }
}

impl Default for AccelerationStructureDescriptor {
    fn default() -> Self {
        Self::new().expect("Failed to create MTL4AccelerationStructureDescriptor")
    }
}

impl Clone for AccelerationStructureDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for AccelerationStructureDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for AccelerationStructureDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for AccelerationStructureDescriptor {}
unsafe impl Sync for AccelerationStructureDescriptor {}

impl std::fmt::Debug for AccelerationStructureDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureDescriptor").finish()
    }
}
