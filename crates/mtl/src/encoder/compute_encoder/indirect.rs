//! Indirect command execution and counter sampling methods for ComputeCommandEncoder.

use std::ffi::c_void;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::sel;

use super::ComputeCommandEncoder;

impl ComputeCommandEncoder {
    // =========================================================================
    // Indirect Command Execution
    // =========================================================================

    /// Execute commands from an indirect command buffer (raw pointer version).
    ///
    /// C++ equivalent: `void executeCommandsInBuffer(const IndirectCommandBuffer*, NS::Range)`
    ///
    /// # Safety
    ///
    /// The indirect command buffer pointer must be valid.
    pub unsafe fn execute_commands_in_buffer_ptr(
        &self,
        indirect_command_buffer: *const c_void,
        offset: UInteger,
        length: UInteger,
    ) {
        let range = mtl_foundation::Range::new(offset, length);
        unsafe {
            mtl_sys::msg_send_2::<(), *const c_void, mtl_foundation::Range>(
                self.as_ptr(),
                sel!(executeCommandsInBuffer: withRange:),
                indirect_command_buffer,
                range,
            );
        }
    }

    /// Execute commands from an indirect command buffer with indirect range (raw pointer version).
    ///
    /// C++ equivalent: `void executeCommandsInBuffer(const IndirectCommandBuffer*, const Buffer*, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// The indirect command buffer and range buffer pointers must be valid.
    pub unsafe fn execute_commands_in_buffer_with_indirect_range_ptr(
        &self,
        indirect_command_buffer: *const c_void,
        indirect_range_buffer: *const c_void,
        indirect_buffer_offset: UInteger,
    ) {
        unsafe {
            mtl_sys::msg_send_3::<(), *const c_void, *const c_void, UInteger>(
                self.as_ptr(),
                sel!(executeCommandsInBuffer: indirectBuffer: indirectBufferOffset:),
                indirect_command_buffer,
                indirect_range_buffer,
                indirect_buffer_offset,
            );
        }
    }

    // =========================================================================
    // Counter Sampling
    // =========================================================================

    /// Sample counters (raw pointer version).
    ///
    /// C++ equivalent: `void sampleCountersInBuffer(...)`
    ///
    /// # Safety
    ///
    /// The sample buffer pointer must be a valid counter sample buffer object.
    pub unsafe fn sample_counters_in_buffer_ptr(
        &self,
        sample_buffer: *const c_void,
        sample_index: UInteger,
        barrier: bool,
    ) {
        unsafe {
            mtl_sys::msg_send_3::<(), *const c_void, UInteger, bool>(
                self.as_ptr(),
                sel!(sampleCountersInBuffer: atSampleIndex: withBarrier:),
                sample_buffer,
                sample_index,
                barrier,
            );
        }
    }
}
