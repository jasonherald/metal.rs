//! MTL4 ComputeCommandEncoder implementation.
//!
//! Corresponds to `Metal/MTL4ComputeCommandEncoder.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, msg_send_3, msg_send_4, msg_send_5, sel};

use super::enums::VisibilityOptions;
use crate::{ComputePipelineState, Device, Size};

// ============================================================
// ComputeCommandEncoder
// ============================================================

/// MTL4 compute command encoder.
///
/// C++ equivalent: `MTL4::ComputeCommandEncoder`
///
/// ComputeCommandEncoder encodes compute dispatch commands and resource bindings.
#[repr(transparent)]
pub struct ComputeCommandEncoder(NonNull<c_void>);

impl ComputeCommandEncoder {
    /// Create a ComputeCommandEncoder from a raw pointer.
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

    /// Set the compute pipeline state.
    ///
    /// C++ equivalent: `void setComputePipelineState(const MTL::ComputePipelineState*)`
    pub fn set_compute_pipeline_state(&self, pipeline: &ComputePipelineState) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setComputePipelineState:),
                pipeline.as_ptr(),
            );
        }
    }

    // ========== Argument Table ==========

    /// Set the argument table at index.
    ///
    /// C++ equivalent: `void setArgumentTable(const MTL4::ArgumentTable*, NS::UInteger)`
    pub fn set_argument_table(&self, table: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(self.as_ptr(), sel!(setArgumentTable:atIndex:), table, index);
        }
    }

    // ========== Buffer Binding ==========

    /// Set a buffer at index with offset.
    ///
    /// C++ equivalent: `void setBuffer(const MTL::Buffer*, NS::UInteger, NS::UInteger)`
    pub fn set_buffer(&self, buffer: *const c_void, offset: UInteger, index: UInteger) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(setBuffer:offset:atIndex:),
                buffer,
                offset,
                index,
            );
        }
    }

    /// Set multiple buffers.
    ///
    /// C++ equivalent: `void setBuffers(const MTL::Buffer* const*, const NS::UInteger*, NS::Range)`
    pub fn set_buffers(
        &self,
        buffers: *const *const c_void,
        offsets: *const UInteger,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        unsafe {
            // Create NSRange struct
            let range = (range_location, range_length);
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(setBuffers:offsets:withRange:),
                buffers,
                offsets,
                range,
            );
        }
    }

    /// Set bytes at index.
    ///
    /// C++ equivalent: `void setBytes(const void*, NS::UInteger, NS::UInteger)`
    pub fn set_bytes(&self, bytes: *const c_void, length: UInteger, index: UInteger) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(setBytes:length:atIndex:),
                bytes,
                length,
                index,
            );
        }
    }

    // ========== Texture Binding ==========

    /// Set a texture at index.
    ///
    /// C++ equivalent: `void setTexture(const MTL::Texture*, NS::UInteger)`
    pub fn set_texture(&self, texture: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(self.as_ptr(), sel!(setTexture:atIndex:), texture, index);
        }
    }

    /// Set multiple textures.
    ///
    /// C++ equivalent: `void setTextures(const MTL::Texture* const*, NS::Range)`
    pub fn set_textures(
        &self,
        textures: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        unsafe {
            let range = (range_location, range_length);
            let _: () = msg_send_2(self.as_ptr(), sel!(setTextures:withRange:), textures, range);
        }
    }

    // ========== Sampler Binding ==========

    /// Set a sampler state at index.
    ///
    /// C++ equivalent: `void setSamplerState(const MTL::SamplerState*, NS::UInteger)`
    pub fn set_sampler_state(&self, sampler: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setSamplerState:atIndex:),
                sampler,
                index,
            );
        }
    }

    /// Set a sampler state with LOD clamp.
    ///
    /// C++ equivalent: `void setSamplerState(const MTL::SamplerState*, float, float, NS::UInteger)`
    pub fn set_sampler_state_with_lod(
        &self,
        sampler: *const c_void,
        lod_min_clamp: f32,
        lod_max_clamp: f32,
        index: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_4(
                self.as_ptr(),
                sel!(setSamplerState:lodMinClamp:lodMaxClamp:atIndex:),
                sampler,
                lod_min_clamp,
                lod_max_clamp,
                index,
            );
        }
    }

    // ========== Threadgroup Memory ==========

    /// Set threadgroup memory length at index.
    ///
    /// C++ equivalent: `void setThreadgroupMemoryLength(NS::UInteger, NS::UInteger)`
    pub fn set_threadgroup_memory_length(&self, length: UInteger, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setThreadgroupMemoryLength:atIndex:),
                length,
                index,
            );
        }
    }

    // ========== Dispatch Methods ==========

    /// Dispatch threadgroups.
    ///
    /// C++ equivalent: `void dispatchThreadgroups(MTL::Size, MTL::Size)`
    pub fn dispatch_threadgroups(
        &self,
        threadgroups_per_grid: Size,
        threads_per_threadgroup: Size,
    ) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(dispatchThreadgroups:threadsPerThreadgroup:),
                threadgroups_per_grid,
                threads_per_threadgroup,
            );
        }
    }

    /// Dispatch threads.
    ///
    /// C++ equivalent: `void dispatchThreads(MTL::Size, MTL::Size)`
    pub fn dispatch_threads(&self, threads_per_grid: Size, threads_per_threadgroup: Size) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(dispatchThreads:threadsPerThreadgroup:),
                threads_per_grid,
                threads_per_threadgroup,
            );
        }
    }

    /// Dispatch threadgroups with indirect buffer.
    ///
    /// C++ equivalent: `void dispatchThreadgroups(const MTL::Buffer*, NS::UInteger, MTL::Size)`
    pub fn dispatch_threadgroups_indirect(
        &self,
        indirect_buffer: *const c_void,
        indirect_buffer_offset: UInteger,
        threads_per_threadgroup: Size,
    ) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(dispatchThreadgroupsWithIndirectBuffer:indirectBufferOffset:threadsPerThreadgroup:),
                indirect_buffer,
                indirect_buffer_offset,
                threads_per_threadgroup,
            );
        }
    }

    /// Dispatch threads with indirect buffer.
    ///
    /// C++ equivalent: `void dispatchThreads(const MTL::Buffer*, NS::UInteger, MTL::Size)`
    pub fn dispatch_threads_indirect(
        &self,
        indirect_buffer: *const c_void,
        indirect_buffer_offset: UInteger,
        threads_per_threadgroup: Size,
    ) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(dispatchThreadsWithIndirectBuffer:indirectBufferOffset:threadsPerThreadgroup:),
                indirect_buffer,
                indirect_buffer_offset,
                threads_per_threadgroup,
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

    /// Use resource with usage.
    ///
    /// C++ equivalent: `void useResource(const MTL::Resource*, MTL::ResourceUsage)`
    pub fn use_resource(&self, resource: *const c_void, usage: UInteger) {
        unsafe {
            let _: () = msg_send_2(self.as_ptr(), sel!(useResource:usage:), resource, usage);
        }
    }

    /// Use multiple resources.
    ///
    /// C++ equivalent: `void useResources(const MTL::Resource* const*, NS::UInteger, MTL::ResourceUsage)`
    pub fn use_resources(&self, resources: *const *const c_void, count: UInteger, usage: UInteger) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(useResources:count:usage:),
                resources,
                count,
                usage,
            );
        }
    }

    /// Use heap with usage.
    ///
    /// C++ equivalent: `void useHeap(const MTL::Heap*, MTL::ResourceUsage)`
    pub fn use_heap(&self, heap: *const c_void, usage: UInteger) {
        unsafe {
            let _: () = msg_send_2(self.as_ptr(), sel!(useHeap:usage:), heap, usage);
        }
    }

    /// Use multiple heaps.
    ///
    /// C++ equivalent: `void useHeaps(const MTL::Heap* const*, NS::UInteger, MTL::ResourceUsage)`
    pub fn use_heaps(&self, heaps: *const *const c_void, count: UInteger, usage: UInteger) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(useHeaps:count:usage:),
                heaps,
                count,
                usage,
            );
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

    // ========== Encoding ==========

    /// End encoding.
    ///
    /// C++ equivalent: `void endEncoding()`
    pub fn end_encoding(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(endEncoding));
        }
    }

    // ========== Copy Operations ==========

    /// Copy from buffer to buffer.
    ///
    /// C++ equivalent: `void copyFromBuffer(...)`
    pub fn copy_from_buffer_to_buffer(
        &self,
        source_buffer: *const c_void,
        source_offset: UInteger,
        destination_buffer: *const c_void,
        destination_offset: UInteger,
        size: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_5(
                self.as_ptr(),
                sel!(copyFromBuffer:sourceOffset:toBuffer:destinationOffset:size:),
                source_buffer,
                source_offset,
                destination_buffer,
                destination_offset,
                size,
            );
        }
    }

    /// Fill buffer with value.
    ///
    /// C++ equivalent: `void fillBuffer(const MTL::Buffer*, NS::Range, uint8_t)`
    pub fn fill_buffer(
        &self,
        buffer: *const c_void,
        range_location: UInteger,
        range_length: UInteger,
        value: u8,
    ) {
        unsafe {
            let range = (range_location, range_length);
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(fillBuffer:range:value:),
                buffer,
                range,
                value,
            );
        }
    }

    // ========== Texture Operations ==========

    /// Generate mipmaps for a texture.
    ///
    /// C++ equivalent: `void generateMipmaps(const MTL::Texture*)`
    pub fn generate_mipmaps(&self, texture: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(generateMipmapsForTexture:), texture);
        }
    }

    /// Optimize texture contents for CPU access.
    ///
    /// C++ equivalent: `void optimizeContentsForCPUAccess(const MTL::Texture*)`
    pub fn optimize_contents_for_cpu_access(&self, texture: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(optimizeContentsForCPUAccess:), texture);
        }
    }

    /// Optimize texture contents for CPU access with slice and level.
    ///
    /// C++ equivalent: `void optimizeContentsForCPUAccess(const MTL::Texture*, NS::UInteger, NS::UInteger)`
    pub fn optimize_contents_for_cpu_access_slice_level(
        &self,
        texture: *const c_void,
        slice: UInteger,
        level: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(optimizeContentsForCPUAccess:slice:level:),
                texture,
                slice,
                level,
            );
        }
    }

    /// Optimize texture contents for GPU access.
    ///
    /// C++ equivalent: `void optimizeContentsForGPUAccess(const MTL::Texture*)`
    pub fn optimize_contents_for_gpu_access(&self, texture: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(optimizeContentsForGPUAccess:), texture);
        }
    }

    /// Optimize texture contents for GPU access with slice and level.
    ///
    /// C++ equivalent: `void optimizeContentsForGPUAccess(const MTL::Texture*, NS::UInteger, NS::UInteger)`
    pub fn optimize_contents_for_gpu_access_slice_level(
        &self,
        texture: *const c_void,
        slice: UInteger,
        level: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(optimizeContentsForGPUAccess:slice:level:),
                texture,
                slice,
                level,
            );
        }
    }

    // ========== Counter/Timestamp Methods ==========

    /// Write a timestamp to a counter heap.
    ///
    /// C++ equivalent: `void writeTimestamp(MTL4::TimestampGranularity, const MTL4::CounterHeap*, NS::UInteger)`
    pub fn write_timestamp(
        &self,
        granularity: super::TimestampGranularity,
        counter_heap: *const c_void,
        index: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(writeTimestampWithGranularity:intoHeap:atIndex:),
                granularity.0,
                counter_heap,
                index,
            );
        }
    }

    // ========== Acceleration Structure Methods ==========

    /// Build an acceleration structure.
    ///
    /// C++ equivalent: `void buildAccelerationStructure(const MTL::AccelerationStructure*, const MTL4::AccelerationStructureDescriptor*, const MTL4::BufferRange)`
    pub fn build_acceleration_structure(
        &self,
        acceleration_structure: &crate::AccelerationStructure,
        descriptor: &super::AccelerationStructureDescriptor,
        scratch_buffer: super::BufferRange,
    ) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(buildAccelerationStructure:descriptor:scratchBuffer:),
                acceleration_structure.as_ptr(),
                descriptor.as_ptr(),
                scratch_buffer,
            );
        }
    }

    /// Copy an acceleration structure.
    ///
    /// C++ equivalent: `void copyAccelerationStructure(const MTL::AccelerationStructure*, const MTL::AccelerationStructure*)`
    pub fn copy_acceleration_structure(
        &self,
        source: &crate::AccelerationStructure,
        destination: &crate::AccelerationStructure,
    ) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(copyAccelerationStructure:toAccelerationStructure:),
                source.as_ptr(),
                destination.as_ptr(),
            );
        }
    }

    /// Copy and compact an acceleration structure.
    ///
    /// C++ equivalent: `void copyAndCompactAccelerationStructure(const MTL::AccelerationStructure*, const MTL::AccelerationStructure*)`
    pub fn copy_and_compact_acceleration_structure(
        &self,
        source: &crate::AccelerationStructure,
        destination: &crate::AccelerationStructure,
    ) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(copyAndCompactAccelerationStructure:toAccelerationStructure:),
                source.as_ptr(),
                destination.as_ptr(),
            );
        }
    }

    /// Refit an acceleration structure.
    ///
    /// C++ equivalent: `void refitAccelerationStructure(const MTL::AccelerationStructure*, const MTL4::AccelerationStructureDescriptor*, const MTL::AccelerationStructure*, const MTL4::BufferRange)`
    pub fn refit_acceleration_structure(
        &self,
        source: &crate::AccelerationStructure,
        descriptor: &super::AccelerationStructureDescriptor,
        destination: &crate::AccelerationStructure,
        scratch_buffer: super::BufferRange,
    ) {
        unsafe {
            let _: () = msg_send_4(
                self.as_ptr(),
                sel!(refitAccelerationStructure:descriptor:destination:scratchBuffer:),
                source.as_ptr(),
                descriptor.as_ptr(),
                destination.as_ptr(),
                scratch_buffer,
            );
        }
    }

    /// Refit an acceleration structure with options.
    ///
    /// C++ equivalent: `void refitAccelerationStructure(const MTL::AccelerationStructure*, const MTL4::AccelerationStructureDescriptor*, const MTL::AccelerationStructure*, const MTL4::BufferRange, MTL::AccelerationStructureRefitOptions)`
    pub fn refit_acceleration_structure_with_options(
        &self,
        source: &crate::AccelerationStructure,
        descriptor: &super::AccelerationStructureDescriptor,
        destination: &crate::AccelerationStructure,
        scratch_buffer: super::BufferRange,
        options: crate::AccelerationStructureRefitOptions,
    ) {
        unsafe {
            let _: () = msg_send_5(
                self.as_ptr(),
                sel!(refitAccelerationStructure:descriptor:destination:scratchBuffer:options:),
                source.as_ptr(),
                descriptor.as_ptr(),
                destination.as_ptr(),
                scratch_buffer,
                options,
            );
        }
    }

    /// Write the compacted acceleration structure size to a buffer.
    ///
    /// C++ equivalent: `void writeCompactedAccelerationStructureSize(const MTL::AccelerationStructure*, const MTL4::BufferRange)`
    pub fn write_compacted_acceleration_structure_size(
        &self,
        acceleration_structure: &crate::AccelerationStructure,
        buffer: super::BufferRange,
    ) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(writeCompactedAccelerationStructureSize:toBuffer:),
                acceleration_structure.as_ptr(),
                buffer,
            );
        }
    }

    // ========== Tensor Copy Methods ==========

    /// Copy from tensor to tensor.
    ///
    /// C++ equivalent: `void copyFromTensor(const MTL::Tensor*, const MTL::TensorExtents*, const MTL::TensorExtents*, const MTL::Tensor*, const MTL::TensorExtents*, const MTL::TensorExtents*)`
    pub fn copy_from_tensor(
        &self,
        source_tensor: &crate::Tensor,
        source_origin: &crate::TensorExtents,
        source_dimensions: &crate::TensorExtents,
        destination_tensor: &crate::Tensor,
        destination_origin: &crate::TensorExtents,
        destination_dimensions: &crate::TensorExtents,
    ) {
        unsafe {
            metal_sys::msg_send_6::<
                (),
                *const c_void,
                *const c_void,
                *const c_void,
                *const c_void,
                *const c_void,
                *const c_void,
            >(
                self.as_ptr(),
                sel!(copyFromTensor:sourceOrigin:sourceDimensions:toTensor:destinationOrigin:destinationDimensions:),
                source_tensor.as_ptr(),
                source_origin.as_ptr(),
                source_dimensions.as_ptr(),
                destination_tensor.as_ptr(),
                destination_origin.as_ptr(),
                destination_dimensions.as_ptr(),
            );
        }
    }

    // ========== Texture Copy Methods ==========

    /// Copy from texture to texture (simple version).
    ///
    /// C++ equivalent: `void copyFromTexture(const MTL::Texture*, const MTL::Texture*)`
    pub fn copy_from_texture_to_texture(
        &self,
        source_texture: &crate::Texture,
        destination_texture: &crate::Texture,
    ) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(copyFromTexture:toTexture:),
                source_texture.as_ptr(),
                destination_texture.as_ptr(),
            );
        }
    }

    /// Copy from texture to texture with slice and level ranges.
    ///
    /// C++ equivalent: `void copyFromTexture(const MTL::Texture*, NS::UInteger, NS::UInteger, const MTL::Texture*, NS::UInteger, NS::UInteger, NS::UInteger, NS::UInteger)`
    pub fn copy_from_texture_with_slices(
        &self,
        source_texture: &crate::Texture,
        source_slice: UInteger,
        source_level: UInteger,
        destination_texture: &crate::Texture,
        destination_slice: UInteger,
        destination_level: UInteger,
        slice_count: UInteger,
        level_count: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_8::<
                (),
                *const c_void,
                UInteger,
                UInteger,
                *const c_void,
                UInteger,
                UInteger,
                UInteger,
                UInteger,
            >(
                self.as_ptr(),
                sel!(copyFromTexture:sourceSlice:sourceLevel:toTexture:destinationSlice:destinationLevel:sliceCount:levelCount:),
                source_texture.as_ptr(),
                source_slice,
                source_level,
                destination_texture.as_ptr(),
                destination_slice,
                destination_level,
                slice_count,
                level_count,
            );
        }
    }

    /// Copy from texture to texture with origin and size.
    ///
    /// C++ equivalent: `void copyFromTexture(const MTL::Texture*, NS::UInteger, NS::UInteger, MTL::Origin, MTL::Size, const MTL::Texture*, NS::UInteger, NS::UInteger, MTL::Origin)`
    pub fn copy_from_texture_with_origin(
        &self,
        source_texture: &crate::Texture,
        source_slice: UInteger,
        source_level: UInteger,
        source_origin: crate::Origin,
        source_size: Size,
        destination_texture: &crate::Texture,
        destination_slice: UInteger,
        destination_level: UInteger,
        destination_origin: crate::Origin,
    ) {
        unsafe {
            metal_sys::msg_send_9::<
                (),
                *const c_void,
                UInteger,
                UInteger,
                crate::Origin,
                Size,
                *const c_void,
                UInteger,
                UInteger,
                crate::Origin,
            >(
                self.as_ptr(),
                sel!(copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:),
                source_texture.as_ptr(),
                source_slice,
                source_level,
                source_origin,
                source_size,
                destination_texture.as_ptr(),
                destination_slice,
                destination_level,
                destination_origin,
            );
        }
    }

    /// Copy from texture to buffer.
    ///
    /// C++ equivalent: `void copyFromTexture(const MTL::Texture*, NS::UInteger, NS::UInteger, MTL::Origin, MTL::Size, const MTL::Buffer*, NS::UInteger, NS::UInteger, NS::UInteger)`
    pub fn copy_from_texture_to_buffer(
        &self,
        source_texture: &crate::Texture,
        source_slice: UInteger,
        source_level: UInteger,
        source_origin: crate::Origin,
        source_size: Size,
        destination_buffer: &crate::Buffer,
        destination_offset: UInteger,
        destination_bytes_per_row: UInteger,
        destination_bytes_per_image: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_9::<
                (),
                *const c_void,
                UInteger,
                UInteger,
                crate::Origin,
                Size,
                *const c_void,
                UInteger,
                UInteger,
                UInteger,
            >(
                self.as_ptr(),
                sel!(copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:),
                source_texture.as_ptr(),
                source_slice,
                source_level,
                source_origin,
                source_size,
                destination_buffer.as_ptr(),
                destination_offset,
                destination_bytes_per_row,
                destination_bytes_per_image,
            );
        }
    }

    /// Copy from texture to buffer with blit options.
    ///
    /// C++ equivalent: `void copyFromTexture(const MTL::Texture*, NS::UInteger, NS::UInteger, MTL::Origin, MTL::Size, const MTL::Buffer*, NS::UInteger, NS::UInteger, NS::UInteger, MTL::BlitOption)`
    pub fn copy_from_texture_to_buffer_with_options(
        &self,
        source_texture: &crate::Texture,
        source_slice: UInteger,
        source_level: UInteger,
        source_origin: crate::Origin,
        source_size: Size,
        destination_buffer: &crate::Buffer,
        destination_offset: UInteger,
        destination_bytes_per_row: UInteger,
        destination_bytes_per_image: UInteger,
        options: crate::BlitOption,
    ) {
        unsafe {
            metal_sys::msg_send_10::<
                (),
                *const c_void,
                UInteger,
                UInteger,
                crate::Origin,
                Size,
                *const c_void,
                UInteger,
                UInteger,
                UInteger,
                crate::BlitOption,
            >(
                self.as_ptr(),
                sel!(copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:options:),
                source_texture.as_ptr(),
                source_slice,
                source_level,
                source_origin,
                source_size,
                destination_buffer.as_ptr(),
                destination_offset,
                destination_bytes_per_row,
                destination_bytes_per_image,
                options,
            );
        }
    }

    // ========== Indirect Command Buffer Methods ==========

    /// Copy indirect command buffer.
    ///
    /// C++ equivalent: `void copyIndirectCommandBuffer(const MTL::IndirectCommandBuffer*, NS::Range, const MTL::IndirectCommandBuffer*, NS::UInteger)`
    pub fn copy_indirect_command_buffer(
        &self,
        source: &crate::IndirectCommandBuffer,
        source_range_location: UInteger,
        source_range_length: UInteger,
        destination: &crate::IndirectCommandBuffer,
        destination_index: UInteger,
    ) {
        unsafe {
            let range = metal_foundation::Range::new(source_range_location, source_range_length);
            let _: () = msg_send_4(
                self.as_ptr(),
                sel!(copyIndirectCommandBuffer:sourceRange:destination:destinationIndex:),
                source.as_ptr(),
                range,
                destination.as_ptr(),
                destination_index,
            );
        }
    }

    /// Optimize indirect command buffer.
    ///
    /// C++ equivalent: `void optimizeIndirectCommandBuffer(const MTL::IndirectCommandBuffer*, NS::Range)`
    pub fn optimize_indirect_command_buffer(
        &self,
        indirect_command_buffer: &crate::IndirectCommandBuffer,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        unsafe {
            let range = metal_foundation::Range::new(range_location, range_length);
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(optimizeIndirectCommandBuffer:withRange:),
                indirect_command_buffer.as_ptr(),
                range,
            );
        }
    }

    /// Reset commands in an indirect command buffer.
    ///
    /// C++ equivalent: `void resetCommandsInBuffer(const MTL::IndirectCommandBuffer*, NS::Range)`
    pub fn reset_commands_in_buffer(
        &self,
        buffer: &crate::IndirectCommandBuffer,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        unsafe {
            let range = metal_foundation::Range::new(range_location, range_length);
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(resetCommandsInBuffer:withRange:),
                buffer.as_ptr(),
                range,
            );
        }
    }

    /// Execute commands in an indirect command buffer.
    ///
    /// C++ equivalent: `void executeCommandsInBuffer(const MTL::IndirectCommandBuffer*, NS::Range)`
    pub fn execute_commands_in_buffer(
        &self,
        indirect_command_buffer: &crate::IndirectCommandBuffer,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        unsafe {
            let range = metal_foundation::Range::new(range_location, range_length);
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(executeCommandsInBuffer:withRange:),
                indirect_command_buffer.as_ptr(),
                range,
            );
        }
    }

    /// Execute commands in an indirect command buffer with indirect range.
    ///
    /// C++ equivalent: `void executeCommandsInBuffer(const MTL::IndirectCommandBuffer*, MTL::GPUAddress)`
    pub fn execute_commands_in_buffer_indirect(
        &self,
        indirect_command_buffer: &crate::IndirectCommandBuffer,
        indirect_range_buffer: u64,
    ) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(executeCommandsInBuffer:indirectBuffer:),
                indirect_command_buffer.as_ptr(),
                indirect_range_buffer,
            );
        }
    }

    // ========== Stage Information ==========

    /// Get the stages supported by this encoder.
    ///
    /// C++ equivalent: `MTL::Stages stages()`
    pub fn stages(&self) -> crate::Stages {
        unsafe { msg_send_0(self.as_ptr(), sel!(stages)) }
    }
}

impl Clone for ComputeCommandEncoder {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ComputeCommandEncoder {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for ComputeCommandEncoder {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ComputeCommandEncoder {}
unsafe impl Sync for ComputeCommandEncoder {}

impl std::fmt::Debug for ComputeCommandEncoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComputeCommandEncoder")
            .field("label", &self.label())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_command_encoder_size() {
        assert_eq!(
            std::mem::size_of::<ComputeCommandEncoder>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
