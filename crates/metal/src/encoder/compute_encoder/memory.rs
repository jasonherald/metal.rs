//! Memory barrier and fence methods for ComputeCommandEncoder.

use std::ffi::c_void;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_1, sel};

use crate::Buffer;
use crate::Texture;
use crate::enums::{BarrierScope, ResourceUsage};

use super::ComputeCommandEncoder;

impl ComputeCommandEncoder {
    // =========================================================================
    // Memory Barriers
    // =========================================================================

    /// Insert a memory barrier with scope.
    ///
    /// C++ equivalent: `void memoryBarrier(MTL::BarrierScope)`
    #[inline]
    pub fn memory_barrier_with_scope(&self, scope: BarrierScope) {
        unsafe {
            msg_send_1::<(), BarrierScope>(self.as_ptr(), sel!(memoryBarrierWithScope:), scope);
        }
    }

    /// Insert a memory barrier for specific resources (raw pointer version).
    ///
    /// C++ equivalent: `void memoryBarrier(const Resource* const*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The resources pointer must point to a valid array of resource pointers with the specified count.
    #[inline]
    pub unsafe fn memory_barrier_with_resources_ptr(
        &self,
        resources: *const *const c_void,
        count: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, UInteger>(
                self.as_ptr(),
                sel!(memoryBarrierWithResources: count:),
                resources,
                count,
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

    // =========================================================================
    // Fence Operations
    // =========================================================================

    /// Update a fence (raw pointer version).
    ///
    /// C++ equivalent: `void updateFence(const Fence*)`
    ///
    /// # Safety
    ///
    /// The fence pointer must be a valid Metal fence object.
    #[inline]
    pub unsafe fn update_fence_ptr(&self, fence: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(updateFence:), fence);
        }
    }

    /// Update a fence.
    ///
    /// C++ equivalent: `void updateFence(const Fence*)`
    #[inline]
    pub fn update_fence(&self, fence: &crate::Fence) {
        unsafe { self.update_fence_ptr(fence.as_ptr()) };
    }

    /// Wait for a fence (raw pointer version).
    ///
    /// C++ equivalent: `void waitForFence(const Fence*)`
    ///
    /// # Safety
    ///
    /// The fence pointer must be a valid Metal fence object.
    #[inline]
    pub unsafe fn wait_for_fence_ptr(&self, fence: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(waitForFence:), fence);
        }
    }

    /// Wait for a fence.
    ///
    /// C++ equivalent: `void waitForFence(const Fence*)`
    #[inline]
    pub fn wait_for_fence(&self, fence: &crate::Fence) {
        unsafe { self.wait_for_fence_ptr(fence.as_ptr()) };
    }
}
