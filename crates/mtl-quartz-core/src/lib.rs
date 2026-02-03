// Clippy allows for FFI binding patterns
#![allow(clippy::not_unsafe_ptr_arg_deref)] // Raw pointer args are passed to Obj-C, not dereferenced in Rust
#![allow(clippy::missing_safety_doc)] // from_raw patterns are consistent across the crate

//! QuartzCore/CoreAnimation bindings.
//!
//! Provides CAMetalLayer and CAMetalDrawable for display integration.
//!
//! This crate provides safe Rust bindings to the QuartzCore framework's Metal
//! integration types. These are used to display Metal-rendered content on screen.
//!
//! # Example
//!
//! ```ignore
//! use quartz_core::MetalLayer;
//! use mtl::device;
//!
//! // Create a Metal layer
//! let layer = MetalLayer::layer().expect("failed to create layer");
//!
//! // Set the device
//! let device = device::system_default().expect("no Metal device");
//! layer.set_device(&device);
//!
//! // Get a drawable to render to
//! if let Some(drawable) = layer.next_drawable() {
//!     let texture = drawable.texture();
//!     // Render to texture...
//!     drawable.present();
//! }
//! ```

#![allow(dead_code)]

mod metal_drawable;
mod metal_layer;
mod types;

pub use metal_drawable::MetalDrawable;
pub use metal_layer::MetalLayer;
pub use types::{CGColorSpaceRef, CGFloat, CGSize};
