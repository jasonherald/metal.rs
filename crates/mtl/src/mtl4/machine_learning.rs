//! MTL4 Machine Learning implementation.
//!
//! Corresponds to `Metal/MTL4MachineLearningPipeline.hpp` and
//! `Metal/MTL4MachineLearningCommandEncoder.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Integer, Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, msg_send_2, sel};

use super::{ArgumentTable, FunctionDescriptor};
use crate::{Device, Heap};

// ============================================================
// MachineLearningPipelineDescriptor
// ============================================================

/// Descriptor for creating a machine learning pipeline.
///
/// C++ equivalent: `MTL4::MachineLearningPipelineDescriptor`
#[repr(transparent)]
pub struct MachineLearningPipelineDescriptor(NonNull<c_void>);

impl MachineLearningPipelineDescriptor {
    /// Create a MachineLearningPipelineDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new machine learning pipeline descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTL4MachineLearningPipelineDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
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
        if let Some(ns_label) = mtl_foundation::String::from_str(label) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the machine learning function descriptor.
    ///
    /// C++ equivalent: `FunctionDescriptor* machineLearningFunctionDescriptor() const`
    pub fn machine_learning_function_descriptor(&self) -> Option<FunctionDescriptor> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_0(self.as_ptr(), sel!(machineLearningFunctionDescriptor));
            FunctionDescriptor::from_raw(ptr)
        }
    }

    /// Set the machine learning function descriptor.
    ///
    /// C++ equivalent: `void setMachineLearningFunctionDescriptor(const MTL4::FunctionDescriptor*)`
    pub fn set_machine_learning_function_descriptor(&self, descriptor: &FunctionDescriptor) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setMachineLearningFunctionDescriptor:),
                descriptor.as_ptr(),
            );
        }
    }

    /// Get input dimensions at buffer index (as raw pointer to TensorExtents).
    ///
    /// C++ equivalent: `MTL::TensorExtents* inputDimensionsAtBufferIndex(NS::Integer)`
    pub fn input_dimensions_at_buffer_index_raw(&self, buffer_index: Integer) -> *mut c_void {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(inputDimensionsAtBufferIndex:),
                buffer_index,
            )
        }
    }

    /// Set input dimensions at buffer index (from raw pointer to TensorExtents).
    ///
    /// C++ equivalent: `void setInputDimensions(const MTL::TensorExtents*, NS::Integer)`
    pub fn set_input_dimensions_raw(&self, dimensions: *const c_void, buffer_index: Integer) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setInputDimensions:atBufferIndex:),
                dimensions,
                buffer_index,
            );
        }
    }

    /// Reset the descriptor to its default state.
    ///
    /// C++ equivalent: `void reset()`
    pub fn reset(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(reset));
        }
    }
}

impl Default for MachineLearningPipelineDescriptor {
    fn default() -> Self {
        Self::new().expect("Failed to create MTL4MachineLearningPipelineDescriptor")
    }
}

impl Clone for MachineLearningPipelineDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for MachineLearningPipelineDescriptor {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for MachineLearningPipelineDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for MachineLearningPipelineDescriptor {}
unsafe impl Sync for MachineLearningPipelineDescriptor {}

impl std::fmt::Debug for MachineLearningPipelineDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MachineLearningPipelineDescriptor")
            .field("label", &self.label())
            .finish()
    }
}

// ============================================================
// MachineLearningPipelineReflection
// ============================================================

/// Reflection data for a machine learning pipeline.
///
/// C++ equivalent: `MTL4::MachineLearningPipelineReflection`
#[repr(transparent)]
pub struct MachineLearningPipelineReflection(NonNull<c_void>);

impl MachineLearningPipelineReflection {
    /// Create a MachineLearningPipelineReflection from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the bindings array (as raw pointer to NSArray).
    ///
    /// C++ equivalent: `NS::Array* bindings() const`
    pub fn bindings_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(bindings)) }
    }
}

impl Clone for MachineLearningPipelineReflection {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for MachineLearningPipelineReflection {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for MachineLearningPipelineReflection {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for MachineLearningPipelineReflection {}
unsafe impl Sync for MachineLearningPipelineReflection {}

impl std::fmt::Debug for MachineLearningPipelineReflection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MachineLearningPipelineReflection").finish()
    }
}

// ============================================================
// MachineLearningPipelineState
// ============================================================

/// A compiled machine learning pipeline state.
///
/// C++ equivalent: `MTL4::MachineLearningPipelineState`
#[repr(transparent)]
pub struct MachineLearningPipelineState(NonNull<c_void>);

impl MachineLearningPipelineState {
    /// Create a MachineLearningPipelineState from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the device.
    ///
    /// C++ equivalent: `MTL::Device* device() const`
    pub fn device(&self) -> Option<Device> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            Device::from_raw(ptr)
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

    /// Get the intermediates heap size.
    ///
    /// C++ equivalent: `NS::UInteger intermediatesHeapSize() const`
    pub fn intermediates_heap_size(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(intermediatesHeapSize)) }
    }

    /// Get the pipeline reflection.
    ///
    /// C++ equivalent: `MachineLearningPipelineReflection* reflection() const`
    pub fn reflection(&self) -> Option<MachineLearningPipelineReflection> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(reflection));
            MachineLearningPipelineReflection::from_raw(ptr)
        }
    }
}

impl Clone for MachineLearningPipelineState {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for MachineLearningPipelineState {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for MachineLearningPipelineState {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for MachineLearningPipelineState {}
unsafe impl Sync for MachineLearningPipelineState {}

impl std::fmt::Debug for MachineLearningPipelineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MachineLearningPipelineState")
            .field("label", &self.label())
            .field("intermediates_heap_size", &self.intermediates_heap_size())
            .finish()
    }
}

// ============================================================
// MachineLearningCommandEncoder
// ============================================================

/// A command encoder for machine learning operations.
///
/// C++ equivalent: `MTL4::MachineLearningCommandEncoder`
#[repr(transparent)]
pub struct MachineLearningCommandEncoder(NonNull<c_void>);

impl MachineLearningCommandEncoder {
    /// Create a MachineLearningCommandEncoder from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Set the pipeline state.
    ///
    /// C++ equivalent: `void setPipelineState(const MTL4::MachineLearningPipelineState*)`
    pub fn set_pipeline_state(&self, pipeline_state: &MachineLearningPipelineState) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setPipelineState:),
                pipeline_state.as_ptr(),
            );
        }
    }

    /// Set the argument table.
    ///
    /// C++ equivalent: `void setArgumentTable(const MTL4::ArgumentTable*)`
    pub fn set_argument_table(&self, argument_table: &ArgumentTable) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setArgumentTable:),
                argument_table.as_ptr(),
            );
        }
    }

    /// Dispatch the neural network.
    ///
    /// C++ equivalent: `void dispatchNetwork(const MTL::Heap*)`
    pub fn dispatch_network(&self, intermediates_heap: &Heap) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(dispatchNetworkWithIntermediatesHeap:),
                intermediates_heap.as_ptr(),
            );
        }
    }

    /// End encoding.
    ///
    /// C++ equivalent: `void endEncoding()`
    pub fn end_encoding(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(endEncoding));
        }
    }
}

impl Clone for MachineLearningCommandEncoder {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for MachineLearningCommandEncoder {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for MachineLearningCommandEncoder {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for MachineLearningCommandEncoder {}
unsafe impl Sync for MachineLearningCommandEncoder {}

impl std::fmt::Debug for MachineLearningCommandEncoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MachineLearningCommandEncoder").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_learning_pipeline_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<MachineLearningPipelineDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_machine_learning_pipeline_reflection_size() {
        assert_eq!(
            std::mem::size_of::<MachineLearningPipelineReflection>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_machine_learning_pipeline_state_size() {
        assert_eq!(
            std::mem::size_of::<MachineLearningPipelineState>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_machine_learning_command_encoder_size() {
        assert_eq!(
            std::mem::size_of::<MachineLearningCommandEncoder>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
