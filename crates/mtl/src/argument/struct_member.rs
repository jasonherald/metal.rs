//! A member of a struct type.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, sel};

use crate::enums::DataType;

use super::{ArrayType, PointerType, StructType, TensorReferenceType, TextureReferenceType};

/// A member of a struct type.
///
/// C++ equivalent: `MTL::StructMember`
#[repr(transparent)]
pub struct StructMember(pub(crate) NonNull<c_void>);

impl StructMember {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal StructMember.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the argument index.
    ///
    /// C++ equivalent: `NS::UInteger argumentIndex() const`
    #[inline]
    pub fn argument_index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(argumentIndex)) }
    }

    /// Get the data type.
    ///
    /// C++ equivalent: `DataType dataType() const`
    #[inline]
    pub fn data_type(&self) -> DataType {
        unsafe { msg_send_0(self.as_ptr(), sel!(dataType)) }
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
                mtl_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Get the offset.
    ///
    /// C++ equivalent: `NS::UInteger offset() const`
    #[inline]
    pub fn offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(offset)) }
    }

    /// Get the array type (if this member is an array).
    ///
    /// C++ equivalent: `ArrayType* arrayType()`
    pub fn array_type(&self) -> Option<ArrayType> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(arrayType));
            ArrayType::from_raw(ptr)
        }
    }

    /// Get the pointer type (if this member is a pointer).
    ///
    /// C++ equivalent: `PointerType* pointerType()`
    pub fn pointer_type(&self) -> Option<PointerType> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(pointerType));
            PointerType::from_raw(ptr)
        }
    }

    /// Get the struct type (if this member is a struct).
    ///
    /// C++ equivalent: `StructType* structType()`
    pub fn struct_type(&self) -> Option<StructType> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(structType));
            StructType::from_raw(ptr)
        }
    }

    /// Get the texture reference type (if this member is a texture).
    ///
    /// C++ equivalent: `TextureReferenceType* textureReferenceType()`
    pub fn texture_reference_type(&self) -> Option<TextureReferenceType> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(textureReferenceType));
            TextureReferenceType::from_raw(ptr)
        }
    }

    /// Get the tensor reference type (if this member is a tensor).
    ///
    /// C++ equivalent: `TensorReferenceType* tensorReferenceType()`
    pub fn tensor_reference_type(&self) -> Option<TensorReferenceType> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(tensorReferenceType));
            TensorReferenceType::from_raw(ptr)
        }
    }
}

impl Clone for StructMember {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for StructMember {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for StructMember {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for StructMember {}
unsafe impl Sync for StructMember {}
