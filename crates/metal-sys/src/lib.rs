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

mod runtime;
mod msg_send;
mod block;
mod macros;
mod functions;

// Re-export runtime types
pub use runtime::{Sel, Class, CachedSel, CachedClass, Protocol, MethodDescription, get_protocol};

// Re-export message sending functions
pub use msg_send::{
    msg_send_0, msg_send_1, msg_send_2, msg_send_3, msg_send_4,
    msg_send_5, msg_send_6, msg_send_7, msg_send_8, msg_send_9, msg_send_10,
};

// Re-export block types
pub use block::{
    BlockLiteral, RcBlock,
    VoidBlock, OneArgBlock, TwoArgBlock, ThreeArgBlock, EventBlock, DeallocatorBlock,
    LogHandlerBlock, HeapOneArgBlock,
    NewLibraryCompletionHandler, NewRenderPipelineStateCompletionHandler,
    NewComputePipelineStateCompletionHandler,
    NewRenderPipelineStateWithReflectionCompletionHandler,
    NewComputePipelineStateWithReflectionCompletionHandler,
    CommandBufferHandler, DrawablePresentedHandler, DeviceNotificationHandler,
    SharedEventNotificationHandler,
};

// Re-export Metal C functions
pub use functions::{
    MTLCreateSystemDefaultDevice, create_system_default_device,
    IOCompressionMethod, IOCompressionStatus, IOCompressionContext,
    MTLIOCompressionContextDefaultChunkSize, MTLIOCreateCompressionContext,
    MTLIOCompressionContextAppendData, MTLIOFlushAndDestroyCompressionContext,
    io_compression_default_chunk_size, DeviceObserver,
};

#[cfg(target_os = "macos")]
pub use functions::{
    MTLCopyAllDevices, MTLCopyAllDevicesWithObserver, MTLRemoveDeviceObserver,
    copy_all_devices,
};
