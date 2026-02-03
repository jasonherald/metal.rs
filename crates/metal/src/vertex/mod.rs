//! Vertex descriptor types.
//!
//! Corresponds to `Metal/MTLVertexDescriptor.hpp`.
//!
//! Vertex descriptors describe the organization of vertex data in buffers
//! for use with render pipelines.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, sel, Class};

use crate::enums::{VertexFormat, VertexStepFunction};

/// Constant indicating that the stride should be computed dynamically.
///
/// C++ equivalent: `MTL::BufferLayoutStrideDynamic`
pub const BUFFER_LAYOUT_STRIDE_DYNAMIC: UInteger = UInteger::MAX;

// ============================================================================
// VertexBufferLayoutDescriptor
// ============================================================================

/// Describes the layout of vertex data in a buffer.
///
/// C++ equivalent: `MTL::VertexBufferLayoutDescriptor`
#[repr(transparent)]
pub struct VertexBufferLayoutDescriptor(pub(crate) NonNull<c_void>);

impl VertexBufferLayoutDescriptor {
    /// Create a new vertex buffer layout descriptor.
    ///
    /// C++ equivalent: `static VertexBufferLayoutDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = Class::get("MTLVertexBufferLayoutDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a VertexBufferLayoutDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal vertex buffer layout descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the descriptor.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the stride, in bytes, between vertex data in the buffer.
    ///
    /// C++ equivalent: `NS::UInteger stride() const`
    #[inline]
    pub fn stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(stride)) }
    }

    /// Set the stride, in bytes, between vertex data in the buffer.
    ///
    /// C++ equivalent: `void setStride(NS::UInteger stride)`
    #[inline]
    pub fn set_stride(&self, stride: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setStride:), stride);
        }
    }

    /// Get the step function that determines how the data is fetched.
    ///
    /// C++ equivalent: `VertexStepFunction stepFunction() const`
    #[inline]
    pub fn step_function(&self) -> VertexStepFunction {
        unsafe { msg_send_0(self.as_ptr(), sel!(stepFunction)) }
    }

    /// Set the step function that determines how the data is fetched.
    ///
    /// C++ equivalent: `void setStepFunction(MTL::VertexStepFunction stepFunction)`
    #[inline]
    pub fn set_step_function(&self, step_function: VertexStepFunction) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setStepFunction:), step_function);
        }
    }

    /// Get the step rate that determines how often the data is fetched.
    ///
    /// C++ equivalent: `NS::UInteger stepRate() const`
    #[inline]
    pub fn step_rate(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(stepRate)) }
    }

    /// Set the step rate that determines how often the data is fetched.
    ///
    /// C++ equivalent: `void setStepRate(NS::UInteger stepRate)`
    #[inline]
    pub fn set_step_rate(&self, step_rate: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setStepRate:), step_rate);
        }
    }
}

impl Clone for VertexBufferLayoutDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for VertexBufferLayoutDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for VertexBufferLayoutDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for VertexBufferLayoutDescriptor {}
unsafe impl Sync for VertexBufferLayoutDescriptor {}

impl std::fmt::Debug for VertexBufferLayoutDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VertexBufferLayoutDescriptor")
            .field("stride", &self.stride())
            .field("step_function", &self.step_function())
            .field("step_rate", &self.step_rate())
            .finish()
    }
}

// ============================================================================
// VertexBufferLayoutDescriptorArray
// ============================================================================

/// An array of vertex buffer layout descriptors.
///
/// C++ equivalent: `MTL::VertexBufferLayoutDescriptorArray`
#[repr(transparent)]
pub struct VertexBufferLayoutDescriptorArray(pub(crate) NonNull<c_void>);

impl VertexBufferLayoutDescriptorArray {
    /// Create a new vertex buffer layout descriptor array.
    ///
    /// C++ equivalent: `static VertexBufferLayoutDescriptorArray* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = Class::get("MTLVertexBufferLayoutDescriptorArray")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a VertexBufferLayoutDescriptorArray from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal vertex buffer layout descriptor array object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the array.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Array Access
    // =========================================================================

    /// Get the descriptor at the specified index.
    ///
    /// C++ equivalent: `VertexBufferLayoutDescriptor* object(NS::UInteger index)`
    pub fn object(&self, index: UInteger) -> Option<VertexBufferLayoutDescriptor> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(objectAtIndexedSubscript:), index);
            if ptr.is_null() {
                return None;
            }
            // Retain since we're returning an owned reference
            msg_send_0::<*mut c_void>(ptr, sel!(retain));
            VertexBufferLayoutDescriptor::from_raw(ptr)
        }
    }

    /// Set the descriptor at the specified index.
    ///
    /// C++ equivalent: `void setObject(const MTL::VertexBufferLayoutDescriptor* bufferDesc, NS::UInteger index)`
    pub fn set_object(&self, buffer_desc: &VertexBufferLayoutDescriptor, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setObject:atIndexedSubscript:),
                buffer_desc.as_ptr(),
                index,
            );
        }
    }
}

impl Clone for VertexBufferLayoutDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for VertexBufferLayoutDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for VertexBufferLayoutDescriptorArray {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for VertexBufferLayoutDescriptorArray {}
unsafe impl Sync for VertexBufferLayoutDescriptorArray {}

impl std::fmt::Debug for VertexBufferLayoutDescriptorArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VertexBufferLayoutDescriptorArray").finish()
    }
}

// ============================================================================
// VertexAttributeDescriptor
// ============================================================================

/// Describes a vertex attribute in a vertex descriptor.
///
/// C++ equivalent: `MTL::VertexAttributeDescriptor`
#[repr(transparent)]
pub struct VertexAttributeDescriptor(pub(crate) NonNull<c_void>);

impl VertexAttributeDescriptor {
    /// Create a new vertex attribute descriptor.
    ///
    /// C++ equivalent: `static VertexAttributeDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = Class::get("MTLVertexAttributeDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a VertexAttributeDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal vertex attribute descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the descriptor.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the format of the vertex attribute.
    ///
    /// C++ equivalent: `VertexFormat format() const`
    #[inline]
    pub fn format(&self) -> VertexFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(format)) }
    }

    /// Set the format of the vertex attribute.
    ///
    /// C++ equivalent: `void setFormat(MTL::VertexFormat format)`
    #[inline]
    pub fn set_format(&self, format: VertexFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setFormat:), format);
        }
    }

    /// Get the offset, in bytes, of this attribute in the vertex buffer.
    ///
    /// C++ equivalent: `NS::UInteger offset() const`
    #[inline]
    pub fn offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(offset)) }
    }

    /// Set the offset, in bytes, of this attribute in the vertex buffer.
    ///
    /// C++ equivalent: `void setOffset(NS::UInteger offset)`
    #[inline]
    pub fn set_offset(&self, offset: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOffset:), offset);
        }
    }

    /// Get the index of the buffer that contains this attribute.
    ///
    /// C++ equivalent: `NS::UInteger bufferIndex() const`
    #[inline]
    pub fn buffer_index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(bufferIndex)) }
    }

    /// Set the index of the buffer that contains this attribute.
    ///
    /// C++ equivalent: `void setBufferIndex(NS::UInteger bufferIndex)`
    #[inline]
    pub fn set_buffer_index(&self, buffer_index: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setBufferIndex:), buffer_index);
        }
    }
}

impl Clone for VertexAttributeDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for VertexAttributeDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for VertexAttributeDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for VertexAttributeDescriptor {}
unsafe impl Sync for VertexAttributeDescriptor {}

impl std::fmt::Debug for VertexAttributeDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VertexAttributeDescriptor")
            .field("format", &self.format())
            .field("offset", &self.offset())
            .field("buffer_index", &self.buffer_index())
            .finish()
    }
}

// ============================================================================
// VertexAttributeDescriptorArray
// ============================================================================

/// An array of vertex attribute descriptors.
///
/// C++ equivalent: `MTL::VertexAttributeDescriptorArray`
#[repr(transparent)]
pub struct VertexAttributeDescriptorArray(pub(crate) NonNull<c_void>);

impl VertexAttributeDescriptorArray {
    /// Create a new vertex attribute descriptor array.
    ///
    /// C++ equivalent: `static VertexAttributeDescriptorArray* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = Class::get("MTLVertexAttributeDescriptorArray")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a VertexAttributeDescriptorArray from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal vertex attribute descriptor array object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the array.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Array Access
    // =========================================================================

    /// Get the descriptor at the specified index.
    ///
    /// C++ equivalent: `VertexAttributeDescriptor* object(NS::UInteger index)`
    pub fn object(&self, index: UInteger) -> Option<VertexAttributeDescriptor> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(objectAtIndexedSubscript:), index);
            if ptr.is_null() {
                return None;
            }
            // Retain since we're returning an owned reference
            msg_send_0::<*mut c_void>(ptr, sel!(retain));
            VertexAttributeDescriptor::from_raw(ptr)
        }
    }

    /// Set the descriptor at the specified index.
    ///
    /// C++ equivalent: `void setObject(const MTL::VertexAttributeDescriptor* attributeDesc, NS::UInteger index)`
    pub fn set_object(&self, attribute_desc: &VertexAttributeDescriptor, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setObject:atIndexedSubscript:),
                attribute_desc.as_ptr(),
                index,
            );
        }
    }
}

impl Clone for VertexAttributeDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for VertexAttributeDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for VertexAttributeDescriptorArray {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for VertexAttributeDescriptorArray {}
unsafe impl Sync for VertexAttributeDescriptorArray {}

impl std::fmt::Debug for VertexAttributeDescriptorArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VertexAttributeDescriptorArray").finish()
    }
}

// ============================================================================
// VertexDescriptor
// ============================================================================

/// Describes the organization of vertex data for a render pipeline.
///
/// C++ equivalent: `MTL::VertexDescriptor`
#[repr(transparent)]
pub struct VertexDescriptor(pub(crate) NonNull<c_void>);

impl VertexDescriptor {
    /// Create a new vertex descriptor.
    ///
    /// C++ equivalent: `static VertexDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = Class::get("MTLVertexDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new vertex descriptor using the class method.
    ///
    /// C++ equivalent: `static VertexDescriptor* vertexDescriptor()`
    pub fn vertex_descriptor() -> Option<Self> {
        unsafe {
            let class = Class::get("MTLVertexDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(vertexDescriptor));
            if ptr.is_null() {
                return None;
            }
            // Retain since this is an autoreleased object
            msg_send_0::<*mut c_void>(ptr, sel!(retain));
            Self::from_raw(ptr)
        }
    }

    /// Create a VertexDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal vertex descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the descriptor.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the array of vertex buffer layout descriptors.
    ///
    /// C++ equivalent: `VertexBufferLayoutDescriptorArray* layouts() const`
    pub fn layouts(&self) -> VertexBufferLayoutDescriptorArray {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(layouts));
            // Retain since we're returning an owned reference
            msg_send_0::<*mut c_void>(ptr, sel!(retain));
            VertexBufferLayoutDescriptorArray::from_raw(ptr)
                .expect("vertex descriptor layouts should not be null")
        }
    }

    /// Get the array of vertex attribute descriptors.
    ///
    /// C++ equivalent: `VertexAttributeDescriptorArray* attributes() const`
    pub fn attributes(&self) -> VertexAttributeDescriptorArray {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(attributes));
            // Retain since we're returning an owned reference
            msg_send_0::<*mut c_void>(ptr, sel!(retain));
            VertexAttributeDescriptorArray::from_raw(ptr)
                .expect("vertex descriptor attributes should not be null")
        }
    }

    // =========================================================================
    // Methods
    // =========================================================================

    /// Reset the vertex descriptor to default values.
    ///
    /// C++ equivalent: `void reset()`
    #[inline]
    pub fn reset(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(reset));
        }
    }
}

impl Clone for VertexDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for VertexDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for VertexDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for VertexDescriptor {}
unsafe impl Sync for VertexDescriptor {}

impl std::fmt::Debug for VertexDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VertexDescriptor").finish()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_layout_stride_dynamic() {
        assert_eq!(BUFFER_LAYOUT_STRIDE_DYNAMIC, UInteger::MAX);
    }

    #[test]
    fn test_vertex_descriptor_creation() {
        let desc = VertexDescriptor::new();
        assert!(desc.is_some());
    }

    #[test]
    fn test_vertex_descriptor_class_method() {
        let desc = VertexDescriptor::vertex_descriptor();
        assert!(desc.is_some());
    }

    #[test]
    fn test_vertex_buffer_layout_descriptor() {
        let desc = VertexBufferLayoutDescriptor::new();
        assert!(desc.is_some());

        let desc = desc.unwrap();
        // Default stride should be 0
        assert_eq!(desc.stride(), 0);

        desc.set_stride(32);
        assert_eq!(desc.stride(), 32);

        // Default step function should be PER_VERTEX
        assert_eq!(desc.step_function(), VertexStepFunction::PER_VERTEX);

        desc.set_step_function(VertexStepFunction::PER_INSTANCE);
        assert_eq!(desc.step_function(), VertexStepFunction::PER_INSTANCE);

        // Default step rate should be 1
        assert_eq!(desc.step_rate(), 1);

        desc.set_step_rate(2);
        assert_eq!(desc.step_rate(), 2);
    }

    #[test]
    fn test_vertex_attribute_descriptor() {
        let desc = VertexAttributeDescriptor::new();
        assert!(desc.is_some());

        let desc = desc.unwrap();
        // Default format should be INVALID
        assert_eq!(desc.format(), VertexFormat::INVALID);

        desc.set_format(VertexFormat::FLOAT3);
        assert_eq!(desc.format(), VertexFormat::FLOAT3);

        // Default offset should be 0
        assert_eq!(desc.offset(), 0);

        desc.set_offset(16);
        assert_eq!(desc.offset(), 16);

        // Default buffer index should be 0
        assert_eq!(desc.buffer_index(), 0);

        desc.set_buffer_index(1);
        assert_eq!(desc.buffer_index(), 1);
    }

    #[test]
    fn test_vertex_descriptor_layouts_and_attributes() {
        let desc = VertexDescriptor::new().unwrap();

        // Get layouts array and configure
        let layouts = desc.layouts();
        let layout = layouts.object(0);
        assert!(layout.is_some());

        let layout = layout.unwrap();
        layout.set_stride(32);
        layout.set_step_function(VertexStepFunction::PER_VERTEX);

        // Get attributes array and configure
        let attributes = desc.attributes();
        let attr = attributes.object(0);
        assert!(attr.is_some());

        let attr = attr.unwrap();
        attr.set_format(VertexFormat::FLOAT3);
        attr.set_offset(0);
        attr.set_buffer_index(0);
    }

    #[test]
    fn test_vertex_descriptor_reset() {
        let desc = VertexDescriptor::new().unwrap();

        // Configure some attributes
        let attributes = desc.attributes();
        let attr = attributes.object(0).unwrap();
        attr.set_format(VertexFormat::FLOAT4);
        attr.set_offset(16);

        // Reset should clear to defaults
        desc.reset();

        let attr = attributes.object(0).unwrap();
        assert_eq!(attr.format(), VertexFormat::INVALID);
        assert_eq!(attr.offset(), 0);
    }
}
