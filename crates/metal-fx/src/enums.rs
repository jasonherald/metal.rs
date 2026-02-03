//! MetalFX enumerations.
//!
//! Corresponds to enums in `MetalFX/MTLFXSpatialScaler.hpp`.

use metal_foundation::Integer;

/// Color processing mode for spatial scaling.
///
/// C++ equivalent: `MTLFX::SpatialScalerColorProcessingMode`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SpatialScalerColorProcessingMode(pub Integer);

impl SpatialScalerColorProcessingMode {
    /// Perceptual color processing (sRGB).
    pub const PERCEPTUAL: Self = Self(0);

    /// Linear color processing.
    pub const LINEAR: Self = Self(1);

    /// HDR color processing.
    pub const HDR: Self = Self(2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_processing_mode_values() {
        assert_eq!(SpatialScalerColorProcessingMode::PERCEPTUAL.0, 0);
        assert_eq!(SpatialScalerColorProcessingMode::LINEAR.0, 1);
        assert_eq!(SpatialScalerColorProcessingMode::HDR.0, 2);
    }
}
