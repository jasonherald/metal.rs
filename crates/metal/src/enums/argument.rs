//! Argument enumerations.
//!
//! Corresponds to `Metal/MTLArgument.hpp`.

use metal_foundation::{Integer, UInteger};

/// Index type for indexed drawing.
///
/// C++ equivalent: `MTL::IndexType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IndexType(pub UInteger);

impl IndexType {
    pub const UINT16: Self = Self(0);
    pub const UINT32: Self = Self(1);
}

/// Binding type for shader arguments.
///
/// C++ equivalent: `MTL::BindingType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BindingType(pub Integer);

impl BindingType {
    pub const BUFFER: Self = Self(0);
    pub const THREADGROUP_MEMORY: Self = Self(1);
    pub const TEXTURE: Self = Self(2);
    pub const SAMPLER: Self = Self(3);
    pub const IMAGEBLOCK_DATA: Self = Self(16);
    pub const IMAGEBLOCK: Self = Self(17);
    pub const VISIBLE_FUNCTION_TABLE: Self = Self(24);
    pub const PRIMITIVE_ACCELERATION_STRUCTURE: Self = Self(25);
    pub const INSTANCE_ACCELERATION_STRUCTURE: Self = Self(26);
    pub const INTERSECTION_FUNCTION_TABLE: Self = Self(27);
    pub const OBJECT_PAYLOAD: Self = Self(34);
    pub const TENSOR: Self = Self(37);
}

/// Argument type for shader arguments.
///
/// C++ equivalent: `MTL::ArgumentType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ArgumentType(pub UInteger);

impl ArgumentType {
    pub const BUFFER: Self = Self(0);
    pub const THREADGROUP_MEMORY: Self = Self(1);
    pub const TEXTURE: Self = Self(2);
    pub const SAMPLER: Self = Self(3);
    pub const IMAGEBLOCK_DATA: Self = Self(16);
    pub const IMAGEBLOCK: Self = Self(17);
    pub const VISIBLE_FUNCTION_TABLE: Self = Self(24);
    pub const PRIMITIVE_ACCELERATION_STRUCTURE: Self = Self(25);
    pub const INSTANCE_ACCELERATION_STRUCTURE: Self = Self(26);
    pub const INTERSECTION_FUNCTION_TABLE: Self = Self(27);
}

/// Binding access mode.
///
/// C++ equivalent: `MTL::BindingAccess`
///
/// Note: ArgumentAccess is an alias with the same values.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BindingAccess(pub UInteger);

impl BindingAccess {
    pub const READ_ONLY: Self = Self(0);
    pub const READ_WRITE: Self = Self(1);
    pub const WRITE_ONLY: Self = Self(2);
}

/// Argument access mode (alias for BindingAccess).
///
/// C++ equivalent: `MTL::ArgumentAccess`
pub type ArgumentAccess = BindingAccess;

// Re-export the access constants under the ArgumentAccess name for compatibility
impl BindingAccess {
    /// Alias for READ_ONLY (ArgumentAccess compatibility)
    pub const ARGUMENT_ACCESS_READ_ONLY: Self = Self::READ_ONLY;
    /// Alias for READ_WRITE (ArgumentAccess compatibility)
    pub const ARGUMENT_ACCESS_READ_WRITE: Self = Self::READ_WRITE;
    /// Alias for WRITE_ONLY (ArgumentAccess compatibility)
    pub const ARGUMENT_ACCESS_WRITE_ONLY: Self = Self::WRITE_ONLY;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_type_values() {
        assert_eq!(IndexType::UINT16.0, 0);
        assert_eq!(IndexType::UINT32.0, 1);
    }

    #[test]
    fn test_binding_type_values() {
        assert_eq!(BindingType::BUFFER.0, 0);
        assert_eq!(BindingType::TEXTURE.0, 2);
        assert_eq!(BindingType::TENSOR.0, 37);
    }

    #[test]
    fn test_binding_access_values() {
        assert_eq!(BindingAccess::READ_ONLY.0, 0);
        assert_eq!(BindingAccess::READ_WRITE.0, 1);
        assert_eq!(BindingAccess::WRITE_ONLY.0, 2);
    }
}
