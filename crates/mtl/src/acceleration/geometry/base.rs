//! Base geometry descriptor for acceleration structures.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::Buffer;

/// Base descriptor for geometry in acceleration structures.
///
/// C++ equivalent: `MTL::AccelerationStructureGeometryDescriptor`
#[repr(transparent)]
pub struct AccelerationStructureGeometryDescriptor(pub(crate) NonNull<c_void>);

impl AccelerationStructureGeometryDescriptor {
    /// Create a new geometry descriptor.
    ///
    /// C++ equivalent: `static AccelerationStructureGeometryDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLAccelerationStructureGeometryDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal acceleration structure geometry descriptor.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get whether duplicate intersection function invocation is allowed.
    ///
    /// C++ equivalent: `bool allowDuplicateIntersectionFunctionInvocation() const`
    #[inline]
    pub fn allow_duplicate_intersection_function_invocation(&self) -> bool {
        unsafe {
            msg_send_0(
                self.as_ptr(),
                sel!(allowDuplicateIntersectionFunctionInvocation),
            )
        }
    }

    /// Set whether duplicate intersection function invocation is allowed.
    ///
    /// C++ equivalent: `void setAllowDuplicateIntersectionFunctionInvocation(bool)`
    #[inline]
    pub fn set_allow_duplicate_intersection_function_invocation(&self, allow: bool) {
        unsafe {
            msg_send_1::<(), bool>(
                self.as_ptr(),
                sel!(setAllowDuplicateIntersectionFunctionInvocation:),
                allow,
            );
        }
    }

    /// Get the intersection function table offset.
    ///
    /// C++ equivalent: `NS::UInteger intersectionFunctionTableOffset() const`
    #[inline]
    pub fn intersection_function_table_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(intersectionFunctionTableOffset)) }
    }

    /// Set the intersection function table offset.
    ///
    /// C++ equivalent: `void setIntersectionFunctionTableOffset(NS::UInteger)`
    #[inline]
    pub fn set_intersection_function_table_offset(&self, offset: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(
                self.as_ptr(),
                sel!(setIntersectionFunctionTableOffset:),
                offset,
            );
        }
    }

    /// Get whether geometry is opaque.
    ///
    /// C++ equivalent: `bool opaque() const`
    #[inline]
    pub fn opaque(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(opaque)) }
    }

    /// Set whether geometry is opaque.
    ///
    /// C++ equivalent: `void setOpaque(bool)`
    #[inline]
    pub fn set_opaque(&self, opaque: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setOpaque:), opaque);
        }
    }

    /// Get the label.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
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

    /// Set the label.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = mtl_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the primitive data buffer.
    ///
    /// C++ equivalent: `Buffer* primitiveDataBuffer() const`
    pub fn primitive_data_buffer(&self) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(primitiveDataBuffer));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Buffer::from_raw(ptr)
        }
    }

    /// Set the primitive data buffer.
    ///
    /// C++ equivalent: `void setPrimitiveDataBuffer(Buffer*)`
    pub fn set_primitive_data_buffer(&self, buffer: Option<&Buffer>) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setPrimitiveDataBuffer:),
                buffer.map_or(std::ptr::null(), |b| b.as_ptr()),
            );
        }
    }

    /// Get the primitive data buffer offset.
    ///
    /// C++ equivalent: `NS::UInteger primitiveDataBufferOffset() const`
    #[inline]
    pub fn primitive_data_buffer_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(primitiveDataBufferOffset)) }
    }

    /// Set the primitive data buffer offset.
    ///
    /// C++ equivalent: `void setPrimitiveDataBufferOffset(NS::UInteger)`
    #[inline]
    pub fn set_primitive_data_buffer_offset(&self, offset: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setPrimitiveDataBufferOffset:), offset);
        }
    }

    /// Get the primitive data element size.
    ///
    /// C++ equivalent: `NS::UInteger primitiveDataElementSize() const`
    #[inline]
    pub fn primitive_data_element_size(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(primitiveDataElementSize)) }
    }

    /// Set the primitive data element size.
    ///
    /// C++ equivalent: `void setPrimitiveDataElementSize(NS::UInteger)`
    #[inline]
    pub fn set_primitive_data_element_size(&self, size: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setPrimitiveDataElementSize:), size);
        }
    }

    /// Get the primitive data stride.
    ///
    /// C++ equivalent: `NS::UInteger primitiveDataStride() const`
    #[inline]
    pub fn primitive_data_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(primitiveDataStride)) }
    }

    /// Set the primitive data stride.
    ///
    /// C++ equivalent: `void setPrimitiveDataStride(NS::UInteger)`
    #[inline]
    pub fn set_primitive_data_stride(&self, stride: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setPrimitiveDataStride:), stride);
        }
    }
}

impl Default for AccelerationStructureGeometryDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create geometry descriptor")
    }
}

impl Clone for AccelerationStructureGeometryDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy geometry descriptor")
        }
    }
}

impl Drop for AccelerationStructureGeometryDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for AccelerationStructureGeometryDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for AccelerationStructureGeometryDescriptor {}
unsafe impl Sync for AccelerationStructureGeometryDescriptor {}
