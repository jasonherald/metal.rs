//! Base type for Metal type reflection.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, sel};

use crate::enums::DataType;

/// Base type for Metal type reflection.
///
/// C++ equivalent: `MTL::Type`
#[repr(transparent)]
pub struct Type(pub(crate) NonNull<c_void>);

impl Type {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal Type.
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
}

impl Clone for Type {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for Type {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for Type {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for Type {}
unsafe impl Sync for Type {}
