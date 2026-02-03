//! Counter enumerations.
//!
//! Corresponds to `Metal/MTLCounters.hpp`.

use metal_foundation::Integer;

/// Counter sample buffer error codes.
///
/// C++ equivalent: `MTL::CounterSampleBufferError`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CounterSampleBufferError(pub Integer);

impl CounterSampleBufferError {
    pub const OUT_OF_MEMORY: Self = Self(0);
    pub const INVALID: Self = Self(1);
    pub const INTERNAL: Self = Self(2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_sample_buffer_error_values() {
        assert_eq!(CounterSampleBufferError::OUT_OF_MEMORY.0, 0);
        assert_eq!(CounterSampleBufferError::INVALID.0, 1);
        assert_eq!(CounterSampleBufferError::INTERNAL.0, 2);
    }
}
