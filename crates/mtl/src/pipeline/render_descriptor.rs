//! Render pipeline descriptor.
//!
//! Corresponds to `MTL::RenderPipelineDescriptor`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{
    PixelFormat, PrimitiveTopologyClass, ShaderValidation, TessellationControlPointIndexType,
    TessellationFactorFormat, TessellationFactorStepFunction, TessellationPartitionMode, Winding,
};

use super::{PipelineBufferDescriptorArray, RenderPipelineColorAttachmentDescriptorArray};

pub struct RenderPipelineDescriptor(pub(crate) NonNull<c_void>);

impl RenderPipelineDescriptor {
    /// Allocate a new render pipeline descriptor.
    ///
    /// C++ equivalent: `static RenderPipelineDescriptor* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let class = mtl_sys::class!(MTLRenderPipelineDescriptor);
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize the descriptor.
    ///
    /// C++ equivalent: `RenderPipelineDescriptor* init()`
    pub fn init(self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            // init consumes ownership - we must not release the old pointer
            // because ObjC init either returns the same pointer (still valid)
            // or releases the old and returns a new one
            std::mem::forget(self);
            Self::from_raw(ptr)
        }
    }

    /// Create a new render pipeline descriptor.
    pub fn new() -> Option<Self> {
        Self::alloc().and_then(|d| d.init())
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid render pipeline descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
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

    // =========================================================================
    // Basic Properties
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

    /// Set the label.
    ///
    /// C++ equivalent: `void setLabel(const NS::String* label)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = mtl_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    // =========================================================================
    // Shader Functions
    // =========================================================================

    /// Get the vertex function.
    ///
    /// C++ equivalent: `Function* vertexFunction() const`
    pub fn vertex_function(&self) -> Option<crate::Function> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(vertexFunction));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Function::from_raw(ptr)
        }
    }

    /// Set the vertex function.
    ///
    /// C++ equivalent: `void setVertexFunction(const MTL::Function* vertexFunction)`
    pub fn set_vertex_function(&self, function: Option<&crate::Function>) {
        unsafe {
            let ptr = function.map_or(std::ptr::null(), |f| f.as_ptr());
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setVertexFunction:), ptr);
        }
    }

    /// Get the fragment function.
    ///
    /// C++ equivalent: `Function* fragmentFunction() const`
    pub fn fragment_function(&self) -> Option<crate::Function> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(fragmentFunction));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Function::from_raw(ptr)
        }
    }

    /// Set the fragment function.
    ///
    /// C++ equivalent: `void setFragmentFunction(const MTL::Function* fragmentFunction)`
    pub fn set_fragment_function(&self, function: Option<&crate::Function>) {
        unsafe {
            let ptr = function.map_or(std::ptr::null(), |f| f.as_ptr());
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setFragmentFunction:), ptr);
        }
    }

    // =========================================================================
    // Color Attachments
    // =========================================================================

    /// Get the color attachments array.
    ///
    /// C++ equivalent: `RenderPipelineColorAttachmentDescriptorArray* colorAttachments() const`
    pub fn color_attachments(&self) -> RenderPipelineColorAttachmentDescriptorArray {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(colorAttachments));
            RenderPipelineColorAttachmentDescriptorArray::from_raw(ptr)
                .expect("colorAttachments returned null")
        }
    }

    // =========================================================================
    // Depth/Stencil Attachments
    // =========================================================================

    /// Get the depth attachment pixel format.
    ///
    /// C++ equivalent: `PixelFormat depthAttachmentPixelFormat() const`
    #[inline]
    pub fn depth_attachment_pixel_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(depthAttachmentPixelFormat)) }
    }

    /// Set the depth attachment pixel format.
    ///
    /// C++ equivalent: `void setDepthAttachmentPixelFormat(MTL::PixelFormat depthAttachmentPixelFormat)`
    #[inline]
    pub fn set_depth_attachment_pixel_format(&self, format: PixelFormat) {
        unsafe {
            msg_send_1::<(), PixelFormat>(
                self.as_ptr(),
                sel!(setDepthAttachmentPixelFormat:),
                format,
            );
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
    /// C++ equivalent: `void setStencilAttachmentPixelFormat(MTL::PixelFormat stencilAttachmentPixelFormat)`
    #[inline]
    pub fn set_stencil_attachment_pixel_format(&self, format: PixelFormat) {
        unsafe {
            msg_send_1::<(), PixelFormat>(
                self.as_ptr(),
                sel!(setStencilAttachmentPixelFormat:),
                format,
            );
        }
    }

    // =========================================================================
    // Rasterization
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
    /// C++ equivalent: `void setRasterSampleCount(NS::UInteger rasterSampleCount)`
    #[inline]
    pub fn set_raster_sample_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setRasterSampleCount:), count);
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
    /// C++ equivalent: `void setRasterizationEnabled(bool rasterizationEnabled)`
    #[inline]
    pub fn set_rasterization_enabled(&self, enabled: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setRasterizationEnabled:), enabled);
        }
    }

    /// Get the input primitive topology.
    ///
    /// C++ equivalent: `PrimitiveTopologyClass inputPrimitiveTopology() const`
    #[inline]
    pub fn input_primitive_topology(&self) -> PrimitiveTopologyClass {
        unsafe { msg_send_0(self.as_ptr(), sel!(inputPrimitiveTopology)) }
    }

    /// Set the input primitive topology.
    ///
    /// C++ equivalent: `void setInputPrimitiveTopology(MTL::PrimitiveTopologyClass inputPrimitiveTopology)`
    #[inline]
    pub fn set_input_primitive_topology(&self, topology: PrimitiveTopologyClass) {
        unsafe {
            msg_send_1::<(), PrimitiveTopologyClass>(
                self.as_ptr(),
                sel!(setInputPrimitiveTopology:),
                topology,
            );
        }
    }

    // =========================================================================
    // Alpha Coverage
    // =========================================================================

    /// Check if alpha-to-coverage is enabled.
    ///
    /// C++ equivalent: `bool isAlphaToCoverageEnabled() const`
    #[inline]
    pub fn is_alpha_to_coverage_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isAlphaToCoverageEnabled)) }
    }

    /// Set alpha-to-coverage enabled.
    ///
    /// C++ equivalent: `void setAlphaToCoverageEnabled(bool alphaToCoverageEnabled)`
    #[inline]
    pub fn set_alpha_to_coverage_enabled(&self, enabled: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setAlphaToCoverageEnabled:), enabled);
        }
    }

    /// Check if alpha-to-one is enabled.
    ///
    /// C++ equivalent: `bool isAlphaToOneEnabled() const`
    #[inline]
    pub fn is_alpha_to_one_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isAlphaToOneEnabled)) }
    }

    /// Set alpha-to-one enabled.
    ///
    /// C++ equivalent: `void setAlphaToOneEnabled(bool alphaToOneEnabled)`
    #[inline]
    pub fn set_alpha_to_one_enabled(&self, enabled: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setAlphaToOneEnabled:), enabled);
        }
    }

    // =========================================================================
    // Tessellation
    // =========================================================================

    /// Get the maximum tessellation factor.
    ///
    /// C++ equivalent: `NS::UInteger maxTessellationFactor() const`
    #[inline]
    pub fn max_tessellation_factor(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxTessellationFactor)) }
    }

    /// Set the maximum tessellation factor.
    ///
    /// C++ equivalent: `void setMaxTessellationFactor(NS::UInteger maxTessellationFactor)`
    #[inline]
    pub fn set_max_tessellation_factor(&self, factor: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxTessellationFactor:), factor);
        }
    }

    /// Check if tessellation factor scale is enabled.
    ///
    /// C++ equivalent: `bool isTessellationFactorScaleEnabled() const`
    #[inline]
    pub fn is_tessellation_factor_scale_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isTessellationFactorScaleEnabled)) }
    }

    /// Set tessellation factor scale enabled.
    ///
    /// C++ equivalent: `void setTessellationFactorScaleEnabled(bool tessellationFactorScaleEnabled)`
    #[inline]
    pub fn set_tessellation_factor_scale_enabled(&self, enabled: bool) {
        unsafe {
            msg_send_1::<(), bool>(
                self.as_ptr(),
                sel!(setTessellationFactorScaleEnabled:),
                enabled,
            );
        }
    }

    /// Get the tessellation factor format.
    ///
    /// C++ equivalent: `TessellationFactorFormat tessellationFactorFormat() const`
    #[inline]
    pub fn tessellation_factor_format(&self) -> TessellationFactorFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(tessellationFactorFormat)) }
    }

    /// Set the tessellation factor format.
    ///
    /// C++ equivalent: `void setTessellationFactorFormat(MTL::TessellationFactorFormat tessellationFactorFormat)`
    #[inline]
    pub fn set_tessellation_factor_format(&self, format: TessellationFactorFormat) {
        unsafe {
            msg_send_1::<(), TessellationFactorFormat>(
                self.as_ptr(),
                sel!(setTessellationFactorFormat:),
                format,
            );
        }
    }

    /// Get the tessellation control point index type.
    ///
    /// C++ equivalent: `TessellationControlPointIndexType tessellationControlPointIndexType() const`
    #[inline]
    pub fn tessellation_control_point_index_type(&self) -> TessellationControlPointIndexType {
        unsafe { msg_send_0(self.as_ptr(), sel!(tessellationControlPointIndexType)) }
    }

    /// Set the tessellation control point index type.
    ///
    /// C++ equivalent: `void setTessellationControlPointIndexType(MTL::TessellationControlPointIndexType tessellationControlPointIndexType)`
    #[inline]
    pub fn set_tessellation_control_point_index_type(
        &self,
        index_type: TessellationControlPointIndexType,
    ) {
        unsafe {
            msg_send_1::<(), TessellationControlPointIndexType>(
                self.as_ptr(),
                sel!(setTessellationControlPointIndexType:),
                index_type,
            );
        }
    }

    /// Get the tessellation factor step function.
    ///
    /// C++ equivalent: `TessellationFactorStepFunction tessellationFactorStepFunction() const`
    #[inline]
    pub fn tessellation_factor_step_function(&self) -> TessellationFactorStepFunction {
        unsafe { msg_send_0(self.as_ptr(), sel!(tessellationFactorStepFunction)) }
    }

    /// Set the tessellation factor step function.
    ///
    /// C++ equivalent: `void setTessellationFactorStepFunction(MTL::TessellationFactorStepFunction tessellationFactorStepFunction)`
    #[inline]
    pub fn set_tessellation_factor_step_function(&self, func: TessellationFactorStepFunction) {
        unsafe {
            msg_send_1::<(), TessellationFactorStepFunction>(
                self.as_ptr(),
                sel!(setTessellationFactorStepFunction:),
                func,
            );
        }
    }

    /// Get the tessellation output winding order.
    ///
    /// C++ equivalent: `Winding tessellationOutputWindingOrder() const`
    #[inline]
    pub fn tessellation_output_winding_order(&self) -> Winding {
        unsafe { msg_send_0(self.as_ptr(), sel!(tessellationOutputWindingOrder)) }
    }

    /// Set the tessellation output winding order.
    ///
    /// C++ equivalent: `void setTessellationOutputWindingOrder(MTL::Winding tessellationOutputWindingOrder)`
    #[inline]
    pub fn set_tessellation_output_winding_order(&self, winding: Winding) {
        unsafe {
            msg_send_1::<(), Winding>(
                self.as_ptr(),
                sel!(setTessellationOutputWindingOrder:),
                winding,
            );
        }
    }

    /// Get the tessellation partition mode.
    ///
    /// C++ equivalent: `TessellationPartitionMode tessellationPartitionMode() const`
    #[inline]
    pub fn tessellation_partition_mode(&self) -> TessellationPartitionMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(tessellationPartitionMode)) }
    }

    /// Set the tessellation partition mode.
    ///
    /// C++ equivalent: `void setTessellationPartitionMode(MTL::TessellationPartitionMode tessellationPartitionMode)`
    #[inline]
    pub fn set_tessellation_partition_mode(&self, mode: TessellationPartitionMode) {
        unsafe {
            msg_send_1::<(), TessellationPartitionMode>(
                self.as_ptr(),
                sel!(setTessellationPartitionMode:),
                mode,
            );
        }
    }

    // =========================================================================
    // Indirect Command Buffers
    // =========================================================================

    /// Check if the pipeline supports indirect command buffers.
    ///
    /// C++ equivalent: `bool supportIndirectCommandBuffers() const`
    #[inline]
    pub fn support_indirect_command_buffers(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportIndirectCommandBuffers)) }
    }

    /// Set indirect command buffer support.
    ///
    /// C++ equivalent: `void setSupportIndirectCommandBuffers(bool supportIndirectCommandBuffers)`
    #[inline]
    pub fn set_support_indirect_command_buffers(&self, support: bool) {
        unsafe {
            msg_send_1::<(), bool>(
                self.as_ptr(),
                sel!(setSupportIndirectCommandBuffers:),
                support,
            );
        }
    }

    // =========================================================================
    // Shader Validation
    // =========================================================================

    /// Get the shader validation mode.
    ///
    /// C++ equivalent: `ShaderValidation shaderValidation() const`
    #[inline]
    pub fn shader_validation(&self) -> ShaderValidation {
        unsafe { msg_send_0(self.as_ptr(), sel!(shaderValidation)) }
    }

    /// Set the shader validation mode.
    ///
    /// C++ equivalent: `void setShaderValidation(MTL::ShaderValidation shaderValidation)`
    #[inline]
    pub fn set_shader_validation(&self, validation: ShaderValidation) {
        unsafe {
            msg_send_1::<(), ShaderValidation>(
                self.as_ptr(),
                sel!(setShaderValidation:),
                validation,
            );
        }
    }

    // =========================================================================
    // Vertex Amplification
    // =========================================================================

    /// Get the maximum vertex amplification count.
    ///
    /// C++ equivalent: `NS::UInteger maxVertexAmplificationCount() const`
    #[inline]
    pub fn max_vertex_amplification_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxVertexAmplificationCount)) }
    }

    /// Set the maximum vertex amplification count.
    ///
    /// C++ equivalent: `void setMaxVertexAmplificationCount(NS::UInteger maxVertexAmplificationCount)`
    #[inline]
    pub fn set_max_vertex_amplification_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxVertexAmplificationCount:), count);
        }
    }

    // =========================================================================
    // Call Stack Depth
    // =========================================================================

    /// Get the maximum vertex call stack depth.
    ///
    /// C++ equivalent: `NS::UInteger maxVertexCallStackDepth() const`
    #[inline]
    pub fn max_vertex_call_stack_depth(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxVertexCallStackDepth)) }
    }

    /// Set the maximum vertex call stack depth.
    ///
    /// C++ equivalent: `void setMaxVertexCallStackDepth(NS::UInteger maxVertexCallStackDepth)`
    #[inline]
    pub fn set_max_vertex_call_stack_depth(&self, depth: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxVertexCallStackDepth:), depth);
        }
    }

    /// Get the maximum fragment call stack depth.
    ///
    /// C++ equivalent: `NS::UInteger maxFragmentCallStackDepth() const`
    #[inline]
    pub fn max_fragment_call_stack_depth(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxFragmentCallStackDepth)) }
    }

    /// Set the maximum fragment call stack depth.
    ///
    /// C++ equivalent: `void setMaxFragmentCallStackDepth(NS::UInteger maxFragmentCallStackDepth)`
    #[inline]
    pub fn set_max_fragment_call_stack_depth(&self, depth: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxFragmentCallStackDepth:), depth);
        }
    }

    // =========================================================================
    // Binary Function Support
    // =========================================================================

    /// Check if support adding vertex binary functions is enabled.
    ///
    /// C++ equivalent: `bool supportAddingVertexBinaryFunctions() const`
    #[inline]
    pub fn support_adding_vertex_binary_functions(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportAddingVertexBinaryFunctions)) }
    }

    /// Set support adding vertex binary functions.
    ///
    /// C++ equivalent: `void setSupportAddingVertexBinaryFunctions(bool supportAddingVertexBinaryFunctions)`
    #[inline]
    pub fn set_support_adding_vertex_binary_functions(&self, support: bool) {
        unsafe {
            msg_send_1::<(), bool>(
                self.as_ptr(),
                sel!(setSupportAddingVertexBinaryFunctions:),
                support,
            );
        }
    }

    /// Check if support adding fragment binary functions is enabled.
    ///
    /// C++ equivalent: `bool supportAddingFragmentBinaryFunctions() const`
    #[inline]
    pub fn support_adding_fragment_binary_functions(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportAddingFragmentBinaryFunctions)) }
    }

    /// Set support adding fragment binary functions.
    ///
    /// C++ equivalent: `void setSupportAddingFragmentBinaryFunctions(bool supportAddingFragmentBinaryFunctions)`
    #[inline]
    pub fn set_support_adding_fragment_binary_functions(&self, support: bool) {
        unsafe {
            msg_send_1::<(), bool>(
                self.as_ptr(),
                sel!(setSupportAddingFragmentBinaryFunctions:),
                support,
            );
        }
    }

    // =========================================================================
    // Sample Count (Legacy)
    // =========================================================================

    /// Get the sample count (deprecated - use rasterSampleCount instead).
    ///
    /// C++ equivalent: `NS::UInteger sampleCount() const`
    #[inline]
    #[deprecated(note = "use raster_sample_count() instead")]
    pub fn sample_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(sampleCount)) }
    }

    /// Set the sample count (deprecated - use set_raster_sample_count instead).
    ///
    /// C++ equivalent: `void setSampleCount(NS::UInteger sampleCount)`
    #[inline]
    #[deprecated(note = "use set_raster_sample_count() instead")]
    pub fn set_sample_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setSampleCount:), count);
        }
    }

    // =========================================================================
    // Vertex Descriptor
    // =========================================================================

    /// Get the vertex descriptor.
    ///
    /// C++ equivalent: `VertexDescriptor* vertexDescriptor() const`
    pub fn vertex_descriptor(&self) -> Option<crate::VertexDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(vertexDescriptor));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::VertexDescriptor::from_raw(ptr)
        }
    }

    /// Set the vertex descriptor.
    ///
    /// C++ equivalent: `void setVertexDescriptor(const MTL::VertexDescriptor* vertexDescriptor)`
    pub fn set_vertex_descriptor(&self, descriptor: Option<&crate::VertexDescriptor>) {
        unsafe {
            let ptr = descriptor.map_or(std::ptr::null(), |d| d.as_ptr());
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setVertexDescriptor:), ptr);
        }
    }

    // =========================================================================
    // Buffer Descriptors
    // =========================================================================

    /// Get the vertex buffer descriptors array.
    ///
    /// C++ equivalent: `PipelineBufferDescriptorArray* vertexBuffers() const`
    pub fn vertex_buffers(&self) -> Option<PipelineBufferDescriptorArray> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(vertexBuffers));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            PipelineBufferDescriptorArray::from_raw(ptr)
        }
    }

    /// Get the fragment buffer descriptors array.
    ///
    /// C++ equivalent: `PipelineBufferDescriptorArray* fragmentBuffers() const`
    pub fn fragment_buffers(&self) -> Option<PipelineBufferDescriptorArray> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(fragmentBuffers));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            PipelineBufferDescriptorArray::from_raw(ptr)
        }
    }

    // =========================================================================
    // Linked Functions
    // =========================================================================

    /// Get the vertex linked functions.
    ///
    /// C++ equivalent: `LinkedFunctions* vertexLinkedFunctions() const`
    pub fn vertex_linked_functions(&self) -> Option<crate::LinkedFunctions> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(vertexLinkedFunctions));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::LinkedFunctions::from_raw(ptr)
        }
    }

    /// Set the vertex linked functions.
    ///
    /// C++ equivalent: `void setVertexLinkedFunctions(const MTL::LinkedFunctions* vertexLinkedFunctions)`
    pub fn set_vertex_linked_functions(&self, functions: Option<&crate::LinkedFunctions>) {
        unsafe {
            let ptr = functions.map_or(std::ptr::null(), |f| f.as_ptr());
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setVertexLinkedFunctions:), ptr);
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
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::LinkedFunctions::from_raw(ptr)
        }
    }

    /// Set the fragment linked functions.
    ///
    /// C++ equivalent: `void setFragmentLinkedFunctions(const MTL::LinkedFunctions* fragmentLinkedFunctions)`
    pub fn set_fragment_linked_functions(&self, functions: Option<&crate::LinkedFunctions>) {
        unsafe {
            let ptr = functions.map_or(std::ptr::null(), |f| f.as_ptr());
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setFragmentLinkedFunctions:), ptr);
        }
    }

    // =========================================================================
    // Preloaded Libraries (Raw)
    // =========================================================================

    /// Get the vertex preloaded libraries array (raw pointer).
    ///
    /// C++ equivalent: `NS::Array* vertexPreloadedLibraries() const`
    pub fn vertex_preloaded_libraries_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(vertexPreloadedLibraries)) }
    }

    /// Set the vertex preloaded libraries array (raw pointer).
    ///
    /// C++ equivalent: `void setVertexPreloadedLibraries(const NS::Array* vertexPreloadedLibraries)`
    ///
    /// # Safety
    ///
    /// The pointer must be a valid NS::Array of Library objects.
    pub unsafe fn set_vertex_preloaded_libraries_raw(&self, libraries: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setVertexPreloadedLibraries:), libraries);
        }
    }

    /// Get the fragment preloaded libraries array (raw pointer).
    ///
    /// C++ equivalent: `NS::Array* fragmentPreloadedLibraries() const`
    pub fn fragment_preloaded_libraries_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(fragmentPreloadedLibraries)) }
    }

    /// Set the fragment preloaded libraries array (raw pointer).
    ///
    /// C++ equivalent: `void setFragmentPreloadedLibraries(const NS::Array* fragmentPreloadedLibraries)`
    ///
    /// # Safety
    ///
    /// The pointer must be a valid NS::Array of Library objects.
    pub unsafe fn set_fragment_preloaded_libraries_raw(&self, libraries: *const c_void) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setFragmentPreloadedLibraries:),
                libraries,
            );
        }
    }

    // =========================================================================
    // Binary Archives (Raw)
    // =========================================================================

    /// Get the binary archives array (raw pointer).
    ///
    /// C++ equivalent: `NS::Array* binaryArchives() const`
    pub fn binary_archives_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(binaryArchives)) }
    }

    /// Set the binary archives array (raw pointer).
    ///
    /// C++ equivalent: `void setBinaryArchives(const NS::Array* binaryArchives)`
    ///
    /// # Safety
    ///
    /// The pointer must be a valid NS::Array of BinaryArchive objects.
    pub unsafe fn set_binary_archives_raw(&self, archives: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setBinaryArchives:), archives);
        }
    }
}

impl Clone for RenderPipelineDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("copy returned null")
        }
    }
}

impl Drop for RenderPipelineDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Default for RenderPipelineDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create render pipeline descriptor")
    }
}

impl Referencing for RenderPipelineDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RenderPipelineDescriptor {}
unsafe impl Sync for RenderPipelineDescriptor {}

impl std::fmt::Debug for RenderPipelineDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderPipelineDescriptor")
            .field("label", &self.label())
            .field("raster_sample_count", &self.raster_sample_count())
            .finish()
    }
}
