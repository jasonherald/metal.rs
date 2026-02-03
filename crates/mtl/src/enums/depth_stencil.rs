//! Depth and stencil enumerations.
//!
//! Corresponds to `Metal/MTLDepthStencil.hpp`.

use mtl_foundation::UInteger;

/// Compare function for depth/stencil testing.
///
/// C++ equivalent: `MTL::CompareFunction`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CompareFunction(pub UInteger);

impl CompareFunction {
    pub const NEVER: Self = Self(0);
    pub const LESS: Self = Self(1);
    pub const EQUAL: Self = Self(2);
    pub const LESS_EQUAL: Self = Self(3);
    pub const GREATER: Self = Self(4);
    pub const NOT_EQUAL: Self = Self(5);
    pub const GREATER_EQUAL: Self = Self(6);
    pub const ALWAYS: Self = Self(7);
}

/// Stencil operation.
///
/// C++ equivalent: `MTL::StencilOperation`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StencilOperation(pub UInteger);

impl StencilOperation {
    pub const KEEP: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const REPLACE: Self = Self(2);
    pub const INCREMENT_CLAMP: Self = Self(3);
    pub const DECREMENT_CLAMP: Self = Self(4);
    pub const INVERT: Self = Self(5);
    pub const INCREMENT_WRAP: Self = Self(6);
    pub const DECREMENT_WRAP: Self = Self(7);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_function_values() {
        assert_eq!(CompareFunction::NEVER.0, 0);
        assert_eq!(CompareFunction::LESS.0, 1);
        assert_eq!(CompareFunction::ALWAYS.0, 7);
    }

    #[test]
    fn test_stencil_operation_values() {
        assert_eq!(StencilOperation::KEEP.0, 0);
        assert_eq!(StencilOperation::DECREMENT_WRAP.0, 7);
    }
}
