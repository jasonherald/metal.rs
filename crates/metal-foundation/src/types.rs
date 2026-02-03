//! Foundation type definitions.
//!
//! Corresponds to `Foundation/NSTypes.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! using TimeInterval = double;
//! using Integer = std::intptr_t;
//! using UInteger = std::uintptr_t;
//!
//! const Integer  IntegerMax = INTPTR_MAX;
//! const Integer  IntegerMin = INTPTR_MIN;
//! const UInteger UIntegerMax = UINTPTR_MAX;
//!
//! struct OperatingSystemVersion {
//!     Integer majorVersion;
//!     Integer minorVersion;
//!     Integer patchVersion;
//! } _NS_PACKED;
//! }
//! ```

/// Time interval in seconds.
///
/// C++ equivalent: `NS::TimeInterval`
pub type TimeInterval = f64;

/// Signed integer type (pointer-sized).
///
/// C++ equivalent: `NS::Integer` (`std::intptr_t`)
pub type Integer = isize;

/// Unsigned integer type (pointer-sized).
///
/// C++ equivalent: `NS::UInteger` (`std::uintptr_t`)
pub type UInteger = usize;

/// Maximum value for Integer.
///
/// C++ equivalent: `NS::IntegerMax`
pub const INTEGER_MAX: Integer = isize::MAX;

/// Minimum value for Integer.
///
/// C++ equivalent: `NS::IntegerMin`
pub const INTEGER_MIN: Integer = isize::MIN;

/// Maximum value for UInteger.
///
/// C++ equivalent: `NS::UIntegerMax`
pub const UINTEGER_MAX: UInteger = usize::MAX;

/// Operating system version structure.
///
/// C++ equivalent: `NS::OperatingSystemVersion`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct OperatingSystemVersion {
    /// Major version number.
    pub major_version: Integer,
    /// Minor version number.
    pub minor_version: Integer,
    /// Patch version number.
    pub patch_version: Integer,
}

impl OperatingSystemVersion {
    /// Create a new operating system version.
    #[inline]
    pub const fn new(major: Integer, minor: Integer, patch: Integer) -> Self {
        Self {
            major_version: major,
            minor_version: minor,
            patch_version: patch,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_sizes() {
        assert_eq!(std::mem::size_of::<Integer>(), std::mem::size_of::<isize>());
        assert_eq!(
            std::mem::size_of::<UInteger>(),
            std::mem::size_of::<usize>()
        );
        assert_eq!(
            std::mem::size_of::<TimeInterval>(),
            std::mem::size_of::<f64>()
        );
    }

    #[test]
    fn test_operating_system_version() {
        let version = OperatingSystemVersion::new(14, 0, 1);
        let major = { version.major_version };
        let minor = { version.minor_version };
        let patch = { version.patch_version };
        assert_eq!(major, 14);
        assert_eq!(minor, 0);
        assert_eq!(patch, 1);
    }
}
