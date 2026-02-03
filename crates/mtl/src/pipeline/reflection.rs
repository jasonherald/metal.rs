//! Pipeline reflection types.
//!
//! Corresponds to `MTL::ComputePipelineReflection` and `MTL::RenderPipelineReflection`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_0, sel};

pub struct ComputePipelineReflection(pub(crate) NonNull<c_void>);

impl ComputePipelineReflection {
    /// Create a new compute pipeline reflection.
    ///
    /// C++ equivalent: `static ComputePipelineReflection* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLComputePipelineReflection")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a ComputePipelineReflection from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal compute pipeline reflection object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the reflection.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the arguments array (deprecated, use bindings instead).
    ///
    /// Returns an array of MTLArgument objects.
    ///
    /// C++ equivalent: `NS::Array* arguments() const`
    pub fn arguments_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(arguments)) }
    }

    /// Get the bindings array.
    ///
    /// Returns an array of objects conforming to MTLBinding protocol.
    ///
    /// C++ equivalent: `NS::Array* bindings() const`
    pub fn bindings_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(bindings)) }
    }
}

impl Clone for ComputePipelineReflection {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ComputePipelineReflection {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for ComputePipelineReflection {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ComputePipelineReflection {}
unsafe impl Sync for ComputePipelineReflection {}

impl std::fmt::Debug for ComputePipelineReflection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComputePipelineReflection").finish()
    }
}

// ============================================================================
// RenderPipelineReflection
// ============================================================================

/// Reflection information for a render pipeline.
///
/// C++ equivalent: `MTL::RenderPipelineReflection`
#[repr(transparent)]
pub struct RenderPipelineReflection(pub(crate) NonNull<c_void>);

impl RenderPipelineReflection {
    /// Create a new render pipeline reflection.
    ///
    /// C++ equivalent: `static RenderPipelineReflection* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLRenderPipelineReflection")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a RenderPipelineReflection from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal render pipeline reflection object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the reflection.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Properties - Arguments (deprecated)
    // =========================================================================

    /// Get the vertex arguments array (deprecated, use vertex_bindings instead).
    ///
    /// C++ equivalent: `NS::Array* vertexArguments() const`
    pub fn vertex_arguments_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(vertexArguments)) }
    }

    /// Get the fragment arguments array (deprecated, use fragment_bindings instead).
    ///
    /// C++ equivalent: `NS::Array* fragmentArguments() const`
    pub fn fragment_arguments_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(fragmentArguments)) }
    }

    /// Get the tile arguments array (deprecated, use tile_bindings instead).
    ///
    /// C++ equivalent: `NS::Array* tileArguments() const`
    pub fn tile_arguments_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(tileArguments)) }
    }

    // =========================================================================
    // Properties - Bindings
    // =========================================================================

    /// Get the vertex bindings array.
    ///
    /// C++ equivalent: `NS::Array* vertexBindings() const`
    pub fn vertex_bindings_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(vertexBindings)) }
    }

    /// Get the fragment bindings array.
    ///
    /// C++ equivalent: `NS::Array* fragmentBindings() const`
    pub fn fragment_bindings_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(fragmentBindings)) }
    }

    /// Get the tile bindings array.
    ///
    /// C++ equivalent: `NS::Array* tileBindings() const`
    pub fn tile_bindings_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(tileBindings)) }
    }

    /// Get the object bindings array.
    ///
    /// C++ equivalent: `NS::Array* objectBindings() const`
    pub fn object_bindings_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(objectBindings)) }
    }

    /// Get the mesh bindings array.
    ///
    /// C++ equivalent: `NS::Array* meshBindings() const`
    pub fn mesh_bindings_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(meshBindings)) }
    }
}

impl Clone for RenderPipelineReflection {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for RenderPipelineReflection {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for RenderPipelineReflection {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RenderPipelineReflection {}
unsafe impl Sync for RenderPipelineReflection {}

impl std::fmt::Debug for RenderPipelineReflection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderPipelineReflection").finish()
    }
}
