//! MTL4 LinkingDescriptor implementation.
//!
//! Corresponds to `Metal/MTL4LinkingDescriptor.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

// ============================================================
// StaticLinkingDescriptor
// ============================================================

/// Descriptor for static shader linking.
///
/// C++ equivalent: `MTL4::StaticLinkingDescriptor`
///
/// StaticLinkingDescriptor specifies functions to be statically linked
/// with a pipeline.
#[repr(transparent)]
pub struct StaticLinkingDescriptor(NonNull<c_void>);

impl StaticLinkingDescriptor {
    /// Create a StaticLinkingDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new static linking descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTL4StaticLinkingDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the function descriptors (as raw pointer to NSArray).
    ///
    /// C++ equivalent: `NS::Array* functionDescriptors() const`
    pub fn function_descriptors_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(functionDescriptors)) }
    }

    /// Set the function descriptors (from raw pointer to NSArray).
    ///
    /// C++ equivalent: `void setFunctionDescriptors(const NS::Array*)`
    pub fn set_function_descriptors_raw(&self, descriptors: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setFunctionDescriptors:), descriptors);
        }
    }

    /// Get the groups dictionary (as raw pointer to NSDictionary).
    ///
    /// C++ equivalent: `NS::Dictionary* groups() const`
    pub fn groups_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(groups)) }
    }

    /// Set the groups dictionary (from raw pointer to NSDictionary).
    ///
    /// C++ equivalent: `void setGroups(const NS::Dictionary*)`
    pub fn set_groups_raw(&self, groups: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setGroups:), groups);
        }
    }

    /// Get the private function descriptors (as raw pointer to NSArray).
    ///
    /// C++ equivalent: `NS::Array* privateFunctionDescriptors() const`
    pub fn private_function_descriptors_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(privateFunctionDescriptors)) }
    }

    /// Set the private function descriptors (from raw pointer to NSArray).
    ///
    /// C++ equivalent: `void setPrivateFunctionDescriptors(const NS::Array*)`
    pub fn set_private_function_descriptors_raw(&self, descriptors: *const c_void) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setPrivateFunctionDescriptors:),
                descriptors,
            );
        }
    }
}

impl Clone for StaticLinkingDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for StaticLinkingDescriptor {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for StaticLinkingDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for StaticLinkingDescriptor {}
unsafe impl Sync for StaticLinkingDescriptor {}

impl std::fmt::Debug for StaticLinkingDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StaticLinkingDescriptor").finish()
    }
}

// ============================================================
// PipelineStageDynamicLinkingDescriptor
// ============================================================

/// Descriptor for pipeline stage dynamic linking.
///
/// C++ equivalent: `MTL4::PipelineStageDynamicLinkingDescriptor`
///
/// PipelineStageDynamicLinkingDescriptor specifies dynamic linking
/// configuration for a single pipeline stage.
#[repr(transparent)]
pub struct PipelineStageDynamicLinkingDescriptor(NonNull<c_void>);

impl PipelineStageDynamicLinkingDescriptor {
    /// Create a PipelineStageDynamicLinkingDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new pipeline stage dynamic linking descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTL4PipelineStageDynamicLinkingDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the binary linked functions (as raw pointer to NSArray).
    ///
    /// C++ equivalent: `NS::Array* binaryLinkedFunctions() const`
    pub fn binary_linked_functions_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(binaryLinkedFunctions)) }
    }

    /// Set the binary linked functions (from raw pointer to NSArray).
    ///
    /// C++ equivalent: `void setBinaryLinkedFunctions(const NS::Array*)`
    pub fn set_binary_linked_functions_raw(&self, functions: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setBinaryLinkedFunctions:), functions);
        }
    }

    /// Get the maximum call stack depth.
    ///
    /// C++ equivalent: `NS::UInteger maxCallStackDepth() const`
    pub fn max_call_stack_depth(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxCallStackDepth)) }
    }

    /// Set the maximum call stack depth.
    ///
    /// C++ equivalent: `void setMaxCallStackDepth(NS::UInteger)`
    pub fn set_max_call_stack_depth(&self, depth: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMaxCallStackDepth:), depth);
        }
    }

    /// Get the preloaded libraries (as raw pointer to NSArray).
    ///
    /// C++ equivalent: `NS::Array* preloadedLibraries() const`
    pub fn preloaded_libraries_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(preloadedLibraries)) }
    }

    /// Set the preloaded libraries (from raw pointer to NSArray).
    ///
    /// C++ equivalent: `void setPreloadedLibraries(const NS::Array*)`
    pub fn set_preloaded_libraries_raw(&self, libraries: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setPreloadedLibraries:), libraries);
        }
    }
}

impl Clone for PipelineStageDynamicLinkingDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for PipelineStageDynamicLinkingDescriptor {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for PipelineStageDynamicLinkingDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for PipelineStageDynamicLinkingDescriptor {}
unsafe impl Sync for PipelineStageDynamicLinkingDescriptor {}

impl std::fmt::Debug for PipelineStageDynamicLinkingDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipelineStageDynamicLinkingDescriptor")
            .field("max_call_stack_depth", &self.max_call_stack_depth())
            .finish()
    }
}

// ============================================================
// RenderPipelineDynamicLinkingDescriptor
// ============================================================

/// Descriptor for render pipeline dynamic linking.
///
/// C++ equivalent: `MTL4::RenderPipelineDynamicLinkingDescriptor`
///
/// RenderPipelineDynamicLinkingDescriptor provides access to dynamic
/// linking descriptors for each render pipeline stage.
#[repr(transparent)]
pub struct RenderPipelineDynamicLinkingDescriptor(NonNull<c_void>);

impl RenderPipelineDynamicLinkingDescriptor {
    /// Create a RenderPipelineDynamicLinkingDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new render pipeline dynamic linking descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTL4RenderPipelineDynamicLinkingDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the fragment linking descriptor.
    ///
    /// C++ equivalent: `PipelineStageDynamicLinkingDescriptor* fragmentLinkingDescriptor() const`
    pub fn fragment_linking_descriptor(&self) -> Option<PipelineStageDynamicLinkingDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(fragmentLinkingDescriptor));
            PipelineStageDynamicLinkingDescriptor::from_raw(ptr)
        }
    }

    /// Get the mesh linking descriptor.
    ///
    /// C++ equivalent: `PipelineStageDynamicLinkingDescriptor* meshLinkingDescriptor() const`
    pub fn mesh_linking_descriptor(&self) -> Option<PipelineStageDynamicLinkingDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(meshLinkingDescriptor));
            PipelineStageDynamicLinkingDescriptor::from_raw(ptr)
        }
    }

    /// Get the object linking descriptor.
    ///
    /// C++ equivalent: `PipelineStageDynamicLinkingDescriptor* objectLinkingDescriptor() const`
    pub fn object_linking_descriptor(&self) -> Option<PipelineStageDynamicLinkingDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(objectLinkingDescriptor));
            PipelineStageDynamicLinkingDescriptor::from_raw(ptr)
        }
    }

    /// Get the tile linking descriptor.
    ///
    /// C++ equivalent: `PipelineStageDynamicLinkingDescriptor* tileLinkingDescriptor() const`
    pub fn tile_linking_descriptor(&self) -> Option<PipelineStageDynamicLinkingDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(tileLinkingDescriptor));
            PipelineStageDynamicLinkingDescriptor::from_raw(ptr)
        }
    }

    /// Get the vertex linking descriptor.
    ///
    /// C++ equivalent: `PipelineStageDynamicLinkingDescriptor* vertexLinkingDescriptor() const`
    pub fn vertex_linking_descriptor(&self) -> Option<PipelineStageDynamicLinkingDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(vertexLinkingDescriptor));
            PipelineStageDynamicLinkingDescriptor::from_raw(ptr)
        }
    }
}

impl Clone for RenderPipelineDynamicLinkingDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for RenderPipelineDynamicLinkingDescriptor {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for RenderPipelineDynamicLinkingDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RenderPipelineDynamicLinkingDescriptor {}
unsafe impl Sync for RenderPipelineDynamicLinkingDescriptor {}

impl std::fmt::Debug for RenderPipelineDynamicLinkingDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderPipelineDynamicLinkingDescriptor")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_linking_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<StaticLinkingDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_pipeline_stage_dynamic_linking_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<PipelineStageDynamicLinkingDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_render_pipeline_dynamic_linking_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<RenderPipelineDynamicLinkingDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
