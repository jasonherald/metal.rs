//! MTL4 StitchedFunctionDescriptor implementation.
//!
//! Corresponds to `Metal/MTL4StitchedFunctionDescriptor.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::FunctionStitchingGraph;

// ============================================================
// StitchedFunctionDescriptor
// ============================================================

/// Descriptor for stitched functions.
///
/// C++ equivalent: `MTL4::StitchedFunctionDescriptor`
///
/// StitchedFunctionDescriptor extends FunctionDescriptor to support
/// function stitching with a graph-based composition model.
#[repr(transparent)]
pub struct StitchedFunctionDescriptor(NonNull<c_void>);

impl StitchedFunctionDescriptor {
    /// Create a StitchedFunctionDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new stitched function descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTL4StitchedFunctionDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the function graph.
    ///
    /// C++ equivalent: `MTL::FunctionStitchingGraph* functionGraph() const`
    pub fn function_graph(&self) -> Option<FunctionStitchingGraph> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(functionGraph));
            FunctionStitchingGraph::from_raw(ptr)
        }
    }

    /// Set the function graph.
    ///
    /// C++ equivalent: `void setFunctionGraph(const MTL::FunctionStitchingGraph*)`
    pub fn set_function_graph(&self, graph: &FunctionStitchingGraph) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setFunctionGraph:), graph.as_ptr());
        }
    }

    /// Get the function descriptors array (as raw pointer to NSArray).
    ///
    /// C++ equivalent: `NS::Array* functionDescriptors() const`
    pub fn function_descriptors_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(functionDescriptors)) }
    }

    /// Set the function descriptors array (from raw pointer to NSArray).
    ///
    /// C++ equivalent: `void setFunctionDescriptors(const NS::Array*)`
    pub fn set_function_descriptors_raw(&self, descriptors: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setFunctionDescriptors:), descriptors);
        }
    }
}

impl Default for StitchedFunctionDescriptor {
    fn default() -> Self {
        Self::new().expect("Failed to create MTL4StitchedFunctionDescriptor")
    }
}

impl Clone for StitchedFunctionDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for StitchedFunctionDescriptor {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for StitchedFunctionDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for StitchedFunctionDescriptor {}
unsafe impl Sync for StitchedFunctionDescriptor {}

impl std::fmt::Debug for StitchedFunctionDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StitchedFunctionDescriptor").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stitched_function_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<StitchedFunctionDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
