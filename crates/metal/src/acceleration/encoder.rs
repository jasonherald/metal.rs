//! Acceleration structure command encoder.
//!
//! Contains `AccelerationStructureCommandEncoder`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{
    msg_send_0, msg_send_1, msg_send_2, msg_send_3, msg_send_4, msg_send_5, msg_send_6, sel,
};

use crate::enums::{AccelerationStructureRefitOptions, DataType, ResourceUsage};
use crate::{Buffer, Fence, Heap};

use super::{AccelerationStructure, AccelerationStructureDescriptor};

pub struct AccelerationStructureCommandEncoder(pub(crate) NonNull<c_void>);

impl AccelerationStructureCommandEncoder {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal acceleration structure command encoder.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // CommandEncoder base methods

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
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// End encoding commands.
    ///
    /// C++ equivalent: `void endEncoding()`
    #[inline]
    pub fn end_encoding(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(endEncoding));
        }
    }

    /// Insert a debug signpost.
    ///
    /// C++ equivalent: `void insertDebugSignpost(const NS::String*)`
    pub fn insert_debug_signpost(&self, name: &str) {
        if let Some(ns_name) = metal_foundation::String::from_str(name) {
            unsafe {
                msg_send_1::<(), *const c_void>(
                    self.as_ptr(),
                    sel!(insertDebugSignpost:),
                    ns_name.as_ptr(),
                );
            }
        }
    }

    /// Push a debug group.
    ///
    /// C++ equivalent: `void pushDebugGroup(const NS::String*)`
    pub fn push_debug_group(&self, name: &str) {
        if let Some(ns_name) = metal_foundation::String::from_str(name) {
            unsafe {
                msg_send_1::<(), *const c_void>(
                    self.as_ptr(),
                    sel!(pushDebugGroup:),
                    ns_name.as_ptr(),
                );
            }
        }
    }

    /// Pop a debug group.
    ///
    /// C++ equivalent: `void popDebugGroup()`
    #[inline]
    pub fn pop_debug_group(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(popDebugGroup));
        }
    }

    // Acceleration structure methods

    /// Build an acceleration structure.
    ///
    /// C++ equivalent: `void buildAccelerationStructure(AccelerationStructure*, AccelerationStructureDescriptor*, Buffer*, NS::UInteger)`
    pub fn build_acceleration_structure(
        &self,
        acceleration_structure: &AccelerationStructure,
        descriptor: &AccelerationStructureDescriptor,
        scratch_buffer: &Buffer,
        scratch_buffer_offset: UInteger,
    ) {
        unsafe {
            msg_send_4::<(), *const c_void, *const c_void, *const c_void, UInteger>(
                self.as_ptr(),
                sel!(buildAccelerationStructure:descriptor:scratchBuffer:scratchBufferOffset:),
                acceleration_structure.as_ptr(),
                descriptor.as_ptr(),
                scratch_buffer.as_ptr(),
                scratch_buffer_offset,
            );
        }
    }

    /// Copy an acceleration structure.
    ///
    /// C++ equivalent: `void copyAccelerationStructure(AccelerationStructure*, AccelerationStructure*)`
    pub fn copy_acceleration_structure(
        &self,
        source: &AccelerationStructure,
        destination: &AccelerationStructure,
    ) {
        unsafe {
            msg_send_2::<(), *const c_void, *const c_void>(
                self.as_ptr(),
                sel!(copyAccelerationStructure:toAccelerationStructure:),
                source.as_ptr(),
                destination.as_ptr(),
            );
        }
    }

    /// Copy and compact an acceleration structure.
    ///
    /// C++ equivalent: `void copyAndCompactAccelerationStructure(AccelerationStructure*, AccelerationStructure*)`
    pub fn copy_and_compact_acceleration_structure(
        &self,
        source: &AccelerationStructure,
        destination: &AccelerationStructure,
    ) {
        unsafe {
            msg_send_2::<(), *const c_void, *const c_void>(
                self.as_ptr(),
                sel!(copyAndCompactAccelerationStructure:toAccelerationStructure:),
                source.as_ptr(),
                destination.as_ptr(),
            );
        }
    }

    /// Refit an acceleration structure.
    ///
    /// C++ equivalent: `void refitAccelerationStructure(AccelerationStructure*, AccelerationStructureDescriptor*, AccelerationStructure*, Buffer*, NS::UInteger)`
    pub fn refit_acceleration_structure(
        &self,
        source: &AccelerationStructure,
        descriptor: &AccelerationStructureDescriptor,
        destination: &AccelerationStructure,
        scratch_buffer: &Buffer,
        scratch_buffer_offset: UInteger,
    ) {
        unsafe {
            msg_send_5::<(), *const c_void, *const c_void, *const c_void, *const c_void, UInteger>(
                self.as_ptr(),
                sel!(refitAccelerationStructure:descriptor:destination:scratchBuffer:scratchBufferOffset:),
                source.as_ptr(),
                descriptor.as_ptr(),
                destination.as_ptr(),
                scratch_buffer.as_ptr(),
                scratch_buffer_offset,
            );
        }
    }

    /// Refit an acceleration structure with options.
    ///
    /// C++ equivalent: `void refitAccelerationStructure(AccelerationStructure*, AccelerationStructureDescriptor*, AccelerationStructure*, Buffer*, NS::UInteger, AccelerationStructureRefitOptions)`
    pub fn refit_acceleration_structure_with_options(
        &self,
        source: &AccelerationStructure,
        descriptor: &AccelerationStructureDescriptor,
        destination: &AccelerationStructure,
        scratch_buffer: &Buffer,
        scratch_buffer_offset: UInteger,
        options: AccelerationStructureRefitOptions,
    ) {
        unsafe {
            msg_send_6::<
                (),
                *const c_void,
                *const c_void,
                *const c_void,
                *const c_void,
                UInteger,
                AccelerationStructureRefitOptions,
            >(
                self.as_ptr(),
                sel!(refitAccelerationStructure:descriptor:destination:scratchBuffer:scratchBufferOffset:options:),
                source.as_ptr(),
                descriptor.as_ptr(),
                destination.as_ptr(),
                scratch_buffer.as_ptr(),
                scratch_buffer_offset,
                options,
            );
        }
    }

    /// Write the compacted size of an acceleration structure.
    ///
    /// C++ equivalent: `void writeCompactedAccelerationStructureSize(AccelerationStructure*, Buffer*, NS::UInteger)`
    pub fn write_compacted_acceleration_structure_size(
        &self,
        acceleration_structure: &AccelerationStructure,
        buffer: &Buffer,
        offset: UInteger,
    ) {
        unsafe {
            msg_send_3::<(), *const c_void, *const c_void, UInteger>(
                self.as_ptr(),
                sel!(writeCompactedAccelerationStructureSize:toBuffer:offset:),
                acceleration_structure.as_ptr(),
                buffer.as_ptr(),
                offset,
            );
        }
    }

    /// Write the compacted size of an acceleration structure with size data type.
    ///
    /// C++ equivalent: `void writeCompactedAccelerationStructureSize(AccelerationStructure*, Buffer*, NS::UInteger, DataType)`
    pub fn write_compacted_acceleration_structure_size_with_type(
        &self,
        acceleration_structure: &AccelerationStructure,
        buffer: &Buffer,
        offset: UInteger,
        size_data_type: DataType,
    ) {
        unsafe {
            msg_send_4::<(), *const c_void, *const c_void, UInteger, DataType>(
                self.as_ptr(),
                sel!(writeCompactedAccelerationStructureSize:toBuffer:offset:sizeDataType:),
                acceleration_structure.as_ptr(),
                buffer.as_ptr(),
                offset,
                size_data_type,
            );
        }
    }

    // Counter sampling methods

    /// Sample counters in a buffer.
    ///
    /// # Safety
    ///
    /// The sample_buffer pointer must be valid.
    ///
    /// C++ equivalent: `void sampleCountersInBuffer(CounterSampleBuffer*, NS::UInteger, bool)`
    pub unsafe fn sample_counters_in_buffer_ptr(
        &self,
        sample_buffer: *const c_void,
        sample_index: UInteger,
        barrier: bool,
    ) {
        unsafe {
            msg_send_3::<(), *const c_void, UInteger, bool>(
                self.as_ptr(),
                sel!(sampleCountersInBuffer:atSampleIndex:withBarrier:),
                sample_buffer,
                sample_index,
                barrier,
            );
        }
    }

    // Fence methods

    /// Wait for a fence.
    ///
    /// C++ equivalent: `void waitForFence(Fence*)`
    pub fn wait_for_fence(&self, fence: &Fence) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(waitForFence:), fence.as_ptr());
        }
    }

    /// Update a fence.
    ///
    /// C++ equivalent: `void updateFence(Fence*)`
    pub fn update_fence(&self, fence: &Fence) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(updateFence:), fence.as_ptr());
        }
    }

    // Resource methods

    /// Use a resource.
    ///
    /// C++ equivalent: `void useResource(Resource*, ResourceUsage)`
    pub fn use_resource<R: Referencing>(&self, resource: &R, usage: ResourceUsage) {
        unsafe {
            msg_send_2::<(), *const c_void, ResourceUsage>(
                self.as_ptr(),
                sel!(useResource:usage:),
                resource.as_ptr(),
                usage,
            );
        }
    }

    /// Use multiple resources.
    ///
    /// # Safety
    ///
    /// The resources pointer must be valid for the given count.
    pub unsafe fn use_resources_ptr(
        &self,
        resources: *const *const c_void,
        count: UInteger,
        usage: ResourceUsage,
    ) {
        unsafe {
            msg_send_3::<(), *const *const c_void, UInteger, ResourceUsage>(
                self.as_ptr(),
                sel!(useResources:count:usage:),
                resources,
                count,
                usage,
            );
        }
    }

    /// Use a heap.
    ///
    /// C++ equivalent: `void useHeap(Heap*)`
    pub fn use_heap(&self, heap: &Heap) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(useHeap:), heap.as_ptr());
        }
    }

    /// Use multiple heaps.
    ///
    /// # Safety
    ///
    /// The heaps pointer must be valid for the given count.
    pub unsafe fn use_heaps_ptr(&self, heaps: *const *const c_void, count: UInteger) {
        unsafe {
            msg_send_2::<(), *const *const c_void, UInteger>(
                self.as_ptr(),
                sel!(useHeaps:count:),
                heaps,
                count,
            );
        }
    }
}

impl Clone for AccelerationStructureCommandEncoder {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for AccelerationStructureCommandEncoder {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for AccelerationStructureCommandEncoder {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for AccelerationStructureCommandEncoder {}
unsafe impl Sync for AccelerationStructureCommandEncoder {}

impl std::fmt::Debug for AccelerationStructureCommandEncoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccelerationStructureCommandEncoder")
            .field("label", &self.label())
            .finish()
    }
}
