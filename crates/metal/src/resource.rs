//! Resource protocol.
//!
//! Corresponds to `Metal/MTLResource.hpp`.
//!
//! This module defines the Resource trait which corresponds to the
//! `MTL::Resource` protocol in metal-cpp. Resource types like Buffer
//! and Texture already have these methods as inherent methods, so this
//! trait serves primarily for documentation and generic programming.

use crate::enums::{CPUCacheMode, HazardTrackingMode, PurgeableState, ResourceOptions, StorageMode};
use crate::Device;
use crate::heap::Heap;
use metal_foundation::UInteger;

/// Protocol for GPU-accessible resources.
///
/// C++ equivalent: `MTL::Resource`
///
/// This trait corresponds to the Objective-C protocol that resource types
/// conform to. In Rust, concrete types like [`Buffer`](crate::Buffer) and
/// [`Texture`](crate::Texture) implement these methods directly.
///
/// # Example
///
/// ```ignore
/// let buffer = device.new_buffer(1024, ResourceOptions::default()).unwrap();
/// println!("Label: {:?}", buffer.label());
/// println!("Storage mode: {:?}", buffer.storage_mode());
/// ```
pub trait Resource {
    /// Get the allocated size of this resource in bytes.
    ///
    /// C++ equivalent: `NS::UInteger allocatedSize() const`
    fn allocated_size(&self) -> UInteger;
    /// Get the CPU cache mode for this resource.
    ///
    /// C++ equivalent: `CPUCacheMode cpuCacheMode() const`
    fn cpu_cache_mode(&self) -> CPUCacheMode;

    /// Get the device that created this resource.
    ///
    /// C++ equivalent: `Device* device() const`
    fn device(&self) -> Device;

    /// Get the hazard tracking mode for this resource.
    ///
    /// C++ equivalent: `HazardTrackingMode hazardTrackingMode() const`
    fn hazard_tracking_mode(&self) -> HazardTrackingMode;

    /// Get the heap this resource was allocated from.
    ///
    /// Returns `None` if the resource was not allocated from a heap.
    ///
    /// C++ equivalent: `Heap* heap() const`
    fn heap(&self) -> Option<Heap>;

    /// Get the offset within the heap where this resource is allocated.
    ///
    /// C++ equivalent: `NS::UInteger heapOffset() const`
    fn heap_offset(&self) -> UInteger;

    /// Check if this resource can be aliased with other resources.
    ///
    /// C++ equivalent: `bool isAliasable()`
    fn is_aliasable(&self) -> bool;

    /// Get the debug label for this resource.
    ///
    /// C++ equivalent: `NS::String* label() const`
    fn label(&self) -> Option<String>;

    /// Make this resource aliasable with other resources.
    ///
    /// C++ equivalent: `void makeAliasable()`
    fn make_aliasable(&self);

    /// Get the resource options used to create this resource.
    ///
    /// C++ equivalent: `ResourceOptions resourceOptions() const`
    fn resource_options(&self) -> ResourceOptions;

    /// Set the debug label for this resource.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    fn set_label(&self, label: &str);

    /// Set the purgeable state for this resource.
    ///
    /// Returns the previous purgeable state.
    ///
    /// C++ equivalent: `PurgeableState setPurgeableState(PurgeableState)`
    fn set_purgeable_state(&self, state: PurgeableState) -> PurgeableState;

    /// Get the storage mode for this resource.
    ///
    /// C++ equivalent: `StorageMode storageMode() const`
    fn storage_mode(&self) -> StorageMode;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_trait() {
        // Verify the trait is object-safe
        fn _check_object_safety(_: &dyn Resource) {}
    }
}
