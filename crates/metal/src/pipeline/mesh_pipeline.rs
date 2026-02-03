//! Mesh render pipeline descriptor.
//!
//! Corresponds to `MTL::MeshRenderPipelineDescriptor`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{PixelFormat, ShaderValidation};
use crate::types::Size;

use super::{
    PipelineBufferDescriptorArray, RenderPipelineColorAttachmentDescriptorArray,
};

pub struct MeshRenderPipelineDescriptor(pub(crate) NonNull<c_void>);

impl MeshRenderPipelineDescriptor {
    /// Allocate a new mesh render pipeline descriptor.
    ///
    /// C++ equivalent: `static MeshRenderPipelineDescriptor* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let cls = metal_sys::Class::get("MTLMeshRenderPipelineDescriptor")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize an allocated descriptor.
    ///
    /// C++ equivalent: `MeshRenderPipelineDescriptor* init()`
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new mesh render pipeline descriptor.
    pub fn new() -> Option<Self> {
        Self::alloc()?.init()
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal mesh render pipeline descriptor.
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

    /// Set the label.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    // =========================================================================
    // Object Function
    // =========================================================================

    /// Get the object function.
    ///
    /// C++ equivalent: `Function* objectFunction() const`
    pub fn object_function(&self) -> Option<crate::Function> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(objectFunction));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            crate::Function::from_raw(ptr)
        }
    }

    /// Set the object function.
    ///
    /// C++ equivalent: `void setObjectFunction(const Function*)`
    pub fn set_object_function(&self, function: Option<&crate::Function>) {
        let ptr = function.map(|f| f.as_ptr()).unwrap_or(std::ptr::null());
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setObjectFunction:), ptr);
        }
    }

    /// Get the object buffers.
    ///
    /// C++ equivalent: `PipelineBufferDescriptorArray* objectBuffers() const`
    pub fn object_buffers(&self) -> Option<PipelineBufferDescriptorArray> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(objectBuffers));
            PipelineBufferDescriptorArray::from_raw(ptr)
        }
    }

    /// Get the object linked functions.
    ///
    /// C++ equivalent: `LinkedFunctions* objectLinkedFunctions() const`
    pub fn object_linked_functions(&self) -> Option<crate::LinkedFunctions> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(objectLinkedFunctions));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            crate::LinkedFunctions::from_raw(ptr)
        }
    }

    /// Set the object linked functions.
    ///
    /// C++ equivalent: `void setObjectLinkedFunctions(const LinkedFunctions*)`
    pub fn set_object_linked_functions(&self, functions: Option<&crate::LinkedFunctions>) {
        let ptr = functions.map(|f| f.as_ptr()).unwrap_or(std::ptr::null());
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setObjectLinkedFunctions:), ptr);
        }
    }

    // =========================================================================
    // Mesh Function
    // =========================================================================

    /// Get the mesh function.
    ///
    /// C++ equivalent: `Function* meshFunction() const`
    pub fn mesh_function(&self) -> Option<crate::Function> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(meshFunction));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            crate::Function::from_raw(ptr)
        }
    }

    /// Set the mesh function.
    ///
    /// C++ equivalent: `void setMeshFunction(const Function*)`
    pub fn set_mesh_function(&self, function: Option<&crate::Function>) {
        let ptr = function.map(|f| f.as_ptr()).unwrap_or(std::ptr::null());
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setMeshFunction:), ptr);
        }
    }

    /// Get the mesh buffers.
    ///
    /// C++ equivalent: `PipelineBufferDescriptorArray* meshBuffers() const`
    pub fn mesh_buffers(&self) -> Option<PipelineBufferDescriptorArray> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(meshBuffers));
            PipelineBufferDescriptorArray::from_raw(ptr)
        }
    }

    /// Get the mesh linked functions.
    ///
    /// C++ equivalent: `LinkedFunctions* meshLinkedFunctions() const`
    pub fn mesh_linked_functions(&self) -> Option<crate::LinkedFunctions> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(meshLinkedFunctions));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            crate::LinkedFunctions::from_raw(ptr)
        }
    }

    /// Set the mesh linked functions.
    ///
    /// C++ equivalent: `void setMeshLinkedFunctions(const LinkedFunctions*)`
    pub fn set_mesh_linked_functions(&self, functions: Option<&crate::LinkedFunctions>) {
        let ptr = functions.map(|f| f.as_ptr()).unwrap_or(std::ptr::null());
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setMeshLinkedFunctions:), ptr);
        }
    }

    // =========================================================================
    // Fragment Function
    // =========================================================================

    /// Get the fragment function.
    ///
    /// C++ equivalent: `Function* fragmentFunction() const`
    pub fn fragment_function(&self) -> Option<crate::Function> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(fragmentFunction));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            crate::Function::from_raw(ptr)
        }
    }

    /// Set the fragment function.
    ///
    /// C++ equivalent: `void setFragmentFunction(const Function*)`
    pub fn set_fragment_function(&self, function: Option<&crate::Function>) {
        let ptr = function.map(|f| f.as_ptr()).unwrap_or(std::ptr::null());
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setFragmentFunction:), ptr);
        }
    }

    /// Get the fragment buffers.
    ///
    /// C++ equivalent: `PipelineBufferDescriptorArray* fragmentBuffers() const`
    pub fn fragment_buffers(&self) -> Option<PipelineBufferDescriptorArray> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(fragmentBuffers));
            PipelineBufferDescriptorArray::from_raw(ptr)
        }
    }

    /// Get the fragment linked functions.
    ///
    /// C++ equivalent: `LinkedFunctions* fragmentLinkedFunctions() const`
    pub fn fragment_linked_functions(&self) -> Option<crate::LinkedFunctions> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(fragmentLinkedFunctions));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            crate::LinkedFunctions::from_raw(ptr)
        }
    }

    /// Set the fragment linked functions.
    ///
    /// C++ equivalent: `void setFragmentLinkedFunctions(const LinkedFunctions*)`
    pub fn set_fragment_linked_functions(&self, functions: Option<&crate::LinkedFunctions>) {
        let ptr = functions.map(|f| f.as_ptr()).unwrap_or(std::ptr::null());
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setFragmentLinkedFunctions:), ptr);
        }
    }

    // =========================================================================
    // Attachments
    // =========================================================================

    /// Get the color attachments.
    ///
    /// C++ equivalent: `RenderPipelineColorAttachmentDescriptorArray* colorAttachments() const`
    pub fn color_attachments(&self) -> Option<RenderPipelineColorAttachmentDescriptorArray> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(colorAttachments));
            RenderPipelineColorAttachmentDescriptorArray::from_raw(ptr)
        }
    }

    /// Get the depth attachment pixel format.
    ///
    /// C++ equivalent: `PixelFormat depthAttachmentPixelFormat() const`
    #[inline]
    pub fn depth_attachment_pixel_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(depthAttachmentPixelFormat)) }
    }

    /// Set the depth attachment pixel format.
    ///
    /// C++ equivalent: `void setDepthAttachmentPixelFormat(PixelFormat)`
    #[inline]
    pub fn set_depth_attachment_pixel_format(&self, format: PixelFormat) {
        unsafe {
            msg_send_1::<(), PixelFormat>(self.as_ptr(), sel!(setDepthAttachmentPixelFormat:), format);
        }
    }

    /// Get the stencil attachment pixel format.
    ///
    /// C++ equivalent: `PixelFormat stencilAttachmentPixelFormat() const`
    #[inline]
    pub fn stencil_attachment_pixel_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(stencilAttachmentPixelFormat)) }
    }

    /// Set the stencil attachment pixel format.
    ///
    /// C++ equivalent: `void setStencilAttachmentPixelFormat(PixelFormat)`
    #[inline]
    pub fn set_stencil_attachment_pixel_format(&self, format: PixelFormat) {
        unsafe {
            msg_send_1::<(), PixelFormat>(self.as_ptr(), sel!(setStencilAttachmentPixelFormat:), format);
        }
    }

    // =========================================================================
    // Raster Settings
    // =========================================================================

    /// Get the raster sample count.
    ///
    /// C++ equivalent: `NS::UInteger rasterSampleCount() const`
    #[inline]
    pub fn raster_sample_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(rasterSampleCount)) }
    }

    /// Set the raster sample count.
    ///
    /// C++ equivalent: `void setRasterSampleCount(NS::UInteger)`
    #[inline]
    pub fn set_raster_sample_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setRasterSampleCount:), count);
        }
    }

    /// Check if alpha to coverage is enabled.
    ///
    /// C++ equivalent: `bool isAlphaToCoverageEnabled() const`
    #[inline]
    pub fn is_alpha_to_coverage_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isAlphaToCoverageEnabled)) }
    }

    /// Set alpha to coverage enabled.
    ///
    /// C++ equivalent: `void setAlphaToCoverageEnabled(bool)`
    #[inline]
    pub fn set_alpha_to_coverage_enabled(&self, enabled: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setAlphaToCoverageEnabled:), enabled);
        }
    }

    /// Check if alpha to one is enabled.
    ///
    /// C++ equivalent: `bool isAlphaToOneEnabled() const`
    #[inline]
    pub fn is_alpha_to_one_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isAlphaToOneEnabled)) }
    }

    /// Set alpha to one enabled.
    ///
    /// C++ equivalent: `void setAlphaToOneEnabled(bool)`
    #[inline]
    pub fn set_alpha_to_one_enabled(&self, enabled: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setAlphaToOneEnabled:), enabled);
        }
    }

    /// Check if rasterization is enabled.
    ///
    /// C++ equivalent: `bool isRasterizationEnabled() const`
    #[inline]
    pub fn is_rasterization_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isRasterizationEnabled)) }
    }

    /// Set rasterization enabled.
    ///
    /// C++ equivalent: `void setRasterizationEnabled(bool)`
    #[inline]
    pub fn set_rasterization_enabled(&self, enabled: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setRasterizationEnabled:), enabled);
        }
    }

    // =========================================================================
    // Thread Settings
    // =========================================================================

    /// Get the max total threadgroups per mesh grid.
    ///
    /// C++ equivalent: `NS::UInteger maxTotalThreadgroupsPerMeshGrid() const`
    #[inline]
    pub fn max_total_threadgroups_per_mesh_grid(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxTotalThreadgroupsPerMeshGrid)) }
    }

    /// Set the max total threadgroups per mesh grid.
    ///
    /// C++ equivalent: `void setMaxTotalThreadgroupsPerMeshGrid(NS::UInteger)`
    #[inline]
    pub fn set_max_total_threadgroups_per_mesh_grid(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxTotalThreadgroupsPerMeshGrid:), count);
        }
    }

    /// Get the max total threads per object threadgroup.
    ///
    /// C++ equivalent: `NS::UInteger maxTotalThreadsPerObjectThreadgroup() const`
    #[inline]
    pub fn max_total_threads_per_object_threadgroup(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxTotalThreadsPerObjectThreadgroup)) }
    }

    /// Set the max total threads per object threadgroup.
    ///
    /// C++ equivalent: `void setMaxTotalThreadsPerObjectThreadgroup(NS::UInteger)`
    #[inline]
    pub fn set_max_total_threads_per_object_threadgroup(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxTotalThreadsPerObjectThreadgroup:), count);
        }
    }

    /// Get the max total threads per mesh threadgroup.
    ///
    /// C++ equivalent: `NS::UInteger maxTotalThreadsPerMeshThreadgroup() const`
    #[inline]
    pub fn max_total_threads_per_mesh_threadgroup(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxTotalThreadsPerMeshThreadgroup)) }
    }

    /// Set the max total threads per mesh threadgroup.
    ///
    /// C++ equivalent: `void setMaxTotalThreadsPerMeshThreadgroup(NS::UInteger)`
    #[inline]
    pub fn set_max_total_threads_per_mesh_threadgroup(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxTotalThreadsPerMeshThreadgroup:), count);
        }
    }

    /// Get whether object threadgroup size is multiple of thread execution width.
    ///
    /// C++ equivalent: `bool objectThreadgroupSizeIsMultipleOfThreadExecutionWidth() const`
    #[inline]
    pub fn object_threadgroup_size_is_multiple_of_thread_execution_width(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(objectThreadgroupSizeIsMultipleOfThreadExecutionWidth)) }
    }

    /// Set whether object threadgroup size is multiple of thread execution width.
    ///
    /// C++ equivalent: `void setObjectThreadgroupSizeIsMultipleOfThreadExecutionWidth(bool)`
    #[inline]
    pub fn set_object_threadgroup_size_is_multiple_of_thread_execution_width(&self, is_multiple: bool) {
        unsafe {
            msg_send_1::<(), bool>(
                self.as_ptr(),
                sel!(setObjectThreadgroupSizeIsMultipleOfThreadExecutionWidth:),
                is_multiple,
            );
        }
    }

    /// Get whether mesh threadgroup size is multiple of thread execution width.
    ///
    /// C++ equivalent: `bool meshThreadgroupSizeIsMultipleOfThreadExecutionWidth() const`
    #[inline]
    pub fn mesh_threadgroup_size_is_multiple_of_thread_execution_width(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(meshThreadgroupSizeIsMultipleOfThreadExecutionWidth)) }
    }

    /// Set whether mesh threadgroup size is multiple of thread execution width.
    ///
    /// C++ equivalent: `void setMeshThreadgroupSizeIsMultipleOfThreadExecutionWidth(bool)`
    #[inline]
    pub fn set_mesh_threadgroup_size_is_multiple_of_thread_execution_width(&self, is_multiple: bool) {
        unsafe {
            msg_send_1::<(), bool>(
                self.as_ptr(),
                sel!(setMeshThreadgroupSizeIsMultipleOfThreadExecutionWidth:),
                is_multiple,
            );
        }
    }

    /// Get the required threads per object threadgroup.
    ///
    /// C++ equivalent: `Size requiredThreadsPerObjectThreadgroup() const`
    #[inline]
    pub fn required_threads_per_object_threadgroup(&self) -> Size {
        unsafe { msg_send_0(self.as_ptr(), sel!(requiredThreadsPerObjectThreadgroup)) }
    }

    /// Set the required threads per object threadgroup.
    ///
    /// C++ equivalent: `void setRequiredThreadsPerObjectThreadgroup(Size)`
    #[inline]
    pub fn set_required_threads_per_object_threadgroup(&self, size: Size) {
        unsafe {
            msg_send_1::<(), Size>(self.as_ptr(), sel!(setRequiredThreadsPerObjectThreadgroup:), size);
        }
    }

    /// Get the required threads per mesh threadgroup.
    ///
    /// C++ equivalent: `Size requiredThreadsPerMeshThreadgroup() const`
    #[inline]
    pub fn required_threads_per_mesh_threadgroup(&self) -> Size {
        unsafe { msg_send_0(self.as_ptr(), sel!(requiredThreadsPerMeshThreadgroup)) }
    }

    /// Set the required threads per mesh threadgroup.
    ///
    /// C++ equivalent: `void setRequiredThreadsPerMeshThreadgroup(Size)`
    #[inline]
    pub fn set_required_threads_per_mesh_threadgroup(&self, size: Size) {
        unsafe {
            msg_send_1::<(), Size>(self.as_ptr(), sel!(setRequiredThreadsPerMeshThreadgroup:), size);
        }
    }

    /// Get the payload memory length.
    ///
    /// C++ equivalent: `NS::UInteger payloadMemoryLength() const`
    #[inline]
    pub fn payload_memory_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(payloadMemoryLength)) }
    }

    /// Set the payload memory length.
    ///
    /// C++ equivalent: `void setPayloadMemoryLength(NS::UInteger)`
    #[inline]
    pub fn set_payload_memory_length(&self, length: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setPayloadMemoryLength:), length);
        }
    }

    /// Get the max vertex amplification count.
    ///
    /// C++ equivalent: `NS::UInteger maxVertexAmplificationCount() const`
    #[inline]
    pub fn max_vertex_amplification_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxVertexAmplificationCount)) }
    }

    /// Set the max vertex amplification count.
    ///
    /// C++ equivalent: `void setMaxVertexAmplificationCount(NS::UInteger)`
    #[inline]
    pub fn set_max_vertex_amplification_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxVertexAmplificationCount:), count);
        }
    }

    // =========================================================================
    // Other Settings
    // =========================================================================

    /// Get whether indirect command buffers are supported.
    ///
    /// C++ equivalent: `bool supportIndirectCommandBuffers() const`
    #[inline]
    pub fn support_indirect_command_buffers(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportIndirectCommandBuffers)) }
    }

    /// Set whether indirect command buffers are supported.
    ///
    /// C++ equivalent: `void setSupportIndirectCommandBuffers(bool)`
    #[inline]
    pub fn set_support_indirect_command_buffers(&self, support: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setSupportIndirectCommandBuffers:), support);
        }
    }

    /// Get the shader validation mode.
    ///
    /// C++ equivalent: `ShaderValidation shaderValidation() const`
    #[inline]
    pub fn shader_validation(&self) -> ShaderValidation {
        unsafe { msg_send_0(self.as_ptr(), sel!(shaderValidation)) }
    }

    /// Set the shader validation mode.
    ///
    /// C++ equivalent: `void setShaderValidation(ShaderValidation)`
    #[inline]
    pub fn set_shader_validation(&self, validation: ShaderValidation) {
        unsafe {
            msg_send_1::<(), ShaderValidation>(self.as_ptr(), sel!(setShaderValidation:), validation);
        }
    }

    /// Get the binary archives (raw NSArray pointer).
    ///
    /// C++ equivalent: `NS::Array* binaryArchives() const`
    pub fn binary_archives_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(binaryArchives)) }
    }

    /// Set the binary archives.
    ///
    /// C++ equivalent: `void setBinaryArchives(const NS::Array*)`
    ///
    /// # Safety
    ///
    /// The pointer must be a valid NSArray of BinaryArchive objects.
    pub unsafe fn set_binary_archives_raw(&self, archives: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setBinaryArchives:), archives);
        }
    }

    /// Reset the descriptor to default values.
    ///
    /// C++ equivalent: `void reset()`
    #[inline]
    pub fn reset(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(reset));
        }
    }
}

impl Default for MeshRenderPipelineDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create MeshRenderPipelineDescriptor")
    }
}

impl Clone for MeshRenderPipelineDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("copy returned null")
        }
    }
}

impl Drop for MeshRenderPipelineDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
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
