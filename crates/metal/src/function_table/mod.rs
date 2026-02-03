//! Function table types.
//!
//! Corresponds to `Metal/MTLFunctionHandle.hpp`, `Metal/MTLVisibleFunctionTable.hpp`,
//! and `Metal/MTLIntersectionFunctionTable.hpp`.
//!
//! Function tables provide a way to dynamically dispatch to shader functions,
//! which is essential for ray tracing and other advanced rendering techniques.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Range, Referencing, UInteger};
use metal_sys::{Class, msg_send_0, msg_send_1, msg_send_2, msg_send_3, sel};

use crate::Buffer;
use crate::enums::{FunctionType, IntersectionFunctionSignature};
use crate::types::ResourceID;

// ============================================================================
// IntersectionFunctionBufferArguments
// ============================================================================

/// Arguments for an intersection function buffer.
///
/// C++ equivalent: `MTL::IntersectionFunctionBufferArguments`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct IntersectionFunctionBufferArguments {
    pub intersection_function_buffer: u64,
    pub intersection_function_buffer_size: u64,
    pub intersection_function_stride: u64,
}

// ============================================================================
// FunctionHandle
// ============================================================================

/// A handle to a compiled function.
///
/// C++ equivalent: `MTL::FunctionHandle`
#[repr(transparent)]
pub struct FunctionHandle(pub(crate) NonNull<c_void>);

impl FunctionHandle {
    /// Create a FunctionHandle from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal function handle object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the function handle.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the device that created this function handle.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("function handle has no device")
        }
    }

    /// Get the type of the function.
    ///
    /// C++ equivalent: `FunctionType functionType() const`
    #[inline]
    pub fn function_type(&self) -> FunctionType {
        unsafe { msg_send_0(self.as_ptr(), sel!(functionType)) }
    }

    /// Get the GPU resource ID for bindless access.
    ///
    /// C++ equivalent: `ResourceID gpuResourceID() const`
    #[inline]
    pub fn gpu_resource_id(&self) -> ResourceID {
        unsafe { msg_send_0(self.as_ptr(), sel!(gpuResourceID)) }
    }

    /// Get the name of the function.
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
}

impl Clone for FunctionHandle {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for FunctionHandle {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for FunctionHandle {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for FunctionHandle {}
unsafe impl Sync for FunctionHandle {}

impl std::fmt::Debug for FunctionHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionHandle")
            .field("name", &self.name())
            .field("function_type", &self.function_type())
            .finish()
    }
}

// ============================================================================
// VisibleFunctionTableDescriptor
// ============================================================================

/// A descriptor for creating a visible function table.
///
/// C++ equivalent: `MTL::VisibleFunctionTableDescriptor`
#[repr(transparent)]
pub struct VisibleFunctionTableDescriptor(pub(crate) NonNull<c_void>);

impl VisibleFunctionTableDescriptor {
    /// Create a new visible function table descriptor.
    ///
    /// C++ equivalent: `static VisibleFunctionTableDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = Class::get("MTLVisibleFunctionTableDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new visible function table descriptor using the class method.
    ///
    /// C++ equivalent: `static VisibleFunctionTableDescriptor* visibleFunctionTableDescriptor()`
    pub fn visible_function_table_descriptor() -> Option<Self> {
        unsafe {
            let class = Class::get("MTLVisibleFunctionTableDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(visibleFunctionTableDescriptor));
            if ptr.is_null() {
                return None;
            }
            // Retain since this is an autoreleased object
            msg_send_0::<*mut c_void>(ptr, sel!(retain));
            Self::from_raw(ptr)
        }
    }

    /// Create a VisibleFunctionTableDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal visible function table descriptor object.
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

    /// Get the number of functions in the table.
    ///
    /// C++ equivalent: `NS::UInteger functionCount() const`
    #[inline]
    pub fn function_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(functionCount)) }
    }

    /// Set the number of functions in the table.
    ///
    /// C++ equivalent: `void setFunctionCount(NS::UInteger functionCount)`
    #[inline]
    pub fn set_function_count(&self, function_count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setFunctionCount:), function_count);
        }
    }
}

impl Clone for VisibleFunctionTableDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for VisibleFunctionTableDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for VisibleFunctionTableDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for VisibleFunctionTableDescriptor {}
unsafe impl Sync for VisibleFunctionTableDescriptor {}

impl std::fmt::Debug for VisibleFunctionTableDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VisibleFunctionTableDescriptor")
            .field("function_count", &self.function_count())
            .finish()
    }
}

// ============================================================================
// VisibleFunctionTable
// ============================================================================

/// A table of visible functions for shader function pointers.
///
/// C++ equivalent: `MTL::VisibleFunctionTable`
///
/// Inherits from Resource.
#[repr(transparent)]
pub struct VisibleFunctionTable(pub(crate) NonNull<c_void>);

impl VisibleFunctionTable {
    /// Create a VisibleFunctionTable from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal visible function table object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the function table.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the GPU resource ID for bindless access.
    ///
    /// C++ equivalent: `ResourceID gpuResourceID() const`
    #[inline]
    pub fn gpu_resource_id(&self) -> ResourceID {
        unsafe { msg_send_0(self.as_ptr(), sel!(gpuResourceID)) }
    }

    // =========================================================================
    // Methods
    // =========================================================================

    /// Set a function at the specified index.
    ///
    /// C++ equivalent: `void setFunction(const MTL::FunctionHandle* function, NS::UInteger index)`
    pub fn set_function(&self, function: &FunctionHandle, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setFunction:atIndex:),
                function.as_ptr(),
                index,
            );
        }
    }

    /// Set multiple functions at a range of indices.
    ///
    /// C++ equivalent: `void setFunctions(const MTL::FunctionHandle* const functions[], NS::Range range)`
    pub fn set_functions(&self, functions: &[&FunctionHandle], range: Range) {
        let ptrs: Vec<*const c_void> = functions.iter().map(|f| f.as_ptr()).collect();
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setFunctions:withRange:),
                ptrs.as_ptr(),
                range,
            );
        }
    }
}

impl Clone for VisibleFunctionTable {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for VisibleFunctionTable {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for VisibleFunctionTable {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for VisibleFunctionTable {}
unsafe impl Sync for VisibleFunctionTable {}

impl std::fmt::Debug for VisibleFunctionTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VisibleFunctionTable").finish()
    }
}

// ============================================================================
// IntersectionFunctionTableDescriptor
// ============================================================================

/// A descriptor for creating an intersection function table.
///
/// C++ equivalent: `MTL::IntersectionFunctionTableDescriptor`
#[repr(transparent)]
pub struct IntersectionFunctionTableDescriptor(pub(crate) NonNull<c_void>);

impl IntersectionFunctionTableDescriptor {
    /// Create a new intersection function table descriptor.
    ///
    /// C++ equivalent: `static IntersectionFunctionTableDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = Class::get("MTLIntersectionFunctionTableDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new intersection function table descriptor using the class method.
    ///
    /// C++ equivalent: `static IntersectionFunctionTableDescriptor* intersectionFunctionTableDescriptor()`
    pub fn intersection_function_table_descriptor() -> Option<Self> {
        unsafe {
            let class = Class::get("MTLIntersectionFunctionTableDescriptor")?;
            let ptr: *mut c_void =
                msg_send_0(class.as_ptr(), sel!(intersectionFunctionTableDescriptor));
            if ptr.is_null() {
                return None;
            }
            // Retain since this is an autoreleased object
            msg_send_0::<*mut c_void>(ptr, sel!(retain));
            Self::from_raw(ptr)
        }
    }

    /// Create an IntersectionFunctionTableDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal intersection function table descriptor object.
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

    /// Get the number of functions in the table.
    ///
    /// C++ equivalent: `NS::UInteger functionCount() const`
    #[inline]
    pub fn function_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(functionCount)) }
    }

    /// Set the number of functions in the table.
    ///
    /// C++ equivalent: `void setFunctionCount(NS::UInteger functionCount)`
    #[inline]
    pub fn set_function_count(&self, function_count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setFunctionCount:), function_count);
        }
    }
}

impl Clone for IntersectionFunctionTableDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for IntersectionFunctionTableDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for IntersectionFunctionTableDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for IntersectionFunctionTableDescriptor {}
unsafe impl Sync for IntersectionFunctionTableDescriptor {}

impl std::fmt::Debug for IntersectionFunctionTableDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IntersectionFunctionTableDescriptor")
            .field("function_count", &self.function_count())
            .finish()
    }
}

// ============================================================================
// IntersectionFunctionTable
// ============================================================================

/// A table of intersection functions for ray tracing.
///
/// C++ equivalent: `MTL::IntersectionFunctionTable`
///
/// Inherits from Resource.
#[repr(transparent)]
pub struct IntersectionFunctionTable(pub(crate) NonNull<c_void>);

impl IntersectionFunctionTable {
    /// Create an IntersectionFunctionTable from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal intersection function table object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the function table.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the GPU resource ID for bindless access.
    ///
    /// C++ equivalent: `ResourceID gpuResourceID() const`
    #[inline]
    pub fn gpu_resource_id(&self) -> ResourceID {
        unsafe { msg_send_0(self.as_ptr(), sel!(gpuResourceID)) }
    }

    // =========================================================================
    // Buffer Methods
    // =========================================================================

    /// Set a buffer at the specified index.
    ///
    /// C++ equivalent: `void setBuffer(const MTL::Buffer* buffer, NS::UInteger offset, NS::UInteger index)`
    pub fn set_buffer(&self, buffer: &Buffer, offset: UInteger, index: UInteger) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(setBuffer:offset:atIndex:),
                buffer.as_ptr(),
                offset,
                index,
            );
        }
    }

    /// Set multiple buffers at a range of indices.
    ///
    /// C++ equivalent: `void setBuffers(const MTL::Buffer* const buffers[], const NS::UInteger offsets[], NS::Range range)`
    pub fn set_buffers(&self, buffers: &[&Buffer], offsets: &[UInteger], range: Range) {
        let ptrs: Vec<*const c_void> = buffers.iter().map(|b| b.as_ptr()).collect();
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(setBuffers:offsets:withRange:),
                ptrs.as_ptr(),
                offsets.as_ptr(),
                range,
            );
        }
    }

    // =========================================================================
    // Function Methods
    // =========================================================================

    /// Set a function at the specified index.
    ///
    /// C++ equivalent: `void setFunction(const MTL::FunctionHandle* function, NS::UInteger index)`
    pub fn set_function(&self, function: &FunctionHandle, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setFunction:atIndex:),
                function.as_ptr(),
                index,
            );
        }
    }

    /// Set multiple functions at a range of indices.
    ///
    /// C++ equivalent: `void setFunctions(const MTL::FunctionHandle* const functions[], NS::Range range)`
    pub fn set_functions(&self, functions: &[&FunctionHandle], range: Range) {
        let ptrs: Vec<*const c_void> = functions.iter().map(|f| f.as_ptr()).collect();
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setFunctions:withRange:),
                ptrs.as_ptr(),
                range,
            );
        }
    }

    // =========================================================================
    // Opaque Intersection Function Methods
    // =========================================================================

    /// Set an opaque curve intersection function at the specified index.
    ///
    /// C++ equivalent: `void setOpaqueCurveIntersectionFunction(MTL::IntersectionFunctionSignature signature, NS::UInteger index)`
    pub fn set_opaque_curve_intersection_function_at_index(
        &self,
        signature: IntersectionFunctionSignature,
        index: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setOpaqueCurveIntersectionFunctionWithSignature:atIndex:),
                signature,
                index,
            );
        }
    }

    /// Set an opaque curve intersection function for a range of indices.
    ///
    /// C++ equivalent: `void setOpaqueCurveIntersectionFunction(MTL::IntersectionFunctionSignature signature, NS::Range range)`
    pub fn set_opaque_curve_intersection_function_with_range(
        &self,
        signature: IntersectionFunctionSignature,
        range: Range,
    ) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setOpaqueCurveIntersectionFunctionWithSignature:withRange:),
                signature,
                range,
            );
        }
    }

    /// Set an opaque triangle intersection function at the specified index.
    ///
    /// C++ equivalent: `void setOpaqueTriangleIntersectionFunction(MTL::IntersectionFunctionSignature signature, NS::UInteger index)`
    pub fn set_opaque_triangle_intersection_function_at_index(
        &self,
        signature: IntersectionFunctionSignature,
        index: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setOpaqueTriangleIntersectionFunctionWithSignature:atIndex:),
                signature,
                index,
            );
        }
    }

    /// Set an opaque triangle intersection function for a range of indices.
    ///
    /// C++ equivalent: `void setOpaqueTriangleIntersectionFunction(MTL::IntersectionFunctionSignature signature, NS::Range range)`
    pub fn set_opaque_triangle_intersection_function_with_range(
        &self,
        signature: IntersectionFunctionSignature,
        range: Range,
    ) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setOpaqueTriangleIntersectionFunctionWithSignature:withRange:),
                signature,
                range,
            );
        }
    }

    // =========================================================================
    // Visible Function Table Methods
    // =========================================================================

    /// Set a visible function table at the specified buffer index.
    ///
    /// C++ equivalent: `void setVisibleFunctionTable(const MTL::VisibleFunctionTable* functionTable, NS::UInteger bufferIndex)`
    pub fn set_visible_function_table(
        &self,
        function_table: &VisibleFunctionTable,
        buffer_index: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setVisibleFunctionTable:atBufferIndex:),
                function_table.as_ptr(),
                buffer_index,
            );
        }
    }

    /// Set multiple visible function tables at a range of buffer indices.
    ///
    /// C++ equivalent: `void setVisibleFunctionTables(const MTL::VisibleFunctionTable* const functionTables[], NS::Range bufferRange)`
    pub fn set_visible_function_tables(
        &self,
        function_tables: &[&VisibleFunctionTable],
        buffer_range: Range,
    ) {
        let ptrs: Vec<*const c_void> = function_tables.iter().map(|t| t.as_ptr()).collect();
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setVisibleFunctionTables:withBufferRange:),
                ptrs.as_ptr(),
                buffer_range,
            );
        }
    }
}

impl Clone for IntersectionFunctionTable {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for IntersectionFunctionTable {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for IntersectionFunctionTable {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for IntersectionFunctionTable {}
unsafe impl Sync for IntersectionFunctionTable {}

impl std::fmt::Debug for IntersectionFunctionTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IntersectionFunctionTable").finish()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_function_buffer_arguments_size() {
        assert_eq!(
            std::mem::size_of::<IntersectionFunctionBufferArguments>(),
            24
        );
    }

    #[test]
    fn test_visible_function_table_descriptor_creation() {
        let desc = VisibleFunctionTableDescriptor::new();
        assert!(desc.is_some());
    }

    #[test]
    fn test_visible_function_table_descriptor_class_method() {
        let desc = VisibleFunctionTableDescriptor::visible_function_table_descriptor();
        assert!(desc.is_some());
    }

    #[test]
    fn test_visible_function_table_descriptor_function_count() {
        let desc = VisibleFunctionTableDescriptor::new().unwrap();
        // Default should be 0
        assert_eq!(desc.function_count(), 0);

        desc.set_function_count(10);
        assert_eq!(desc.function_count(), 10);
    }

    #[test]
    fn test_intersection_function_table_descriptor_creation() {
        let desc = IntersectionFunctionTableDescriptor::new();
        assert!(desc.is_some());
    }

    #[test]
    fn test_intersection_function_table_descriptor_class_method() {
        let desc = IntersectionFunctionTableDescriptor::intersection_function_table_descriptor();
        assert!(desc.is_some());
    }

    #[test]
    fn test_intersection_function_table_descriptor_function_count() {
        let desc = IntersectionFunctionTableDescriptor::new().unwrap();
        // Default should be 0
        assert_eq!(desc.function_count(), 0);

        desc.set_function_count(5);
        assert_eq!(desc.function_count(), 5);
    }
}
