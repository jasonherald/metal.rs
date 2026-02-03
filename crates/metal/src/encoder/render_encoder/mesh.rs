//! Object and mesh shader bindings.
//!
//! This module contains methods for binding resources to object and mesh shaders.

use std::ffi::c_void;

use metal_foundation::{Referencing, UInteger};
use metal_sys::sel;

use crate::Buffer;
use crate::Texture;

use super::RenderCommandEncoder;

impl RenderCommandEncoder {
    // =========================================================================
    // Object Shader Bindings
    // =========================================================================

    /// Set an object buffer.
    ///
    /// C++ equivalent: `void setObjectBuffer(const Buffer*, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_object_buffer(&self, buffer: &Buffer, offset: UInteger, index: UInteger) {
        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setObjectBuffer: offset: atIndex:),
                buffer.as_ptr(),
                offset,
                index,
            );
        }
    }

    /// Set an object texture.
    ///
    /// C++ equivalent: `void setObjectTexture(const Texture*, NS::UInteger)`
    #[inline]
    pub fn set_object_texture(&self, texture: &Texture, index: UInteger) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setObjectTexture: atIndex:),
                texture.as_ptr(),
                index,
            );
        }
    }

    /// Set an object sampler state.
    ///
    /// C++ equivalent: `void setObjectSamplerState(const SamplerState*, NS::UInteger)`
    #[inline]
    pub fn set_object_sampler_state(&self, sampler: &crate::SamplerState, index: UInteger) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setObjectSamplerState: atIndex:),
                sampler.as_ptr(),
                index,
            );
        }
    }

    /// Set the object threadgroup memory length.
    ///
    /// C++ equivalent: `void setObjectThreadgroupMemoryLength(NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_object_threadgroup_memory_length(&self, length: UInteger, index: UInteger) {
        unsafe {
            metal_sys::msg_send_2::<(), UInteger, UInteger>(
                self.as_ptr(),
                sel!(setObjectThreadgroupMemoryLength: atIndex:),
                length,
                index,
            );
        }
    }

    /// Set the object buffer offset.
    ///
    /// C++ equivalent: `void setObjectBufferOffset(NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_object_buffer_offset(&self, offset: UInteger, index: UInteger) {
        unsafe {
            metal_sys::msg_send_2::<(), UInteger, UInteger>(
                self.as_ptr(),
                sel!(setObjectBufferOffset: atIndex:),
                offset,
                index,
            );
        }
    }

    /// Set inline object bytes.
    ///
    /// C++ equivalent: `void setObjectBytes(const void*, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_object_bytes(&self, bytes: &[u8], index: UInteger) {
        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setObjectBytes: length: atIndex:),
                bytes.as_ptr() as *const c_void,
                bytes.len() as UInteger,
                index,
            );
        }
    }

    /// Set multiple object buffers at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setObjectBuffers(const Buffer* const*, const NS::UInteger*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The buffers and offsets pointers must be valid arrays with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_object_buffers_ptr(
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
                sel!(setObjectBuffers: offsets: withRange:),
                buffers,
                offsets,
                range,
            );
        }
    }

    /// Set multiple object textures at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setObjectTextures(const Texture* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The textures pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_object_textures_ptr(
        &self,
        textures: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(setObjectTextures: withRange:),
                textures,
                range,
            );
        }
    }

    /// Set an object sampler state with LOD clamps.
    ///
    /// C++ equivalent: `void setObjectSamplerState(const SamplerState*, float, float, NS::UInteger)`
    #[inline]
    pub fn set_object_sampler_state_with_lod_clamps(
        &self,
        sampler: &crate::SamplerState,
        lod_min_clamp: f32,
        lod_max_clamp: f32,
        index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_4::<(), *const c_void, f32, f32, UInteger>(
                self.as_ptr(),
                sel!(setObjectSamplerState: lodMinClamp: lodMaxClamp: atIndex:),
                sampler.as_ptr(),
                lod_min_clamp,
                lod_max_clamp,
                index,
            );
        }
    }

    /// Set multiple object sampler states at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setObjectSamplerStates(const SamplerState* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The samplers pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_object_sampler_states_ptr(
        &self,
        samplers: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(setObjectSamplerStates: withRange:),
                samplers,
                range,
            );
        }
    }

    /// Set multiple object sampler states with LOD clamps at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setObjectSamplerStates(const SamplerState* const*, const float*, const float*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The samplers, lod_min_clamps, and lod_max_clamps pointers must be valid arrays with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_object_sampler_states_with_lod_clamps_ptr(
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
                sel!(setObjectSamplerStates: lodMinClamps: lodMaxClamps: withRange:),
                samplers,
                lod_min_clamps,
                lod_max_clamps,
                range,
            );
        }
    }

    // =========================================================================
    // Mesh Shader Bindings
    // =========================================================================

    /// Set a mesh buffer.
    ///
    /// C++ equivalent: `void setMeshBuffer(const Buffer*, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_mesh_buffer(&self, buffer: &Buffer, offset: UInteger, index: UInteger) {
        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setMeshBuffer: offset: atIndex:),
                buffer.as_ptr(),
                offset,
                index,
            );
        }
    }

    /// Set a mesh texture.
    ///
    /// C++ equivalent: `void setMeshTexture(const Texture*, NS::UInteger)`
    #[inline]
    pub fn set_mesh_texture(&self, texture: &Texture, index: UInteger) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setMeshTexture: atIndex:),
                texture.as_ptr(),
                index,
            );
        }
    }

    /// Set a mesh sampler state.
    ///
    /// C++ equivalent: `void setMeshSamplerState(const SamplerState*, NS::UInteger)`
    #[inline]
    pub fn set_mesh_sampler_state(&self, sampler: &crate::SamplerState, index: UInteger) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setMeshSamplerState: atIndex:),
                sampler.as_ptr(),
                index,
            );
        }
    }

    /// Set the mesh buffer offset.
    ///
    /// C++ equivalent: `void setMeshBufferOffset(NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_mesh_buffer_offset(&self, offset: UInteger, index: UInteger) {
        unsafe {
            metal_sys::msg_send_2::<(), UInteger, UInteger>(
                self.as_ptr(),
                sel!(setMeshBufferOffset: atIndex:),
                offset,
                index,
            );
        }
    }

    /// Set inline mesh bytes.
    ///
    /// C++ equivalent: `void setMeshBytes(const void*, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn set_mesh_bytes(&self, bytes: &[u8], index: UInteger) {
        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setMeshBytes: length: atIndex:),
                bytes.as_ptr() as *const c_void,
                bytes.len() as UInteger,
                index,
            );
        }
    }

    /// Set multiple mesh buffers at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setMeshBuffers(const Buffer* const*, const NS::UInteger*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The buffers and offsets pointers must be valid arrays with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_mesh_buffers_ptr(
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
                sel!(setMeshBuffers: offsets: withRange:),
                buffers,
                offsets,
                range,
            );
        }
    }

    /// Set multiple mesh textures at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setMeshTextures(const Texture* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The textures pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_mesh_textures_ptr(
        &self,
        textures: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(setMeshTextures: withRange:),
                textures,
                range,
            );
        }
    }

    /// Set a mesh sampler state with LOD clamps.
    ///
    /// C++ equivalent: `void setMeshSamplerState(const SamplerState*, float, float, NS::UInteger)`
    #[inline]
    pub fn set_mesh_sampler_state_with_lod_clamps(
        &self,
        sampler: &crate::SamplerState,
        lod_min_clamp: f32,
        lod_max_clamp: f32,
        index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_4::<(), *const c_void, f32, f32, UInteger>(
                self.as_ptr(),
                sel!(setMeshSamplerState: lodMinClamp: lodMaxClamp: atIndex:),
                sampler.as_ptr(),
                lod_min_clamp,
                lod_max_clamp,
                index,
            );
        }
    }

    /// Set multiple mesh sampler states at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setMeshSamplerStates(const SamplerState* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The samplers pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_mesh_sampler_states_ptr(
        &self,
        samplers: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(setMeshSamplerStates: withRange:),
                samplers,
                range,
            );
        }
    }

    /// Set multiple mesh sampler states with LOD clamps at a range of indices (raw pointer version).
    ///
    /// C++ equivalent: `void setMeshSamplerStates(const SamplerState* const*, const float*, const float*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The samplers, lod_min_clamps, and lod_max_clamps pointers must be valid arrays with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_mesh_sampler_states_with_lod_clamps_ptr(
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
                sel!(setMeshSamplerStates: lodMinClamps: lodMaxClamps: withRange:),
                samplers,
                lod_min_clamps,
                lod_max_clamps,
                range,
            );
        }
    }
}
