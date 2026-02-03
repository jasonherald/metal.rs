//! Motion triangle geometry descriptor for acceleration structures.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{AttributeFormat, IndexType, MatrixLayout};
use crate::Buffer;

/// Descriptor for motion triangle geometry in acceleration structures.
///
/// C++ equivalent: `MTL::AccelerationStructureMotionTriangleGeometryDescriptor`
#[repr(transparent)]
pub struct AccelerationStructureMotionTriangleGeometryDescriptor(pub(crate) NonNull<c_void>);

impl AccelerationStructureMotionTriangleGeometryDescriptor {
    /// Create a new motion triangle geometry descriptor.
    ///
    /// C++ equivalent: `static AccelerationStructureMotionTriangleGeometryDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get(
                "MTLAccelerationStructureMotionTriangleGeometryDescriptor",
            )?;
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
    /// The pointer must be a valid Metal motion triangle geometry descriptor.
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
        unsafe { msg_send_0(self.as_ptr(), sel!(allowDuplicateIntersectionFunctionInvocation)) }
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

    // Motion triangle-specific properties

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

    /// Get the vertex format.
    ///
    /// C++ equivalent: `AttributeFormat vertexFormat() const`
    #[inline]
    pub fn vertex_format(&self) -> AttributeFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(vertexFormat)) }
    }

    /// Set the vertex format.
    ///
    /// C++ equivalent: `void setVertexFormat(AttributeFormat)`
    #[inline]
    pub fn set_vertex_format(&self, format: AttributeFormat) {
        unsafe {
            msg_send_1::<(), AttributeFormat>(self.as_ptr(), sel!(setVertexFormat:), format);
        }
    }

    /// Get the vertex stride.
    ///
    /// C++ equivalent: `NS::UInteger vertexStride() const`
    #[inline]
    pub fn vertex_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(vertexStride)) }
    }

    /// Set the vertex stride.
    ///
    /// C++ equivalent: `void setVertexStride(NS::UInteger)`
    #[inline]
    pub fn set_vertex_stride(&self, stride: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setVertexStride:), stride);
        }
    }

    /// Get the triangle count.
    ///
    /// C++ equivalent: `NS::UInteger triangleCount() const`
    #[inline]
    pub fn triangle_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(triangleCount)) }
    }

    /// Set the triangle count.
    ///
    /// C++ equivalent: `void setTriangleCount(NS::UInteger)`
    #[inline]
    pub fn set_triangle_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setTriangleCount:), count);
        }
    }

    /// Get the transformation matrix buffer.
    ///
    /// C++ equivalent: `Buffer* transformationMatrixBuffer() const`
    pub fn transformation_matrix_buffer(&self) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(transformationMatrixBuffer));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Buffer::from_raw(ptr)
        }
    }

    /// Set the transformation matrix buffer.
    ///
    /// C++ equivalent: `void setTransformationMatrixBuffer(Buffer*)`
    pub fn set_transformation_matrix_buffer(&self, buffer: Option<&Buffer>) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setTransformationMatrixBuffer:),
                buffer.map_or(std::ptr::null(), |b| b.as_ptr()),
            );
        }
    }

    /// Get the transformation matrix buffer offset.
    ///
    /// C++ equivalent: `NS::UInteger transformationMatrixBufferOffset() const`
    #[inline]
    pub fn transformation_matrix_buffer_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(transformationMatrixBufferOffset)) }
    }

    /// Set the transformation matrix buffer offset.
    ///
    /// C++ equivalent: `void setTransformationMatrixBufferOffset(NS::UInteger)`
    #[inline]
    pub fn set_transformation_matrix_buffer_offset(&self, offset: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(
                self.as_ptr(),
                sel!(setTransformationMatrixBufferOffset:),
                offset,
            );
        }
    }

    /// Get the transformation matrix layout.
    ///
    /// C++ equivalent: `MatrixLayout transformationMatrixLayout() const`
    #[inline]
    pub fn transformation_matrix_layout(&self) -> MatrixLayout {
        unsafe { msg_send_0(self.as_ptr(), sel!(transformationMatrixLayout)) }
    }

    /// Set the transformation matrix layout.
    ///
    /// C++ equivalent: `void setTransformationMatrixLayout(MatrixLayout)`
    #[inline]
    pub fn set_transformation_matrix_layout(&self, layout: MatrixLayout) {
        unsafe {
            msg_send_1::<(), MatrixLayout>(
                self.as_ptr(),
                sel!(setTransformationMatrixLayout:),
                layout,
            );
        }
    }

    /// Get the vertex buffers as a raw NS::Array pointer.
    ///
    /// C++ equivalent: `NS::Array* vertexBuffers() const`
    ///
    /// # Safety
    ///
    /// The returned pointer is an NS::Array containing MotionKeyframeData objects.
    /// The caller must manage the memory appropriately.
    #[inline]
    pub fn vertex_buffers_ptr(&self) -> *const c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(vertexBuffers)) }
    }

    /// Set the vertex buffers from a raw NS::Array pointer.
    ///
    /// C++ equivalent: `void setVertexBuffers(const NS::Array*)`
    ///
    /// # Safety
    ///
    /// The vertex_buffers pointer must be a valid NS::Array or null.
    pub unsafe fn set_vertex_buffers_ptr(&self, vertex_buffers: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setVertexBuffers:),
                vertex_buffers,
            );
        }
    }
}

impl Default for AccelerationStructureMotionTriangleGeometryDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create motion triangle geometry descriptor")
    }
}

impl Clone for AccelerationStructureMotionTriangleGeometryDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy motion triangle geometry descriptor")
        }
    }
}

impl Drop for AccelerationStructureMotionTriangleGeometryDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for AccelerationStructureMotionTriangleGeometryDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for AccelerationStructureMotionTriangleGeometryDescriptor {}
unsafe impl Sync for AccelerationStructureMotionTriangleGeometryDescriptor {}
