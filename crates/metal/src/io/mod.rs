//! Metal IO command types.
//!
//! Corresponds to `Metal/MTLIOCommandQueue.hpp`, `Metal/MTLIOCommandBuffer.hpp`,
//! and `Metal/MTLIOCompressor.hpp`.
//!
//! IO commands provide asynchronous file loading directly to GPU buffers and textures.

use std::ffi::c_void;

use crate::enums::device::IOCompressionMethod;
use crate::enums::IOCompressionStatus;

mod command_buffer;
mod command_queue;
mod command_queue_descriptor;
mod file_handle;
mod scratch_buffer;
mod scratch_buffer_allocator;

pub use command_buffer::IOCommandBuffer;
pub use command_queue::IOCommandQueue;
pub use command_queue_descriptor::IOCommandQueueDescriptor;
pub use file_handle::IOFileHandle;
pub use scratch_buffer::IOScratchBuffer;
pub use scratch_buffer_allocator::IOScratchBufferAllocator;

// ============================================================================
// IO Compression Functions
// ============================================================================

/// Opaque compression context handle.
pub type IOCompressionContext = *mut c_void;

#[link(name = "Metal", kind = "framework")]
unsafe extern "C" {
    fn MTLIOCompressionContextDefaultChunkSize() -> usize;
    fn MTLIOCreateCompressionContext(
        path: *const std::ffi::c_char,
        compression_type: IOCompressionMethod,
        chunk_size: usize,
    ) -> IOCompressionContext;
    fn MTLIOCompressionContextAppendData(
        context: IOCompressionContext,
        data: *const c_void,
        size: usize,
    );
    fn MTLIOFlushAndDestroyCompressionContext(context: IOCompressionContext)
        -> IOCompressionStatus;
}

/// Get the default chunk size for IO compression.
///
/// C++ equivalent: `size_t IOCompressionContextDefaultChunkSize()`
#[inline]
pub fn io_compression_context_default_chunk_size() -> usize {
    unsafe { MTLIOCompressionContextDefaultChunkSize() }
}

/// Create a compression context for writing compressed data to a file.
///
/// C++ equivalent: `IOCompressionContext IOCreateCompressionContext(const char*, IOCompressionMethod, size_t)`
pub fn io_create_compression_context(
    path: &str,
    compression_type: IOCompressionMethod,
    chunk_size: usize,
) -> Option<IOCompressionContext> {
    let c_path = std::ffi::CString::new(path).ok()?;
    let ctx =
        unsafe { MTLIOCreateCompressionContext(c_path.as_ptr(), compression_type, chunk_size) };
    if ctx.is_null() {
        None
    } else {
        Some(ctx)
    }
}

/// Append data to a compression context.
///
/// # Safety
///
/// The context must be valid, and data must point to at least `size` bytes.
///
/// C++ equivalent: `void IOCompressionContextAppendData(IOCompressionContext, const void*, size_t)`
pub unsafe fn io_compression_context_append_data(
    context: IOCompressionContext,
    data: *const c_void,
    size: usize,
) {
    unsafe {
        MTLIOCompressionContextAppendData(context, data, size);
    }
}

/// Flush and destroy a compression context.
///
/// C++ equivalent: `IOCompressionStatus IOFlushAndDestroyCompressionContext(IOCompressionContext)`
pub fn io_flush_and_destroy_compression_context(
    context: IOCompressionContext,
) -> IOCompressionStatus {
    unsafe { MTLIOFlushAndDestroyCompressionContext(context) }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::{IOCommandQueueType, IOPriority};

    #[test]
    fn test_io_command_queue_descriptor_creation() {
        let descriptor = IOCommandQueueDescriptor::new();
        assert!(descriptor.is_some());
    }

    #[test]
    fn test_io_command_queue_descriptor_properties() {
        let descriptor = IOCommandQueueDescriptor::new().unwrap();

        descriptor.set_priority(IOPriority::HIGH);
        assert_eq!(descriptor.priority(), IOPriority::HIGH);

        descriptor.set_queue_type(IOCommandQueueType::SERIAL);
        assert_eq!(descriptor.queue_type(), IOCommandQueueType::SERIAL);

        descriptor.set_max_command_buffer_count(4);
        assert_eq!(descriptor.max_command_buffer_count(), 4);

        descriptor.set_max_commands_in_flight(8);
        assert_eq!(descriptor.max_commands_in_flight(), 8);
    }

    #[test]
    fn test_io_file_handle_size() {
        assert_eq!(
            std::mem::size_of::<IOFileHandle>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_io_command_queue_size() {
        assert_eq!(
            std::mem::size_of::<IOCommandQueue>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_io_command_buffer_size() {
        assert_eq!(
            std::mem::size_of::<IOCommandBuffer>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_io_compression_default_chunk_size() {
        let size = io_compression_context_default_chunk_size();
        // Should be a reasonable value (typically 64KB or similar)
        assert!(size > 0);
    }
}
