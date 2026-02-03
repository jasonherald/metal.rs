//! Constant values for specializing a function.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, sel};

/// Constant values for specializing a function.
///
/// C++ equivalent: `MTL::FunctionConstantValues`
///
/// Function constant values allow you to specialize a function at runtime
/// by providing values for function constants defined in shader code.
#[repr(transparent)]
pub struct FunctionConstantValues(pub(crate) NonNull<c_void>);

impl FunctionConstantValues {
    /// Allocate a new function constant values object.
    ///
    /// C++ equivalent: `static FunctionConstantValues* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let cls = mtl_sys::Class::get("MTLFunctionConstantValues")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize an allocated function constant values object.
    ///
    /// C++ equivalent: `FunctionConstantValues* init()`
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new function constant values object.
    pub fn new() -> Option<Self> {
        Self::alloc()?.init()
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal function constant values object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Reset all constant values.
    ///
    /// C++ equivalent: `void reset()`
    #[inline]
    pub fn reset(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(reset));
        }
    }

    /// Set a constant value by index.
    ///
    /// C++ equivalent: `void setConstantValue(const void*, DataType, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The value pointer must be valid and point to data of the correct type.
    pub unsafe fn set_constant_value_at_index(
        &self,
        value: *const c_void,
        data_type: crate::DataType,
        index: UInteger,
    ) {
        unsafe {
            mtl_sys::msg_send_3::<(), *const c_void, crate::DataType, UInteger>(
                self.as_ptr(),
                sel!(setConstantValue: type: atIndex:),
                value,
                data_type,
                index,
            );
        }
    }

    /// Set a constant value by name.
    ///
    /// C++ equivalent: `void setConstantValue(const void*, DataType, const NS::String*)`
    ///
    /// # Safety
    ///
    /// The value pointer must be valid and point to data of the correct type.
    pub unsafe fn set_constant_value_with_name(
        &self,
        value: *const c_void,
        data_type: crate::DataType,
        name: &str,
    ) {
        if let Some(ns_name) = mtl_foundation::String::from_str(name) {
            unsafe {
                mtl_sys::msg_send_3::<(), *const c_void, crate::DataType, *const c_void>(
                    self.as_ptr(),
                    sel!(setConstantValue: type: withName:),
                    value,
                    data_type,
                    ns_name.as_ptr(),
                );
            }
        }
    }

    /// Set multiple constant values in a range.
    ///
    /// C++ equivalent: `void setConstantValues(const void*, DataType, NS::Range)`
    ///
    /// # Safety
    ///
    /// The values pointer must be valid and point to an array of data of the correct type.
    pub unsafe fn set_constant_values(
        &self,
        values: *const c_void,
        data_type: crate::DataType,
        location: UInteger,
        length: UInteger,
    ) {
        let range = mtl_foundation::Range::new(location, length);
        unsafe {
            mtl_sys::msg_send_3::<(), *const c_void, crate::DataType, mtl_foundation::Range>(
                self.as_ptr(),
                sel!(setConstantValues: type: withRange:),
                values,
                data_type,
                range,
            );
        }
    }

    /// Set a boolean constant value by index.
    pub fn set_bool_at_index(&self, value: bool, index: UInteger) {
        unsafe {
            self.set_constant_value_at_index(
                &value as *const bool as *const c_void,
                crate::DataType::BOOL,
                index,
            );
        }
    }

    /// Set an integer constant value by index.
    pub fn set_int_at_index(&self, value: i32, index: UInteger) {
        unsafe {
            self.set_constant_value_at_index(
                &value as *const i32 as *const c_void,
                crate::DataType::INT,
                index,
            );
        }
    }

    /// Set an unsigned integer constant value by index.
    pub fn set_uint_at_index(&self, value: u32, index: UInteger) {
        unsafe {
            self.set_constant_value_at_index(
                &value as *const u32 as *const c_void,
                crate::DataType::UINT,
                index,
            );
        }
    }

    /// Set a float constant value by index.
    pub fn set_float_at_index(&self, value: f32, index: UInteger) {
        unsafe {
            self.set_constant_value_at_index(
                &value as *const f32 as *const c_void,
                crate::DataType::FLOAT,
                index,
            );
        }
    }
}

impl Default for FunctionConstantValues {
    fn default() -> Self {
        Self::new().expect("failed to create FunctionConstantValues")
    }
}

impl Clone for FunctionConstantValues {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy FunctionConstantValues")
        }
    }
}

impl Drop for FunctionConstantValues {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for FunctionConstantValues {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for FunctionConstantValues {}
unsafe impl Sync for FunctionConstantValues {}

impl std::fmt::Debug for FunctionConstantValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionConstantValues").finish()
    }
}
