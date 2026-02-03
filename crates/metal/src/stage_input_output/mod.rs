//! Stage input/output descriptors.
//!
//! Corresponds to `Metal/MTLStageInputOutputDescriptor.hpp`.
//!
//! These descriptors are used to describe the layout of data passed between
//! pipeline stages in compute kernels.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, sel};

use crate::enums::{AttributeFormat, IndexType, StepFunction};

// ============================================================================
// BufferLayoutDescriptor
// ============================================================================

/// Describes the layout of data in a buffer for stage input/output.
///
/// C++ equivalent: `MTL::BufferLayoutDescriptor`
#[repr(transparent)]
pub struct BufferLayoutDescriptor(pub(crate) NonNull<c_void>);

impl BufferLayoutDescriptor {
    /// Create a new buffer layout descriptor.
    ///
    /// C++ equivalent: `BufferLayoutDescriptor::alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::class!(MTLBufferLayoutDescriptor);
            let obj: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if obj.is_null() {
                return None;
            }
            let obj: *mut c_void = msg_send_0(obj, sel!(init));
            Self::from_raw(obj)
        }
    }

    /// Create from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the step function.
    ///
    /// C++ equivalent: `StepFunction stepFunction() const`
    #[inline]
    pub fn step_function(&self) -> StepFunction {
        unsafe { msg_send_0(self.as_ptr(), sel!(stepFunction)) }
    }

    /// Set the step function.
    ///
    /// C++ equivalent: `void setStepFunction(StepFunction)`
    #[inline]
    pub fn set_step_function(&self, step_function: StepFunction) {
        unsafe {
            msg_send_1::<(), StepFunction>(self.as_ptr(), sel!(setStepFunction:), step_function);
        }
    }

    /// Get the step rate.
    ///
    /// C++ equivalent: `NS::UInteger stepRate() const`
    #[inline]
    pub fn step_rate(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(stepRate)) }
    }

    /// Set the step rate.
    ///
    /// C++ equivalent: `void setStepRate(NS::UInteger)`
    #[inline]
    pub fn set_step_rate(&self, step_rate: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setStepRate:), step_rate);
        }
    }

    /// Get the stride.
    ///
    /// C++ equivalent: `NS::UInteger stride() const`
    #[inline]
    pub fn stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(stride)) }
    }

    /// Set the stride.
    ///
    /// C++ equivalent: `void setStride(NS::UInteger)`
    #[inline]
    pub fn set_stride(&self, stride: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setStride:), stride);
        }
    }
}

impl Default for BufferLayoutDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create BufferLayoutDescriptor")
    }
}

impl Clone for BufferLayoutDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for BufferLayoutDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for BufferLayoutDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for BufferLayoutDescriptor {}
unsafe impl Sync for BufferLayoutDescriptor {}

// ============================================================================
// BufferLayoutDescriptorArray
// ============================================================================

/// An array of buffer layout descriptors.
///
/// C++ equivalent: `MTL::BufferLayoutDescriptorArray`
#[repr(transparent)]
pub struct BufferLayoutDescriptorArray(pub(crate) NonNull<c_void>);

impl BufferLayoutDescriptorArray {
    /// Create from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the descriptor at the specified index.
    ///
    /// C++ equivalent: `BufferLayoutDescriptor* object(NS::UInteger index)`
    pub fn object_at(&self, index: UInteger) -> Option<BufferLayoutDescriptor> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(objectAtIndexedSubscript:), index);
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            BufferLayoutDescriptor::from_raw(ptr)
        }
    }

    /// Set the descriptor at the specified index.
    ///
    /// C++ equivalent: `void setObject(const BufferLayoutDescriptor*, NS::UInteger)`
    pub fn set_object_at(&self, descriptor: &BufferLayoutDescriptor, index: UInteger) {
        unsafe {
            msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setObject: atIndexedSubscript:),
                descriptor.as_ptr(),
                index,
            );
        }
    }
}

impl Clone for BufferLayoutDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for BufferLayoutDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for BufferLayoutDescriptorArray {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for BufferLayoutDescriptorArray {}
unsafe impl Sync for BufferLayoutDescriptorArray {}

// ============================================================================
// AttributeDescriptor
// ============================================================================

/// Describes an attribute in a stage input/output descriptor.
///
/// C++ equivalent: `MTL::AttributeDescriptor`
#[repr(transparent)]
pub struct AttributeDescriptor(pub(crate) NonNull<c_void>);

impl AttributeDescriptor {
    /// Create a new attribute descriptor.
    ///
    /// C++ equivalent: `AttributeDescriptor::alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::class!(MTLAttributeDescriptor);
            let obj: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if obj.is_null() {
                return None;
            }
            let obj: *mut c_void = msg_send_0(obj, sel!(init));
            Self::from_raw(obj)
        }
    }

    /// Create from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the format.
    ///
    /// C++ equivalent: `AttributeFormat format() const`
    #[inline]
    pub fn format(&self) -> AttributeFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(format)) }
    }

    /// Set the format.
    ///
    /// C++ equivalent: `void setFormat(AttributeFormat)`
    #[inline]
    pub fn set_format(&self, format: AttributeFormat) {
        unsafe {
            msg_send_1::<(), AttributeFormat>(self.as_ptr(), sel!(setFormat:), format);
        }
    }

    /// Get the offset.
    ///
    /// C++ equivalent: `NS::UInteger offset() const`
    #[inline]
    pub fn offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(offset)) }
    }

    /// Set the offset.
    ///
    /// C++ equivalent: `void setOffset(NS::UInteger)`
    #[inline]
    pub fn set_offset(&self, offset: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setOffset:), offset);
        }
    }

    /// Get the buffer index.
    ///
    /// C++ equivalent: `NS::UInteger bufferIndex() const`
    #[inline]
    pub fn buffer_index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(bufferIndex)) }
    }

    /// Set the buffer index.
    ///
    /// C++ equivalent: `void setBufferIndex(NS::UInteger)`
    #[inline]
    pub fn set_buffer_index(&self, buffer_index: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setBufferIndex:), buffer_index);
        }
    }
}

impl Default for AttributeDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create AttributeDescriptor")
    }
}

impl Clone for AttributeDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for AttributeDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for AttributeDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for AttributeDescriptor {}
unsafe impl Sync for AttributeDescriptor {}

// ============================================================================
// AttributeDescriptorArray
// ============================================================================

/// An array of attribute descriptors.
///
/// C++ equivalent: `MTL::AttributeDescriptorArray`
#[repr(transparent)]
pub struct AttributeDescriptorArray(pub(crate) NonNull<c_void>);

impl AttributeDescriptorArray {
    /// Create from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the descriptor at the specified index.
    ///
    /// C++ equivalent: `AttributeDescriptor* object(NS::UInteger index)`
    pub fn object_at(&self, index: UInteger) -> Option<AttributeDescriptor> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(objectAtIndexedSubscript:), index);
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            AttributeDescriptor::from_raw(ptr)
        }
    }

    /// Set the descriptor at the specified index.
    ///
    /// C++ equivalent: `void setObject(const AttributeDescriptor*, NS::UInteger)`
    pub fn set_object_at(&self, descriptor: &AttributeDescriptor, index: UInteger) {
        unsafe {
            msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setObject: atIndexedSubscript:),
                descriptor.as_ptr(),
                index,
            );
        }
    }
}

impl Clone for AttributeDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for AttributeDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for AttributeDescriptorArray {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for AttributeDescriptorArray {}
unsafe impl Sync for AttributeDescriptorArray {}

// ============================================================================
// StageInputOutputDescriptor
// ============================================================================

/// Describes the input and output data for a compute kernel stage.
///
/// C++ equivalent: `MTL::StageInputOutputDescriptor`
#[repr(transparent)]
pub struct StageInputOutputDescriptor(pub(crate) NonNull<c_void>);

impl StageInputOutputDescriptor {
    /// Create a new stage input/output descriptor.
    ///
    /// C++ equivalent: `StageInputOutputDescriptor::stageInputOutputDescriptor()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::class!(MTLStageInputOutputDescriptor);
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(stageInputOutputDescriptor));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the buffer layouts array.
    ///
    /// C++ equivalent: `BufferLayoutDescriptorArray* layouts() const`
    pub fn layouts(&self) -> Option<BufferLayoutDescriptorArray> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(layouts));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            BufferLayoutDescriptorArray::from_raw(ptr)
        }
    }

    /// Get the attributes array.
    ///
    /// C++ equivalent: `AttributeDescriptorArray* attributes() const`
    pub fn attributes(&self) -> Option<AttributeDescriptorArray> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(attributes));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            AttributeDescriptorArray::from_raw(ptr)
        }
    }

    /// Get the index type.
    ///
    /// C++ equivalent: `IndexType indexType() const`
    #[inline]
    pub fn index_type(&self) -> IndexType {
        unsafe { msg_send_0(self.as_ptr(), sel!(indexType)) }
    }

    /// Set the index type.
    ///
    /// C++ equivalent: `void setIndexType(IndexType)`
    #[inline]
    pub fn set_index_type(&self, index_type: IndexType) {
        unsafe {
            msg_send_1::<(), IndexType>(self.as_ptr(), sel!(setIndexType:), index_type);
        }
    }

    /// Get the index buffer index.
    ///
    /// C++ equivalent: `NS::UInteger indexBufferIndex() const`
    #[inline]
    pub fn index_buffer_index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(indexBufferIndex)) }
    }

    /// Set the index buffer index.
    ///
    /// C++ equivalent: `void setIndexBufferIndex(NS::UInteger)`
    #[inline]
    pub fn set_index_buffer_index(&self, index: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setIndexBufferIndex:), index);
        }
    }

    /// Reset the descriptor to its default state.
    ///
    /// C++ equivalent: `void reset()`
    #[inline]
    pub fn reset(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(reset));
        }
    }
}

impl Default for StageInputOutputDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create StageInputOutputDescriptor")
    }
}

impl Clone for StageInputOutputDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for StageInputOutputDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for StageInputOutputDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for StageInputOutputDescriptor {}
unsafe impl Sync for StageInputOutputDescriptor {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_sizes() {
        assert_eq!(
            std::mem::size_of::<BufferLayoutDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
        assert_eq!(
            std::mem::size_of::<AttributeDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
        assert_eq!(
            std::mem::size_of::<StageInputOutputDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
