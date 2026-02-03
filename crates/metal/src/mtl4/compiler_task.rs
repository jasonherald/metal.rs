//! MTL4 CompilerTask implementation.
//!
//! Corresponds to `Metal/MTL4CompilerTask.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, sel};

use super::enums::CompilerTaskStatus;

// Forward declaration - Compiler will be defined in compiler.rs
// We use a raw pointer here to avoid circular dependencies

// ============================================================
// CompilerTask
// ============================================================

/// A task representing an asynchronous compilation operation.
///
/// C++ equivalent: `MTL4::CompilerTask`
///
/// CompilerTask represents an in-progress compilation that can be
/// waited on or queried for status.
#[repr(transparent)]
pub struct CompilerTask(NonNull<c_void>);

impl CompilerTask {
    /// Create a CompilerTask from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the compiler that created this task.
    ///
    /// C++ equivalent: `Compiler* compiler() const`
    ///
    /// Returns the raw pointer to the compiler. Use `Compiler::from_raw`
    /// to wrap it if needed.
    pub fn compiler_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(compiler)) }
    }

    /// Get the task status.
    ///
    /// C++ equivalent: `CompilerTaskStatus status() const`
    pub fn status(&self) -> CompilerTaskStatus {
        unsafe { msg_send_0(self.as_ptr(), sel!(status)) }
    }

    /// Wait until the task is completed.
    ///
    /// C++ equivalent: `void waitUntilCompleted()`
    ///
    /// Blocks the current thread until the compilation finishes.
    pub fn wait_until_completed(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(waitUntilCompleted));
        }
    }
}

impl Clone for CompilerTask {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CompilerTask {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for CompilerTask {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CompilerTask {}
unsafe impl Sync for CompilerTask {}

impl std::fmt::Debug for CompilerTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompilerTask")
            .field("status", &self.status())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_task_size() {
        assert_eq!(
            std::mem::size_of::<CompilerTask>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
