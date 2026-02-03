//! Metal indirect command buffer and command types.
//!
//! Corresponds to `Metal/MTLIndirectCommandBuffer.hpp` and `Metal/MTLIndirectCommandEncoder.hpp`.
//!
//! Indirect command buffers allow you to encode GPU commands from the GPU itself,
//! enabling GPU-driven rendering and compute workflows.
//!
//! # Example
//!
//! ```ignore
//! use mtl::{device, IndirectCommandBufferDescriptor, IndirectCommandType};
//!
//! let device = device::system_default().expect("no Metal device");
//!
//! let mut desc = IndirectCommandBufferDescriptor::new().unwrap();
//! desc.set_command_types(IndirectCommandType::DRAW | IndirectCommandType::DRAW_INDEXED);
//! desc.set_max_vertex_buffer_bind_count(8);
//!
//! let icb = device.new_indirect_command_buffer(&desc, 100, ResourceOptions::default());
//! ```

mod buffer;
mod buffer_descriptor;
mod compute_command;
mod render_command;

pub use buffer::IndirectCommandBuffer;
pub use buffer_descriptor::IndirectCommandBufferDescriptor;
pub use compute_command::IndirectComputeCommand;
pub use render_command::IndirectRenderCommand;

// ============================================================================
// IndirectCommandBufferExecutionRange
// ============================================================================

/// Execution range for indirect command buffers.
///
/// C++ equivalent: `MTL::IndirectCommandBufferExecutionRange`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct IndirectCommandBufferExecutionRange {
    /// The starting index.
    pub location: u32,
    /// The number of commands.
    pub length: u32,
}

impl IndirectCommandBufferExecutionRange {
    /// Create a new execution range.
    #[inline]
    pub const fn new(location: u32, length: u32) -> Self {
        Self { location, length }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;

    #[test]
    fn test_indirect_command_buffer_execution_range_size() {
        assert_eq!(
            std::mem::size_of::<IndirectCommandBufferExecutionRange>(),
            8
        );
    }

    #[test]
    fn test_indirect_command_buffer_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<IndirectCommandBufferDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_indirect_command_buffer_size() {
        assert_eq!(
            std::mem::size_of::<IndirectCommandBuffer>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_indirect_render_command_size() {
        assert_eq!(
            std::mem::size_of::<IndirectRenderCommand>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_indirect_compute_command_size() {
        assert_eq!(
            std::mem::size_of::<IndirectComputeCommand>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
