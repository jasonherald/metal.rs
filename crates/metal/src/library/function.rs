//! A compiled shader function.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, sel};

use crate::enums::{FunctionOptions, FunctionType, PatchType};

/// A compiled shader function.
///
/// C++ equivalent: `MTL::Function`
#[repr(transparent)]
pub struct Function(pub(crate) NonNull<c_void>);

impl Function {
    /// Create a Function from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal function object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the function.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the label for this function.
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

    /// Set the label for this function.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the device that created this function.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("function has no device")
        }
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

    /// Get the function type.
    ///
    /// C++ equivalent: `FunctionType functionType() const`
    #[inline]
    pub fn function_type(&self) -> FunctionType {
        unsafe { msg_send_0(self.as_ptr(), sel!(functionType)) }
    }

    /// Get the patch type (for tessellation shaders).
    ///
    /// C++ equivalent: `PatchType patchType() const`
    #[inline]
    pub fn patch_type(&self) -> PatchType {
        unsafe { msg_send_0(self.as_ptr(), sel!(patchType)) }
    }

    /// Get the patch control point count (for tessellation shaders).
    ///
    /// C++ equivalent: `NS::Integer patchControlPointCount() const`
    #[inline]
    pub fn patch_control_point_count(&self) -> metal_foundation::Integer {
        unsafe { msg_send_0(self.as_ptr(), sel!(patchControlPointCount)) }
    }

    /// Get the function options.
    ///
    /// C++ equivalent: `FunctionOptions options() const`
    #[inline]
    pub fn options(&self) -> FunctionOptions {
        unsafe { msg_send_0(self.as_ptr(), sel!(options)) }
    }

    // =========================================================================
    // Reflection
    // =========================================================================

    /// Get the function constants dictionary (raw NSDictionary pointer).
    ///
    /// C++ equivalent: `NS::Dictionary* functionConstantsDictionary() const`
    pub fn function_constants_dictionary_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(functionConstantsDictionary)) }
    }

    /// Get the stage input attributes (raw NSArray pointer).
    ///
    /// C++ equivalent: `NS::Array* stageInputAttributes() const`
    ///
    /// Returns an array of Attribute objects.
    pub fn stage_input_attributes_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(stageInputAttributes)) }
    }

    /// Get the vertex attributes (raw NSArray pointer).
    ///
    /// C++ equivalent: `NS::Array* vertexAttributes() const`
    ///
    /// Returns an array of VertexAttribute objects (for vertex functions).
    pub fn vertex_attributes_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(vertexAttributes)) }
    }

    // =========================================================================
    // Argument Encoder Creation
    // =========================================================================

    /// Create a new argument encoder for the buffer at the specified index.
    ///
    /// C++ equivalent: `ArgumentEncoder* newArgumentEncoder(NS::UInteger bufferIndex)`
    pub fn new_argument_encoder(&self, buffer_index: UInteger) -> Option<crate::ArgumentEncoder> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newArgumentEncoderWithBufferIndex:),
                buffer_index,
            );
            crate::ArgumentEncoder::from_raw(ptr)
        }
    }

    /// Create a new argument encoder with reflection for the buffer at the specified index.
    ///
    /// C++ equivalent: `ArgumentEncoder* newArgumentEncoder(NS::UInteger bufferIndex, const AutoreleasedArgument* reflection)`
    ///
    /// # Safety
    ///
    /// The reflection pointer must be a valid pointer to an Argument pointer, or null.
    pub unsafe fn new_argument_encoder_with_reflection(
        &self,
        buffer_index: UInteger,
        reflection: *mut *mut c_void,
    ) -> Option<crate::ArgumentEncoder> {
        unsafe {
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newArgumentEncoderWithBufferIndex: reflection:),
                buffer_index,
                reflection,
            );
            crate::ArgumentEncoder::from_raw(ptr)
        }
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for Function {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for Function {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for Function {}
unsafe impl Sync for Function {}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name())
            .field("function_type", &self.function_type())
            .field("label", &self.label())
            .finish()
    }
}
