//! TemporalScaler implementation.
//!
//! Corresponds to `MetalFX/MTLFXTemporalScaler.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

// ============================================================
// TemporalScalerDescriptor
// ============================================================

/// Descriptor for creating a temporal scaler.
///
/// C++ equivalent: `MTLFX::TemporalScalerDescriptor`
#[repr(transparent)]
pub struct TemporalScalerDescriptor(NonNull<c_void>);

impl TemporalScalerDescriptor {
    /// Create a TemporalScalerDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new temporal scaler descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLFXTemporalScalerDescriptor")?;
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
    pub fn color_texture_format(&self) -> mtl_gpu::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(colorTextureFormat)) }
    }

    /// Set the color texture format.
    pub fn set_color_texture_format(&self, format: mtl_gpu::PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setColorTextureFormat:), format);
        }
    }

    /// Get the depth texture format.
    pub fn depth_texture_format(&self) -> mtl_gpu::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(depthTextureFormat)) }
    }

    /// Set the depth texture format.
    pub fn set_depth_texture_format(&self, format: mtl_gpu::PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDepthTextureFormat:), format);
        }
    }

    /// Get the motion texture format.
    pub fn motion_texture_format(&self) -> mtl_gpu::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTextureFormat)) }
    }

    /// Set the motion texture format.
    pub fn set_motion_texture_format(&self, format: mtl_gpu::PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMotionTextureFormat:), format);
        }
    }

    /// Get the output texture format.
    pub fn output_texture_format(&self) -> mtl_gpu::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(outputTextureFormat)) }
    }

    /// Set the output texture format.
    pub fn set_output_texture_format(&self, format: mtl_gpu::PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOutputTextureFormat:), format);
        }
    }

    /// Get the reactive mask texture format.
    pub fn reactive_mask_texture_format(&self) -> mtl_gpu::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(reactiveMaskTextureFormat)) }
    }

    /// Set the reactive mask texture format.
    pub fn set_reactive_mask_texture_format(&self, format: mtl_gpu::PixelFormat) {
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

    /// Check if auto exposure is enabled.
    pub fn is_auto_exposure_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isAutoExposureEnabled)) }
    }

    /// Set auto exposure enabled.
    pub fn set_auto_exposure_enabled(&self, enabled: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setAutoExposureEnabled:), enabled);
        }
    }

    /// Check if input content properties are enabled.
    pub fn is_input_content_properties_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isInputContentPropertiesEnabled)) }
    }

    /// Set input content properties enabled.
    pub fn set_input_content_properties_enabled(&self, enabled: bool) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setInputContentPropertiesEnabled:),
                enabled,
            );
        }
    }

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

    /// Check if reactive mask texture is enabled.
    pub fn is_reactive_mask_texture_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isReactiveMaskTextureEnabled)) }
    }

    /// Set reactive mask texture enabled.
    pub fn set_reactive_mask_texture_enabled(&self, enabled: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setReactiveMaskTextureEnabled:), enabled);
        }
    }

    // ========== Scale Limits ==========

    /// Get the minimum input content scale.
    pub fn input_content_min_scale(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(inputContentMinScale)) }
    }

    /// Set the minimum input content scale.
    pub fn set_input_content_min_scale(&self, scale: f32) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setInputContentMinScale:), scale);
        }
    }

    /// Get the maximum input content scale.
    pub fn input_content_max_scale(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(inputContentMaxScale)) }
    }

    /// Set the maximum input content scale.
    pub fn set_input_content_max_scale(&self, scale: f32) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setInputContentMaxScale:), scale);
        }
    }

    // ========== Factory Methods ==========

    /// Create a new temporal scaler.
    pub fn new_temporal_scaler(&self, device: &mtl_gpu::Device) -> Option<TemporalScaler> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newTemporalScalerWithDevice:),
                device.as_ptr(),
            );
            TemporalScaler::from_raw(ptr)
        }
    }

    // ========== Static Methods ==========

    /// Check if a device supports temporal scaling.
    pub fn supports_device(device: &mtl_gpu::Device) -> bool {
        unsafe {
            let class = match mtl_sys::Class::get("MTLFXTemporalScalerDescriptor") {
                Some(c) => c,
                None => return false,
            };
            msg_send_1(class.as_ptr(), sel!(supportsDevice:), device.as_ptr())
        }
    }
}

impl Clone for TemporalScalerDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for TemporalScalerDescriptor {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for TemporalScalerDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for TemporalScalerDescriptor {}
unsafe impl Sync for TemporalScalerDescriptor {}

// ============================================================
// TemporalScaler
// ============================================================

/// AI-based temporal upscaler.
///
/// C++ equivalent: `MTLFX::TemporalScaler`
///
/// TemporalScaler uses machine learning with motion vectors to provide
/// high-quality temporal upscaling across multiple frames.
#[repr(transparent)]
pub struct TemporalScaler(NonNull<c_void>);

impl TemporalScaler {
    /// Create a TemporalScaler from a raw pointer.
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

    /// Get the required texture usage for the color texture.
    pub fn color_texture_usage(&self) -> mtl_gpu::TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(colorTextureUsage)) }
    }

    /// Get the required texture usage for the depth texture.
    pub fn depth_texture_usage(&self) -> mtl_gpu::TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(depthTextureUsage)) }
    }

    /// Get the required texture usage for the motion texture.
    pub fn motion_texture_usage(&self) -> mtl_gpu::TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTextureUsage)) }
    }

    /// Get the required texture usage for the output texture.
    pub fn output_texture_usage(&self) -> mtl_gpu::TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(outputTextureUsage)) }
    }

    /// Get the required texture usage for the reactive mask texture.
    pub fn reactive_texture_usage(&self) -> mtl_gpu::TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(reactiveTextureUsage)) }
    }

    // ========== Content Dimensions ==========

    /// Get the input content width.
    pub fn input_content_width(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(inputContentWidth)) }
    }

    /// Set the input content width.
    pub fn set_input_content_width(&self, width: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setInputContentWidth:), width);
        }
    }

    /// Get the input content height.
    pub fn input_content_height(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(inputContentHeight)) }
    }

    /// Set the input content height.
    pub fn set_input_content_height(&self, height: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setInputContentHeight:), height);
        }
    }

    // ========== Textures ==========

    /// Get the color texture.
    pub fn color_texture(&self) -> Option<mtl_gpu::Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(colorTexture));
            mtl_gpu::Texture::from_raw(ptr)
        }
    }

    /// Set the color texture.
    pub fn set_color_texture(&self, texture: &mtl_gpu::Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setColorTexture:), texture.as_ptr());
        }
    }

    /// Get the depth texture.
    pub fn depth_texture(&self) -> Option<mtl_gpu::Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(depthTexture));
            mtl_gpu::Texture::from_raw(ptr)
        }
    }

    /// Set the depth texture.
    pub fn set_depth_texture(&self, texture: &mtl_gpu::Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDepthTexture:), texture.as_ptr());
        }
    }

    /// Get the motion texture.
    pub fn motion_texture(&self) -> Option<mtl_gpu::Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(motionTexture));
            mtl_gpu::Texture::from_raw(ptr)
        }
    }

    /// Set the motion texture.
    pub fn set_motion_texture(&self, texture: &mtl_gpu::Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMotionTexture:), texture.as_ptr());
        }
    }

    /// Get the output texture.
    pub fn output_texture(&self) -> Option<mtl_gpu::Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(outputTexture));
            mtl_gpu::Texture::from_raw(ptr)
        }
    }

    /// Set the output texture.
    pub fn set_output_texture(&self, texture: &mtl_gpu::Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOutputTexture:), texture.as_ptr());
        }
    }

    /// Get the exposure texture.
    pub fn exposure_texture(&self) -> Option<mtl_gpu::Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(exposureTexture));
            mtl_gpu::Texture::from_raw(ptr)
        }
    }

    /// Set the exposure texture.
    pub fn set_exposure_texture(&self, texture: &mtl_gpu::Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setExposureTexture:), texture.as_ptr());
        }
    }

    /// Get the reactive mask texture.
    pub fn reactive_mask_texture(&self) -> Option<mtl_gpu::Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(reactiveMaskTexture));
            mtl_gpu::Texture::from_raw(ptr)
        }
    }

    /// Set the reactive mask texture.
    pub fn set_reactive_mask_texture(&self, texture: &mtl_gpu::Texture) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setReactiveMaskTexture:),
                texture.as_ptr(),
            );
        }
    }

    // ========== Exposure ==========

    /// Get the pre-exposure value.
    pub fn pre_exposure(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(preExposure)) }
    }

    /// Set the pre-exposure value.
    pub fn set_pre_exposure(&self, value: f32) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setPreExposure:), value);
        }
    }

    // ========== Jitter ==========

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

    // ========== Motion Vector Scale ==========

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

    // ========== State ==========

    /// Check if history should be reset.
    pub fn reset(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(reset)) }
    }

    /// Set whether history should be reset.
    pub fn set_reset(&self, reset: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setReset:), reset);
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

    // ========== Read-Only Properties ==========

    /// Get the color texture format.
    pub fn color_texture_format(&self) -> mtl_gpu::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(colorTextureFormat)) }
    }

    /// Get the depth texture format.
    pub fn depth_texture_format(&self) -> mtl_gpu::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(depthTextureFormat)) }
    }

    /// Get the motion texture format.
    pub fn motion_texture_format(&self) -> mtl_gpu::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTextureFormat)) }
    }

    /// Get the reactive texture format.
    pub fn reactive_texture_format(&self) -> mtl_gpu::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(reactiveTextureFormat)) }
    }

    /// Get the output texture format.
    pub fn output_texture_format(&self) -> mtl_gpu::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(outputTextureFormat)) }
    }

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

    /// Get the minimum input content scale.
    pub fn input_content_min_scale(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(inputContentMinScale)) }
    }

    /// Get the maximum input content scale.
    pub fn input_content_max_scale(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(inputContentMaxScale)) }
    }

    // ========== Fence ==========

    /// Get the fence.
    pub fn fence(&self) -> Option<mtl_gpu::Fence> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(fence));
            mtl_gpu::Fence::from_raw(ptr)
        }
    }

    /// Set the fence.
    pub fn set_fence(&self, fence: &mtl_gpu::Fence) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setFence:), fence.as_ptr());
        }
    }

    // ========== Encoding ==========

    /// Encode the scaling operation to a command buffer.
    pub fn encode_to_command_buffer(&self, command_buffer: &mtl_gpu::CommandBuffer) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(encodeToCommandBuffer:),
                command_buffer.as_ptr(),
            );
        }
    }

    /// Encode the scaling operation to an MTL4 command buffer.
    ///
    /// C++ equivalent: `void encodeToCommandBuffer(MTL4::CommandBuffer*)`
    ///
    /// This is the MTL4FX variant that works with Metal 4 command buffers.
    pub fn encode_to_mtl4_command_buffer(&self, command_buffer: &mtl_gpu::mtl4::CommandBuffer) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(encodeToCommandBuffer:),
                command_buffer.as_ptr(),
            );
        }
    }
}

impl Clone for TemporalScaler {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for TemporalScaler {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for TemporalScaler {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for TemporalScaler {}
unsafe impl Sync for TemporalScaler {}

impl std::fmt::Debug for TemporalScaler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemporalScaler")
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
    fn test_temporal_scaler_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<TemporalScalerDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_temporal_scaler_size() {
        assert_eq!(
            std::mem::size_of::<TemporalScaler>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_temporal_scaler_descriptor_creation() {
        // May fail if MetalFX is not available on the system
        if let Some(desc) = TemporalScalerDescriptor::new() {
            assert_eq!(desc.input_width(), 0);
            assert_eq!(desc.input_height(), 0);
        }
    }

    #[test]
    fn test_temporal_scaler_descriptor_properties() {
        if let Some(desc) = TemporalScalerDescriptor::new() {
            desc.set_input_width(1920);
            desc.set_input_height(1080);
            desc.set_output_width(3840);
            desc.set_output_height(2160);

            assert_eq!(desc.input_width(), 1920);
            assert_eq!(desc.input_height(), 1080);
            assert_eq!(desc.output_width(), 3840);
            assert_eq!(desc.output_height(), 2160);

            desc.set_auto_exposure_enabled(true);
            assert!(desc.is_auto_exposure_enabled());
        }
    }
}
