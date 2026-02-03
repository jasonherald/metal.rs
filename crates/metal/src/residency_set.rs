//! Metal residency set for VRAM management.
//!
//! Corresponds to `Metal/MTLResidencySet.hpp`.
//!
//! Residency sets manage which allocations are resident in GPU memory.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, sel};

use crate::Device;

// ============================================================================
// ResidencySetDescriptor
// ============================================================================

/// Descriptor for creating a residency set.
///
/// C++ equivalent: `MTL::ResidencySetDescriptor`
#[repr(transparent)]
pub struct ResidencySetDescriptor(pub(crate) NonNull<c_void>);

impl ResidencySetDescriptor {
    /// Create a new residency set descriptor.
    ///
    /// C++ equivalent: `ResidencySetDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLResidencySetDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal ResidencySetDescriptor.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the initial capacity.
    ///
    /// C++ equivalent: `NS::UInteger initialCapacity() const`
    #[inline]
    pub fn initial_capacity(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(initialCapacity)) }
    }

    /// Set the initial capacity.
    ///
    /// C++ equivalent: `void setInitialCapacity(NS::UInteger)`
    pub fn set_initial_capacity(&self, capacity: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setInitialCapacity:), capacity);
        }
    }

    /// Get the label.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                metal_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Set the label.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }
}

impl Default for ResidencySetDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create ResidencySetDescriptor")
    }
}

impl Clone for ResidencySetDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("copy should succeed")
        }
    }
}

impl Drop for ResidencySetDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for ResidencySetDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ResidencySetDescriptor {}
unsafe impl Sync for ResidencySetDescriptor {}

// ============================================================================
// ResidencySet
// ============================================================================

/// A residency set for managing GPU memory residency.
///
/// C++ equivalent: `MTL::ResidencySet`
#[repr(transparent)]
pub struct ResidencySet(pub(crate) NonNull<c_void>);

impl ResidencySet {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal ResidencySet.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the device that created this residency set.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            // Retain for our reference
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            Device::from_raw(ptr).expect("device should be valid")
        }
    }

    /// Get the label.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                metal_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Get the allocated size in bytes.
    ///
    /// C++ equivalent: `uint64_t allocatedSize() const`
    #[inline]
    pub fn allocated_size(&self) -> u64 {
        unsafe { msg_send_0(self.as_ptr(), sel!(allocatedSize)) }
    }

    /// Get the allocation count.
    ///
    /// C++ equivalent: `NS::UInteger allocationCount() const`
    #[inline]
    pub fn allocation_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(allocationCount)) }
    }

    /// Get all allocations as a raw NS::Array pointer.
    ///
    /// C++ equivalent: `NS::Array* allAllocations() const`
    #[inline]
    pub fn all_allocations_ptr(&self) -> *const c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(allAllocations)) }
    }

    /// Add an allocation using a raw pointer.
    ///
    /// C++ equivalent: `void addAllocation(const MTL::Allocation*)`
    pub fn add_allocation_ptr(&self, allocation: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(addAllocation:), allocation);
        }
    }

    /// Add multiple allocations.
    ///
    /// C++ equivalent: `void addAllocations(const MTL::Allocation* const[], NS::UInteger)`
    pub fn add_allocations_ptr(&self, allocations: *const *const c_void, count: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(addAllocations:count:),
                allocations,
                count,
            );
        }
    }

    /// Remove an allocation using a raw pointer.
    ///
    /// C++ equivalent: `void removeAllocation(const MTL::Allocation*)`
    pub fn remove_allocation_ptr(&self, allocation: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(removeAllocation:), allocation);
        }
    }

    /// Remove multiple allocations.
    ///
    /// C++ equivalent: `void removeAllocations(const MTL::Allocation* const[], NS::UInteger)`
    pub fn remove_allocations_ptr(&self, allocations: *const *const c_void, count: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(removeAllocations:count:),
                allocations,
                count,
            );
        }
    }

    /// Remove all allocations.
    ///
    /// C++ equivalent: `void removeAllAllocations()`
    pub fn remove_all_allocations(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(removeAllAllocations));
        }
    }

    /// Check if the set contains an allocation.
    ///
    /// C++ equivalent: `bool containsAllocation(const MTL::Allocation*)`
    pub fn contains_allocation_ptr(&self, allocation: *const c_void) -> bool {
        unsafe { msg_send_1(self.as_ptr(), sel!(containsAllocation:), allocation) }
    }

    /// Request residency for all allocations in the set.
    ///
    /// C++ equivalent: `void requestResidency()`
    pub fn request_residency(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(requestResidency));
        }
    }

    /// End residency for all allocations in the set.
    ///
    /// C++ equivalent: `void endResidency()`
    pub fn end_residency(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(endResidency));
        }
    }

    /// Commit changes to the residency set.
    ///
    /// C++ equivalent: `void commit()`
    pub fn commit(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(commit));
        }
    }
}

impl Clone for ResidencySet {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ResidencySet {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for ResidencySet {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ResidencySet {}
unsafe impl Sync for ResidencySet {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_residency_set_descriptor_creation() {
        let descriptor = ResidencySetDescriptor::new();
        assert!(descriptor.is_some());
    }

    #[test]
    fn test_residency_set_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<ResidencySetDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_residency_set_size() {
        assert_eq!(
            std::mem::size_of::<ResidencySet>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_residency_set_descriptor_initial_capacity() {
        let descriptor = ResidencySetDescriptor::new().unwrap();
        descriptor.set_initial_capacity(100);
        assert_eq!(descriptor.initial_capacity(), 100);
    }
}
