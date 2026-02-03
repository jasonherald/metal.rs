//! MTL4 MeshRenderPipeline implementation.
//!
//! Corresponds to `Metal/MTL4MeshRenderPipeline.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use super::enums::{
    AlphaToCoverageState, AlphaToOneState, IndirectCommandBufferSupportState,
    LogicalToPhysicalColorAttachmentMappingState,
};
use super::{
    FunctionDescriptor, PipelineOptions, RenderPipelineColorAttachmentDescriptorArray,
    StaticLinkingDescriptor,
};
use crate::Size;

// ============================================================
// MeshRenderPipelineDescriptor
// ============================================================

/// Descriptor for MTL4 mesh render pipelines.
///
/// C++ equivalent: `MTL4::MeshRenderPipelineDescriptor`
///
/// MeshRenderPipelineDescriptor configures mesh shading pipelines
/// with object, mesh, and fragment stages.
#[repr(transparent)]
pub struct MeshRenderPipelineDescriptor(NonNull<c_void>);

impl MeshRenderPipelineDescriptor {
    /// Create a MeshRenderPipelineDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new mesh render pipeline descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4MeshRenderPipelineDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    // ========== Base Pipeline Properties ==========

    /// Get the label.
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
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the pipeline options.
    pub fn options(&self) -> Option<PipelineOptions> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(options));
            PipelineOptions::from_raw(ptr)
        }
    }

    /// Set the pipeline options.
    pub fn set_options(&self, options: &PipelineOptions) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOptions:), options.as_ptr());
        }
    }

    // ========== Object Function ==========

    /// Get the object function descriptor.
    pub fn object_function_descriptor(&self) -> Option<FunctionDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(objectFunctionDescriptor));
            FunctionDescriptor::from_raw(ptr)
        }
    }

    /// Set the object function descriptor.
    pub fn set_object_function_descriptor(&self, descriptor: &FunctionDescriptor) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setObjectFunctionDescriptor:),
                descriptor.as_ptr(),
            );
        }
    }

    /// Get the object static linking descriptor.
    pub fn object_static_linking_descriptor(&self) -> Option<StaticLinkingDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(objectStaticLinkingDescriptor));
            StaticLinkingDescriptor::from_raw(ptr)
        }
    }

    /// Set the object static linking descriptor.
    pub fn set_object_static_linking_descriptor(&self, descriptor: &StaticLinkingDescriptor) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setObjectStaticLinkingDescriptor:),
                descriptor.as_ptr(),
            );
        }
    }

    // ========== Mesh Function ==========

    /// Get the mesh function descriptor.
    pub fn mesh_function_descriptor(&self) -> Option<FunctionDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(meshFunctionDescriptor));
            FunctionDescriptor::from_raw(ptr)
        }
    }

    /// Set the mesh function descriptor.
    pub fn set_mesh_function_descriptor(&self, descriptor: &FunctionDescriptor) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setMeshFunctionDescriptor:),
                descriptor.as_ptr(),
            );
        }
    }

    /// Get the mesh static linking descriptor.
    pub fn mesh_static_linking_descriptor(&self) -> Option<StaticLinkingDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(meshStaticLinkingDescriptor));
            StaticLinkingDescriptor::from_raw(ptr)
        }
    }

    /// Set the mesh static linking descriptor.
    pub fn set_mesh_static_linking_descriptor(&self, descriptor: &StaticLinkingDescriptor) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setMeshStaticLinkingDescriptor:),
                descriptor.as_ptr(),
            );
        }
    }

    // ========== Fragment Function ==========

    /// Get the fragment function descriptor.
    pub fn fragment_function_descriptor(&self) -> Option<FunctionDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(fragmentFunctionDescriptor));
            FunctionDescriptor::from_raw(ptr)
        }
    }

    /// Set the fragment function descriptor.
    pub fn set_fragment_function_descriptor(&self, descriptor: &FunctionDescriptor) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setFragmentFunctionDescriptor:),
                descriptor.as_ptr(),
            );
        }
    }

    /// Get the fragment static linking descriptor.
    pub fn fragment_static_linking_descriptor(&self) -> Option<StaticLinkingDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(fragmentStaticLinkingDescriptor));
            StaticLinkingDescriptor::from_raw(ptr)
        }
    }

    /// Set the fragment static linking descriptor.
    pub fn set_fragment_static_linking_descriptor(&self, descriptor: &StaticLinkingDescriptor) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setFragmentStaticLinkingDescriptor:),
                descriptor.as_ptr(),
            );
        }
    }

    // ========== Color Attachments ==========

    /// Get the color attachments array.
    pub fn color_attachments(&self) -> Option<RenderPipelineColorAttachmentDescriptorArray> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(colorAttachments));
            RenderPipelineColorAttachmentDescriptorArray::from_raw(ptr)
        }
    }

    /// Get the color attachment mapping state.
    pub fn color_attachment_mapping_state(&self) -> LogicalToPhysicalColorAttachmentMappingState {
        unsafe { msg_send_0(self.as_ptr(), sel!(colorAttachmentMappingState)) }
    }

    /// Set the color attachment mapping state.
    pub fn set_color_attachment_mapping_state(
        &self,
        state: LogicalToPhysicalColorAttachmentMappingState,
    ) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setColorAttachmentMappingState:), state);
        }
    }

    // ========== Rasterization ==========

    /// Get whether rasterization is enabled.
    pub fn is_rasterization_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isRasterizationEnabled)) }
    }

    /// Set whether rasterization is enabled.
    pub fn set_rasterization_enabled(&self, enabled: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setRasterizationEnabled:), enabled);
        }
    }

    /// Get the raster sample count.
    pub fn raster_sample_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(rasterSampleCount)) }
    }

    /// Set the raster sample count.
    pub fn set_raster_sample_count(&self, count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setRasterSampleCount:), count);
        }
    }

    // ========== Alpha ==========

    /// Get the alpha to coverage state.
    pub fn alpha_to_coverage_state(&self) -> AlphaToCoverageState {
        unsafe { msg_send_0(self.as_ptr(), sel!(alphaToCoverageState)) }
    }

    /// Set the alpha to coverage state.
    pub fn set_alpha_to_coverage_state(&self, state: AlphaToCoverageState) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setAlphaToCoverageState:), state);
        }
    }

    /// Get the alpha to one state.
    pub fn alpha_to_one_state(&self) -> AlphaToOneState {
        unsafe { msg_send_0(self.as_ptr(), sel!(alphaToOneState)) }
    }

    /// Set the alpha to one state.
    pub fn set_alpha_to_one_state(&self, state: AlphaToOneState) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setAlphaToOneState:), state);
        }
    }

    // ========== Object Threadgroup ==========

    /// Get the maximum total threads per object threadgroup.
    pub fn max_total_threads_per_object_threadgroup(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxTotalThreadsPerObjectThreadgroup)) }
    }

    /// Set the maximum total threads per object threadgroup.
    pub fn set_max_total_threads_per_object_threadgroup(&self, max: UInteger) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setMaxTotalThreadsPerObjectThreadgroup:),
                max,
            );
        }
    }

    /// Get the required threads per object threadgroup.
    pub fn required_threads_per_object_threadgroup(&self) -> Size {
        unsafe { msg_send_0(self.as_ptr(), sel!(requiredThreadsPerObjectThreadgroup)) }
    }

    /// Set the required threads per object threadgroup.
    pub fn set_required_threads_per_object_threadgroup(&self, size: Size) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setRequiredThreadsPerObjectThreadgroup:),
                size,
            );
        }
    }

    /// Get whether object threadgroup size is multiple of thread execution width.
    pub fn object_threadgroup_size_is_multiple_of_thread_execution_width(&self) -> bool {
        unsafe {
            msg_send_0(
                self.as_ptr(),
                sel!(objectThreadgroupSizeIsMultipleOfThreadExecutionWidth),
            )
        }
    }

    /// Set whether object threadgroup size is multiple of thread execution width.
    pub fn set_object_threadgroup_size_is_multiple_of_thread_execution_width(&self, value: bool) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setObjectThreadgroupSizeIsMultipleOfThreadExecutionWidth:),
                value,
            );
        }
    }

    // ========== Mesh Threadgroup ==========

    /// Get the maximum total threads per mesh threadgroup.
    pub fn max_total_threads_per_mesh_threadgroup(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxTotalThreadsPerMeshThreadgroup)) }
    }

    /// Set the maximum total threads per mesh threadgroup.
    pub fn set_max_total_threads_per_mesh_threadgroup(&self, max: UInteger) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setMaxTotalThreadsPerMeshThreadgroup:),
                max,
            );
        }
    }

    /// Get the required threads per mesh threadgroup.
    pub fn required_threads_per_mesh_threadgroup(&self) -> Size {
        unsafe { msg_send_0(self.as_ptr(), sel!(requiredThreadsPerMeshThreadgroup)) }
    }

    /// Set the required threads per mesh threadgroup.
    pub fn set_required_threads_per_mesh_threadgroup(&self, size: Size) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setRequiredThreadsPerMeshThreadgroup:),
                size,
            );
        }
    }

    /// Get whether mesh threadgroup size is multiple of thread execution width.
    pub fn mesh_threadgroup_size_is_multiple_of_thread_execution_width(&self) -> bool {
        unsafe {
            msg_send_0(
                self.as_ptr(),
                sel!(meshThreadgroupSizeIsMultipleOfThreadExecutionWidth),
            )
        }
    }

    /// Set whether mesh threadgroup size is multiple of thread execution width.
    pub fn set_mesh_threadgroup_size_is_multiple_of_thread_execution_width(&self, value: bool) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setMeshThreadgroupSizeIsMultipleOfThreadExecutionWidth:),
                value,
            );
        }
    }

    // ========== Mesh Grid ==========

    /// Get the maximum total threadgroups per mesh grid.
    pub fn max_total_threadgroups_per_mesh_grid(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxTotalThreadgroupsPerMeshGrid)) }
    }

    /// Set the maximum total threadgroups per mesh grid.
    pub fn set_max_total_threadgroups_per_mesh_grid(&self, max: UInteger) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setMaxTotalThreadgroupsPerMeshGrid:),
                max,
            );
        }
    }

    // ========== Payload ==========

    /// Get the payload memory length.
    pub fn payload_memory_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(payloadMemoryLength)) }
    }

    /// Set the payload memory length.
    pub fn set_payload_memory_length(&self, length: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setPayloadMemoryLength:), length);
        }
    }

    // ========== Vertex Amplification ==========

    /// Get the maximum vertex amplification count.
    pub fn max_vertex_amplification_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxVertexAmplificationCount)) }
    }

    /// Set the maximum vertex amplification count.
    pub fn set_max_vertex_amplification_count(&self, count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMaxVertexAmplificationCount:), count);
        }
    }

    // ========== Binary Linking ==========

    /// Get whether object binary linking is supported.
    pub fn support_object_binary_linking(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportObjectBinaryLinking)) }
    }

    /// Set whether object binary linking is supported.
    pub fn set_support_object_binary_linking(&self, support: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setSupportObjectBinaryLinking:), support);
        }
    }

    /// Get whether mesh binary linking is supported.
    pub fn support_mesh_binary_linking(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportMeshBinaryLinking)) }
    }

    /// Set whether mesh binary linking is supported.
    pub fn set_support_mesh_binary_linking(&self, support: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setSupportMeshBinaryLinking:), support);
        }
    }

    /// Get whether fragment binary linking is supported.
    pub fn support_fragment_binary_linking(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportFragmentBinaryLinking)) }
    }

    /// Set whether fragment binary linking is supported.
    pub fn set_support_fragment_binary_linking(&self, support: bool) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setSupportFragmentBinaryLinking:),
                support,
            );
        }
    }

    // ========== Indirect Command Buffers ==========

    /// Get the indirect command buffer support state.
    pub fn support_indirect_command_buffers(&self) -> IndirectCommandBufferSupportState {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportIndirectCommandBuffers)) }
    }

    /// Set the indirect command buffer support state.
    pub fn set_support_indirect_command_buffers(&self, state: IndirectCommandBufferSupportState) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setSupportIndirectCommandBuffers:),
                state,
            );
        }
    }

    /// Reset the descriptor to its default state.
    pub fn reset(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(reset));
        }
    }
}

impl Clone for MeshRenderPipelineDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for MeshRenderPipelineDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for MeshRenderPipelineDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for MeshRenderPipelineDescriptor {}
unsafe impl Sync for MeshRenderPipelineDescriptor {}

impl std::fmt::Debug for MeshRenderPipelineDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MeshRenderPipelineDescriptor")
            .field("label", &self.label())
            .field("raster_sample_count", &self.raster_sample_count())
            .field("is_rasterization_enabled", &self.is_rasterization_enabled())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_render_pipeline_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<MeshRenderPipelineDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
