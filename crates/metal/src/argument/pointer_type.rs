//! A pointer type for reflection.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, sel};

use crate::enums::{BindingAccess, DataType};

use super::{ArrayType, StructType};

/// A pointer type for reflection.
///
/// C++ equivalent: `MTL::PointerType`
#[repr(transparent)]
pub struct PointerType(pub(crate) NonNull<c_void>);

impl PointerType {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal PointerType.
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

    /// Get the alignment.
    ///
    /// C++ equivalent: `NS::UInteger alignment() const`
    #[inline]
    pub fn alignment(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(alignment)) }
    }

    /// Get the data size.
    ///
    /// C++ equivalent: `NS::UInteger dataSize() const`
    #[inline]
    pub fn data_size(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(dataSize)) }
    }

    /// Get the element type.
    ///
    /// C++ equivalent: `DataType elementType() const`
    #[inline]
    pub fn element_type(&self) -> DataType {
        unsafe { msg_send_0(self.as_ptr(), sel!(elementType)) }
    }

    /// Check if the element is an argument buffer.
    ///
    /// C++ equivalent: `bool elementIsArgumentBuffer() const`
    #[inline]
    pub fn element_is_argument_buffer(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(elementIsArgumentBuffer)) }
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

    /// Get the element struct type (if element is a struct).
    ///
    /// C++ equivalent: `StructType* elementStructType()`
    pub fn element_struct_type(&self) -> Option<StructType> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(elementStructType));
            StructType::from_raw(ptr)
        }
    }
}

impl Clone for PointerType {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for PointerType {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for PointerType {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for PointerType {}
unsafe impl Sync for PointerType {}
