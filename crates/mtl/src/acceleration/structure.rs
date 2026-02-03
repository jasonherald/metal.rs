//! Core acceleration structure types.
//!
//! Contains `AccelerationStructure` and `AccelerationStructureSizes`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

pub struct AccelerationStructure(pub(crate) NonNull<c_void>);

impl AccelerationStructure {
    /// Create an AccelerationStructure from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal acceleration structure object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the size in bytes of the acceleration structure.
    ///
    /// C++ equivalent: `NS::UInteger size() const`
    #[inline]
    pub fn size(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(size)) }
    }

    /// Get the GPU resource ID.
    ///
    /// C++ equivalent: `ResourceID gpuResourceID() const`
    #[inline]
    pub fn gpu_resource_id(&self) -> u64 {
        unsafe { msg_send_0(self.as_ptr(), sel!(gpuResourceID)) }
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
                mtl_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
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
        if let Some(ns_label) = mtl_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }
}

impl Clone for AccelerationStructure {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for AccelerationStructure {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for AccelerationStructure {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for AccelerationStructure {}
unsafe impl Sync for AccelerationStructure {}

impl std::fmt::Debug for AccelerationStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccelerationStructure")
            .field("size", &self.size())
            .field("label", &self.label())
            .finish()
    }
}

// ============================================================================
// AccelerationStructureSizes
// ============================================================================

/// Sizes required for building an acceleration structure.
///
/// C++ equivalent: `MTL::AccelerationStructureSizes`
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct AccelerationStructureSizes {
    /// The size of the acceleration structure in bytes.
    pub acceleration_structure_size: UInteger,
    /// The size of the scratch buffer needed during building.
    pub build_scratch_buffer_size: UInteger,
    /// The size of the scratch buffer needed during refitting.
    pub refit_scratch_buffer_size: UInteger,
}
