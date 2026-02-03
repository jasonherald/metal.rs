//! Function constant definition.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, sel};

use crate::enums::DataType;

/// Function constant definition.
///
/// C++ equivalent: `MTL::FunctionConstant`
///
/// Contains information about a function constant defined in shader code.
#[repr(transparent)]
pub struct FunctionConstant(pub(crate) NonNull<c_void>);

impl FunctionConstant {
    /// Allocate a new function constant.
    ///
    /// C++ equivalent: `static FunctionConstant* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let cls = mtl_sys::Class::get("MTLFunctionConstant")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize an allocated function constant.
    ///
    /// C++ equivalent: `FunctionConstant* init()`
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new function constant.
    pub fn new() -> Option<Self> {
        Self::alloc()?.init()
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal function constant object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the function constant name.
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

    /// Get the function constant data type.
    ///
    /// C++ equivalent: `DataType type() const`
    #[inline]
    pub fn constant_type(&self) -> DataType {
        unsafe { msg_send_0(self.as_ptr(), sel!(type)) }
    }

    /// Get the function constant index.
    ///
    /// C++ equivalent: `NS::UInteger index() const`
    #[inline]
    pub fn index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(index)) }
    }

    /// Check if the function constant is required.
    ///
    /// C++ equivalent: `bool required() const`
    #[inline]
    pub fn required(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(required)) }
    }
}

impl Clone for FunctionConstant {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for FunctionConstant {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for FunctionConstant {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for FunctionConstant {}
unsafe impl Sync for FunctionConstant {}

impl std::fmt::Debug for FunctionConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionConstant")
            .field("name", &self.name())
            .field("type", &self.constant_type())
            .field("index", &self.index())
            .field("required", &self.required())
            .finish()
    }
}
