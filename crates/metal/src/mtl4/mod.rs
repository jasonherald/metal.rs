//! Metal 4 API bindings.
//!
//! This module provides bindings to Metal 4 APIs, which offer enhanced
//! control over command buffering, pipelines, and GPU resource management.
//!
//! # Key Types
//!
//! ## Command Infrastructure
//! - [`CommandQueue`] - Metal 4 command queue with explicit residency management
//! - [`CommandBuffer`] - Metal 4 command buffer with explicit allocator
//! - [`CommandAllocator`] - Memory allocator for command buffer recording
//! - [`CommitFeedback`] - Feedback about committed command buffers
//!
//! ## Command Encoders
//! - [`CommandEncoder`] - Base command encoder with barrier and fence methods
//! - [`ComputeCommandEncoder`] - Compute shader dispatch and resource binding
//! - [`RenderCommandEncoder`] - Render pass encoding with draw commands
//! - [`MachineLearningCommandEncoder`] - ML network dispatch encoder
//!
//! ## Render Pass
//! - [`RenderPassDescriptor`] - Metal 4 render pass configuration
//!
//! ## Pipelines
//! - [`ComputePipelineDescriptor`] - Compute pipeline configuration
//! - [`RenderPipelineDescriptor`] - Render pipeline configuration
//! - [`MeshRenderPipelineDescriptor`] - Mesh shading pipeline configuration
//! - [`TileRenderPipelineDescriptor`] - Tile shading pipeline configuration
//! - [`MachineLearningPipelineDescriptor`] - ML pipeline configuration
//! - [`MachineLearningPipelineState`] - Compiled ML pipeline
//!
//! ## Function Descriptors
//! - [`FunctionDescriptor`] - Function specification for pipelines
//! - [`LibraryFunctionDescriptor`] - Function from a library by name
//! - [`SpecializedFunctionDescriptor`] - Function with constant specialization
//! - [`StitchedFunctionDescriptor`] - Graph-based function composition
//! - [`StaticLinkingDescriptor`] - Static shader linking configuration
//! - [`PipelineStageDynamicLinkingDescriptor`] - Dynamic linking per stage
//! - [`RenderPipelineDynamicLinkingDescriptor`] - Dynamic linking for render pipelines
//!
//! ## Compiler
//! - [`Compiler`] - Shader compiler for creating pipelines and functions
//! - [`CompilerTask`] - Asynchronous compilation task
//! - [`BinaryFunction`] - Precompiled binary function for linking
//! - [`LibraryDescriptor`] - Library creation configuration
//! - [`PipelineDataSetSerializer`] - Pipeline data serialization
//!
//! ## Archives
//! - [`Archive`] - Pre-compiled pipeline archive
//!
//! ## Argument Binding
//! - [`ArgumentTable`] - GPU resource binding table
//! - [`ArgumentTableDescriptor`] - Configuration for argument tables
//!
//! ## Counters
//! - [`CounterHeap`] - GPU performance counter storage
//! - [`CounterHeapDescriptor`] - Counter heap configuration
//!
//! ## Machine Learning
//! - [`MachineLearningPipelineDescriptor`] - ML pipeline descriptor
//! - [`MachineLearningPipelineState`] - Compiled ML pipeline state
//! - [`MachineLearningPipelineReflection`] - ML pipeline reflection data

mod enums;
mod command_allocator;
mod command_buffer;
mod command_queue;
mod commit_feedback;
mod command_encoder;
mod compute_command_encoder;
mod render_command_encoder;
mod render_pass;
mod function_descriptor;
mod linking_descriptor;
mod pipeline_state;
mod compute_pipeline;
mod render_pipeline;
mod mesh_render_pipeline;
mod tile_render_pipeline;
mod binary_function;
mod library_descriptor;
mod pipeline_data_set_serializer;
mod compiler_task;
mod compiler;
mod argument_table;
mod archive;
mod library_function_descriptor;
mod specialized_function_descriptor;
mod stitched_function_descriptor;
mod counters;
mod machine_learning;
mod acceleration_structure;

// Re-export enums
pub use enums::{
    AlphaToCoverageState, AlphaToOneState, BinaryFunctionOptions, BlendState, CommandQueueError,
    CompilerTaskStatus, IndirectCommandBufferSupportState,
    LogicalToPhysicalColorAttachmentMappingState, PipelineDataSetSerializerConfiguration,
    RenderEncoderOptions, ShaderReflection, VisibilityOptions,
};

// Re-export command types
pub use command_allocator::{CommandAllocator, CommandAllocatorDescriptor};
pub use command_buffer::{CommandBuffer, CommandBufferOptions};
pub use command_queue::{CommandQueue, CommandQueueDescriptor, CommitOptions};
pub use commit_feedback::CommitFeedback;

// Re-export command encoder types
pub use command_encoder::CommandEncoder;
pub use compute_command_encoder::ComputeCommandEncoder;
pub use render_command_encoder::RenderCommandEncoder;

// Re-export function and linking descriptors
pub use function_descriptor::FunctionDescriptor;
pub use linking_descriptor::{
    PipelineStageDynamicLinkingDescriptor, RenderPipelineDynamicLinkingDescriptor,
    StaticLinkingDescriptor,
};

// Re-export pipeline state types
pub use pipeline_state::{PipelineDescriptor, PipelineOptions};

// Re-export compute pipeline types
pub use compute_pipeline::ComputePipelineDescriptor;

// Re-export render pipeline types
pub use render_pipeline::{
    RenderPipelineBinaryFunctionsDescriptor, RenderPipelineColorAttachmentDescriptor,
    RenderPipelineColorAttachmentDescriptorArray, RenderPipelineDescriptor,
};

// Re-export mesh render pipeline types
pub use mesh_render_pipeline::MeshRenderPipelineDescriptor;

// Re-export tile render pipeline types
pub use tile_render_pipeline::TileRenderPipelineDescriptor;

// Re-export binary function types
pub use binary_function::{BinaryFunction, BinaryFunctionDescriptor};

// Re-export library descriptor types
pub use library_descriptor::LibraryDescriptor;

// Re-export pipeline data set serializer types
pub use pipeline_data_set_serializer::{
    PipelineDataSetSerializer, PipelineDataSetSerializerDescriptor,
};

// Re-export compiler types
pub use compiler_task::CompilerTask;
pub use compiler::{Compiler, CompilerDescriptor, CompilerTaskOptions};

// Re-export render pass types
pub use render_pass::RenderPassDescriptor;

// Re-export argument table types
pub use argument_table::{ArgumentTable, ArgumentTableDescriptor};

// Re-export archive types
pub use archive::Archive;

// Re-export function descriptor types
pub use library_function_descriptor::LibraryFunctionDescriptor;
pub use specialized_function_descriptor::SpecializedFunctionDescriptor;
pub use stitched_function_descriptor::StitchedFunctionDescriptor;

// Re-export counter types
pub use counters::{
    CounterHeap, CounterHeapDescriptor, CounterHeapType, TimestampGranularity, TimestampHeapEntry,
};

// Re-export machine learning types
pub use machine_learning::{
    MachineLearningCommandEncoder, MachineLearningPipelineDescriptor,
    MachineLearningPipelineReflection, MachineLearningPipelineState,
};

// Re-export acceleration structure types
pub use acceleration_structure::{
    AccelerationStructureBoundingBoxGeometryDescriptor,
    AccelerationStructureCurveGeometryDescriptor, AccelerationStructureDescriptor,
    AccelerationStructureGeometryDescriptor,
    AccelerationStructureMotionBoundingBoxGeometryDescriptor,
    AccelerationStructureMotionCurveGeometryDescriptor,
    AccelerationStructureMotionTriangleGeometryDescriptor,
    AccelerationStructureTriangleGeometryDescriptor, BufferRange,
    IndirectInstanceAccelerationStructureDescriptor, InstanceAccelerationStructureDescriptor,
    PrimitiveAccelerationStructureDescriptor,
};
