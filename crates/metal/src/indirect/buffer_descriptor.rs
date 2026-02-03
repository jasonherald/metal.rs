//! Descriptor for creating an indirect command buffer.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::IndirectCommandType;

/// Descriptor for creating an indirect command buffer.
///
/// C++ equivalent: `MTL::IndirectCommandBufferDescriptor`
#[repr(transparent)]
pub struct IndirectCommandBufferDescriptor(NonNull<c_void>);

impl IndirectCommandBufferDescriptor {
    /// Allocate a new indirect command buffer descriptor.
    ///
    /// C++ equivalent: `static IndirectCommandBufferDescriptor* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let cls = metal_sys::Class::get("MTLIndirectCommandBufferDescriptor")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize an allocated descriptor.
    ///
    /// C++ equivalent: `IndirectCommandBufferDescriptor* init()`
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new indirect command buffer descriptor.
    pub fn new() -> Option<Self> {
        Self::alloc()?.init()
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal indirect command buffer descriptor.
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
    // Command Types
    // =========================================================================

    /// Get the command types that can be encoded.
    ///
    /// C++ equivalent: `IndirectCommandType commandTypes() const`
    #[inline]
    pub fn command_types(&self) -> IndirectCommandType {
        unsafe { msg_send_0(self.as_ptr(), sel!(commandTypes)) }
    }

    /// Set the command types that can be encoded.
    ///
    /// C++ equivalent: `void setCommandTypes(IndirectCommandType)`
    #[inline]
    pub fn set_command_types(&self, types: IndirectCommandType) {
        unsafe {
            msg_send_1::<(), IndirectCommandType>(self.as_ptr(), sel!(setCommandTypes:), types);
        }
    }

    // =========================================================================
    // Inherit Properties
    // =========================================================================

    /// Get whether commands inherit buffers from the encoder.
    ///
    /// C++ equivalent: `bool inheritBuffers() const`
    #[inline]
    pub fn inherit_buffers(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(inheritBuffers)) }
    }

    /// Set whether commands inherit buffers from the encoder.
    ///
    /// C++ equivalent: `void setInheritBuffers(bool)`
    #[inline]
    pub fn set_inherit_buffers(&self, inherit: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setInheritBuffers:), inherit);
        }
    }

    /// Get whether commands inherit cull mode from the encoder.
    ///
    /// C++ equivalent: `bool inheritCullMode() const`
    #[inline]
    pub fn inherit_cull_mode(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(inheritCullMode)) }
    }

    /// Set whether commands inherit cull mode from the encoder.
    ///
    /// C++ equivalent: `void setInheritCullMode(bool)`
    #[inline]
    pub fn set_inherit_cull_mode(&self, inherit: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setInheritCullMode:), inherit);
        }
    }

    /// Get whether commands inherit depth bias from the encoder.
    ///
    /// C++ equivalent: `bool inheritDepthBias() const`
    #[inline]
    pub fn inherit_depth_bias(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(inheritDepthBias)) }
    }

    /// Set whether commands inherit depth bias from the encoder.
    ///
    /// C++ equivalent: `void setInheritDepthBias(bool)`
    #[inline]
    pub fn set_inherit_depth_bias(&self, inherit: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setInheritDepthBias:), inherit);
        }
    }

    /// Get whether commands inherit depth clip mode from the encoder.
    ///
    /// C++ equivalent: `bool inheritDepthClipMode() const`
    #[inline]
    pub fn inherit_depth_clip_mode(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(inheritDepthClipMode)) }
    }

    /// Set whether commands inherit depth clip mode from the encoder.
    ///
    /// C++ equivalent: `void setInheritDepthClipMode(bool)`
    #[inline]
    pub fn set_inherit_depth_clip_mode(&self, inherit: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setInheritDepthClipMode:), inherit);
        }
    }

    /// Get whether commands inherit depth stencil state from the encoder.
    ///
    /// C++ equivalent: `bool inheritDepthStencilState() const`
    #[inline]
    pub fn inherit_depth_stencil_state(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(inheritDepthStencilState)) }
    }

    /// Set whether commands inherit depth stencil state from the encoder.
    ///
    /// C++ equivalent: `void setInheritDepthStencilState(bool)`
    #[inline]
    pub fn set_inherit_depth_stencil_state(&self, inherit: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setInheritDepthStencilState:), inherit);
        }
    }

    /// Get whether commands inherit front facing winding from the encoder.
    ///
    /// C++ equivalent: `bool inheritFrontFacingWinding() const`
    #[inline]
    pub fn inherit_front_facing_winding(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(inheritFrontFacingWinding)) }
    }

    /// Set whether commands inherit front facing winding from the encoder.
    ///
    /// C++ equivalent: `void setInheritFrontFacingWinding(bool)`
    #[inline]
    pub fn set_inherit_front_facing_winding(&self, inherit: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setInheritFrontFacingWinding:), inherit);
        }
    }

    /// Get whether commands inherit pipeline state from the encoder.
    ///
    /// C++ equivalent: `bool inheritPipelineState() const`
    #[inline]
    pub fn inherit_pipeline_state(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(inheritPipelineState)) }
    }

    /// Set whether commands inherit pipeline state from the encoder.
    ///
    /// C++ equivalent: `void setInheritPipelineState(bool)`
    #[inline]
    pub fn set_inherit_pipeline_state(&self, inherit: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setInheritPipelineState:), inherit);
        }
    }

    /// Get whether commands inherit triangle fill mode from the encoder.
    ///
    /// C++ equivalent: `bool inheritTriangleFillMode() const`
    #[inline]
    pub fn inherit_triangle_fill_mode(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(inheritTriangleFillMode)) }
    }

    /// Set whether commands inherit triangle fill mode from the encoder.
    ///
    /// C++ equivalent: `void setInheritTriangleFillMode(bool)`
    #[inline]
    pub fn set_inherit_triangle_fill_mode(&self, inherit: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setInheritTriangleFillMode:), inherit);
        }
    }

    // =========================================================================
    // Buffer Bind Counts
    // =========================================================================

    /// Get the maximum number of vertex buffer bindings.
    ///
    /// C++ equivalent: `NS::UInteger maxVertexBufferBindCount() const`
    #[inline]
    pub fn max_vertex_buffer_bind_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxVertexBufferBindCount)) }
    }

    /// Set the maximum number of vertex buffer bindings.
    ///
    /// C++ equivalent: `void setMaxVertexBufferBindCount(NS::UInteger)`
    #[inline]
    pub fn set_max_vertex_buffer_bind_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxVertexBufferBindCount:), count);
        }
    }

    /// Get the maximum number of fragment buffer bindings.
    ///
    /// C++ equivalent: `NS::UInteger maxFragmentBufferBindCount() const`
    #[inline]
    pub fn max_fragment_buffer_bind_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxFragmentBufferBindCount)) }
    }

    /// Set the maximum number of fragment buffer bindings.
    ///
    /// C++ equivalent: `void setMaxFragmentBufferBindCount(NS::UInteger)`
    #[inline]
    pub fn set_max_fragment_buffer_bind_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxFragmentBufferBindCount:), count);
        }
    }

    /// Get the maximum number of kernel buffer bindings.
    ///
    /// C++ equivalent: `NS::UInteger maxKernelBufferBindCount() const`
    #[inline]
    pub fn max_kernel_buffer_bind_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxKernelBufferBindCount)) }
    }

    /// Set the maximum number of kernel buffer bindings.
    ///
    /// C++ equivalent: `void setMaxKernelBufferBindCount(NS::UInteger)`
    #[inline]
    pub fn set_max_kernel_buffer_bind_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxKernelBufferBindCount:), count);
        }
    }

    /// Get the maximum number of kernel threadgroup memory bindings.
    ///
    /// C++ equivalent: `NS::UInteger maxKernelThreadgroupMemoryBindCount() const`
    #[inline]
    pub fn max_kernel_threadgroup_memory_bind_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxKernelThreadgroupMemoryBindCount)) }
    }

    /// Set the maximum number of kernel threadgroup memory bindings.
    ///
    /// C++ equivalent: `void setMaxKernelThreadgroupMemoryBindCount(NS::UInteger)`
    #[inline]
    pub fn set_max_kernel_threadgroup_memory_bind_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(
                self.as_ptr(),
                sel!(setMaxKernelThreadgroupMemoryBindCount:),
                count,
            );
        }
    }

    /// Get the maximum number of mesh buffer bindings.
    ///
    /// C++ equivalent: `NS::UInteger maxMeshBufferBindCount() const`
    #[inline]
    pub fn max_mesh_buffer_bind_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxMeshBufferBindCount)) }
    }

    /// Set the maximum number of mesh buffer bindings.
    ///
    /// C++ equivalent: `void setMaxMeshBufferBindCount(NS::UInteger)`
    #[inline]
    pub fn set_max_mesh_buffer_bind_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxMeshBufferBindCount:), count);
        }
    }

    /// Get the maximum number of object buffer bindings.
    ///
    /// C++ equivalent: `NS::UInteger maxObjectBufferBindCount() const`
    #[inline]
    pub fn max_object_buffer_bind_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxObjectBufferBindCount)) }
    }

    /// Set the maximum number of object buffer bindings.
    ///
    /// C++ equivalent: `void setMaxObjectBufferBindCount(NS::UInteger)`
    #[inline]
    pub fn set_max_object_buffer_bind_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxObjectBufferBindCount:), count);
        }
    }

    /// Get the maximum number of object threadgroup memory bindings.
    ///
    /// C++ equivalent: `NS::UInteger maxObjectThreadgroupMemoryBindCount() const`
    #[inline]
    pub fn max_object_threadgroup_memory_bind_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxObjectThreadgroupMemoryBindCount)) }
    }

    /// Set the maximum number of object threadgroup memory bindings.
    ///
    /// C++ equivalent: `void setMaxObjectThreadgroupMemoryBindCount(NS::UInteger)`
    #[inline]
    pub fn set_max_object_threadgroup_memory_bind_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(
                self.as_ptr(),
                sel!(setMaxObjectThreadgroupMemoryBindCount:),
                count,
            );
        }
    }

    // =========================================================================
    // Support Flags
    // =========================================================================

    /// Get whether color attachment mapping is supported.
    ///
    /// C++ equivalent: `bool supportColorAttachmentMapping() const`
    #[inline]
    pub fn support_color_attachment_mapping(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportColorAttachmentMapping)) }
    }

    /// Set whether color attachment mapping is supported.
    ///
    /// C++ equivalent: `void setSupportColorAttachmentMapping(bool)`
    #[inline]
    pub fn set_support_color_attachment_mapping(&self, support: bool) {
        unsafe {
            msg_send_1::<(), bool>(
                self.as_ptr(),
                sel!(setSupportColorAttachmentMapping:),
                support,
            );
        }
    }

    /// Get whether dynamic attribute stride is supported.
    ///
    /// C++ equivalent: `bool supportDynamicAttributeStride() const`
    #[inline]
    pub fn support_dynamic_attribute_stride(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportDynamicAttributeStride)) }
    }

    /// Set whether dynamic attribute stride is supported.
    ///
    /// C++ equivalent: `void setSupportDynamicAttributeStride(bool)`
    #[inline]
    pub fn set_support_dynamic_attribute_stride(&self, support: bool) {
        unsafe {
            msg_send_1::<(), bool>(
                self.as_ptr(),
                sel!(setSupportDynamicAttributeStride:),
                support,
            );
        }
    }

    /// Get whether ray tracing is supported.
    ///
    /// C++ equivalent: `bool supportRayTracing() const`
    #[inline]
    pub fn support_ray_tracing(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportRayTracing)) }
    }

    /// Set whether ray tracing is supported.
    ///
    /// C++ equivalent: `void setSupportRayTracing(bool)`
    #[inline]
    pub fn set_support_ray_tracing(&self, support: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setSupportRayTracing:), support);
        }
    }
}

impl Default for IndirectCommandBufferDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create IndirectCommandBufferDescriptor")
    }
}

impl Clone for IndirectCommandBufferDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy IndirectCommandBufferDescriptor")
        }
    }
}

impl Drop for IndirectCommandBufferDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for IndirectCommandBufferDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for IndirectCommandBufferDescriptor {}
unsafe impl Sync for IndirectCommandBufferDescriptor {}

impl std::fmt::Debug for IndirectCommandBufferDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndirectCommandBufferDescriptor")
            .field("command_types", &self.command_types())
            .field(
                "max_vertex_buffer_bind_count",
                &self.max_vertex_buffer_bind_count(),
            )
            .finish()
    }
}
