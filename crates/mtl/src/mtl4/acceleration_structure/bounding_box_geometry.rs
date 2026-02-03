//! Descriptor for bounding box geometry in an acceleration structure.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use super::BufferRange;

/// Descriptor for bounding box geometry in an acceleration structure.
///
/// C++ equivalent: `MTL4::AccelerationStructureBoundingBoxGeometryDescriptor`
#[repr(transparent)]
pub struct AccelerationStructureBoundingBoxGeometryDescriptor(NonNull<c_void>);

impl AccelerationStructureBoundingBoxGeometryDescriptor {
    /// Create an AccelerationStructureBoundingBoxGeometryDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new bounding box geometry descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class =
                mtl_sys::Class::get("MTL4AccelerationStructureBoundingBoxGeometryDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the bounding box buffer.
    ///
    /// C++ equivalent: `BufferRange boundingBoxBuffer() const`
    pub fn bounding_box_buffer(&self) -> BufferRange {
        unsafe { msg_send_0(self.as_ptr(), sel!(boundingBoxBuffer)) }
    }

    /// Set the bounding box buffer.
    ///
    /// C++ equivalent: `void setBoundingBoxBuffer(const MTL4::BufferRange)`
    pub fn set_bounding_box_buffer(&self, buffer: BufferRange) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setBoundingBoxBuffer:), buffer);
        }
    }

    /// Get the bounding box count.
    ///
    /// C++ equivalent: `NS::UInteger boundingBoxCount() const`
    pub fn bounding_box_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(boundingBoxCount)) }
    }

    /// Set the bounding box count.
    ///
    /// C++ equivalent: `void setBoundingBoxCount(NS::UInteger)`
    pub fn set_bounding_box_count(&self, count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setBoundingBoxCount:), count);
        }
    }

    /// Get the bounding box stride.
    ///
    /// C++ equivalent: `NS::UInteger boundingBoxStride() const`
    pub fn bounding_box_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(boundingBoxStride)) }
    }

    /// Set the bounding box stride.
    ///
    /// C++ equivalent: `void setBoundingBoxStride(NS::UInteger)`
    pub fn set_bounding_box_stride(&self, stride: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setBoundingBoxStride:), stride);
        }
    }
}

impl Default for AccelerationStructureBoundingBoxGeometryDescriptor {
    fn default() -> Self {
        Self::new()
            .expect("Failed to create MTL4AccelerationStructureBoundingBoxGeometryDescriptor")
    }
}

impl Clone for AccelerationStructureBoundingBoxGeometryDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for AccelerationStructureBoundingBoxGeometryDescriptor {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for AccelerationStructureBoundingBoxGeometryDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for AccelerationStructureBoundingBoxGeometryDescriptor {}
unsafe impl Sync for AccelerationStructureBoundingBoxGeometryDescriptor {}

impl std::fmt::Debug for AccelerationStructureBoundingBoxGeometryDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureBoundingBoxGeometryDescriptor")
            .field("bounding_box_count", &self.bounding_box_count())
            .finish()
    }
}
