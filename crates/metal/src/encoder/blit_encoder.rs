//! Blit command encoder.
//!
//! Corresponds to `Metal/MTLBlitCommandEncoder.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::BlitOption;
use crate::types::{Origin, Region, Size};
use crate::Buffer;
use crate::Texture;

/// A command encoder for data transfer operations.
///
/// C++ equivalent: `MTL::BlitCommandEncoder`
///
/// Blit encoders are used to copy data between buffers and textures,
/// generate mipmaps, and synchronize resources.
#[repr(transparent)]
pub struct BlitCommandEncoder(pub(crate) NonNull<c_void>);

impl BlitCommandEncoder {
    /// Create a BlitCommandEncoder from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal blit command encoder object.
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
    // Tensor Copy
    // =========================================================================

    /// Copy data from one tensor to another (raw pointer version).
    ///
    /// C++ equivalent: `void copyFromTensor(...)`
    ///
    /// # Safety
    ///
    /// All tensor and extents pointers must be valid Metal tensor/extents objects.
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn copy_from_tensor_ptr(
        &self,
        source_tensor: *const c_void,
        source_origin: *const c_void,
        source_dimensions: *const c_void,
        destination_tensor: *const c_void,
        destination_origin: *const c_void,
        destination_dimensions: *const c_void,
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
                sel!(copyFromTensor: sourceOrigin: sourceDimensions: toTensor: destinationOrigin: destinationDimensions:),
                source_tensor,
                source_origin,
                source_dimensions,
                destination_tensor,
                destination_origin,
                destination_dimensions,
            );
        }
    }

    // =========================================================================
    // Buffer to Buffer Copy
    // =========================================================================

    /// Copy data from one buffer to another.
    ///
    /// C++ equivalent: `void copyFromBuffer(const Buffer*, NS::UInteger, const Buffer*, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn copy_from_buffer_to_buffer(
        &self,
        source_buffer: &Buffer,
        source_offset: UInteger,
        destination_buffer: &Buffer,
        destination_offset: UInteger,
        size: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_5::<(), *const c_void, UInteger, *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(copyFromBuffer: sourceOffset: toBuffer: destinationOffset: size:),
                source_buffer.as_ptr(),
                source_offset,
                destination_buffer.as_ptr(),
                destination_offset,
                size,
            );
        }
    }

    // =========================================================================
    // Buffer to Texture Copy
    // =========================================================================

    /// Copy data from a buffer to a texture.
    ///
    /// C++ equivalent: `void copyFromBuffer(...toTexture...)`
    #[allow(clippy::too_many_arguments)]
    pub fn copy_from_buffer_to_texture(
        &self,
        source_buffer: &Buffer,
        source_offset: UInteger,
        source_bytes_per_row: UInteger,
        source_bytes_per_image: UInteger,
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
                UInteger,
                Size,
                *const c_void,
                UInteger,
                UInteger,
                Origin,
            >(
                self.as_ptr(),
                sel!(copyFromBuffer: sourceOffset: sourceBytesPerRow: sourceBytesPerImage: sourceSize: toTexture: destinationSlice: destinationLevel: destinationOrigin:),
                source_buffer.as_ptr(),
                source_offset,
                source_bytes_per_row,
                source_bytes_per_image,
                source_size,
                destination_texture.as_ptr(),
                destination_slice,
                destination_level,
                destination_origin,
            );
        }
    }

    /// Copy data from a buffer to a texture with options.
    ///
    /// C++ equivalent: `void copyFromBuffer(...toTexture...options:)`
    #[allow(clippy::too_many_arguments)]
    pub fn copy_from_buffer_to_texture_with_options(
        &self,
        source_buffer: &Buffer,
        source_offset: UInteger,
        source_bytes_per_row: UInteger,
        source_bytes_per_image: UInteger,
        source_size: Size,
        destination_texture: &Texture,
        destination_slice: UInteger,
        destination_level: UInteger,
        destination_origin: Origin,
        options: BlitOption,
    ) {
        unsafe {
            metal_sys::msg_send_10::<
                (),
                *const c_void,
                UInteger,
                UInteger,
                UInteger,
                Size,
                *const c_void,
                UInteger,
                UInteger,
                Origin,
                BlitOption,
            >(
                self.as_ptr(),
                sel!(copyFromBuffer: sourceOffset: sourceBytesPerRow: sourceBytesPerImage: sourceSize: toTexture: destinationSlice: destinationLevel: destinationOrigin: options:),
                source_buffer.as_ptr(),
                source_offset,
                source_bytes_per_row,
                source_bytes_per_image,
                source_size,
                destination_texture.as_ptr(),
                destination_slice,
                destination_level,
                destination_origin,
                options,
            );
        }
    }

    // =========================================================================
    // Texture to Buffer Copy
    // =========================================================================

    /// Copy data from a texture to a buffer.
    ///
    /// C++ equivalent: `void copyFromTexture(...toBuffer...)`
    #[allow(clippy::too_many_arguments)]
    pub fn copy_from_texture_to_buffer(
        &self,
        source_texture: &Texture,
        source_slice: UInteger,
        source_level: UInteger,
        source_origin: Origin,
        source_size: Size,
        destination_buffer: &Buffer,
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
                Origin,
                Size,
                *const c_void,
                UInteger,
                UInteger,
                UInteger,
            >(
                self.as_ptr(),
                sel!(copyFromTexture: sourceSlice: sourceLevel: sourceOrigin: sourceSize: toBuffer: destinationOffset: destinationBytesPerRow: destinationBytesPerImage:),
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

    /// Copy data from a texture to a buffer with options.
    ///
    /// C++ equivalent: `void copyFromTexture(...toBuffer...options:)`
    #[allow(clippy::too_many_arguments)]
    pub fn copy_from_texture_to_buffer_with_options(
        &self,
        source_texture: &Texture,
        source_slice: UInteger,
        source_level: UInteger,
        source_origin: Origin,
        source_size: Size,
        destination_buffer: &Buffer,
        destination_offset: UInteger,
        destination_bytes_per_row: UInteger,
        destination_bytes_per_image: UInteger,
        options: BlitOption,
    ) {
        unsafe {
            metal_sys::msg_send_10::<
                (),
                *const c_void,
                UInteger,
                UInteger,
                Origin,
                Size,
                *const c_void,
                UInteger,
                UInteger,
                UInteger,
                BlitOption,
            >(
                self.as_ptr(),
                sel!(copyFromTexture: sourceSlice: sourceLevel: sourceOrigin: sourceSize: toBuffer: destinationOffset: destinationBytesPerRow: destinationBytesPerImage: options:),
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

    // =========================================================================
    // Texture to Texture Copy
    // =========================================================================

    /// Copy data from one texture to another (full copy).
    ///
    /// C++ equivalent: `void copyFromTexture(const Texture*, const Texture*)`
    #[inline]
    pub fn copy_from_texture_to_texture(
        &self,
        source_texture: &Texture,
        destination_texture: &Texture,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, *const c_void>(
                self.as_ptr(),
                sel!(copyFromTexture: toTexture:),
                source_texture.as_ptr(),
                destination_texture.as_ptr(),
            );
        }
    }

    /// Copy data from one texture to another (region copy).
    ///
    /// C++ equivalent: `void copyFromTexture(...toTexture...)`
    #[allow(clippy::too_many_arguments)]
    pub fn copy_from_texture_to_texture_region(
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
                sel!(copyFromTexture: sourceSlice: sourceLevel: sourceOrigin: sourceSize: toTexture: destinationSlice: destinationLevel: destinationOrigin:),
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

    /// Copy slices and levels between textures.
    ///
    /// C++ equivalent: `void copyFromTexture(...sliceCount: levelCount:)`
    #[allow(clippy::too_many_arguments)]
    pub fn copy_from_texture_to_texture_slices(
        &self,
        source_texture: &Texture,
        source_slice: UInteger,
        source_level: UInteger,
        destination_texture: &Texture,
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
                sel!(copyFromTexture: sourceSlice: sourceLevel: toTexture: destinationSlice: destinationLevel: sliceCount: levelCount:),
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

    // =========================================================================
    // Fill Operations
    // =========================================================================

    /// Fill a buffer with a value.
    ///
    /// C++ equivalent: `void fillBuffer(const Buffer*, NS::Range, uint8_t)`
    pub fn fill_buffer(&self, buffer: &Buffer, offset: UInteger, length: UInteger, value: u8) {
        let range = metal_foundation::Range::new(offset, length);
        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, metal_foundation::Range, u8>(
                self.as_ptr(),
                sel!(fillBuffer: range: value:),
                buffer.as_ptr(),
                range,
                value,
            );
        }
    }

    // =========================================================================
    // Mipmap Generation
    // =========================================================================

    /// Generate mipmaps for a texture.
    ///
    /// C++ equivalent: `void generateMipmaps(const Texture*)`
    #[inline]
    pub fn generate_mipmaps(&self, texture: &Texture) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(generateMipmapsForTexture:),
                texture.as_ptr(),
            );
        }
    }

    // =========================================================================
    // Optimization Operations
    // =========================================================================

    /// Optimize a texture's contents for CPU access.
    ///
    /// C++ equivalent: `void optimizeContentsForCPUAccess(const Texture*)`
    #[inline]
    pub fn optimize_contents_for_cpu_access(&self, texture: &Texture) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(optimizeContentsForCPUAccess:),
                texture.as_ptr(),
            );
        }
    }

    /// Optimize a texture slice's contents for CPU access.
    ///
    /// C++ equivalent: `void optimizeContentsForCPUAccess(const Texture*, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn optimize_contents_for_cpu_access_slice(
        &self,
        texture: &Texture,
        slice: UInteger,
        level: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(optimizeContentsForCPUAccess: slice: level:),
                texture.as_ptr(),
                slice,
                level,
            );
        }
    }

    /// Optimize a texture's contents for GPU access.
    ///
    /// C++ equivalent: `void optimizeContentsForGPUAccess(const Texture*)`
    #[inline]
    pub fn optimize_contents_for_gpu_access(&self, texture: &Texture) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(optimizeContentsForGPUAccess:),
                texture.as_ptr(),
            );
        }
    }

    /// Optimize a texture slice's contents for GPU access.
    ///
    /// C++ equivalent: `void optimizeContentsForGPUAccess(const Texture*, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn optimize_contents_for_gpu_access_slice(
        &self,
        texture: &Texture,
        slice: UInteger,
        level: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(optimizeContentsForGPUAccess: slice: level:),
                texture.as_ptr(),
                slice,
                level,
            );
        }
    }

    // =========================================================================
    // Synchronization
    // =========================================================================

    /// Synchronize a managed resource (raw pointer version).
    ///
    /// This ensures the GPU's copy of a managed resource is synchronized with the CPU.
    ///
    /// C++ equivalent: `void synchronizeResource(const Resource*)`
    ///
    /// # Safety
    ///
    /// The resource pointer must be a valid Metal resource object.
    #[inline]
    pub unsafe fn synchronize_resource_ptr(&self, resource: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(synchronizeResource:), resource);
        }
    }

    /// Synchronize a managed buffer.
    ///
    /// C++ equivalent: `void synchronizeResource(const Resource*)`
    #[inline]
    pub fn synchronize_buffer(&self, buffer: &Buffer) {
        unsafe { self.synchronize_resource_ptr(buffer.as_ptr()) };
    }

    /// Synchronize a managed texture.
    ///
    /// C++ equivalent: `void synchronizeResource(const Resource*)`
    #[inline]
    pub fn synchronize_texture(&self, texture: &Texture) {
        unsafe { self.synchronize_resource_ptr(texture.as_ptr()) };
    }

    /// Synchronize a texture slice.
    ///
    /// C++ equivalent: `void synchronizeTexture(const Texture*, NS::UInteger, NS::UInteger)`
    #[inline]
    pub fn synchronize_texture_slice(&self, texture: &Texture, slice: UInteger, level: UInteger) {
        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, UInteger, UInteger>(
                self.as_ptr(),
                sel!(synchronizeTexture: slice: level:),
                texture.as_ptr(),
                slice,
                level,
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

    // =========================================================================
    // Texture Access Counters
    // =========================================================================

    /// Get texture access counters.
    ///
    /// C++ equivalent: `void getTextureAccessCounters(...)`
    #[allow(clippy::too_many_arguments)]
    pub fn get_texture_access_counters(
        &self,
        texture: &Texture,
        region: Region,
        mip_level: UInteger,
        slice: UInteger,
        reset_counters: bool,
        counters_buffer: &Buffer,
        counters_buffer_offset: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_7::<
                (),
                *const c_void,
                Region,
                UInteger,
                UInteger,
                bool,
                *const c_void,
                UInteger,
            >(
                self.as_ptr(),
                sel!(getTextureAccessCounters: region: mipLevel: slice: resetCounters: countersBuffer: countersBufferOffset:),
                texture.as_ptr(),
                region,
                mip_level,
                slice,
                reset_counters,
                counters_buffer.as_ptr(),
                counters_buffer_offset,
            );
        }
    }

    /// Reset texture access counters.
    ///
    /// C++ equivalent: `void resetTextureAccessCounters(...)`
    pub fn reset_texture_access_counters(
        &self,
        texture: &Texture,
        region: Region,
        mip_level: UInteger,
        slice: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_4::<(), *const c_void, Region, UInteger, UInteger>(
                self.as_ptr(),
                sel!(resetTextureAccessCounters: region: mipLevel: slice:),
                texture.as_ptr(),
                region,
                mip_level,
                slice,
            );
        }
    }

    // =========================================================================
    // Indirect Command Buffer Operations
    // =========================================================================

    /// Copy an indirect command buffer (raw pointer version).
    ///
    /// C++ equivalent: `void copyIndirectCommandBuffer(...)`
    ///
    /// # Safety
    ///
    /// The source and destination pointers must be valid indirect command buffer objects.
    pub unsafe fn copy_indirect_command_buffer_ptr(
        &self,
        source: *const c_void,
        source_offset: UInteger,
        source_length: UInteger,
        destination: *const c_void,
        destination_index: UInteger,
    ) {
        let range = metal_foundation::Range::new(source_offset, source_length);
        unsafe {
            metal_sys::msg_send_4::<(), *const c_void, metal_foundation::Range, *const c_void, UInteger>(
                self.as_ptr(),
                sel!(copyIndirectCommandBuffer: sourceRange: destination: destinationIndex:),
                source,
                range,
                destination,
                destination_index,
            );
        }
    }

    /// Optimize an indirect command buffer (raw pointer version).
    ///
    /// C++ equivalent: `void optimizeIndirectCommandBuffer(...)`
    ///
    /// # Safety
    ///
    /// The indirect command buffer pointer must be valid.
    pub unsafe fn optimize_indirect_command_buffer_ptr(
        &self,
        indirect_command_buffer: *const c_void,
        offset: UInteger,
        length: UInteger,
    ) {
        let range = metal_foundation::Range::new(offset, length);
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(optimizeIndirectCommandBuffer: withRange:),
                indirect_command_buffer,
                range,
            );
        }
    }

    /// Reset commands in an indirect command buffer (raw pointer version).
    ///
    /// C++ equivalent: `void resetCommandsInBuffer(...)`
    ///
    /// # Safety
    ///
    /// The buffer pointer must be a valid indirect command buffer object.
    pub unsafe fn reset_commands_in_buffer_ptr(
        &self,
        buffer: *const c_void,
        offset: UInteger,
        length: UInteger,
    ) {
        let range = metal_foundation::Range::new(offset, length);
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(resetCommandsInBuffer: withRange:),
                buffer,
                range,
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

    /// Resolve counters (raw pointer version).
    ///
    /// C++ equivalent: `void resolveCounters(...)`
    ///
    /// # Safety
    ///
    /// The sample buffer pointer must be a valid counter sample buffer object.
    pub unsafe fn resolve_counters_ptr(
        &self,
        sample_buffer: *const c_void,
        offset: UInteger,
        length: UInteger,
        destination_buffer: &Buffer,
        destination_offset: UInteger,
    ) {
        let range = metal_foundation::Range::new(offset, length);
        unsafe {
            metal_sys::msg_send_4::<
                (),
                *const c_void,
                metal_foundation::Range,
                *const c_void,
                UInteger,
            >(
                self.as_ptr(),
                sel!(resolveCounters: inRange: destinationBuffer: destinationOffset:),
                sample_buffer,
                range,
                destination_buffer.as_ptr(),
                destination_offset,
            );
        }
    }
}

impl Clone for BlitCommandEncoder {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for BlitCommandEncoder {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for BlitCommandEncoder {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for BlitCommandEncoder {}
unsafe impl Sync for BlitCommandEncoder {}

impl std::fmt::Debug for BlitCommandEncoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlitCommandEncoder")
            .field("label", &self.label())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blit_encoder_size() {
        assert_eq!(
            std::mem::size_of::<BlitCommandEncoder>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
