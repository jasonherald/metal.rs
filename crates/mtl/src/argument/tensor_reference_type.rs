//! A tensor reference type for reflection.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_0, sel};

use crate::enums::{BindingAccess, DataType, TensorDataType};

/// A tensor reference type for reflection.
///
/// C++ equivalent: `MTL::TensorReferenceType`
#[repr(transparent)]
pub struct TensorReferenceType(pub(crate) NonNull<c_void>);

impl TensorReferenceType {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal TensorReferenceType.
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

    /// Get the index type.
    ///
    /// C++ equivalent: `DataType indexType() const`
    #[inline]
    pub fn index_type(&self) -> DataType {
        unsafe { msg_send_0(self.as_ptr(), sel!(indexType)) }
    }

    /// Get the tensor data type.
    ///
    /// C++ equivalent: `TensorDataType tensorDataType() const`
    #[inline]
    pub fn tensor_data_type(&self) -> TensorDataType {
        unsafe { msg_send_0(self.as_ptr(), sel!(tensorDataType)) }
    }

    /// Get the dimensions as a raw pointer.
    ///
    /// C++ equivalent: `TensorExtents* dimensions() const`
    #[inline]
    pub fn dimensions_ptr(&self) -> *const c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(dimensions)) }
    }
}

impl Clone for TensorReferenceType {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for TensorReferenceType {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for TensorReferenceType {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for TensorReferenceType {}
unsafe impl Sync for TensorReferenceType {}
