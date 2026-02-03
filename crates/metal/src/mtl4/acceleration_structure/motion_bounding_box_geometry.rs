//! Descriptor for motion bounding box geometry in an acceleration structure.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use super::BufferRange;

/// Descriptor for motion bounding box geometry in an acceleration structure.
///
/// C++ equivalent: `MTL4::AccelerationStructureMotionBoundingBoxGeometryDescriptor`
#[repr(transparent)]
pub struct AccelerationStructureMotionBoundingBoxGeometryDescriptor(NonNull<c_void>);

impl AccelerationStructureMotionBoundingBoxGeometryDescriptor {
    /// Create from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new motion bounding box geometry descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get(
                "MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor",
            )?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the bounding box buffers (for motion keyframes).
    pub fn bounding_box_buffers(&self) -> BufferRange {
        unsafe { msg_send_0(self.as_ptr(), sel!(boundingBoxBuffers)) }
    }

    /// Set the bounding box buffers (for motion keyframes).
    pub fn set_bounding_box_buffers(&self, buffers: BufferRange) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setBoundingBoxBuffers:), buffers);
        }
    }

    /// Get the bounding box count.
    pub fn bounding_box_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(boundingBoxCount)) }
    }

    /// Set the bounding box count.
    pub fn set_bounding_box_count(&self, count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setBoundingBoxCount:), count);
        }
    }

    /// Get the bounding box stride.
    pub fn bounding_box_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(boundingBoxStride)) }
    }

    /// Set the bounding box stride.
    pub fn set_bounding_box_stride(&self, stride: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setBoundingBoxStride:), stride);
        }
    }
}

impl Default for AccelerationStructureMotionBoundingBoxGeometryDescriptor {
    fn default() -> Self {
        Self::new()
            .expect("Failed to create MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor")
    }
}

impl Clone for AccelerationStructureMotionBoundingBoxGeometryDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for AccelerationStructureMotionBoundingBoxGeometryDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for AccelerationStructureMotionBoundingBoxGeometryDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for AccelerationStructureMotionBoundingBoxGeometryDescriptor {}
unsafe impl Sync for AccelerationStructureMotionBoundingBoxGeometryDescriptor {}

impl std::fmt::Debug for AccelerationStructureMotionBoundingBoxGeometryDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureMotionBoundingBoxGeometryDescriptor")
            .field("bounding_box_count", &self.bounding_box_count())
            .finish()
    }
}
