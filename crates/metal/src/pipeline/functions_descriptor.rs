//! Pipeline functions descriptors and utilities.
//!
//! This module contains:
//! - `RenderPipelineFunctionsDescriptor` for specifying additional shader functions
//! - `LogicalToPhysicalColorAttachmentMap` for remapping color attachments

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

pub struct RenderPipelineFunctionsDescriptor(pub(crate) NonNull<c_void>);

impl RenderPipelineFunctionsDescriptor {
    /// Create a new render pipeline functions descriptor.
    ///
    /// C++ equivalent: `static RenderPipelineFunctionsDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLRenderPipelineFunctionsDescriptor")?;
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
    /// The pointer must be a valid render pipeline functions descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Additional Binary Functions (Raw)
    // =========================================================================

    /// Get the vertex additional binary functions array (raw pointer).
    ///
    /// C++ equivalent: `NS::Array* vertexAdditionalBinaryFunctions() const`
    pub fn vertex_additional_binary_functions_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(vertexAdditionalBinaryFunctions)) }
    }

    /// Set the vertex additional binary functions array (raw pointer).
    ///
    /// C++ equivalent: `void setVertexAdditionalBinaryFunctions(const NS::Array*)`
    ///
    /// # Safety
    ///
    /// The pointer must be a valid NS::Array of BinaryFunction objects.
    pub unsafe fn set_vertex_additional_binary_functions_raw(&self, functions: *const c_void) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setVertexAdditionalBinaryFunctions:),
                functions,
            );
        }
    }

    /// Get the fragment additional binary functions array (raw pointer).
    ///
    /// C++ equivalent: `NS::Array* fragmentAdditionalBinaryFunctions() const`
    pub fn fragment_additional_binary_functions_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(fragmentAdditionalBinaryFunctions)) }
    }

    /// Set the fragment additional binary functions array (raw pointer).
    ///
    /// C++ equivalent: `void setFragmentAdditionalBinaryFunctions(const NS::Array*)`
    ///
    /// # Safety
    ///
    /// The pointer must be a valid NS::Array of BinaryFunction objects.
    pub unsafe fn set_fragment_additional_binary_functions_raw(&self, functions: *const c_void) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setFragmentAdditionalBinaryFunctions:),
                functions,
            );
        }
    }

    /// Get the tile additional binary functions array (raw pointer).
    ///
    /// C++ equivalent: `NS::Array* tileAdditionalBinaryFunctions() const`
    pub fn tile_additional_binary_functions_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(tileAdditionalBinaryFunctions)) }
    }

    /// Set the tile additional binary functions array (raw pointer).
    ///
    /// C++ equivalent: `void setTileAdditionalBinaryFunctions(const NS::Array*)`
    ///
    /// # Safety
    ///
    /// The pointer must be a valid NS::Array of BinaryFunction objects.
    pub unsafe fn set_tile_additional_binary_functions_raw(&self, functions: *const c_void) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setTileAdditionalBinaryFunctions:),
                functions,
            );
        }
    }
}

impl Default for RenderPipelineFunctionsDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create RenderPipelineFunctionsDescriptor")
    }
}

impl Clone for RenderPipelineFunctionsDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("copy returned null")
        }
    }
}

impl Drop for RenderPipelineFunctionsDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for RenderPipelineFunctionsDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RenderPipelineFunctionsDescriptor {}
unsafe impl Sync for RenderPipelineFunctionsDescriptor {}

impl std::fmt::Debug for RenderPipelineFunctionsDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderPipelineFunctionsDescriptor").finish()
    }
}

// ============================================================================
// LogicalToPhysicalColorAttachmentMap
// ============================================================================

/// Maps logical color attachment indices to physical indices.
///
/// C++ equivalent: `MTL::LogicalToPhysicalColorAttachmentMap`
#[repr(transparent)]
pub struct LogicalToPhysicalColorAttachmentMap(pub(crate) NonNull<c_void>);

impl LogicalToPhysicalColorAttachmentMap {
    /// Allocate a new logical to physical color attachment map.
    ///
    /// C++ equivalent: `static LogicalToPhysicalColorAttachmentMap* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let cls = metal_sys::Class::get("MTLLogicalToPhysicalColorAttachmentMap")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize an allocated map.
    ///
    /// C++ equivalent: `LogicalToPhysicalColorAttachmentMap* init()`
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new logical to physical color attachment map.
    pub fn new() -> Option<Self> {
        Self::alloc()?.init()
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal logical to physical color attachment map.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the physical index for a logical index.
    ///
    /// C++ equivalent: `NS::UInteger getPhysicalIndex(NS::UInteger logicalIndex)`
    #[inline]
    pub fn physical_index(&self, logical_index: UInteger) -> UInteger {
        unsafe { msg_send_1(self.as_ptr(), sel!(getPhysicalIndex:), logical_index) }
    }

    /// Set the physical index for a logical index.
    ///
    /// C++ equivalent: `void setPhysicalIndex(NS::UInteger physicalIndex, NS::UInteger logicalIndex)`
    #[inline]
    pub fn set_physical_index(&self, physical_index: UInteger, logical_index: UInteger) {
        unsafe {
            let _: () = metal_sys::msg_send_2(
                self.as_ptr(),
                sel!(setPhysicalIndex: forLogicalIndex:),
                physical_index,
                logical_index,
            );
        }
    }

    /// Reset the map to default values.
    ///
    /// C++ equivalent: `void reset()`
    #[inline]
    pub fn reset(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(reset));
        }
    }
}

impl Default for LogicalToPhysicalColorAttachmentMap {
    fn default() -> Self {
        Self::new().expect("failed to create LogicalToPhysicalColorAttachmentMap")
    }
}

impl Clone for LogicalToPhysicalColorAttachmentMap {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("copy returned null")
        }
    }
}

impl Drop for LogicalToPhysicalColorAttachmentMap {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for LogicalToPhysicalColorAttachmentMap {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for LogicalToPhysicalColorAttachmentMap {}
unsafe impl Sync for LogicalToPhysicalColorAttachmentMap {}

impl std::fmt::Debug for LogicalToPhysicalColorAttachmentMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogicalToPhysicalColorAttachmentMap").finish()
    }
}

