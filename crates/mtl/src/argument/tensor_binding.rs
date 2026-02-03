//! Tensor binding information.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, sel};

use crate::enums::{BindingAccess, BindingType, DataType, TensorDataType};

/// Tensor binding information.
///
/// C++ equivalent: `MTL::TensorBinding`
#[repr(transparent)]
pub struct TensorBinding(pub(crate) NonNull<c_void>);

impl TensorBinding {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal TensorBinding.
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

    // TensorBinding-specific

    /// Get the dimensions as a raw pointer.
    ///
    /// C++ equivalent: `TensorExtents* dimensions() const`
    #[inline]
    pub fn dimensions_ptr(&self) -> *const c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(dimensions)) }
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
}

impl Clone for TensorBinding {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for TensorBinding {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for TensorBinding {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for TensorBinding {}
unsafe impl Sync for TensorBinding {}
