//! CAMetalLayer implementation.
//!
//! Corresponds to `QuartzCore/CAMetalLayer.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::metal_drawable::MetalDrawable;
use crate::types::{CGColorSpaceRef, CGSize};

/// A Core Animation layer that Metal can render into.
///
/// C++ equivalent: `CA::MetalLayer`
///
/// A MetalLayer provides a drawable surface backed by textures that Metal can
/// render to. The layer's content is then composited by Core Animation for display.
#[repr(transparent)]
pub struct MetalLayer(NonNull<c_void>);

impl MetalLayer {
    /// Create a MetalLayer from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid CAMetalLayer object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the layer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new MetalLayer.
    ///
    /// C++ equivalent: `static MetalLayer* layer()`
    pub fn layer() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("CAMetalLayer")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(layer));
            Self::from_raw(ptr)
        }
    }

    /// Get the Metal device used by this layer.
    ///
    /// C++ equivalent: `MTL::Device* device() const`
    pub fn device(&self) -> Option<mtl_gpu::Device> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            mtl_gpu::Device::from_raw(ptr)
        }
    }

    /// Set the Metal device for this layer.
    ///
    /// C++ equivalent: `void setDevice(MTL::Device* device)`
    pub fn set_device(&self, device: &mtl_gpu::Device) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDevice:), device.as_ptr());
        }
    }

    /// Get the pixel format of the layer's textures.
    ///
    /// C++ equivalent: `MTL::PixelFormat pixelFormat() const`
    pub fn pixel_format(&self) -> mtl_gpu::PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(pixelFormat)) }
    }

    /// Set the pixel format of the layer's textures.
    ///
    /// C++ equivalent: `void setPixelFormat(MTL::PixelFormat pixelFormat)`
    pub fn set_pixel_format(&self, pixel_format: mtl_gpu::PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setPixelFormat:), pixel_format);
        }
    }

    /// Check whether the layer's textures are for rendering only.
    ///
    /// C++ equivalent: `bool framebufferOnly() const`
    ///
    /// When true (the default), the textures can only be used as render targets.
    /// When false, the textures can also be sampled or used with compute.
    pub fn framebuffer_only(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(framebufferOnly)) }
    }

    /// Set whether the layer's textures should be for rendering only.
    ///
    /// C++ equivalent: `void setFramebufferOnly(bool framebufferOnly)`
    pub fn set_framebuffer_only(&self, framebuffer_only: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setFramebufferOnly:), framebuffer_only);
        }
    }

    /// Get the size of the layer's drawable textures.
    ///
    /// C++ equivalent: `CGSize drawableSize() const`
    pub fn drawable_size(&self) -> CGSize {
        unsafe { msg_send_0(self.as_ptr(), sel!(drawableSize)) }
    }

    /// Set the size of the layer's drawable textures.
    ///
    /// C++ equivalent: `void setDrawableSize(CGSize drawableSize)`
    pub fn set_drawable_size(&self, drawable_size: CGSize) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDrawableSize:), drawable_size);
        }
    }

    /// Get the next drawable texture.
    ///
    /// C++ equivalent: `MetalDrawable* nextDrawable()`
    ///
    /// Returns None if no drawable is available (e.g., if all drawables are in use).
    /// This is a blocking call that may wait for a drawable to become available.
    pub fn next_drawable(&self) -> Option<MetalDrawable> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(nextDrawable));
            MetalDrawable::from_raw(ptr)
        }
    }

    /// Get the maximum number of drawables.
    ///
    /// C++ equivalent: `NS::UInteger maximumDrawableCount() const`
    ///
    /// The default value is 3.
    pub fn maximum_drawable_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maximumDrawableCount)) }
    }

    /// Set the maximum number of drawables.
    ///
    /// C++ equivalent: `void setMaximumDrawableCount(NS::UInteger maximumDrawableCount)`
    ///
    /// Valid values are 2 or 3.
    pub fn set_maximum_drawable_count(&self, count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMaximumDrawableCount:), count);
        }
    }

    /// Check whether presentation is synchronized with the display.
    ///
    /// C++ equivalent: `bool displaySyncEnabled() const`
    ///
    /// When true (the default on macOS), the layer waits for the display's vsync
    /// before presenting new content.
    #[cfg(target_os = "macos")]
    pub fn display_sync_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(displaySyncEnabled)) }
    }

    /// Set whether presentation is synchronized with the display.
    ///
    /// C++ equivalent: `void setDisplaySyncEnabled(bool displaySyncEnabled)`
    #[cfg(target_os = "macos")]
    pub fn set_display_sync_enabled(&self, enabled: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDisplaySyncEnabled:), enabled);
        }
    }

    /// Get the color space of the layer.
    ///
    /// C++ equivalent: `CGColorSpaceRef colorspace() const`
    pub fn colorspace(&self) -> CGColorSpaceRef {
        unsafe { msg_send_0(self.as_ptr(), sel!(colorspace)) }
    }

    /// Set the color space of the layer.
    ///
    /// C++ equivalent: `void setColorspace(CGColorSpaceRef colorspace)`
    pub fn set_colorspace(&self, colorspace: CGColorSpaceRef) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setColorspace:), colorspace);
        }
    }

    /// Check whether nextDrawable is allowed to time out.
    ///
    /// C++ equivalent: `bool allowsNextDrawableTimeout() const`
    ///
    /// When true (the default), nextDrawable may return nil if no drawable
    /// becomes available within one second. When false, nextDrawable blocks
    /// indefinitely until a drawable is available.
    pub fn allows_next_drawable_timeout(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(allowsNextDrawableTimeout)) }
    }

    /// Set whether nextDrawable is allowed to time out.
    ///
    /// C++ equivalent: `void setAllowsNextDrawableTimeout(bool allowsNextDrawableTimeout)`
    pub fn set_allows_next_drawable_timeout(&self, allows: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setAllowsNextDrawableTimeout:), allows);
        }
    }

    /// Get the residency set used by this layer.
    ///
    /// C++ equivalent: `MTL::ResidencySet* residencySet() const`
    pub fn residency_set(&self) -> Option<mtl_gpu::ResidencySet> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(residencySet));
            mtl_gpu::ResidencySet::from_raw(ptr)
        }
    }
}

impl Clone for MetalLayer {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for MetalLayer {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for MetalLayer {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

// SAFETY: MetalLayer is a reference-counted object that is thread-safe
unsafe impl Send for MetalLayer {}
unsafe impl Sync for MetalLayer {}

impl std::fmt::Debug for MetalLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetalLayer")
            .field("pixel_format", &self.pixel_format())
            .field("drawable_size", &self.drawable_size())
            .field("maximum_drawable_count", &self.maximum_drawable_count())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metal_layer_size() {
        // MetalLayer should be pointer-sized
        assert_eq!(
            std::mem::size_of::<MetalLayer>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_metal_layer_creation() {
        // Create a MetalLayer
        let layer = MetalLayer::layer();
        // Layer creation should succeed on macOS with QuartzCore available
        if let Some(layer) = layer {
            // Check default values
            assert!(layer.framebuffer_only());
            assert!(layer.maximum_drawable_count() >= 2);
        }
    }

    #[test]
    fn test_metal_layer_properties() {
        let layer = match MetalLayer::layer() {
            Some(l) => l,
            None => return, // Skip if layer creation not available
        };

        // Test drawable size
        let size = CGSize::new(800.0, 600.0);
        layer.set_drawable_size(size);
        let retrieved_size = layer.drawable_size();
        assert_eq!(retrieved_size.width, size.width);
        assert_eq!(retrieved_size.height, size.height);

        // Test framebuffer only
        layer.set_framebuffer_only(false);
        assert!(!layer.framebuffer_only());
        layer.set_framebuffer_only(true);
        assert!(layer.framebuffer_only());

        // Test maximum drawable count
        layer.set_maximum_drawable_count(2);
        assert_eq!(layer.maximum_drawable_count(), 2);
        layer.set_maximum_drawable_count(3);
        assert_eq!(layer.maximum_drawable_count(), 3);
    }
}
