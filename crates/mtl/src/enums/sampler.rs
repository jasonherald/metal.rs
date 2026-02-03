//! Sampler enumerations.
//!
//! Corresponds to `Metal/MTLSampler.hpp`.

use mtl_foundation::UInteger;

/// Sampler minification/magnification filter.
///
/// C++ equivalent: `MTL::SamplerMinMagFilter`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SamplerMinMagFilter(pub UInteger);

impl SamplerMinMagFilter {
    pub const NEAREST: Self = Self(0);
    pub const LINEAR: Self = Self(1);
}

/// Sampler mipmap filter.
///
/// C++ equivalent: `MTL::SamplerMipFilter`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SamplerMipFilter(pub UInteger);

impl SamplerMipFilter {
    pub const NOT_MIPMAPPED: Self = Self(0);
    pub const NEAREST: Self = Self(1);
    pub const LINEAR: Self = Self(2);
}

/// Sampler address mode.
///
/// C++ equivalent: `MTL::SamplerAddressMode`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SamplerAddressMode(pub UInteger);

impl SamplerAddressMode {
    pub const CLAMP_TO_EDGE: Self = Self(0);
    pub const MIRROR_CLAMP_TO_EDGE: Self = Self(1);
    pub const REPEAT: Self = Self(2);
    pub const MIRROR_REPEAT: Self = Self(3);
    pub const CLAMP_TO_ZERO: Self = Self(4);
    pub const CLAMP_TO_BORDER_COLOR: Self = Self(5);
}

/// Sampler border color.
///
/// C++ equivalent: `MTL::SamplerBorderColor`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SamplerBorderColor(pub UInteger);

impl SamplerBorderColor {
    pub const TRANSPARENT_BLACK: Self = Self(0);
    pub const OPAQUE_BLACK: Self = Self(1);
    pub const OPAQUE_WHITE: Self = Self(2);
}

/// Sampler reduction mode.
///
/// C++ equivalent: `MTL::SamplerReductionMode`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SamplerReductionMode(pub UInteger);

impl SamplerReductionMode {
    pub const WEIGHTED_AVERAGE: Self = Self(0);
    pub const MINIMUM: Self = Self(1);
    pub const MAXIMUM: Self = Self(2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sampler_filter_values() {
        assert_eq!(SamplerMinMagFilter::NEAREST.0, 0);
        assert_eq!(SamplerMinMagFilter::LINEAR.0, 1);
    }

    #[test]
    fn test_sampler_address_mode_values() {
        assert_eq!(SamplerAddressMode::CLAMP_TO_EDGE.0, 0);
        assert_eq!(SamplerAddressMode::CLAMP_TO_BORDER_COLOR.0, 5);
    }
}
