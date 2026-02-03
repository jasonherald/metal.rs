//! Objective-C runtime types.
//!
//! Corresponds to `Foundation/NSObjCRuntime.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! _NS_ENUM(Integer, ComparisonResult) {
//!     OrderedAscending = -1L,
//!     OrderedSame,
//!     OrderedDescending
//! };
//!
//! const Integer NotFound = IntegerMax;
//! }
//! ```

use crate::types::{INTEGER_MAX, Integer};

/// Result of a comparison operation.
///
/// C++ equivalent: `NS::ComparisonResult`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ComparisonResult(pub Integer);

impl ComparisonResult {
    /// Left operand is smaller than right operand.
    ///
    /// C++ equivalent: `NS::OrderedAscending`
    pub const ORDERED_ASCENDING: Self = Self(-1);

    /// Both operands are equal.
    ///
    /// C++ equivalent: `NS::OrderedSame`
    pub const ORDERED_SAME: Self = Self(0);

    /// Left operand is greater than right operand.
    ///
    /// C++ equivalent: `NS::OrderedDescending`
    pub const ORDERED_DESCENDING: Self = Self(1);

    /// Returns the raw integer value.
    #[inline]
    pub const fn raw(&self) -> Integer {
        self.0
    }

    /// Creates from a raw integer value.
    #[inline]
    pub const fn from_raw(value: Integer) -> Self {
        Self(value)
    }
}

impl From<Integer> for ComparisonResult {
    #[inline]
    fn from(value: Integer) -> Self {
        Self(value)
    }
}

impl From<ComparisonResult> for Integer {
    #[inline]
    fn from(value: ComparisonResult) -> Self {
        value.0
    }
}

/// Sentinel value indicating "not found".
///
/// C++ equivalent: `NS::NotFound`
pub const NOT_FOUND: Integer = INTEGER_MAX;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparison_result_values() {
        assert_eq!(ComparisonResult::ORDERED_ASCENDING.raw(), -1);
        assert_eq!(ComparisonResult::ORDERED_SAME.raw(), 0);
        assert_eq!(ComparisonResult::ORDERED_DESCENDING.raw(), 1);
    }

    #[test]
    fn test_not_found() {
        assert_eq!(NOT_FOUND, isize::MAX);
    }
}
