//! Vertex attribute reflection information.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, sel};

use crate::enums::DataType;

/// Vertex attribute reflection information.
///
/// C++ equivalent: `MTL::VertexAttribute`
///
/// Contains information about a vertex attribute in a vertex function.
#[repr(transparent)]
pub struct VertexAttribute(pub(crate) NonNull<c_void>);

impl VertexAttribute {
    /// Allocate a new vertex attribute.
    ///
    /// C++ equivalent: `static VertexAttribute* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let cls = metal_sys::Class::get("MTLVertexAttribute")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize an allocated vertex attribute.
    ///
    /// C++ equivalent: `VertexAttribute* init()`
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new vertex attribute.
    pub fn new() -> Option<Self> {
        Self::alloc()?.init()
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal vertex attribute object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the attribute name.
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

    /// Get the attribute index.
    ///
    /// C++ equivalent: `NS::UInteger attributeIndex() const`
    #[inline]
    pub fn attribute_index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(attributeIndex)) }
    }

    /// Get the attribute data type.
    ///
    /// C++ equivalent: `DataType attributeType() const`
    #[inline]
    pub fn attribute_type(&self) -> DataType {
        unsafe { msg_send_0(self.as_ptr(), sel!(attributeType)) }
    }

    /// Check if the attribute is active.
    ///
    /// C++ equivalent: `bool isActive() const`
    #[inline]
    pub fn is_active(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isActive)) }
    }

    /// Check if this is patch data.
    ///
    /// C++ equivalent: `bool isPatchData() const`
    #[inline]
    pub fn is_patch_data(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isPatchData)) }
    }

    /// Check if this is patch control point data.
    ///
    /// C++ equivalent: `bool isPatchControlPointData() const`
    #[inline]
    pub fn is_patch_control_point_data(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isPatchControlPointData)) }
    }
}

impl Clone for VertexAttribute {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for VertexAttribute {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for VertexAttribute {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for VertexAttribute {}
unsafe impl Sync for VertexAttribute {}

impl std::fmt::Debug for VertexAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VertexAttribute")
            .field("name", &self.name())
            .field("attribute_index", &self.attribute_index())
            .field("attribute_type", &self.attribute_type())
            .field("is_active", &self.is_active())
            .finish()
    }
}
