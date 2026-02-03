//! Buffer binding information.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, sel};

use crate::enums::{BindingAccess, BindingType, DataType};

use super::{PointerType, StructType};

/// Buffer binding information.
///
/// C++ equivalent: `MTL::BufferBinding`
#[repr(transparent)]
pub struct BufferBinding(pub(crate) NonNull<c_void>);

impl BufferBinding {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal BufferBinding.
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

    // BufferBinding-specific

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
}

impl Clone for BufferBinding {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for BufferBinding {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for BufferBinding {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for BufferBinding {}
unsafe impl Sync for BufferBinding {}
