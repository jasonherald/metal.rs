//! Metal pass descriptors.
//!
//! Corresponds to `Metal/MTLRenderPass.hpp`, `Metal/MTLComputePass.hpp`, `Metal/MTLBlitPass.hpp`,
//! and `Metal/MTLResourceStatePass.hpp`.
//!
//! Pass descriptors configure the state and resources for command encoders.

mod attachment;
mod blit_pass;
mod blit_sample_buffer;
mod color_attachment;
mod compute_pass;
mod compute_sample_buffer;
mod depth_attachment;
mod render_pass;
mod render_sample_buffer;
mod resource_state;
mod stencil_attachment;

pub use attachment::RenderPassAttachmentDescriptor;
pub use blit_pass::BlitPassDescriptor;
pub use blit_sample_buffer::{
    BlitPassSampleBufferAttachmentDescriptor, BlitPassSampleBufferAttachmentDescriptorArray,
};
pub use color_attachment::{
    RenderPassColorAttachmentDescriptor, RenderPassColorAttachmentDescriptorArray,
};
pub use compute_pass::ComputePassDescriptor;
pub use compute_sample_buffer::{
    ComputePassSampleBufferAttachmentDescriptor, ComputePassSampleBufferAttachmentDescriptorArray,
};
pub use depth_attachment::RenderPassDepthAttachmentDescriptor;
pub use render_pass::RenderPassDescriptor;
pub use render_sample_buffer::{
    RenderPassSampleBufferAttachmentDescriptor, RenderPassSampleBufferAttachmentDescriptorArray,
};
pub use resource_state::{
    ResourceStatePassDescriptor, ResourceStatePassSampleBufferAttachmentDescriptor,
    ResourceStatePassSampleBufferAttachmentDescriptorArray,
};
pub use stencil_attachment::RenderPassStencilAttachmentDescriptor;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;

    #[test]
    fn test_render_pass_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<RenderPassDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_compute_pass_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<ComputePassDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_blit_pass_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<BlitPassDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
