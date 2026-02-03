//! Indirect command buffer enumerations.
//!
//! Corresponds to `Metal/MTLIndirectCommandBuffer.hpp`.

use mtl_foundation::UInteger;

/// Indirect command type (bitflags).
///
/// C++ equivalent: `MTL::IndirectCommandType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IndirectCommandType(pub UInteger);

impl IndirectCommandType {
    pub const NONE: Self = Self(0);
    pub const DRAW: Self = Self(1);
    pub const DRAW_INDEXED: Self = Self(1 << 1);
    pub const DRAW_PATCHES: Self = Self(1 << 2);
    pub const DRAW_INDEXED_PATCHES: Self = Self(1 << 3);
    pub const CONCURRENT_DISPATCH: Self = Self(1 << 5);
    pub const CONCURRENT_DISPATCH_THREADS: Self = Self(1 << 6);
    pub const DRAW_MESH_THREADGROUPS: Self = Self(1 << 7);
    pub const DRAW_MESH_THREADS: Self = Self(1 << 8);

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

impl std::ops::BitOr for IndirectCommandType {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for IndirectCommandType {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for IndirectCommandType {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indirect_command_type_values() {
        assert_eq!(IndirectCommandType::DRAW.0, 1);
        assert_eq!(IndirectCommandType::DRAW_INDEXED.0, 2);
        assert_eq!(IndirectCommandType::DRAW_MESH_THREADS.0, 256);
    }

    #[test]
    fn test_indirect_command_type_bitor() {
        let cmd = IndirectCommandType::DRAW | IndirectCommandType::DRAW_INDEXED;
        assert!(cmd.contains(IndirectCommandType::DRAW));
        assert!(cmd.contains(IndirectCommandType::DRAW_INDEXED));
    }
}
