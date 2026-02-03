//! Blit command encoder enumerations.
//!
//! Corresponds to `Metal/MTLBlitCommandEncoder.hpp`.

use metal_foundation::UInteger;

/// Blit options (bitflags).
///
/// C++ equivalent: `MTL::BlitOption`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BlitOption(pub UInteger);

impl BlitOption {
    pub const NONE: Self = Self(0);
    pub const DEPTH_FROM_DEPTH_STENCIL: Self = Self(1);
    pub const STENCIL_FROM_DEPTH_STENCIL: Self = Self(1 << 1);
    pub const ROW_LINEAR_PVRTC: Self = Self(1 << 2);

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

impl std::ops::BitOr for BlitOption {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for BlitOption {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for BlitOption {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blit_option_values() {
        assert_eq!(BlitOption::NONE.0, 0);
        assert_eq!(BlitOption::DEPTH_FROM_DEPTH_STENCIL.0, 1);
        assert_eq!(BlitOption::STENCIL_FROM_DEPTH_STENCIL.0, 2);
        assert_eq!(BlitOption::ROW_LINEAR_PVRTC.0, 4);
    }

    #[test]
    fn test_blit_option_bitor() {
        let opt = BlitOption::DEPTH_FROM_DEPTH_STENCIL | BlitOption::STENCIL_FROM_DEPTH_STENCIL;
        assert!(opt.contains(BlitOption::DEPTH_FROM_DEPTH_STENCIL));
        assert!(opt.contains(BlitOption::STENCIL_FROM_DEPTH_STENCIL));
    }
}
