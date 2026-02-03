//! FrameInterpolator implementation.
//!
//! Corresponds to `MetalFX/MTLFXFrameInterpolator.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

// ============================================================
// FrameInterpolatorDescriptor
// ============================================================

/// Descriptor for creating a frame interpolator.
///
/// C++ equivalent: `MTLFX::FrameInterpolatorDescriptor`
#[repr(transparent)]
pub struct FrameInterpolatorDescriptor(NonNull<c_void>);

impl FrameInterpolatorDescriptor {
    /// Create a FrameInterpolatorDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new frame interpolator descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLFXFrameInterpolatorDescriptor")?;
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
    pub fn color_texture_format(&self) -> metal::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(colorTextureFormat)) }
    }

    /// Set the color texture format.
    pub fn set_color_texture_format(&self, format: metal::PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setColorTextureFormat:), format);
        }
    }

    /// Get the output texture format.
    pub fn output_texture_format(&self) -> metal::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(outputTextureFormat)) }
    }

    /// Set the output texture format.
    pub fn set_output_texture_format(&self, format: metal::PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOutputTextureFormat:), format);
        }
    }

    /// Get the depth texture format.
    pub fn depth_texture_format(&self) -> metal::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(depthTextureFormat)) }
    }

    /// Set the depth texture format.
    pub fn set_depth_texture_format(&self, format: metal::PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDepthTextureFormat:), format);
        }
    }

    /// Get the motion texture format.
    pub fn motion_texture_format(&self) -> metal::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTextureFormat)) }
    }

    /// Set the motion texture format.
    pub fn set_motion_texture_format(&self, format: metal::PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMotionTextureFormat:), format);
        }
    }

    /// Get the UI texture format.
    pub fn ui_texture_format(&self) -> metal::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(uiTextureFormat)) }
    }

    /// Set the UI texture format.
    pub fn set_ui_texture_format(&self, format: metal::PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setUITextureFormat:), format);
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

    // ========== Scaler ==========

    /// Get the scaler object.
    ///
    /// Returns a raw pointer to the MTLFX::FrameInterpolatableScaler.
    ///
    /// C++ equivalent: `MTLFX::FrameInterpolatableScaler* scaler() const`
    pub fn scaler_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(scaler)) }
    }

    /// Set the scaler object.
    ///
    /// Takes a raw pointer to an MTLFX::FrameInterpolatableScaler.
    ///
    /// C++ equivalent: `void setScaler(MTLFX::FrameInterpolatableScaler*)`
    ///
    /// # Safety
    ///
    /// The scaler pointer must be valid or null.
    pub unsafe fn set_scaler_raw(&self, scaler: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setScaler:), scaler);
        }
    }

    // ========== Factory Methods ==========

    /// Create a new frame interpolator.
    pub fn new_frame_interpolator(&self, device: &metal::Device) -> Option<FrameInterpolator> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newFrameInterpolatorWithDevice:),
                device.as_ptr(),
            );
            FrameInterpolator::from_raw(ptr)
        }
    }

    // ========== Static Methods ==========

    /// Check if a device supports frame interpolation.
    pub fn supports_device(device: &metal::Device) -> bool {
        unsafe {
            let class = match metal_sys::Class::get("MTLFXFrameInterpolatorDescriptor") {
                Some(c) => c,
                None => return false,
            };
            msg_send_1(class.as_ptr(), sel!(supportsDevice:), device.as_ptr())
        }
    }
}

impl Clone for FrameInterpolatorDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for FrameInterpolatorDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for FrameInterpolatorDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for FrameInterpolatorDescriptor {}
unsafe impl Sync for FrameInterpolatorDescriptor {}

// ============================================================
// FrameInterpolator
// ============================================================

/// AI-based frame interpolator for generating intermediate frames.
///
/// C++ equivalent: `MTLFX::FrameInterpolator`
///
/// FrameInterpolator uses machine learning to generate intermediate frames
/// between rendered frames, providing smoother gameplay at higher frame rates.
#[repr(transparent)]
pub struct FrameInterpolator(NonNull<c_void>);

impl FrameInterpolator {
    /// Create a FrameInterpolator from a raw pointer.
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
    pub fn color_texture_usage(&self) -> metal::TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(colorTextureUsage)) }
    }

    /// Get the required texture usage for the output texture.
    pub fn output_texture_usage(&self) -> metal::TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(outputTextureUsage)) }
    }

    /// Get the required texture usage for the depth texture.
    pub fn depth_texture_usage(&self) -> metal::TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(depthTextureUsage)) }
    }

    /// Get the required texture usage for the motion texture.
    pub fn motion_texture_usage(&self) -> metal::TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTextureUsage)) }
    }

    /// Get the required texture usage for the UI texture.
    pub fn ui_texture_usage(&self) -> metal::TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(uiTextureUsage)) }
    }

    // ========== Read-Only Format Properties ==========

    /// Get the color texture format.
    pub fn color_texture_format(&self) -> metal::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(colorTextureFormat)) }
    }

    /// Get the depth texture format.
    pub fn depth_texture_format(&self) -> metal::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(depthTextureFormat)) }
    }

    /// Get the motion texture format.
    pub fn motion_texture_format(&self) -> metal::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTextureFormat)) }
    }

    /// Get the output texture format.
    pub fn output_texture_format(&self) -> metal::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(outputTextureFormat)) }
    }

    /// Get the UI texture format.
    pub fn ui_texture_format(&self) -> metal::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(uiTextureFormat)) }
    }

    // ========== Read-Only Dimension Properties ==========

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

    // ========== Textures ==========

    /// Get the color texture.
    pub fn color_texture(&self) -> Option<metal::Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(colorTexture));
            metal::Texture::from_raw(ptr)
        }
    }

    /// Set the color texture.
    pub fn set_color_texture(&self, texture: &metal::Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setColorTexture:), texture.as_ptr());
        }
    }

    /// Get the previous color texture.
    pub fn prev_color_texture(&self) -> Option<metal::Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(prevColorTexture));
            metal::Texture::from_raw(ptr)
        }
    }

    /// Set the previous color texture.
    pub fn set_prev_color_texture(&self, texture: &metal::Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setPrevColorTexture:), texture.as_ptr());
        }
    }

    /// Get the depth texture.
    pub fn depth_texture(&self) -> Option<metal::Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(depthTexture));
            metal::Texture::from_raw(ptr)
        }
    }

    /// Set the depth texture.
    pub fn set_depth_texture(&self, texture: &metal::Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDepthTexture:), texture.as_ptr());
        }
    }

    /// Get the motion texture.
    pub fn motion_texture(&self) -> Option<metal::Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(motionTexture));
            metal::Texture::from_raw(ptr)
        }
    }

    /// Set the motion texture.
    pub fn set_motion_texture(&self, texture: &metal::Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMotionTexture:), texture.as_ptr());
        }
    }

    /// Get the UI texture.
    pub fn ui_texture(&self) -> Option<metal::Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(uiTexture));
            metal::Texture::from_raw(ptr)
        }
    }

    /// Set the UI texture.
    pub fn set_ui_texture(&self, texture: &metal::Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setUITexture:), texture.as_ptr());
        }
    }

    /// Get the output texture.
    pub fn output_texture(&self) -> Option<metal::Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(outputTexture));
            metal::Texture::from_raw(ptr)
        }
    }

    /// Set the output texture.
    pub fn set_output_texture(&self, texture: &metal::Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOutputTexture:), texture.as_ptr());
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

    // ========== Timing ==========

    /// Get the delta time between frames.
    pub fn delta_time(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(deltaTime)) }
    }

    /// Set the delta time between frames.
    pub fn set_delta_time(&self, delta_time: f32) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDeltaTime:), delta_time);
        }
    }

    // ========== Camera Properties ==========

    /// Get the near plane distance.
    pub fn near_plane(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(nearPlane)) }
    }

    /// Set the near plane distance.
    pub fn set_near_plane(&self, near_plane: f32) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setNearPlane:), near_plane);
        }
    }

    /// Get the far plane distance.
    pub fn far_plane(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(farPlane)) }
    }

    /// Set the far plane distance.
    pub fn set_far_plane(&self, far_plane: f32) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setFarPlane:), far_plane);
        }
    }

    /// Get the field of view.
    pub fn field_of_view(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(fieldOfView)) }
    }

    /// Set the field of view.
    pub fn set_field_of_view(&self, fov: f32) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setFieldOfView:), fov);
        }
    }

    /// Get the aspect ratio.
    pub fn aspect_ratio(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(aspectRatio)) }
    }

    /// Set the aspect ratio.
    pub fn set_aspect_ratio(&self, aspect_ratio: f32) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setAspectRatio:), aspect_ratio);
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

    // ========== UI Properties ==========

    /// Check if UI texture is composited.
    pub fn is_ui_texture_composited(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isUITextureComposited)) }
    }

    /// Set whether UI texture is composited.
    pub fn set_is_ui_texture_composited(&self, composited: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setIsUITextureComposited:), composited);
        }
    }

    // ========== State ==========

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

    // ========== Fence ==========

    /// Get the fence.
    pub fn fence(&self) -> Option<metal::Fence> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(fence));
            metal::Fence::from_raw(ptr)
        }
    }

    /// Set the fence.
    pub fn set_fence(&self, fence: &metal::Fence) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setFence:), fence.as_ptr());
        }
    }

    // ========== Encoding ==========

    /// Encode the frame interpolation operation to a command buffer.
    pub fn encode_to_command_buffer(&self, command_buffer: &metal::CommandBuffer) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(encodeToCommandBuffer:),
                command_buffer.as_ptr(),
            );
        }
    }

    /// Encode the frame interpolation operation to an MTL4 command buffer.
    ///
    /// C++ equivalent: `void encodeToCommandBuffer(MTL4::CommandBuffer*)`
    ///
    /// This is the MTL4FX variant that works with Metal 4 command buffers.
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

impl Clone for FrameInterpolator {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for FrameInterpolator {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for FrameInterpolator {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for FrameInterpolator {}
unsafe impl Sync for FrameInterpolator {}

impl std::fmt::Debug for FrameInterpolator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FrameInterpolator")
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
    fn test_frame_interpolator_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<FrameInterpolatorDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_frame_interpolator_size() {
        assert_eq!(
            std::mem::size_of::<FrameInterpolator>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_frame_interpolator_descriptor_creation() {
        // May fail if MetalFX is not available on the system
        if let Some(desc) = FrameInterpolatorDescriptor::new() {
            assert_eq!(desc.input_width(), 0);
            assert_eq!(desc.input_height(), 0);
        }
    }

    #[test]
    fn test_frame_interpolator_descriptor_properties() {
        if let Some(desc) = FrameInterpolatorDescriptor::new() {
            desc.set_input_width(1920);
            desc.set_input_height(1080);
            desc.set_output_width(1920);
            desc.set_output_height(1080);

            assert_eq!(desc.input_width(), 1920);
            assert_eq!(desc.input_height(), 1080);
            assert_eq!(desc.output_width(), 1920);
            assert_eq!(desc.output_height(), 1080);
        }
    }
}
