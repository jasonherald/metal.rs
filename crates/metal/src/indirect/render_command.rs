//! A render command within an indirect command buffer.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Integer, Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, msg_send_3, sel};

use crate::enums::{CullMode, DepthClipMode, IndexType, PrimitiveType, TriangleFillMode, Winding};
use crate::types::Size;
use crate::{Buffer, DepthStencilState, RenderPipelineState};

/// A render command within an indirect command buffer.
///
/// C++ equivalent: `MTL::IndirectRenderCommand`
///
/// Indirect render commands can encode draw calls and state changes
/// that will be executed when the indirect command buffer is executed.
#[repr(transparent)]
pub struct IndirectRenderCommand(NonNull<c_void>);

impl IndirectRenderCommand {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal indirect render command.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Reset this command.
    ///
    /// C++ equivalent: `void reset()`
    #[inline]
    pub fn reset(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(reset));
        }
    }

    /// Set a barrier for this command.
    ///
    /// C++ equivalent: `void setBarrier()`
    #[inline]
    pub fn set_barrier(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(setBarrier));
        }
    }

    /// Clear a barrier for this command.
    ///
    /// C++ equivalent: `void clearBarrier()`
    #[inline]
    pub fn clear_barrier(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(clearBarrier));
        }
    }

    /// Set the render pipeline state.
    ///
    /// C++ equivalent: `void setRenderPipelineState(const RenderPipelineState*)`
    pub fn set_render_pipeline_state(&self, state: &RenderPipelineState) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setRenderPipelineState:),
                state.as_ptr(),
            );
        }
    }

    /// Set a vertex buffer.
    ///
    /// C++ equivalent: `void setVertexBuffer(const Buffer*, NS::UInteger offset, NS::UInteger index)`
    pub fn set_vertex_buffer(&self, buffer: &Buffer, offset: UInteger, index: UInteger) {
        unsafe {
            msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setVertexBuffer: offset: atIndex:),
                buffer.as_ptr(),
                offset,
                index,
            );
        }
    }

    /// Set a vertex buffer with stride.
    ///
    /// C++ equivalent: `void setVertexBuffer(const Buffer*, NS::UInteger offset, NS::UInteger stride, NS::UInteger index)`
    pub fn set_vertex_buffer_with_stride(
        &self,
        buffer: &Buffer,
        offset: UInteger,
        stride: UInteger,
        index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_4::<(), *const c_void, UInteger, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setVertexBuffer: offset: attributeStride: atIndex:),
                buffer.as_ptr(),
                offset,
                stride,
                index,
            );
        }
    }

    /// Set a fragment buffer.
    ///
    /// C++ equivalent: `void setFragmentBuffer(const Buffer*, NS::UInteger offset, NS::UInteger index)`
    pub fn set_fragment_buffer(&self, buffer: &Buffer, offset: UInteger, index: UInteger) {
        unsafe {
            msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setFragmentBuffer: offset: atIndex:),
                buffer.as_ptr(),
                offset,
                index,
            );
        }
    }

    /// Set a mesh buffer.
    ///
    /// C++ equivalent: `void setMeshBuffer(const Buffer*, NS::UInteger offset, NS::UInteger index)`
    pub fn set_mesh_buffer(&self, buffer: &Buffer, offset: UInteger, index: UInteger) {
        unsafe {
            msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setMeshBuffer: offset: atIndex:),
                buffer.as_ptr(),
                offset,
                index,
            );
        }
    }

    /// Set an object buffer.
    ///
    /// C++ equivalent: `void setObjectBuffer(const Buffer*, NS::UInteger offset, NS::UInteger index)`
    pub fn set_object_buffer(&self, buffer: &Buffer, offset: UInteger, index: UInteger) {
        unsafe {
            msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setObjectBuffer: offset: atIndex:),
                buffer.as_ptr(),
                offset,
                index,
            );
        }
    }

    /// Set object threadgroup memory length.
    ///
    /// C++ equivalent: `void setObjectThreadgroupMemoryLength(NS::UInteger, NS::UInteger)`
    pub fn set_object_threadgroup_memory_length(&self, length: UInteger, index: UInteger) {
        unsafe {
            msg_send_2::<(), UInteger, UInteger>(
                self.as_ptr(),
                sel!(setObjectThreadgroupMemoryLength: atIndex:),
                length,
                index,
            );
        }
    }

    /// Set the cull mode.
    ///
    /// C++ equivalent: `void setCullMode(CullMode)`
    #[inline]
    pub fn set_cull_mode(&self, mode: CullMode) {
        unsafe {
            msg_send_1::<(), CullMode>(self.as_ptr(), sel!(setCullMode:), mode);
        }
    }

    /// Set the depth bias.
    ///
    /// C++ equivalent: `void setDepthBias(float, float, float)`
    pub fn set_depth_bias(&self, depth_bias: f32, slope_scale: f32, clamp: f32) {
        unsafe {
            msg_send_3::<(), f32, f32, f32>(
                self.as_ptr(),
                sel!(setDepthBias: slopeScale: clamp:),
                depth_bias,
                slope_scale,
                clamp,
            );
        }
    }

    /// Set the depth clip mode.
    ///
    /// C++ equivalent: `void setDepthClipMode(DepthClipMode)`
    #[inline]
    pub fn set_depth_clip_mode(&self, mode: DepthClipMode) {
        unsafe {
            msg_send_1::<(), DepthClipMode>(self.as_ptr(), sel!(setDepthClipMode:), mode);
        }
    }

    /// Set the depth stencil state.
    ///
    /// C++ equivalent: `void setDepthStencilState(const DepthStencilState*)`
    pub fn set_depth_stencil_state(&self, state: &DepthStencilState) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setDepthStencilState:),
                state.as_ptr(),
            );
        }
    }

    /// Set the front facing winding.
    ///
    /// C++ equivalent: `void setFrontFacingWinding(Winding)`
    #[inline]
    pub fn set_front_facing_winding(&self, winding: Winding) {
        unsafe {
            msg_send_1::<(), Winding>(self.as_ptr(), sel!(setFrontFacingWinding:), winding);
        }
    }

    /// Set the triangle fill mode.
    ///
    /// C++ equivalent: `void setTriangleFillMode(TriangleFillMode)`
    #[inline]
    pub fn set_triangle_fill_mode(&self, mode: TriangleFillMode) {
        unsafe {
            msg_send_1::<(), TriangleFillMode>(self.as_ptr(), sel!(setTriangleFillMode:), mode);
        }
    }

    /// Draw primitives.
    ///
    /// C++ equivalent: `void drawPrimitives(...)`
    pub fn draw_primitives(
        &self,
        primitive_type: PrimitiveType,
        vertex_start: UInteger,
        vertex_count: UInteger,
        instance_count: UInteger,
        base_instance: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_5::<(), PrimitiveType, UInteger, UInteger, UInteger, UInteger>(
                self.as_ptr(),
                sel!(drawPrimitives: vertexStart: vertexCount: instanceCount: baseInstance:),
                primitive_type,
                vertex_start,
                vertex_count,
                instance_count,
                base_instance,
            );
        }
    }

    /// Draw indexed primitives.
    ///
    /// C++ equivalent: `void drawIndexedPrimitives(...)`
    #[allow(clippy::too_many_arguments)]
    pub fn draw_indexed_primitives(
        &self,
        primitive_type: PrimitiveType,
        index_count: UInteger,
        index_type: IndexType,
        index_buffer: &Buffer,
        index_buffer_offset: UInteger,
        instance_count: UInteger,
        base_vertex: Integer,
        base_instance: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_8::<
                (),
                PrimitiveType,
                UInteger,
                IndexType,
                *const c_void,
                UInteger,
                UInteger,
                Integer,
                UInteger,
            >(
                self.as_ptr(),
                sel!(drawIndexedPrimitives: indexCount: indexType: indexBuffer: indexBufferOffset: instanceCount: baseVertex: baseInstance:),
                primitive_type,
                index_count,
                index_type,
                index_buffer.as_ptr(),
                index_buffer_offset,
                instance_count,
                base_vertex,
                base_instance,
            );
        }
    }

    /// Draw mesh threadgroups.
    ///
    /// C++ equivalent: `void drawMeshThreadgroups(...)`
    pub fn draw_mesh_threadgroups(
        &self,
        threadgroups_per_grid: Size,
        threads_per_object_threadgroup: Size,
        threads_per_mesh_threadgroup: Size,
    ) {
        unsafe {
            msg_send_3::<(), Size, Size, Size>(
                self.as_ptr(),
                sel!(drawMeshThreadgroups: threadsPerObjectThreadgroup: threadsPerMeshThreadgroup:),
                threadgroups_per_grid,
                threads_per_object_threadgroup,
                threads_per_mesh_threadgroup,
            );
        }
    }

    /// Draw mesh threads.
    ///
    /// C++ equivalent: `void drawMeshThreads(...)`
    pub fn draw_mesh_threads(
        &self,
        threads_per_grid: Size,
        threads_per_object_threadgroup: Size,
        threads_per_mesh_threadgroup: Size,
    ) {
        unsafe {
            msg_send_3::<(), Size, Size, Size>(
                self.as_ptr(),
                sel!(drawMeshThreads: threadsPerObjectThreadgroup: threadsPerMeshThreadgroup:),
                threads_per_grid,
                threads_per_object_threadgroup,
                threads_per_mesh_threadgroup,
            );
        }
    }
}

impl Referencing for IndirectRenderCommand {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

// Note: IndirectRenderCommand is not reference counted - it's a view into the ICB

impl std::fmt::Debug for IndirectRenderCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndirectRenderCommand").finish()
    }
}
