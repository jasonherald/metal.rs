//! A texture reference type for reflection.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_0, sel};

use crate::enums::{BindingAccess, DataType, TextureType};

/// A texture reference type for reflection.
///
/// C++ equivalent: `MTL::TextureReferenceType`
#[repr(transparent)]
pub struct TextureReferenceType(pub(crate) NonNull<c_void>);

impl TextureReferenceType {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal TextureReferenceType.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the data type.
    ///
    /// C++ equivalent: `DataType dataType() const`
    #[inline]
    pub fn data_type(&self) -> DataType {
        unsafe { msg_send_0(self.as_ptr(), sel!(dataType)) }
    }

    /// Get the access mode.
    ///
    /// C++ equivalent: `BindingAccess access() const`
    #[inline]
    pub fn access(&self) -> BindingAccess {
        unsafe { msg_send_0(self.as_ptr(), sel!(access)) }
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
}

impl Clone for TextureReferenceType {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for TextureReferenceType {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for TextureReferenceType {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for TextureReferenceType {}
unsafe impl Sync for TextureReferenceType {}
