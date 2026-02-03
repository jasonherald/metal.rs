//! Configuration for creating texture views.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{PixelFormat, TextureSwizzleChannels, TextureType};

/// Configuration for creating texture views.
///
/// C++ equivalent: `MTL::TextureViewDescriptor`
#[repr(transparent)]
pub struct TextureViewDescriptor(pub(crate) NonNull<c_void>);

impl TextureViewDescriptor {
    /// Create a new texture view descriptor.
    ///
    /// C++ equivalent: `static TextureViewDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLTextureViewDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a TextureViewDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal texture view descriptor object.
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

    /// Get the pixel format.
    #[inline]
    pub fn pixel_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(pixelFormat)) }
    }

    /// Get the texture type.
    #[inline]
    pub fn texture_type(&self) -> TextureType {
        unsafe { msg_send_0(self.as_ptr(), sel!(textureType)) }
    }

    /// Get the level range.
    #[inline]
    pub fn level_range(&self) -> metal_foundation::Range {
        unsafe { msg_send_0(self.as_ptr(), sel!(levelRange)) }
    }

    /// Get the slice range.
    #[inline]
    pub fn slice_range(&self) -> metal_foundation::Range {
        unsafe { msg_send_0(self.as_ptr(), sel!(sliceRange)) }
    }

    /// Get the swizzle channels.
    #[inline]
    pub fn swizzle(&self) -> TextureSwizzleChannels {
        unsafe { msg_send_0(self.as_ptr(), sel!(swizzle)) }
    }

    // =========================================================================
    // Properties - Setters
    // =========================================================================

    /// Set the pixel format.
    #[inline]
    pub fn set_pixel_format(&self, pixel_format: PixelFormat) {
        unsafe {
            msg_send_1::<(), PixelFormat>(self.as_ptr(), sel!(setPixelFormat:), pixel_format);
        }
    }

    /// Set the texture type.
    #[inline]
    pub fn set_texture_type(&self, texture_type: TextureType) {
        unsafe {
            msg_send_1::<(), TextureType>(self.as_ptr(), sel!(setTextureType:), texture_type);
        }
    }

    /// Set the level range.
    #[inline]
    pub fn set_level_range(&self, range: metal_foundation::Range) {
        unsafe {
            msg_send_1::<(), metal_foundation::Range>(self.as_ptr(), sel!(setLevelRange:), range);
        }
    }

    /// Set the slice range.
    #[inline]
    pub fn set_slice_range(&self, range: metal_foundation::Range) {
        unsafe {
            msg_send_1::<(), metal_foundation::Range>(self.as_ptr(), sel!(setSliceRange:), range);
        }
    }

    /// Set the swizzle channels.
    #[inline]
    pub fn set_swizzle(&self, swizzle: TextureSwizzleChannels) {
        unsafe {
            msg_send_1::<(), TextureSwizzleChannels>(self.as_ptr(), sel!(setSwizzle:), swizzle);
        }
    }
}

impl Default for TextureViewDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create texture view descriptor")
    }
}

impl Clone for TextureViewDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy texture view descriptor")
        }
    }
}

impl Drop for TextureViewDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for TextureViewDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for TextureViewDescriptor {}
unsafe impl Sync for TextureViewDescriptor {}

impl std::fmt::Debug for TextureViewDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextureViewDescriptor")
            .field("pixel_format", &self.pixel_format())
            .field("texture_type", &self.texture_type())
            .finish()
    }
}
