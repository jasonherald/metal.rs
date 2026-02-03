//! Configuration for creating textures.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{
    CPUCacheMode, HazardTrackingMode, PixelFormat, ResourceOptions, SparsePageSize, StorageMode,
    TextureCompressionType, TextureSwizzleChannels, TextureType, TextureUsage,
};

/// Configuration for creating textures.
///
/// C++ equivalent: `MTL::TextureDescriptor`
#[repr(transparent)]
pub struct TextureDescriptor(pub(crate) NonNull<c_void>);

impl TextureDescriptor {
    /// Create a new texture descriptor.
    ///
    /// C++ equivalent: `static TextureDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLTextureDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a descriptor for a 2D texture.
    ///
    /// C++ equivalent: `static TextureDescriptor* texture2DDescriptor(PixelFormat, NS::UInteger, NS::UInteger, bool)`
    pub fn texture_2d_descriptor(
        pixel_format: PixelFormat,
        width: UInteger,
        height: UInteger,
        mipmapped: bool,
    ) -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLTextureDescriptor")?;
            let ptr: *mut c_void = metal_sys::msg_send_4(
                class.as_ptr(),
                sel!(texture2DDescriptorWithPixelFormat: width: height: mipmapped:),
                pixel_format,
                width,
                height,
                mipmapped,
            );
            if ptr.is_null() {
                return None;
            }
            // Retain the autoreleased object
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Self::from_raw(ptr)
        }
    }

    /// Create a descriptor for a cube texture.
    ///
    /// C++ equivalent: `static TextureDescriptor* textureCubeDescriptor(PixelFormat, NS::UInteger, bool)`
    pub fn texture_cube_descriptor(
        pixel_format: PixelFormat,
        size: UInteger,
        mipmapped: bool,
    ) -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLTextureDescriptor")?;
            let ptr: *mut c_void = metal_sys::msg_send_3(
                class.as_ptr(),
                sel!(textureCubeDescriptorWithPixelFormat: size: mipmapped:),
                pixel_format,
                size,
                mipmapped,
            );
            if ptr.is_null() {
                return None;
            }
            // Retain the autoreleased object
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Self::from_raw(ptr)
        }
    }

    /// Create a descriptor for a texture buffer.
    ///
    /// C++ equivalent: `static TextureDescriptor* textureBufferDescriptor(PixelFormat, NS::UInteger, ResourceOptions, TextureUsage)`
    pub fn texture_buffer_descriptor(
        pixel_format: PixelFormat,
        width: UInteger,
        resource_options: ResourceOptions,
        usage: TextureUsage,
    ) -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLTextureDescriptor")?;
            let ptr: *mut c_void = metal_sys::msg_send_4(
                class.as_ptr(),
                sel!(textureBufferDescriptorWithPixelFormat: width: resourceOptions: usage:),
                pixel_format,
                width,
                resource_options,
                usage,
            );
            if ptr.is_null() {
                return None;
            }
            // Retain the autoreleased object
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Self::from_raw(ptr)
        }
    }

    /// Create a TextureDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal texture descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Properties - Getters
    // =========================================================================

    /// Get the texture type.
    #[inline]
    pub fn texture_type(&self) -> TextureType {
        unsafe { msg_send_0(self.as_ptr(), sel!(textureType)) }
    }

    /// Get the pixel format.
    #[inline]
    pub fn pixel_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(pixelFormat)) }
    }

    /// Get the width.
    #[inline]
    pub fn width(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(width)) }
    }

    /// Get the height.
    #[inline]
    pub fn height(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(height)) }
    }

    /// Get the depth.
    #[inline]
    pub fn depth(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(depth)) }
    }

    /// Get the mipmap level count.
    #[inline]
    pub fn mipmap_level_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(mipmapLevelCount)) }
    }

    /// Get the sample count.
    #[inline]
    pub fn sample_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(sampleCount)) }
    }

    /// Get the array length.
    #[inline]
    pub fn array_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(arrayLength)) }
    }

    /// Get the storage mode.
    #[inline]
    pub fn storage_mode(&self) -> StorageMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(storageMode)) }
    }

    /// Get the CPU cache mode.
    #[inline]
    pub fn cpu_cache_mode(&self) -> CPUCacheMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(cpuCacheMode)) }
    }

    /// Get the hazard tracking mode.
    #[inline]
    pub fn hazard_tracking_mode(&self) -> HazardTrackingMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(hazardTrackingMode)) }
    }

    /// Get the resource options.
    #[inline]
    pub fn resource_options(&self) -> ResourceOptions {
        unsafe { msg_send_0(self.as_ptr(), sel!(resourceOptions)) }
    }

    /// Get the usage flags.
    #[inline]
    pub fn usage(&self) -> TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(usage)) }
    }

    /// Get whether GPU-optimized contents are allowed.
    #[inline]
    pub fn allow_gpu_optimized_contents(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(allowGPUOptimizedContents)) }
    }

    /// Get the compression type.
    #[inline]
    pub fn compression_type(&self) -> TextureCompressionType {
        unsafe { msg_send_0(self.as_ptr(), sel!(compressionType)) }
    }

    /// Get the swizzle channels.
    #[inline]
    pub fn swizzle(&self) -> TextureSwizzleChannels {
        unsafe { msg_send_0(self.as_ptr(), sel!(swizzle)) }
    }

    /// Get the placement sparse page size.
    #[inline]
    pub fn placement_sparse_page_size(&self) -> SparsePageSize {
        unsafe { msg_send_0(self.as_ptr(), sel!(placementSparsePageSize)) }
    }

    // =========================================================================
    // Properties - Setters
    // =========================================================================

    /// Set the texture type.
    #[inline]
    pub fn set_texture_type(&self, texture_type: TextureType) {
        unsafe {
            msg_send_1::<(), TextureType>(self.as_ptr(), sel!(setTextureType:), texture_type);
        }
    }

    /// Set the pixel format.
    #[inline]
    pub fn set_pixel_format(&self, pixel_format: PixelFormat) {
        unsafe {
            msg_send_1::<(), PixelFormat>(self.as_ptr(), sel!(setPixelFormat:), pixel_format);
        }
    }

    /// Set the width.
    #[inline]
    pub fn set_width(&self, width: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setWidth:), width);
        }
    }

    /// Set the height.
    #[inline]
    pub fn set_height(&self, height: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setHeight:), height);
        }
    }

    /// Set the depth.
    #[inline]
    pub fn set_depth(&self, depth: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setDepth:), depth);
        }
    }

    /// Set the mipmap level count.
    #[inline]
    pub fn set_mipmap_level_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMipmapLevelCount:), count);
        }
    }

    /// Set the sample count.
    #[inline]
    pub fn set_sample_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setSampleCount:), count);
        }
    }

    /// Set the array length.
    #[inline]
    pub fn set_array_length(&self, length: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setArrayLength:), length);
        }
    }

    /// Set the storage mode.
    #[inline]
    pub fn set_storage_mode(&self, mode: StorageMode) {
        unsafe {
            msg_send_1::<(), StorageMode>(self.as_ptr(), sel!(setStorageMode:), mode);
        }
    }

    /// Set the CPU cache mode.
    #[inline]
    pub fn set_cpu_cache_mode(&self, mode: CPUCacheMode) {
        unsafe {
            msg_send_1::<(), CPUCacheMode>(self.as_ptr(), sel!(setCpuCacheMode:), mode);
        }
    }

    /// Set the hazard tracking mode.
    #[inline]
    pub fn set_hazard_tracking_mode(&self, mode: HazardTrackingMode) {
        unsafe {
            msg_send_1::<(), HazardTrackingMode>(self.as_ptr(), sel!(setHazardTrackingMode:), mode);
        }
    }

    /// Set the resource options.
    #[inline]
    pub fn set_resource_options(&self, options: ResourceOptions) {
        unsafe {
            msg_send_1::<(), ResourceOptions>(self.as_ptr(), sel!(setResourceOptions:), options);
        }
    }

    /// Set the usage flags.
    #[inline]
    pub fn set_usage(&self, usage: TextureUsage) {
        unsafe {
            msg_send_1::<(), TextureUsage>(self.as_ptr(), sel!(setUsage:), usage);
        }
    }

    /// Set whether GPU-optimized contents are allowed.
    #[inline]
    pub fn set_allow_gpu_optimized_contents(&self, allow: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setAllowGPUOptimizedContents:), allow);
        }
    }

    /// Set the compression type.
    #[inline]
    pub fn set_compression_type(&self, compression_type: TextureCompressionType) {
        unsafe {
            msg_send_1::<(), TextureCompressionType>(
                self.as_ptr(),
                sel!(setCompressionType:),
                compression_type,
            );
        }
    }

    /// Set the swizzle channels.
    #[inline]
    pub fn set_swizzle(&self, swizzle: TextureSwizzleChannels) {
        unsafe {
            msg_send_1::<(), TextureSwizzleChannels>(self.as_ptr(), sel!(setSwizzle:), swizzle);
        }
    }

    /// Set the placement sparse page size.
    #[inline]
    pub fn set_placement_sparse_page_size(&self, size: SparsePageSize) {
        unsafe {
            msg_send_1::<(), SparsePageSize>(
                self.as_ptr(),
                sel!(setPlacementSparsePageSize:),
                size,
            );
        }
    }
}

impl Default for TextureDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create texture descriptor")
    }
}

impl Clone for TextureDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy texture descriptor")
        }
    }
}

impl Drop for TextureDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for TextureDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for TextureDescriptor {}
unsafe impl Sync for TextureDescriptor {}

impl std::fmt::Debug for TextureDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextureDescriptor")
            .field("texture_type", &self.texture_type())
            .field("pixel_format", &self.pixel_format())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("depth", &self.depth())
            .finish()
    }
}
