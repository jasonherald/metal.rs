//! SpatialScaler implementation.
//!
//! Corresponds to `MetalFX/MTLFXSpatialScaler.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::SpatialScalerColorProcessingMode;

// ============================================================
// SpatialScalerDescriptor
// ============================================================

/// Descriptor for creating a spatial scaler.
///
/// C++ equivalent: `MTLFX::SpatialScalerDescriptor`
#[repr(transparent)]
pub struct SpatialScalerDescriptor(NonNull<c_void>);

impl SpatialScalerDescriptor {
    /// Create a SpatialScalerDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid MTLFXSpatialScalerDescriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new spatial scaler descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLFXSpatialScalerDescriptor")?;
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
    pub fn color_texture_format(&self) -> mtl::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(colorTextureFormat)) }
    }

    /// Set the color texture format.
    pub fn set_color_texture_format(&self, format: mtl::PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setColorTextureFormat:), format);
        }
    }

    /// Get the output texture format.
    pub fn output_texture_format(&self) -> mtl::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(outputTextureFormat)) }
    }

    /// Set the output texture format.
    pub fn set_output_texture_format(&self, format: mtl::PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOutputTextureFormat:), format);
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

    // ========== Color Processing ==========

    /// Get the color processing mode.
    pub fn color_processing_mode(&self) -> SpatialScalerColorProcessingMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(colorProcessingMode)) }
    }

    /// Set the color processing mode.
    pub fn set_color_processing_mode(&self, mode: SpatialScalerColorProcessingMode) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setColorProcessingMode:), mode);
        }
    }

    // ========== Factory Methods ==========

    /// Create a new spatial scaler.
    ///
    /// C++ equivalent: `SpatialScaler* newSpatialScaler(const MTL::Device*) const`
    pub fn new_spatial_scaler(&self, device: &mtl::Device) -> Option<SpatialScaler> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newSpatialScalerWithDevice:),
                device.as_ptr(),
            );
            SpatialScaler::from_raw(ptr)
        }
    }

    // ========== Static Methods ==========

    /// Check if a device supports spatial scaling.
    ///
    /// C++ equivalent: `static bool supportsDevice(const MTL::Device*)`
    pub fn supports_device(device: &mtl::Device) -> bool {
        unsafe {
            let class = match mtl_sys::Class::get("MTLFXSpatialScalerDescriptor") {
                Some(c) => c,
                None => return false,
            };
            msg_send_1(class.as_ptr(), sel!(supportsDevice:), device.as_ptr())
        }
    }
}

impl Clone for SpatialScalerDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for SpatialScalerDescriptor {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for SpatialScalerDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for SpatialScalerDescriptor {}
unsafe impl Sync for SpatialScalerDescriptor {}

// ============================================================
// SpatialScaler
// ============================================================

/// AI-based spatial upscaler.
///
/// C++ equivalent: `MTLFX::SpatialScaler`
///
/// SpatialScaler uses machine learning to upscale images with high quality.
/// It operates on a single frame and doesn't require motion vectors.
#[repr(transparent)]
pub struct SpatialScaler(NonNull<c_void>);

impl SpatialScaler {
    /// Create a SpatialScaler from a raw pointer.
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
    pub fn color_texture_usage(&self) -> mtl::TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(colorTextureUsage)) }
    }

    /// Get the required texture usage for the output texture.
    pub fn output_texture_usage(&self) -> mtl::TextureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(outputTextureUsage)) }
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
    pub fn color_texture(&self) -> Option<mtl::Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(colorTexture));
            mtl::Texture::from_raw(ptr)
        }
    }

    /// Set the color texture.
    pub fn set_color_texture(&self, texture: &mtl::Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setColorTexture:), texture.as_ptr());
        }
    }

    /// Get the output texture.
    pub fn output_texture(&self) -> Option<mtl::Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(outputTexture));
            mtl::Texture::from_raw(ptr)
        }
    }

    /// Set the output texture.
    pub fn set_output_texture(&self, texture: &mtl::Texture) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOutputTexture:), texture.as_ptr());
        }
    }

    // ========== Read-Only Properties ==========

    /// Get the color texture format.
    pub fn color_texture_format(&self) -> mtl::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(colorTextureFormat)) }
    }

    /// Get the output texture format.
    pub fn output_texture_format(&self) -> mtl::PixelFormat {
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

    /// Get the color processing mode.
    pub fn color_processing_mode(&self) -> SpatialScalerColorProcessingMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(colorProcessingMode)) }
    }

    // ========== Fence ==========

    /// Get the fence.
    pub fn fence(&self) -> Option<mtl::Fence> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(fence));
            mtl::Fence::from_raw(ptr)
        }
    }

    /// Set the fence.
    pub fn set_fence(&self, fence: &mtl::Fence) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setFence:), fence.as_ptr());
        }
    }

    // ========== Encoding ==========

    /// Encode the scaling operation to a command buffer.
    ///
    /// C++ equivalent: `void encodeToCommandBuffer(MTL::CommandBuffer*)`
    pub fn encode_to_command_buffer(&self, command_buffer: &mtl::CommandBuffer) {
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
    pub fn encode_to_mtl4_command_buffer(&self, command_buffer: &mtl::mtl4::CommandBuffer) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(encodeToCommandBuffer:),
                command_buffer.as_ptr(),
            );
        }
    }
}

impl Clone for SpatialScaler {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for SpatialScaler {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for SpatialScaler {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for SpatialScaler {}
unsafe impl Sync for SpatialScaler {}

impl std::fmt::Debug for SpatialScaler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpatialScaler")
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
    fn test_spatial_scaler_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<SpatialScalerDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_spatial_scaler_size() {
        assert_eq!(
            std::mem::size_of::<SpatialScaler>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_spatial_scaler_descriptor_creation() {
        // May fail if MetalFX is not available on the system
        if let Some(desc) = SpatialScalerDescriptor::new() {
            // Test default values
            assert_eq!(desc.input_width(), 0);
            assert_eq!(desc.input_height(), 0);
        }
    }

    #[test]
    fn test_spatial_scaler_descriptor_properties() {
        if let Some(desc) = SpatialScalerDescriptor::new() {
            desc.set_input_width(1920);
            desc.set_input_height(1080);
            desc.set_output_width(3840);
            desc.set_output_height(2160);

            assert_eq!(desc.input_width(), 1920);
            assert_eq!(desc.input_height(), 1080);
            assert_eq!(desc.output_width(), 3840);
            assert_eq!(desc.output_height(), 2160);

            desc.set_color_processing_mode(SpatialScalerColorProcessingMode::HDR);
            assert_eq!(
                desc.color_processing_mode(),
                SpatialScalerColorProcessingMode::HDR
            );
        }
    }
}
