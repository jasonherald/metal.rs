//! Metal heap resources.
//!
//! Corresponds to `Metal/MTLHeap.hpp`.
//!
//! Heaps allow you to allocate multiple resources from a single memory allocation.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{
    CPUCacheMode, HazardTrackingMode, HeapType, PurgeableState, ResourceOptions, SparsePageSize,
    StorageMode,
};

/// A memory pool from which resources can be allocated.
///
/// C++ equivalent: `MTL::Heap`
#[repr(transparent)]
pub struct Heap(pub(crate) NonNull<c_void>);

impl Heap {
    /// Create a Heap from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal heap object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the heap.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the label for this heap.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                mtl_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Set the label for this heap.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = mtl_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the device that created this heap.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("heap has no device")
        }
    }

    /// Get the storage mode.
    ///
    /// C++ equivalent: `StorageMode storageMode() const`
    #[inline]
    pub fn storage_mode(&self) -> StorageMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(storageMode)) }
    }

    /// Get the CPU cache mode.
    ///
    /// C++ equivalent: `CPUCacheMode cpuCacheMode() const`
    #[inline]
    pub fn cpu_cache_mode(&self) -> CPUCacheMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(cpuCacheMode)) }
    }

    /// Get the hazard tracking mode.
    ///
    /// C++ equivalent: `HazardTrackingMode hazardTrackingMode() const`
    #[inline]
    pub fn hazard_tracking_mode(&self) -> HazardTrackingMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(hazardTrackingMode)) }
    }

    /// Get the resource options.
    ///
    /// C++ equivalent: `ResourceOptions resourceOptions() const`
    #[inline]
    pub fn resource_options(&self) -> ResourceOptions {
        unsafe { msg_send_0(self.as_ptr(), sel!(resourceOptions)) }
    }

    /// Get the heap size in bytes.
    ///
    /// C++ equivalent: `NS::UInteger size() const`
    #[inline]
    pub fn size(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(size)) }
    }

    /// Get the currently used size in bytes.
    ///
    /// C++ equivalent: `NS::UInteger usedSize() const`
    #[inline]
    pub fn used_size(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(usedSize)) }
    }

    /// Get the current size available for new allocations.
    ///
    /// C++ equivalent: `NS::UInteger currentAllocatedSize() const`
    #[inline]
    pub fn current_allocated_size(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(currentAllocatedSize)) }
    }

    /// Get the maximum size available for new allocations.
    ///
    /// C++ equivalent: `NS::UInteger maxAvailableSize(NS::UInteger alignment)`
    #[inline]
    pub fn max_available_size(&self, alignment: UInteger) -> UInteger {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(maxAvailableSizeWithAlignment:),
                alignment,
            )
        }
    }

    /// Get the heap type.
    ///
    /// C++ equivalent: `HeapType type() const`
    #[inline]
    pub fn heap_type(&self) -> HeapType {
        unsafe { msg_send_0(self.as_ptr(), sel!(type)) }
    }

    // =========================================================================
    // Purgeable State
    // =========================================================================

    /// Set the purgeable state.
    ///
    /// C++ equivalent: `PurgeableState setPurgeableState(PurgeableState state)`
    #[inline]
    pub fn set_purgeable_state(&self, state: PurgeableState) -> PurgeableState {
        unsafe { msg_send_1(self.as_ptr(), sel!(setPurgeableState:), state) }
    }

    // =========================================================================
    // Resource Creation
    // =========================================================================

    /// Create a buffer from the heap.
    ///
    /// C++ equivalent: `Buffer* newBuffer(NS::UInteger length, MTL::ResourceOptions options)`
    pub fn new_buffer(
        &self,
        length: UInteger,
        options: ResourceOptions,
    ) -> Option<crate::buffer::Buffer> {
        unsafe {
            let ptr: *mut c_void = mtl_sys::msg_send_2(
                self.as_ptr(),
                sel!(newBufferWithLength: options:),
                length,
                options,
            );
            crate::buffer::Buffer::from_raw(ptr)
        }
    }

    /// Create a buffer from the heap with offset.
    ///
    /// C++ equivalent: `Buffer* newBuffer(NS::UInteger length, MTL::ResourceOptions options, NS::UInteger offset)`
    pub fn new_buffer_with_offset(
        &self,
        length: UInteger,
        options: ResourceOptions,
        offset: UInteger,
    ) -> Option<crate::buffer::Buffer> {
        unsafe {
            let ptr: *mut c_void = mtl_sys::msg_send_3(
                self.as_ptr(),
                sel!(newBufferWithLength: options: offset:),
                length,
                options,
                offset,
            );
            crate::buffer::Buffer::from_raw(ptr)
        }
    }

    /// Create a texture from the heap.
    ///
    /// C++ equivalent: `Texture* newTexture(const TextureDescriptor*)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn new_texture(&self, descriptor: *const c_void) -> Option<crate::texture::Texture> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(newTextureWithDescriptor:), descriptor);
            crate::texture::Texture::from_raw(ptr)
        }
    }

    /// Create a texture from the heap with offset.
    ///
    /// C++ equivalent: `Texture* newTexture(const TextureDescriptor*, NS::UInteger offset)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn new_texture_with_offset(
        &self,
        descriptor: *const c_void,
        offset: UInteger,
    ) -> Option<crate::texture::Texture> {
        unsafe {
            let ptr: *mut c_void = mtl_sys::msg_send_2(
                self.as_ptr(),
                sel!(newTextureWithDescriptor: offset:),
                descriptor,
                offset,
            );
            crate::texture::Texture::from_raw(ptr)
        }
    }

    // =========================================================================
    // Acceleration Structure Creation
    // =========================================================================

    /// Create an acceleration structure from the heap with a given size.
    ///
    /// C++ equivalent: `AccelerationStructure* newAccelerationStructure(NS::UInteger size)`
    pub fn new_acceleration_structure_with_size(
        &self,
        size: UInteger,
    ) -> Option<crate::acceleration::AccelerationStructure> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(newAccelerationStructureWithSize:), size);
            crate::acceleration::AccelerationStructure::from_raw(ptr)
        }
    }

    /// Create an acceleration structure from the heap with a descriptor.
    ///
    /// C++ equivalent: `AccelerationStructure* newAccelerationStructure(const AccelerationStructureDescriptor*)`
    pub fn new_acceleration_structure(
        &self,
        descriptor: &crate::acceleration::AccelerationStructureDescriptor,
    ) -> Option<crate::acceleration::AccelerationStructure> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newAccelerationStructureWithDescriptor:),
                descriptor.as_ptr(),
            );
            crate::acceleration::AccelerationStructure::from_raw(ptr)
        }
    }

    /// Create an acceleration structure from the heap with size and offset.
    ///
    /// C++ equivalent: `AccelerationStructure* newAccelerationStructure(NS::UInteger size, NS::UInteger offset)`
    pub fn new_acceleration_structure_with_size_and_offset(
        &self,
        size: UInteger,
        offset: UInteger,
    ) -> Option<crate::acceleration::AccelerationStructure> {
        unsafe {
            let ptr: *mut c_void = mtl_sys::msg_send_2(
                self.as_ptr(),
                sel!(newAccelerationStructureWithSize: offset:),
                size,
                offset,
            );
            crate::acceleration::AccelerationStructure::from_raw(ptr)
        }
    }

    /// Create an acceleration structure from the heap with descriptor and offset.
    ///
    /// C++ equivalent: `AccelerationStructure* newAccelerationStructure(const AccelerationStructureDescriptor*, NS::UInteger offset)`
    pub fn new_acceleration_structure_with_offset(
        &self,
        descriptor: &crate::acceleration::AccelerationStructureDescriptor,
        offset: UInteger,
    ) -> Option<crate::acceleration::AccelerationStructure> {
        unsafe {
            let ptr: *mut c_void = mtl_sys::msg_send_2(
                self.as_ptr(),
                sel!(newAccelerationStructureWithDescriptor: offset:),
                descriptor.as_ptr(),
                offset,
            );
            crate::acceleration::AccelerationStructure::from_raw(ptr)
        }
    }
}

impl Clone for Heap {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for Heap {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for Heap {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for Heap {}
unsafe impl Sync for Heap {}

impl std::fmt::Debug for Heap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Heap")
            .field("size", &self.size())
            .field("used_size", &self.used_size())
            .field("label", &self.label())
            .finish()
    }
}

// ============================================================================
// Heap Descriptor
// ============================================================================

/// A configuration for a heap.
///
/// C++ equivalent: `MTL::HeapDescriptor`
#[repr(transparent)]
pub struct HeapDescriptor(pub(crate) NonNull<c_void>);

impl HeapDescriptor {
    /// Create a new heap descriptor.
    ///
    /// C++ equivalent: `static HeapDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLHeapDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a HeapDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal heap descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the heap size.
    ///
    /// C++ equivalent: `NS::UInteger size() const`
    #[inline]
    pub fn size(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(size)) }
    }

    /// Set the heap size.
    ///
    /// C++ equivalent: `void setSize(NS::UInteger)`
    #[inline]
    pub fn set_size(&self, size: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setSize:), size);
        }
    }

    /// Get the storage mode.
    ///
    /// C++ equivalent: `StorageMode storageMode() const`
    #[inline]
    pub fn storage_mode(&self) -> StorageMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(storageMode)) }
    }

    /// Set the storage mode.
    ///
    /// C++ equivalent: `void setStorageMode(StorageMode)`
    #[inline]
    pub fn set_storage_mode(&self, mode: StorageMode) {
        unsafe {
            msg_send_1::<(), StorageMode>(self.as_ptr(), sel!(setStorageMode:), mode);
        }
    }

    /// Get the CPU cache mode.
    ///
    /// C++ equivalent: `CPUCacheMode cpuCacheMode() const`
    #[inline]
    pub fn cpu_cache_mode(&self) -> CPUCacheMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(cpuCacheMode)) }
    }

    /// Set the CPU cache mode.
    ///
    /// C++ equivalent: `void setCpuCacheMode(CPUCacheMode)`
    #[inline]
    pub fn set_cpu_cache_mode(&self, mode: CPUCacheMode) {
        unsafe {
            msg_send_1::<(), CPUCacheMode>(self.as_ptr(), sel!(setCpuCacheMode:), mode);
        }
    }

    /// Get the hazard tracking mode.
    ///
    /// C++ equivalent: `HazardTrackingMode hazardTrackingMode() const`
    #[inline]
    pub fn hazard_tracking_mode(&self) -> HazardTrackingMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(hazardTrackingMode)) }
    }

    /// Set the hazard tracking mode.
    ///
    /// C++ equivalent: `void setHazardTrackingMode(HazardTrackingMode)`
    #[inline]
    pub fn set_hazard_tracking_mode(&self, mode: HazardTrackingMode) {
        unsafe {
            msg_send_1::<(), HazardTrackingMode>(self.as_ptr(), sel!(setHazardTrackingMode:), mode);
        }
    }

    /// Get the resource options.
    ///
    /// C++ equivalent: `ResourceOptions resourceOptions() const`
    #[inline]
    pub fn resource_options(&self) -> ResourceOptions {
        unsafe { msg_send_0(self.as_ptr(), sel!(resourceOptions)) }
    }

    /// Set the resource options.
    ///
    /// C++ equivalent: `void setResourceOptions(ResourceOptions)`
    #[inline]
    pub fn set_resource_options(&self, options: ResourceOptions) {
        unsafe {
            msg_send_1::<(), ResourceOptions>(self.as_ptr(), sel!(setResourceOptions:), options);
        }
    }

    /// Get the heap type.
    ///
    /// C++ equivalent: `HeapType type() const`
    #[inline]
    pub fn heap_type(&self) -> HeapType {
        unsafe { msg_send_0(self.as_ptr(), sel!(type)) }
    }

    /// Set the heap type.
    ///
    /// C++ equivalent: `void setType(HeapType)`
    #[inline]
    pub fn set_heap_type(&self, heap_type: HeapType) {
        unsafe {
            msg_send_1::<(), HeapType>(self.as_ptr(), sel!(setType:), heap_type);
        }
    }

    // =========================================================================
    // Sparse Page Size Properties
    // =========================================================================

    /// Get the sparse page size.
    ///
    /// C++ equivalent: `SparsePageSize sparsePageSize() const`
    #[inline]
    pub fn sparse_page_size(&self) -> SparsePageSize {
        unsafe { msg_send_0(self.as_ptr(), sel!(sparsePageSize)) }
    }

    /// Set the sparse page size.
    ///
    /// C++ equivalent: `void setSparsePageSize(SparsePageSize)`
    #[inline]
    pub fn set_sparse_page_size(&self, size: SparsePageSize) {
        unsafe {
            msg_send_1::<(), SparsePageSize>(self.as_ptr(), sel!(setSparsePageSize:), size);
        }
    }

    /// Get the maximum compatible placement sparse page size.
    ///
    /// C++ equivalent: `SparsePageSize maxCompatiblePlacementSparsePageSize() const`
    #[inline]
    pub fn max_compatible_placement_sparse_page_size(&self) -> SparsePageSize {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxCompatiblePlacementSparsePageSize)) }
    }

    /// Set the maximum compatible placement sparse page size.
    ///
    /// C++ equivalent: `void setMaxCompatiblePlacementSparsePageSize(SparsePageSize)`
    #[inline]
    pub fn set_max_compatible_placement_sparse_page_size(&self, size: SparsePageSize) {
        unsafe {
            msg_send_1::<(), SparsePageSize>(
                self.as_ptr(),
                sel!(setMaxCompatiblePlacementSparsePageSize:),
                size,
            );
        }
    }
}

impl Default for HeapDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create heap descriptor")
    }
}

impl Clone for HeapDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy heap descriptor")
        }
    }
}

impl Drop for HeapDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for HeapDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for HeapDescriptor {}
unsafe impl Sync for HeapDescriptor {}

impl std::fmt::Debug for HeapDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HeapDescriptor")
            .field("size", &self.size())
            .field("storage_mode", &self.storage_mode())
            .field("heap_type", &self.heap_type())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_size() {
        assert_eq!(
            std::mem::size_of::<Heap>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_heap_descriptor_creation() {
        let desc = HeapDescriptor::new();
        assert!(desc.is_some());
    }
}
