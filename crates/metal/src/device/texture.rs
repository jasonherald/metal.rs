//! Device texture creation methods.
//!
//! Corresponds to texture creation methods in `Metal/MTLDevice.hpp`.

use std::ffi::c_void;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_1, msg_send_3, sel};

use super::Device;
use crate::enums::TextureType;
use crate::error::ValidationError;
use crate::texture::{Texture, TextureDescriptor};

impl Device {
    // =========================================================================
    // Texture Creation
    // =========================================================================

    /// Create a new texture with the specified descriptor.
    ///
    /// C++ equivalent: `Texture* newTexture(const TextureDescriptor*)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn new_texture(&self, descriptor: *const c_void) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(newTextureWithDescriptor:), descriptor);
            Texture::from_raw(ptr)
        }
    }

    /// Create a texture with validation.
    ///
    /// This safe method validates the descriptor before calling Metal APIs:
    /// - Ensures width and height are > 0
    /// - Ensures depth is > 0 for 3D textures
    /// - Ensures array length is > 0 for array textures
    /// - Validates mipmap count is within allowed range
    /// - Validates sample count is supported by the device
    ///
    /// Use this method instead of `new_texture` to avoid process aborts
    /// from Metal's validation layer.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let desc = TextureDescriptor::texture_2d_descriptor(
    ///     PixelFormat::BGRA8_UNORM, 256, 256, false
    /// ).unwrap();
    ///
    /// match device.new_texture_with_descriptor(&desc) {
    ///     Ok(texture) => { /* use texture */ }
    ///     Err(ValidationError::InvalidTextureDimensions { .. }) => { /* handle error */ }
    ///     Err(e) => { /* handle other errors */ }
    /// }
    /// ```
    pub fn new_texture_with_descriptor(
        &self,
        descriptor: &TextureDescriptor,
    ) -> Result<Texture, ValidationError> {
        let width = descriptor.width();
        let height = descriptor.height();
        let depth = descriptor.depth();

        // Validate dimensions are non-zero
        if width == 0 || height == 0 {
            return Err(ValidationError::InvalidTextureDimensions {
                width,
                height,
                depth,
            });
        }

        // Validate depth for 3D textures
        let texture_type = descriptor.texture_type();
        if texture_type == TextureType::TYPE_3D && depth == 0 {
            return Err(ValidationError::InvalidTextureDimensions {
                width,
                height,
                depth,
            });
        }

        // Validate array length for array textures
        let array_length = descriptor.array_length();
        let is_array_type = matches!(
            texture_type,
            TextureType::TYPE_1D_ARRAY
                | TextureType::TYPE_2D_ARRAY
                | TextureType::TYPE_CUBE_ARRAY
                | TextureType::TYPE_2D_MULTISAMPLE_ARRAY
        );
        if is_array_type && array_length == 0 {
            return Err(ValidationError::InvalidArrayLength);
        }

        // Validate mipmap count
        let max_dim = width.max(height).max(depth);
        let max_mipmaps = 1 + ((max_dim as f64).log2().floor() as UInteger);
        let requested_mipmaps = descriptor.mipmap_level_count();
        if requested_mipmaps > max_mipmaps {
            return Err(ValidationError::InvalidMipmapCount {
                requested: requested_mipmaps,
                max_allowed: max_mipmaps,
            });
        }

        // Validate sample count
        let sample_count = descriptor.sample_count();
        if sample_count > 1 && !self.supports_texture_sample_count(sample_count) {
            return Err(ValidationError::UnsupportedTextureSampleCount(sample_count));
        }

        // Call unsafe implementation
        unsafe {
            self.new_texture(descriptor.as_ptr())
                .ok_or(ValidationError::CreationFailed(None))
        }
    }

    /// Create a new texture backed by an IOSurface.
    ///
    /// C++ equivalent: `Texture* newTexture(const TextureDescriptor*, IOSurfaceRef, NS::UInteger plane)`
    ///
    /// # Safety
    ///
    /// The descriptor and IOSurface pointers must be valid.
    pub unsafe fn new_texture_with_iosurface(
        &self,
        descriptor: *const c_void,
        iosurface: *const c_void,
        plane: UInteger,
    ) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newTextureWithDescriptor: iosurface: plane:),
                descriptor,
                iosurface,
                plane,
            );
            Texture::from_raw(ptr)
        }
    }

    /// Create a shared texture with another device.
    ///
    /// C++ equivalent: `Texture* newSharedTexture(const SharedTextureHandle*)`
    ///
    /// # Safety
    ///
    /// The handle pointer must be valid.
    pub unsafe fn new_shared_texture_with_handle(&self, handle: *const c_void) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(newSharedTextureWithHandle:), handle);
            Texture::from_raw(ptr)
        }
    }

    /// Create a shared texture with the specified descriptor.
    ///
    /// C++ equivalent: `Texture* newSharedTexture(const TextureDescriptor*)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn new_shared_texture_with_descriptor(
        &self,
        descriptor: *const c_void,
    ) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newSharedTextureWithDescriptor:),
                descriptor,
            );
            Texture::from_raw(ptr)
        }
    }
}

#[cfg(test)]
mod tests {
    // Texture creation tests require TextureDescriptor which will be implemented
    // in Phase 4 (Resources). Basic texture tests are placeholder for now.
}
