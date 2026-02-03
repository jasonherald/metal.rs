//! Descriptor for creating specialized functions.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::FunctionOptions;

use super::FunctionConstantValues;

/// Descriptor for creating specialized functions.
///
/// C++ equivalent: `MTL::FunctionDescriptor`
///
/// Used to create specialized functions from a library.
#[repr(transparent)]
pub struct FunctionDescriptor(pub(crate) NonNull<c_void>);

impl FunctionDescriptor {
    /// Allocate a new function descriptor.
    ///
    /// C++ equivalent: `static FunctionDescriptor* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let cls = metal_sys::Class::get("MTLFunctionDescriptor")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize an allocated function descriptor.
    ///
    /// C++ equivalent: `FunctionDescriptor* init()`
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new function descriptor.
    pub fn new() -> Option<Self> {
        Self::alloc()?.init()
    }

    /// Create a function descriptor using the factory method.
    ///
    /// C++ equivalent: `static FunctionDescriptor* functionDescriptor()`
    pub fn function_descriptor() -> Option<Self> {
        unsafe {
            let cls = metal_sys::Class::get("MTLFunctionDescriptor")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(functionDescriptor));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal function descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the function name.
    ///
    /// C++ equivalent: `NS::String* name() const`
    pub fn name(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(name));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                metal_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Set the function name.
    ///
    /// C++ equivalent: `void setName(const NS::String*)`
    pub fn set_name(&self, name: &str) {
        if let Some(ns_name) = metal_foundation::String::from_str(name) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setName:), ns_name.as_ptr());
            }
        }
    }

    /// Get the specialized name.
    ///
    /// C++ equivalent: `NS::String* specializedName() const`
    pub fn specialized_name(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(specializedName));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                metal_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Set the specialized name.
    ///
    /// C++ equivalent: `void setSpecializedName(const NS::String*)`
    pub fn set_specialized_name(&self, name: &str) {
        if let Some(ns_name) = metal_foundation::String::from_str(name) {
            unsafe {
                msg_send_1::<(), *const c_void>(
                    self.as_ptr(),
                    sel!(setSpecializedName:),
                    ns_name.as_ptr(),
                );
            }
        }
    }

    /// Get the function constant values.
    ///
    /// C++ equivalent: `FunctionConstantValues* constantValues() const`
    pub fn constant_values(&self) -> Option<FunctionConstantValues> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(constantValues));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            FunctionConstantValues::from_raw(ptr)
        }
    }

    /// Set the function constant values.
    ///
    /// C++ equivalent: `void setConstantValues(const FunctionConstantValues*)`
    pub fn set_constant_values(&self, values: Option<&FunctionConstantValues>) {
        let ptr = values.map(|v| v.as_ptr()).unwrap_or(std::ptr::null());
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setConstantValues:), ptr);
        }
    }

    /// Get the function options.
    ///
    /// C++ equivalent: `FunctionOptions options() const`
    #[inline]
    pub fn options(&self) -> FunctionOptions {
        unsafe { msg_send_0(self.as_ptr(), sel!(options)) }
    }

    /// Set the function options.
    ///
    /// C++ equivalent: `void setOptions(FunctionOptions)`
    #[inline]
    pub fn set_options(&self, options: FunctionOptions) {
        unsafe {
            msg_send_1::<(), FunctionOptions>(self.as_ptr(), sel!(setOptions:), options);
        }
    }

    /// Get the binary archives (raw NSArray pointer).
    ///
    /// C++ equivalent: `NS::Array* binaryArchives() const`
    pub fn binary_archives_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(binaryArchives)) }
    }

    /// Set the binary archives.
    ///
    /// C++ equivalent: `void setBinaryArchives(const NS::Array*)`
    ///
    /// # Safety
    ///
    /// The archives pointer must be a valid NSArray of BinaryArchive objects or null.
    pub unsafe fn set_binary_archives_raw(&self, archives: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setBinaryArchives:), archives);
        }
    }
}

impl Default for FunctionDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create FunctionDescriptor")
    }
}

impl Clone for FunctionDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy FunctionDescriptor")
        }
    }
}

impl Drop for FunctionDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for FunctionDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for FunctionDescriptor {}
unsafe impl Sync for FunctionDescriptor {}

impl std::fmt::Debug for FunctionDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionDescriptor")
            .field("name", &self.name())
            .field("specialized_name", &self.specialized_name())
            .field("options", &self.options())
            .finish()
    }
}
