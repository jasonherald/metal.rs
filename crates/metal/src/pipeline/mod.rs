//! Metal pipeline state objects.
//!
//! This module contains types for creating and managing render and compute pipelines.
//!
//! Corresponds to `Metal/MTLRenderPipeline.hpp` and `Metal/MTLComputePipeline.hpp`.

mod buffer_descriptor;
mod color_attachment;
mod compute_descriptor;
mod compute_state;
mod functions_descriptor;
mod mesh_pipeline;
mod reflection;
mod render_descriptor;
mod render_state;
mod tile_pipeline;

// Re-export all public types
pub use buffer_descriptor::{PipelineBufferDescriptor, PipelineBufferDescriptorArray};
pub use color_attachment::{
    RenderPipelineColorAttachmentDescriptor, RenderPipelineColorAttachmentDescriptorArray,
};
pub use compute_descriptor::ComputePipelineDescriptor;
pub use compute_state::ComputePipelineState;
pub use functions_descriptor::{LogicalToPhysicalColorAttachmentMap, RenderPipelineFunctionsDescriptor};
pub use mesh_pipeline::MeshRenderPipelineDescriptor;
pub use reflection::{ComputePipelineReflection, RenderPipelineReflection};
pub use render_descriptor::RenderPipelineDescriptor;
pub use render_state::RenderPipelineState;
pub use tile_pipeline::{
    TileRenderPipelineColorAttachmentDescriptor, TileRenderPipelineColorAttachmentDescriptorArray,
    TileRenderPipelineDescriptor,
};
