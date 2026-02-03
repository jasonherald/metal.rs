//! Render pipeline state.
//!
//! Corresponds to `MTL::RenderPipelineState`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{RenderStages, ShaderValidation};
use crate::types::{ResourceID, Size};

use super::RenderPipelineFunctionsDescriptor;
use super::RenderPipelineReflection;

/// A compiled render pipeline configuration.
///
/// C++ equivalent: `MTL::RenderPipelineState`
#[repr(transparent)]
pub struct RenderPipelineState(pub(crate) NonNull<c_void>);

impl RenderPipelineState {
    /// Create a RenderPipelineState from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal render pipeline state object.
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
                metal_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
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

    /// Check if threadgroup size is multiple of thread execution width.
    ///
    /// C++ equivalent: `bool threadgroupSizeMatchesTileSize() const`
    #[inline]
    pub fn threadgroup_size_matches_tile_size(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(threadgroupSizeMatchesTileSize)) }
    }

    /// Get the imageblock sample length.
    ///
    /// C++ equivalent: `NS::UInteger imageblockSampleLength() const`
    #[inline]
    pub fn imageblock_sample_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(imageblockSampleLength)) }
    }

    /// Check if alpha-to-coverage is supported.
    ///
    /// C++ equivalent: `bool supportIndirectCommandBuffers() const`
    #[inline]
    pub fn support_indirect_command_buffers(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportIndirectCommandBuffers)) }
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

    // =========================================================================
    // Mesh/Object Shader Properties
    // =========================================================================

    /// Get the maximum total threadgroups per mesh grid.
    ///
    /// C++ equivalent: `NS::UInteger maxTotalThreadgroupsPerMeshGrid() const`
    #[inline]
    pub fn max_total_threadgroups_per_mesh_grid(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxTotalThreadgroupsPerMeshGrid)) }
    }

    /// Get the maximum total threads per mesh threadgroup.
    ///
    /// C++ equivalent: `NS::UInteger maxTotalThreadsPerMeshThreadgroup() const`
    #[inline]
    pub fn max_total_threads_per_mesh_threadgroup(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxTotalThreadsPerMeshThreadgroup)) }
    }

    /// Get the maximum total threads per object threadgroup.
    ///
    /// C++ equivalent: `NS::UInteger maxTotalThreadsPerObjectThreadgroup() const`
    #[inline]
    pub fn max_total_threads_per_object_threadgroup(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxTotalThreadsPerObjectThreadgroup)) }
    }

    /// Get the mesh thread execution width.
    ///
    /// C++ equivalent: `NS::UInteger meshThreadExecutionWidth() const`
    #[inline]
    pub fn mesh_thread_execution_width(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(meshThreadExecutionWidth)) }
    }

    /// Get the object thread execution width.
    ///
    /// C++ equivalent: `NS::UInteger objectThreadExecutionWidth() const`
    #[inline]
    pub fn object_thread_execution_width(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(objectThreadExecutionWidth)) }
    }

    /// Get the required threads per mesh threadgroup.
    ///
    /// C++ equivalent: `Size requiredThreadsPerMeshThreadgroup() const`
    #[inline]
    pub fn required_threads_per_mesh_threadgroup(&self) -> Size {
        unsafe { msg_send_0(self.as_ptr(), sel!(requiredThreadsPerMeshThreadgroup)) }
    }

    /// Get the required threads per object threadgroup.
    ///
    /// C++ equivalent: `Size requiredThreadsPerObjectThreadgroup() const`
    #[inline]
    pub fn required_threads_per_object_threadgroup(&self) -> Size {
        unsafe { msg_send_0(self.as_ptr(), sel!(requiredThreadsPerObjectThreadgroup)) }
    }

    /// Get the required threads per tile threadgroup.
    ///
    /// C++ equivalent: `Size requiredThreadsPerTileThreadgroup() const`
    #[inline]
    pub fn required_threads_per_tile_threadgroup(&self) -> Size {
        unsafe { msg_send_0(self.as_ptr(), sel!(requiredThreadsPerTileThreadgroup)) }
    }

    // =========================================================================
    // Function Handles
    // =========================================================================

    /// Get a function handle by name and stage.
    ///
    /// C++ equivalent: `FunctionHandle* functionHandle(const NS::String* name, MTL::RenderStages stage)`
    pub fn function_handle_with_name(
        &self,
        name: &str,
        stage: RenderStages,
    ) -> Option<crate::FunctionHandle> {
        let ns_name = metal_foundation::String::from_str(name)?;
        unsafe {
            let ptr: *mut c_void = metal_sys::msg_send_2(
                self.as_ptr(),
                sel!(functionHandleWithFunction: stage:),
                ns_name.as_ptr(),
                stage,
            );
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::FunctionHandle::from_raw(ptr)
        }
    }

    /// Get a function handle from a function and stage.
    ///
    /// C++ equivalent: `FunctionHandle* functionHandle(const MTL::Function* function, MTL::RenderStages stage)`
    pub fn function_handle_with_function(
        &self,
        function: &crate::Function,
        stage: RenderStages,
    ) -> Option<crate::FunctionHandle> {
        unsafe {
            let ptr: *mut c_void = metal_sys::msg_send_2(
                self.as_ptr(),
                sel!(functionHandleWithFunction: stage:),
                function.as_ptr(),
                stage,
            );
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::FunctionHandle::from_raw(ptr)
        }
    }

    // =========================================================================
    // Function Tables
    // =========================================================================

    /// Create a new intersection function table.
    ///
    /// C++ equivalent: `IntersectionFunctionTable* newIntersectionFunctionTable(const IntersectionFunctionTableDescriptor*, RenderStages)`
    pub fn new_intersection_function_table(
        &self,
        descriptor: &crate::IntersectionFunctionTableDescriptor,
        stage: RenderStages,
    ) -> Option<crate::IntersectionFunctionTable> {
        unsafe {
            let ptr: *mut c_void = metal_sys::msg_send_2(
                self.as_ptr(),
                sel!(newIntersectionFunctionTableWithDescriptor: stage:),
                descriptor.as_ptr(),
                stage,
            );
            if ptr.is_null() {
                None
            } else {
                crate::IntersectionFunctionTable::from_raw(ptr)
            }
        }
    }

    /// Create a new visible function table.
    ///
    /// C++ equivalent: `VisibleFunctionTable* newVisibleFunctionTable(const VisibleFunctionTableDescriptor*, RenderStages)`
    pub fn new_visible_function_table(
        &self,
        descriptor: &crate::VisibleFunctionTableDescriptor,
        stage: RenderStages,
    ) -> Option<crate::VisibleFunctionTable> {
        unsafe {
            let ptr: *mut c_void = metal_sys::msg_send_2(
                self.as_ptr(),
                sel!(newVisibleFunctionTableWithDescriptor: stage:),
                descriptor.as_ptr(),
                stage,
            );
            if ptr.is_null() {
                None
            } else {
                crate::VisibleFunctionTable::from_raw(ptr)
            }
        }
    }

    // =========================================================================
    // Pipeline State Creation
    // =========================================================================

    /// Create a new render pipeline state with additional binary functions.
    ///
    /// C++ equivalent: `RenderPipelineState* newRenderPipelineState(const RenderPipelineFunctionsDescriptor*, NS::Error**)`
    pub fn new_render_pipeline_state(
        &self,
        additional_functions: &RenderPipelineFunctionsDescriptor,
    ) -> Result<RenderPipelineState, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = metal_sys::msg_send_2(
                self.as_ptr(),
                sel!(newRenderPipelineStateWithAdditionalBinaryFunctions: error:),
                additional_functions.as_ptr(),
                &mut error as *mut _,
            );
            if ptr.is_null() {
                if !error.is_null() {
                    return Err(
                        metal_foundation::Error::from_ptr(error).expect("error should be valid")
                    );
                }
                return Err(metal_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error"));
            }
            Ok(RenderPipelineState::from_raw(ptr).expect("failed to create pipeline state"))
        }
    }

    // =========================================================================
    // Reflection
    // =========================================================================

    /// Get the pipeline reflection information.
    ///
    /// C++ equivalent: `RenderPipelineReflection* reflection() const`
    pub fn reflection(&self) -> Option<RenderPipelineReflection> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(reflection));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            RenderPipelineReflection::from_raw(ptr)
        }
    }
}

impl Clone for RenderPipelineState {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for RenderPipelineState {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for RenderPipelineState {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RenderPipelineState {}
unsafe impl Sync for RenderPipelineState {}

impl std::fmt::Debug for RenderPipelineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderPipelineState")
            .field("label", &self.label())
            .finish()
    }
}
