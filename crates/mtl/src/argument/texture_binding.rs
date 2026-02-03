//! Texture binding information.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, sel};

use crate::enums::{BindingAccess, BindingType, DataType, TextureType};

/// Texture binding information.
///
/// C++ equivalent: `MTL::TextureBinding`
#[repr(transparent)]
pub struct TextureBinding(pub(crate) NonNull<c_void>);

impl TextureBinding {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal TextureBinding.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // Inherited from Binding

    /// Get the access mode.
    #[inline]
    pub fn access(&self) -> BindingAccess {
        unsafe { msg_send_0(self.as_ptr(), sel!(access)) }
    }

    /// Get the index.
    #[inline]
    pub fn index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(index)) }
    }

    /// Check if this is an argument.
    #[inline]
    pub fn is_argument(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isArgument)) }
    }

    /// Check if this binding is used.
    #[inline]
    pub fn is_used(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isUsed)) }
    }

    /// Get the name.
    pub fn name(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(name));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                mtl_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Get the binding type.
    #[inline]
    pub fn binding_type(&self) -> BindingType {
        unsafe { msg_send_0(self.as_ptr(), sel!(type)) }
    }

    // TextureBinding-specific

    /// Get the array length.
    ///
    /// C++ equivalent: `NS::UInteger arrayLength() const`
    #[inline]
    pub fn array_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(arrayLength)) }
    }

    /// Check if this is a depth texture.
    ///
    /// C++ equivalent: `bool isDepthTexture() const`
    #[inline]
    pub fn is_depth_texture(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isDepthTexture)) }
    }

    /// Get the texture data type.
    ///
    /// C++ equivalent: `DataType textureDataType() const`
    #[inline]
    pub fn texture_data_type(&self) -> DataType {
        unsafe { msg_send_0(self.as_ptr(), sel!(textureDataType)) }
    }

    /// Get the texture type.
    ///
    /// C++ equivalent: `TextureType textureType() const`
    #[inline]
    pub fn texture_type(&self) -> TextureType {
        unsafe { msg_send_0(self.as_ptr(), sel!(textureType)) }
    }

    /// Check if this is a depth texture (deprecated).
    ///
    /// C++ equivalent: `bool depthTexture() const`
    #[deprecated(note = "Use is_depth_texture instead")]
    #[inline]
    pub fn depth_texture(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(depthTexture)) }
    }
}

impl Clone for TextureBinding {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for TextureBinding {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for TextureBinding {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for TextureBinding {}
unsafe impl Sync for TextureBinding {}
