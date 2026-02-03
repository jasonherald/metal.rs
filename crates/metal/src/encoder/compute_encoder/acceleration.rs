//! Acceleration structure and function table methods for ComputeCommandEncoder.

use std::ffi::c_void;

use metal_foundation::{Referencing, UInteger};
use metal_sys::sel;

use super::ComputeCommandEncoder;

impl ComputeCommandEncoder {
    // =========================================================================
    // Acceleration Structure Bindings
    // =========================================================================

    /// Set an acceleration structure at a buffer index (raw pointer version).
    ///
    /// C++ equivalent: `void setAccelerationStructure(const AccelerationStructure*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The acceleration structure pointer must be a valid Metal acceleration structure object.
    #[inline]
    pub unsafe fn set_acceleration_structure_ptr(
        &self,
        acceleration_structure: *const c_void,
        buffer_index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setAccelerationStructure: atBufferIndex:),
                acceleration_structure,
                buffer_index,
            );
        }
    }

    /// Set an acceleration structure at a buffer index.
    ///
    /// C++ equivalent: `void setAccelerationStructure(const AccelerationStructure*, NS::UInteger)`
    #[inline]
    pub fn set_acceleration_structure(
        &self,
        acceleration_structure: &crate::AccelerationStructure,
        buffer_index: UInteger,
    ) {
        unsafe { self.set_acceleration_structure_ptr(acceleration_structure.as_ptr(), buffer_index) };
    }

    // =========================================================================
    // Function Tables (Ray Tracing)
    // =========================================================================

    /// Set a visible function table at a buffer index (raw pointer version).
    ///
    /// C++ equivalent: `void setVisibleFunctionTable(const VisibleFunctionTable*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The visible function table pointer must be valid.
    #[inline]
    pub unsafe fn set_visible_function_table_ptr(
        &self,
        visible_function_table: *const c_void,
        buffer_index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setVisibleFunctionTable: atBufferIndex:),
                visible_function_table,
                buffer_index,
            );
        }
    }

    /// Set an intersection function table at a buffer index (raw pointer version).
    ///
    /// C++ equivalent: `void setIntersectionFunctionTable(const IntersectionFunctionTable*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The intersection function table pointer must be valid.
    #[inline]
    pub unsafe fn set_intersection_function_table_ptr(
        &self,
        intersection_function_table: *const c_void,
        buffer_index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setIntersectionFunctionTable: atBufferIndex:),
                intersection_function_table,
                buffer_index,
            );
        }
    }

    /// Set multiple visible function tables at a range of buffer indices (raw pointer version).
    ///
    /// C++ equivalent: `void setVisibleFunctionTables(const VisibleFunctionTable* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The visible function tables pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_visible_function_tables_ptr(
        &self,
        visible_function_tables: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(setVisibleFunctionTables: withBufferRange:),
                visible_function_tables,
                range,
            );
        }
    }

    /// Set multiple intersection function tables at a range of buffer indices (raw pointer version).
    ///
    /// C++ equivalent: `void setIntersectionFunctionTables(const IntersectionFunctionTable* const*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The intersection function tables pointer must be a valid array with at least `range.length` elements.
    #[inline]
    pub unsafe fn set_intersection_function_tables_ptr(
        &self,
        intersection_function_tables: *const *const c_void,
        range_location: UInteger,
        range_length: UInteger,
    ) {
        let range = metal_foundation::Range::new(range_location, range_length);
        unsafe {
            metal_sys::msg_send_2::<(), *const *const c_void, metal_foundation::Range>(
                self.as_ptr(),
                sel!(setIntersectionFunctionTables: withBufferRange:),
                intersection_function_tables,
                range,
            );
        }
    }
}
