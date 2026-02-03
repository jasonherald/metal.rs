//! Range type for Foundation.
//!
//! Corresponds to `Foundation/NSRange.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! struct Range {
//!     static Range Make(UInteger loc, UInteger len);
//!     Range(UInteger loc, UInteger len);
//!     bool     Equal(const Range& range) const;
//!     bool     LocationInRange(UInteger loc) const;
//!     UInteger Max() const;
//!     UInteger location;
//!     UInteger length;
//! } _NS_PACKED;
//! }
//! ```

use crate::types::UInteger;

/// A structure used to describe a portion of a series.
///
/// C++ equivalent: `NS::Range`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Range {
    /// The start index of the range.
    pub location: UInteger,
    /// The number of items in the range.
    pub length: UInteger,
}

impl Range {
    /// Create a new range.
    ///
    /// C++ equivalent: `NS::Range::Range(UInteger loc, UInteger len)`
    #[inline]
    pub const fn new(location: UInteger, length: UInteger) -> Self {
        Self { location, length }
    }

    /// Create a new range (static factory method).
    ///
    /// C++ equivalent: `NS::Range::Make(UInteger loc, UInteger len)`
    #[inline]
    pub const fn make(location: UInteger, length: UInteger) -> Self {
        Self::new(location, length)
    }

    /// Check if two ranges are equal.
    ///
    /// C++ equivalent: `NS::Range::Equal(const Range& range) const`
    #[inline]
    pub const fn equal(&self, other: &Range) -> bool {
        self.location == other.location && self.length == other.length
    }

    /// Check if a location is within this range.
    ///
    /// C++ equivalent: `NS::Range::LocationInRange(UInteger loc) const`
    #[inline]
    pub const fn location_in_range(&self, loc: UInteger) -> bool {
        loc >= self.location && (loc - self.location) < self.length
    }

    /// Get the maximum value in the range (location + length).
    ///
    /// C++ equivalent: `NS::Range::Max() const`
    #[inline]
    pub const fn max(&self) -> UInteger {
        self.location + self.length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_new() {
        let range = Range::new(10, 5);
        let location = { range.location };
        let length = { range.length };
        assert_eq!(location, 10);
        assert_eq!(length, 5);
    }

    #[test]
    fn test_range_equal() {
        let r1 = Range::new(10, 5);
        let r2 = Range::new(10, 5);
        let r3 = Range::new(10, 6);

        assert!(r1.equal(&r2));
        assert!(!r1.equal(&r3));
    }

    #[test]
    fn test_location_in_range() {
        let range = Range::new(10, 5);

        assert!(!range.location_in_range(9));
        assert!(range.location_in_range(10));
        assert!(range.location_in_range(14));
        assert!(!range.location_in_range(15));
    }

    #[test]
    fn test_range_max() {
        let range = Range::new(10, 5);
        assert_eq!(range.max(), 15);
    }
}
