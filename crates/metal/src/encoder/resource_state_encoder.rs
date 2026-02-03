//! Resource state command encoder.
//!
//! Corresponds to `Metal/MTLResourceStateCommandEncoder.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::Buffer;
use crate::Texture;
use crate::enums::SparseTextureMappingMode;
use crate::sync::Fence;
use crate::types::{Origin, Region, Size};

/// Arguments for indirect texture mapping operations.
///
/// C++ equivalent: `MTL::MapIndirectArguments`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default)]
pub struct MapIndirectArguments {
    /// X origin of the region.
    pub region_origin_x: u32,
    /// Y origin of the region.
    pub region_origin_y: u32,
    /// Z origin of the region.
    pub region_origin_z: u32,
    /// Width of the region.
    pub region_size_width: u32,
    /// Height of the region.
    pub region_size_height: u32,
    /// Depth of the region.
    pub region_size_depth: u32,
    /// Mipmap level.
    pub mip_map_level: u32,
    /// Slice ID.
    pub slice_id: u32,
}

/// A command encoder for resource state operations.
///
/// C++ equivalent: `MTL::ResourceStateCommandEncoder`
///
/// Resource state encoders are used to update sparse texture mappings
/// and manage resource state for sparse resources.
#[repr(transparent)]
pub struct ResourceStateCommandEncoder(pub(crate) NonNull<c_void>);

impl ResourceStateCommandEncoder {
    /// Create a ResourceStateCommandEncoder from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal resource state command encoder object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the encoder.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // CommandEncoder base methods
    // =========================================================================

    /// Get the device that created this encoder.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("encoder has no device")
        }
    }

    /// Get the command buffer that this encoder is encoding into.
    ///
    /// C++ equivalent: `CommandBuffer* commandBuffer() const`
    pub fn command_buffer(&self) -> crate::CommandBuffer {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(commandBuffer));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::CommandBuffer::from_raw(ptr).expect("encoder has no command buffer")
        }
    }

    /// Get the label for this encoder.
    ///
    /// C++ equivalent: `NS::String* label() const`
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

    /// Set the label for this encoder.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// End encoding commands with this encoder.
    ///
    /// C++ equivalent: `void endEncoding()`
    #[inline]
    pub fn end_encoding(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(endEncoding));
        }
    }

    /// Insert a debug signpost.
    ///
    /// C++ equivalent: `void insertDebugSignpost(const NS::String*)`
    pub fn insert_debug_signpost(&self, string: &str) {
        if let Some(ns_string) = metal_foundation::String::from_str(string) {
            unsafe {
                msg_send_1::<(), *const c_void>(
                    self.as_ptr(),
                    sel!(insertDebugSignpost:),
                    ns_string.as_ptr(),
                );
            }
        }
    }

    /// Push a debug group.
    ///
    /// C++ equivalent: `void pushDebugGroup(const NS::String*)`
    pub fn push_debug_group(&self, string: &str) {
        if let Some(ns_string) = metal_foundation::String::from_str(string) {
            unsafe {
                msg_send_1::<(), *const c_void>(
                    self.as_ptr(),
                    sel!(pushDebugGroup:),
                    ns_string.as_ptr(),
                );
            }
        }
    }

    /// Pop the current debug group.
    ///
    /// C++ equivalent: `void popDebugGroup()`
    #[inline]
    pub fn pop_debug_group(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(popDebugGroup));
        }
    }

    /// Insert a barrier to synchronize queue stages.
    ///
    /// C++ equivalent: `void barrierAfterQueueStages(Stages, Stages)`
    #[inline]
    pub fn barrier_after_queue_stages(
        &self,
        after_stages: crate::enums::Stages,
        before_stages: crate::enums::Stages,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), crate::enums::Stages, crate::enums::Stages>(
                self.as_ptr(),
                sel!(barrierAfterQueueStages:beforeQueueStages:),
                after_stages,
                before_stages,
            );
        }
    }

    // =========================================================================
    // Texture Mapping Operations
    // =========================================================================

    /// Update sparse texture mapping for a region.
    ///
    /// C++ equivalent: `void updateTextureMapping(const Texture*, SparseTextureMappingMode, Region, NS::UInteger, NS::UInteger)`
    pub fn update_texture_mapping(
        &self,
        texture: &Texture,
        mode: SparseTextureMappingMode,
        region: Region,
        mip_level: UInteger,
        slice: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_5::<
                (),
                *const c_void,
                SparseTextureMappingMode,
                Region,
                UInteger,
                UInteger,
            >(
                self.as_ptr(),
                sel!(updateTextureMapping: mode: region: mipLevel: slice:),
                texture.as_ptr(),
                mode,
                region,
                mip_level,
                slice,
            );
        }
    }

    /// Update sparse texture mapping using indirect arguments from a buffer.
    ///
    /// C++ equivalent: `void updateTextureMapping(const Texture*, SparseTextureMappingMode, const Buffer*, NS::UInteger)`
    pub fn update_texture_mapping_indirect(
        &self,
        texture: &Texture,
        mode: SparseTextureMappingMode,
        indirect_buffer: &Buffer,
        indirect_buffer_offset: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_4::<
                (),
                *const c_void,
                SparseTextureMappingMode,
                *const c_void,
                UInteger,
            >(
                self.as_ptr(),
                sel!(updateTextureMapping: mode: indirectBuffer: indirectBufferOffset:),
                texture.as_ptr(),
                mode,
                indirect_buffer.as_ptr(),
                indirect_buffer_offset,
            );
        }
    }

    /// Update sparse texture mappings for multiple regions.
    ///
    /// C++ equivalent: `void updateTextureMappings(const Texture*, SparseTextureMappingMode, const Region*, const NS::UInteger*, const NS::UInteger*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The regions, mip_levels, and slices arrays must have at least `num_regions` elements.
    pub unsafe fn update_texture_mappings(
        &self,
        texture: &Texture,
        mode: SparseTextureMappingMode,
        regions: *const Region,
        mip_levels: *const UInteger,
        slices: *const UInteger,
        num_regions: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_6::<
                (),
                *const c_void,
                SparseTextureMappingMode,
                *const Region,
                *const UInteger,
                *const UInteger,
                UInteger,
            >(
                self.as_ptr(),
                sel!(updateTextureMappings: mode: regions: mipLevels: slices: numRegions:),
                texture.as_ptr(),
                mode,
                regions,
                mip_levels,
                slices,
                num_regions,
            );
        }
    }

    /// Move texture mappings from one texture to another.
    ///
    /// C++ equivalent: `void moveTextureMappingsFromTexture(...)`
    #[allow(clippy::too_many_arguments)]
    pub fn move_texture_mappings_from_texture(
        &self,
        source_texture: &Texture,
        source_slice: UInteger,
        source_level: UInteger,
        source_origin: Origin,
        source_size: Size,
        destination_texture: &Texture,
        destination_slice: UInteger,
        destination_level: UInteger,
        destination_origin: Origin,
    ) {
        unsafe {
            metal_sys::msg_send_9::<
                (),
                *const c_void,
                UInteger,
                UInteger,
                Origin,
                Size,
                *const c_void,
                UInteger,
                UInteger,
                Origin,
            >(
                self.as_ptr(),
                sel!(moveTextureMappingsFromTexture: sourceSlice: sourceLevel: sourceOrigin: sourceSize: toTexture: destinationSlice: destinationLevel: destinationOrigin:),
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

    // =========================================================================
    // Fence Operations
    // =========================================================================

    /// Update a fence.
    ///
    /// C++ equivalent: `void updateFence(const Fence*)`
    #[inline]
    pub fn update_fence(&self, fence: &Fence) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(updateFence:), fence.as_ptr());
        }
    }

    /// Wait for a fence.
    ///
    /// C++ equivalent: `void waitForFence(const Fence*)`
    #[inline]
    pub fn wait_for_fence(&self, fence: &Fence) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(waitForFence:), fence.as_ptr());
        }
    }
}

impl Clone for ResourceStateCommandEncoder {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ResourceStateCommandEncoder {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for ResourceStateCommandEncoder {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ResourceStateCommandEncoder {}
unsafe impl Sync for ResourceStateCommandEncoder {}

impl std::fmt::Debug for ResourceStateCommandEncoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResourceStateCommandEncoder")
            .field("label", &self.label())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_state_encoder_size() {
        assert_eq!(
            std::mem::size_of::<ResourceStateCommandEncoder>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_map_indirect_arguments_size() {
        // 8 u32 fields = 32 bytes
        assert_eq!(std::mem::size_of::<MapIndirectArguments>(), 32);
    }
}
