//! MTL4 RenderPipeline implementation.
//!
//! Corresponds to `Metal/MTL4RenderPipeline.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, sel};

use crate::{BlendFactor, BlendOperation, ColorWriteMask, PixelFormat, PrimitiveTopologyClass};
use super::enums::{
    AlphaToCoverageState, AlphaToOneState, BlendState, IndirectCommandBufferSupportState,
    LogicalToPhysicalColorAttachmentMappingState,
};
use super::{FunctionDescriptor, PipelineOptions, StaticLinkingDescriptor};

// ============================================================
// RenderPipelineColorAttachmentDescriptor
// ============================================================

/// Descriptor for a render pipeline color attachment.
///
/// C++ equivalent: `MTL4::RenderPipelineColorAttachmentDescriptor`
#[repr(transparent)]
pub struct RenderPipelineColorAttachmentDescriptor(NonNull<c_void>);

impl RenderPipelineColorAttachmentDescriptor {
    /// Create a RenderPipelineColorAttachmentDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new color attachment descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4RenderPipelineColorAttachmentDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the pixel format.
    ///
    /// C++ equivalent: `MTL::PixelFormat pixelFormat() const`
    pub fn pixel_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(pixelFormat)) }
    }

    /// Set the pixel format.
    ///
    /// C++ equivalent: `void setPixelFormat(MTL::PixelFormat)`
    pub fn set_pixel_format(&self, format: PixelFormat) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setPixelFormat:), format);
        }
    }

    /// Get the blending state.
    ///
    /// C++ equivalent: `BlendState blendingState() const`
    pub fn blending_state(&self) -> BlendState {
        unsafe { msg_send_0(self.as_ptr(), sel!(blendingState)) }
    }

    /// Set the blending state.
    ///
    /// C++ equivalent: `void setBlendingState(MTL4::BlendState)`
    pub fn set_blending_state(&self, state: BlendState) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setBlendingState:), state);
        }
    }

    /// Get the source RGB blend factor.
    ///
    /// C++ equivalent: `MTL::BlendFactor sourceRGBBlendFactor() const`
    pub fn source_rgb_blend_factor(&self) -> BlendFactor {
        unsafe { msg_send_0(self.as_ptr(), sel!(sourceRGBBlendFactor)) }
    }

    /// Set the source RGB blend factor.
    ///
    /// C++ equivalent: `void setSourceRGBBlendFactor(MTL::BlendFactor)`
    pub fn set_source_rgb_blend_factor(&self, factor: BlendFactor) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setSourceRGBBlendFactor:), factor);
        }
    }

    /// Get the destination RGB blend factor.
    ///
    /// C++ equivalent: `MTL::BlendFactor destinationRGBBlendFactor() const`
    pub fn destination_rgb_blend_factor(&self) -> BlendFactor {
        unsafe { msg_send_0(self.as_ptr(), sel!(destinationRGBBlendFactor)) }
    }

    /// Set the destination RGB blend factor.
    ///
    /// C++ equivalent: `void setDestinationRGBBlendFactor(MTL::BlendFactor)`
    pub fn set_destination_rgb_blend_factor(&self, factor: BlendFactor) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDestinationRGBBlendFactor:), factor);
        }
    }

    /// Get the RGB blend operation.
    ///
    /// C++ equivalent: `MTL::BlendOperation rgbBlendOperation() const`
    pub fn rgb_blend_operation(&self) -> BlendOperation {
        unsafe { msg_send_0(self.as_ptr(), sel!(rgbBlendOperation)) }
    }

    /// Set the RGB blend operation.
    ///
    /// C++ equivalent: `void setRgbBlendOperation(MTL::BlendOperation)`
    pub fn set_rgb_blend_operation(&self, operation: BlendOperation) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setRgbBlendOperation:), operation);
        }
    }

    /// Get the source alpha blend factor.
    ///
    /// C++ equivalent: `MTL::BlendFactor sourceAlphaBlendFactor() const`
    pub fn source_alpha_blend_factor(&self) -> BlendFactor {
        unsafe { msg_send_0(self.as_ptr(), sel!(sourceAlphaBlendFactor)) }
    }

    /// Set the source alpha blend factor.
    ///
    /// C++ equivalent: `void setSourceAlphaBlendFactor(MTL::BlendFactor)`
    pub fn set_source_alpha_blend_factor(&self, factor: BlendFactor) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setSourceAlphaBlendFactor:), factor);
        }
    }

    /// Get the destination alpha blend factor.
    ///
    /// C++ equivalent: `MTL::BlendFactor destinationAlphaBlendFactor() const`
    pub fn destination_alpha_blend_factor(&self) -> BlendFactor {
        unsafe { msg_send_0(self.as_ptr(), sel!(destinationAlphaBlendFactor)) }
    }

    /// Set the destination alpha blend factor.
    ///
    /// C++ equivalent: `void setDestinationAlphaBlendFactor(MTL::BlendFactor)`
    pub fn set_destination_alpha_blend_factor(&self, factor: BlendFactor) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDestinationAlphaBlendFactor:), factor);
        }
    }

    /// Get the alpha blend operation.
    ///
    /// C++ equivalent: `MTL::BlendOperation alphaBlendOperation() const`
    pub fn alpha_blend_operation(&self) -> BlendOperation {
        unsafe { msg_send_0(self.as_ptr(), sel!(alphaBlendOperation)) }
    }

    /// Set the alpha blend operation.
    ///
    /// C++ equivalent: `void setAlphaBlendOperation(MTL::BlendOperation)`
    pub fn set_alpha_blend_operation(&self, operation: BlendOperation) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setAlphaBlendOperation:), operation);
        }
    }

    /// Get the write mask.
    ///
    /// C++ equivalent: `MTL::ColorWriteMask writeMask() const`
    pub fn write_mask(&self) -> ColorWriteMask {
        unsafe { msg_send_0(self.as_ptr(), sel!(writeMask)) }
    }

    /// Set the write mask.
    ///
    /// C++ equivalent: `void setWriteMask(MTL::ColorWriteMask)`
    pub fn set_write_mask(&self, mask: ColorWriteMask) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setWriteMask:), mask);
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

impl Clone for RenderPipelineColorAttachmentDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for RenderPipelineColorAttachmentDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for RenderPipelineColorAttachmentDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RenderPipelineColorAttachmentDescriptor {}
unsafe impl Sync for RenderPipelineColorAttachmentDescriptor {}

// ============================================================
// RenderPipelineColorAttachmentDescriptorArray
// ============================================================

/// Array of render pipeline color attachment descriptors.
///
/// C++ equivalent: `MTL4::RenderPipelineColorAttachmentDescriptorArray`
#[repr(transparent)]
pub struct RenderPipelineColorAttachmentDescriptorArray(NonNull<c_void>);

impl RenderPipelineColorAttachmentDescriptorArray {
    /// Create a RenderPipelineColorAttachmentDescriptorArray from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new color attachment descriptor array.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4RenderPipelineColorAttachmentDescriptorArray")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the color attachment descriptor at the given index.
    ///
    /// C++ equivalent: `RenderPipelineColorAttachmentDescriptor* object(NS::UInteger)`
    pub fn object(&self, index: UInteger) -> Option<RenderPipelineColorAttachmentDescriptor> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(objectAtIndexedSubscript:), index);
            RenderPipelineColorAttachmentDescriptor::from_raw(ptr)
        }
    }

    /// Set the color attachment descriptor at the given index.
    ///
    /// C++ equivalent: `void setObject(const RenderPipelineColorAttachmentDescriptor*, NS::UInteger)`
    pub fn set_object(
        &self,
        attachment: &RenderPipelineColorAttachmentDescriptor,
        index: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setObject:atIndexedSubscript:),
                attachment.as_ptr(),
                index,
            );
        }
    }

    /// Reset the array to its default state.
    ///
    /// C++ equivalent: `void reset()`
    pub fn reset(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(reset));
        }
    }
}

impl Clone for RenderPipelineColorAttachmentDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for RenderPipelineColorAttachmentDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for RenderPipelineColorAttachmentDescriptorArray {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RenderPipelineColorAttachmentDescriptorArray {}
unsafe impl Sync for RenderPipelineColorAttachmentDescriptorArray {}

// ============================================================
// RenderPipelineBinaryFunctionsDescriptor
// ============================================================

/// Descriptor for render pipeline binary functions.
///
/// C++ equivalent: `MTL4::RenderPipelineBinaryFunctionsDescriptor`
#[repr(transparent)]
pub struct RenderPipelineBinaryFunctionsDescriptor(NonNull<c_void>);

impl RenderPipelineBinaryFunctionsDescriptor {
    /// Create a RenderPipelineBinaryFunctionsDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new binary functions descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4RenderPipelineBinaryFunctionsDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the vertex additional binary functions (as raw pointer to NSArray).
    pub fn vertex_additional_binary_functions_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(vertexAdditionalBinaryFunctions)) }
    }

    /// Set the vertex additional binary functions (from raw pointer to NSArray).
    pub fn set_vertex_additional_binary_functions_raw(&self, functions: *const c_void) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setVertexAdditionalBinaryFunctions:),
                functions,
            );
        }
    }

    /// Get the fragment additional binary functions (as raw pointer to NSArray).
    pub fn fragment_additional_binary_functions_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(fragmentAdditionalBinaryFunctions)) }
    }

    /// Set the fragment additional binary functions (from raw pointer to NSArray).
    pub fn set_fragment_additional_binary_functions_raw(&self, functions: *const c_void) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setFragmentAdditionalBinaryFunctions:),
                functions,
            );
        }
    }

    /// Get the tile additional binary functions (as raw pointer to NSArray).
    pub fn tile_additional_binary_functions_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(tileAdditionalBinaryFunctions)) }
    }

    /// Set the tile additional binary functions (from raw pointer to NSArray).
    pub fn set_tile_additional_binary_functions_raw(&self, functions: *const c_void) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setTileAdditionalBinaryFunctions:),
                functions,
            );
        }
    }

    /// Get the object additional binary functions (as raw pointer to NSArray).
    pub fn object_additional_binary_functions_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(objectAdditionalBinaryFunctions)) }
    }

    /// Set the object additional binary functions (from raw pointer to NSArray).
    pub fn set_object_additional_binary_functions_raw(&self, functions: *const c_void) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setObjectAdditionalBinaryFunctions:),
                functions,
            );
        }
    }

    /// Get the mesh additional binary functions (as raw pointer to NSArray).
    pub fn mesh_additional_binary_functions_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(meshAdditionalBinaryFunctions)) }
    }

    /// Set the mesh additional binary functions (from raw pointer to NSArray).
    pub fn set_mesh_additional_binary_functions_raw(&self, functions: *const c_void) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setMeshAdditionalBinaryFunctions:),
                functions,
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

impl Clone for RenderPipelineBinaryFunctionsDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for RenderPipelineBinaryFunctionsDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for RenderPipelineBinaryFunctionsDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RenderPipelineBinaryFunctionsDescriptor {}
unsafe impl Sync for RenderPipelineBinaryFunctionsDescriptor {}

// ============================================================
// RenderPipelineDescriptor
// ============================================================

/// Descriptor for MTL4 render pipelines.
///
/// C++ equivalent: `MTL4::RenderPipelineDescriptor`
#[repr(transparent)]
pub struct RenderPipelineDescriptor(NonNull<c_void>);

impl RenderPipelineDescriptor {
    /// Create a RenderPipelineDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new render pipeline descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4RenderPipelineDescriptor")?;
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

    // ========== Vertex Function ==========

    /// Get the vertex function descriptor.
    pub fn vertex_function_descriptor(&self) -> Option<FunctionDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(vertexFunctionDescriptor));
            FunctionDescriptor::from_raw(ptr)
        }
    }

    /// Set the vertex function descriptor.
    pub fn set_vertex_function_descriptor(&self, descriptor: &FunctionDescriptor) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setVertexFunctionDescriptor:),
                descriptor.as_ptr(),
            );
        }
    }

    /// Get the vertex static linking descriptor.
    pub fn vertex_static_linking_descriptor(&self) -> Option<StaticLinkingDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(vertexStaticLinkingDescriptor));
            StaticLinkingDescriptor::from_raw(ptr)
        }
    }

    /// Set the vertex static linking descriptor.
    pub fn set_vertex_static_linking_descriptor(&self, descriptor: &StaticLinkingDescriptor) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setVertexStaticLinkingDescriptor:),
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

    // ========== Vertex Descriptor ==========

    /// Get the vertex descriptor (as raw pointer).
    pub fn vertex_descriptor_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(vertexDescriptor)) }
    }

    /// Set the vertex descriptor (from raw pointer).
    pub fn set_vertex_descriptor_raw(&self, descriptor: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setVertexDescriptor:), descriptor);
        }
    }

    // ========== Primitive Topology ==========

    /// Get the input primitive topology.
    pub fn input_primitive_topology(&self) -> PrimitiveTopologyClass {
        unsafe { msg_send_0(self.as_ptr(), sel!(inputPrimitiveTopology)) }
    }

    /// Set the input primitive topology.
    pub fn set_input_primitive_topology(&self, topology: PrimitiveTopologyClass) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setInputPrimitiveTopology:), topology);
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

    /// Get whether vertex binary linking is supported.
    pub fn support_vertex_binary_linking(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportVertexBinaryLinking)) }
    }

    /// Set whether vertex binary linking is supported.
    pub fn set_support_vertex_binary_linking(&self, support: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setSupportVertexBinaryLinking:), support);
        }
    }

    /// Get whether fragment binary linking is supported.
    pub fn support_fragment_binary_linking(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportFragmentBinaryLinking)) }
    }

    /// Set whether fragment binary linking is supported.
    pub fn set_support_fragment_binary_linking(&self, support: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setSupportFragmentBinaryLinking:), support);
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
            let _: () = msg_send_1(self.as_ptr(), sel!(setSupportIndirectCommandBuffers:), state);
        }
    }

    /// Reset the descriptor to its default state.
    pub fn reset(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(reset));
        }
    }
}

impl Clone for RenderPipelineDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for RenderPipelineDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
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
            .field("is_rasterization_enabled", &self.is_rasterization_enabled())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_pipeline_color_attachment_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<RenderPipelineColorAttachmentDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_render_pipeline_color_attachment_descriptor_array_size() {
        assert_eq!(
            std::mem::size_of::<RenderPipelineColorAttachmentDescriptorArray>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_render_pipeline_binary_functions_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<RenderPipelineBinaryFunctionsDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_render_pipeline_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<RenderPipelineDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
