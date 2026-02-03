//! Curve geometry descriptors for acceleration structures.
//!
//! Contains `AccelerationStructureCurveGeometryDescriptor`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::Buffer;
use crate::enums::{AttributeFormat, CurveBasis, CurveEndCaps, CurveType, IndexType};

pub struct AccelerationStructureCurveGeometryDescriptor(pub(crate) NonNull<c_void>);

impl AccelerationStructureCurveGeometryDescriptor {
    /// Create a new curve geometry descriptor.
    ///
    /// C++ equivalent: `static AccelerationStructureCurveGeometryDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLAccelerationStructureCurveGeometryDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal curve geometry descriptor.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // Inherited properties from AccelerationStructureGeometryDescriptor

    /// Get whether duplicate intersection function invocation is allowed.
    #[inline]
    pub fn allow_duplicate_intersection_function_invocation(&self) -> bool {
        unsafe {
            msg_send_0(
                self.as_ptr(),
                sel!(allowDuplicateIntersectionFunctionInvocation),
            )
        }
    }

    /// Set whether duplicate intersection function invocation is allowed.
    #[inline]
    pub fn set_allow_duplicate_intersection_function_invocation(&self, allow: bool) {
        unsafe {
            msg_send_1::<(), bool>(
                self.as_ptr(),
                sel!(setAllowDuplicateIntersectionFunctionInvocation:),
                allow,
            );
        }
    }

    /// Get the intersection function table offset.
    #[inline]
    pub fn intersection_function_table_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(intersectionFunctionTableOffset)) }
    }

    /// Set the intersection function table offset.
    #[inline]
    pub fn set_intersection_function_table_offset(&self, offset: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(
                self.as_ptr(),
                sel!(setIntersectionFunctionTableOffset:),
                offset,
            );
        }
    }

    /// Get whether geometry is opaque.
    #[inline]
    pub fn opaque(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(opaque)) }
    }

    /// Set whether geometry is opaque.
    #[inline]
    pub fn set_opaque(&self, opaque: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setOpaque:), opaque);
        }
    }

    /// Get the label.
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
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the primitive data buffer.
    pub fn primitive_data_buffer(&self) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(primitiveDataBuffer));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Buffer::from_raw(ptr)
        }
    }

    /// Set the primitive data buffer.
    pub fn set_primitive_data_buffer(&self, buffer: Option<&Buffer>) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setPrimitiveDataBuffer:),
                buffer.map_or(std::ptr::null(), |b| b.as_ptr()),
            );
        }
    }

    /// Get the primitive data buffer offset.
    #[inline]
    pub fn primitive_data_buffer_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(primitiveDataBufferOffset)) }
    }

    /// Set the primitive data buffer offset.
    #[inline]
    pub fn set_primitive_data_buffer_offset(&self, offset: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setPrimitiveDataBufferOffset:), offset);
        }
    }

    /// Get the primitive data element size.
    #[inline]
    pub fn primitive_data_element_size(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(primitiveDataElementSize)) }
    }

    /// Set the primitive data element size.
    #[inline]
    pub fn set_primitive_data_element_size(&self, size: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setPrimitiveDataElementSize:), size);
        }
    }

    /// Get the primitive data stride.
    #[inline]
    pub fn primitive_data_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(primitiveDataStride)) }
    }

    /// Set the primitive data stride.
    #[inline]
    pub fn set_primitive_data_stride(&self, stride: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setPrimitiveDataStride:), stride);
        }
    }

    // Curve-specific properties

    /// Get the control point buffer.
    ///
    /// C++ equivalent: `Buffer* controlPointBuffer() const`
    pub fn control_point_buffer(&self) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(controlPointBuffer));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Buffer::from_raw(ptr)
        }
    }

    /// Set the control point buffer.
    ///
    /// C++ equivalent: `void setControlPointBuffer(Buffer*)`
    pub fn set_control_point_buffer(&self, buffer: Option<&Buffer>) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setControlPointBuffer:),
                buffer.map_or(std::ptr::null(), |b| b.as_ptr()),
            );
        }
    }

    /// Get the control point buffer offset.
    ///
    /// C++ equivalent: `NS::UInteger controlPointBufferOffset() const`
    #[inline]
    pub fn control_point_buffer_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(controlPointBufferOffset)) }
    }

    /// Set the control point buffer offset.
    ///
    /// C++ equivalent: `void setControlPointBufferOffset(NS::UInteger)`
    #[inline]
    pub fn set_control_point_buffer_offset(&self, offset: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setControlPointBufferOffset:), offset);
        }
    }

    /// Get the control point count.
    ///
    /// C++ equivalent: `NS::UInteger controlPointCount() const`
    #[inline]
    pub fn control_point_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(controlPointCount)) }
    }

    /// Set the control point count.
    ///
    /// C++ equivalent: `void setControlPointCount(NS::UInteger)`
    #[inline]
    pub fn set_control_point_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setControlPointCount:), count);
        }
    }

    /// Get the control point format.
    ///
    /// C++ equivalent: `AttributeFormat controlPointFormat() const`
    #[inline]
    pub fn control_point_format(&self) -> AttributeFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(controlPointFormat)) }
    }

    /// Set the control point format.
    ///
    /// C++ equivalent: `void setControlPointFormat(AttributeFormat)`
    #[inline]
    pub fn set_control_point_format(&self, format: AttributeFormat) {
        unsafe {
            msg_send_1::<(), AttributeFormat>(self.as_ptr(), sel!(setControlPointFormat:), format);
        }
    }

    /// Get the control point stride.
    ///
    /// C++ equivalent: `NS::UInteger controlPointStride() const`
    #[inline]
    pub fn control_point_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(controlPointStride)) }
    }

    /// Set the control point stride.
    ///
    /// C++ equivalent: `void setControlPointStride(NS::UInteger)`
    #[inline]
    pub fn set_control_point_stride(&self, stride: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setControlPointStride:), stride);
        }
    }

    /// Get the curve basis.
    ///
    /// C++ equivalent: `CurveBasis curveBasis() const`
    #[inline]
    pub fn curve_basis(&self) -> CurveBasis {
        unsafe { msg_send_0(self.as_ptr(), sel!(curveBasis)) }
    }

    /// Set the curve basis.
    ///
    /// C++ equivalent: `void setCurveBasis(CurveBasis)`
    #[inline]
    pub fn set_curve_basis(&self, basis: CurveBasis) {
        unsafe {
            msg_send_1::<(), CurveBasis>(self.as_ptr(), sel!(setCurveBasis:), basis);
        }
    }

    /// Get the curve end caps.
    ///
    /// C++ equivalent: `CurveEndCaps curveEndCaps() const`
    #[inline]
    pub fn curve_end_caps(&self) -> CurveEndCaps {
        unsafe { msg_send_0(self.as_ptr(), sel!(curveEndCaps)) }
    }

    /// Set the curve end caps.
    ///
    /// C++ equivalent: `void setCurveEndCaps(CurveEndCaps)`
    #[inline]
    pub fn set_curve_end_caps(&self, end_caps: CurveEndCaps) {
        unsafe {
            msg_send_1::<(), CurveEndCaps>(self.as_ptr(), sel!(setCurveEndCaps:), end_caps);
        }
    }

    /// Get the curve type.
    ///
    /// C++ equivalent: `CurveType curveType() const`
    #[inline]
    pub fn curve_type(&self) -> CurveType {
        unsafe { msg_send_0(self.as_ptr(), sel!(curveType)) }
    }

    /// Set the curve type.
    ///
    /// C++ equivalent: `void setCurveType(CurveType)`
    #[inline]
    pub fn set_curve_type(&self, curve_type: CurveType) {
        unsafe {
            msg_send_1::<(), CurveType>(self.as_ptr(), sel!(setCurveType:), curve_type);
        }
    }

    /// Get the index buffer.
    ///
    /// C++ equivalent: `Buffer* indexBuffer() const`
    pub fn index_buffer(&self) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(indexBuffer));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Buffer::from_raw(ptr)
        }
    }

    /// Set the index buffer.
    ///
    /// C++ equivalent: `void setIndexBuffer(Buffer*)`
    pub fn set_index_buffer(&self, buffer: Option<&Buffer>) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setIndexBuffer:),
                buffer.map_or(std::ptr::null(), |b| b.as_ptr()),
            );
        }
    }

    /// Get the index buffer offset.
    ///
    /// C++ equivalent: `NS::UInteger indexBufferOffset() const`
    #[inline]
    pub fn index_buffer_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(indexBufferOffset)) }
    }

    /// Set the index buffer offset.
    ///
    /// C++ equivalent: `void setIndexBufferOffset(NS::UInteger)`
    #[inline]
    pub fn set_index_buffer_offset(&self, offset: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setIndexBufferOffset:), offset);
        }
    }

    /// Get the index type.
    ///
    /// C++ equivalent: `IndexType indexType() const`
    #[inline]
    pub fn index_type(&self) -> IndexType {
        unsafe { msg_send_0(self.as_ptr(), sel!(indexType)) }
    }

    /// Set the index type.
    ///
    /// C++ equivalent: `void setIndexType(IndexType)`
    #[inline]
    pub fn set_index_type(&self, index_type: IndexType) {
        unsafe {
            msg_send_1::<(), IndexType>(self.as_ptr(), sel!(setIndexType:), index_type);
        }
    }

    /// Get the radius buffer.
    ///
    /// C++ equivalent: `Buffer* radiusBuffer() const`
    pub fn radius_buffer(&self) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(radiusBuffer));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Buffer::from_raw(ptr)
        }
    }

    /// Set the radius buffer.
    ///
    /// C++ equivalent: `void setRadiusBuffer(Buffer*)`
    pub fn set_radius_buffer(&self, buffer: Option<&Buffer>) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setRadiusBuffer:),
                buffer.map_or(std::ptr::null(), |b| b.as_ptr()),
            );
        }
    }

    /// Get the radius buffer offset.
    ///
    /// C++ equivalent: `NS::UInteger radiusBufferOffset() const`
    #[inline]
    pub fn radius_buffer_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(radiusBufferOffset)) }
    }

    /// Set the radius buffer offset.
    ///
    /// C++ equivalent: `void setRadiusBufferOffset(NS::UInteger)`
    #[inline]
    pub fn set_radius_buffer_offset(&self, offset: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setRadiusBufferOffset:), offset);
        }
    }

    /// Get the radius format.
    ///
    /// C++ equivalent: `AttributeFormat radiusFormat() const`
    #[inline]
    pub fn radius_format(&self) -> AttributeFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(radiusFormat)) }
    }

    /// Set the radius format.
    ///
    /// C++ equivalent: `void setRadiusFormat(AttributeFormat)`
    #[inline]
    pub fn set_radius_format(&self, format: AttributeFormat) {
        unsafe {
            msg_send_1::<(), AttributeFormat>(self.as_ptr(), sel!(setRadiusFormat:), format);
        }
    }

    /// Get the radius stride.
    ///
    /// C++ equivalent: `NS::UInteger radiusStride() const`
    #[inline]
    pub fn radius_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(radiusStride)) }
    }

    /// Set the radius stride.
    ///
    /// C++ equivalent: `void setRadiusStride(NS::UInteger)`
    #[inline]
    pub fn set_radius_stride(&self, stride: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setRadiusStride:), stride);
        }
    }

    /// Get the segment control point count.
    ///
    /// C++ equivalent: `NS::UInteger segmentControlPointCount() const`
    #[inline]
    pub fn segment_control_point_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(segmentControlPointCount)) }
    }

    /// Set the segment control point count.
    ///
    /// C++ equivalent: `void setSegmentControlPointCount(NS::UInteger)`
    #[inline]
    pub fn set_segment_control_point_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setSegmentControlPointCount:), count);
        }
    }

    /// Get the segment count.
    ///
    /// C++ equivalent: `NS::UInteger segmentCount() const`
    #[inline]
    pub fn segment_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(segmentCount)) }
    }

    /// Set the segment count.
    ///
    /// C++ equivalent: `void setSegmentCount(NS::UInteger)`
    #[inline]
    pub fn set_segment_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setSegmentCount:), count);
        }
    }
}

impl Default for AccelerationStructureCurveGeometryDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create curve geometry descriptor")
    }
}

impl Clone for AccelerationStructureCurveGeometryDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy curve geometry descriptor")
        }
    }
}

impl Drop for AccelerationStructureCurveGeometryDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
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
