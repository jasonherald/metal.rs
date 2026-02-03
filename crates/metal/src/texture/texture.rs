//! A texture resource that stores formatted image data.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{
    PixelFormat, ResourceOptions, TextureCompressionType, TextureSparseTier,
    TextureSwizzleChannels, TextureType, TextureUsage,
};
use crate::types::ResourceID;

use super::{SharedTextureHandle, TextureViewDescriptor};

/// A texture resource that stores formatted image data.
///
/// C++ equivalent: `MTL::Texture`
#[repr(transparent)]
pub struct Texture(pub(crate) NonNull<c_void>);

impl Texture {
    /// Create a Texture from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal texture object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the texture.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Texture Properties
    // =========================================================================

    /// Get the texture type.
    ///
    /// C++ equivalent: `TextureType textureType() const`
    #[inline]
    pub fn texture_type(&self) -> TextureType {
        unsafe { msg_send_0(self.as_ptr(), sel!(textureType)) }
    }

    /// Get the pixel format.
    ///
    /// C++ equivalent: `PixelFormat pixelFormat() const`
    #[inline]
    pub fn pixel_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(pixelFormat)) }
    }

    /// Get the width of the texture.
    ///
    /// C++ equivalent: `NS::UInteger width() const`
    #[inline]
    pub fn width(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(width)) }
    }

    /// Get the height of the texture.
    ///
    /// C++ equivalent: `NS::UInteger height() const`
    #[inline]
    pub fn height(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(height)) }
    }

    /// Get the depth of the texture.
    ///
    /// C++ equivalent: `NS::UInteger depth() const`
    #[inline]
    pub fn depth(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(depth)) }
    }

    /// Get the number of mipmap levels.
    ///
    /// C++ equivalent: `NS::UInteger mipmapLevelCount() const`
    #[inline]
    pub fn mipmap_level_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(mipmapLevelCount)) }
    }

    /// Get the sample count (for multisampled textures).
    ///
    /// C++ equivalent: `NS::UInteger sampleCount() const`
    #[inline]
    pub fn sample_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(sampleCount)) }
    }

    /// Get the array length (for texture arrays).
    ///
    /// C++ equivalent: `NS::UInteger arrayLength() const`
    #[inline]
    pub fn array_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(arrayLength)) }
    }

    /// Get the texture usage flags.
    ///
    /// C++ equivalent: `TextureUsage usage() const`
    #[inline]
    pub fn usage(&self) -> TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(usage)) }
    }

    /// Check if this is a shareable texture.
    ///
    /// C++ equivalent: `bool isShareable() const`
    #[inline]
    pub fn is_shareable(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isShareable)) }
    }

    /// Check if this is a framebuffer-only texture.
    ///
    /// C++ equivalent: `bool isFramebufferOnly() const`
    #[inline]
    pub fn is_framebuffer_only(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isFramebufferOnly)) }
    }

    /// Get the first mipmap level used in a texture view.
    ///
    /// C++ equivalent: `NS::UInteger firstMipmapInTail() const`
    #[inline]
    pub fn first_mipmap_in_tail(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(firstMipmapInTail)) }
    }

    /// Get the tail size in bytes for sparse textures.
    ///
    /// C++ equivalent: `NS::UInteger tailSizeInBytes() const`
    #[inline]
    pub fn tail_size_in_bytes(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(tailSizeInBytes)) }
    }

    /// Check if the tail is sparse.
    ///
    /// C++ equivalent: `bool isSparse() const`
    #[inline]
    pub fn is_sparse(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isSparse)) }
    }

    /// Get the GPU resource ID for bindless access.
    ///
    /// C++ equivalent: `ResourceID gpuResourceID() const`
    #[inline]
    pub fn gpu_resource_id(&self) -> ResourceID {
        unsafe { msg_send_0(self.as_ptr(), sel!(gpuResourceID)) }
    }

    /// Check if texture allows GPU-optimized contents.
    ///
    /// C++ equivalent: `bool allowGPUOptimizedContents() const`
    #[inline]
    pub fn allow_gpu_optimized_contents(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(allowGPUOptimizedContents)) }
    }

    /// Get the texture compression type.
    ///
    /// C++ equivalent: `TextureCompressionType compressionType() const`
    #[inline]
    pub fn compression_type(&self) -> TextureCompressionType {
        unsafe { msg_send_0(self.as_ptr(), sel!(compressionType)) }
    }

    /// Get the texture swizzle channels.
    ///
    /// C++ equivalent: `TextureSwizzleChannels swizzle() const`
    #[inline]
    pub fn swizzle(&self) -> TextureSwizzleChannels {
        unsafe { msg_send_0(self.as_ptr(), sel!(swizzle)) }
    }

    /// Get the parent texture (if this is a view).
    ///
    /// C++ equivalent: `Texture* parentTexture() const`
    pub fn parent_texture(&self) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(parentTexture));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Texture::from_raw(ptr)
        }
    }

    /// Get the parent relative level (for texture views).
    ///
    /// C++ equivalent: `NS::UInteger parentRelativeLevel() const`
    #[inline]
    pub fn parent_relative_level(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(parentRelativeLevel)) }
    }

    /// Get the parent relative slice (for texture views).
    ///
    /// C++ equivalent: `NS::UInteger parentRelativeSlice() const`
    #[inline]
    pub fn parent_relative_slice(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(parentRelativeSlice)) }
    }

    /// Get the buffer (if texture is backed by a buffer).
    ///
    /// C++ equivalent: `Buffer* buffer() const`
    pub fn buffer(&self) -> Option<crate::buffer::Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(buffer));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::buffer::Buffer::from_raw(ptr)
        }
    }

    /// Get the buffer offset (if texture is backed by a buffer).
    ///
    /// C++ equivalent: `NS::UInteger bufferOffset() const`
    #[inline]
    pub fn buffer_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(bufferOffset)) }
    }

    /// Get the buffer bytes per row (if texture is backed by a buffer).
    ///
    /// C++ equivalent: `NS::UInteger bufferBytesPerRow() const`
    #[inline]
    pub fn buffer_bytes_per_row(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(bufferBytesPerRow)) }
    }

    /// Get the IOSurface backing this texture, if any.
    ///
    /// C++ equivalent: `IOSurfaceRef iosurface() const`
    ///
    /// Returns an opaque pointer to the IOSurfaceRef, or None if the texture
    /// is not backed by an IOSurface.
    #[inline]
    pub fn iosurface(&self) -> Option<*mut c_void> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(iosurface));
            if ptr.is_null() { None } else { Some(ptr) }
        }
    }

    /// Get the IOSurface plane index for this texture.
    ///
    /// C++ equivalent: `NS::UInteger iosurfacePlane() const`
    #[inline]
    pub fn iosurface_plane(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(iosurfacePlane)) }
    }

    /// Get the sparse texture tier for this texture.
    ///
    /// C++ equivalent: `TextureSparseTier sparseTextureTier() const`
    #[inline]
    pub fn sparse_texture_tier(&self) -> TextureSparseTier {
        unsafe { msg_send_0(self.as_ptr(), sel!(sparseTextureTier)) }
    }

    /// Get the remote storage texture (for cross-process sharing).
    ///
    /// C++ equivalent: `Texture* remoteStorageTexture() const`
    pub fn remote_storage_texture(&self) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(remoteStorageTexture));
            if ptr.is_null() {
                return None;
            }
            // Retain the autoreleased object
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Texture::from_raw(ptr)
        }
    }

    /// Create a remote texture view for a different device.
    ///
    /// C++ equivalent: `Texture* newRemoteTextureViewForDevice(const Device*)`
    pub fn new_remote_texture_view_for_device(&self, device: &crate::Device) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newRemoteTextureViewForDevice:),
                device.as_ptr(),
            );
            Texture::from_raw(ptr)
        }
    }

    // =========================================================================
    // Texture View Creation
    // =========================================================================

    /// Create a texture view with a different pixel format.
    ///
    /// C++ equivalent: `Texture* newTextureView(PixelFormat)`
    pub fn new_texture_view_with_pixel_format(&self, pixel_format: PixelFormat) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newTextureViewWithPixelFormat:),
                pixel_format,
            );
            Texture::from_raw(ptr)
        }
    }

    /// Create a texture view with different pixel format and texture type.
    ///
    /// C++ equivalent: `Texture* newTextureView(PixelFormat, TextureType, NS::Range levels, NS::Range slices)`
    pub fn new_texture_view(
        &self,
        pixel_format: PixelFormat,
        texture_type: TextureType,
        level_range: metal_foundation::Range,
        slice_range: metal_foundation::Range,
    ) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = metal_sys::msg_send_4(
                self.as_ptr(),
                sel!(newTextureViewWithPixelFormat: textureType: levels: slices:),
                pixel_format,
                texture_type,
                level_range,
                slice_range,
            );
            Texture::from_raw(ptr)
        }
    }

    /// Create a texture view with swizzle.
    ///
    /// C++ equivalent: `Texture* newTextureView(PixelFormat, TextureType, NS::Range, NS::Range, TextureSwizzleChannels)`
    pub fn new_texture_view_with_swizzle(
        &self,
        pixel_format: PixelFormat,
        texture_type: TextureType,
        level_range: metal_foundation::Range,
        slice_range: metal_foundation::Range,
        swizzle: TextureSwizzleChannels,
    ) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = metal_sys::msg_send_5(
                self.as_ptr(),
                sel!(newTextureViewWithPixelFormat: textureType: levels: slices: swizzle:),
                pixel_format,
                texture_type,
                level_range,
                slice_range,
                swizzle,
            );
            Texture::from_raw(ptr)
        }
    }

    /// Create a shared texture handle for cross-process sharing.
    ///
    /// C++ equivalent: `SharedTextureHandle* newSharedTextureHandle()`
    pub fn new_shared_texture_handle(&self) -> Option<SharedTextureHandle> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(newSharedTextureHandle));
            SharedTextureHandle::from_raw(ptr)
        }
    }

    /// Create a texture view with a descriptor.
    ///
    /// C++ equivalent: `Texture* newTextureView(const TextureViewDescriptor*)`
    pub fn new_texture_view_with_descriptor(
        &self,
        descriptor: &TextureViewDescriptor,
    ) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newTextureViewWithDescriptor:),
                descriptor.as_ptr(),
            );
            Texture::from_raw(ptr)
        }
    }

    // =========================================================================
    // Texture Data Operations
    // =========================================================================

    /// Get texture data bytes (full version with slice).
    ///
    /// C++ equivalent: `void getBytes(void*, NS::UInteger, NS::UInteger, MTL::Region, NS::UInteger, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The `pixel_bytes` pointer must be valid for writing the requested data.
    pub unsafe fn get_bytes(
        &self,
        pixel_bytes: *mut c_void,
        bytes_per_row: UInteger,
        bytes_per_image: UInteger,
        region: crate::types::Region,
        mipmap_level: UInteger,
        slice: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_6::<
                (),
                *mut c_void,
                UInteger,
                UInteger,
                crate::types::Region,
                UInteger,
                UInteger,
            >(
                self.as_ptr(),
                sel!(getBytes: bytesPerRow: bytesPerImage: fromRegion: mipmapLevel: slice:),
                pixel_bytes,
                bytes_per_row,
                bytes_per_image,
                region,
                mipmap_level,
                slice,
            );
        }
    }

    /// Get texture data bytes (simple version for 2D textures).
    ///
    /// C++ equivalent: `void getBytes(void*, NS::UInteger, MTL::Region, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The `pixel_bytes` pointer must be valid for writing the requested data.
    pub unsafe fn get_bytes_simple(
        &self,
        pixel_bytes: *mut c_void,
        bytes_per_row: UInteger,
        region: crate::types::Region,
        mipmap_level: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_4::<(), *mut c_void, UInteger, crate::types::Region, UInteger>(
                self.as_ptr(),
                sel!(getBytes: bytesPerRow: fromRegion: mipmapLevel:),
                pixel_bytes,
                bytes_per_row,
                region,
                mipmap_level,
            );
        }
    }

    /// Replace a region of the texture (full version with slice).
    ///
    /// C++ equivalent: `void replaceRegion(MTL::Region, NS::UInteger, NS::UInteger, const void*, NS::UInteger, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The `pixel_bytes` pointer must be valid for reading the data to copy.
    pub unsafe fn replace_region(
        &self,
        region: crate::types::Region,
        mipmap_level: UInteger,
        slice: UInteger,
        pixel_bytes: *const c_void,
        bytes_per_row: UInteger,
        bytes_per_image: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_6::<
                (),
                crate::types::Region,
                UInteger,
                UInteger,
                *const c_void,
                UInteger,
                UInteger,
            >(
                self.as_ptr(),
                sel!(replaceRegion: mipmapLevel: slice: withBytes: bytesPerRow: bytesPerImage:),
                region,
                mipmap_level,
                slice,
                pixel_bytes,
                bytes_per_row,
                bytes_per_image,
            );
        }
    }

    /// Replace a region of the texture (simple version for 2D textures).
    ///
    /// C++ equivalent: `void replaceRegion(MTL::Region, NS::UInteger, const void*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The `pixel_bytes` pointer must be valid for reading the data to copy.
    pub unsafe fn replace_region_simple(
        &self,
        region: crate::types::Region,
        mipmap_level: UInteger,
        pixel_bytes: *const c_void,
        bytes_per_row: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_4::<(), crate::types::Region, UInteger, *const c_void, UInteger>(
                self.as_ptr(),
                sel!(replaceRegion: mipmapLevel: withBytes: bytesPerRow:),
                region,
                mipmap_level,
                pixel_bytes,
                bytes_per_row,
            );
        }
    }

    // =========================================================================
    // Resource Properties (inherited from MTLResource)
    // =========================================================================

    /// Get the label for this texture.
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

    /// Set the label for this texture.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the device that created this texture.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("texture has no device")
        }
    }

    /// Get the resource options.
    ///
    /// C++ equivalent: `ResourceOptions resourceOptions() const`
    #[inline]
    pub fn resource_options(&self) -> ResourceOptions {
        unsafe { msg_send_0(self.as_ptr(), sel!(resourceOptions)) }
    }

    /// Get the allocated size.
    ///
    /// C++ equivalent: `NS::UInteger allocatedSize() const`
    #[inline]
    pub fn allocated_size(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(allocatedSize)) }
    }
}

impl Clone for Texture {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for Texture {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for Texture {}
unsafe impl Sync for Texture {}

impl std::fmt::Debug for Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Texture")
            .field("texture_type", &self.texture_type())
            .field("pixel_format", &self.pixel_format())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("depth", &self.depth())
            .field("label", &self.label())
            .finish()
    }
}
