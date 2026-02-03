//! Acceleration structure enumerations.
//!
//! Corresponds to `Metal/MTLAccelerationStructure.hpp`.

use metal_foundation::{Integer, UInteger};

/// Matrix layout for acceleration structure transforms.
///
/// C++ equivalent: `MTL::MatrixLayout`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct MatrixLayout(pub Integer);

impl MatrixLayout {
    pub const COLUMN_MAJOR: Self = Self(0);
    pub const ROW_MAJOR: Self = Self(1);
}

/// Motion border mode for motion blur.
///
/// C++ equivalent: `MTL::MotionBorderMode`
///
/// Note: Uses u32, not UInteger.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct MotionBorderMode(pub u32);

impl MotionBorderMode {
    pub const CLAMP: Self = Self(0);
    pub const VANISH: Self = Self(1);
}

/// Curve type for curve geometry.
///
/// C++ equivalent: `MTL::CurveType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CurveType(pub Integer);

impl CurveType {
    pub const ROUND: Self = Self(0);
    pub const FLAT: Self = Self(1);
}

/// Curve basis for curve geometry.
///
/// C++ equivalent: `MTL::CurveBasis`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CurveBasis(pub Integer);

impl CurveBasis {
    pub const B_SPLINE: Self = Self(0);
    pub const CATMULL_ROM: Self = Self(1);
    pub const LINEAR: Self = Self(2);
    pub const BEZIER: Self = Self(3);
}

/// Curve end caps style.
///
/// C++ equivalent: `MTL::CurveEndCaps`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CurveEndCaps(pub Integer);

impl CurveEndCaps {
    pub const NONE: Self = Self(0);
    pub const DISK: Self = Self(1);
    pub const SPHERE: Self = Self(2);
}

/// Acceleration structure instance descriptor type.
///
/// C++ equivalent: `MTL::AccelerationStructureInstanceDescriptorType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AccelerationStructureInstanceDescriptorType(pub UInteger);

impl AccelerationStructureInstanceDescriptorType {
    pub const DEFAULT: Self = Self(0);
    pub const USER_ID: Self = Self(1);
    pub const MOTION: Self = Self(2);
    pub const INDIRECT: Self = Self(3);
    pub const INDIRECT_MOTION: Self = Self(4);
}

/// Transform type for acceleration structures.
///
/// C++ equivalent: `MTL::TransformType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TransformType(pub Integer);

impl TransformType {
    pub const PACKED_FLOAT4X3: Self = Self(0);
    pub const COMPONENT: Self = Self(1);
}

/// Acceleration structure refit options (bitflags).
///
/// C++ equivalent: `MTL::AccelerationStructureRefitOptions`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AccelerationStructureRefitOptions(pub UInteger);

impl AccelerationStructureRefitOptions {
    pub const NONE: Self = Self(0);
    pub const VERTEX_DATA: Self = Self(1);
    pub const PER_PRIMITIVE_DATA: Self = Self(1 << 1);

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

impl std::ops::BitOr for AccelerationStructureRefitOptions {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for AccelerationStructureRefitOptions {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for AccelerationStructureRefitOptions {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Acceleration structure usage options (bitflags).
///
/// C++ equivalent: `MTL::AccelerationStructureUsage`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AccelerationStructureUsage(pub UInteger);

impl AccelerationStructureUsage {
    pub const NONE: Self = Self(0);
    pub const REFIT: Self = Self(1);
    pub const PREFER_FAST_BUILD: Self = Self(1 << 1);
    pub const EXTENDED_LIMITS: Self = Self(1 << 2);
    pub const PREFER_FAST_INTERSECTION: Self = Self(1 << 4);
    pub const MINIMIZE_MEMORY: Self = Self(1 << 5);

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

impl std::ops::BitOr for AccelerationStructureUsage {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for AccelerationStructureUsage {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for AccelerationStructureUsage {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Acceleration structure instance options (bitflags).
///
/// C++ equivalent: `MTL::AccelerationStructureInstanceOptions`
///
/// Note: Uses u32, not UInteger.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AccelerationStructureInstanceOptions(pub u32);

impl AccelerationStructureInstanceOptions {
    pub const NONE: Self = Self(0);
    pub const DISABLE_TRIANGLE_CULLING: Self = Self(1);
    pub const TRIANGLE_FRONT_FACING_WINDING_COUNTER_CLOCKWISE: Self = Self(1 << 1);
    pub const OPAQUE: Self = Self(1 << 2);
    pub const NON_OPAQUE: Self = Self(1 << 3);

    /// Returns the raw bits.
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }

    /// Creates from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
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

impl std::ops::BitOr for AccelerationStructureInstanceOptions {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for AccelerationStructureInstanceOptions {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for AccelerationStructureInstanceOptions {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Intersection function signature options (bitflags).
///
/// C++ equivalent: `MTL::IntersectionFunctionSignature`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IntersectionFunctionSignature(pub UInteger);

impl IntersectionFunctionSignature {
    pub const NONE: Self = Self(0);
    pub const INSTANCING: Self = Self(1);
    pub const TRIANGLE_DATA: Self = Self(1 << 1);
    pub const WORLD_SPACE_DATA: Self = Self(1 << 2);
    pub const INSTANCE_MOTION: Self = Self(1 << 3);
    pub const PRIMITIVE_MOTION: Self = Self(1 << 4);
    pub const EXTENDED_LIMITS: Self = Self(1 << 5);
    pub const MAX_LEVELS: Self = Self(1 << 6);
    pub const CURVE_DATA: Self = Self(1 << 7);
    pub const INTERSECTION_FUNCTION_BUFFER: Self = Self(1 << 8);
    pub const USER_DATA: Self = Self(1 << 9);

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

impl std::ops::BitOr for IntersectionFunctionSignature {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for IntersectionFunctionSignature {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for IntersectionFunctionSignature {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_layout_values() {
        assert_eq!(MatrixLayout::COLUMN_MAJOR.0, 0);
        assert_eq!(MatrixLayout::ROW_MAJOR.0, 1);
    }

    #[test]
    fn test_motion_border_mode_values() {
        assert_eq!(MotionBorderMode::CLAMP.0, 0);
        assert_eq!(MotionBorderMode::VANISH.0, 1);
    }

    #[test]
    fn test_curve_basis_values() {
        assert_eq!(CurveBasis::B_SPLINE.0, 0);
        assert_eq!(CurveBasis::BEZIER.0, 3);
    }

    #[test]
    fn test_acceleration_structure_usage_bitor() {
        let usage =
            AccelerationStructureUsage::REFIT | AccelerationStructureUsage::PREFER_FAST_BUILD;
        assert!(usage.contains(AccelerationStructureUsage::REFIT));
        assert!(usage.contains(AccelerationStructureUsage::PREFER_FAST_BUILD));
    }

    #[test]
    fn test_acceleration_structure_instance_options_bitor() {
        let opts = AccelerationStructureInstanceOptions::DISABLE_TRIANGLE_CULLING
            | AccelerationStructureInstanceOptions::OPAQUE;
        assert!(opts.contains(AccelerationStructureInstanceOptions::DISABLE_TRIANGLE_CULLING));
        assert!(opts.contains(AccelerationStructureInstanceOptions::OPAQUE));
    }

    #[test]
    fn test_motion_border_mode_size() {
        assert_eq!(std::mem::size_of::<MotionBorderMode>(), 4);
    }

    #[test]
    fn test_instance_options_size() {
        assert_eq!(
            std::mem::size_of::<AccelerationStructureInstanceOptions>(),
            4
        );
    }
}
