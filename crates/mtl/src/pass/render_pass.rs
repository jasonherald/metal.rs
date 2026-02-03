//! Render pass descriptor.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::VisibilityResultType;
use crate::types::SamplePosition;

use super::{
    RenderPassColorAttachmentDescriptorArray, RenderPassDepthAttachmentDescriptor,
    RenderPassStencilAttachmentDescriptor,
};

/// A render pass descriptor that configures a render pass.
///
/// C++ equivalent: `MTL::RenderPassDescriptor`
#[repr(transparent)]
pub struct RenderPassDescriptor(NonNull<c_void>);

impl RenderPassDescriptor {
    /// Create a RenderPassDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal render pass descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new render pass descriptor.
    ///
    /// C++ equivalent: `RenderPassDescriptor::renderPassDescriptor()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::class!(MTLRenderPassDescriptor);
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(renderPassDescriptor));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Self::from_raw(ptr)
        }
    }

    // =========================================================================
    // Color Attachments
    // =========================================================================

    /// Get the array of color attachment descriptors.
    ///
    /// C++ equivalent: `RenderPassColorAttachmentDescriptorArray* colorAttachments() const`
    pub fn color_attachments(&self) -> Option<RenderPassColorAttachmentDescriptorArray> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(colorAttachments));
            RenderPassColorAttachmentDescriptorArray::from_raw(ptr)
        }
    }

    // =========================================================================
    // Depth Attachment
    // =========================================================================

    /// Get the depth attachment descriptor.
    ///
    /// C++ equivalent: `RenderPassDepthAttachmentDescriptor* depthAttachment() const`
    pub fn depth_attachment(&self) -> Option<RenderPassDepthAttachmentDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(depthAttachment));
            RenderPassDepthAttachmentDescriptor::from_raw(ptr)
        }
    }

    /// Set the depth attachment descriptor.
    ///
    /// C++ equivalent: `void setDepthAttachment(const RenderPassDepthAttachmentDescriptor*)`
    pub fn set_depth_attachment(&self, attachment: Option<&RenderPassDepthAttachmentDescriptor>) {
        unsafe {
            let ptr = attachment.map_or(std::ptr::null(), |a| a.as_ptr());
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setDepthAttachment:), ptr);
        }
    }

    // =========================================================================
    // Stencil Attachment
    // =========================================================================

    /// Get the stencil attachment descriptor.
    ///
    /// C++ equivalent: `RenderPassStencilAttachmentDescriptor* stencilAttachment() const`
    pub fn stencil_attachment(&self) -> Option<RenderPassStencilAttachmentDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(stencilAttachment));
            RenderPassStencilAttachmentDescriptor::from_raw(ptr)
        }
    }

    /// Set the stencil attachment descriptor.
    ///
    /// C++ equivalent: `void setStencilAttachment(const RenderPassStencilAttachmentDescriptor*)`
    pub fn set_stencil_attachment(
        &self,
        attachment: Option<&RenderPassStencilAttachmentDescriptor>,
    ) {
        unsafe {
            let ptr = attachment.map_or(std::ptr::null(), |a| a.as_ptr());
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setStencilAttachment:), ptr);
        }
    }

    // =========================================================================
    // Visibility Result Buffer
    // =========================================================================

    /// Get the visibility result buffer.
    ///
    /// C++ equivalent: `Buffer* visibilityResultBuffer() const`
    pub fn visibility_result_buffer(&self) -> Option<crate::Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(visibilityResultBuffer));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Buffer::from_raw(ptr)
        }
    }

    /// Set the visibility result buffer.
    ///
    /// C++ equivalent: `void setVisibilityResultBuffer(const Buffer*)`
    pub fn set_visibility_result_buffer(&self, buffer: Option<&crate::Buffer>) {
        unsafe {
            let ptr = buffer.map_or(std::ptr::null(), |b| b.as_ptr());
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setVisibilityResultBuffer:), ptr);
        }
    }

    /// Get the visibility result type.
    ///
    /// C++ equivalent: `VisibilityResultType visibilityResultType() const`
    #[inline]
    pub fn visibility_result_type(&self) -> VisibilityResultType {
        unsafe { msg_send_0(self.as_ptr(), sel!(visibilityResultType)) }
    }

    /// Set the visibility result type.
    ///
    /// C++ equivalent: `void setVisibilityResultType(VisibilityResultType)`
    #[inline]
    pub fn set_visibility_result_type(&self, result_type: VisibilityResultType) {
        unsafe {
            msg_send_1::<(), VisibilityResultType>(
                self.as_ptr(),
                sel!(setVisibilityResultType:),
                result_type,
            );
        }
    }

    // =========================================================================
    // Render Target Properties
    // =========================================================================

    /// Get the render target width.
    ///
    /// C++ equivalent: `NS::UInteger renderTargetWidth() const`
    #[inline]
    pub fn render_target_width(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(renderTargetWidth)) }
    }

    /// Set the render target width.
    ///
    /// C++ equivalent: `void setRenderTargetWidth(NS::UInteger)`
    #[inline]
    pub fn set_render_target_width(&self, width: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setRenderTargetWidth:), width);
        }
    }

    /// Get the render target height.
    ///
    /// C++ equivalent: `NS::UInteger renderTargetHeight() const`
    #[inline]
    pub fn render_target_height(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(renderTargetHeight)) }
    }

    /// Set the render target height.
    ///
    /// C++ equivalent: `void setRenderTargetHeight(NS::UInteger)`
    #[inline]
    pub fn set_render_target_height(&self, height: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setRenderTargetHeight:), height);
        }
    }

    /// Get the render target array length.
    ///
    /// C++ equivalent: `NS::UInteger renderTargetArrayLength() const`
    #[inline]
    pub fn render_target_array_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(renderTargetArrayLength)) }
    }

    /// Set the render target array length.
    ///
    /// C++ equivalent: `void setRenderTargetArrayLength(NS::UInteger)`
    #[inline]
    pub fn set_render_target_array_length(&self, length: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setRenderTargetArrayLength:), length);
        }
    }

    // =========================================================================
    // Sample Count and Positions
    // =========================================================================

    /// Get the default raster sample count.
    ///
    /// C++ equivalent: `NS::UInteger defaultRasterSampleCount() const`
    #[inline]
    pub fn default_raster_sample_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(defaultRasterSampleCount)) }
    }

    /// Set the default raster sample count.
    ///
    /// C++ equivalent: `void setDefaultRasterSampleCount(NS::UInteger)`
    #[inline]
    pub fn set_default_raster_sample_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setDefaultRasterSampleCount:), count);
        }
    }

    /// Set the sample positions.
    ///
    /// C++ equivalent: `void setSamplePositions(const SamplePosition*, NS::UInteger)`
    pub fn set_sample_positions(&self, positions: &[SamplePosition]) {
        unsafe {
            mtl_sys::msg_send_2::<(), *const SamplePosition, UInteger>(
                self.as_ptr(),
                sel!(setSamplePositions: count:),
                positions.as_ptr(),
                positions.len() as UInteger,
            );
        }
    }

    /// Get the sample positions.
    ///
    /// C++ equivalent: `NS::UInteger getSamplePositions(SamplePosition*, NS::UInteger)`
    pub fn get_sample_positions(&self, positions: &mut [SamplePosition]) -> UInteger {
        unsafe {
            mtl_sys::msg_send_2(
                self.as_ptr(),
                sel!(getSamplePositions: count:),
                positions.as_mut_ptr(),
                positions.len() as UInteger,
            )
        }
    }

    // =========================================================================
    // Tile Properties
    // =========================================================================

    /// Get the tile width.
    ///
    /// C++ equivalent: `NS::UInteger tileWidth() const`
    #[inline]
    pub fn tile_width(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(tileWidth)) }
    }

    /// Set the tile width.
    ///
    /// C++ equivalent: `void setTileWidth(NS::UInteger)`
    #[inline]
    pub fn set_tile_width(&self, width: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setTileWidth:), width);
        }
    }

    /// Get the tile height.
    ///
    /// C++ equivalent: `NS::UInteger tileHeight() const`
    #[inline]
    pub fn tile_height(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(tileHeight)) }
    }

    /// Set the tile height.
    ///
    /// C++ equivalent: `void setTileHeight(NS::UInteger)`
    #[inline]
    pub fn set_tile_height(&self, height: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setTileHeight:), height);
        }
    }

    /// Get the threadgroup memory length.
    ///
    /// C++ equivalent: `NS::UInteger threadgroupMemoryLength() const`
    #[inline]
    pub fn threadgroup_memory_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(threadgroupMemoryLength)) }
    }

    /// Set the threadgroup memory length.
    ///
    /// C++ equivalent: `void setThreadgroupMemoryLength(NS::UInteger)`
    #[inline]
    pub fn set_threadgroup_memory_length(&self, length: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setThreadgroupMemoryLength:), length);
        }
    }

    /// Get the imageblock sample length.
    ///
    /// C++ equivalent: `NS::UInteger imageblockSampleLength() const`
    #[inline]
    pub fn imageblock_sample_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(imageblockSampleLength)) }
    }

    /// Set the imageblock sample length.
    ///
    /// C++ equivalent: `void setImageblockSampleLength(NS::UInteger)`
    #[inline]
    pub fn set_imageblock_sample_length(&self, length: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setImageblockSampleLength:), length);
        }
    }

    // =========================================================================
    // Rasterization Rate Map
    // =========================================================================

    /// Get the rasterization rate map.
    ///
    /// C++ equivalent: `RasterizationRateMap* rasterizationRateMap() const`
    ///
    /// Returns a raw pointer to the rasterization rate map.
    pub fn rasterization_rate_map(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(rasterizationRateMap)) }
    }

    /// Set the rasterization rate map.
    ///
    /// C++ equivalent: `void setRasterizationRateMap(const RasterizationRateMap*)`
    ///
    /// # Safety
    ///
    /// The rate_map pointer must be valid or null.
    pub unsafe fn set_rasterization_rate_map(&self, rate_map: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setRasterizationRateMap:),
                rate_map,
            );
        }
    }

    // =========================================================================
    // Color Attachment Mapping
    // =========================================================================

    /// Check if color attachment mapping is supported.
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
}

impl Default for RenderPassDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create RenderPassDescriptor")
    }
}

impl Clone for RenderPassDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for RenderPassDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for RenderPassDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RenderPassDescriptor {}
unsafe impl Sync for RenderPassDescriptor {}

impl std::fmt::Debug for RenderPassDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderPassDescriptor")
            .field("render_target_width", &self.render_target_width())
            .field("render_target_height", &self.render_target_height())
            .field(
                "default_raster_sample_count",
                &self.default_raster_sample_count(),
            )
            .finish()
    }
}
