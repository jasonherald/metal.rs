//! MTL4 Counters implementation.
//!
//! Corresponds to `Metal/MTL4Counters.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Integer, Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

// ============================================================
// Enums
// ============================================================

/// Counter heap type.
///
/// C++ equivalent: `MTL4::CounterHeapType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CounterHeapType(pub Integer);

impl CounterHeapType {
    /// Invalid counter heap type.
    pub const INVALID: Self = Self(0);

    /// Timestamp counter heap.
    pub const TIMESTAMP: Self = Self(1);
}

/// Timestamp granularity.
///
/// C++ equivalent: `MTL4::TimestampGranularity`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TimestampGranularity(pub Integer);

impl TimestampGranularity {
    /// Relaxed timestamp granularity.
    pub const RELAXED: Self = Self(0);

    /// Precise timestamp granularity.
    pub const PRECISE: Self = Self(1);
}

// ============================================================
// TimestampHeapEntry
// ============================================================

/// A timestamp entry in a counter heap.
///
/// C++ equivalent: `MTL4::TimestampHeapEntry`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default)]
pub struct TimestampHeapEntry {
    /// The timestamp value.
    pub timestamp: u64,
}

// ============================================================
// CounterHeapDescriptor
// ============================================================

/// Descriptor for creating a counter heap.
///
/// C++ equivalent: `MTL4::CounterHeapDescriptor`
#[repr(transparent)]
pub struct CounterHeapDescriptor(NonNull<c_void>);

impl CounterHeapDescriptor {
    /// Create a CounterHeapDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new counter heap descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4CounterHeapDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the counter heap type.
    ///
    /// C++ equivalent: `CounterHeapType type() const`
    pub fn heap_type(&self) -> CounterHeapType {
        unsafe { msg_send_0(self.as_ptr(), sel!(type)) }
    }

    /// Set the counter heap type.
    ///
    /// C++ equivalent: `void setType(MTL4::CounterHeapType)`
    pub fn set_heap_type(&self, heap_type: CounterHeapType) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setType:), heap_type);
        }
    }

    /// Get the counter count.
    ///
    /// C++ equivalent: `NS::UInteger count() const`
    pub fn count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(count)) }
    }

    /// Set the counter count.
    ///
    /// C++ equivalent: `void setCount(NS::UInteger)`
    pub fn set_count(&self, count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setCount:), count);
        }
    }
}

impl Default for CounterHeapDescriptor {
    fn default() -> Self {
        Self::new().expect("Failed to create MTL4CounterHeapDescriptor")
    }
}

impl Clone for CounterHeapDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CounterHeapDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for CounterHeapDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CounterHeapDescriptor {}
unsafe impl Sync for CounterHeapDescriptor {}

impl std::fmt::Debug for CounterHeapDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CounterHeapDescriptor")
            .field("type", &self.heap_type())
            .field("count", &self.count())
            .finish()
    }
}

// ============================================================
// CounterHeap
// ============================================================

/// A heap for GPU performance counters.
///
/// C++ equivalent: `MTL4::CounterHeap`
///
/// CounterHeap provides storage for GPU performance counters and timestamps.
#[repr(transparent)]
pub struct CounterHeap(NonNull<c_void>);

impl CounterHeap {
    /// Create a CounterHeap from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the label.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ns_string: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ns_string.is_null() {
                return None;
            }
            let c_str: *const i8 = msg_send_0(ns_string, sel!(UTF8String));
            if c_str.is_null() {
                return None;
            }
            Some(
                std::ffi::CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .into_owned(),
            )
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

    /// Get the counter heap type.
    ///
    /// C++ equivalent: `CounterHeapType type() const`
    pub fn heap_type(&self) -> CounterHeapType {
        unsafe { msg_send_0(self.as_ptr(), sel!(type)) }
    }

    /// Get the counter count.
    ///
    /// C++ equivalent: `NS::UInteger count() const`
    pub fn count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(count)) }
    }

    /// Invalidate a range of counters.
    ///
    /// C++ equivalent: `void invalidateCounterRange(NS::Range)`
    pub fn invalidate_counter_range(&self, location: UInteger, length: UInteger) {
        unsafe {
            let range = (location, length);
            let _: () = msg_send_1(self.as_ptr(), sel!(invalidateCounterRange:), range);
        }
    }

    /// Resolve a range of counters and return the data.
    ///
    /// C++ equivalent: `NS::Data* resolveCounterRange(NS::Range)`
    ///
    /// Returns the raw data pointer. Caller is responsible for interpreting
    /// the data based on the counter heap type.
    pub fn resolve_counter_range_raw(
        &self,
        location: UInteger,
        length: UInteger,
    ) -> *mut c_void {
        unsafe {
            let range = (location, length);
            msg_send_1(self.as_ptr(), sel!(resolveCounterRange:), range)
        }
    }
}

impl Clone for CounterHeap {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CounterHeap {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for CounterHeap {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CounterHeap {}
unsafe impl Sync for CounterHeap {}

impl std::fmt::Debug for CounterHeap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CounterHeap")
            .field("label", &self.label())
            .field("type", &self.heap_type())
            .field("count", &self.count())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_heap_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<CounterHeapDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_counter_heap_size() {
        assert_eq!(
            std::mem::size_of::<CounterHeap>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_timestamp_heap_entry_size() {
        assert_eq!(std::mem::size_of::<TimestampHeapEntry>(), 8);
    }

    #[test]
    fn test_counter_heap_type_values() {
        assert_eq!(CounterHeapType::INVALID.0, 0);
        assert_eq!(CounterHeapType::TIMESTAMP.0, 1);
    }

    #[test]
    fn test_timestamp_granularity_values() {
        assert_eq!(TimestampGranularity::RELAXED.0, 0);
        assert_eq!(TimestampGranularity::PRECISE.0, 1);
    }
}
