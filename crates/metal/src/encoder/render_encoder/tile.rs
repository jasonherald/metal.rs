//! Tile shader bindings and properties.
//!
//! This module contains methods for binding resources to tile shaders
//! and querying tile properties.

use std::ffi::c_void;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, sel};

use crate::Buffer;
use crate::Texture;

use super::RenderCommandEncoder;

impl RenderCommandEncoder {
    // =========================================================================
    // Tile Bindings
    // =========================================================================

    /// Set a tile buffer.
    ///
    /// C++ equivalent: `void setTileBuffer(const Buffer*, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_tile_buffer(&self, buffer: &Buffer, offset: UInteger, index: UInteger) {
        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setTileBuffer: offset: atIndex:),
                buffer.as_ptr(),
                offset,
                index,
            );
        }
    }

    /// Set a tile texture.
    ///
    /// C++ equivalent: `void setTileTexture(const Texture*, NS::UInteger)`
    #[inline]
    pub fn set_tile_texture(&self, texture: &Texture, index: UInteger) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setTileTexture: atIndex:),
                texture.as_ptr(),
                index,
            );
        }
    }

    /// Set a tile sampler state.
    ///
    /// C++ equivalent: `void setTileSamplerState(const SamplerState*, NS::UInteger)`
    #[inline]
    pub fn set_tile_sampler_state(&self, sampler: &crate::SamplerState, index: UInteger) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setTileSamplerState: atIndex:),
                sampler.as_ptr(),
                index,
            );
        }
    }

    /// Set inline tile bytes.
    ///
    /// C++ equivalent: `void setTileBytes(const void*, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_tile_bytes(&self, bytes: &[u8], index: UInteger) {
        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setTileBytes: length: atIndex:),
                bytes.as_ptr() as *const c_void,
                bytes.len() as UInteger,
                index,
            );
        }
    }

    /// Set the tile buffer offset.
    ///
    /// C++ equivalent: `void setTileBufferOffset(NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_tile_buffer_offset(&self, offset: UInteger, index: UInteger) {
        unsafe {
            metal_sys::msg_send_2::<(), UInteger, UInteger>(
                self.as_ptr(),
                sel!(setTileBufferOffset: atIndex:),
                offset,
                index,
            );
        }
    }

    /// Set multiple tile buffers at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setTileBuffers(const Buffer* const*, const NS::UInteger*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The buffers and offsets pointers must be valid arrays with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_tile_buffers_ptr(
        &self,
        buffers: *const *const c_void,
        offsets: *const UInteger,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_3::<(), *const *const c_void, *const UInteger, metal_foundation::Range>(
                self.as_ptr(),
                sel!(setTileBuffers: offsets: withRange:),
                buffers,
                offsets,
                range,
            );
        }
    }

    /// Set multiple tile textures at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setTileTextures(const Texture* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The textures pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_tile_textures_ptr(
        &self,
        textures: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(setTileTextures: withRange:),
                textures,
                range,
            );
        }
    }

    /// Set a tile sampler state with LOD clamps.
    ///
    /// C++ equivalent: `void setTileSamplerState(const SamplerState*, float, float, NS::UInteger)`
    #[inline]
    pub fn set_tile_sampler_state_with_lod_clamps(
        &self,
        sampler: &crate::SamplerState,
        lod_min_clamp: f32,
        lod_max_clamp: f32,
        index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_4::<(), *const c_void, f32, f32, UInteger>(
                self.as_ptr(),
                sel!(setTileSamplerState: lodMinClamp: lodMaxClamp: atIndex:),
                sampler.as_ptr(),
                lod_min_clamp,
                lod_max_clamp,
                index,
            );
        }
    }

    /// Set multiple tile sampler states at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setTileSamplerStates(const SamplerState* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The samplers pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_tile_sampler_states_ptr(
        &self,
        samplers: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(setTileSamplerStates: withRange:),
                samplers,
                range,
            );
        }
    }

    /// Set multiple tile sampler states with LOD clamps at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setTileSamplerStates(const SamplerState* const*, const float*, const float*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The samplers, lod_min_clamps, and lod_max_clamps pointers must be valid arrays with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_tile_sampler_states_with_lod_clamps_ptr(
        &self,
        samplers: *const *const c_void,
        lod_min_clamps: *const f32,
        lod_max_clamps: *const f32,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_4::<(), *const *const c_void, *const f32, *const f32, metal_foundation::Range>(
                self.as_ptr(),
                sel!(setTileSamplerStates: lodMinClamps: lodMaxClamps: withRange:),
                samplers,
                lod_min_clamps,
                lod_max_clamps,
                range,
            );
        }
    }

    // =========================================================================
    // Tile Properties
    // =========================================================================

    /// Get the tile width.
    ///
    /// C++ equivalent: `NS::UInteger tileWidth() const`
    #[inline]
    pub fn tile_width(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(tileWidth)) }
    }

    /// Get the tile height.
    ///
    /// C++ equivalent: `NS::UInteger tileHeight() const`
    #[inline]
    pub fn tile_height(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(tileHeight)) }
    }

    // =========================================================================
    // Threadgroup Memory
    // =========================================================================

    /// Set the threadgroup memory length at an index with offset.
    ///
    /// C++ equivalent: `void setThreadgroupMemoryLength(NS::UInteger, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_threadgroup_memory_length(
        &self,
        length: UInteger,
        offset: UInteger,
        index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_3::<(), UInteger, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setThreadgroupMemoryLength: offset: atIndex:),
                length,
                offset,
                index,
            );
        }
    }
}
