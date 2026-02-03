//! Information about a function argument.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, sel};

use crate::enums::{ArgumentType, BindingAccess, DataType, TextureType};

use super::{PointerType, StructType};

/// Information about a function argument.
///
/// C++ equivalent: `MTL::Argument`
#[repr(transparent)]
pub struct Argument(pub(crate) NonNull<c_void>);

impl Argument {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal Argument.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the access mode.
    ///
    /// C++ equivalent: `BindingAccess access() const`
    #[inline]
    pub fn access(&self) -> BindingAccess {
        unsafe { msg_send_0(self.as_ptr(), sel!(access)) }
    }

    /// Get the array length.
    ///
    /// C++ equivalent: `NS::UInteger arrayLength() const`
    #[inline]
    pub fn array_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(arrayLength)) }
    }

    /// Get the buffer alignment.
    ///
    /// C++ equivalent: `NS::UInteger bufferAlignment() const`
    #[inline]
    pub fn buffer_alignment(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(bufferAlignment)) }
    }

    /// Get the buffer data size.
    ///
    /// C++ equivalent: `NS::UInteger bufferDataSize() const`
    #[inline]
    pub fn buffer_data_size(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(bufferDataSize)) }
    }

    /// Get the buffer data type.
    ///
    /// C++ equivalent: `DataType bufferDataType() const`
    #[inline]
    pub fn buffer_data_type(&self) -> DataType {
        unsafe { msg_send_0(self.as_ptr(), sel!(bufferDataType)) }
    }

    /// Get the buffer pointer type.
    ///
    /// C++ equivalent: `PointerType* bufferPointerType() const`
    pub fn buffer_pointer_type(&self) -> Option<PointerType> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(bufferPointerType));
            PointerType::from_raw(ptr)
        }
    }

    /// Get the buffer struct type.
    ///
    /// C++ equivalent: `StructType* bufferStructType() const`
    pub fn buffer_struct_type(&self) -> Option<StructType> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(bufferStructType));
            StructType::from_raw(ptr)
        }
    }

    /// Get the index.
    ///
    /// C++ equivalent: `NS::UInteger index() const`
    #[inline]
    pub fn index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(index)) }
    }

    /// Check if the argument is active.
    ///
    /// C++ equivalent: `bool isActive() const`
    #[inline]
    pub fn is_active(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isActive)) }
    }

    /// Check if this is a depth texture.
    ///
    /// C++ equivalent: `bool isDepthTexture() const`
    #[inline]
    pub fn is_depth_texture(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isDepthTexture)) }
    }

    /// Get the name.
    ///
    /// C++ equivalent: `NS::String* name() const`
    pub fn name(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(name));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                metal_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
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

    /// Get the threadgroup memory alignment.
    ///
    /// C++ equivalent: `NS::UInteger threadgroupMemoryAlignment() const`
    #[inline]
    pub fn threadgroup_memory_alignment(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(threadgroupMemoryAlignment)) }
    }

    /// Get the threadgroup memory data size.
    ///
    /// C++ equivalent: `NS::UInteger threadgroupMemoryDataSize() const`
    #[inline]
    pub fn threadgroup_memory_data_size(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(threadgroupMemoryDataSize)) }
    }

    /// Get the argument type.
    ///
    /// C++ equivalent: `ArgumentType type() const`
    #[inline]
    pub fn argument_type(&self) -> ArgumentType {
        unsafe { msg_send_0(self.as_ptr(), sel!(type)) }
    }

    /// Check if the argument is active (deprecated).
    ///
    /// C++ equivalent: `bool active() const`
    #[deprecated(note = "Use is_active instead")]
    #[inline]
    pub fn active(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(active)) }
    }
}

impl Clone for Argument {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for Argument {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for Argument {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for Argument {}
unsafe impl Sync for Argument {}
