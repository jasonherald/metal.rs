//! Device enumerations.
//!
//! Corresponds to `Metal/MTLDevice.hpp`.

use metal_foundation::{Integer, UInteger};

/// IO compression method.
///
/// C++ equivalent: `MTL::IOCompressionMethod`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IOCompressionMethod(pub Integer);

impl IOCompressionMethod {
    pub const ZLIB: Self = Self(0);
    pub const LZFSE: Self = Self(1);
    pub const LZ4: Self = Self(2);
    pub const LZMA: Self = Self(3);
    pub const LZ_BITMAP: Self = Self(4);
}

/// Feature set (legacy, deprecated).
///
/// C++ equivalent: `MTL::FeatureSet`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FeatureSet(pub UInteger);

impl FeatureSet {
    // iOS GPU Families
    pub const IOS_GPU_FAMILY1_V1: Self = Self(0);
    pub const IOS_GPU_FAMILY2_V1: Self = Self(1);
    pub const IOS_GPU_FAMILY1_V2: Self = Self(2);
    pub const IOS_GPU_FAMILY2_V2: Self = Self(3);
    pub const IOS_GPU_FAMILY3_V1: Self = Self(4);
    pub const IOS_GPU_FAMILY1_V3: Self = Self(5);
    pub const IOS_GPU_FAMILY2_V3: Self = Self(6);
    pub const IOS_GPU_FAMILY3_V2: Self = Self(7);
    pub const IOS_GPU_FAMILY1_V4: Self = Self(8);
    pub const IOS_GPU_FAMILY2_V4: Self = Self(9);
    pub const IOS_GPU_FAMILY3_V3: Self = Self(10);
    pub const IOS_GPU_FAMILY4_V1: Self = Self(11);
    pub const IOS_GPU_FAMILY1_V5: Self = Self(12);
    pub const IOS_GPU_FAMILY2_V5: Self = Self(13);
    pub const IOS_GPU_FAMILY3_V4: Self = Self(14);
    pub const IOS_GPU_FAMILY4_V2: Self = Self(15);
    pub const IOS_GPU_FAMILY5_V1: Self = Self(16);

    // macOS GPU Families
    pub const MACOS_GPU_FAMILY1_V1: Self = Self(10000);
    pub const OSX_GPU_FAMILY1_V1: Self = Self(10000);
    pub const MACOS_GPU_FAMILY1_V2: Self = Self(10001);
    pub const OSX_GPU_FAMILY1_V2: Self = Self(10001);
    pub const MACOS_READ_WRITE_TEXTURE_TIER2: Self = Self(10002);
    pub const OSX_READ_WRITE_TEXTURE_TIER2: Self = Self(10002);
    pub const MACOS_GPU_FAMILY1_V3: Self = Self(10003);
    pub const MACOS_GPU_FAMILY1_V4: Self = Self(10004);
    pub const MACOS_GPU_FAMILY2_V1: Self = Self(10005);

    // watchOS GPU Families
    pub const WATCHOS_GPU_FAMILY1_V1: Self = Self(20000);
    pub const WATCHOS_GPU_FAMILY2_V1: Self = Self(20001);

    // tvOS GPU Families
    pub const TVOS_GPU_FAMILY1_V1: Self = Self(30000);
    pub const TVOS_GPU_FAMILY1_V2: Self = Self(30001);
    pub const TVOS_GPU_FAMILY1_V3: Self = Self(30002);
    pub const TVOS_GPU_FAMILY2_V1: Self = Self(30003);
    pub const TVOS_GPU_FAMILY1_V4: Self = Self(30004);
    pub const TVOS_GPU_FAMILY2_V2: Self = Self(30005);
}

/// GPU family.
///
/// C++ equivalent: `MTL::GPUFamily`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct GPUFamily(pub Integer);

impl GPUFamily {
    pub const APPLE1: Self = Self(1001);
    pub const APPLE2: Self = Self(1002);
    pub const APPLE3: Self = Self(1003);
    pub const APPLE4: Self = Self(1004);
    pub const APPLE5: Self = Self(1005);
    pub const APPLE6: Self = Self(1006);
    pub const APPLE7: Self = Self(1007);
    pub const APPLE8: Self = Self(1008);
    pub const APPLE9: Self = Self(1009);
    pub const APPLE10: Self = Self(1010);
    pub const MAC1: Self = Self(2001);
    pub const MAC2: Self = Self(2002);
    pub const COMMON1: Self = Self(3001);
    pub const COMMON2: Self = Self(3002);
    pub const COMMON3: Self = Self(3003);
    pub const MAC_CATALYST1: Self = Self(4001);
    pub const MAC_CATALYST2: Self = Self(4002);
    pub const METAL3: Self = Self(5001);
    pub const METAL4: Self = Self(5002);
}

/// Device location.
///
/// C++ equivalent: `MTL::DeviceLocation`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DeviceLocation(pub UInteger);

impl DeviceLocation {
    pub const BUILT_IN: Self = Self(0);
    pub const SLOT: Self = Self(1);
    pub const EXTERNAL: Self = Self(2);
    pub const UNSPECIFIED: Self = Self(UInteger::MAX);
}

/// Read-write texture tier.
///
/// C++ equivalent: `MTL::ReadWriteTextureTier`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ReadWriteTextureTier(pub UInteger);

impl ReadWriteTextureTier {
    pub const NONE: Self = Self(0);
    pub const TIER1: Self = Self(1);
    pub const TIER2: Self = Self(2);
}

/// Argument buffers tier.
///
/// C++ equivalent: `MTL::ArgumentBuffersTier`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ArgumentBuffersTier(pub UInteger);

impl ArgumentBuffersTier {
    pub const TIER1: Self = Self(0);
    pub const TIER2: Self = Self(1);
}

/// Sparse texture region alignment mode.
///
/// C++ equivalent: `MTL::SparseTextureRegionAlignmentMode`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SparseTextureRegionAlignmentMode(pub UInteger);

impl SparseTextureRegionAlignmentMode {
    pub const OUTWARD: Self = Self(0);
    pub const INWARD: Self = Self(1);
}

/// Counter sampling point.
///
/// C++ equivalent: `MTL::CounterSamplingPoint`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CounterSamplingPoint(pub UInteger);

impl CounterSamplingPoint {
    pub const AT_STAGE_BOUNDARY: Self = Self(0);
    pub const AT_DRAW_BOUNDARY: Self = Self(1);
    pub const AT_DISPATCH_BOUNDARY: Self = Self(2);
    pub const AT_TILE_DISPATCH_BOUNDARY: Self = Self(3);
    pub const AT_BLIT_BOUNDARY: Self = Self(4);
}

/// Pipeline options (bitflags).
///
/// C++ equivalent: `MTL::PipelineOption`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PipelineOption(pub UInteger);

impl PipelineOption {
    pub const NONE: Self = Self(0);
    pub const ARGUMENT_INFO: Self = Self(1);
    pub const BINDING_INFO: Self = Self(1);
    pub const BUFFER_TYPE_INFO: Self = Self(1 << 1);
    pub const FAIL_ON_BINARY_ARCHIVE_MISS: Self = Self(1 << 2);

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

impl std::ops::BitOr for PipelineOption {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for PipelineOption {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for PipelineOption {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_family_values() {
        assert_eq!(GPUFamily::APPLE1.0, 1001);
        assert_eq!(GPUFamily::MAC1.0, 2001);
        assert_eq!(GPUFamily::METAL3.0, 5001);
        assert_eq!(GPUFamily::METAL4.0, 5002);
    }

    #[test]
    fn test_feature_set_values() {
        assert_eq!(FeatureSet::IOS_GPU_FAMILY1_V1.0, 0);
        assert_eq!(FeatureSet::MACOS_GPU_FAMILY1_V1.0, 10000);
        assert_eq!(FeatureSet::TVOS_GPU_FAMILY1_V1.0, 30000);
    }

    #[test]
    fn test_device_location_values() {
        assert_eq!(DeviceLocation::BUILT_IN.0, 0);
        assert_eq!(DeviceLocation::UNSPECIFIED.0, UInteger::MAX);
    }

    #[test]
    fn test_pipeline_option_bitor() {
        let opt = PipelineOption::ARGUMENT_INFO | PipelineOption::BUFFER_TYPE_INFO;
        assert!(opt.contains(PipelineOption::ARGUMENT_INFO));
        assert!(opt.contains(PipelineOption::BUFFER_TYPE_INFO));
    }
}
