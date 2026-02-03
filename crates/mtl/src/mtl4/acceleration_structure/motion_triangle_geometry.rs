//! Descriptor for motion triangle geometry in an acceleration structure.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::{AttributeFormat, IndexType, MatrixLayout};

use super::BufferRange;

/// Descriptor for motion triangle geometry in an acceleration structure.
///
/// C++ equivalent: `MTL4::AccelerationStructureMotionTriangleGeometryDescriptor`
#[repr(transparent)]
pub struct AccelerationStructureMotionTriangleGeometryDescriptor(NonNull<c_void>);

impl AccelerationStructureMotionTriangleGeometryDescriptor {
    /// Create an AccelerationStructureMotionTriangleGeometryDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new motion triangle geometry descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class =
                mtl_sys::Class::get("MTL4AccelerationStructureMotionTriangleGeometryDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
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

    /// Get the transformation matrix buffer.
    pub fn transformation_matrix_buffer(&self) -> BufferRange {
        unsafe { msg_send_0(self.as_ptr(), sel!(transformationMatrixBuffer)) }
    }

    /// Set the transformation matrix buffer.
    pub fn set_transformation_matrix_buffer(&self, buffer: BufferRange) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setTransformationMatrixBuffer:), buffer);
        }
    }

    /// Get the transformation matrix layout.
    pub fn transformation_matrix_layout(&self) -> MatrixLayout {
        unsafe { msg_send_0(self.as_ptr(), sel!(transformationMatrixLayout)) }
    }

    /// Set the transformation matrix layout.
    pub fn set_transformation_matrix_layout(&self, layout: MatrixLayout) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setTransformationMatrixLayout:), layout);
        }
    }

    /// Get the triangle count.
    pub fn triangle_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(triangleCount)) }
    }

    /// Set the triangle count.
    pub fn set_triangle_count(&self, count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setTriangleCount:), count);
        }
    }

    /// Get the vertex buffers (for motion keyframes).
    pub fn vertex_buffers(&self) -> BufferRange {
        unsafe { msg_send_0(self.as_ptr(), sel!(vertexBuffers)) }
    }

    /// Set the vertex buffers (for motion keyframes).
    pub fn set_vertex_buffers(&self, buffers: BufferRange) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setVertexBuffers:), buffers);
        }
    }

    /// Get the vertex format.
    pub fn vertex_format(&self) -> AttributeFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(vertexFormat)) }
    }

    /// Set the vertex format.
    pub fn set_vertex_format(&self, format: AttributeFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setVertexFormat:), format);
        }
    }

    /// Get the vertex stride.
    pub fn vertex_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(vertexStride)) }
    }

    /// Set the vertex stride.
    pub fn set_vertex_stride(&self, stride: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setVertexStride:), stride);
        }
    }
}

impl Default for AccelerationStructureMotionTriangleGeometryDescriptor {
    fn default() -> Self {
        Self::new()
            .expect("Failed to create MTL4AccelerationStructureMotionTriangleGeometryDescriptor")
    }
}

impl Clone for AccelerationStructureMotionTriangleGeometryDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for AccelerationStructureMotionTriangleGeometryDescriptor {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
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

impl std::fmt::Debug for AccelerationStructureMotionTriangleGeometryDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureMotionTriangleGeometryDescriptor")
            .field("triangle_count", &self.triangle_count())
            .finish()
    }
}
