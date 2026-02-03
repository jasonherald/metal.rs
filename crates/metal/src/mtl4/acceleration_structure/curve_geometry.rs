//! Descriptor for curve geometry in an acceleration structure.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::{AttributeFormat, CurveBasis, CurveEndCaps, CurveType, IndexType};

use super::BufferRange;

/// Descriptor for curve geometry in an acceleration structure.
///
/// C++ equivalent: `MTL4::AccelerationStructureCurveGeometryDescriptor`
#[repr(transparent)]
pub struct AccelerationStructureCurveGeometryDescriptor(NonNull<c_void>);

impl AccelerationStructureCurveGeometryDescriptor {
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

    /// Create a new curve geometry descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class =
                metal_sys::Class::get("MTL4AccelerationStructureCurveGeometryDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the control point buffer.
    pub fn control_point_buffer(&self) -> BufferRange {
        unsafe { msg_send_0(self.as_ptr(), sel!(controlPointBuffer)) }
    }

    /// Set the control point buffer.
    pub fn set_control_point_buffer(&self, buffer: BufferRange) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setControlPointBuffer:), buffer);
        }
    }

    /// Get the control point count.
    pub fn control_point_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(controlPointCount)) }
    }

    /// Set the control point count.
    pub fn set_control_point_count(&self, count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setControlPointCount:), count);
        }
    }

    /// Get the control point format.
    pub fn control_point_format(&self) -> AttributeFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(controlPointFormat)) }
    }

    /// Set the control point format.
    pub fn set_control_point_format(&self, format: AttributeFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setControlPointFormat:), format);
        }
    }

    /// Get the control point stride.
    pub fn control_point_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(controlPointStride)) }
    }

    /// Set the control point stride.
    pub fn set_control_point_stride(&self, stride: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setControlPointStride:), stride);
        }
    }

    /// Get the curve basis.
    pub fn curve_basis(&self) -> CurveBasis {
        unsafe { msg_send_0(self.as_ptr(), sel!(curveBasis)) }
    }

    /// Set the curve basis.
    pub fn set_curve_basis(&self, basis: CurveBasis) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setCurveBasis:), basis);
        }
    }

    /// Get the curve end caps.
    pub fn curve_end_caps(&self) -> CurveEndCaps {
        unsafe { msg_send_0(self.as_ptr(), sel!(curveEndCaps)) }
    }

    /// Set the curve end caps.
    pub fn set_curve_end_caps(&self, end_caps: CurveEndCaps) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setCurveEndCaps:), end_caps);
        }
    }

    /// Get the curve type.
    pub fn curve_type(&self) -> CurveType {
        unsafe { msg_send_0(self.as_ptr(), sel!(curveType)) }
    }

    /// Set the curve type.
    pub fn set_curve_type(&self, curve_type: CurveType) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setCurveType:), curve_type);
        }
    }

    /// Get the index buffer.
    pub fn index_buffer(&self) -> BufferRange {
        unsafe { msg_send_0(self.as_ptr(), sel!(indexBuffer)) }
    }

    /// Set the index buffer.
    pub fn set_index_buffer(&self, buffer: BufferRange) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setIndexBuffer:), buffer);
        }
    }

    /// Get the index type.
    pub fn index_type(&self) -> IndexType {
        unsafe { msg_send_0(self.as_ptr(), sel!(indexType)) }
    }

    /// Set the index type.
    pub fn set_index_type(&self, index_type: IndexType) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setIndexType:), index_type);
        }
    }

    /// Get the radius buffer.
    pub fn radius_buffer(&self) -> BufferRange {
        unsafe { msg_send_0(self.as_ptr(), sel!(radiusBuffer)) }
    }

    /// Set the radius buffer.
    pub fn set_radius_buffer(&self, buffer: BufferRange) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setRadiusBuffer:), buffer);
        }
    }

    /// Get the radius format.
    pub fn radius_format(&self) -> AttributeFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(radiusFormat)) }
    }

    /// Set the radius format.
    pub fn set_radius_format(&self, format: AttributeFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setRadiusFormat:), format);
        }
    }

    /// Get the radius stride.
    pub fn radius_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(radiusStride)) }
    }

    /// Set the radius stride.
    pub fn set_radius_stride(&self, stride: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setRadiusStride:), stride);
        }
    }

    /// Get the segment control point count.
    pub fn segment_control_point_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(segmentControlPointCount)) }
    }

    /// Set the segment control point count.
    pub fn set_segment_control_point_count(&self, count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setSegmentControlPointCount:), count);
        }
    }

    /// Get the segment count.
    pub fn segment_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(segmentCount)) }
    }

    /// Set the segment count.
    pub fn set_segment_count(&self, count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setSegmentCount:), count);
        }
    }
}

impl Default for AccelerationStructureCurveGeometryDescriptor {
    fn default() -> Self {
        Self::new().expect("Failed to create MTL4AccelerationStructureCurveGeometryDescriptor")
    }
}

impl Clone for AccelerationStructureCurveGeometryDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for AccelerationStructureCurveGeometryDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for AccelerationStructureCurveGeometryDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for AccelerationStructureCurveGeometryDescriptor {}
unsafe impl Sync for AccelerationStructureCurveGeometryDescriptor {}

impl std::fmt::Debug for AccelerationStructureCurveGeometryDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureCurveGeometryDescriptor")
            .field("segment_count", &self.segment_count())
            .finish()
    }
}
