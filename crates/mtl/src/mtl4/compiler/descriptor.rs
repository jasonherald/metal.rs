//! MTL4 Compiler descriptor.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::mtl4::PipelineDataSetSerializer;

/// Descriptor for creating a compiler.
///
/// C++ equivalent: `MTL4::CompilerDescriptor`
#[repr(transparent)]
pub struct CompilerDescriptor(NonNull<c_void>);

impl CompilerDescriptor {
    /// Create a CompilerDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new compiler descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTL4CompilerDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the label.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ns_string: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ns_string.is_null() {
                return None;
            }
            let c_str: *const i8 = msg_send_0(ns_string, sel!(UTF8String));
            if c_str.is_null() {
                return None;
            }
            Some(
                std::ffi::CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    }

    /// Set the label.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = mtl_foundation::String::from_str(label) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the pipeline data set serializer.
    ///
    /// C++ equivalent: `PipelineDataSetSerializer* pipelineDataSetSerializer() const`
    pub fn pipeline_data_set_serializer(&self) -> Option<PipelineDataSetSerializer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(pipelineDataSetSerializer));
            PipelineDataSetSerializer::from_raw(ptr)
        }
    }

    /// Set the pipeline data set serializer.
    ///
    /// C++ equivalent: `void setPipelineDataSetSerializer(const MTL4::PipelineDataSetSerializer*)`
    pub fn set_pipeline_data_set_serializer(&self, serializer: &PipelineDataSetSerializer) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setPipelineDataSetSerializer:),
                serializer.as_ptr(),
            );
        }
    }
}

impl Clone for CompilerDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CompilerDescriptor {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for CompilerDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CompilerDescriptor {}
unsafe impl Sync for CompilerDescriptor {}

impl std::fmt::Debug for CompilerDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompilerDescriptor")
            .field("label", &self.label())
            .finish()
    }
}
