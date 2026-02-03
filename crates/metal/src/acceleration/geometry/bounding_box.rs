//! Bounding box geometry descriptor for acceleration structures.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::Buffer;

/// Descriptor for bounding box geometry in acceleration structures.
///
/// C++ equivalent: `MTL::AccelerationStructureBoundingBoxGeometryDescriptor`
#[repr(transparent)]
pub struct AccelerationStructureBoundingBoxGeometryDescriptor(pub(crate) NonNull<c_void>);

impl AccelerationStructureBoundingBoxGeometryDescriptor {
    /// Create a new bounding box geometry descriptor.
    ///
    /// C++ equivalent: `static AccelerationStructureBoundingBoxGeometryDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class =
                metal_sys::Class::get("MTLAccelerationStructureBoundingBoxGeometryDescriptor")?;
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
    /// The pointer must be a valid Metal bounding box geometry descriptor.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // Inherited properties from AccelerationStructureGeometryDescriptor

    /// Get whether duplicate intersection function invocation is allowed.
    #[inline]
    pub fn allow_duplicate_intersection_function_invocation(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(allowDuplicateIntersectionFunctionInvocation)) }
    }

    /// Set whether duplicate intersection function invocation is allowed.
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
    #[inline]
    pub fn intersection_function_table_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(intersectionFunctionTableOffset)) }
    }

    /// Set the intersection function table offset.
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
    #[inline]
    pub fn opaque(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(opaque)) }
    }

    /// Set whether geometry is opaque.
    #[inline]
    pub fn set_opaque(&self, opaque: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setOpaque:), opaque);
        }
    }

    /// Get the label.
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

    /// Set the label.
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the primitive data buffer.
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
    #[inline]
    pub fn primitive_data_buffer_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(primitiveDataBufferOffset)) }
    }

    /// Set the primitive data buffer offset.
    #[inline]
    pub fn set_primitive_data_buffer_offset(&self, offset: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setPrimitiveDataBufferOffset:), offset);
        }
    }

    /// Get the primitive data element size.
    #[inline]
    pub fn primitive_data_element_size(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(primitiveDataElementSize)) }
    }

    /// Set the primitive data element size.
    #[inline]
    pub fn set_primitive_data_element_size(&self, size: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setPrimitiveDataElementSize:), size);
        }
    }

    /// Get the primitive data stride.
    #[inline]
    pub fn primitive_data_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(primitiveDataStride)) }
    }

    /// Set the primitive data stride.
    #[inline]
    pub fn set_primitive_data_stride(&self, stride: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setPrimitiveDataStride:), stride);
        }
    }

    // Bounding box-specific properties

    /// Get the bounding box buffer.
    ///
    /// C++ equivalent: `Buffer* boundingBoxBuffer() const`
    pub fn bounding_box_buffer(&self) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(boundingBoxBuffer));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Buffer::from_raw(ptr)
        }
    }

    /// Set the bounding box buffer.
    ///
    /// C++ equivalent: `void setBoundingBoxBuffer(Buffer*)`
    pub fn set_bounding_box_buffer(&self, buffer: Option<&Buffer>) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setBoundingBoxBuffer:),
                buffer.map_or(std::ptr::null(), |b| b.as_ptr()),
            );
        }
    }

    /// Get the bounding box buffer offset.
    ///
    /// C++ equivalent: `NS::UInteger boundingBoxBufferOffset() const`
    #[inline]
    pub fn bounding_box_buffer_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(boundingBoxBufferOffset)) }
    }

    /// Set the bounding box buffer offset.
    ///
    /// C++ equivalent: `void setBoundingBoxBufferOffset(NS::UInteger)`
    #[inline]
    pub fn set_bounding_box_buffer_offset(&self, offset: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setBoundingBoxBufferOffset:), offset);
        }
    }

    /// Get the bounding box count.
    ///
    /// C++ equivalent: `NS::UInteger boundingBoxCount() const`
    #[inline]
    pub fn bounding_box_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(boundingBoxCount)) }
    }

    /// Set the bounding box count.
    ///
    /// C++ equivalent: `void setBoundingBoxCount(NS::UInteger)`
    #[inline]
    pub fn set_bounding_box_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setBoundingBoxCount:), count);
        }
    }

    /// Get the bounding box stride.
    ///
    /// C++ equivalent: `NS::UInteger boundingBoxStride() const`
    #[inline]
    pub fn bounding_box_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(boundingBoxStride)) }
    }

    /// Set the bounding box stride.
    ///
    /// C++ equivalent: `void setBoundingBoxStride(NS::UInteger)`
    #[inline]
    pub fn set_bounding_box_stride(&self, stride: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setBoundingBoxStride:), stride);
        }
    }
}

impl Default for AccelerationStructureBoundingBoxGeometryDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create bounding box geometry descriptor")
    }
}

impl Clone for AccelerationStructureBoundingBoxGeometryDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy bounding box geometry descriptor")
        }
    }
}

impl Drop for AccelerationStructureBoundingBoxGeometryDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for AccelerationStructureBoundingBoxGeometryDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for AccelerationStructureBoundingBoxGeometryDescriptor {}
unsafe impl Sync for AccelerationStructureBoundingBoxGeometryDescriptor {}
