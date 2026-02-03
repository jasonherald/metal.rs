//! Resource enumerations.
//!
//! Corresponds to `Metal/MTLResource.hpp`.

use metal_foundation::{Integer, UInteger};

/// Purgeable state of a resource.
///
/// C++ equivalent: `MTL::PurgeableState`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PurgeableState(pub UInteger);

impl PurgeableState {
    pub const KEEP_CURRENT: Self = Self(1);
    pub const NON_VOLATILE: Self = Self(2);
    pub const VOLATILE: Self = Self(3);
    pub const EMPTY: Self = Self(4);
}

/// CPU cache mode for resource memory.
///
/// C++ equivalent: `MTL::CPUCacheMode`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CPUCacheMode(pub UInteger);

impl CPUCacheMode {
    pub const DEFAULT_CACHE: Self = Self(0);
    pub const WRITE_COMBINED: Self = Self(1);
}

/// Storage mode for resource memory.
///
/// C++ equivalent: `MTL::StorageMode`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StorageMode(pub UInteger);

impl StorageMode {
    pub const SHARED: Self = Self(0);
    pub const MANAGED: Self = Self(1);
    pub const PRIVATE: Self = Self(2);
    pub const MEMORYLESS: Self = Self(3);
}

/// Hazard tracking mode for resources.
///
/// C++ equivalent: `MTL::HazardTrackingMode`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct HazardTrackingMode(pub UInteger);

impl HazardTrackingMode {
    pub const DEFAULT: Self = Self(0);
    pub const UNTRACKED: Self = Self(1);
    pub const TRACKED: Self = Self(2);
}

/// Sparse page size for sparse resources.
///
/// C++ equivalent: `MTL::SparsePageSize`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SparsePageSize(pub Integer);

impl SparsePageSize {
    pub const SIZE_16: Self = Self(101);
    pub const SIZE_64: Self = Self(102);
    pub const SIZE_256: Self = Self(103);
}

/// Buffer sparse tier.
///
/// C++ equivalent: `MTL::BufferSparseTier`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BufferSparseTier(pub Integer);

impl BufferSparseTier {
    pub const NONE: Self = Self(0);
    pub const TIER1: Self = Self(1);
}

/// Texture sparse tier.
///
/// C++ equivalent: `MTL::TextureSparseTier`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TextureSparseTier(pub Integer);

impl TextureSparseTier {
    pub const NONE: Self = Self(0);
    pub const TIER1: Self = Self(1);
    pub const TIER2: Self = Self(2);
}

/// Resource creation options (bitflags).
///
/// C++ equivalent: `MTL::ResourceOptions`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ResourceOptions(pub UInteger);

impl ResourceOptions {
    pub const CPU_CACHE_MODE_DEFAULT_CACHE: Self = Self(0);
    pub const CPU_CACHE_MODE_WRITE_COMBINED: Self = Self(1);
    pub const STORAGE_MODE_SHARED: Self = Self(0);
    pub const STORAGE_MODE_MANAGED: Self = Self(1 << 4);
    pub const STORAGE_MODE_PRIVATE: Self = Self(1 << 5);
    pub const STORAGE_MODE_MEMORYLESS: Self = Self(1 << 5);
    pub const HAZARD_TRACKING_MODE_DEFAULT: Self = Self(0);
    pub const HAZARD_TRACKING_MODE_UNTRACKED: Self = Self(1 << 8);
    pub const HAZARD_TRACKING_MODE_TRACKED: Self = Self(1 << 9);
    pub const OPTION_CPU_CACHE_MODE_DEFAULT: Self = Self(0);
    pub const OPTION_CPU_CACHE_MODE_WRITE_COMBINED: Self = Self(1);

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

impl std::ops::BitOr for ResourceOptions {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for ResourceOptions {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for ResourceOptions {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAndAssign for ResourceOptions {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

/// Sparse texture mapping mode.
///
/// C++ equivalent: `MTL::SparseTextureMappingMode`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SparseTextureMappingMode(pub UInteger);

impl SparseTextureMappingMode {
    /// Map the sparse texture region.
    pub const MAP: Self = Self(0);
    /// Unmap the sparse texture region.
    pub const UNMAP: Self = Self(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_purgeable_state_values() {
        assert_eq!(PurgeableState::KEEP_CURRENT.0, 1);
        assert_eq!(PurgeableState::NON_VOLATILE.0, 2);
        assert_eq!(PurgeableState::VOLATILE.0, 3);
        assert_eq!(PurgeableState::EMPTY.0, 4);
    }

    #[test]
    fn test_storage_mode_values() {
        assert_eq!(StorageMode::SHARED.0, 0);
        assert_eq!(StorageMode::MANAGED.0, 1);
        assert_eq!(StorageMode::PRIVATE.0, 2);
        assert_eq!(StorageMode::MEMORYLESS.0, 3);
    }

    #[test]
    fn test_resource_options_bitor() {
        let opts =
            ResourceOptions::CPU_CACHE_MODE_WRITE_COMBINED | ResourceOptions::STORAGE_MODE_PRIVATE;
        assert!(opts.contains(ResourceOptions::CPU_CACHE_MODE_WRITE_COMBINED));
    }
}
