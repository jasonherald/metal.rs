//! An array type for reflection.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, sel};

use crate::enums::DataType;

use super::{PointerType, StructType, TensorReferenceType, TextureReferenceType};

/// An array type for reflection.
///
/// C++ equivalent: `MTL::ArrayType`
#[repr(transparent)]
pub struct ArrayType(pub(crate) NonNull<c_void>);

impl ArrayType {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal ArrayType.
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

    /// Get the argument index stride.
    ///
    /// C++ equivalent: `NS::UInteger argumentIndexStride() const`
    #[inline]
    pub fn argument_index_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(argumentIndexStride)) }
    }

    /// Get the array length.
    ///
    /// C++ equivalent: `NS::UInteger arrayLength() const`
    #[inline]
    pub fn array_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(arrayLength)) }
    }

    /// Get the element type.
    ///
    /// C++ equivalent: `DataType elementType() const`
    #[inline]
    pub fn element_type(&self) -> DataType {
        unsafe { msg_send_0(self.as_ptr(), sel!(elementType)) }
    }

    /// Get the stride.
    ///
    /// C++ equivalent: `NS::UInteger stride() const`
    #[inline]
    pub fn stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(stride)) }
    }

    /// Get the element array type (if element is an array).
    ///
    /// C++ equivalent: `ArrayType* elementArrayType()`
    pub fn element_array_type(&self) -> Option<ArrayType> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(elementArrayType));
            ArrayType::from_raw(ptr)
        }
    }

    /// Get the element pointer type (if element is a pointer).
    ///
    /// C++ equivalent: `PointerType* elementPointerType()`
    pub fn element_pointer_type(&self) -> Option<PointerType> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(elementPointerType));
            PointerType::from_raw(ptr)
        }
    }

    /// Get the element struct type (if element is a struct).
    ///
    /// C++ equivalent: `StructType* elementStructType()`
    pub fn element_struct_type(&self) -> Option<StructType> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(elementStructType));
            StructType::from_raw(ptr)
        }
    }

    /// Get the element texture reference type (if element is a texture).
    ///
    /// C++ equivalent: `TextureReferenceType* elementTextureReferenceType()`
    pub fn element_texture_reference_type(&self) -> Option<TextureReferenceType> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(elementTextureReferenceType));
            TextureReferenceType::from_raw(ptr)
        }
    }

    /// Get the element tensor reference type (if element is a tensor).
    ///
    /// C++ equivalent: `TensorReferenceType* elementTensorReferenceType()`
    pub fn element_tensor_reference_type(&self) -> Option<TensorReferenceType> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(elementTensorReferenceType));
            TensorReferenceType::from_raw(ptr)
        }
    }
}

impl Clone for ArrayType {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ArrayType {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for ArrayType {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ArrayType {}
unsafe impl Sync for ArrayType {}
