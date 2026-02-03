//! Low-level Objective-C runtime bindings for Metal.
//!
//! This crate provides the foundational FFI layer for communicating with
//! the Objective-C runtime. It includes:
//!
//! - Architecture-aware message sending (`objc_msgSend` variants)
//! - Selector and class caching
//! - Objective-C block support (Clang ABI)
//! - Custom macros for enums and bitflags
//!
//! # Safety
//!
//! This crate is intentionally unsafe. All public functions are marked
//! `unsafe` and require the caller to uphold Objective-C runtime invariants.
//! Higher-level safe wrappers are provided by the `metal` crate.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

mod block;
mod functions;
mod macros;
mod msg_send;
mod runtime;

// Re-export runtime types
pub use runtime::{CachedClass, CachedSel, Class, MethodDescription, Protocol, Sel, get_protocol};

// Re-export message sending functions
pub use msg_send::{
    msg_send_0, msg_send_1, msg_send_2, msg_send_3, msg_send_4, msg_send_5, msg_send_6, msg_send_7,
    msg_send_8, msg_send_9, msg_send_10,
};

// Re-export block types
pub use block::{
    BlockLiteral, CommandBufferHandler, DeallocatorBlock, DeviceNotificationHandler,
    DrawablePresentedHandler, EventBlock, HeapOneArgBlock, LogHandlerBlock,
    NewComputePipelineStateCompletionHandler,
    NewComputePipelineStateWithReflectionCompletionHandler, NewLibraryCompletionHandler,
    NewRenderPipelineStateCompletionHandler, NewRenderPipelineStateWithReflectionCompletionHandler,
    OneArgBlock, RcBlock, SharedEventNotificationHandler, ThreeArgBlock, TwoArgBlock, VoidBlock,
};

// Re-export Metal C functions
pub use functions::{
    DeviceObserver, IOCompressionContext, IOCompressionMethod, IOCompressionStatus,
    MTLCreateSystemDefaultDevice, MTLIOCompressionContextAppendData,
    MTLIOCompressionContextDefaultChunkSize, MTLIOCreateCompressionContext,
    MTLIOFlushAndDestroyCompressionContext, create_system_default_device,
    io_compression_default_chunk_size,
};

#[cfg(target_os = "macos")]
pub use functions::{
    MTLCopyAllDevices, MTLCopyAllDevicesWithObserver, MTLRemoveDeviceObserver, copy_all_devices,
};
