//! Heap enumerations.
//!
//! Corresponds to `Metal/MTLHeap.hpp`.

use metal_foundation::Integer;

/// Heap type.
///
/// C++ equivalent: `MTL::HeapType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct HeapType(pub Integer);

impl HeapType {
    pub const AUTOMATIC: Self = Self(0);
    pub const PLACEMENT: Self = Self(1);
    pub const SPARSE: Self = Self(2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_type_values() {
        assert_eq!(HeapType::AUTOMATIC.0, 0);
        assert_eq!(HeapType::PLACEMENT.0, 1);
        assert_eq!(HeapType::SPARSE.0, 2);
    }
}
