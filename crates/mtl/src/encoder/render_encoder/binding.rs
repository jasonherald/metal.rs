//! Resource binding methods for vertex and fragment stages.
//!
//! This module contains methods for binding buffers, textures, and samplers
//! to both vertex and fragment shader stages.

use std::ffi::c_void;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::sel;

use crate::Buffer;
use crate::Texture;

use super::RenderCommandEncoder;

impl RenderCommandEncoder {
    // Vertex Buffers
    // =========================================================================

    /// Set a vertex buffer.
    ///
    /// C++ equivalent: `void setVertexBuffer(const Buffer*, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_vertex_buffer(&self, buffer: &Buffer, offset: UInteger, index: UInteger) {
        unsafe {
            mtl_sys::msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setVertexBuffer: offset: atIndex:),
                buffer.as_ptr(),
                offset,
                index,
            );
        }
    }

    /// Set a vertex buffer with attribute stride.
    ///
    /// C++ equivalent: `void setVertexBuffer(const Buffer*, NS::UInteger, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_vertex_buffer_with_stride(
        &self,
        buffer: &Buffer,
        offset: UInteger,
        stride: UInteger,
        index: UInteger,
    ) {
        unsafe {
            mtl_sys::msg_send_4::<(), *const c_void, UInteger, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setVertexBuffer: offset: attributeStride: atIndex:),
                buffer.as_ptr(),
                offset,
                stride,
                index,
            );
        }
    }

    /// Set the vertex buffer offset.
    ///
    /// C++ equivalent: `void setVertexBufferOffset(NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_vertex_buffer_offset(&self, offset: UInteger, index: UInteger) {
        unsafe {
            mtl_sys::msg_send_2::<(), UInteger, UInteger>(
                self.as_ptr(),
                sel!(setVertexBufferOffset: atIndex:),
                offset,
                index,
            );
        }
    }

    /// Set inline vertex bytes.
    ///
    /// C++ equivalent: `void setVertexBytes(const void*, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_vertex_bytes(&self, bytes: &[u8], index: UInteger) {
        unsafe {
            mtl_sys::msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setVertexBytes: length: atIndex:),
                bytes.as_ptr() as *const c_void,
                bytes.len() as UInteger,
                index,
            );
        }
    }

    /// Set inline vertex bytes with attribute stride.
    ///
    /// C++ equivalent: `void setVertexBytes(const void*, NS::UInteger, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_vertex_bytes_with_stride(&self, bytes: &[u8], stride: UInteger, index: UInteger) {
        unsafe {
            mtl_sys::msg_send_4::<(), *const c_void, UInteger, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setVertexBytes: length: attributeStride: atIndex:),
                bytes.as_ptr() as *const c_void,
                bytes.len() as UInteger,
                stride,
                index,
            );
        }
    }

    /// Set the vertex buffer offset with attribute stride.
    ///
    /// C++ equivalent: `void setVertexBufferOffset(NS::UInteger, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_vertex_buffer_offset_with_stride(
        &self,
        offset: UInteger,
        stride: UInteger,
        index: UInteger,
    ) {
        unsafe {
            mtl_sys::msg_send_3::<(), UInteger, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setVertexBufferOffset: attributeStride: atIndex:),
                offset,
                stride,
                index,
            );
        }
    }

    /// Set multiple vertex buffers at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setVertexBuffers(const Buffer* const*, const NS::UInteger*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The buffers and offsets pointers must be valid arrays with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_vertex_buffers_ptr(
        &self,
        buffers: *const *const c_void,
        offsets: *const UInteger,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = mtl_foundation::Range::new(range_location, range_length);
        unsafe {
            mtl_sys::msg_send_3::<
                (),
                *const *const c_void,
                *const UInteger,
                mtl_foundation::Range,
            >(
                self.as_ptr(),
                sel!(setVertexBuffers: offsets: withRange:),
                buffers,
                offsets,
                range,
            );
        }
    }

    /// Set multiple vertex buffers with strides at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setVertexBuffers(const Buffer* const*, const NS::UInteger*, const NS::UInteger*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The buffers, offsets, and strides pointers must be valid arrays with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_vertex_buffers_with_strides_ptr(
        &self,
        buffers: *const *const c_void,
        offsets: *const UInteger,
        strides: *const UInteger,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = mtl_foundation::Range::new(range_location, range_length);
        unsafe {
            mtl_sys::msg_send_4::<
                (),
                *const *const c_void,
                *const UInteger,
                *const UInteger,
                mtl_foundation::Range,
            >(
                self.as_ptr(),
                sel!(setVertexBuffers: offsets: attributeStrides: withRange:),
                buffers,
                offsets,
                strides,
                range,
            );
        }
    }

    // =========================================================================
    // Vertex Textures
    // =========================================================================

    /// Set a vertex texture.
    ///
    /// C++ equivalent: `void setVertexTexture(const Texture*, NS::UInteger)`
    #[inline]
    pub fn set_vertex_texture(&self, texture: &Texture, index: UInteger) {
        unsafe {
            mtl_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setVertexTexture: atIndex:),
                texture.as_ptr(),
                index,
            );
        }
    }

    /// Set multiple vertex textures at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setVertexTextures(const Texture* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The textures pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_vertex_textures_ptr(
        &self,
        textures: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = mtl_foundation::Range::new(range_location, range_length);
        unsafe {
            mtl_sys::msg_send_2::<(), *const *const c_void, mtl_foundation::Range>(
                self.as_ptr(),
                sel!(setVertexTextures: withRange:),
                textures,
                range,
            );
        }
    }

    // =========================================================================
    // Vertex Samplers
    // =========================================================================

    /// Set a vertex sampler state.
    ///
    /// C++ equivalent: `void setVertexSamplerState(const SamplerState*, NS::UInteger)`
    #[inline]
    pub fn set_vertex_sampler_state(&self, sampler: &crate::SamplerState, index: UInteger) {
        unsafe {
            mtl_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setVertexSamplerState: atIndex:),
                sampler.as_ptr(),
                index,
            );
        }
    }

    /// Set a vertex sampler state with LOD clamps.
    ///
    /// C++ equivalent: `void setVertexSamplerState(const SamplerState*, float, float, NS::UInteger)`
    #[inline]
    pub fn set_vertex_sampler_state_with_lod_clamps(
        &self,
        sampler: &crate::SamplerState,
        lod_min_clamp: f32,
        lod_max_clamp: f32,
        index: UInteger,
    ) {
        unsafe {
            mtl_sys::msg_send_4::<(), *const c_void, f32, f32, UInteger>(
                self.as_ptr(),
                sel!(setVertexSamplerState: lodMinClamp: lodMaxClamp: atIndex:),
                sampler.as_ptr(),
                lod_min_clamp,
                lod_max_clamp,
                index,
            );
        }
    }

    /// Set multiple vertex sampler states at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setVertexSamplerStates(const SamplerState* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The samplers pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_vertex_sampler_states_ptr(
        &self,
        samplers: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = mtl_foundation::Range::new(range_location, range_length);
        unsafe {
            mtl_sys::msg_send_2::<(), *const *const c_void, mtl_foundation::Range>(
                self.as_ptr(),
                sel!(setVertexSamplerStates: withRange:),
                samplers,
                range,
            );
        }
    }

    /// Set multiple vertex sampler states with LOD clamps at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setVertexSamplerStates(const SamplerState* const*, const float*, const float*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The samplers, lod_min_clamps, and lod_max_clamps pointers must be valid arrays with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_vertex_sampler_states_with_lod_clamps_ptr(
        &self,
        samplers: *const *const c_void,
        lod_min_clamps: *const f32,
        lod_max_clamps: *const f32,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = mtl_foundation::Range::new(range_location, range_length);
        unsafe {
            mtl_sys::msg_send_4::<
                (),
                *const *const c_void,
                *const f32,
                *const f32,
                mtl_foundation::Range,
            >(
                self.as_ptr(),
                sel!(setVertexSamplerStates: lodMinClamps: lodMaxClamps: withRange:),
                samplers,
                lod_min_clamps,
                lod_max_clamps,
                range,
            );
        }
    }

    // =========================================================================
    // Fragment Buffers
    // =========================================================================

    /// Set a fragment buffer.
    ///
    /// C++ equivalent: `void setFragmentBuffer(const Buffer*, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_fragment_buffer(&self, buffer: &Buffer, offset: UInteger, index: UInteger) {
        unsafe {
            mtl_sys::msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setFragmentBuffer: offset: atIndex:),
                buffer.as_ptr(),
                offset,
                index,
            );
        }
    }

    /// Set the fragment buffer offset.
    ///
    /// C++ equivalent: `void setFragmentBufferOffset(NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_fragment_buffer_offset(&self, offset: UInteger, index: UInteger) {
        unsafe {
            mtl_sys::msg_send_2::<(), UInteger, UInteger>(
                self.as_ptr(),
                sel!(setFragmentBufferOffset: atIndex:),
                offset,
                index,
            );
        }
    }

    /// Set inline fragment bytes.
    ///
    /// C++ equivalent: `void setFragmentBytes(const void*, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_fragment_bytes(&self, bytes: &[u8], index: UInteger) {
        unsafe {
            mtl_sys::msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setFragmentBytes: length: atIndex:),
                bytes.as_ptr() as *const c_void,
                bytes.len() as UInteger,
                index,
            );
        }
    }

    /// Set multiple fragment buffers at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setFragmentBuffers(const Buffer* const*, const NS::UInteger*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The buffers and offsets pointers must be valid arrays with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_fragment_buffers_ptr(
        &self,
        buffers: *const *const c_void,
        offsets: *const UInteger,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = mtl_foundation::Range::new(range_location, range_length);
        unsafe {
            mtl_sys::msg_send_3::<
                (),
                *const *const c_void,
                *const UInteger,
                mtl_foundation::Range,
            >(
                self.as_ptr(),
                sel!(setFragmentBuffers: offsets: withRange:),
                buffers,
                offsets,
                range,
            );
        }
    }

    // =========================================================================
    // Fragment Textures
    // =========================================================================

    /// Set a fragment texture.
    ///
    /// C++ equivalent: `void setFragmentTexture(const Texture*, NS::UInteger)`
    #[inline]
    pub fn set_fragment_texture(&self, texture: &Texture, index: UInteger) {
        unsafe {
            mtl_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setFragmentTexture: atIndex:),
                texture.as_ptr(),
                index,
            );
        }
    }

    /// Set multiple fragment textures at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setFragmentTextures(const Texture* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The textures pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_fragment_textures_ptr(
        &self,
        textures: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = mtl_foundation::Range::new(range_location, range_length);
        unsafe {
            mtl_sys::msg_send_2::<(), *const *const c_void, mtl_foundation::Range>(
                self.as_ptr(),
                sel!(setFragmentTextures: withRange:),
                textures,
                range,
            );
        }
    }

    // =========================================================================
    // Fragment Samplers
    // =========================================================================

    /// Set a fragment sampler state.
    ///
    /// C++ equivalent: `void setFragmentSamplerState(const SamplerState*, NS::UInteger)`
    #[inline]
    pub fn set_fragment_sampler_state(&self, sampler: &crate::SamplerState, index: UInteger) {
        unsafe {
            mtl_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setFragmentSamplerState: atIndex:),
                sampler.as_ptr(),
                index,
            );
        }
    }

    /// Set a fragment sampler state with LOD clamps.
    ///
    /// C++ equivalent: `void setFragmentSamplerState(const SamplerState*, float, float, NS::UInteger)`
    #[inline]
    pub fn set_fragment_sampler_state_with_lod_clamps(
        &self,
        sampler: &crate::SamplerState,
        lod_min_clamp: f32,
        lod_max_clamp: f32,
        index: UInteger,
    ) {
        unsafe {
            mtl_sys::msg_send_4::<(), *const c_void, f32, f32, UInteger>(
                self.as_ptr(),
                sel!(setFragmentSamplerState: lodMinClamp: lodMaxClamp: atIndex:),
                sampler.as_ptr(),
                lod_min_clamp,
                lod_max_clamp,
                index,
            );
        }
    }

    /// Set multiple fragment sampler states at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setFragmentSamplerStates(const SamplerState* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The samplers pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_fragment_sampler_states_ptr(
        &self,
        samplers: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = mtl_foundation::Range::new(range_location, range_length);
        unsafe {
            mtl_sys::msg_send_2::<(), *const *const c_void, mtl_foundation::Range>(
                self.as_ptr(),
                sel!(setFragmentSamplerStates: withRange:),
                samplers,
                range,
            );
        }
    }

    /// Set multiple fragment sampler states with LOD clamps at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setFragmentSamplerStates(const SamplerState* const*, const float*, const float*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The samplers, lod_min_clamps, and lod_max_clamps pointers must be valid arrays with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_fragment_sampler_states_with_lod_clamps_ptr(
        &self,
        samplers: *const *const c_void,
        lod_min_clamps: *const f32,
        lod_max_clamps: *const f32,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = mtl_foundation::Range::new(range_location, range_length);
        unsafe {
            mtl_sys::msg_send_4::<
                (),
                *const *const c_void,
                *const f32,
                *const f32,
                mtl_foundation::Range,
            >(
                self.as_ptr(),
                sel!(setFragmentSamplerStates: lodMinClamps: lodMaxClamps: withRange:),
                samplers,
                lod_min_clamps,
                lod_max_clamps,
                range,
            );
        }
    }

    // =========================================================================
}
