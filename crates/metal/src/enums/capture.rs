//! Capture manager enumerations.
//!
//! Corresponds to `Metal/MTLCaptureManager.hpp`.

use metal_foundation::Integer;

/// Capture error codes.
///
/// C++ equivalent: `MTL::CaptureError`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CaptureError(pub Integer);

impl CaptureError {
    pub const NOT_SUPPORTED: Self = Self(1);
    pub const ALREADY_CAPTURING: Self = Self(2);
    pub const INVALID_DESCRIPTOR: Self = Self(3);
}

/// Capture destination.
///
/// C++ equivalent: `MTL::CaptureDestination`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CaptureDestination(pub Integer);

impl CaptureDestination {
    pub const DEVELOPER_TOOLS: Self = Self(1);
    pub const GPU_TRACE_DOCUMENT: Self = Self(2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_error_values() {
        assert_eq!(CaptureError::NOT_SUPPORTED.0, 1);
        assert_eq!(CaptureError::ALREADY_CAPTURING.0, 2);
        assert_eq!(CaptureError::INVALID_DESCRIPTOR.0, 3);
    }

    #[test]
    fn test_capture_destination_values() {
        assert_eq!(CaptureDestination::DEVELOPER_TOOLS.0, 1);
        assert_eq!(CaptureDestination::GPU_TRACE_DOCUMENT.0, 2);
    }
}
