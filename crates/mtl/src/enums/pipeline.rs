//! Pipeline enumerations.
//!
//! Corresponds to `Metal/MTLRenderPipeline.hpp` and `Metal/MTLPipeline.hpp`.

use mtl_foundation::{Integer, UInteger};

/// Blend factor for color blending.
///
/// C++ equivalent: `MTL::BlendFactor`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BlendFactor(pub UInteger);

impl BlendFactor {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const SOURCE_COLOR: Self = Self(2);
    pub const ONE_MINUS_SOURCE_COLOR: Self = Self(3);
    pub const SOURCE_ALPHA: Self = Self(4);
    pub const ONE_MINUS_SOURCE_ALPHA: Self = Self(5);
    pub const DESTINATION_COLOR: Self = Self(6);
    pub const ONE_MINUS_DESTINATION_COLOR: Self = Self(7);
    pub const DESTINATION_ALPHA: Self = Self(8);
    pub const ONE_MINUS_DESTINATION_ALPHA: Self = Self(9);
    pub const SOURCE_ALPHA_SATURATED: Self = Self(10);
    pub const BLEND_COLOR: Self = Self(11);
    pub const ONE_MINUS_BLEND_COLOR: Self = Self(12);
    pub const BLEND_ALPHA: Self = Self(13);
    pub const ONE_MINUS_BLEND_ALPHA: Self = Self(14);
    pub const SOURCE1_COLOR: Self = Self(15);
    pub const ONE_MINUS_SOURCE1_COLOR: Self = Self(16);
    pub const SOURCE1_ALPHA: Self = Self(17);
    pub const ONE_MINUS_SOURCE1_ALPHA: Self = Self(18);
    pub const UNSPECIALIZED: Self = Self(19);
}

/// Blend operation.
///
/// C++ equivalent: `MTL::BlendOperation`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BlendOperation(pub UInteger);

impl BlendOperation {
    pub const ADD: Self = Self(0);
    pub const SUBTRACT: Self = Self(1);
    pub const REVERSE_SUBTRACT: Self = Self(2);
    pub const MIN: Self = Self(3);
    pub const MAX: Self = Self(4);
    pub const UNSPECIALIZED: Self = Self(5);
}

/// Primitive topology class.
///
/// C++ equivalent: `MTL::PrimitiveTopologyClass`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PrimitiveTopologyClass(pub UInteger);

impl PrimitiveTopologyClass {
    pub const UNSPECIFIED: Self = Self(0);
    pub const POINT: Self = Self(1);
    pub const LINE: Self = Self(2);
    pub const TRIANGLE: Self = Self(3);
}

/// Tessellation partition mode.
///
/// C++ equivalent: `MTL::TessellationPartitionMode`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TessellationPartitionMode(pub UInteger);

impl TessellationPartitionMode {
    pub const POW2: Self = Self(0);
    pub const INTEGER: Self = Self(1);
    pub const FRACTIONAL_ODD: Self = Self(2);
    pub const FRACTIONAL_EVEN: Self = Self(3);
}

/// Tessellation factor step function.
///
/// C++ equivalent: `MTL::TessellationFactorStepFunction`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TessellationFactorStepFunction(pub UInteger);

impl TessellationFactorStepFunction {
    pub const CONSTANT: Self = Self(0);
    pub const PER_PATCH: Self = Self(1);
    pub const PER_INSTANCE: Self = Self(2);
    pub const PER_PATCH_AND_PER_INSTANCE: Self = Self(3);
}

/// Tessellation factor format.
///
/// C++ equivalent: `MTL::TessellationFactorFormat`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TessellationFactorFormat(pub UInteger);

impl TessellationFactorFormat {
    pub const HALF: Self = Self(0);
}

/// Tessellation control point index type.
///
/// C++ equivalent: `MTL::TessellationControlPointIndexType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TessellationControlPointIndexType(pub UInteger);

impl TessellationControlPointIndexType {
    pub const NONE: Self = Self(0);
    pub const UINT16: Self = Self(1);
    pub const UINT32: Self = Self(2);
}

/// Color write mask (bitflags).
///
/// C++ equivalent: `MTL::ColorWriteMask`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ColorWriteMask(pub UInteger);

impl ColorWriteMask {
    pub const NONE: Self = Self(0);
    pub const RED: Self = Self(1 << 3);
    pub const GREEN: Self = Self(1 << 2);
    pub const BLUE: Self = Self(1 << 1);
    pub const ALPHA: Self = Self(1);
    pub const ALL: Self = Self(15);
    pub const UNSPECIALIZED: Self = Self(1 << 4);

    /// Returns the raw bits.
    #[inline]
    pub const fn bits(&self) -> UInteger {
        self.0
    }

    /// Creates from raw bits.
    #[inline]
    pub const fn from_bits(bits: UInteger) -> Self {
        Self(bits)
    }

    /// Check if empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Check if contains all flags in other.
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for ColorWriteMask {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for ColorWriteMask {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for ColorWriteMask {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Buffer mutability for pipeline state.
///
/// C++ equivalent: `MTL::Mutability`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Mutability(pub UInteger);

impl Mutability {
    pub const DEFAULT: Self = Self(0);
    pub const MUTABLE: Self = Self(1);
    pub const IMMUTABLE: Self = Self(2);
}

/// Shader validation mode.
///
/// C++ equivalent: `MTL::ShaderValidation`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ShaderValidation(pub Integer);

impl ShaderValidation {
    pub const DEFAULT: Self = Self(0);
    pub const ENABLED: Self = Self(1);
    pub const DISABLED: Self = Self(2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blend_factor_values() {
        assert_eq!(BlendFactor::ZERO.0, 0);
        assert_eq!(BlendFactor::ONE.0, 1);
        assert_eq!(BlendFactor::SOURCE_ALPHA.0, 4);
        assert_eq!(BlendFactor::UNSPECIALIZED.0, 19);
    }

    #[test]
    fn test_blend_operation_values() {
        assert_eq!(BlendOperation::ADD.0, 0);
        assert_eq!(BlendOperation::MAX.0, 4);
    }

    #[test]
    fn test_color_write_mask_values() {
        assert_eq!(ColorWriteMask::NONE.0, 0);
        assert_eq!(ColorWriteMask::ALL.0, 15);
        assert_eq!(ColorWriteMask::RED.0, 8);
        assert_eq!(ColorWriteMask::GREEN.0, 4);
        assert_eq!(ColorWriteMask::BLUE.0, 2);
        assert_eq!(ColorWriteMask::ALPHA.0, 1);
    }

    #[test]
    fn test_color_write_mask_bitor() {
        let mask = ColorWriteMask::RED | ColorWriteMask::GREEN;
        assert!(mask.contains(ColorWriteMask::RED));
        assert!(mask.contains(ColorWriteMask::GREEN));
    }
}
