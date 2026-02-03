//! IO command buffer and queue enumerations.
//!
//! Corresponds to `Metal/MTLIOCommandBuffer.hpp`, `Metal/MTLIOCommandQueue.hpp`,
//! and `Metal/MTLIOCompressor.hpp`.

use metal_foundation::Integer;

/// IO command buffer status.
///
/// C++ equivalent: `MTL::IOStatus`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IOStatus(pub Integer);

impl IOStatus {
    pub const PENDING: Self = Self(0);
    pub const CANCELLED: Self = Self(1);
    pub const ERROR: Self = Self(2);
    pub const COMPLETE: Self = Self(3);
}

/// IO command queue priority.
///
/// C++ equivalent: `MTL::IOPriority`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IOPriority(pub Integer);

impl IOPriority {
    pub const HIGH: Self = Self(0);
    pub const NORMAL: Self = Self(1);
    pub const LOW: Self = Self(2);
}

/// IO command queue type.
///
/// C++ equivalent: `MTL::IOCommandQueueType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IOCommandQueueType(pub Integer);

impl IOCommandQueueType {
    pub const CONCURRENT: Self = Self(0);
    pub const SERIAL: Self = Self(1);
}

/// IO error codes.
///
/// C++ equivalent: `MTL::IOError`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IOError(pub Integer);

impl IOError {
    pub const URL_INVALID: Self = Self(1);
    pub const INTERNAL: Self = Self(2);
}

/// IO compression status.
///
/// C++ equivalent: `MTL::IOCompressionStatus`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IOCompressionStatus(pub Integer);

impl IOCompressionStatus {
    pub const COMPLETE: Self = Self(0);
    pub const ERROR: Self = Self(1);
}

// Note: IOCompressionMethod is defined in device.rs as it's part of MTLDevice.hpp

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_status_values() {
        assert_eq!(IOStatus::PENDING.0, 0);
        assert_eq!(IOStatus::CANCELLED.0, 1);
        assert_eq!(IOStatus::ERROR.0, 2);
        assert_eq!(IOStatus::COMPLETE.0, 3);
    }

    #[test]
    fn test_io_priority_values() {
        assert_eq!(IOPriority::HIGH.0, 0);
        assert_eq!(IOPriority::NORMAL.0, 1);
        assert_eq!(IOPriority::LOW.0, 2);
    }

    #[test]
    fn test_io_command_queue_type_values() {
        assert_eq!(IOCommandQueueType::CONCURRENT.0, 0);
        assert_eq!(IOCommandQueueType::SERIAL.0, 1);
    }

    #[test]
    fn test_io_error_values() {
        assert_eq!(IOError::URL_INVALID.0, 1);
        assert_eq!(IOError::INTERNAL.0, 2);
    }

    #[test]
    fn test_io_compression_status_values() {
        assert_eq!(IOCompressionStatus::COMPLETE.0, 0);
        assert_eq!(IOCompressionStatus::ERROR.0, 1);
    }
}
