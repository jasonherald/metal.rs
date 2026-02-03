//! Drawing commands for render encoder.
//!
//! This module contains methods for drawing primitives, indexed primitives,
//! patches, and mesh shaders.

use std::ffi::c_void;

use mtl_foundation::{Integer, Referencing, UInteger};
use mtl_sys::{msg_send_1, sel};

use crate::Buffer;
use crate::enums::{IndexType, PrimitiveType};
use crate::types::Size;

use super::RenderCommandEncoder;

impl RenderCommandEncoder {
    // =========================================================================
    // Drawing
    // =========================================================================

    /// Draw primitives.
    ///
    /// C++ equivalent: `void drawPrimitives(MTL::PrimitiveType, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn draw_primitives(
        &self,
        primitive_type: PrimitiveType,
        vertex_start: UInteger,
        vertex_count: UInteger,
    ) {
        unsafe {
            mtl_sys::msg_send_3::<(), PrimitiveType, UInteger, UInteger>(
                self.as_ptr(),
                sel!(drawPrimitives: vertexStart: vertexCount:),
                primitive_type,
                vertex_start,
                vertex_count,
            );
        }
    }

    /// Draw primitives with instance count.
    ///
    /// C++ equivalent: `void drawPrimitives(MTL::PrimitiveType, NS::UInteger, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn draw_primitives_instanced(
        &self,
        primitive_type: PrimitiveType,
        vertex_start: UInteger,
        vertex_count: UInteger,
        instance_count: UInteger,
    ) {
        unsafe {
            mtl_sys::msg_send_4::<(), PrimitiveType, UInteger, UInteger, UInteger>(
                self.as_ptr(),
                sel!(drawPrimitives: vertexStart: vertexCount: instanceCount:),
                primitive_type,
                vertex_start,
                vertex_count,
                instance_count,
            );
        }
    }

    /// Draw primitives with instance count and base instance.
    ///
    /// C++ equivalent: `void drawPrimitives(MTL::PrimitiveType, NS::UInteger, NS::UInteger, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn draw_primitives_instanced_base_instance(
        &self,
        primitive_type: PrimitiveType,
        vertex_start: UInteger,
        vertex_count: UInteger,
        instance_count: UInteger,
        base_instance: UInteger,
    ) {
        unsafe {
            mtl_sys::msg_send_5::<(), PrimitiveType, UInteger, UInteger, UInteger, UInteger>(
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

    /// Draw primitives using an indirect buffer.
    ///
    /// C++ equivalent: `void drawPrimitives(MTL::PrimitiveType, const Buffer*, NS::UInteger)`
    #[inline]
    pub fn draw_primitives_indirect(
        &self,
        primitive_type: PrimitiveType,
        indirect_buffer: &Buffer,
        indirect_buffer_offset: UInteger,
    ) {
        unsafe {
            mtl_sys::msg_send_3::<(), PrimitiveType, *const c_void, UInteger>(
                self.as_ptr(),
                sel!(drawPrimitives: indirectBuffer: indirectBufferOffset:),
                primitive_type,
                indirect_buffer.as_ptr(),
                indirect_buffer_offset,
            );
        }
    }

    // =========================================================================
    // Indexed Drawing
    // =========================================================================

    /// Draw indexed primitives.
    ///
    /// C++ equivalent: `void drawIndexedPrimitives(MTL::PrimitiveType, NS::UInteger, MTL::IndexType, const Buffer*, NS::UInteger)`
    #[inline]
    pub fn draw_indexed_primitives(
        &self,
        primitive_type: PrimitiveType,
        index_count: UInteger,
        index_type: IndexType,
        index_buffer: &Buffer,
        index_buffer_offset: UInteger,
    ) {
        unsafe {
            mtl_sys::msg_send_5::<(), PrimitiveType, UInteger, IndexType, *const c_void, UInteger>(
                self.as_ptr(),
                sel!(drawIndexedPrimitives: indexCount: indexType: indexBuffer: indexBufferOffset:),
                primitive_type,
                index_count,
                index_type,
                index_buffer.as_ptr(),
                index_buffer_offset,
            );
        }
    }

    /// Draw indexed primitives with instance count.
    ///
    /// C++ equivalent: `void drawIndexedPrimitives(MTL::PrimitiveType, NS::UInteger, MTL::IndexType, const Buffer*, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn draw_indexed_primitives_instanced(
        &self,
        primitive_type: PrimitiveType,
        index_count: UInteger,
        index_type: IndexType,
        index_buffer: &Buffer,
        index_buffer_offset: UInteger,
        instance_count: UInteger,
    ) {
        unsafe {
            mtl_sys::msg_send_6::<
                (),
                PrimitiveType,
                UInteger,
                IndexType,
                *const c_void,
                UInteger,
                UInteger,
            >(
                self.as_ptr(),
                sel!(drawIndexedPrimitives: indexCount: indexType: indexBuffer: indexBufferOffset: instanceCount:),
                primitive_type,
                index_count,
                index_type,
                index_buffer.as_ptr(),
                index_buffer_offset,
                instance_count,
            );
        }
    }

    /// Draw indexed primitives with base vertex and base instance.
    ///
    /// C++ equivalent: `void drawIndexedPrimitives(...instanceCount: baseVertex: baseInstance:)`
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn draw_indexed_primitives_instanced_base_vertex_base_instance(
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
            mtl_sys::msg_send_8::<
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

    /// Draw indexed primitives using an indirect buffer.
    ///
    /// C++ equivalent: `void drawIndexedPrimitives(MTL::PrimitiveType, MTL::IndexType, const Buffer*, NS::UInteger, const Buffer*, NS::UInteger)`
    #[inline]
    pub fn draw_indexed_primitives_indirect(
        &self,
        primitive_type: PrimitiveType,
        index_type: IndexType,
        index_buffer: &Buffer,
        index_buffer_offset: UInteger,
        indirect_buffer: &Buffer,
        indirect_buffer_offset: UInteger,
    ) {
        unsafe {
            mtl_sys::msg_send_6::<
                (),
                PrimitiveType,
                IndexType,
                *const c_void,
                UInteger,
                *const c_void,
                UInteger,
            >(
                self.as_ptr(),
                sel!(drawIndexedPrimitives: indexType: indexBuffer: indexBufferOffset: indirectBuffer: indirectBufferOffset:),
                primitive_type,
                index_type,
                index_buffer.as_ptr(),
                index_buffer_offset,
                indirect_buffer.as_ptr(),
                indirect_buffer_offset,
            );
        }
    }

    // =========================================================================
    // Tessellation
    // =========================================================================

    /// Set the tessellation factor buffer.
    ///
    /// C++ equivalent: `void setTessellationFactorBuffer(const Buffer*, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_tessellation_factor_buffer(
        &self,
        buffer: &Buffer,
        offset: UInteger,
        instance_stride: UInteger,
    ) {
        unsafe {
            mtl_sys::msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setTessellationFactorBuffer: offset: instanceStride:),
                buffer.as_ptr(),
                offset,
                instance_stride,
            );
        }
    }

    /// Set the tessellation factor scale.
    ///
    /// C++ equivalent: `void setTessellationFactorScale(float)`
    #[inline]
    pub fn set_tessellation_factor_scale(&self, scale: f32) {
        unsafe {
            msg_send_1::<(), f32>(self.as_ptr(), sel!(setTessellationFactorScale:), scale);
        }
    }

    /// Draw patches.
    ///
    /// C++ equivalent: `void drawPatches(...)`
    #[allow(clippy::too_many_arguments)]
    pub fn draw_patches(
        &self,
        number_of_patch_control_points: UInteger,
        patch_start: UInteger,
        patch_count: UInteger,
        patch_index_buffer: Option<&Buffer>,
        patch_index_buffer_offset: UInteger,
        instance_count: UInteger,
        base_instance: UInteger,
    ) {
        let patch_index_buffer_ptr = patch_index_buffer
            .map(|b| b.as_ptr())
            .unwrap_or(std::ptr::null());
        unsafe {
            mtl_sys::msg_send_7::<
                (),
                UInteger,
                UInteger,
                UInteger,
                *const c_void,
                UInteger,
                UInteger,
                UInteger,
            >(
                self.as_ptr(),
                sel!(drawPatches: patchStart: patchCount: patchIndexBuffer: patchIndexBufferOffset: instanceCount: baseInstance:),
                number_of_patch_control_points,
                patch_start,
                patch_count,
                patch_index_buffer_ptr,
                patch_index_buffer_offset,
                instance_count,
                base_instance,
            );
        }
    }

    /// Draw indexed patches.
    ///
    /// C++ equivalent: `void drawIndexedPatches(...)`
    #[allow(clippy::too_many_arguments)]
    pub fn draw_indexed_patches(
        &self,
        number_of_patch_control_points: UInteger,
        patch_start: UInteger,
        patch_count: UInteger,
        patch_index_buffer: Option<&Buffer>,
        patch_index_buffer_offset: UInteger,
        control_point_index_buffer: &Buffer,
        control_point_index_buffer_offset: UInteger,
        instance_count: UInteger,
        base_instance: UInteger,
    ) {
        let patch_index_buffer_ptr = patch_index_buffer
            .map(|b| b.as_ptr())
            .unwrap_or(std::ptr::null());
        unsafe {
            mtl_sys::msg_send_9::<
                (),
                UInteger,
                UInteger,
                UInteger,
                *const c_void,
                UInteger,
                *const c_void,
                UInteger,
                UInteger,
                UInteger,
            >(
                self.as_ptr(),
                sel!(drawIndexedPatches: patchStart: patchCount: patchIndexBuffer: patchIndexBufferOffset: controlPointIndexBuffer: controlPointIndexBufferOffset: instanceCount: baseInstance:),
                number_of_patch_control_points,
                patch_start,
                patch_count,
                patch_index_buffer_ptr,
                patch_index_buffer_offset,
                control_point_index_buffer.as_ptr(),
                control_point_index_buffer_offset,
                instance_count,
                base_instance,
            );
        }
    }

    /// Draw indexed patches with indirect buffer.
    ///
    /// C++ equivalent: `void drawIndexedPatches(...indirectBuffer...)`
    #[allow(clippy::too_many_arguments)]
    pub fn draw_indexed_patches_indirect(
        &self,
        number_of_patch_control_points: UInteger,
        patch_index_buffer: Option<&Buffer>,
        patch_index_buffer_offset: UInteger,
        control_point_index_buffer: &Buffer,
        control_point_index_buffer_offset: UInteger,
        indirect_buffer: &Buffer,
        indirect_buffer_offset: UInteger,
    ) {
        let patch_index_buffer_ptr = patch_index_buffer
            .map(|b| b.as_ptr())
            .unwrap_or(std::ptr::null());
        unsafe {
            mtl_sys::msg_send_7::<
                (),
                UInteger,
                *const c_void,
                UInteger,
                *const c_void,
                UInteger,
                *const c_void,
                UInteger,
            >(
                self.as_ptr(),
                sel!(drawIndexedPatches: patchIndexBuffer: patchIndexBufferOffset: controlPointIndexBuffer: controlPointIndexBufferOffset: indirectBuffer: indirectBufferOffset:),
                number_of_patch_control_points,
                patch_index_buffer_ptr,
                patch_index_buffer_offset,
                control_point_index_buffer.as_ptr(),
                control_point_index_buffer_offset,
                indirect_buffer.as_ptr(),
                indirect_buffer_offset,
            );
        }
    }

    /// Draw patches with indirect buffer.
    ///
    /// C++ equivalent: `void drawPatches(...indirectBuffer...)`
    pub fn draw_patches_indirect(
        &self,
        number_of_patch_control_points: UInteger,
        patch_index_buffer: Option<&Buffer>,
        patch_index_buffer_offset: UInteger,
        indirect_buffer: &Buffer,
        indirect_buffer_offset: UInteger,
    ) {
        let patch_index_buffer_ptr = patch_index_buffer
            .map(|b| b.as_ptr())
            .unwrap_or(std::ptr::null());
        unsafe {
            mtl_sys::msg_send_5::<(), UInteger, *const c_void, UInteger, *const c_void, UInteger>(
                self.as_ptr(),
                sel!(drawPatches: patchIndexBuffer: patchIndexBufferOffset: indirectBuffer: indirectBufferOffset:),
                number_of_patch_control_points,
                patch_index_buffer_ptr,
                patch_index_buffer_offset,
                indirect_buffer.as_ptr(),
                indirect_buffer_offset,
            );
        }
    }

    // =========================================================================
    // Mesh Shaders
    // =========================================================================

    /// Draw mesh threadgroups.
    ///
    /// C++ equivalent: `void drawMeshThreadgroups(MTL::Size, MTL::Size, MTL::Size)`
    #[inline]
    pub fn draw_mesh_threadgroups(
        &self,
        threadgroups_per_grid: Size,
        threads_per_object_threadgroup: Size,
        threads_per_mesh_threadgroup: Size,
    ) {
        unsafe {
            mtl_sys::msg_send_3::<(), Size, Size, Size>(
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
    /// C++ equivalent: `void drawMeshThreads(MTL::Size, MTL::Size, MTL::Size)`
    #[inline]
    pub fn draw_mesh_threads(
        &self,
        threads_per_grid: Size,
        threads_per_object_threadgroup: Size,
        threads_per_mesh_threadgroup: Size,
    ) {
        unsafe {
            mtl_sys::msg_send_3::<(), Size, Size, Size>(
                self.as_ptr(),
                sel!(drawMeshThreads: threadsPerObjectThreadgroup: threadsPerMeshThreadgroup:),
                threads_per_grid,
                threads_per_object_threadgroup,
                threads_per_mesh_threadgroup,
            );
        }
    }

    /// Draw mesh threadgroups with indirect buffer.
    ///
    /// C++ equivalent: `void drawMeshThreadgroups(const Buffer*, NS::UInteger, MTL::Size, MTL::Size)`
    #[inline]
    pub fn draw_mesh_threadgroups_indirect(
        &self,
        indirect_buffer: &Buffer,
        indirect_buffer_offset: UInteger,
        threads_per_object_threadgroup: Size,
        threads_per_mesh_threadgroup: Size,
    ) {
        unsafe {
            mtl_sys::msg_send_4::<(), *const c_void, UInteger, Size, Size>(
                self.as_ptr(),
                sel!(drawMeshThreadgroupsWithIndirectBuffer: indirectBufferOffset: threadsPerObjectThreadgroup: threadsPerMeshThreadgroup:),
                indirect_buffer.as_ptr(),
                indirect_buffer_offset,
                threads_per_object_threadgroup,
                threads_per_mesh_threadgroup,
            );
        }
    }

    // =========================================================================
    // Tile Dispatch
    // =========================================================================

    /// Dispatch threads per tile.
    ///
    /// C++ equivalent: `void dispatchThreadsPerTile(MTL::Size)`
    #[inline]
    pub fn dispatch_threads_per_tile(&self, threads_per_tile: Size) {
        unsafe {
            msg_send_1::<(), Size>(
                self.as_ptr(),
                sel!(dispatchThreadsPerTile:),
                threads_per_tile,
            );
        }
    }
}
