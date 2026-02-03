//! MTL4 RenderPass implementation.
//!
//! Corresponds to `Metal/MTL4RenderPass.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, msg_send_2, sel};

use crate::{
    Buffer, RenderPassColorAttachmentDescriptorArray, RenderPassDepthAttachmentDescriptor,
    RenderPassStencilAttachmentDescriptor, SamplePosition, VisibilityResultType,
};

// ============================================================
// RenderPassDescriptor
// ============================================================

/// MTL4 render pass descriptor.
///
/// C++ equivalent: `MTL4::RenderPassDescriptor`
///
/// RenderPassDescriptor configures a render pass including color, depth,
/// and stencil attachments, sample positions, and tile dimensions.
#[repr(transparent)]
pub struct RenderPassDescriptor(NonNull<c_void>);

impl RenderPassDescriptor {
    /// Create a RenderPassDescriptor from a raw pointer.
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
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTL4RenderPassDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    // ========== Color Attachments ==========

    /// Get the color attachments array.
    ///
    /// C++ equivalent: `MTL::RenderPassColorAttachmentDescriptorArray* colorAttachments() const`
    pub fn color_attachments(&self) -> Option<RenderPassColorAttachmentDescriptorArray> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(colorAttachments));
            RenderPassColorAttachmentDescriptorArray::from_raw(ptr)
        }
    }

    // ========== Depth Attachment ==========

    /// Get the depth attachment.
    ///
    /// C++ equivalent: `MTL::RenderPassDepthAttachmentDescriptor* depthAttachment() const`
    pub fn depth_attachment(&self) -> Option<RenderPassDepthAttachmentDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(depthAttachment));
            RenderPassDepthAttachmentDescriptor::from_raw(ptr)
        }
    }

    /// Set the depth attachment.
    ///
    /// C++ equivalent: `void setDepthAttachment(const MTL::RenderPassDepthAttachmentDescriptor*)`
    pub fn set_depth_attachment(&self, attachment: &RenderPassDepthAttachmentDescriptor) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setDepthAttachment:),
                attachment.as_ptr(),
            );
        }
    }

    // ========== Stencil Attachment ==========

    /// Get the stencil attachment.
    ///
    /// C++ equivalent: `MTL::RenderPassStencilAttachmentDescriptor* stencilAttachment() const`
    pub fn stencil_attachment(&self) -> Option<RenderPassStencilAttachmentDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(stencilAttachment));
            RenderPassStencilAttachmentDescriptor::from_raw(ptr)
        }
    }

    /// Set the stencil attachment.
    ///
    /// C++ equivalent: `void setStencilAttachment(const MTL::RenderPassStencilAttachmentDescriptor*)`
    pub fn set_stencil_attachment(&self, attachment: &RenderPassStencilAttachmentDescriptor) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setStencilAttachment:),
                attachment.as_ptr(),
            );
        }
    }

    // ========== Render Target Dimensions ==========

    /// Get the render target width.
    ///
    /// C++ equivalent: `NS::UInteger renderTargetWidth() const`
    pub fn render_target_width(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(renderTargetWidth)) }
    }

    /// Set the render target width.
    ///
    /// C++ equivalent: `void setRenderTargetWidth(NS::UInteger)`
    pub fn set_render_target_width(&self, width: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setRenderTargetWidth:), width);
        }
    }

    /// Get the render target height.
    ///
    /// C++ equivalent: `NS::UInteger renderTargetHeight() const`
    pub fn render_target_height(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(renderTargetHeight)) }
    }

    /// Set the render target height.
    ///
    /// C++ equivalent: `void setRenderTargetHeight(NS::UInteger)`
    pub fn set_render_target_height(&self, height: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setRenderTargetHeight:), height);
        }
    }

    /// Get the render target array length.
    ///
    /// C++ equivalent: `NS::UInteger renderTargetArrayLength() const`
    pub fn render_target_array_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(renderTargetArrayLength)) }
    }

    /// Set the render target array length.
    ///
    /// C++ equivalent: `void setRenderTargetArrayLength(NS::UInteger)`
    pub fn set_render_target_array_length(&self, length: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setRenderTargetArrayLength:), length);
        }
    }

    // ========== Sample Count ==========

    /// Get the default raster sample count.
    ///
    /// C++ equivalent: `NS::UInteger defaultRasterSampleCount() const`
    pub fn default_raster_sample_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(defaultRasterSampleCount)) }
    }

    /// Set the default raster sample count.
    ///
    /// C++ equivalent: `void setDefaultRasterSampleCount(NS::UInteger)`
    pub fn set_default_raster_sample_count(&self, count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDefaultRasterSampleCount:), count);
        }
    }

    // ========== Sample Positions ==========

    /// Get sample positions.
    ///
    /// C++ equivalent: `NS::UInteger getSamplePositions(MTL::SamplePosition*, NS::UInteger)`
    pub fn get_sample_positions(&self, positions: &mut [SamplePosition]) -> UInteger {
        unsafe {
            msg_send_2(
                self.as_ptr(),
                sel!(getSamplePositions:count:),
                positions.as_mut_ptr(),
                positions.len() as UInteger,
            )
        }
    }

    /// Set sample positions.
    ///
    /// C++ equivalent: `void setSamplePositions(const MTL::SamplePosition*, NS::UInteger)`
    pub fn set_sample_positions(&self, positions: &[SamplePosition]) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setSamplePositions:count:),
                positions.as_ptr(),
                positions.len() as UInteger,
            );
        }
    }

    // ========== Tile Dimensions ==========

    /// Get the tile width.
    ///
    /// C++ equivalent: `NS::UInteger tileWidth() const`
    pub fn tile_width(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(tileWidth)) }
    }

    /// Set the tile width.
    ///
    /// C++ equivalent: `void setTileWidth(NS::UInteger)`
    pub fn set_tile_width(&self, width: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setTileWidth:), width);
        }
    }

    /// Get the tile height.
    ///
    /// C++ equivalent: `NS::UInteger tileHeight() const`
    pub fn tile_height(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(tileHeight)) }
    }

    /// Set the tile height.
    ///
    /// C++ equivalent: `void setTileHeight(NS::UInteger)`
    pub fn set_tile_height(&self, height: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setTileHeight:), height);
        }
    }

    // ========== Threadgroup Memory ==========

    /// Get the threadgroup memory length.
    ///
    /// C++ equivalent: `NS::UInteger threadgroupMemoryLength() const`
    pub fn threadgroup_memory_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(threadgroupMemoryLength)) }
    }

    /// Set the threadgroup memory length.
    ///
    /// C++ equivalent: `void setThreadgroupMemoryLength(NS::UInteger)`
    pub fn set_threadgroup_memory_length(&self, length: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setThreadgroupMemoryLength:), length);
        }
    }

    // ========== Imageblock ==========

    /// Get the imageblock sample length.
    ///
    /// C++ equivalent: `NS::UInteger imageblockSampleLength() const`
    pub fn imageblock_sample_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(imageblockSampleLength)) }
    }

    /// Set the imageblock sample length.
    ///
    /// C++ equivalent: `void setImageblockSampleLength(NS::UInteger)`
    pub fn set_imageblock_sample_length(&self, length: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setImageblockSampleLength:), length);
        }
    }

    // ========== Rasterization Rate Map ==========

    /// Get the rasterization rate map (as raw pointer).
    ///
    /// C++ equivalent: `MTL::RasterizationRateMap* rasterizationRateMap() const`
    pub fn rasterization_rate_map_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(rasterizationRateMap)) }
    }

    /// Set the rasterization rate map (from raw pointer).
    ///
    /// C++ equivalent: `void setRasterizationRateMap(const MTL::RasterizationRateMap*)`
    pub fn set_rasterization_rate_map_raw(&self, map: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setRasterizationRateMap:), map);
        }
    }

    // ========== Visibility Result ==========

    /// Get the visibility result buffer.
    ///
    /// C++ equivalent: `MTL::Buffer* visibilityResultBuffer() const`
    pub fn visibility_result_buffer(&self) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(visibilityResultBuffer));
            Buffer::from_raw(ptr)
        }
    }

    /// Set the visibility result buffer.
    ///
    /// C++ equivalent: `void setVisibilityResultBuffer(const MTL::Buffer*)`
    pub fn set_visibility_result_buffer(&self, buffer: &Buffer) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setVisibilityResultBuffer:),
                buffer.as_ptr(),
            );
        }
    }

    /// Get the visibility result type.
    ///
    /// C++ equivalent: `MTL::VisibilityResultType visibilityResultType() const`
    pub fn visibility_result_type(&self) -> VisibilityResultType {
        unsafe { msg_send_0(self.as_ptr(), sel!(visibilityResultType)) }
    }

    /// Set the visibility result type.
    ///
    /// C++ equivalent: `void setVisibilityResultType(MTL::VisibilityResultType)`
    pub fn set_visibility_result_type(&self, result_type: VisibilityResultType) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setVisibilityResultType:), result_type);
        }
    }

    // ========== Color Attachment Mapping ==========

    /// Check if color attachment mapping is supported.
    ///
    /// C++ equivalent: `bool supportColorAttachmentMapping() const`
    pub fn support_color_attachment_mapping(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportColorAttachmentMapping)) }
    }

    /// Set whether color attachment mapping is supported.
    ///
    /// C++ equivalent: `void setSupportColorAttachmentMapping(bool)`
    pub fn set_support_color_attachment_mapping(&self, support: bool) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setSupportColorAttachmentMapping:),
                support,
            );
        }
    }
}

impl Default for RenderPassDescriptor {
    fn default() -> Self {
        Self::new().expect("Failed to create MTL4RenderPassDescriptor")
    }
}

impl Clone for RenderPassDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for RenderPassDescriptor {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
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
            .field("tile_width", &self.tile_width())
            .field("tile_height", &self.tile_height())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_pass_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<RenderPassDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
