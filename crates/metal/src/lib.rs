// Clippy allows for FFI binding patterns
#![allow(clippy::not_unsafe_ptr_arg_deref)] // Raw pointer args are passed to Obj-C, not dereferenced in Rust
#![allow(clippy::missing_safety_doc)] // from_raw patterns are consistent across the crate
#![allow(clippy::module_inception)] // module::module pattern mirrors Metal API structure
#![allow(clippy::self_named_constructors)] // Matches Objective-C factory method naming
#![allow(clippy::forget_non_drop)] // Intentional for preventing block cleanup in async callbacks
#![allow(clippy::too_many_arguments)] // Matches Metal API signatures

//! Metal framework bindings for Rust.
//!
//! This crate provides safe, idiomatic Rust bindings to Apple's Metal graphics API.
//! It is a 1:1 translation of the official metal-cpp library with zero external dependencies.
//!
//! # Module Organization
//!
//! The crate is organized to mirror the Metal framework structure:
//!
//! - [`types`] - Core value types (Origin, Size, Region, Viewport, etc.)
//! - [`enums`] - All Metal enumerations and option flags
//! - [`device`] - GPU device interface and creation functions
//! - [`buffer`] - Buffer resources for data storage
//! - [`texture`] - Texture resources for image data
//! - [`command_queue`] - Command queue for command buffer scheduling
//! - [`command_buffer`] - Command buffer for GPU command encoding
//! - [`sampler`] - Sampler state for texture sampling
//! - [`depth_stencil`] - Depth and stencil testing state
//!
//! # Example
//!
//! ```ignore
//! use metal::device;
//!
//! // Get the default GPU
//! let device = device::system_default().expect("no Metal device");
//! println!("Using GPU: {}", device.name());
//!
//! // Create a command queue
//! let queue = device.new_command_queue().expect("failed to create queue");
//!
//! // Create a buffer
//! let buffer = device.new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)
//!     .expect("failed to create buffer");
//! ```

// Core modules
pub mod device;
pub mod enums;
pub mod error;
pub mod types;

// Allocation and Resource protocols
pub mod allocation;
pub mod resource;

// Resource modules
pub mod buffer;
pub mod heap;
pub mod texture;

// Command modules
pub mod command_buffer;
pub mod command_queue;

// Encoder modules
pub mod encoder;

// State modules
pub mod depth_stencil;
pub mod sampler;

// Library and pipeline modules
pub mod library;
pub mod pipeline;

// Pass descriptor modules
pub mod pass;

// Synchronization modules
pub mod sync;

// Acceleration structure modules
pub mod acceleration;

// IO modules
pub mod io;

// Argument reflection modules
pub mod argument;
pub mod argument_descriptor;

// Command buffer encoder info
pub mod command_buffer_encoder_info;

// Binary archive modules
pub mod binary_archive;

// Capture/debugging modules
pub mod capture;

// Counter/profiling modules
pub mod counter;

// Indirect command modules
pub mod indirect;

// Function stitching modules
pub mod function_stitching;

// Function table modules
pub mod function_table;

// Residency set modules
pub mod residency_set;

// Tensor modules
pub mod tensor;

// Drawable modules
pub mod drawable;

// Vertex descriptor modules
pub mod vertex;

// Stage input/output descriptor modules
pub mod stage_input_output;

// Rasterization rate modules
pub mod rasterization_rate;

// Log state modules
pub mod log_state;

// Function log modules
pub mod function_log;

// Texture view pool modules
pub mod texture_view_pool;

// Metal 4 modules
pub mod mtl4;

// Re-export commonly used types at crate root
pub use enums::*;
pub use error::ValidationError;
pub use types::*;

// Re-export Device and creation functions for convenience
pub use device::{Architecture, Device};

// Re-export resource types
pub use buffer::Buffer;
pub use heap::{Heap, HeapDescriptor};
pub use texture::{SharedTextureHandle, Texture, TextureDescriptor, TextureViewDescriptor};

// Re-export command types
pub use command_buffer::{CommandBuffer, CommandBufferDescriptor};
pub use command_queue::{CommandQueue, CommandQueueDescriptor};

// Re-export state types
pub use depth_stencil::{DepthStencilDescriptor, DepthStencilState, StencilDescriptor};
pub use sampler::{SamplerDescriptor, SamplerState};

// Re-export library types
pub use library::{
    Attribute, CompileOptions, DynamicLibrary, Function, FunctionConstant, FunctionConstantValues,
    FunctionDescriptor, FunctionReflection, IntersectionFunctionDescriptor, Library,
    LinkedFunctions, VertexAttribute,
};

// Re-export pipeline types
pub use pipeline::{
    ComputePipelineDescriptor, ComputePipelineState, LogicalToPhysicalColorAttachmentMap,
    MeshRenderPipelineDescriptor, PipelineBufferDescriptor, PipelineBufferDescriptorArray,
    RenderPipelineColorAttachmentDescriptor, RenderPipelineColorAttachmentDescriptorArray,
    RenderPipelineDescriptor, RenderPipelineFunctionsDescriptor, RenderPipelineReflection,
    RenderPipelineState, TileRenderPipelineColorAttachmentDescriptor,
    TileRenderPipelineColorAttachmentDescriptorArray, TileRenderPipelineDescriptor,
};

// Re-export sync types
pub use sync::{Event, Fence, SharedEvent, SharedEventHandle, SharedEventListener};

// Re-export pass descriptor types
pub use pass::{
    BlitPassDescriptor, BlitPassSampleBufferAttachmentDescriptor,
    BlitPassSampleBufferAttachmentDescriptorArray, ComputePassDescriptor,
    ComputePassSampleBufferAttachmentDescriptor, ComputePassSampleBufferAttachmentDescriptorArray,
    RenderPassColorAttachmentDescriptor, RenderPassColorAttachmentDescriptorArray,
    RenderPassDepthAttachmentDescriptor, RenderPassDescriptor,
    RenderPassSampleBufferAttachmentDescriptor, RenderPassSampleBufferAttachmentDescriptorArray,
    RenderPassStencilAttachmentDescriptor, ResourceStatePassDescriptor,
    ResourceStatePassSampleBufferAttachmentDescriptor,
    ResourceStatePassSampleBufferAttachmentDescriptorArray,
};

// Re-export acceleration structure types
pub use acceleration::{
    AccelerationStructure, AccelerationStructureBoundingBoxGeometryDescriptor,
    AccelerationStructureCommandEncoder, AccelerationStructureCurveGeometryDescriptor,
    AccelerationStructureDescriptor, AccelerationStructureGeometryDescriptor,
    AccelerationStructureMotionBoundingBoxGeometryDescriptor,
    AccelerationStructureMotionCurveGeometryDescriptor,
    AccelerationStructureMotionTriangleGeometryDescriptor, AccelerationStructurePassDescriptor,
    AccelerationStructurePassSampleBufferAttachmentDescriptor,
    AccelerationStructurePassSampleBufferAttachmentDescriptorArray, AccelerationStructureSizes,
    AccelerationStructureTriangleGeometryDescriptor,
    IndirectInstanceAccelerationStructureDescriptor, InstanceAccelerationStructureDescriptor,
    MotionKeyframeData, PrimitiveAccelerationStructureDescriptor,
};

// Re-export encoder types
pub use encoder::{
    BlitCommandEncoder, ComputeCommandEncoder, DispatchThreadgroupsIndirectArguments,
    DispatchThreadsIndirectArguments, MapIndirectArguments, ParallelRenderCommandEncoder,
    RenderCommandEncoder, ResourceStateCommandEncoder, StageInRegionIndirectArguments,
};

// Re-export IO types
pub use io::{
    IOCommandBuffer, IOCommandQueue, IOCommandQueueDescriptor, IOCompressionContext, IOFileHandle,
    IOScratchBuffer, IOScratchBufferAllocator, io_compression_context_append_data,
    io_compression_context_default_chunk_size, io_create_compression_context,
    io_flush_and_destroy_compression_context,
};

// Re-export argument types
pub use argument::{
    ATTRIBUTE_STRIDE_STATIC, Argument, ArgumentEncoder, ArrayType, Binding, BufferBinding,
    ObjectPayloadBinding, PointerType, StructMember, StructType, TensorBinding,
    TensorReferenceType, TextureBinding, TextureReferenceType, ThreadgroupBinding, Type,
};
pub use argument_descriptor::ArgumentDescriptor;

// Re-export command buffer encoder info
pub use command_buffer_encoder_info::CommandBufferEncoderInfo;

// Re-export binary archive types
pub use binary_archive::{BinaryArchive, BinaryArchiveDescriptor, BinaryArchiveError};

// Re-export function stitching types
pub use function_stitching::{
    FunctionStitchingAttribute, FunctionStitchingAttributeAlwaysInline,
    FunctionStitchingFunctionNode, FunctionStitchingGraph, FunctionStitchingInputNode,
    FunctionStitchingNode, StitchedLibraryDescriptor, StitchedLibraryOptions,
};

// Re-export residency set types
pub use residency_set::{ResidencySet, ResidencySetDescriptor};

// Re-export tensor types
pub use tensor::{Tensor, TensorDescriptor, TensorExtents};

// Re-export drawable types
pub use drawable::{Drawable, TimeInterval};

// Re-export capture types
pub use capture::{CaptureDescriptor, CaptureManager, CaptureScope};

// Re-export counter types
pub use counter::{
    COUNTER_DONT_SAMPLE, COUNTER_ERROR_VALUE, Counter, CounterResultStageUtilization,
    CounterResultStatistic, CounterResultTimestamp, CounterSampleBuffer,
    CounterSampleBufferDescriptor, CounterSet,
};

// Re-export indirect command types
pub use indirect::{
    IndirectCommandBuffer, IndirectCommandBufferDescriptor, IndirectCommandBufferExecutionRange,
    IndirectComputeCommand, IndirectRenderCommand,
};

// Re-export vertex descriptor types
pub use vertex::{
    BUFFER_LAYOUT_STRIDE_DYNAMIC, VertexAttributeDescriptor, VertexAttributeDescriptorArray,
    VertexBufferLayoutDescriptor, VertexBufferLayoutDescriptorArray, VertexDescriptor,
};

// Re-export function table types
pub use function_table::{
    FunctionHandle, IntersectionFunctionBufferArguments, IntersectionFunctionTable,
    IntersectionFunctionTableDescriptor, VisibleFunctionTable, VisibleFunctionTableDescriptor,
};

// Re-export rasterization rate types
pub use rasterization_rate::{
    RasterizationRateLayerArray, RasterizationRateLayerDescriptor, RasterizationRateMap,
    RasterizationRateMapDescriptor, RasterizationRateSampleArray,
};

// Re-export log state types
pub use log_state::{LogState, LogStateDescriptor};

// Re-export function log types
pub use function_log::{FunctionLog, FunctionLogDebugLocation, LogContainer};

// Re-export texture view pool types
pub use texture_view_pool::{ResourceViewPoolDescriptor, TextureViewPool};

// Re-export foundation types for convenience
pub use metal_foundation::{Integer, UInteger};

// Re-export protocol traits
pub use allocation::Allocation;
pub use resource::Resource;

// Re-export stage input/output types
pub use stage_input_output::{
    AttributeDescriptor, AttributeDescriptorArray, BufferLayoutDescriptor,
    BufferLayoutDescriptorArray, StageInputOutputDescriptor,
};
