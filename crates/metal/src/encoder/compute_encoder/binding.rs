//! Resource binding methods for ComputeCommandEncoder.

use std::ffi::c_void;

use metal_foundation::{Referencing, UInteger};
use metal_sys::sel;

use crate::Buffer;
use crate::Texture;

use super::ComputeCommandEncoder;

impl ComputeCommandEncoder {
    // =========================================================================
    // Buffer Bindings
    // =========================================================================

    /// Set a buffer at an index.
    ///
    /// C++ equivalent: `void setBuffer(const Buffer*, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_buffer(&self, buffer: &Buffer, offset: UInteger, index: UInteger) {
        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setBuffer: offset: atIndex:),
                buffer.as_ptr(),
                offset,
                index,
            );
        }
    }

    /// Set a buffer at an index with attribute stride.
    ///
    /// C++ equivalent: `void setBuffer(const Buffer*, NS::UInteger, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_buffer_with_stride(
        &self,
        buffer: &Buffer,
        offset: UInteger,
        stride: UInteger,
        index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_4::<(), *const c_void, UInteger, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setBuffer: offset: attributeStride: atIndex:),
                buffer.as_ptr(),
                offset,
                stride,
                index,
            );
        }
    }

    /// Set the buffer offset at an index.
    ///
    /// C++ equivalent: `void setBufferOffset(NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_buffer_offset(&self, offset: UInteger, index: UInteger) {
        unsafe {
            metal_sys::msg_send_2::<(), UInteger, UInteger>(
                self.as_ptr(),
                sel!(setBufferOffset: atIndex:),
                offset,
                index,
            );
        }
    }

    /// Set the buffer offset at an index with attribute stride.
    ///
    /// C++ equivalent: `void setBufferOffset(NS::UInteger, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_buffer_offset_with_stride(
        &self,
        offset: UInteger,
        stride: UInteger,
        index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_3::<(), UInteger, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setBufferOffset: attributeStride: atIndex:),
                offset,
                stride,
                index,
            );
        }
    }

    /// Set inline bytes at an index.
    ///
    /// C++ equivalent: `void setBytes(const void*, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_bytes(&self, bytes: &[u8], index: UInteger) {
        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setBytes: length: atIndex:),
                bytes.as_ptr() as *const c_void,
                bytes.len() as UInteger,
                index,
            );
        }
    }

    /// Set inline bytes at an index with attribute stride.
    ///
    /// C++ equivalent: `void setBytes(const void*, NS::UInteger, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_bytes_with_stride(&self, bytes: &[u8], stride: UInteger, index: UInteger) {
        unsafe {
            metal_sys::msg_send_4::<(), *const c_void, UInteger, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setBytes: length: attributeStride: atIndex:),
                bytes.as_ptr() as *const c_void,
                bytes.len() as UInteger,
                stride,
                index,
            );
        }
    }

    /// Set multiple buffers at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setBuffers(const Buffer* const*, const NS::UInteger*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The buffers and offsets pointers must be valid arrays with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_buffers_ptr(
        &self,
        buffers: *const *const c_void,
        offsets: *const UInteger,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_3::<
                (),
                *const *const c_void,
                *const UInteger,
                metal_foundation::Range,
            >(
                self.as_ptr(),
                sel!(setBuffers: offsets: withRange:),
                buffers,
                offsets,
                range,
            );
        }
    }

    /// Set multiple buffers at a range of indices with strides (raw pointer version).
    ///
    /// C++ equivalent: `void setBuffers(const Buffer* const*, const NS::UInteger*, const NS::UInteger*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The buffers, offsets, and strides pointers must be valid arrays with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_buffers_with_strides_ptr(
        &self,
        buffers: *const *const c_void,
        offsets: *const UInteger,
        strides: *const UInteger,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_4::<
                (),
                *const *const c_void,
                *const UInteger,
                *const UInteger,
                metal_foundation::Range,
            >(
                self.as_ptr(),
                sel!(setBuffers: offsets: attributeStrides: withRange:),
                buffers,
                offsets,
                strides,
                range,
            );
        }
    }

    // =========================================================================
    // Texture Bindings
    // =========================================================================

    /// Set a texture at an index.
    ///
    /// C++ equivalent: `void setTexture(const Texture*, NS::UInteger)`
    #[inline]
    pub fn set_texture(&self, texture: &Texture, index: UInteger) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setTexture: atIndex:),
                texture.as_ptr(),
                index,
            );
        }
    }

    /// Set multiple textures at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setTextures(const Texture* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The textures pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_textures_ptr(
        &self,
        textures: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(setTextures: withRange:),
                textures,
                range,
            );
        }
    }

    // =========================================================================
    // Sampler Bindings
    // =========================================================================

    /// Set a sampler state at an index.
    ///
    /// C++ equivalent: `void setSamplerState(const SamplerState*, NS::UInteger)`
    #[inline]
    pub fn set_sampler_state(&self, sampler: &crate::SamplerState, index: UInteger) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setSamplerState: atIndex:),
                sampler.as_ptr(),
                index,
            );
        }
    }

    /// Set a sampler state with LOD clamps at an index.
    ///
    /// C++ equivalent: `void setSamplerState(const SamplerState*, float, float, NS::UInteger)`
    #[inline]
    pub fn set_sampler_state_with_lod_clamps(
        &self,
        sampler: &crate::SamplerState,
        lod_min_clamp: f32,
        lod_max_clamp: f32,
        index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_4::<(), *const c_void, f32, f32, UInteger>(
                self.as_ptr(),
                sel!(setSamplerState: lodMinClamp: lodMaxClamp: atIndex:),
                sampler.as_ptr(),
                lod_min_clamp,
                lod_max_clamp,
                index,
            );
        }
    }

    /// Set multiple sampler states at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setSamplerStates(const SamplerState* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The samplers pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_sampler_states_ptr(
        &self,
        samplers: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(setSamplerStates: withRange:),
                samplers,
                range,
            );
        }
    }

    /// Set multiple sampler states with LOD clamps at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setSamplerStates(const SamplerState* const*, const float*, const float*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The samplers, lod_min_clamps, and lod_max_clamps pointers must be valid arrays with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_sampler_states_with_lod_clamps_ptr(
        &self,
        samplers: *const *const c_void,
        lod_min_clamps: *const f32,
        lod_max_clamps: *const f32,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_4::<
                (),
                *const *const c_void,
                *const f32,
                *const f32,
                metal_foundation::Range,
            >(
                self.as_ptr(),
                sel!(setSamplerStates: lodMinClamps: lodMaxClamps: withRange:),
                samplers,
                lod_min_clamps,
                lod_max_clamps,
                range,
            );
        }
    }

    // =========================================================================
    // Threadgroup Memory
    // =========================================================================

    /// Set the threadgroup memory length at an index.
    ///
    /// C++ equivalent: `void setThreadgroupMemoryLength(NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_threadgroup_memory_length(&self, length: UInteger, index: UInteger) {
        unsafe {
            metal_sys::msg_send_2::<(), UInteger, UInteger>(
                self.as_ptr(),
                sel!(setThreadgroupMemoryLength: atIndex:),
                length,
                index,
            );
        }
    }

    /// Set the imageblock dimensions.
    ///
    /// C++ equivalent: `void setImageblockWidth(NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_imageblock_width(&self, width: UInteger, height: UInteger) {
        unsafe {
            metal_sys::msg_send_2::<(), UInteger, UInteger>(
                self.as_ptr(),
                sel!(setImageblockWidth: height:),
                width,
                height,
            );
        }
    }
}
