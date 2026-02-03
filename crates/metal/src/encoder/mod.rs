//! Metal command encoders.
//!
//! Corresponds to `Metal/MTLCommandEncoder.hpp`, `Metal/MTLBlitCommandEncoder.hpp`,
//! `Metal/MTLComputeCommandEncoder.hpp`, `Metal/MTLRenderCommandEncoder.hpp`,
//! `Metal/MTLParallelRenderCommandEncoder.hpp`, and `Metal/MTLResourceStateCommandEncoder.hpp`.
//!
//! Command encoders are used to encode GPU commands into command buffers.
//! Each encoder type corresponds to a specific type of GPU work:
//! - [`BlitCommandEncoder`] - Data transfer operations
//! - [`ComputeCommandEncoder`] - Compute shader dispatch
//! - [`RenderCommandEncoder`] - Graphics rendering
//! - [`ParallelRenderCommandEncoder`] - Parallel rendering with multiple render encoders
//! - [`ResourceStateCommandEncoder`] - Sparse texture mapping operations

mod blit_encoder;
mod compute_encoder;
mod parallel_render_encoder;
mod render_encoder;
mod resource_state_encoder;

pub use blit_encoder::BlitCommandEncoder;
pub use compute_encoder::ComputeCommandEncoder;
pub use parallel_render_encoder::ParallelRenderCommandEncoder;
pub use render_encoder::RenderCommandEncoder;
pub use resource_state_encoder::{MapIndirectArguments, ResourceStateCommandEncoder};

// Re-export compute types
pub use compute_encoder::{
    DispatchThreadgroupsIndirectArguments, DispatchThreadsIndirectArguments,
    StageInRegionIndirectArguments,
};
