//! Metal texture resources.
//!
//! Corresponds to `Metal/MTLTexture.hpp`.
//!
//! Textures store formatted image data for shader access.

mod descriptor;
mod shared_handle;
mod texture;
mod view_descriptor;

pub use descriptor::TextureDescriptor;
pub use shared_handle::SharedTextureHandle;
pub use texture::Texture;
pub use view_descriptor::TextureViewDescriptor;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::PixelFormat;
    use std::ffi::c_void;

    #[test]
    fn test_texture_size() {
        assert_eq!(
            std::mem::size_of::<Texture>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_texture_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<TextureDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_texture_descriptor_creation() {
        let desc = TextureDescriptor::new();
        assert!(desc.is_some());
    }

    #[test]
    fn test_texture_2d_descriptor() {
        let desc =
            TextureDescriptor::texture_2d_descriptor(PixelFormat::RGBA8_UNORM, 256, 256, true);
        assert!(desc.is_some());

        let desc = desc.unwrap();
        assert_eq!(desc.width(), 256);
        assert_eq!(desc.height(), 256);
        assert_eq!(desc.pixel_format(), PixelFormat::RGBA8_UNORM);
    }

    #[test]
    fn test_texture_descriptor_setters() {
        let desc = TextureDescriptor::new().unwrap();

        desc.set_width(512);
        desc.set_height(512);
        desc.set_pixel_format(PixelFormat::BGRA8_UNORM);
        desc.set_texture_type(crate::enums::TextureType::TYPE_2D);

        assert_eq!(desc.width(), 512);
        assert_eq!(desc.height(), 512);
        assert_eq!(desc.pixel_format(), PixelFormat::BGRA8_UNORM);
        assert_eq!(desc.texture_type(), crate::enums::TextureType::TYPE_2D);
    }

    #[test]
    fn test_shared_texture_handle_size() {
        assert_eq!(
            std::mem::size_of::<SharedTextureHandle>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
