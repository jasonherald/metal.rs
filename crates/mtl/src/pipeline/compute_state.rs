//! Compute pipeline state.
//!
//! Corresponds to `MTL::ComputePipelineState`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, msg_send_2, sel};

use crate::enums::ShaderValidation;
use crate::types::{ResourceID, Size};

use super::ComputePipelineReflection;

/// A compiled compute pipeline configuration.
///
/// C++ equivalent: `MTL::ComputePipelineState`
#[repr(transparent)]
pub struct ComputePipelineState(pub(crate) NonNull<c_void>);

impl ComputePipelineState {
    /// Create a ComputePipelineState from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal compute pipeline state object.
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
    // Properties
    // =========================================================================

    /// Get the label.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                mtl_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Get the device.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("pipeline state has no device")
        }
    }

    /// Get the maximum total threads per threadgroup.
    ///
    /// C++ equivalent: `NS::UInteger maxTotalThreadsPerThreadgroup() const`
    #[inline]
    pub fn max_total_threads_per_threadgroup(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxTotalThreadsPerThreadgroup)) }
    }

    /// Get the thread execution width.
    ///
    /// C++ equivalent: `NS::UInteger threadExecutionWidth() const`
    #[inline]
    pub fn thread_execution_width(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(threadExecutionWidth)) }
    }

    /// Get the static threadgroup memory length.
    ///
    /// C++ equivalent: `NS::UInteger staticThreadgroupMemoryLength() const`
    #[inline]
    pub fn static_threadgroup_memory_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(staticThreadgroupMemoryLength)) }
    }

    /// Check if the pipeline supports indirect command buffers.
    ///
    /// C++ equivalent: `bool supportIndirectCommandBuffers() const`
    #[inline]
    pub fn support_indirect_command_buffers(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportIndirectCommandBuffers)) }
    }

    /// Get the imageblock memory length for given imageblock dimensions.
    ///
    /// C++ equivalent: `NS::UInteger imageblockMemoryLength(MTL::Size imageblockDimensions)`
    #[inline]
    pub fn imageblock_memory_length(&self, dimensions: Size) -> UInteger {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(imageblockMemoryLengthForDimensions:),
                dimensions,
            )
        }
    }

    /// Get the GPU resource ID for bindless access.
    ///
    /// C++ equivalent: `ResourceID gpuResourceID() const`
    #[inline]
    pub fn gpu_resource_id(&self) -> ResourceID {
        unsafe { msg_send_0(self.as_ptr(), sel!(gpuResourceID)) }
    }

    /// Get the shader validation mode.
    ///
    /// C++ equivalent: `ShaderValidation shaderValidation() const`
    #[inline]
    pub fn shader_validation(&self) -> ShaderValidation {
        unsafe { msg_send_0(self.as_ptr(), sel!(shaderValidation)) }
    }

    /// Get the required threads per threadgroup.
    ///
    /// C++ equivalent: `Size requiredThreadsPerThreadgroup() const`
    #[inline]
    pub fn required_threads_per_threadgroup(&self) -> Size {
        unsafe { msg_send_0(self.as_ptr(), sel!(requiredThreadsPerThreadgroup)) }
    }

    // =========================================================================
    // Function Handles
    // =========================================================================

    /// Get a function handle for a function by name.
    ///
    /// C++ equivalent: `FunctionHandle* functionHandle(const NS::String* name)`
    pub fn function_handle_with_name(&self, name: &str) -> Option<crate::FunctionHandle> {
        let ns_name = mtl_foundation::String::from_str(name)?;
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(functionHandleWithName:),
                ns_name.as_ptr(),
            );
            crate::FunctionHandle::from_raw(ptr)
        }
    }

    /// Get a function handle for a function.
    ///
    /// C++ equivalent: `FunctionHandle* functionHandle(const MTL::Function* function)`
    pub fn function_handle_with_function(
        &self,
        function: &crate::Function,
    ) -> Option<crate::FunctionHandle> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(functionHandleWithFunction:),
                function.as_ptr(),
            );
            crate::FunctionHandle::from_raw(ptr)
        }
    }

    /// Get a function handle for a binary function.
    ///
    /// C++ equivalent: `FunctionHandle* functionHandle(const MTL4::BinaryFunction* function)`
    pub fn function_handle_with_binary_function(
        &self,
        binary_function: *const c_void,
    ) -> Option<crate::FunctionHandle> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(functionHandleWithBinaryFunction:),
                binary_function,
            );
            crate::FunctionHandle::from_raw(ptr)
        }
    }

    // =========================================================================
    // Pipeline State Creation
    // =========================================================================

    /// Create a new compute pipeline state with additional functions.
    ///
    /// C++ equivalent: `ComputePipelineState* newComputePipelineState(const NS::Array* functions, NS::Error** error)`
    pub fn new_compute_pipeline_state_with_functions(
        &self,
        functions: *const c_void,
    ) -> Result<ComputePipelineState, mtl_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newComputePipelineStateWithFunctions:error:),
                functions,
                &mut error,
            );
            if ptr.is_null() {
                if !error.is_null() {
                    return Err(
                        mtl_foundation::Error::from_ptr(error).expect("error should be valid")
                    );
                }
                return Err(mtl_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error"));
            }
            Ok(ComputePipelineState::from_raw(ptr).unwrap())
        }
    }

    /// Create a new compute pipeline state with additional binary functions.
    ///
    /// C++ equivalent: `ComputePipelineState* newComputePipelineStateWithBinaryFunctions(const NS::Array* additionalBinaryFunctions, NS::Error** error)`
    pub fn new_compute_pipeline_state_with_binary_functions(
        &self,
        binary_functions: *const c_void,
    ) -> Result<ComputePipelineState, mtl_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newComputePipelineStateWithBinaryFunctions:error:),
                binary_functions,
                &mut error,
            );
            if ptr.is_null() {
                if !error.is_null() {
                    return Err(
                        mtl_foundation::Error::from_ptr(error).expect("error should be valid")
                    );
                }
                return Err(mtl_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error"));
            }
            Ok(ComputePipelineState::from_raw(ptr).unwrap())
        }
    }

    // =========================================================================
    // Function Tables
    // =========================================================================

    /// Create a new intersection function table.
    ///
    /// C++ equivalent: `IntersectionFunctionTable* newIntersectionFunctionTable(const MTL::IntersectionFunctionTableDescriptor* descriptor)`
    pub fn new_intersection_function_table(
        &self,
        descriptor: &crate::IntersectionFunctionTableDescriptor,
    ) -> Option<crate::IntersectionFunctionTable> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newIntersectionFunctionTableWithDescriptor:),
                descriptor.as_ptr(),
            );
            crate::IntersectionFunctionTable::from_raw(ptr)
        }
    }

    /// Create a new visible function table.
    ///
    /// C++ equivalent: `VisibleFunctionTable* newVisibleFunctionTable(const MTL::VisibleFunctionTableDescriptor* descriptor)`
    pub fn new_visible_function_table(
        &self,
        descriptor: &crate::VisibleFunctionTableDescriptor,
    ) -> Option<crate::VisibleFunctionTable> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newVisibleFunctionTableWithDescriptor:),
                descriptor.as_ptr(),
            );
            crate::VisibleFunctionTable::from_raw(ptr)
        }
    }

    // =========================================================================
    // Reflection
    // =========================================================================

    /// Get the pipeline reflection information.
    ///
    /// C++ equivalent: `ComputePipelineReflection* reflection()`
    pub fn reflection(&self) -> Option<ComputePipelineReflection> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(reflection));
            ComputePipelineReflection::from_raw(ptr)
        }
    }
}

impl Clone for ComputePipelineState {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ComputePipelineState {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for ComputePipelineState {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ComputePipelineState {}
unsafe impl Sync for ComputePipelineState {}

impl std::fmt::Debug for ComputePipelineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComputePipelineState")
            .field("label", &self.label())
            .field(
                "max_total_threads_per_threadgroup",
                &self.max_total_threads_per_threadgroup(),
            )
            .field("thread_execution_width", &self.thread_execution_width())
            .finish()
    }
}
