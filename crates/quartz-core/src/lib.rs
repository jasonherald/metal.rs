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
//! use metal::device;
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

mod types;
mod metal_layer;
mod metal_drawable;

pub use types::{CGFloat, CGSize, CGColorSpaceRef};
pub use metal_layer::MetalLayer;
pub use metal_drawable::MetalDrawable;
