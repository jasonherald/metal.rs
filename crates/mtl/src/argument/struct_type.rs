//! A struct type for reflection.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::DataType;

use super::StructMember;

/// A struct type for reflection.
///
/// C++ equivalent: `MTL::StructType`
#[repr(transparent)]
pub struct StructType(pub(crate) NonNull<c_void>);

impl StructType {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal StructType.
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

    /// Get a member by name.
    ///
    /// C++ equivalent: `StructMember* memberByName(const NS::String*)`
    pub fn member_by_name(&self, name: &str) -> Option<StructMember> {
        let ns_name = mtl_foundation::String::from_str(name)?;
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(memberByName:), ns_name.as_ptr());
            StructMember::from_raw(ptr)
        }
    }

    /// Get the members as a raw NS::Array pointer.
    ///
    /// C++ equivalent: `NS::Array* members() const`
    #[inline]
    pub fn members_ptr(&self) -> *const c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(members)) }
    }
}

impl Clone for StructType {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for StructType {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for StructType {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for StructType {}
unsafe impl Sync for StructType {}
