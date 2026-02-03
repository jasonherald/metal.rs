//! Base descriptor for geometry in an acceleration structure.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use super::BufferRange;

/// Base descriptor for geometry in an acceleration structure.
///
/// C++ equivalent: `MTL4::AccelerationStructureGeometryDescriptor`
#[repr(transparent)]
pub struct AccelerationStructureGeometryDescriptor(NonNull<c_void>);

impl AccelerationStructureGeometryDescriptor {
    /// Create an AccelerationStructureGeometryDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new acceleration structure geometry descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4AccelerationStructureGeometryDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get whether duplicate intersection function invocation is allowed.
    ///
    /// C++ equivalent: `bool allowDuplicateIntersectionFunctionInvocation() const`
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
    pub fn set_allow_duplicate_intersection_function_invocation(&self, allow: bool) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setAllowDuplicateIntersectionFunctionInvocation:),
                allow,
            );
        }
    }

    /// Get the intersection function table offset.
    ///
    /// C++ equivalent: `NS::UInteger intersectionFunctionTableOffset() const`
    pub fn intersection_function_table_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(intersectionFunctionTableOffset)) }
    }

    /// Set the intersection function table offset.
    ///
    /// C++ equivalent: `void setIntersectionFunctionTableOffset(NS::UInteger)`
    pub fn set_intersection_function_table_offset(&self, offset: UInteger) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setIntersectionFunctionTableOffset:),
                offset,
            );
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
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get whether the geometry is opaque.
    ///
    /// C++ equivalent: `bool opaque() const`
    pub fn opaque(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(opaque)) }
    }

    /// Set whether the geometry is opaque.
    ///
    /// C++ equivalent: `void setOpaque(bool)`
    pub fn set_opaque(&self, opaque: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOpaque:), opaque);
        }
    }

    /// Get the primitive data buffer.
    ///
    /// C++ equivalent: `BufferRange primitiveDataBuffer() const`
    pub fn primitive_data_buffer(&self) -> BufferRange {
        unsafe { msg_send_0(self.as_ptr(), sel!(primitiveDataBuffer)) }
    }

    /// Set the primitive data buffer.
    ///
    /// C++ equivalent: `void setPrimitiveDataBuffer(const MTL4::BufferRange)`
    pub fn set_primitive_data_buffer(&self, buffer: BufferRange) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setPrimitiveDataBuffer:), buffer);
        }
    }

    /// Get the primitive data element size.
    ///
    /// C++ equivalent: `NS::UInteger primitiveDataElementSize() const`
    pub fn primitive_data_element_size(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(primitiveDataElementSize)) }
    }

    /// Set the primitive data element size.
    ///
    /// C++ equivalent: `void setPrimitiveDataElementSize(NS::UInteger)`
    pub fn set_primitive_data_element_size(&self, size: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setPrimitiveDataElementSize:), size);
        }
    }

    /// Get the primitive data stride.
    ///
    /// C++ equivalent: `NS::UInteger primitiveDataStride() const`
    pub fn primitive_data_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(primitiveDataStride)) }
    }

    /// Set the primitive data stride.
    ///
    /// C++ equivalent: `void setPrimitiveDataStride(NS::UInteger)`
    pub fn set_primitive_data_stride(&self, stride: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setPrimitiveDataStride:), stride);
        }
    }
}

impl Default for AccelerationStructureGeometryDescriptor {
    fn default() -> Self {
        Self::new().expect("Failed to create MTL4AccelerationStructureGeometryDescriptor")
    }
}

impl Clone for AccelerationStructureGeometryDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for AccelerationStructureGeometryDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
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

impl std::fmt::Debug for AccelerationStructureGeometryDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureGeometryDescriptor")
            .field("label", &self.label())
            .field("opaque", &self.opaque())
            .finish()
    }
}
