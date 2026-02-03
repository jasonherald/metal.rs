//! MTL4 RenderCommandEncoder implementation.
//!
//! Corresponds to `Metal/MTL4RenderCommandEncoder.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, msg_send_3, msg_send_4, msg_send_5, msg_send_6, sel};

use crate::{
    CullMode, DepthClipMode, DepthStencilState, Device, PrimitiveType, RenderPipelineState,
    RenderStages, ScissorRect, Size, TriangleFillMode, Viewport, Winding,
};
use super::enums::VisibilityOptions;

// ============================================================
// RenderCommandEncoder
// ============================================================

/// MTL4 render command encoder.
///
/// C++ equivalent: `MTL4::RenderCommandEncoder`
///
/// RenderCommandEncoder encodes render commands including draw calls,
/// state changes, and resource bindings.
#[repr(transparent)]
pub struct RenderCommandEncoder(NonNull<c_void>);

impl RenderCommandEncoder {
    /// Create a RenderCommandEncoder from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the device.
    ///
    /// C++ equivalent: `MTL::Device* device() const`
    pub fn device(&self) -> Option<Device> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            Device::from_raw(ptr)
        }
    }

    /// Get the label.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ns_string: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ns_string.is_null() {
                return None;
            }
            let c_str: *const i8 = msg_send_0(ns_string, sel!(UTF8String));
            if c_str.is_null() {
                return None;
            }
            Some(
                std::ffi::CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    }

    /// Set the label.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    // ========== Pipeline State ==========

    /// Set the render pipeline state.
    ///
    /// C++ equivalent: `void setRenderPipelineState(const MTL::RenderPipelineState*)`
    pub fn set_render_pipeline_state(&self, pipeline: &RenderPipelineState) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setRenderPipelineState:),
                pipeline.as_ptr(),
            );
        }
    }

    // ========== Argument Table ==========

    /// Set vertex argument table at index.
    ///
    /// C++ equivalent: `void setVertexArgumentTable(const MTL4::ArgumentTable*, NS::UInteger)`
    pub fn set_vertex_argument_table(&self, table: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setVertexArgumentTable:atIndex:),
                table,
                index,
            );
        }
    }

    /// Set fragment argument table at index.
    ///
    /// C++ equivalent: `void setFragmentArgumentTable(const MTL4::ArgumentTable*, NS::UInteger)`
    pub fn set_fragment_argument_table(&self, table: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setFragmentArgumentTable:atIndex:),
                table,
                index,
            );
        }
    }

    /// Set tile argument table at index.
    ///
    /// C++ equivalent: `void setTileArgumentTable(const MTL4::ArgumentTable*, NS::UInteger)`
    pub fn set_tile_argument_table(&self, table: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setTileArgumentTable:atIndex:),
                table,
                index,
            );
        }
    }

    /// Set object argument table at index.
    ///
    /// C++ equivalent: `void setObjectArgumentTable(const MTL4::ArgumentTable*, NS::UInteger)`
    pub fn set_object_argument_table(&self, table: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setObjectArgumentTable:atIndex:),
                table,
                index,
            );
        }
    }

    /// Set mesh argument table at index.
    ///
    /// C++ equivalent: `void setMeshArgumentTable(const MTL4::ArgumentTable*, NS::UInteger)`
    pub fn set_mesh_argument_table(&self, table: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setMeshArgumentTable:atIndex:),
                table,
                index,
            );
        }
    }

    // ========== Vertex Buffer Binding ==========

    /// Set vertex buffer at index.
    ///
    /// C++ equivalent: `void setVertexBuffer(const MTL::Buffer*, NS::UInteger, NS::UInteger)`
    pub fn set_vertex_buffer(&self, buffer: *const c_void, offset: UInteger, index: UInteger) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(setVertexBuffer:offset:atIndex:),
                buffer,
                offset,
                index,
            );
        }
    }

    /// Set vertex bytes at index.
    ///
    /// C++ equivalent: `void setVertexBytes(const void*, NS::UInteger, NS::UInteger)`
    pub fn set_vertex_bytes(&self, bytes: *const c_void, length: UInteger, index: UInteger) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(setVertexBytes:length:atIndex:),
                bytes,
                length,
                index,
            );
        }
    }

    /// Set vertex texture at index.
    ///
    /// C++ equivalent: `void setVertexTexture(const MTL::Texture*, NS::UInteger)`
    pub fn set_vertex_texture(&self, texture: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(self.as_ptr(), sel!(setVertexTexture:atIndex:), texture, index);
        }
    }

    /// Set vertex sampler state at index.
    ///
    /// C++ equivalent: `void setVertexSamplerState(const MTL::SamplerState*, NS::UInteger)`
    pub fn set_vertex_sampler_state(&self, sampler: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setVertexSamplerState:atIndex:),
                sampler,
                index,
            );
        }
    }

    // ========== Fragment Buffer Binding ==========

    /// Set fragment buffer at index.
    ///
    /// C++ equivalent: `void setFragmentBuffer(const MTL::Buffer*, NS::UInteger, NS::UInteger)`
    pub fn set_fragment_buffer(&self, buffer: *const c_void, offset: UInteger, index: UInteger) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(setFragmentBuffer:offset:atIndex:),
                buffer,
                offset,
                index,
            );
        }
    }

    /// Set fragment bytes at index.
    ///
    /// C++ equivalent: `void setFragmentBytes(const void*, NS::UInteger, NS::UInteger)`
    pub fn set_fragment_bytes(&self, bytes: *const c_void, length: UInteger, index: UInteger) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(setFragmentBytes:length:atIndex:),
                bytes,
                length,
                index,
            );
        }
    }

    /// Set fragment texture at index.
    ///
    /// C++ equivalent: `void setFragmentTexture(const MTL::Texture*, NS::UInteger)`
    pub fn set_fragment_texture(&self, texture: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setFragmentTexture:atIndex:),
                texture,
                index,
            );
        }
    }

    /// Set fragment sampler state at index.
    ///
    /// C++ equivalent: `void setFragmentSamplerState(const MTL::SamplerState*, NS::UInteger)`
    pub fn set_fragment_sampler_state(&self, sampler: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setFragmentSamplerState:atIndex:),
                sampler,
                index,
            );
        }
    }

    // ========== Tile Buffer Binding ==========

    /// Set tile buffer at index.
    ///
    /// C++ equivalent: `void setTileBuffer(const MTL::Buffer*, NS::UInteger, NS::UInteger)`
    pub fn set_tile_buffer(&self, buffer: *const c_void, offset: UInteger, index: UInteger) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(setTileBuffer:offset:atIndex:),
                buffer,
                offset,
                index,
            );
        }
    }

    /// Set tile bytes at index.
    ///
    /// C++ equivalent: `void setTileBytes(const void*, NS::UInteger, NS::UInteger)`
    pub fn set_tile_bytes(&self, bytes: *const c_void, length: UInteger, index: UInteger) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(setTileBytes:length:atIndex:),
                bytes,
                length,
                index,
            );
        }
    }

    /// Set tile texture at index.
    ///
    /// C++ equivalent: `void setTileTexture(const MTL::Texture*, NS::UInteger)`
    pub fn set_tile_texture(&self, texture: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(self.as_ptr(), sel!(setTileTexture:atIndex:), texture, index);
        }
    }

    /// Set tile sampler state at index.
    ///
    /// C++ equivalent: `void setTileSamplerState(const MTL::SamplerState*, NS::UInteger)`
    pub fn set_tile_sampler_state(&self, sampler: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setTileSamplerState:atIndex:),
                sampler,
                index,
            );
        }
    }

    // ========== Render State ==========

    /// Set viewport.
    ///
    /// C++ equivalent: `void setViewport(MTL::Viewport)`
    pub fn set_viewport(&self, viewport: Viewport) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setViewport:), viewport);
        }
    }

    /// Set multiple viewports.
    ///
    /// C++ equivalent: `void setViewports(const MTL::Viewport*, NS::UInteger)`
    pub fn set_viewports(&self, viewports: *const Viewport, count: UInteger) {
        unsafe {
            let _: () = msg_send_2(self.as_ptr(), sel!(setViewports:count:), viewports, count);
        }
    }

    /// Set scissor rect.
    ///
    /// C++ equivalent: `void setScissorRect(MTL::ScissorRect)`
    pub fn set_scissor_rect(&self, rect: ScissorRect) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setScissorRect:), rect);
        }
    }

    /// Set multiple scissor rects.
    ///
    /// C++ equivalent: `void setScissorRects(const MTL::ScissorRect*, NS::UInteger)`
    pub fn set_scissor_rects(&self, rects: *const ScissorRect, count: UInteger) {
        unsafe {
            let _: () = msg_send_2(self.as_ptr(), sel!(setScissorRects:count:), rects, count);
        }
    }

    /// Set front facing winding.
    ///
    /// C++ equivalent: `void setFrontFacingWinding(MTL::Winding)`
    pub fn set_front_facing_winding(&self, winding: Winding) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setFrontFacingWinding:), winding);
        }
    }

    /// Set cull mode.
    ///
    /// C++ equivalent: `void setCullMode(MTL::CullMode)`
    pub fn set_cull_mode(&self, mode: CullMode) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setCullMode:), mode);
        }
    }

    /// Set triangle fill mode.
    ///
    /// C++ equivalent: `void setTriangleFillMode(MTL::TriangleFillMode)`
    pub fn set_triangle_fill_mode(&self, mode: TriangleFillMode) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setTriangleFillMode:), mode);
        }
    }

    /// Set depth clip mode.
    ///
    /// C++ equivalent: `void setDepthClipMode(MTL::DepthClipMode)`
    pub fn set_depth_clip_mode(&self, mode: DepthClipMode) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDepthClipMode:), mode);
        }
    }

    /// Set depth bias.
    ///
    /// C++ equivalent: `void setDepthBias(float, float, float)`
    pub fn set_depth_bias(&self, depth_bias: f32, slope_scale: f32, clamp: f32) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(setDepthBias:slopeScale:clamp:),
                depth_bias,
                slope_scale,
                clamp,
            );
        }
    }

    /// Set depth stencil state.
    ///
    /// C++ equivalent: `void setDepthStencilState(const MTL::DepthStencilState*)`
    pub fn set_depth_stencil_state(&self, state: &DepthStencilState) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDepthStencilState:), state.as_ptr());
        }
    }

    /// Set stencil reference value.
    ///
    /// C++ equivalent: `void setStencilReferenceValue(uint32_t)`
    pub fn set_stencil_reference_value(&self, value: u32) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setStencilReferenceValue:), value);
        }
    }

    /// Set front and back stencil reference values.
    ///
    /// C++ equivalent: `void setStencilReferenceValues(uint32_t, uint32_t)`
    pub fn set_stencil_reference_values(&self, front: u32, back: u32) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setStencilFrontReferenceValue:backReferenceValue:),
                front,
                back,
            );
        }
    }

    /// Set blend color.
    ///
    /// C++ equivalent: `void setBlendColor(float, float, float, float)`
    pub fn set_blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            let _: () = msg_send_4(
                self.as_ptr(),
                sel!(setBlendColorRed:green:blue:alpha:),
                red,
                green,
                blue,
                alpha,
            );
        }
    }

    // ========== Draw Primitives ==========

    /// Draw primitives.
    ///
    /// C++ equivalent: `void drawPrimitives(MTL::PrimitiveType, NS::UInteger, NS::UInteger)`
    pub fn draw_primitives(
        &self,
        primitive_type: PrimitiveType,
        vertex_start: UInteger,
        vertex_count: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(drawPrimitives:vertexStart:vertexCount:),
                primitive_type,
                vertex_start,
                vertex_count,
            );
        }
    }

    /// Draw primitives with instance count.
    ///
    /// C++ equivalent: `void drawPrimitives(MTL::PrimitiveType, NS::UInteger, NS::UInteger, NS::UInteger)`
    pub fn draw_primitives_instanced(
        &self,
        primitive_type: PrimitiveType,
        vertex_start: UInteger,
        vertex_count: UInteger,
        instance_count: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_4(
                self.as_ptr(),
                sel!(drawPrimitives:vertexStart:vertexCount:instanceCount:),
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
    pub fn draw_primitives_instanced_base(
        &self,
        primitive_type: PrimitiveType,
        vertex_start: UInteger,
        vertex_count: UInteger,
        instance_count: UInteger,
        base_instance: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_5(
                self.as_ptr(),
                sel!(drawPrimitives:vertexStart:vertexCount:instanceCount:baseInstance:),
                primitive_type,
                vertex_start,
                vertex_count,
                instance_count,
                base_instance,
            );
        }
    }

    // ========== Draw Indexed Primitives ==========

    /// Draw indexed primitives.
    ///
    /// C++ equivalent: `void drawIndexedPrimitives(...)`
    pub fn draw_indexed_primitives(
        &self,
        primitive_type: PrimitiveType,
        index_count: UInteger,
        index_type: UInteger,
        index_buffer: *const c_void,
        index_buffer_offset: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_5(
                self.as_ptr(),
                sel!(drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:),
                primitive_type,
                index_count,
                index_type,
                index_buffer,
                index_buffer_offset,
            );
        }
    }

    /// Draw indexed primitives with indirect buffer.
    ///
    /// C++ equivalent: `void drawIndexedPrimitives(... indirectBuffer ...)`
    pub fn draw_indexed_primitives_indirect(
        &self,
        primitive_type: PrimitiveType,
        index_type: UInteger,
        index_buffer: *const c_void,
        index_buffer_offset: UInteger,
        indirect_buffer: *const c_void,
        indirect_buffer_offset: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_6(
                self.as_ptr(),
                sel!(drawIndexedPrimitives:indexType:indexBuffer:indexBufferOffset:indirectBuffer:indirectBufferOffset:),
                primitive_type.0 as UInteger,
                index_type,
                index_buffer,
                index_buffer_offset,
                indirect_buffer,
                indirect_buffer_offset,
            );
        }
    }

    // ========== Draw Mesh Threadgroups ==========

    /// Draw mesh threadgroups.
    ///
    /// C++ equivalent: `void drawMeshThreadgroups(MTL::Size, MTL::Size, MTL::Size)`
    pub fn draw_mesh_threadgroups(
        &self,
        threadgroups_per_grid: Size,
        threads_per_object_threadgroup: Size,
        threads_per_mesh_threadgroup: Size,
    ) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(drawMeshThreadgroups:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:),
                threadgroups_per_grid,
                threads_per_object_threadgroup,
                threads_per_mesh_threadgroup,
            );
        }
    }

    /// Draw mesh threadgroups with indirect buffer.
    ///
    /// C++ equivalent: `void drawMeshThreadgroups(const MTL::Buffer*, NS::UInteger, MTL::Size, MTL::Size)`
    pub fn draw_mesh_threadgroups_indirect(
        &self,
        indirect_buffer: *const c_void,
        indirect_buffer_offset: UInteger,
        threads_per_object_threadgroup: Size,
        threads_per_mesh_threadgroup: Size,
    ) {
        unsafe {
            let _: () = msg_send_4(
                self.as_ptr(),
                sel!(drawMeshThreadgroupsWithIndirectBuffer:indirectBufferOffset:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:),
                indirect_buffer,
                indirect_buffer_offset,
                threads_per_object_threadgroup,
                threads_per_mesh_threadgroup,
            );
        }
    }

    /// Draw mesh threads.
    ///
    /// C++ equivalent: `void drawMeshThreads(MTL::Size, MTL::Size, MTL::Size)`
    pub fn draw_mesh_threads(
        &self,
        threads_per_grid: Size,
        threads_per_object_threadgroup: Size,
        threads_per_mesh_threadgroup: Size,
    ) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(drawMeshThreads:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:),
                threads_per_grid,
                threads_per_object_threadgroup,
                threads_per_mesh_threadgroup,
            );
        }
    }

    // ========== Memory Barrier ==========

    /// Insert a barrier.
    ///
    /// C++ equivalent: `void barrier()`
    pub fn barrier(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(barrier));
        }
    }

    /// Insert a barrier for a buffer.
    ///
    /// C++ equivalent: `void barrier(const MTL::Buffer*, MTL4::VisibilityOptions)`
    pub fn barrier_buffer(&self, buffer: *const c_void, visibility: VisibilityOptions) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(barrierWithBuffer:visibilityOptions:),
                buffer,
                visibility.0,
            );
        }
    }

    /// Insert a barrier for a texture.
    ///
    /// C++ equivalent: `void barrier(const MTL::Texture*, MTL4::VisibilityOptions)`
    pub fn barrier_texture(&self, texture: *const c_void, visibility: VisibilityOptions) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(barrierWithTexture:visibilityOptions:),
                texture,
                visibility.0,
            );
        }
    }

    // ========== Fence Methods ==========

    /// Update a fence.
    ///
    /// C++ equivalent: `void updateFence(const MTL::Fence*)`
    pub fn update_fence(&self, fence: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(updateFence:), fence);
        }
    }

    /// Wait for a fence.
    ///
    /// C++ equivalent: `void waitForFence(const MTL::Fence*)`
    pub fn wait_for_fence(&self, fence: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(waitForFence:), fence);
        }
    }

    // ========== Resource Usage ==========

    /// Use resource.
    ///
    /// C++ equivalent: `void useResource(const MTL::Resource*, MTL::ResourceUsage)`
    pub fn use_resource(&self, resource: *const c_void, usage: UInteger) {
        unsafe {
            let _: () = msg_send_2(self.as_ptr(), sel!(useResource:usage:), resource, usage);
        }
    }

    /// Use heap.
    ///
    /// C++ equivalent: `void useHeap(const MTL::Heap*, MTL::ResourceUsage)`
    pub fn use_heap(&self, heap: *const c_void, usage: UInteger) {
        unsafe {
            let _: () = msg_send_2(self.as_ptr(), sel!(useHeap:usage:), heap, usage);
        }
    }

    // ========== Debug Methods ==========

    /// Push a debug group.
    ///
    /// C++ equivalent: `void pushDebugGroup(const NS::String*)`
    pub fn push_debug_group(&self, name: &str) {
        if let Some(ns_name) = metal_foundation::String::from_str(name) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(pushDebugGroup:), ns_name.as_ptr());
            }
        }
    }

    /// Pop a debug group.
    ///
    /// C++ equivalent: `void popDebugGroup()`
    pub fn pop_debug_group(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(popDebugGroup));
        }
    }

    /// Insert a debug signpost.
    ///
    /// C++ equivalent: `void insertDebugSignpost(const NS::String*)`
    pub fn insert_debug_signpost(&self, name: &str) {
        if let Some(ns_name) = metal_foundation::String::from_str(name) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(insertDebugSignpost:), ns_name.as_ptr());
            }
        }
    }

    // ========== Counter/Timestamp Methods ==========

    /// Write a timestamp to a counter heap.
    ///
    /// C++ equivalent: `void writeTimestamp(MTL4::TimestampGranularity, MTL::RenderStages, const MTL4::CounterHeap*, NS::UInteger)`
    pub fn write_timestamp(
        &self,
        granularity: super::TimestampGranularity,
        stage: RenderStages,
        counter_heap: *const c_void,
        index: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_4(
                self.as_ptr(),
                sel!(writeTimestampWithGranularity:afterStage:intoHeap:atIndex:),
                granularity.0,
                stage.0,
                counter_heap,
                index,
            );
        }
    }

    // ========== Encoding ==========

    /// End encoding.
    ///
    /// C++ equivalent: `void endEncoding()`
    pub fn end_encoding(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(endEncoding));
        }
    }
}

impl Clone for RenderCommandEncoder {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for RenderCommandEncoder {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for RenderCommandEncoder {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RenderCommandEncoder {}
unsafe impl Sync for RenderCommandEncoder {}

impl std::fmt::Debug for RenderCommandEncoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderCommandEncoder")
            .field("label", &self.label())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_command_encoder_size() {
        assert_eq!(
            std::mem::size_of::<RenderCommandEncoder>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
