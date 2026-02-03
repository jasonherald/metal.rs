//! Device limits and alignment queries.
//!
//! Corresponds to limit query methods in `MTL::Device` from `Metal/MTLDevice.hpp`.

use std::ffi::c_void;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, msg_send_3, msg_send_4, sel};

use super::Device;
use crate::enums::{
    PixelFormat, ResourceOptions, SparsePageSize, SparseTextureRegionAlignmentMode, TextureType,
};
use crate::types::{Region, Size, SizeAndAlign};

impl Device {
    // =========================================================================
    // Thread Limits
    // =========================================================================

    /// Get the maximum threads per threadgroup.
    ///
    /// C++ equivalent: `Size maxThreadsPerThreadgroup() const`
    #[inline]
    pub fn max_threads_per_threadgroup(&self) -> Size {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxThreadsPerThreadgroup)) }
    }

    /// Get the maximum threadgroup memory length in bytes.
    ///
    /// C++ equivalent: `NS::UInteger maxThreadgroupMemoryLength() const`
    #[inline]
    pub fn max_threadgroup_memory_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxThreadgroupMemoryLength)) }
    }

    // =========================================================================
    // Buffer Limits
    // =========================================================================

    /// Get the maximum buffer length in bytes.
    ///
    /// C++ equivalent: `NS::UInteger maxBufferLength() const`
    #[inline]
    pub fn max_buffer_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxBufferLength)) }
    }

    // =========================================================================
    // Argument Buffer Limits
    // =========================================================================

    /// Get the maximum number of samplers that can be in an argument buffer.
    ///
    /// C++ equivalent: `NS::UInteger maxArgumentBufferSamplerCount() const`
    #[inline]
    pub fn max_argument_buffer_sampler_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxArgumentBufferSamplerCount)) }
    }

    // =========================================================================
    // Texture Alignment
    // =========================================================================

    /// Get the minimum alignment for a linear texture with the given pixel format.
    ///
    /// C++ equivalent: `NS::UInteger minimumLinearTextureAlignmentForPixelFormat(MTL::PixelFormat format)`
    #[inline]
    pub fn minimum_linear_texture_alignment_for_pixel_format(
        &self,
        format: PixelFormat,
    ) -> UInteger {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(minimumLinearTextureAlignmentForPixelFormat:),
                format,
            )
        }
    }

    /// Get the minimum alignment for a texture buffer with the given pixel format.
    ///
    /// C++ equivalent: `NS::UInteger minimumTextureBufferAlignmentForPixelFormat(MTL::PixelFormat format)`
    #[inline]
    pub fn minimum_texture_buffer_alignment_for_pixel_format(
        &self,
        format: PixelFormat,
    ) -> UInteger {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(minimumTextureBufferAlignmentForPixelFormat:),
                format,
            )
        }
    }

    // =========================================================================
    // Sparse Texture Support
    // =========================================================================

    /// Get the sparse tile size in bytes.
    ///
    /// C++ equivalent: `NS::UInteger sparseTileSizeInBytes() const`
    #[inline]
    pub fn sparse_tile_size_in_bytes(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(sparseTileSizeInBytes)) }
    }

    /// Get the sparse tile size in bytes for a specific page size.
    ///
    /// C++ equivalent: `NS::UInteger sparseTileSizeInBytes(MTL::SparsePageSize sparsePageSize)`
    #[inline]
    pub fn sparse_tile_size_in_bytes_for_page_size(&self, page_size: SparsePageSize) -> UInteger {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(sparseTileSizeInBytesForSparsePageSize:),
                page_size,
            )
        }
    }

    /// Get the sparse tile size for a texture configuration.
    ///
    /// C++ equivalent: `Size sparseTileSize(MTL::TextureType, MTL::PixelFormat, NS::UInteger sampleCount)`
    #[inline]
    pub fn sparse_tile_size(
        &self,
        texture_type: TextureType,
        pixel_format: PixelFormat,
        sample_count: UInteger,
    ) -> Size {
        unsafe {
            msg_send_3(
                self.as_ptr(),
                sel!(sparseTileSizeWithTextureType: pixelFormat: sampleCount:),
                texture_type,
                pixel_format,
                sample_count,
            )
        }
    }

    /// Get the sparse tile size for a texture configuration with specific page size.
    ///
    /// C++ equivalent: `Size sparseTileSize(MTL::TextureType, MTL::PixelFormat, NS::UInteger, MTL::SparsePageSize)`
    #[inline]
    pub fn sparse_tile_size_with_page_size(
        &self,
        texture_type: TextureType,
        pixel_format: PixelFormat,
        sample_count: UInteger,
        page_size: SparsePageSize,
    ) -> Size {
        unsafe {
            msg_send_4(
                self.as_ptr(),
                sel!(sparseTileSizeWithTextureType: pixelFormat: sampleCount: sparsePageSize:),
                texture_type,
                pixel_format,
                sample_count,
                page_size,
            )
        }
    }

    /// Convert pixel regions to tile regions for sparse textures.
    ///
    /// C++ equivalent: `void convertSparsePixelRegions(...)`
    pub fn convert_sparse_pixel_regions(
        &self,
        pixel_regions: &[Region],
        tile_regions: &mut [Region],
        tile_size: Size,
        mode: SparseTextureRegionAlignmentMode,
    ) {
        assert_eq!(pixel_regions.len(), tile_regions.len());
        unsafe {
            metal_sys::msg_send_5::<
                (),
                *const Region,
                *mut Region,
                Size,
                SparseTextureRegionAlignmentMode,
                usize,
            >(
                self.as_ptr(),
                sel!(convertSparsePixelRegions: toTileRegions: withTileSize: alignmentMode: numRegions:),
                pixel_regions.as_ptr(),
                tile_regions.as_mut_ptr(),
                tile_size,
                mode,
                pixel_regions.len(),
            );
        }
    }

    /// Convert tile regions to pixel regions for sparse textures.
    ///
    /// C++ equivalent: `void convertSparseTileRegions(...)`
    pub fn convert_sparse_tile_regions(
        &self,
        tile_regions: &[Region],
        pixel_regions: &mut [Region],
        tile_size: Size,
    ) {
        assert_eq!(tile_regions.len(), pixel_regions.len());
        unsafe {
            msg_send_4::<(), *const Region, *mut Region, Size, usize>(
                self.as_ptr(),
                sel!(convertSparseTileRegions: toPixelRegions: withTileSize: numRegions:),
                tile_regions.as_ptr(),
                pixel_regions.as_mut_ptr(),
                tile_size,
                tile_regions.len(),
            );
        }
    }

    // =========================================================================
    // Heap Size and Alignment
    // =========================================================================

    /// Get the size and alignment for a buffer in a heap.
    ///
    /// C++ equivalent: `SizeAndAlign heapBufferSizeAndAlign(NS::UInteger length, MTL::ResourceOptions options)`
    #[inline]
    pub fn heap_buffer_size_and_align(
        &self,
        length: UInteger,
        options: ResourceOptions,
    ) -> SizeAndAlign {
        unsafe {
            msg_send_2(
                self.as_ptr(),
                sel!(heapBufferSizeAndAlignWithLength: options:),
                length,
                options,
            )
        }
    }

    /// Get the size and alignment for a texture in a heap.
    ///
    /// C++ equivalent: `SizeAndAlign heapTextureSizeAndAlign(const MTL::TextureDescriptor* desc)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be a valid MTLTextureDescriptor.
    #[inline]
    pub unsafe fn heap_texture_size_and_align(&self, descriptor: *const c_void) -> SizeAndAlign {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(heapTextureSizeAndAlignWithDescriptor:),
                descriptor,
            )
        }
    }

    /// Get the size and alignment for an acceleration structure in a heap (by size).
    ///
    /// C++ equivalent: `SizeAndAlign heapAccelerationStructureSizeAndAlign(NS::UInteger size)`
    #[inline]
    pub fn heap_acceleration_structure_size_and_align_with_size(
        &self,
        size: UInteger,
    ) -> SizeAndAlign {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(heapAccelerationStructureSizeAndAlignWithSize:),
                size,
            )
        }
    }

    /// Get the size and alignment for an acceleration structure in a heap (by descriptor).
    ///
    /// C++ equivalent: `SizeAndAlign heapAccelerationStructureSizeAndAlign(const MTL::AccelerationStructureDescriptor*)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be a valid MTLAccelerationStructureDescriptor.
    #[inline]
    pub unsafe fn heap_acceleration_structure_size_and_align_with_descriptor(
        &self,
        descriptor: *const c_void,
    ) -> SizeAndAlign {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(heapAccelerationStructureSizeAndAlignWithDescriptor:),
                descriptor,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::system_default;

    #[test]
    fn test_buffer_limits() {
        let device = system_default().expect("no Metal device");

        let max_buffer = device.max_buffer_length();
        assert!(max_buffer > 0, "max buffer length should be positive");
        println!("Max buffer length: {} bytes", max_buffer);
    }

    #[test]
    fn test_thread_limits() {
        let device = system_default().expect("no Metal device");

        let max_threads = device.max_threads_per_threadgroup();
        let w = { max_threads.width };
        let h = { max_threads.height };
        let d = { max_threads.depth };
        assert!(w > 0 && h > 0 && d > 0);
        println!("Max threads per threadgroup: {}x{}x{}", w, h, d);

        let max_tg_memory = device.max_threadgroup_memory_length();
        assert!(max_tg_memory > 0);
        println!("Max threadgroup memory: {} bytes", max_tg_memory);
    }

    #[test]
    fn test_texture_alignment() {
        let device = system_default().expect("no Metal device");

        let alignment =
            device.minimum_linear_texture_alignment_for_pixel_format(PixelFormat::RGBA8_UNORM);
        assert!(alignment > 0);
        println!("RGBA8 linear texture alignment: {} bytes", alignment);
    }
}
