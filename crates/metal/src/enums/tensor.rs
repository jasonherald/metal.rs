//! Tensor enumerations.
//!
//! Corresponds to `Metal/MTLTensor.hpp`.

use metal_foundation::Integer;

/// Tensor data type.
///
/// C++ equivalent: `MTL::TensorDataType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TensorDataType(pub Integer);

impl TensorDataType {
    pub const NONE: Self = Self(0);
    pub const FLOAT32: Self = Self(3);
    pub const FLOAT16: Self = Self(16);
    pub const BFLOAT16: Self = Self(121);
    pub const INT8: Self = Self(45);
    pub const UINT8: Self = Self(49);
    pub const INT16: Self = Self(37);
    pub const UINT16: Self = Self(41);
    pub const INT32: Self = Self(29);
    pub const UINT32: Self = Self(33);
}

/// Tensor error codes.
///
/// C++ equivalent: `MTL::TensorError`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TensorError(pub Integer);

impl TensorError {
    pub const NONE: Self = Self(0);
    pub const INTERNAL_ERROR: Self = Self(1);
    pub const INVALID_DESCRIPTOR: Self = Self(2);
}

/// Tensor usage options.
///
/// C++ equivalent: `MTL::TensorUsage`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TensorUsage(pub metal_foundation::UInteger);

impl TensorUsage {
    pub const COMPUTE: Self = Self(1);
    pub const RENDER: Self = Self(1 << 1);
    pub const MACHINE_LEARNING: Self = Self(1 << 2);
}

impl std::ops::BitOr for TensorUsage {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for TensorUsage {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for TensorUsage {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_data_type_values() {
        assert_eq!(TensorDataType::NONE.0, 0);
        assert_eq!(TensorDataType::FLOAT32.0, 3);
        assert_eq!(TensorDataType::FLOAT16.0, 16);
        assert_eq!(TensorDataType::BFLOAT16.0, 121);
        assert_eq!(TensorDataType::INT8.0, 45);
        assert_eq!(TensorDataType::UINT8.0, 49);
        assert_eq!(TensorDataType::INT16.0, 37);
        assert_eq!(TensorDataType::UINT16.0, 41);
        assert_eq!(TensorDataType::INT32.0, 29);
        assert_eq!(TensorDataType::UINT32.0, 33);
    }

    #[test]
    fn test_tensor_error_values() {
        assert_eq!(TensorError::NONE.0, 0);
        assert_eq!(TensorError::INTERNAL_ERROR.0, 1);
        assert_eq!(TensorError::INVALID_DESCRIPTOR.0, 2);
    }

    #[test]
    fn test_tensor_usage_values() {
        assert_eq!(TensorUsage::COMPUTE.0, 1);
        assert_eq!(TensorUsage::RENDER.0, 2);
        assert_eq!(TensorUsage::MACHINE_LEARNING.0, 4);
    }

    #[test]
    fn test_tensor_usage_bitor() {
        let usage = TensorUsage::COMPUTE | TensorUsage::RENDER;
        assert_eq!(usage.0, 3);
    }
}
