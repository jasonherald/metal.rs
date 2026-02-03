//! Metal GPU counter facilities for profiling.
//!
//! Corresponds to `Metal/MTLCounters.hpp`.
//!
//! The counter API allows you to sample GPU performance counters for profiling
//! and performance analysis.
//!
//! # Counter Result Types
//!
//! Metal provides three types of counter results:
//! - [`CounterResultTimestamp`] - GPU timestamp values
//! - [`CounterResultStageUtilization`] - Cycle counts per pipeline stage
//! - [`CounterResultStatistic`] - Invocation counts and statistics

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::Device;
use crate::enums::{CounterSampleBufferError, StorageMode};

// ============================================================================
// Counter Result Structures
// ============================================================================

/// Timestamp counter result.
///
/// C++ equivalent: `MTL::CounterResultTimestamp`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CounterResultTimestamp {
    /// The GPU timestamp value.
    pub timestamp: u64,
}

/// Stage utilization counter result.
///
/// C++ equivalent: `MTL::CounterResultStageUtilization`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CounterResultStageUtilization {
    /// Total GPU cycles.
    pub total_cycles: u64,
    /// Cycles spent in vertex processing.
    pub vertex_cycles: u64,
    /// Cycles spent in tessellation.
    pub tessellation_cycles: u64,
    /// Cycles spent in post-tessellation vertex processing.
    pub post_tessellation_vertex_cycles: u64,
    /// Cycles spent in fragment processing.
    pub fragment_cycles: u64,
    /// Cycles spent in render target writes.
    pub render_target_cycles: u64,
}

/// Statistics counter result.
///
/// C++ equivalent: `MTL::CounterResultStatistic`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CounterResultStatistic {
    /// Number of tessellation input patches.
    pub tessellation_input_patches: u64,
    /// Number of vertex shader invocations.
    pub vertex_invocations: u64,
    /// Number of post-tessellation vertex invocations.
    pub post_tessellation_vertex_invocations: u64,
    /// Number of clipper invocations.
    pub clipper_invocations: u64,
    /// Number of primitives output by the clipper.
    pub clipper_primitives_out: u64,
    /// Number of fragment shader invocations.
    pub fragment_invocations: u64,
    /// Number of fragments that passed all tests.
    pub fragments_passed: u64,
    /// Number of compute kernel invocations.
    pub compute_kernel_invocations: u64,
}

// ============================================================================
// Constants
// ============================================================================

/// Error value returned when a counter sample fails.
pub const COUNTER_ERROR_VALUE: UInteger = !0;

/// Value indicating that a counter should not be sampled.
pub const COUNTER_DONT_SAMPLE: UInteger = !0;

// ============================================================================
// Counter
// ============================================================================

/// A single GPU performance counter.
///
/// C++ equivalent: `MTL::Counter`
///
/// Counters are obtained from a [`CounterSet`] and represent individual
/// performance metrics that can be sampled.
#[repr(transparent)]
pub struct Counter(NonNull<c_void>);

impl Counter {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal counter.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the name of this counter.
    ///
    /// C++ equivalent: `NS::String* name() const`
    pub fn name(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(name));
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
}

impl Clone for Counter {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for Counter {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for Counter {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for Counter {}
unsafe impl Sync for Counter {}

impl std::fmt::Debug for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Counter")
            .field("name", &self.name())
            .finish()
    }
}

// ============================================================================
// CounterSet
// ============================================================================

/// A set of related GPU performance counters.
///
/// C++ equivalent: `MTL::CounterSet`
///
/// Counter sets group related counters together. You can query available
/// counter sets from a device.
#[repr(transparent)]
pub struct CounterSet(NonNull<c_void>);

impl CounterSet {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal counter set.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the name of this counter set.
    ///
    /// C++ equivalent: `NS::String* name() const`
    pub fn name(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(name));
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

    /// Get the counters in this counter set.
    ///
    /// C++ equivalent: `NS::Array* counters() const`
    ///
    /// Returns the raw NSArray pointer. Use with Foundation array iteration.
    pub fn counters_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(counters)) }
    }

    /// Get the number of counters in this counter set.
    pub fn counter_count(&self) -> UInteger {
        unsafe {
            let array = self.counters_raw();
            if array.is_null() {
                return 0;
            }
            msg_send_0(array as *const c_void, sel!(count))
        }
    }

    /// Get a counter at the specified index.
    pub fn counter_at_index(&self, index: UInteger) -> Option<Counter> {
        unsafe {
            let array = self.counters_raw();
            if array.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_1(array as *const c_void, sel!(objectAtIndex:), index);
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            Counter::from_raw(ptr)
        }
    }
}

impl Clone for CounterSet {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CounterSet {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for CounterSet {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CounterSet {}
unsafe impl Sync for CounterSet {}

impl std::fmt::Debug for CounterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CounterSet")
            .field("name", &self.name())
            .field("counter_count", &self.counter_count())
            .finish()
    }
}

// ============================================================================
// CounterSampleBufferDescriptor
// ============================================================================

/// Descriptor for creating a counter sample buffer.
///
/// C++ equivalent: `MTL::CounterSampleBufferDescriptor`
#[repr(transparent)]
pub struct CounterSampleBufferDescriptor(NonNull<c_void>);

impl CounterSampleBufferDescriptor {
    /// Allocate a new counter sample buffer descriptor.
    ///
    /// C++ equivalent: `static CounterSampleBufferDescriptor* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let cls = mtl_sys::Class::get("MTLCounterSampleBufferDescriptor")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize an allocated counter sample buffer descriptor.
    ///
    /// C++ equivalent: `CounterSampleBufferDescriptor* init()`
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new counter sample buffer descriptor.
    pub fn new() -> Option<Self> {
        Self::alloc()?.init()
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal counter sample buffer descriptor.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the counter set to sample.
    ///
    /// C++ equivalent: `CounterSet* counterSet() const`
    pub fn counter_set(&self) -> Option<CounterSet> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(counterSet));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            CounterSet::from_raw(ptr)
        }
    }

    /// Set the counter set to sample.
    ///
    /// C++ equivalent: `void setCounterSet(const CounterSet*)`
    pub fn set_counter_set(&self, counter_set: &CounterSet) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setCounterSet:),
                counter_set.as_ptr(),
            );
        }
    }

    /// Get the label for this descriptor.
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

    /// Set the label for this descriptor.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = mtl_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the number of samples to store.
    ///
    /// C++ equivalent: `NS::UInteger sampleCount() const`
    #[inline]
    pub fn sample_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(sampleCount)) }
    }

    /// Set the number of samples to store.
    ///
    /// C++ equivalent: `void setSampleCount(NS::UInteger)`
    #[inline]
    pub fn set_sample_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setSampleCount:), count);
        }
    }

    /// Get the storage mode for the sample buffer.
    ///
    /// C++ equivalent: `StorageMode storageMode() const`
    #[inline]
    pub fn storage_mode(&self) -> StorageMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(storageMode)) }
    }

    /// Set the storage mode for the sample buffer.
    ///
    /// C++ equivalent: `void setStorageMode(StorageMode)`
    #[inline]
    pub fn set_storage_mode(&self, mode: StorageMode) {
        unsafe {
            msg_send_1::<(), StorageMode>(self.as_ptr(), sel!(setStorageMode:), mode);
        }
    }
}

impl Default for CounterSampleBufferDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create CounterSampleBufferDescriptor")
    }
}

impl Clone for CounterSampleBufferDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy CounterSampleBufferDescriptor")
        }
    }
}

impl Drop for CounterSampleBufferDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for CounterSampleBufferDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CounterSampleBufferDescriptor {}
unsafe impl Sync for CounterSampleBufferDescriptor {}

impl std::fmt::Debug for CounterSampleBufferDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CounterSampleBufferDescriptor")
            .field("label", &self.label())
            .field("sample_count", &self.sample_count())
            .field("storage_mode", &self.storage_mode())
            .finish()
    }
}

// ============================================================================
// CounterSampleBuffer
// ============================================================================

/// A buffer that stores GPU counter samples.
///
/// C++ equivalent: `MTL::CounterSampleBuffer`
///
/// Counter sample buffers are created from a device using a descriptor.
/// They store sampled counter values that can be resolved to get the results.
#[repr(transparent)]
pub struct CounterSampleBuffer(NonNull<c_void>);

impl CounterSampleBuffer {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal counter sample buffer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the device that created this buffer.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            Device::from_raw(ptr).expect("counter sample buffer has no device")
        }
    }

    /// Get the label for this buffer.
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

    /// Get the number of samples in this buffer.
    ///
    /// C++ equivalent: `NS::UInteger sampleCount() const`
    #[inline]
    pub fn sample_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(sampleCount)) }
    }

    /// Resolve counter values for a range of samples.
    ///
    /// Returns the raw NSData pointer containing the resolved counter values.
    /// The data format depends on the counter set used.
    ///
    /// C++ equivalent: `NS::Data* resolveCounterRange(NS::Range)`
    pub fn resolve_counter_range_raw(
        &self,
        location: UInteger,
        length: UInteger,
    ) -> Result<*mut c_void, CounterSampleBufferError> {
        unsafe {
            let range = mtl_foundation::Range::new(location, length);
            // Note: This method can throw an exception in ObjC, but we handle
            // the error case by checking if the result is null
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(resolveCounterRange:), range);
            if ptr.is_null() {
                Err(CounterSampleBufferError::INVALID)
            } else {
                Ok(ptr)
            }
        }
    }
}

impl Clone for CounterSampleBuffer {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CounterSampleBuffer {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for CounterSampleBuffer {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CounterSampleBuffer {}
unsafe impl Sync for CounterSampleBuffer {}

impl std::fmt::Debug for CounterSampleBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CounterSampleBuffer")
            .field("label", &self.label())
            .field("sample_count", &self.sample_count())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_size() {
        assert_eq!(
            std::mem::size_of::<Counter>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_counter_set_size() {
        assert_eq!(
            std::mem::size_of::<CounterSet>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_counter_sample_buffer_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<CounterSampleBufferDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_counter_sample_buffer_size() {
        assert_eq!(
            std::mem::size_of::<CounterSampleBuffer>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_counter_result_timestamp_size() {
        assert_eq!(std::mem::size_of::<CounterResultTimestamp>(), 8);
    }

    #[test]
    fn test_counter_result_stage_utilization_size() {
        assert_eq!(std::mem::size_of::<CounterResultStageUtilization>(), 48);
    }

    #[test]
    fn test_counter_result_statistic_size() {
        assert_eq!(std::mem::size_of::<CounterResultStatistic>(), 64);
    }

    #[test]
    fn test_constants() {
        assert_eq!(COUNTER_ERROR_VALUE, !0);
        assert_eq!(COUNTER_DONT_SAMPLE, !0);
    }
}
