//! Allocation protocol.
//!
//! Corresponds to `Metal/MTLAllocation.hpp`.
//!
//! This module defines the Allocation trait which corresponds to the
//! `MTL::Allocation` protocol in metal-cpp. Resource types like Buffer,
//! Texture, and Heap already have `allocated_size()` as an inherent method,
//! so this trait serves primarily for documentation and generic programming.

use metal_foundation::UInteger;

/// Protocol for types that track their allocated memory size.
///
/// C++ equivalent: `MTL::Allocation`
///
/// This trait corresponds to the Objective-C protocol that all Metal resource
/// types conform to. In Rust, concrete types like [`Buffer`](crate::Buffer),
/// [`Texture`](crate::Texture), and [`Heap`](crate::Heap) implement this
/// method directly.
///
/// # Example
///
/// ```ignore
/// // All resource types have allocated_size() directly
/// let buffer = device.new_buffer(1024, ResourceOptions::default()).unwrap();
/// println!("Allocated: {} bytes", buffer.allocated_size());
/// ```
pub trait Allocation {
    /// Get the allocated size of this resource in bytes.
    ///
    /// This may be larger than the requested size due to alignment requirements.
    ///
    /// C++ equivalent: `NS::UInteger allocatedSize() const`
    fn allocated_size(&self) -> UInteger;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation_trait() {
        // Verify the trait is object-safe
        fn _check_object_safety(_: &dyn Allocation) {}
    }
}
