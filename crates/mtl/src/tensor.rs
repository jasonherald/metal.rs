//! Metal tensor types for ML operations.
//!
//! Corresponds to `Metal/MTLTensor.hpp`.
//!
//! Tensors represent multi-dimensional arrays for machine learning operations.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Integer, Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, msg_send_2, msg_send_4, sel};

use crate::Buffer;
use crate::enums::{
    CPUCacheMode, HazardTrackingMode, ResourceOptions, StorageMode, TensorDataType, TensorUsage,
};
use crate::types::ResourceID;

// ============================================================================
// TensorExtents
// ============================================================================

/// Extents (dimensions/strides) for a tensor.
///
/// C++ equivalent: `MTL::TensorExtents`
#[repr(transparent)]
pub struct TensorExtents(pub(crate) NonNull<c_void>);

impl TensorExtents {
    /// Create a new tensor extents.
    ///
    /// C++ equivalent: `TensorExtents* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLTensorExtents")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a tensor extents with specific rank and values.
    ///
    /// C++ equivalent: `TensorExtents* alloc()->init(NS::UInteger, const NS::Integer*)`
    pub fn with_values(values: &[Integer]) -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLTensorExtents")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_2(
                ptr,
                sel!(initWithRank:values:),
                values.len() as UInteger,
                values.as_ptr(),
            );
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal TensorExtents.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the rank (number of dimensions).
    ///
    /// C++ equivalent: `NS::UInteger rank() const`
    #[inline]
    pub fn rank(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(rank)) }
    }

    /// Get the extent at a specific dimension index.
    ///
    /// C++ equivalent: `NS::Integer extentAtDimensionIndex(NS::UInteger)`
    #[inline]
    pub fn extent_at_dimension_index(&self, dimension_index: UInteger) -> Integer {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(extentAtDimensionIndex:),
                dimension_index,
            )
        }
    }
}

impl Clone for TensorExtents {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for TensorExtents {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for TensorExtents {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for TensorExtents {}
unsafe impl Sync for TensorExtents {}

// ============================================================================
// TensorDescriptor
// ============================================================================

/// Descriptor for creating a tensor.
///
/// C++ equivalent: `MTL::TensorDescriptor`
#[repr(transparent)]
pub struct TensorDescriptor(pub(crate) NonNull<c_void>);

impl TensorDescriptor {
    /// Create a new tensor descriptor.
    ///
    /// C++ equivalent: `TensorDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLTensorDescriptor")?;
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
    /// The pointer must be a valid Metal TensorDescriptor.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the data type.
    ///
    /// C++ equivalent: `TensorDataType dataType() const`
    #[inline]
    pub fn data_type(&self) -> TensorDataType {
        unsafe { msg_send_0(self.as_ptr(), sel!(dataType)) }
    }

    /// Set the data type.
    ///
    /// C++ equivalent: `void setDataType(MTL::TensorDataType)`
    pub fn set_data_type(&self, data_type: TensorDataType) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDataType:), data_type);
        }
    }

    /// Get the dimensions.
    ///
    /// C++ equivalent: `TensorExtents* dimensions() const`
    pub fn dimensions(&self) -> Option<TensorExtents> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(dimensions));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            TensorExtents::from_raw(ptr)
        }
    }

    /// Set the dimensions.
    ///
    /// C++ equivalent: `void setDimensions(const MTL::TensorExtents*)`
    pub fn set_dimensions(&self, dimensions: &TensorExtents) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setDimensions:), dimensions.as_ptr());
        }
    }

    /// Get the strides.
    ///
    /// C++ equivalent: `TensorExtents* strides() const`
    pub fn strides(&self) -> Option<TensorExtents> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(strides));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            TensorExtents::from_raw(ptr)
        }
    }

    /// Set the strides.
    ///
    /// C++ equivalent: `void setStrides(const MTL::TensorExtents*)`
    pub fn set_strides(&self, strides: &TensorExtents) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setStrides:), strides.as_ptr());
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
    /// C++ equivalent: `void setStorageMode(MTL::StorageMode)`
    pub fn set_storage_mode(&self, storage_mode: StorageMode) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setStorageMode:), storage_mode);
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
    /// C++ equivalent: `void setCpuCacheMode(MTL::CPUCacheMode)`
    pub fn set_cpu_cache_mode(&self, cpu_cache_mode: CPUCacheMode) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setCpuCacheMode:), cpu_cache_mode);
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
    /// C++ equivalent: `void setHazardTrackingMode(MTL::HazardTrackingMode)`
    pub fn set_hazard_tracking_mode(&self, hazard_tracking_mode: HazardTrackingMode) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setHazardTrackingMode:),
                hazard_tracking_mode,
            );
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
    /// C++ equivalent: `void setResourceOptions(MTL::ResourceOptions)`
    pub fn set_resource_options(&self, resource_options: ResourceOptions) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setResourceOptions:), resource_options);
        }
    }

    /// Get the usage.
    ///
    /// C++ equivalent: `TensorUsage usage() const`
    #[inline]
    pub fn usage(&self) -> TensorUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(usage)) }
    }

    /// Set the usage.
    ///
    /// C++ equivalent: `void setUsage(MTL::TensorUsage)`
    pub fn set_usage(&self, usage: TensorUsage) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setUsage:), usage);
        }
    }
}

impl Default for TensorDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create TensorDescriptor")
    }
}

impl Clone for TensorDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("copy should succeed")
        }
    }
}

impl Drop for TensorDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for TensorDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for TensorDescriptor {}
unsafe impl Sync for TensorDescriptor {}

// ============================================================================
// Tensor
// ============================================================================

/// A multi-dimensional array for machine learning.
///
/// C++ equivalent: `MTL::Tensor`
#[repr(transparent)]
pub struct Tensor(pub(crate) NonNull<c_void>);

impl Tensor {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal Tensor.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the backing buffer.
    ///
    /// C++ equivalent: `Buffer* buffer() const`
    pub fn buffer(&self) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(buffer));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            Buffer::from_raw(ptr)
        }
    }

    /// Get the buffer offset.
    ///
    /// C++ equivalent: `NS::UInteger bufferOffset() const`
    #[inline]
    pub fn buffer_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(bufferOffset)) }
    }

    /// Get the data type.
    ///
    /// C++ equivalent: `TensorDataType dataType() const`
    #[inline]
    pub fn data_type(&self) -> TensorDataType {
        unsafe { msg_send_0(self.as_ptr(), sel!(dataType)) }
    }

    /// Get the dimensions.
    ///
    /// C++ equivalent: `TensorExtents* dimensions() const`
    pub fn dimensions(&self) -> Option<TensorExtents> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(dimensions));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            TensorExtents::from_raw(ptr)
        }
    }

    /// Get the strides.
    ///
    /// C++ equivalent: `TensorExtents* strides() const`
    pub fn strides(&self) -> Option<TensorExtents> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(strides));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            TensorExtents::from_raw(ptr)
        }
    }

    /// Get the usage.
    ///
    /// C++ equivalent: `TensorUsage usage() const`
    #[inline]
    pub fn usage(&self) -> TensorUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(usage)) }
    }

    /// Get the GPU resource ID.
    ///
    /// C++ equivalent: `ResourceID gpuResourceID() const`
    #[inline]
    pub fn gpu_resource_id(&self) -> ResourceID {
        unsafe { msg_send_0(self.as_ptr(), sel!(gpuResourceID)) }
    }

    /// Get bytes from the tensor.
    ///
    /// C++ equivalent: `void getBytes(void*, const TensorExtents*, const TensorExtents*, const TensorExtents*)`
    pub fn get_bytes(
        &self,
        bytes: *mut c_void,
        strides: &TensorExtents,
        slice_origin: &TensorExtents,
        slice_dimensions: &TensorExtents,
    ) {
        unsafe {
            let _: () = msg_send_4(
                self.as_ptr(),
                sel!(getBytes:strides:fromSliceOrigin:sliceDimensions:),
                bytes,
                strides.as_ptr(),
                slice_origin.as_ptr(),
                slice_dimensions.as_ptr(),
            );
        }
    }

    /// Replace a slice in the tensor.
    ///
    /// C++ equivalent: `void replaceSliceOrigin(const TensorExtents*, const TensorExtents*, const void*, const TensorExtents*)`
    pub fn replace_slice_origin(
        &self,
        slice_origin: &TensorExtents,
        slice_dimensions: &TensorExtents,
        bytes: *const c_void,
        strides: &TensorExtents,
    ) {
        unsafe {
            let _: () = msg_send_4(
                self.as_ptr(),
                sel!(replaceSliceOrigin:sliceDimensions:withBytes:strides:),
                slice_origin.as_ptr(),
                slice_dimensions.as_ptr(),
                bytes,
                strides.as_ptr(),
            );
        }
    }
}

impl Clone for Tensor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for Tensor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for Tensor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for Tensor {}
unsafe impl Sync for Tensor {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_extents_creation() {
        // TensorExtents may not be available on all systems
        let _extents = TensorExtents::new();
    }

    #[test]
    fn test_tensor_extents_with_values() {
        let values = [2, 3, 4];
        let extents = TensorExtents::with_values(&values);
        // May not be available on all systems
        if let Some(e) = extents {
            assert_eq!(e.rank(), 3);
            assert_eq!(e.extent_at_dimension_index(0), 2);
            assert_eq!(e.extent_at_dimension_index(1), 3);
            assert_eq!(e.extent_at_dimension_index(2), 4);
        }
    }

    #[test]
    fn test_tensor_extents_size() {
        assert_eq!(
            std::mem::size_of::<TensorExtents>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_tensor_descriptor_creation() {
        // TensorDescriptor may not be available on all systems
        let _descriptor = TensorDescriptor::new();
    }

    #[test]
    fn test_tensor_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<TensorDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_tensor_size() {
        assert_eq!(
            std::mem::size_of::<Tensor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_tensor_descriptor_data_type() {
        // TensorDescriptor may not be available on all systems
        if let Some(descriptor) = TensorDescriptor::new() {
            descriptor.set_data_type(TensorDataType::FLOAT32);
            assert_eq!(descriptor.data_type(), TensorDataType::FLOAT32);
        }
    }

    #[test]
    fn test_tensor_descriptor_usage() {
        // TensorDescriptor may not be available on all systems
        if let Some(descriptor) = TensorDescriptor::new() {
            descriptor.set_usage(TensorUsage::COMPUTE | TensorUsage::MACHINE_LEARNING);
            let usage = descriptor.usage();
            assert!((usage.0 & TensorUsage::COMPUTE.0) != 0);
            assert!((usage.0 & TensorUsage::MACHINE_LEARNING.0) != 0);
        }
    }
}
