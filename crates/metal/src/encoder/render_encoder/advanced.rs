//! Advanced render encoder features.
//!
//! This module contains methods for:
//! - Memory barriers
//! - Resource usage declarations
//! - Fence operations
//! - Vertex amplification
//! - Acceleration structure bindings
//! - Function table bindings
//! - Indirect command execution
//! - Counter sampling

use std::ffi::c_void;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{BarrierScope, RenderStages, ResourceUsage};
use crate::types::VertexAmplificationViewMapping;
use crate::Buffer;
use crate::Texture;

use super::RenderCommandEncoder;

impl RenderCommandEncoder {
    // =========================================================================
    // Memory Barriers
    // =========================================================================

    /// Insert a memory barrier with scope.
    ///
    /// C++ equivalent: `void memoryBarrier(MTL::BarrierScope, MTL::RenderStages, MTL::RenderStages)`
    #[inline]
    pub fn memory_barrier_with_scope(
        &self,
        scope: BarrierScope,
        after_stages: RenderStages,
        before_stages: RenderStages,
    ) {
        unsafe {
            metal_sys::msg_send_3::<(), BarrierScope, RenderStages, RenderStages>(
                self.as_ptr(),
                sel!(memoryBarrierWithScope: afterStages: beforeStages:),
                scope,
                after_stages,
                before_stages,
            );
        }
    }

    /// Insert a texture barrier (deprecated).
    ///
    /// C++ equivalent: `void textureBarrier()`
    #[inline]
    pub fn texture_barrier(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(textureBarrier));
        }
    }

    /// Insert a memory barrier for specific resources (raw pointer version).
    ///
    /// C++ equivalent: `void memoryBarrier(const Resource* const*, NS::UInteger, MTL::RenderStages, MTL::RenderStages)`
    ///
    /// # Safety
    ///
    /// The resources pointer must be a valid array with the specified count.
    #[inline]
    pub unsafe fn memory_barrier_with_resources_ptr(
        &self,
        resources: *const *const c_void,
        count: UInteger,
        after_stages: RenderStages,
        before_stages: RenderStages,
    ) {
        unsafe {
            metal_sys::msg_send_4::<(), *const *const c_void, UInteger, RenderStages, RenderStages>(
                self.as_ptr(),
                sel!(memoryBarrierWithResources: count: afterStages: beforeStages:),
                resources,
                count,
                after_stages,
                before_stages,
            );
        }
    }

    // =========================================================================
    // Resource Usage
    // =========================================================================

    /// Declare that a resource will be used (raw pointer version).
    ///
    /// C++ equivalent: `void useResource(const Resource*, MTL::ResourceUsage)`
    ///
    /// # Safety
    ///
    /// The resource pointer must be a valid Metal resource object.
    #[inline]
    pub unsafe fn use_resource_ptr(&self, resource: *const c_void, usage: ResourceUsage) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, ResourceUsage>(
                self.as_ptr(),
                sel!(useResource: usage:),
                resource,
                usage,
            );
        }
    }

    /// Declare that a resource will be used with stages (raw pointer version).
    ///
    /// C++ equivalent: `void useResource(const Resource*, MTL::ResourceUsage, MTL::RenderStages)`
    ///
    /// # Safety
    ///
    /// The resource pointer must be a valid Metal resource object.
    #[inline]
    pub unsafe fn use_resource_ptr_with_stages(
        &self,
        resource: *const c_void,
        usage: ResourceUsage,
        stages: RenderStages,
    ) {
        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, ResourceUsage, RenderStages>(
                self.as_ptr(),
                sel!(useResource: usage: stages:),
                resource,
                usage,
                stages,
            );
        }
    }

    /// Declare that a buffer will be used.
    ///
    /// C++ equivalent: `void useResource(const Resource*, MTL::ResourceUsage)`
    #[inline]
    pub fn use_buffer(&self, buffer: &Buffer, usage: ResourceUsage) {
        unsafe { self.use_resource_ptr(buffer.as_ptr(), usage) };
    }

    /// Declare that a texture will be used.
    ///
    /// C++ equivalent: `void useResource(const Resource*, MTL::ResourceUsage)`
    #[inline]
    pub fn use_texture(&self, texture: &Texture, usage: ResourceUsage) {
        unsafe { self.use_resource_ptr(texture.as_ptr(), usage) };
    }

    /// Declare that a heap will be used (raw pointer version).
    ///
    /// C++ equivalent: `void useHeap(const Heap*)`
    ///
    /// # Safety
    ///
    /// The heap pointer must be a valid Metal heap object.
    #[inline]
    pub unsafe fn use_heap_ptr(&self, heap: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(useHeap:), heap);
        }
    }

    /// Declare that a heap will be used.
    ///
    /// C++ equivalent: `void useHeap(const Heap*)`
    #[inline]
    pub fn use_heap(&self, heap: &crate::Heap) {
        unsafe { self.use_heap_ptr(heap.as_ptr()) };
    }

    /// Declare that a heap will be used with stages (raw pointer version).
    ///
    /// C++ equivalent: `void useHeap(const Heap*, MTL::RenderStages)`
    ///
    /// # Safety
    ///
    /// The heap pointer must be a valid Metal heap object.
    #[inline]
    pub unsafe fn use_heap_ptr_with_stages(&self, heap: *const c_void, stages: RenderStages) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, RenderStages>(
                self.as_ptr(),
                sel!(useHeap: stages:),
                heap,
                stages,
            );
        }
    }

    /// Declare that multiple heaps will be used (raw pointer version).
    ///
    /// C++ equivalent: `void useHeaps(const Heap* const*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The heaps pointer must be a valid array of heap pointers with the specified count.
    #[inline]
    pub unsafe fn use_heaps_ptr(&self, heaps: *const *const c_void, count: UInteger) {
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, UInteger>(
                self.as_ptr(),
                sel!(useHeaps: count:),
                heaps,
                count,
            );
        }
    }

    /// Declare that multiple heaps will be used with stages (raw pointer version).
    ///
    /// C++ equivalent: `void useHeaps(const Heap* const*, NS::UInteger, MTL::RenderStages)`
    ///
    /// # Safety
    ///
    /// The heaps pointer must be a valid array of heap pointers with the specified count.
    #[inline]
    pub unsafe fn use_heaps_with_stages_ptr(
        &self,
        heaps: *const *const c_void,
        count: UInteger,
        stages: RenderStages,
    ) {
        unsafe {
            metal_sys::msg_send_3::<(), *const *const c_void, UInteger, RenderStages>(
                self.as_ptr(),
                sel!(useHeaps: count: stages:),
                heaps,
                count,
                stages,
            );
        }
    }

    /// Declare that multiple resources will be used (raw pointer version).
    ///
    /// C++ equivalent: `void useResources(const Resource* const*, NS::UInteger, MTL::ResourceUsage)`
    ///
    /// # Safety
    ///
    /// The resources pointer must be a valid array of resource pointers with the specified count.
    #[inline]
    pub unsafe fn use_resources_ptr(
        &self,
        resources: *const *const c_void,
        count: UInteger,
        usage: ResourceUsage,
    ) {
        unsafe {
            metal_sys::msg_send_3::<(), *const *const c_void, UInteger, ResourceUsage>(
                self.as_ptr(),
                sel!(useResources: count: usage:),
                resources,
                count,
                usage,
            );
        }
    }

    /// Declare that multiple resources will be used with stages (raw pointer version).
    ///
    /// C++ equivalent: `void useResources(const Resource* const*, NS::UInteger, MTL::ResourceUsage, MTL::RenderStages)`
    ///
    /// # Safety
    ///
    /// The resources pointer must be a valid array of resource pointers with the specified count.
    #[inline]
    pub unsafe fn use_resources_with_stages_ptr(
        &self,
        resources: *const *const c_void,
        count: UInteger,
        usage: ResourceUsage,
        stages: RenderStages,
    ) {
        unsafe {
            metal_sys::msg_send_4::<(), *const *const c_void, UInteger, ResourceUsage, RenderStages>(
                self.as_ptr(),
                sel!(useResources: count: usage: stages:),
                resources,
                count,
                usage,
                stages,
            );
        }
    }

    // =========================================================================
    // Fence Operations
    // =========================================================================

    /// Update a fence (raw pointer version).
    ///
    /// C++ equivalent: `void updateFence(const Fence*, MTL::RenderStages)`
    ///
    /// # Safety
    ///
    /// The fence pointer must be a valid Metal fence object.
    #[inline]
    pub unsafe fn update_fence_ptr(&self, fence: *const c_void, stages: RenderStages) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, RenderStages>(
                self.as_ptr(),
                sel!(updateFence: afterStages:),
                fence,
                stages,
            );
        }
    }

    /// Update a fence.
    ///
    /// C++ equivalent: `void updateFence(const Fence*, MTL::RenderStages)`
    #[inline]
    pub fn update_fence(&self, fence: &crate::Fence, stages: RenderStages) {
        unsafe { self.update_fence_ptr(fence.as_ptr(), stages) };
    }

    /// Wait for a fence (raw pointer version).
    ///
    /// C++ equivalent: `void waitForFence(const Fence*, MTL::RenderStages)`
    ///
    /// # Safety
    ///
    /// The fence pointer must be a valid Metal fence object.
    #[inline]
    pub unsafe fn wait_for_fence_ptr(&self, fence: *const c_void, stages: RenderStages) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, RenderStages>(
                self.as_ptr(),
                sel!(waitForFence: beforeStages:),
                fence,
                stages,
            );
        }
    }

    /// Wait for a fence.
    ///
    /// C++ equivalent: `void waitForFence(const Fence*, MTL::RenderStages)`
    #[inline]
    pub fn wait_for_fence(&self, fence: &crate::Fence, stages: RenderStages) {
        unsafe { self.wait_for_fence_ptr(fence.as_ptr(), stages) };
    }

    // =========================================================================
    // Vertex Amplification
    // =========================================================================

    /// Set vertex amplification count.
    ///
    /// C++ equivalent: `void setVertexAmplificationCount(NS::UInteger, const VertexAmplificationViewMapping*)`
    #[inline]
    pub fn set_vertex_amplification_count(
        &self,
        count: UInteger,
        view_mappings: Option<&[VertexAmplificationViewMapping]>,
    ) {
        let ptr = view_mappings
            .map(|m| m.as_ptr())
            .unwrap_or(std::ptr::null());
        unsafe {
            metal_sys::msg_send_2::<(), UInteger, *const VertexAmplificationViewMapping>(
                self.as_ptr(),
                sel!(setVertexAmplificationCount: viewMappings:),
                count,
                ptr,
            );
        }
    }

    // =========================================================================
    // Acceleration Structure Bindings
    // =========================================================================

    /// Set a vertex acceleration structure (raw pointer version).
    ///
    /// C++ equivalent: `void setVertexAccelerationStructure(const AccelerationStructure*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The acceleration structure pointer must be valid.
    #[inline]
    pub unsafe fn set_vertex_acceleration_structure_ptr(
        &self,
        acceleration_structure: *const c_void,
        buffer_index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setVertexAccelerationStructure: atBufferIndex:),
                acceleration_structure,
                buffer_index,
            );
        }
    }

    /// Set a vertex acceleration structure.
    ///
    /// C++ equivalent: `void setVertexAccelerationStructure(const AccelerationStructure*, NS::UInteger)`
    #[inline]
    pub fn set_vertex_acceleration_structure(
        &self,
        acceleration_structure: &crate::AccelerationStructure,
        buffer_index: UInteger,
    ) {
        unsafe {
            self.set_vertex_acceleration_structure_ptr(acceleration_structure.as_ptr(), buffer_index)
        };
    }

    /// Set a fragment acceleration structure (raw pointer version).
    ///
    /// C++ equivalent: `void setFragmentAccelerationStructure(const AccelerationStructure*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The acceleration structure pointer must be valid.
    #[inline]
    pub unsafe fn set_fragment_acceleration_structure_ptr(
        &self,
        acceleration_structure: *const c_void,
        buffer_index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setFragmentAccelerationStructure: atBufferIndex:),
                acceleration_structure,
                buffer_index,
            );
        }
    }

    /// Set a fragment acceleration structure.
    ///
    /// C++ equivalent: `void setFragmentAccelerationStructure(const AccelerationStructure*, NS::UInteger)`
    #[inline]
    pub fn set_fragment_acceleration_structure(
        &self,
        acceleration_structure: &crate::AccelerationStructure,
        buffer_index: UInteger,
    ) {
        unsafe {
            self.set_fragment_acceleration_structure_ptr(acceleration_structure.as_ptr(), buffer_index)
        };
    }

    /// Set a tile acceleration structure (raw pointer version).
    ///
    /// C++ equivalent: `void setTileAccelerationStructure(const AccelerationStructure*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The acceleration structure pointer must be valid.
    #[inline]
    pub unsafe fn set_tile_acceleration_structure_ptr(
        &self,
        acceleration_structure: *const c_void,
        buffer_index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setTileAccelerationStructure: atBufferIndex:),
                acceleration_structure,
                buffer_index,
            );
        }
    }

    /// Set a tile acceleration structure.
    ///
    /// C++ equivalent: `void setTileAccelerationStructure(const AccelerationStructure*, NS::UInteger)`
    #[inline]
    pub fn set_tile_acceleration_structure(
        &self,
        acceleration_structure: &crate::AccelerationStructure,
        buffer_index: UInteger,
    ) {
        unsafe {
            self.set_tile_acceleration_structure_ptr(acceleration_structure.as_ptr(), buffer_index)
        };
    }

    // =========================================================================
    // Function Table Bindings
    // =========================================================================

    /// Set a vertex visible function table (raw pointer version).
    ///
    /// C++ equivalent: `void setVertexVisibleFunctionTable(const VisibleFunctionTable*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The function table pointer must be valid.
    #[inline]
    pub unsafe fn set_vertex_visible_function_table_ptr(
        &self,
        function_table: *const c_void,
        buffer_index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setVertexVisibleFunctionTable: atBufferIndex:),
                function_table,
                buffer_index,
            );
        }
    }

    /// Set multiple vertex visible function tables (raw pointer version).
    ///
    /// C++ equivalent: `void setVertexVisibleFunctionTables(const VisibleFunctionTable* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The function tables pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_vertex_visible_function_tables_ptr(
        &self,
        function_tables: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(setVertexVisibleFunctionTables: withBufferRange:),
                function_tables,
                range,
            );
        }
    }

    /// Set a vertex intersection function table (raw pointer version).
    ///
    /// C++ equivalent: `void setVertexIntersectionFunctionTable(const IntersectionFunctionTable*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The function table pointer must be valid.
    #[inline]
    pub unsafe fn set_vertex_intersection_function_table_ptr(
        &self,
        function_table: *const c_void,
        buffer_index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setVertexIntersectionFunctionTable: atBufferIndex:),
                function_table,
                buffer_index,
            );
        }
    }

    /// Set multiple vertex intersection function tables (raw pointer version).
    ///
    /// C++ equivalent: `void setVertexIntersectionFunctionTables(const IntersectionFunctionTable* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The function tables pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_vertex_intersection_function_tables_ptr(
        &self,
        function_tables: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(setVertexIntersectionFunctionTables: withBufferRange:),
                function_tables,
                range,
            );
        }
    }

    /// Set a fragment visible function table (raw pointer version).
    ///
    /// C++ equivalent: `void setFragmentVisibleFunctionTable(const VisibleFunctionTable*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The function table pointer must be valid.
    #[inline]
    pub unsafe fn set_fragment_visible_function_table_ptr(
        &self,
        function_table: *const c_void,
        buffer_index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setFragmentVisibleFunctionTable: atBufferIndex:),
                function_table,
                buffer_index,
            );
        }
    }

    /// Set multiple fragment visible function tables (raw pointer version).
    ///
    /// C++ equivalent: `void setFragmentVisibleFunctionTables(const VisibleFunctionTable* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The function tables pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_fragment_visible_function_tables_ptr(
        &self,
        function_tables: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(setFragmentVisibleFunctionTables: withBufferRange:),
                function_tables,
                range,
            );
        }
    }

    /// Set a fragment intersection function table (raw pointer version).
    ///
    /// C++ equivalent: `void setFragmentIntersectionFunctionTable(const IntersectionFunctionTable*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The function table pointer must be valid.
    #[inline]
    pub unsafe fn set_fragment_intersection_function_table_ptr(
        &self,
        function_table: *const c_void,
        buffer_index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setFragmentIntersectionFunctionTable: atBufferIndex:),
                function_table,
                buffer_index,
            );
        }
    }

    /// Set multiple fragment intersection function tables (raw pointer version).
    ///
    /// C++ equivalent: `void setFragmentIntersectionFunctionTables(const IntersectionFunctionTable* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The function tables pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_fragment_intersection_function_tables_ptr(
        &self,
        function_tables: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(setFragmentIntersectionFunctionTables: withBufferRange:),
                function_tables,
                range,
            );
        }
    }

    /// Set a tile visible function table (raw pointer version).
    ///
    /// C++ equivalent: `void setTileVisibleFunctionTable(const VisibleFunctionTable*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The function table pointer must be valid.
    #[inline]
    pub unsafe fn set_tile_visible_function_table_ptr(
        &self,
        function_table: *const c_void,
        buffer_index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setTileVisibleFunctionTable: atBufferIndex:),
                function_table,
                buffer_index,
            );
        }
    }

    /// Set multiple tile visible function tables (raw pointer version).
    ///
    /// C++ equivalent: `void setTileVisibleFunctionTables(const VisibleFunctionTable* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The function tables pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_tile_visible_function_tables_ptr(
        &self,
        function_tables: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(setTileVisibleFunctionTables: withBufferRange:),
                function_tables,
                range,
            );
        }
    }

    /// Set a tile intersection function table (raw pointer version).
    ///
    /// C++ equivalent: `void setTileIntersectionFunctionTable(const IntersectionFunctionTable*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The function table pointer must be valid.
    #[inline]
    pub unsafe fn set_tile_intersection_function_table_ptr(
        &self,
        function_table: *const c_void,
        buffer_index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setTileIntersectionFunctionTable: atBufferIndex:),
                function_table,
                buffer_index,
            );
        }
    }

    /// Set multiple tile intersection function tables (raw pointer version).
    ///
    /// C++ equivalent: `void setTileIntersectionFunctionTables(const IntersectionFunctionTable* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The function tables pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_tile_intersection_function_tables_ptr(
        &self,
        function_tables: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(setTileIntersectionFunctionTables: withBufferRange:),
                function_tables,
                range,
            );
        }
    }

    // =========================================================================
    // Indirect Command Execution
    // =========================================================================

    /// Execute commands from an indirect command buffer (raw pointer version).
    ///
    /// C++ equivalent: `void executeCommandsInBuffer(const IndirectCommandBuffer*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The indirect command buffer pointer must be valid.
    pub unsafe fn execute_commands_in_buffer_ptr(
        &self,
        indirect_command_buffer: *const c_void,
        offset: UInteger,
        length: UInteger,
    ) {
        let range = metal_foundation::Range::new(offset, length);
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(executeCommandsInBuffer: withRange:),
                indirect_command_buffer,
                range,
            );
        }
    }

    /// Execute commands from an indirect command buffer with indirect range (raw pointer version).
    ///
    /// C++ equivalent: `void executeCommandsInBuffer(const IndirectCommandBuffer*, const Buffer*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The indirect command buffer and range buffer pointers must be valid.
    pub unsafe fn execute_commands_in_buffer_with_indirect_range_ptr(
        &self,
        indirect_command_buffer: *const c_void,
        indirect_range_buffer: *const c_void,
        indirect_buffer_offset: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, *const c_void, UInteger>(
                self.as_ptr(),
                sel!(executeCommandsInBuffer: indirectBuffer: indirectBufferOffset:),
                indirect_command_buffer,
                indirect_range_buffer,
                indirect_buffer_offset,
            );
        }
    }

    // =========================================================================
    // Counter Sampling
    // =========================================================================

    /// Sample counters (raw pointer version).
    ///
    /// C++ equivalent: `void sampleCountersInBuffer(...)`
    ///
    /// # Safety
    ///
    /// The sample buffer pointer must be a valid counter sample buffer object.
    pub unsafe fn sample_counters_in_buffer_ptr(
        &self,
        sample_buffer: *const c_void,
        sample_index: UInteger,
        barrier: bool,
    ) {
        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, UInteger, bool>(
                self.as_ptr(),
                sel!(sampleCountersInBuffer: atSampleIndex: withBarrier:),
                sample_buffer,
                sample_index,
                barrier,
            );
        }
    }

    // =========================================================================
    // Color Attachment Map
    // =========================================================================

    /// Set the color attachment map (raw pointer version).
    ///
    /// C++ equivalent: `void setColorAttachmentMap(const LogicalToPhysicalColorAttachmentMap*)`
    ///
    /// # Safety
    ///
    /// The mapping pointer must be valid.
    #[inline]
    pub unsafe fn set_color_attachment_map_ptr(&self, mapping: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setColorAttachmentMap:), mapping);
        }
    }
}
