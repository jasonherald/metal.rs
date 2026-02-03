//! Argument descriptor.
//!
//! Corresponds to `MTL::ArgumentDescriptor` in `Metal/MTLDevice.hpp`.
//!
//! Used to describe arguments for creating argument encoders.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{BindingAccess, DataType, TextureType};

/// Describes an argument for creating an argument encoder.
///
/// C++ equivalent: `MTL::ArgumentDescriptor`
#[repr(transparent)]
pub struct ArgumentDescriptor(pub(crate) NonNull<c_void>);

impl ArgumentDescriptor {
    /// Create a new argument descriptor.
    ///
    /// C++ equivalent: `ArgumentDescriptor::argumentDescriptor()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::class!(MTLArgumentDescriptor);
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(argumentDescriptor));
            Self::from_raw(ptr)
        }
    }

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

    /// Get the data type.
    ///
    /// C++ equivalent: `DataType dataType() const`
    #[inline]
    pub fn data_type(&self) -> DataType {
        unsafe { msg_send_0(self.as_ptr(), sel!(dataType)) }
    }

    /// Set the data type.
    ///
    /// C++ equivalent: `void setDataType(DataType)`
    #[inline]
    pub fn set_data_type(&self, data_type: DataType) {
        unsafe {
            msg_send_1::<(), DataType>(self.as_ptr(), sel!(setDataType:), data_type);
        }
    }

    /// Get the index.
    ///
    /// C++ equivalent: `NS::UInteger index() const`
    #[inline]
    pub fn index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(index)) }
    }

    /// Set the index.
    ///
    /// C++ equivalent: `void setIndex(NS::UInteger)`
    #[inline]
    pub fn set_index(&self, index: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setIndex:), index);
        }
    }

    /// Get the access mode.
    ///
    /// C++ equivalent: `BindingAccess access() const`
    #[inline]
    pub fn access(&self) -> BindingAccess {
        unsafe { msg_send_0(self.as_ptr(), sel!(access)) }
    }

    /// Set the access mode.
    ///
    /// C++ equivalent: `void setAccess(BindingAccess)`
    #[inline]
    pub fn set_access(&self, access: BindingAccess) {
        unsafe {
            msg_send_1::<(), BindingAccess>(self.as_ptr(), sel!(setAccess:), access);
        }
    }

    /// Get the array length.
    ///
    /// C++ equivalent: `NS::UInteger arrayLength() const`
    #[inline]
    pub fn array_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(arrayLength)) }
    }

    /// Set the array length.
    ///
    /// C++ equivalent: `void setArrayLength(NS::UInteger)`
    #[inline]
    pub fn set_array_length(&self, array_length: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setArrayLength:), array_length);
        }
    }

    /// Get the texture type.
    ///
    /// C++ equivalent: `TextureType textureType() const`
    #[inline]
    pub fn texture_type(&self) -> TextureType {
        unsafe { msg_send_0(self.as_ptr(), sel!(textureType)) }
    }

    /// Set the texture type.
    ///
    /// C++ equivalent: `void setTextureType(TextureType)`
    #[inline]
    pub fn set_texture_type(&self, texture_type: TextureType) {
        unsafe {
            msg_send_1::<(), TextureType>(self.as_ptr(), sel!(setTextureType:), texture_type);
        }
    }

    /// Get the constant block alignment.
    ///
    /// C++ equivalent: `NS::UInteger constantBlockAlignment() const`
    #[inline]
    pub fn constant_block_alignment(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(constantBlockAlignment)) }
    }

    /// Set the constant block alignment.
    ///
    /// C++ equivalent: `void setConstantBlockAlignment(NS::UInteger)`
    #[inline]
    pub fn set_constant_block_alignment(&self, alignment: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setConstantBlockAlignment:), alignment);
        }
    }
}

impl Default for ArgumentDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create ArgumentDescriptor")
    }
}

impl Clone for ArgumentDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ArgumentDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for ArgumentDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ArgumentDescriptor {}
unsafe impl Sync for ArgumentDescriptor {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_size() {
        assert_eq!(
            std::mem::size_of::<ArgumentDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
