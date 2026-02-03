//! Information about a binding.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, sel};

use crate::enums::{BindingAccess, BindingType};

/// Information about a binding.
///
/// C++ equivalent: `MTL::Binding`
#[repr(transparent)]
pub struct Binding(pub(crate) NonNull<c_void>);

impl Binding {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal Binding.
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

    /// Get the index.
    ///
    /// C++ equivalent: `NS::UInteger index() const`
    #[inline]
    pub fn index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(index)) }
    }

    /// Check if this is an argument.
    ///
    /// C++ equivalent: `bool isArgument() const`
    #[inline]
    pub fn is_argument(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isArgument)) }
    }

    /// Check if this binding is used.
    ///
    /// C++ equivalent: `bool isUsed() const`
    #[inline]
    pub fn is_used(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isUsed)) }
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

    /// Get the binding type.
    ///
    /// C++ equivalent: `BindingType type() const`
    #[inline]
    pub fn binding_type(&self) -> BindingType {
        unsafe { msg_send_0(self.as_ptr(), sel!(type)) }
    }

    /// Check if this is an argument (deprecated).
    ///
    /// C++ equivalent: `bool argument() const`
    #[deprecated(note = "Use is_argument instead")]
    #[inline]
    pub fn argument(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(argument)) }
    }

    /// Check if this binding is used (deprecated).
    ///
    /// C++ equivalent: `bool used() const`
    #[deprecated(note = "Use is_used instead")]
    #[inline]
    pub fn used(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(used)) }
    }
}

impl Clone for Binding {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for Binding {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for Binding {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for Binding {}
unsafe impl Sync for Binding {}
