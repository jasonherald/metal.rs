//! Function descriptor enumerations.
//!
//! Corresponds to `Metal/MTLFunctionDescriptor.hpp`.

use mtl_foundation::UInteger;

/// Function options (bitflags).
///
/// C++ equivalent: `MTL::FunctionOptions`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FunctionOptions(pub UInteger);

impl FunctionOptions {
    pub const NONE: Self = Self(0);
    pub const COMPILE_TO_BINARY: Self = Self(1);
    /// Alias for STORE_FUNCTION_IN_METAL_SCRIPT
    pub const STORE_FUNCTION_IN_METAL_PIPELINES_SCRIPT: Self = Self(1 << 1);
    pub const STORE_FUNCTION_IN_METAL_SCRIPT: Self = Self(1 << 1);
    pub const FAIL_ON_BINARY_ARCHIVE_MISS: Self = Self(1 << 2);
    pub const PIPELINE_INDEPENDENT: Self = Self(1 << 3);

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

impl std::ops::BitOr for FunctionOptions {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for FunctionOptions {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for FunctionOptions {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_options_values() {
        assert_eq!(FunctionOptions::NONE.0, 0);
        assert_eq!(FunctionOptions::COMPILE_TO_BINARY.0, 1);
        assert_eq!(FunctionOptions::STORE_FUNCTION_IN_METAL_SCRIPT.0, 2);
        assert_eq!(FunctionOptions::FAIL_ON_BINARY_ARCHIVE_MISS.0, 4);
        assert_eq!(FunctionOptions::PIPELINE_INDEPENDENT.0, 8);
    }

    #[test]
    fn test_function_options_bitor() {
        let opt = FunctionOptions::COMPILE_TO_BINARY | FunctionOptions::FAIL_ON_BINARY_ARCHIVE_MISS;
        assert!(opt.contains(FunctionOptions::COMPILE_TO_BINARY));
        assert!(opt.contains(FunctionOptions::FAIL_ON_BINARY_ARCHIVE_MISS));
    }
}
