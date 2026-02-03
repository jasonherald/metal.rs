//! Metal enumerations.
//!
//! This module contains all enum types from the Metal framework.
//!
//! # C++ Header Mapping
//!
//! | Rust Module | C++ Header |
//! |-------------|------------|
//! | [`pixel_format`] | `MTLPixelFormat.hpp` |
//! | [`data_type`] | `MTLDataType.hpp` |
//! | [`resource`] | `MTLResource.hpp` |
//! | [`texture`] | `MTLTexture.hpp` |
//! | [`sampler`] | `MTLSampler.hpp` |
//! | [`depth_stencil`] | `MTLDepthStencil.hpp` |
//! | [`render`] | `MTLRenderCommandEncoder.hpp`, `MTLRenderPass.hpp` |
//! | [`pipeline`] | `MTLRenderPipeline.hpp`, `MTLPipeline.hpp` |
//! | [`command`] | `MTLCommandBuffer.hpp` |
//! | [`argument`] | `MTLArgument.hpp` |
//! | [`device`] | `MTLDevice.hpp` |
//! | [`library`] | `MTLLibrary.hpp` |
//! | [`acceleration`] | `MTLAccelerationStructure.hpp` |
//! | [`heap`] | `MTLHeap.hpp` |
//! | [`blit`] | `MTLBlitCommandEncoder.hpp` |
//! | [`vertex`] | `MTLVertexDescriptor.hpp`, `MTLStageInputOutputDescriptor.hpp` |
//! | [`indirect_command`] | `MTLIndirectCommandBuffer.hpp` |
//! | [`io`] | `MTLIOCommandBuffer.hpp` |
//! | [`capture`] | `MTLCaptureManager.hpp` |
//! | [`counter`] | `MTLCounters.hpp` |
//! | [`function`] | `MTLFunctionDescriptor.hpp` |

pub mod acceleration;
pub mod argument;
pub mod blit;
pub mod capture;
pub mod command;
pub mod counter;
pub mod data_type;
pub mod depth_stencil;
pub mod device;
pub mod function;
pub mod heap;
pub mod indirect_command;
pub mod io;
pub mod library;
pub mod log;
pub mod pipeline;
pub mod pixel_format;
pub mod render;
pub mod resource;
pub mod sampler;
pub mod tensor;
pub mod texture;
pub mod vertex;

// Re-export all types at module level
pub use acceleration::*;
pub use argument::*;
pub use blit::*;
pub use capture::*;
pub use command::*;
pub use counter::*;
pub use data_type::*;
pub use depth_stencil::*;
pub use device::*;
pub use function::*;
pub use heap::*;
pub use indirect_command::*;
pub use io::*;
pub use library::*;
pub use log::*;
pub use pipeline::*;
pub use pixel_format::*;
pub use render::*;
pub use resource::*;
pub use sampler::*;
pub use tensor::*;
pub use texture::*;
pub use vertex::*;
