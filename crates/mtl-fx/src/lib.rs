// Clippy allows for FFI binding patterns
#![allow(clippy::not_unsafe_ptr_arg_deref)] // Raw pointer args are passed to Obj-C, not dereferenced in Rust
#![allow(clippy::missing_safety_doc)] // from_raw patterns are consistent across the crate

//! MetalFX bindings for AI upscaling and frame interpolation.
//!
//! This crate provides safe Rust bindings to Apple's MetalFX framework, which
//! provides AI-powered upscaling and frame interpolation for Metal applications.
//!
//! # Key Types
//!
//! - [`SpatialScaler`] - AI-based spatial upscaling (single-frame)
//! - [`TemporalScaler`] - AI-based temporal upscaling using motion vectors
//! - [`TemporalDenoisedScaler`] - Temporal upscaling with ray tracing denoising
//! - [`FrameInterpolator`] - Frame generation for smoother gameplay
//!
//! # Example
//!
//! ```ignore
//! use metal_fx::{SpatialScalerDescriptor, SpatialScalerColorProcessingMode};
//! use mtl_gpu::PixelFormat;
//!
//! // Create a spatial scaler descriptor
//! let desc = SpatialScalerDescriptor::new().expect("failed to create descriptor");
//! desc.set_color_texture_format(PixelFormat::RGBA16_FLOAT);
//! desc.set_output_texture_format(PixelFormat::RGBA16_FLOAT);
//! desc.set_input_width(1920);
//! desc.set_input_height(1080);
//! desc.set_output_width(3840);
//! desc.set_output_height(2160);
//! desc.set_color_processing_mode(SpatialScalerColorProcessingMode::HDR);
//!
//! // Create the scaler
//! if let Some(scaler) = desc.new_spatial_scaler(&device) {
//!     // Use scaler...
//! }
//! ```

#![allow(dead_code)]

mod enums;
mod frame_interpolator;
mod spatial_scaler;
mod temporal_denoised_scaler;
mod temporal_scaler;

pub use enums::SpatialScalerColorProcessingMode;
pub use frame_interpolator::{FrameInterpolator, FrameInterpolatorDescriptor};
pub use spatial_scaler::{SpatialScaler, SpatialScalerDescriptor};
pub use temporal_denoised_scaler::{TemporalDenoisedScaler, TemporalDenoisedScalerDescriptor};
pub use temporal_scaler::{TemporalScaler, TemporalScalerDescriptor};
