//! Objective-C block support using the Clang blocks ABI.
//!
//! This module provides the exact same block ABI that Clang generates for
//! Objective-C blocks. The structure matches Apple's documented Block ABI:
//! https://clang.llvm.org/docs/Block-ABI-Apple.html
//!
//! # C++ Equivalent
//!
//! In metal-cpp, blocks are defined as:
//! ```cpp
//! using NewLibraryCompletionHandler = void (^)(MTL::Library*, NS::Error*);
//! using NewLibraryCompletionHandlerFunction = std::function<void(MTL::Library*, NS::Error*)>;
//! ```
//!
//! And used like:
//! ```cpp
//! __block MTL::NewLibraryCompletionHandlerFunction blockCompletionHandler = completionHandler;
//! newLibrary(pSource, pOptions, ^(MTL::Library* pLibrary, NS::Error* pError) {
//!     blockCompletionHandler(pLibrary, pError);
//! });
//! ```
//!
//! This module provides the Rust equivalent of that block machinery.

use std::ffi::c_void;
use std::mem;
use std::ptr;

// =============================================================================
// Block ABI Constants (from Clang)
// =============================================================================

/// Block has copy/dispose helpers.
const BLOCK_HAS_COPY_DISPOSE: i32 = 1 << 25;

/// Block has a C++ destructor (or __strong/__weak capture).
#[allow(dead_code)]
const BLOCK_HAS_CTOR: i32 = 1 << 26;

/// Block is a global block.
#[allow(dead_code)]
const BLOCK_IS_GLOBAL: i32 = 1 << 28;

/// Block has a type encoding signature in descriptor.
#[allow(dead_code)]
const BLOCK_HAS_STRET: i32 = 1 << 29;

/// Block has a signature and/or layout.
#[allow(dead_code)]
const BLOCK_HAS_SIGNATURE: i32 = 1 << 30;

// =============================================================================
// External Symbols from libobjc
// =============================================================================

#[link(name = "objc")]
unsafe extern "C" {
    /// The class object for stack-allocated blocks.
    static _NSConcreteStackBlock: *const c_void;

    /// The class object for global (static) blocks.
    static _NSConcreteGlobalBlock: *const c_void;
}

// =============================================================================
// Block Descriptor (Clang ABI - matches struct Block_descriptor_1)
// =============================================================================

/// Block descriptor without copy/dispose helpers.
#[repr(C)]
#[allow(dead_code)]
struct BlockDescriptorBasic {
    /// Reserved for future use (always 0).
    reserved: usize,
    /// Size of the block literal structure.
    size: usize,
}

/// Block descriptor with copy/dispose helpers.
/// Corresponds to struct Block_descriptor with BLOCK_HAS_COPY_DISPOSE.
#[repr(C)]
struct BlockDescriptor {
    /// Reserved for future use (always 0).
    reserved: usize,
    /// Size of the block literal structure.
    size: usize,
    /// Copy helper function pointer.
    copy: unsafe extern "C" fn(*mut c_void, *const c_void),
    /// Dispose helper function pointer.
    dispose: unsafe extern "C" fn(*mut c_void),
}

// =============================================================================
// Block Literal (Clang ABI - matches struct Block_literal_1)
// =============================================================================

/// Block literal structure matching Clang's Block_literal_1.
///
/// This is the actual block object passed to Objective-C APIs.
/// The layout must exactly match what Clang generates.
#[repr(C)]
pub struct BlockLiteral<Closure> {
    /// ISA pointer - points to _NSConcreteStackBlock or _NSConcreteGlobalBlock.
    pub isa: *const c_void,
    /// Block flags.
    pub flags: i32,
    /// Reserved (always 0).
    pub reserved: i32,
    /// Invoke function pointer - the actual function to call.
    pub invoke: *const c_void,
    /// Pointer to the block descriptor.
    pub descriptor: *const c_void,
    /// Captured closure data.
    pub closure: Closure,
}

// =============================================================================
// RcBlock - Reference-counted block that properly matches the ABI
// =============================================================================

/// A reference-counted block wrapping a Rust closure.
///
/// This structure matches the Clang blocks ABI exactly, allowing Rust closures
/// to be passed to Objective-C APIs that expect block parameters.
#[repr(C)]
pub struct RcBlock<Closure> {
    /// The block literal (must be first for ABI compatibility).
    literal: BlockLiteral<Closure>,
    /// The block descriptor (stored inline after the literal).
    descriptor: BlockDescriptor,
}

// Copy helper - increments reference count or copies captured data
unsafe extern "C" fn block_copy_helper<Closure>(_dst: *mut c_void, _src: *const c_void) {
    // For blocks with Clone closures, we could implement actual copying here.
    // For now, this is called when the block is copied to the heap.
    // The block runtime handles the actual memory copy.
}

// Dispose helper - decrements reference count or frees captured data
unsafe extern "C" fn block_dispose_helper<Closure>(_src: *mut c_void) {
    // For blocks with Drop closures, cleanup would happen here.
    // The block runtime calls this when the block's refcount reaches 0.
}

impl<Closure> RcBlock<Closure> {
    /// Create a new block with the specified invoke function.
    ///
    /// # Safety
    ///
    /// The `invoke` function pointer must have the correct calling convention
    /// and signature for the block type. The first parameter must be a pointer
    /// to the BlockLiteral.
    #[inline]
    pub unsafe fn new(closure: Closure, invoke: *const c_void) -> Self {
        let descriptor = BlockDescriptor {
            reserved: 0,
            size: mem::size_of::<BlockLiteral<Closure>>(),
            copy: block_copy_helper::<Closure>,
            dispose: block_dispose_helper::<Closure>,
        };

        let mut block = RcBlock {
            literal: BlockLiteral {
                isa: unsafe { _NSConcreteStackBlock },
                flags: BLOCK_HAS_COPY_DISPOSE,
                reserved: 0,
                invoke,
                descriptor: ptr::null(),
                closure,
            },
            descriptor,
        };

        // Point to our inline descriptor
        block.literal.descriptor = &block.descriptor as *const _ as *const c_void;

        block
    }

    /// Create a new heap-allocated block with the specified invoke function.
    ///
    /// This allocates the block on the heap so it can be safely passed to
    /// Objective-C APIs that retain the block. The returned pointer should
    /// be passed to `as_heap_ptr()` to get the block pointer.
    ///
    /// # Safety
    ///
    /// The `invoke` function pointer must have the correct calling convention
    /// and signature for the block type.
    #[inline]
    pub unsafe fn new_heap(closure: Closure, invoke: *const c_void) -> Box<Self> {
        let descriptor = BlockDescriptor {
            reserved: 0,
            size: mem::size_of::<BlockLiteral<Closure>>(),
            copy: block_copy_helper::<Closure>,
            dispose: block_dispose_helper::<Closure>,
        };

        let mut block = Box::new(RcBlock {
            literal: BlockLiteral {
                isa: unsafe { _NSConcreteStackBlock },
                flags: BLOCK_HAS_COPY_DISPOSE,
                reserved: 0,
                invoke,
                descriptor: ptr::null(),
                closure,
            },
            descriptor,
        });

        // Point to our inline descriptor (now on the heap)
        block.literal.descriptor = &block.descriptor as *const _ as *const c_void;

        block
    }

    /// Get a pointer to the block literal to pass to Objective-C.
    #[inline]
    pub fn as_ptr(&self) -> *const c_void {
        &self.literal as *const _ as *const c_void
    }

    /// Consume the block, preventing its destructor from running.
    ///
    /// Use this after passing the block to Metal/Objective-C APIs that
    /// will retain the block internally. The block runtime will handle
    /// cleanup when the retain count reaches zero.
    #[inline]
    pub fn into_raw(self) -> *const c_void {
        let ptr = self.as_ptr();
        mem::forget(self);
        ptr
    }
}

// =============================================================================
// Concrete Block Types matching metal-cpp signatures
// =============================================================================

// -----------------------------------------------------------------------------
// void (^)(void) - No arguments
// -----------------------------------------------------------------------------

/// Block type: `void (^)(void)`
///
/// Used for simple callbacks with no arguments.
pub type VoidBlock = RcBlock<Box<dyn Fn() + Send>>;

impl VoidBlock {
    /// Create a block from a closure taking no arguments.
    pub fn from_fn<F>(f: F) -> Self
    where
        F: Fn() + Send + 'static,
    {
        unsafe extern "C" fn invoke(block: *mut BlockLiteral<Box<dyn Fn() + Send>>) {
            unsafe { ((*block).closure)() }
        }

        unsafe { RcBlock::new(Box::new(f), invoke as *const c_void) }
    }
}

// -----------------------------------------------------------------------------
// void (^)(id) - One object argument
// -----------------------------------------------------------------------------

/// Block type: `void (^)(id)`
///
/// Used for callbacks with one object argument (e.g., CommandBuffer handlers).
pub type OneArgBlock = RcBlock<Box<dyn Fn(*mut c_void) + Send>>;

/// A heap-allocated one-argument block that can be safely passed to ObjC APIs.
pub struct HeapOneArgBlock {
    inner: *mut OneArgBlock,
}

impl HeapOneArgBlock {
    /// Create a heap-allocated block from a closure taking one pointer argument.
    pub fn from_fn<F>(f: F) -> Self
    where
        F: Fn(*mut c_void) + Send + 'static,
    {
        unsafe extern "C" fn invoke(
            block: *mut BlockLiteral<Box<dyn Fn(*mut c_void) + Send>>,
            arg: *mut c_void,
        ) {
            unsafe { ((*block).closure)(arg) }
        }

        let boxed: Box<OneArgBlock> =
            unsafe { RcBlock::new_heap(Box::new(f) as Box<dyn Fn(*mut c_void) + Send>, invoke as *const c_void) };
        HeapOneArgBlock {
            inner: Box::into_raw(boxed),
        }
    }

    /// Get a pointer to the block literal to pass to Objective-C.
    pub fn as_ptr(&self) -> *const c_void {
        unsafe { (*self.inner).as_ptr() }
    }
}

impl OneArgBlock {
    /// Create a block from a closure taking one pointer argument.
    pub fn from_fn<F>(f: F) -> Self
    where
        F: Fn(*mut c_void) + Send + 'static,
    {
        unsafe extern "C" fn invoke(
            block: *mut BlockLiteral<Box<dyn Fn(*mut c_void) + Send>>,
            arg: *mut c_void,
        ) {
            unsafe { ((*block).closure)(arg) }
        }

        unsafe { RcBlock::new(Box::new(f), invoke as *const c_void) }
    }

    /// Create a heap-allocated block from a closure taking one pointer argument.
    ///
    /// Use this for blocks that need to outlive the current scope, such as
    /// completion handlers passed to Metal APIs.
    pub fn from_fn_heap<F>(f: F) -> HeapOneArgBlock
    where
        F: Fn(*mut c_void) + Send + 'static,
    {
        HeapOneArgBlock::from_fn(f)
    }
}

// -----------------------------------------------------------------------------
// void (^)(id, id) - Two object arguments
// -----------------------------------------------------------------------------

/// Block type: `void (^)(id, id)`
///
/// Used for completion handlers: `void (^)(MTL::Library*, NS::Error*)`
pub type TwoArgBlock = RcBlock<Box<dyn Fn(*mut c_void, *mut c_void) + Send>>;

impl TwoArgBlock {
    /// Create a block from a closure taking two pointer arguments.
    ///
    /// This matches the C++ pattern:
    /// ```cpp
    /// using NewLibraryCompletionHandler = void (^)(MTL::Library*, NS::Error*);
    /// ```
    pub fn from_fn<F>(f: F) -> Self
    where
        F: Fn(*mut c_void, *mut c_void) + Send + 'static,
    {
        unsafe extern "C" fn invoke(
            block: *mut BlockLiteral<Box<dyn Fn(*mut c_void, *mut c_void) + Send>>,
            arg1: *mut c_void,
            arg2: *mut c_void,
        ) {
            unsafe { ((*block).closure)(arg1, arg2) }
        }

        unsafe { RcBlock::new(Box::new(f), invoke as *const c_void) }
    }
}

// -----------------------------------------------------------------------------
// void (^)(id, id, id) - Three object arguments
// -----------------------------------------------------------------------------

/// Block type: `void (^)(id, id, id)`
///
/// Used for completion handlers with reflection:
/// `void (^)(MTL::RenderPipelineState*, MTL::RenderPipelineReflection*, NS::Error*)`
#[allow(clippy::type_complexity)]
pub type ThreeArgBlock = RcBlock<Box<dyn Fn(*mut c_void, *mut c_void, *mut c_void) + Send>>;

#[allow(clippy::type_complexity)]
impl ThreeArgBlock {
    /// Create a block from a closure taking three pointer arguments.
    ///
    /// This matches the C++ pattern:
    /// ```cpp
    /// using NewRenderPipelineStateWithReflectionCompletionHandler =
    ///     void (^)(MTL::RenderPipelineState*, MTL::RenderPipelineReflection*, NS::Error*);
    /// ```
    pub fn from_fn<F>(f: F) -> Self
    where
        F: Fn(*mut c_void, *mut c_void, *mut c_void) + Send + 'static,
    {
        #[allow(clippy::type_complexity)]
        unsafe extern "C" fn invoke(
            block: *mut BlockLiteral<Box<dyn Fn(*mut c_void, *mut c_void, *mut c_void) + Send>>,
            arg1: *mut c_void,
            arg2: *mut c_void,
            arg3: *mut c_void,
        ) {
            unsafe { ((*block).closure)(arg1, arg2, arg3) }
        }

        unsafe { RcBlock::new(Box::new(f), invoke as *const c_void) }
    }
}

// -----------------------------------------------------------------------------
// void (^)(id, uint64_t) - Object and value arguments
// -----------------------------------------------------------------------------

/// Block type: `void (^)(id, uint64_t)`
///
/// Used for SharedEvent notifications: `void (^)(MTL::SharedEvent*, uint64_t)`
pub type EventBlock = RcBlock<Box<dyn Fn(*mut c_void, u64) + Send>>;

impl EventBlock {
    /// Create a block for SharedEvent notifications.
    ///
    /// This matches the C++ pattern for event signaling.
    pub fn from_fn<F>(f: F) -> Self
    where
        F: Fn(*mut c_void, u64) + Send + 'static,
    {
        unsafe extern "C" fn invoke(
            block: *mut BlockLiteral<Box<dyn Fn(*mut c_void, u64) + Send>>,
            event: *mut c_void,
            value: u64,
        ) {
            unsafe { ((*block).closure)(event, value) }
        }

        unsafe { RcBlock::new(Box::new(f), invoke as *const c_void) }
    }
}

// -----------------------------------------------------------------------------
// void (^)(void*, size_t) - Buffer deallocator
// -----------------------------------------------------------------------------

/// Block type: `void (^)(void*, size_t)`
///
/// Used for buffer deallocators in `newBufferWithBytesNoCopy:...deallocator:`.
pub type DeallocatorBlock = RcBlock<Box<dyn Fn(*mut c_void, usize) + Send>>;

impl DeallocatorBlock {
    /// Create a block for buffer deallocation.
    ///
    /// The closure receives the buffer pointer and its size.
    pub fn from_fn<F>(f: F) -> Self
    where
        F: Fn(*mut c_void, usize) + Send + 'static,
    {
        unsafe extern "C" fn invoke(
            block: *mut BlockLiteral<Box<dyn Fn(*mut c_void, usize) + Send>>,
            ptr: *mut c_void,
            size: usize,
        ) {
            unsafe { ((*block).closure)(ptr, size) }
        }

        unsafe { RcBlock::new(Box::new(f), invoke as *const c_void) }
    }
}

// =============================================================================
// Type Aliases matching metal-cpp naming
// =============================================================================

/// Completion handler for library creation.
///
/// C++ equivalent: `using NewLibraryCompletionHandler = void (^)(MTL::Library*, NS::Error*);`
pub type NewLibraryCompletionHandler = TwoArgBlock;

/// Completion handler for render pipeline state creation.
///
/// C++ equivalent: `using NewRenderPipelineStateCompletionHandler = void (^)(MTL::RenderPipelineState*, NS::Error*);`
pub type NewRenderPipelineStateCompletionHandler = TwoArgBlock;

/// Completion handler for compute pipeline state creation.
///
/// C++ equivalent: `using NewComputePipelineStateCompletionHandler = void (^)(MTL::ComputePipelineState*, NS::Error*);`
pub type NewComputePipelineStateCompletionHandler = TwoArgBlock;

/// Completion handler for render pipeline state with reflection.
///
/// C++ equivalent: `using NewRenderPipelineStateWithReflectionCompletionHandler =
///     void (^)(MTL::RenderPipelineState*, MTL::RenderPipelineReflection*, NS::Error*);`
pub type NewRenderPipelineStateWithReflectionCompletionHandler = ThreeArgBlock;

/// Completion handler for compute pipeline state with reflection.
///
/// C++ equivalent: `using NewComputePipelineStateWithReflectionCompletionHandler =
///     void (^)(MTL::ComputePipelineState*, MTL::ComputePipelineReflection*, NS::Error*);`
pub type NewComputePipelineStateWithReflectionCompletionHandler = ThreeArgBlock;

/// Handler for command buffer completion.
///
/// Used with `addCompletedHandler:` and `addScheduledHandler:`.
pub type CommandBufferHandler = OneArgBlock;

/// Handler for drawable presentation.
///
/// Used with `addPresentedHandler:`.
pub type DrawablePresentedHandler = OneArgBlock;

/// Handler for device notifications.
///
/// C++ equivalent: `using DeviceNotificationHandlerBlock =
///     void (^)(MTL::Device* pDevice, MTL::DeviceNotificationName notifyName);`
pub type DeviceNotificationHandler = TwoArgBlock;

/// Handler for SharedEvent notifications.
pub type SharedEventNotificationHandler = EventBlock;

// -----------------------------------------------------------------------------
// void (^)(id, id, isize, id) - Four arguments (for LogState)
// -----------------------------------------------------------------------------

/// Block type: `void (^)(id, id, isize, id)`
///
/// Used for log handlers: `void (^)(NS::String*, NS::String*, LogLevel, NS::String*)`
/// LogLevel in Metal is NS::Integer (isize on 64-bit platforms).
#[allow(clippy::type_complexity)]
pub type LogHandlerBlock =
    RcBlock<Box<dyn Fn(*mut c_void, *mut c_void, isize, *mut c_void) + Send>>;

#[allow(clippy::type_complexity)]
impl LogHandlerBlock {
    /// Create a block for log handling.
    ///
    /// The closure receives (subsystem, category, level, message).
    pub fn from_fn<F>(f: F) -> Self
    where
        F: Fn(*mut c_void, *mut c_void, isize, *mut c_void) + Send + 'static,
    {
        #[allow(clippy::type_complexity)]
        unsafe extern "C" fn invoke(
            block: *mut BlockLiteral<Box<dyn Fn(*mut c_void, *mut c_void, isize, *mut c_void) + Send>>,
            subsystem: *mut c_void,
            category: *mut c_void,
            level: isize,
            message: *mut c_void,
        ) {
            unsafe { ((*block).closure)(subsystem, category, level, message) }
        }

        unsafe { RcBlock::new(Box::new(f), invoke as *const c_void) }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_literal_size() {
        // Verify the block literal has the expected layout
        assert!(mem::size_of::<BlockLiteral<()>>() > 0);
        // isa (8) + flags (4) + reserved (4) + invoke (8) + descriptor (8) + closure
        assert_eq!(
            mem::size_of::<BlockLiteral<()>>(),
            8 + 4 + 4 + 8 + 8 // 32 bytes minimum on 64-bit
        );
    }

    #[test]
    fn test_void_block_creation() {
        let called = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let called_clone = called.clone();

        let block = VoidBlock::from_fn(move || {
            called_clone.store(true, std::sync::atomic::Ordering::SeqCst);
        });

        assert!(!block.as_ptr().is_null());
    }

    #[test]
    fn test_two_arg_block_creation() {
        let block = TwoArgBlock::from_fn(|_obj, _err| {
            // Callback logic
        });

        assert!(!block.as_ptr().is_null());
    }

    #[test]
    fn test_event_block_creation() {
        let block = EventBlock::from_fn(|_event, _value| {
            // Closure will be called with event and value
        });

        assert!(!block.as_ptr().is_null());
    }

    #[test]
    fn test_block_into_raw() {
        let block = VoidBlock::from_fn(|| {});
        let ptr = block.into_raw();
        assert!(!ptr.is_null());
        // Note: memory is intentionally leaked here for Metal to manage
    }
}
