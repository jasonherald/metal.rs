//! A collection of compiled shader functions.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, sel};

use crate::enums::LibraryType;

use super::{
    Function, FunctionConstantValues, FunctionDescriptor, FunctionReflection,
    IntersectionFunctionDescriptor,
};

/// A collection of compiled shader functions.
///
/// C++ equivalent: `MTL::Library`
#[repr(transparent)]
pub struct Library(pub(crate) NonNull<c_void>);

impl Library {
    /// Create a Library from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal library object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the library.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the label for this library.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
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

    /// Set the label for this library.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the device that created this library.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            if ptr.is_null() {
                panic!("library has no device");
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("library has no device")
        }
    }

    /// Get the library type.
    ///
    /// C++ equivalent: `LibraryType type() const`
    #[inline]
    pub fn library_type(&self) -> LibraryType {
        unsafe { msg_send_0(self.as_ptr(), sel!(type)) }
    }

    /// Get the install name (for dynamic libraries).
    ///
    /// C++ equivalent: `NS::String* installName() const`
    pub fn install_name(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(installName));
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

    // =========================================================================
    // Function Retrieval
    // =========================================================================

    /// Get a function by name.
    ///
    /// C++ equivalent: `Function* newFunction(const NS::String* name)`
    pub fn new_function_with_name(&self, name: &str) -> Option<Function> {
        let ns_name = metal_foundation::String::from_str(name)?;
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(newFunctionWithName:), ns_name.as_ptr());
            Function::from_raw(ptr)
        }
    }

    /// Get a function by name with constant values.
    ///
    /// C++ equivalent: `Function* newFunction(const NS::String*, const FunctionConstantValues*, NS::Error**)`
    ///
    /// # Safety
    ///
    /// The constant_values pointer must be valid if not null.
    pub unsafe fn new_function_with_name_and_constants(
        &self,
        name: &str,
        constant_values: *const c_void,
    ) -> Result<Function, metal_foundation::Error> {
        let ns_name = metal_foundation::String::from_str(name).ok_or_else(|| {
            metal_foundation::Error::error(std::ptr::null_mut(), 0, std::ptr::null_mut())
                .expect("failed to create error for invalid string")
        })?;

        let mut error: *mut c_void = std::ptr::null_mut();
        unsafe {
            let ptr: *mut c_void = metal_sys::msg_send_3(
                self.as_ptr(),
                sel!(newFunctionWithName: constantValues: error:),
                ns_name.as_ptr(),
                constant_values,
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    let _: *mut c_void = msg_send_0(error, sel!(retain));
                    return Err(metal_foundation::Error::from_ptr(error).unwrap());
                }
                return Err(metal_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error object"));
            }

            Ok(Function::from_raw(ptr).unwrap())
        }
    }

    /// Get all function names in the library.
    ///
    /// C++ equivalent: `NS::Array* functionNames() const`
    pub fn function_names(&self) -> Vec<String> {
        unsafe {
            let array: *mut c_void = msg_send_0(self.as_ptr(), sel!(functionNames));
            if array.is_null() {
                return Vec::new();
            }

            let count: UInteger = msg_send_0(array, sel!(count));
            let mut names = Vec::with_capacity(count as usize);

            for i in 0..count {
                let obj: *mut c_void = msg_send_1(array, sel!(objectAtIndex:), i);
                if !obj.is_null() {
                    let utf8_ptr: *const std::ffi::c_char =
                        metal_sys::msg_send_0(obj as *const c_void, sel!(UTF8String));
                    if !utf8_ptr.is_null() {
                        let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
                        names.push(c_str.to_string_lossy().into_owned());
                    }
                }
            }

            names
        }
    }

    /// Create a function with a descriptor.
    ///
    /// C++ equivalent: `Function* newFunction(const FunctionDescriptor*, NS::Error**)`
    pub fn new_function_with_descriptor(
        &self,
        descriptor: &FunctionDescriptor,
    ) -> Result<Function, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newFunctionWithDescriptor: error:),
                descriptor.as_ptr(),
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    return Err(
                        metal_foundation::Error::from_ptr(error).expect("error should be valid")
                    );
                }
                return Err(metal_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error object"));
            }

            Ok(Function::from_raw(ptr).expect("failed to create function"))
        }
    }

    /// Create an intersection function with a descriptor.
    ///
    /// C++ equivalent: `Function* newIntersectionFunction(const IntersectionFunctionDescriptor*, NS::Error**)`
    pub fn new_intersection_function(
        &self,
        descriptor: &IntersectionFunctionDescriptor,
    ) -> Result<Function, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newIntersectionFunctionWithDescriptor: error:),
                descriptor.as_ptr(),
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    return Err(
                        metal_foundation::Error::from_ptr(error).expect("error should be valid")
                    );
                }
                return Err(metal_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error object"));
            }

            Ok(Function::from_raw(ptr).expect("failed to create intersection function"))
        }
    }

    /// Get reflection information for a function by name.
    ///
    /// C++ equivalent: `FunctionReflection* reflectionForFunction(const NS::String*)`
    pub fn reflection_for_function(&self, name: &str) -> Option<FunctionReflection> {
        let ns_name = metal_foundation::String::from_str(name)?;
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(reflectionForFunctionWithName:),
                ns_name.as_ptr(),
            );
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            FunctionReflection::from_raw(ptr)
        }
    }

    // =========================================================================
    // Async Function Creation
    // =========================================================================

    /// Create a function with constant values asynchronously.
    ///
    /// C++ equivalent: `void newFunction(const NS::String*, const FunctionConstantValues*, void (^)(Function*, Error*))`
    pub fn new_function_with_name_and_constants_async<F>(
        &self,
        name: &str,
        constant_values: &FunctionConstantValues,
        completion_handler: F,
    ) where
        F: Fn(Option<Function>, Option<metal_foundation::Error>) + Send + 'static,
    {
        let Some(ns_name) = metal_foundation::String::from_str(name) else {
            completion_handler(
                None,
                metal_foundation::Error::error(std::ptr::null_mut(), -1, std::ptr::null_mut()),
            );
            return;
        };

        let block =
            metal_sys::TwoArgBlock::from_fn(move |fn_ptr: *mut c_void, err_ptr: *mut c_void| {
                let function = if fn_ptr.is_null() {
                    None
                } else {
                    unsafe { Function::from_raw(fn_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { metal_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(function, error);
            });

        unsafe {
            metal_sys::msg_send_3::<(), *const c_void, *const c_void, *const c_void>(
                self.as_ptr(),
                sel!(newFunctionWithName:constantValues:completionHandler:),
                ns_name.as_ptr(),
                constant_values.as_ptr(),
                block.as_ptr(),
            );
        }

        std::mem::forget(block);
    }

    /// Create a function with a descriptor asynchronously.
    ///
    /// C++ equivalent: `void newFunction(const FunctionDescriptor*, void (^)(Function*, Error*))`
    pub fn new_function_with_descriptor_async<F>(
        &self,
        descriptor: &FunctionDescriptor,
        completion_handler: F,
    ) where
        F: Fn(Option<Function>, Option<metal_foundation::Error>) + Send + 'static,
    {
        let block =
            metal_sys::TwoArgBlock::from_fn(move |fn_ptr: *mut c_void, err_ptr: *mut c_void| {
                let function = if fn_ptr.is_null() {
                    None
                } else {
                    unsafe { Function::from_raw(fn_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { metal_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(function, error);
            });

        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, *const c_void>(
                self.as_ptr(),
                sel!(newFunctionWithDescriptor:completionHandler:),
                descriptor.as_ptr(),
                block.as_ptr(),
            );
        }

        std::mem::forget(block);
    }

    /// Create an intersection function with a descriptor asynchronously.
    ///
    /// C++ equivalent: `void newIntersectionFunction(const IntersectionFunctionDescriptor*, void (^)(Function*, Error*))`
    pub fn new_intersection_function_async<F>(
        &self,
        descriptor: &IntersectionFunctionDescriptor,
        completion_handler: F,
    ) where
        F: Fn(Option<Function>, Option<metal_foundation::Error>) + Send + 'static,
    {
        let block =
            metal_sys::TwoArgBlock::from_fn(move |fn_ptr: *mut c_void, err_ptr: *mut c_void| {
                let function = if fn_ptr.is_null() {
                    None
                } else {
                    unsafe { Function::from_raw(fn_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { metal_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(function, error);
            });

        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, *const c_void>(
                self.as_ptr(),
                sel!(newIntersectionFunctionWithDescriptor:completionHandler:),
                descriptor.as_ptr(),
                block.as_ptr(),
            );
        }

        std::mem::forget(block);
    }
}

impl Clone for Library {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for Library {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for Library {}
unsafe impl Sync for Library {}

impl std::fmt::Debug for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Library")
            .field("label", &self.label())
            .field("library_type", &self.library_type())
            .finish()
    }
}
