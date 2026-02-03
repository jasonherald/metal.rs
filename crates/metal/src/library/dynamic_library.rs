//! A dynamic Metal library that can be loaded at runtime.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, msg_send_1, sel};

/// A dynamic Metal library that can be loaded at runtime.
///
/// C++ equivalent: `MTL::DynamicLibrary`
///
/// Dynamic libraries allow you to load Metal shader code at runtime
/// and link functions dynamically.
#[repr(transparent)]
pub struct DynamicLibrary(pub(crate) NonNull<c_void>);

impl DynamicLibrary {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal dynamic library.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the device that created this dynamic library.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            if ptr.is_null() {
                panic!("dynamic library has no device");
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("dynamic library has no device")
        }
    }

    /// Get the label for this dynamic library.
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

    /// Set the label for this dynamic library.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the install name for this dynamic library.
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

    /// Serialize the dynamic library to a URL.
    ///
    /// C++ equivalent: `bool serializeToURL(const NS::URL*, NS::Error**)`
    pub fn serialize_to_url(
        &self,
        url: &metal_foundation::Url,
    ) -> Result<(), metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let success: bool = metal_sys::msg_send_2(
                self.as_ptr(),
                sel!(serializeToURL: error:),
                url.as_ptr(),
                &mut error as *mut _,
            );
            if success {
                Ok(())
            } else if !error.is_null() {
                let _: *mut c_void = msg_send_0(error, sel!(retain));
                Err(metal_foundation::Error::from_ptr(error)
                    .expect("error pointer should be valid"))
            } else {
                Err(
                    metal_foundation::Error::error(std::ptr::null_mut(), -1, std::ptr::null_mut())
                        .expect("failed to create error object"),
                )
            }
        }
    }
}

impl Clone for DynamicLibrary {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for DynamicLibrary {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for DynamicLibrary {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for DynamicLibrary {}
unsafe impl Sync for DynamicLibrary {}

impl std::fmt::Debug for DynamicLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicLibrary")
            .field("label", &self.label())
            .field("install_name", &self.install_name())
            .finish()
    }
}
