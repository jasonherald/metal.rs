//! MetalFX TemporalDenoisedScaler implementation.
//!
//! Provides AI-based temporal upscaling with ray tracing denoising.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal::{Device, Fence, PixelFormat, Texture, TextureUsage};
use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, sel};

// ============================================================
// TemporalDenoisedScalerDescriptor
// ============================================================

/// Descriptor for creating a temporal denoised scaler.
///
/// C++ equivalent: `MTLFX::TemporalDenoisedScalerDescriptor`
///
/// TemporalDenoisedScaler combines temporal upscaling with ray tracing
/// denoising for high-quality rendering with path tracing and ray-traced
/// effects.
#[repr(transparent)]
pub struct TemporalDenoisedScalerDescriptor(NonNull<c_void>);

impl TemporalDenoisedScalerDescriptor {
    /// Create from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLFXTemporalDenoisedScalerDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    // ========== Texture Formats ==========

    /// Get the color texture format.
    pub fn color_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(colorTextureFormat)) }
    }

    /// Set the color texture format.
    pub fn set_color_texture_format(&self, format: PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setColorTextureFormat:), format);
        }
    }

    /// Get the depth texture format.
    pub fn depth_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(depthTextureFormat)) }
    }

    /// Set the depth texture format.
    pub fn set_depth_texture_format(&self, format: PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDepthTextureFormat:), format);
        }
    }

    /// Get the motion texture format.
    pub fn motion_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTextureFormat)) }
    }

    /// Set the motion texture format.
    pub fn set_motion_texture_format(&self, format: PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMotionTextureFormat:), format);
        }
    }

    /// Get the diffuse albedo texture format.
    pub fn diffuse_albedo_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(diffuseAlbedoTextureFormat)) }
    }

    /// Set the diffuse albedo texture format.
    pub fn set_diffuse_albedo_texture_format(&self, format: PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDiffuseAlbedoTextureFormat:), format);
        }
    }

    /// Get the specular albedo texture format.
    pub fn specular_albedo_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(specularAlbedoTextureFormat)) }
    }

    /// Set the specular albedo texture format.
    pub fn set_specular_albedo_texture_format(&self, format: PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setSpecularAlbedoTextureFormat:), format);
        }
    }

    /// Get the normal texture format.
    pub fn normal_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(normalTextureFormat)) }
    }

    /// Set the normal texture format.
    pub fn set_normal_texture_format(&self, format: PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setNormalTextureFormat:), format);
        }
    }

    /// Get the roughness texture format.
    pub fn roughness_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(roughnessTextureFormat)) }
    }

    /// Set the roughness texture format.
    pub fn set_roughness_texture_format(&self, format: PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setRoughnessTextureFormat:), format);
        }
    }

    /// Get the specular hit distance texture format.
    pub fn specular_hit_distance_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(specularHitDistanceTextureFormat)) }
    }

    /// Set the specular hit distance texture format.
    pub fn set_specular_hit_distance_texture_format(&self, format: PixelFormat) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setSpecularHitDistanceTextureFormat:),
                format,
            );
        }
    }

    /// Get the denoise strength mask texture format.
    pub fn denoise_strength_mask_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(denoiseStrengthMaskTextureFormat)) }
    }

    /// Set the denoise strength mask texture format.
    pub fn set_denoise_strength_mask_texture_format(&self, format: PixelFormat) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setDenoiseStrengthMaskTextureFormat:),
                format,
            );
        }
    }

    /// Get the transparency overlay texture format.
    pub fn transparency_overlay_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(transparencyOverlayTextureFormat)) }
    }

    /// Set the transparency overlay texture format.
    pub fn set_transparency_overlay_texture_format(&self, format: PixelFormat) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setTransparencyOverlayTextureFormat:),
                format,
            );
        }
    }

    /// Get the output texture format.
    pub fn output_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(outputTextureFormat)) }
    }

    /// Set the output texture format.
    pub fn set_output_texture_format(&self, format: PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOutputTextureFormat:), format);
        }
    }

    /// Get the reactive mask texture format.
    pub fn reactive_mask_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(reactiveMaskTextureFormat)) }
    }

    /// Set the reactive mask texture format.
    pub fn set_reactive_mask_texture_format(&self, format: PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setReactiveMaskTextureFormat:), format);
        }
    }

    // ========== Dimensions ==========

    /// Get the input width.
    pub fn input_width(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(inputWidth)) }
    }

    /// Set the input width.
    pub fn set_input_width(&self, width: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setInputWidth:), width);
        }
    }

    /// Get the input height.
    pub fn input_height(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(inputHeight)) }
    }

    /// Set the input height.
    pub fn set_input_height(&self, height: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setInputHeight:), height);
        }
    }

    /// Get the output width.
    pub fn output_width(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(outputWidth)) }
    }

    /// Set the output width.
    pub fn set_output_width(&self, width: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOutputWidth:), width);
        }
    }

    /// Get the output height.
    pub fn output_height(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(outputHeight)) }
    }

    /// Set the output height.
    pub fn set_output_height(&self, height: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOutputHeight:), height);
        }
    }

    // ========== Options ==========

    /// Check if synchronous initialization is required.
    pub fn requires_synchronous_initialization(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(requiresSynchronousInitialization)) }
    }

    /// Set whether synchronous initialization is required.
    pub fn set_requires_synchronous_initialization(&self, required: bool) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setRequiresSynchronousInitialization:),
                required,
            );
        }
    }

    /// Check if auto exposure is enabled.
    pub fn is_auto_exposure_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isAutoExposureEnabled)) }
    }

    /// Set whether auto exposure is enabled.
    pub fn set_auto_exposure_enabled(&self, enabled: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setAutoExposureEnabled:), enabled);
        }
    }

    /// Check if input content properties are enabled.
    pub fn is_input_content_properties_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isInputContentPropertiesEnabled)) }
    }

    /// Set whether input content properties are enabled.
    pub fn set_input_content_properties_enabled(&self, enabled: bool) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setInputContentPropertiesEnabled:),
                enabled,
            );
        }
    }

    /// Get the input content minimum scale.
    pub fn input_content_min_scale(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(inputContentMinScale)) }
    }

    /// Set the input content minimum scale.
    pub fn set_input_content_min_scale(&self, scale: f32) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setInputContentMinScale:), scale);
        }
    }

    /// Get the input content maximum scale.
    pub fn input_content_max_scale(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(inputContentMaxScale)) }
    }

    /// Set the input content maximum scale.
    pub fn set_input_content_max_scale(&self, scale: f32) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setInputContentMaxScale:), scale);
        }
    }

    /// Check if reactive mask texture is enabled.
    pub fn is_reactive_mask_texture_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isReactiveMaskTextureEnabled)) }
    }

    /// Set whether reactive mask texture is enabled.
    pub fn set_reactive_mask_texture_enabled(&self, enabled: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setReactiveMaskTextureEnabled:), enabled);
        }
    }

    /// Check if specular hit distance texture is enabled.
    pub fn is_specular_hit_distance_texture_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isSpecularHitDistanceTextureEnabled)) }
    }

    /// Set whether specular hit distance texture is enabled.
    pub fn set_specular_hit_distance_texture_enabled(&self, enabled: bool) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setSpecularHitDistanceTextureEnabled:),
                enabled,
            );
        }
    }

    /// Check if denoise strength mask texture is enabled.
    pub fn is_denoise_strength_mask_texture_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isDenoiseStrengthMaskTextureEnabled)) }
    }

    /// Set whether denoise strength mask texture is enabled.
    pub fn set_denoise_strength_mask_texture_enabled(&self, enabled: bool) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setDenoiseStrengthMaskTextureEnabled:),
                enabled,
            );
        }
    }

    /// Check if transparency overlay texture is enabled.
    pub fn is_transparency_overlay_texture_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isTransparencyOverlayTextureEnabled)) }
    }

    /// Set whether transparency overlay texture is enabled.
    pub fn set_transparency_overlay_texture_enabled(&self, enabled: bool) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setTransparencyOverlayTextureEnabled:),
                enabled,
            );
        }
    }

    // ========== Factory Methods ==========

    /// Create a new temporal denoised scaler.
    pub fn new_temporal_denoised_scaler(&self, device: &Device) -> Option<TemporalDenoisedScaler> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(newTemporalDenoisedScaler:), device.as_ptr());
            TemporalDenoisedScaler::from_raw(ptr)
        }
    }

    /// Create a new MTL4 temporal denoised scaler.
    pub fn new_mtl4_temporal_denoised_scaler(
        &self,
        device: &Device,
        compiler: &metal::mtl4::Compiler,
    ) -> Option<TemporalDenoisedScaler> {
        unsafe {
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newTemporalDenoisedScaler:compiler:),
                device.as_ptr(),
                compiler.as_ptr(),
            );
            TemporalDenoisedScaler::from_raw(ptr)
        }
    }

    // ========== Static Methods ==========

    /// Get the supported input content minimum scale for a device.
    pub fn supported_input_content_min_scale(device: &Device) -> f32 {
        unsafe {
            let class = metal_sys::Class::get("MTLFXTemporalDenoisedScalerDescriptor")
                .expect("MTLFXTemporalDenoisedScalerDescriptor class not found");
            msg_send_1(
                class.as_ptr(),
                sel!(supportedInputContentMinScale:),
                device.as_ptr(),
            )
        }
    }

    /// Get the supported input content maximum scale for a device.
    pub fn supported_input_content_max_scale(device: &Device) -> f32 {
        unsafe {
            let class = metal_sys::Class::get("MTLFXTemporalDenoisedScalerDescriptor")
                .expect("MTLFXTemporalDenoisedScalerDescriptor class not found");
            msg_send_1(
                class.as_ptr(),
                sel!(supportedInputContentMaxScale:),
                device.as_ptr(),
            )
        }
    }

    /// Check if Metal 4 FX is supported on a device.
    pub fn supports_metal4_fx(device: &Device) -> bool {
        unsafe {
            let class = metal_sys::Class::get("MTLFXTemporalDenoisedScalerDescriptor")
                .expect("MTLFXTemporalDenoisedScalerDescriptor class not found");
            msg_send_1(class.as_ptr(), sel!(supportsMetal4FX:), device.as_ptr())
        }
    }

    /// Check if temporal denoised scaling is supported on a device.
    pub fn supports_device(device: &Device) -> bool {
        unsafe {
            let class = metal_sys::Class::get("MTLFXTemporalDenoisedScalerDescriptor")
                .expect("MTLFXTemporalDenoisedScalerDescriptor class not found");
            msg_send_1(class.as_ptr(), sel!(supportsDevice:), device.as_ptr())
        }
    }
}

impl Default for TemporalDenoisedScalerDescriptor {
    fn default() -> Self {
        Self::new().expect("Failed to create MTLFXTemporalDenoisedScalerDescriptor")
    }
}

impl Clone for TemporalDenoisedScalerDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for TemporalDenoisedScalerDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for TemporalDenoisedScalerDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for TemporalDenoisedScalerDescriptor {}
unsafe impl Sync for TemporalDenoisedScalerDescriptor {}

impl std::fmt::Debug for TemporalDenoisedScalerDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemporalDenoisedScalerDescriptor")
            .field("input_width", &self.input_width())
            .field("input_height", &self.input_height())
            .field("output_width", &self.output_width())
            .field("output_height", &self.output_height())
            .finish()
    }
}

// ============================================================
// TemporalDenoisedScaler
// ============================================================

/// AI-based temporal upscaling with ray tracing denoising.
///
/// C++ equivalent: `MTLFX::TemporalDenoisedScaler`
///
/// TemporalDenoisedScaler combines temporal upscaling with denoising
/// specifically designed for ray-traced effects like reflections,
/// global illumination, and ambient occlusion.
#[repr(transparent)]
pub struct TemporalDenoisedScaler(NonNull<c_void>);

impl TemporalDenoisedScaler {
    /// Create from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // ========== Texture Usage ==========

    /// Get the color texture usage requirements.
    pub fn color_texture_usage(&self) -> TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(colorTextureUsage)) }
    }

    /// Get the depth texture usage requirements.
    pub fn depth_texture_usage(&self) -> TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(depthTextureUsage)) }
    }

    /// Get the motion texture usage requirements.
    pub fn motion_texture_usage(&self) -> TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTextureUsage)) }
    }

    /// Get the reactive texture usage requirements.
    pub fn reactive_texture_usage(&self) -> TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(reactiveTextureUsage)) }
    }

    /// Get the diffuse albedo texture usage requirements.
    pub fn diffuse_albedo_texture_usage(&self) -> TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(diffuseAlbedoTextureUsage)) }
    }

    /// Get the specular albedo texture usage requirements.
    pub fn specular_albedo_texture_usage(&self) -> TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(specularAlbedoTextureUsage)) }
    }

    /// Get the normal texture usage requirements.
    pub fn normal_texture_usage(&self) -> TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(normalTextureUsage)) }
    }

    /// Get the roughness texture usage requirements.
    pub fn roughness_texture_usage(&self) -> TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(roughnessTextureUsage)) }
    }

    /// Get the specular hit distance texture usage requirements.
    pub fn specular_hit_distance_texture_usage(&self) -> TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(specularHitDistanceTextureUsage)) }
    }

    /// Get the denoise strength mask texture usage requirements.
    pub fn denoise_strength_mask_texture_usage(&self) -> TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(denoiseStrengthMaskTextureUsage)) }
    }

    /// Get the transparency overlay texture usage requirements.
    pub fn transparency_overlay_texture_usage(&self) -> TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(transparencyOverlayTextureUsage)) }
    }

    /// Get the output texture usage requirements.
    pub fn output_texture_usage(&self) -> TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(outputTextureUsage)) }
    }

    // ========== Textures ==========

    /// Get the color texture.
    pub fn color_texture(&self) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(colorTexture));
            Texture::from_raw(ptr)
        }
    }

    /// Set the color texture.
    pub fn set_color_texture(&self, texture: &Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setColorTexture:), texture.as_ptr());
        }
    }

    /// Get the depth texture.
    pub fn depth_texture(&self) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(depthTexture));
            Texture::from_raw(ptr)
        }
    }

    /// Set the depth texture.
    pub fn set_depth_texture(&self, texture: &Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDepthTexture:), texture.as_ptr());
        }
    }

    /// Get the motion texture.
    pub fn motion_texture(&self) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(motionTexture));
            Texture::from_raw(ptr)
        }
    }

    /// Set the motion texture.
    pub fn set_motion_texture(&self, texture: &Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMotionTexture:), texture.as_ptr());
        }
    }

    /// Get the diffuse albedo texture.
    pub fn diffuse_albedo_texture(&self) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(diffuseAlbedoTexture));
            Texture::from_raw(ptr)
        }
    }

    /// Set the diffuse albedo texture.
    pub fn set_diffuse_albedo_texture(&self, texture: &Texture) {
        unsafe {
            let _: () =
                msg_send_1(self.as_ptr(), sel!(setDiffuseAlbedoTexture:), texture.as_ptr());
        }
    }

    /// Get the specular albedo texture.
    pub fn specular_albedo_texture(&self) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(specularAlbedoTexture));
            Texture::from_raw(ptr)
        }
    }

    /// Set the specular albedo texture.
    pub fn set_specular_albedo_texture(&self, texture: &Texture) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setSpecularAlbedoTexture:),
                texture.as_ptr(),
            );
        }
    }

    /// Get the normal texture.
    pub fn normal_texture(&self) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(normalTexture));
            Texture::from_raw(ptr)
        }
    }

    /// Set the normal texture.
    pub fn set_normal_texture(&self, texture: &Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setNormalTexture:), texture.as_ptr());
        }
    }

    /// Get the roughness texture.
    pub fn roughness_texture(&self) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(roughnessTexture));
            Texture::from_raw(ptr)
        }
    }

    /// Set the roughness texture.
    pub fn set_roughness_texture(&self, texture: &Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setRoughnessTexture:), texture.as_ptr());
        }
    }

    /// Get the specular hit distance texture.
    pub fn specular_hit_distance_texture(&self) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(specularHitDistanceTexture));
            Texture::from_raw(ptr)
        }
    }

    /// Set the specular hit distance texture.
    pub fn set_specular_hit_distance_texture(&self, texture: &Texture) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setSpecularHitDistanceTexture:),
                texture.as_ptr(),
            );
        }
    }

    /// Get the denoise strength mask texture.
    pub fn denoise_strength_mask_texture(&self) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(denoiseStrengthMaskTexture));
            Texture::from_raw(ptr)
        }
    }

    /// Set the denoise strength mask texture.
    pub fn set_denoise_strength_mask_texture(&self, texture: &Texture) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setDenoiseStrengthMaskTexture:),
                texture.as_ptr(),
            );
        }
    }

    /// Get the transparency overlay texture.
    pub fn transparency_overlay_texture(&self) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(transparencyOverlayTexture));
            Texture::from_raw(ptr)
        }
    }

    /// Set the transparency overlay texture.
    pub fn set_transparency_overlay_texture(&self, texture: &Texture) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setTransparencyOverlayTexture:),
                texture.as_ptr(),
            );
        }
    }

    /// Get the output texture.
    pub fn output_texture(&self) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(outputTexture));
            Texture::from_raw(ptr)
        }
    }

    /// Set the output texture.
    pub fn set_output_texture(&self, texture: &Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOutputTexture:), texture.as_ptr());
        }
    }

    /// Get the exposure texture.
    pub fn exposure_texture(&self) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(exposureTexture));
            Texture::from_raw(ptr)
        }
    }

    /// Set the exposure texture.
    pub fn set_exposure_texture(&self, texture: &Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setExposureTexture:), texture.as_ptr());
        }
    }

    /// Get the reactive mask texture.
    pub fn reactive_mask_texture(&self) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(reactiveMaskTexture));
            Texture::from_raw(ptr)
        }
    }

    /// Set the reactive mask texture.
    pub fn set_reactive_mask_texture(&self, texture: &Texture) {
        unsafe {
            let _: () =
                msg_send_1(self.as_ptr(), sel!(setReactiveMaskTexture:), texture.as_ptr());
        }
    }

    // ========== Properties ==========

    /// Get the pre-exposure value.
    pub fn pre_exposure(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(preExposure)) }
    }

    /// Set the pre-exposure value.
    pub fn set_pre_exposure(&self, pre_exposure: f32) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setPreExposure:), pre_exposure);
        }
    }

    /// Get the jitter offset X.
    pub fn jitter_offset_x(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(jitterOffsetX)) }
    }

    /// Set the jitter offset X.
    pub fn set_jitter_offset_x(&self, offset: f32) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setJitterOffsetX:), offset);
        }
    }

    /// Get the jitter offset Y.
    pub fn jitter_offset_y(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(jitterOffsetY)) }
    }

    /// Set the jitter offset Y.
    pub fn set_jitter_offset_y(&self, offset: f32) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setJitterOffsetY:), offset);
        }
    }

    /// Get the motion vector scale X.
    pub fn motion_vector_scale_x(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionVectorScaleX)) }
    }

    /// Set the motion vector scale X.
    pub fn set_motion_vector_scale_x(&self, scale: f32) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMotionVectorScaleX:), scale);
        }
    }

    /// Get the motion vector scale Y.
    pub fn motion_vector_scale_y(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionVectorScaleY)) }
    }

    /// Set the motion vector scale Y.
    pub fn set_motion_vector_scale_y(&self, scale: f32) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMotionVectorScaleY:), scale);
        }
    }

    /// Check if history should be reset.
    pub fn should_reset_history(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(shouldResetHistory)) }
    }

    /// Set whether history should be reset.
    pub fn set_should_reset_history(&self, reset: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setShouldResetHistory:), reset);
        }
    }

    /// Check if depth is reversed.
    pub fn is_depth_reversed(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isDepthReversed)) }
    }

    /// Set whether depth is reversed.
    pub fn set_depth_reversed(&self, reversed: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDepthReversed:), reversed);
        }
    }

    // ========== Texture Formats (read-only) ==========

    /// Get the color texture format.
    pub fn color_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(colorTextureFormat)) }
    }

    /// Get the depth texture format.
    pub fn depth_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(depthTextureFormat)) }
    }

    /// Get the motion texture format.
    pub fn motion_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTextureFormat)) }
    }

    /// Get the diffuse albedo texture format.
    pub fn diffuse_albedo_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(diffuseAlbedoTextureFormat)) }
    }

    /// Get the specular albedo texture format.
    pub fn specular_albedo_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(specularAlbedoTextureFormat)) }
    }

    /// Get the normal texture format.
    pub fn normal_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(normalTextureFormat)) }
    }

    /// Get the roughness texture format.
    pub fn roughness_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(roughnessTextureFormat)) }
    }

    /// Get the specular hit distance texture format.
    pub fn specular_hit_distance_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(specularHitDistanceTextureFormat)) }
    }

    /// Get the denoise strength mask texture format.
    pub fn denoise_strength_mask_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(denoiseStrengthMaskTextureFormat)) }
    }

    /// Get the transparency overlay texture format.
    pub fn transparency_overlay_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(transparencyOverlayTextureFormat)) }
    }

    /// Get the reactive mask texture format.
    pub fn reactive_mask_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(reactiveMaskTextureFormat)) }
    }

    /// Get the output texture format.
    pub fn output_texture_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(outputTextureFormat)) }
    }

    // ========== Dimensions (read-only) ==========

    /// Get the input width.
    pub fn input_width(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(inputWidth)) }
    }

    /// Get the input height.
    pub fn input_height(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(inputHeight)) }
    }

    /// Get the output width.
    pub fn output_width(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(outputWidth)) }
    }

    /// Get the output height.
    pub fn output_height(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(outputHeight)) }
    }

    /// Get the input content minimum scale.
    pub fn input_content_min_scale(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(inputContentMinScale)) }
    }

    /// Get the input content maximum scale.
    pub fn input_content_max_scale(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(inputContentMaxScale)) }
    }

    // ========== Matrix Properties ==========

    /// Get the world to view matrix (as raw 16 floats).
    pub fn world_to_view_matrix_raw(&self) -> [f32; 16] {
        unsafe {
            let mut result = [0.0f32; 16];
            let matrix: [f32; 16] = msg_send_0(self.as_ptr(), sel!(worldToViewMatrix));
            result.copy_from_slice(&matrix);
            result
        }
    }

    /// Set the world to view matrix (from raw 16 floats).
    pub fn set_world_to_view_matrix_raw(&self, matrix: &[f32; 16]) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setWorldToViewMatrix:), *matrix);
        }
    }

    /// Get the view to clip matrix (as raw 16 floats).
    pub fn view_to_clip_matrix_raw(&self) -> [f32; 16] {
        unsafe {
            let mut result = [0.0f32; 16];
            let matrix: [f32; 16] = msg_send_0(self.as_ptr(), sel!(viewToClipMatrix));
            result.copy_from_slice(&matrix);
            result
        }
    }

    /// Set the view to clip matrix (from raw 16 floats).
    pub fn set_view_to_clip_matrix_raw(&self, matrix: &[f32; 16]) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setViewToClipMatrix:), *matrix);
        }
    }

    // ========== Fence ==========

    /// Get the fence.
    pub fn fence(&self) -> Option<Fence> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(fence));
            Fence::from_raw(ptr)
        }
    }

    /// Set the fence.
    pub fn set_fence(&self, fence: &Fence) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setFence:), fence.as_ptr());
        }
    }

    // ========== Encoding ==========

    /// Encode the scaling operation to a command buffer.
    pub fn encode_to_command_buffer(&self, command_buffer: &metal::CommandBuffer) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(encodeToCommandBuffer:),
                command_buffer.as_ptr(),
            );
        }
    }

    /// Encode the scaling operation to a MTL4 command buffer.
    pub fn encode_to_mtl4_command_buffer(&self, command_buffer: &metal::mtl4::CommandBuffer) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(encodeToCommandBuffer:),
                command_buffer.as_ptr(),
            );
        }
    }
}

impl Clone for TemporalDenoisedScaler {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for TemporalDenoisedScaler {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for TemporalDenoisedScaler {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for TemporalDenoisedScaler {}
unsafe impl Sync for TemporalDenoisedScaler {}

impl std::fmt::Debug for TemporalDenoisedScaler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemporalDenoisedScaler")
            .field("input_width", &self.input_width())
            .field("input_height", &self.input_height())
            .field("output_width", &self.output_width())
            .field("output_height", &self.output_height())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temporal_denoised_scaler_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<TemporalDenoisedScalerDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_temporal_denoised_scaler_size() {
        assert_eq!(
            std::mem::size_of::<TemporalDenoisedScaler>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
