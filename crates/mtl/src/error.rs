//! Validation error types for Metal API.
//!
//! These errors are returned by safe wrapper methods that validate parameters
//! before calling Metal APIs that would otherwise abort the process.

use std::fmt;

use mtl_foundation::UInteger;

/// Validation error returned when resource creation would fail.
///
/// Metal's validation layer sometimes aborts the process instead of returning
/// an error for certain invalid configurations. These safe wrapper methods
/// validate parameters before calling Metal APIs and return this error type
/// instead of allowing the process to abort.
#[derive(Debug, Clone)]
pub enum ValidationError {
    // =========================================================================
    // Render Pipeline Errors
    // =========================================================================
    /// Render pipeline descriptor is missing a required vertex function.
    MissingVertexFunction,

    /// The raster sample count is not supported by the device.
    UnsupportedRasterSampleCount(UInteger),

    // =========================================================================
    // Compute Pipeline Errors
    // =========================================================================
    /// Compute pipeline descriptor is missing a required compute function.
    MissingComputeFunction,

    // =========================================================================
    // Texture Errors
    // =========================================================================
    /// Texture dimensions are invalid (width, height, or depth is zero).
    InvalidTextureDimensions {
        width: UInteger,
        height: UInteger,
        depth: UInteger,
    },

    /// Requested mipmap count exceeds the maximum allowed for the texture dimensions.
    InvalidMipmapCount {
        requested: UInteger,
        max_allowed: UInteger,
    },

    /// The texture sample count is not supported by the device.
    UnsupportedTextureSampleCount(UInteger),

    /// Array length is invalid (must be > 0 for array textures).
    InvalidArrayLength,

    // =========================================================================
    // Sampler Errors
    // =========================================================================
    /// LOD clamp range is invalid (min must be <= max).
    InvalidLodRange { min: f32, max: f32 },

    /// Max anisotropy value is invalid (must be >= 1 and power of 2).
    InvalidAnisotropy(UInteger),

    // =========================================================================
    // Heap Errors
    // =========================================================================
    /// Heap size is invalid (must be > 0).
    InvalidHeapSize,

    // =========================================================================
    // Generic Errors
    // =========================================================================
    /// Metal failed to create the resource.
    CreationFailed(Option<mtl_foundation::Error>),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Render Pipeline
            ValidationError::MissingVertexFunction => {
                write!(f, "render pipeline descriptor requires a vertex function")
            }
            ValidationError::UnsupportedRasterSampleCount(count) => {
                write!(
                    f,
                    "raster sample count {} is not supported by device",
                    count
                )
            }

            // Compute Pipeline
            ValidationError::MissingComputeFunction => {
                write!(f, "compute pipeline descriptor requires a compute function")
            }

            // Texture
            ValidationError::InvalidTextureDimensions {
                width,
                height,
                depth,
            } => {
                write!(
                    f,
                    "invalid texture dimensions: {}x{}x{} (dimensions must be > 0)",
                    width, height, depth
                )
            }
            ValidationError::InvalidMipmapCount {
                requested,
                max_allowed,
            } => {
                write!(
                    f,
                    "invalid mipmap count: {} exceeds maximum {} for texture dimensions",
                    requested, max_allowed
                )
            }
            ValidationError::UnsupportedTextureSampleCount(count) => {
                write!(
                    f,
                    "texture sample count {} is not supported by device",
                    count
                )
            }
            ValidationError::InvalidArrayLength => {
                write!(f, "array length must be > 0 for array textures")
            }

            // Sampler
            ValidationError::InvalidLodRange { min, max } => {
                write!(
                    f,
                    "invalid LOD clamp range: min ({}) must be <= max ({})",
                    min, max
                )
            }
            ValidationError::InvalidAnisotropy(value) => {
                write!(
                    f,
                    "invalid max anisotropy {}: must be >= 1 and a power of 2",
                    value
                )
            }

            // Heap
            ValidationError::InvalidHeapSize => {
                write!(f, "heap size must be > 0")
            }

            // Generic
            ValidationError::CreationFailed(Some(err)) => {
                write!(f, "resource creation failed: error code {}", err.code())
            }
            ValidationError::CreationFailed(None) => {
                write!(f, "resource creation failed")
            }
        }
    }
}

impl std::error::Error for ValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<mtl_foundation::Error> for ValidationError {
    fn from(err: mtl_foundation::Error) -> Self {
        ValidationError::CreationFailed(Some(err))
    }
}
