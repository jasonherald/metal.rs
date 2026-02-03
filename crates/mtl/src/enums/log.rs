//! Log state enumerations.
//!
//! Corresponds to `Metal/MTLLogState.hpp`.

use mtl_foundation::{Integer, UInteger};

// ============================================================================
// LogLevel
// ============================================================================

/// Log verbosity level.
///
/// C++ equivalent: `MTL::LogLevel`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct LogLevel(pub Integer);

impl LogLevel {
    /// Undefined log level.
    pub const UNDEFINED: Self = Self(0);
    /// Debug log level.
    pub const DEBUG: Self = Self(1);
    /// Info log level.
    pub const INFO: Self = Self(2);
    /// Notice log level.
    pub const NOTICE: Self = Self(3);
    /// Error log level.
    pub const ERROR: Self = Self(4);
    /// Fault log level.
    pub const FAULT: Self = Self(5);
}

// ============================================================================
// LogStateError
// ============================================================================

/// Errors that can occur when creating a log state.
///
/// C++ equivalent: `MTL::LogStateError`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct LogStateError(pub UInteger);

impl LogStateError {
    /// Invalid buffer size error.
    pub const INVALID_SIZE: Self = Self(1);
    /// Invalid configuration error.
    pub const INVALID: Self = Self(2);
}

// ============================================================================
// FunctionLogType
// ============================================================================

/// Type of function log entry.
///
/// C++ equivalent: `MTL::FunctionLogType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FunctionLogType(pub UInteger);

impl FunctionLogType {
    /// Validation log entry.
    pub const VALIDATION: Self = Self(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_values() {
        assert_eq!(LogLevel::UNDEFINED.0, 0);
        assert_eq!(LogLevel::DEBUG.0, 1);
        assert_eq!(LogLevel::INFO.0, 2);
        assert_eq!(LogLevel::NOTICE.0, 3);
        assert_eq!(LogLevel::ERROR.0, 4);
        assert_eq!(LogLevel::FAULT.0, 5);
    }

    #[test]
    fn test_log_state_error_values() {
        assert_eq!(LogStateError::INVALID_SIZE.0, 1);
        assert_eq!(LogStateError::INVALID.0, 2);
    }

    #[test]
    fn test_function_log_type_values() {
        assert_eq!(FunctionLogType::VALIDATION.0, 0);
    }
}
