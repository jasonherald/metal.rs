//! MTL4 CommitFeedback implementation.
//!
//! Corresponds to `Metal/MTL4CommitFeedback.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, sel};

/// Time interval type (CFTimeInterval is f64).
pub type TimeInterval = f64;

// ============================================================
// CommitFeedback
// ============================================================

/// Feedback about committed command buffers.
///
/// C++ equivalent: `MTL4::CommitFeedback`
///
/// CommitFeedback provides information about the execution of committed
/// command buffers, including GPU timing and error information.
#[repr(transparent)]
pub struct CommitFeedback(NonNull<c_void>);

impl CommitFeedback {
    /// Create a CommitFeedback from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the GPU start time.
    ///
    /// C++ equivalent: `CFTimeInterval GPUStartTime() const`
    pub fn gpu_start_time(&self) -> TimeInterval {
        unsafe { msg_send_0(self.as_ptr(), sel!(GPUStartTime)) }
    }

    /// Get the GPU end time.
    ///
    /// C++ equivalent: `CFTimeInterval GPUEndTime() const`
    pub fn gpu_end_time(&self) -> TimeInterval {
        unsafe { msg_send_0(self.as_ptr(), sel!(GPUEndTime)) }
    }

    /// Get the error, if any.
    ///
    /// C++ equivalent: `NS::Error* error() const`
    pub fn error(&self) -> Option<metal_foundation::Error> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(error));
            metal_foundation::Error::from_ptr(ptr)
        }
    }

    /// Calculate the GPU execution duration.
    pub fn gpu_duration(&self) -> TimeInterval {
        self.gpu_end_time() - self.gpu_start_time()
    }
}

impl Clone for CommitFeedback {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CommitFeedback {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for CommitFeedback {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CommitFeedback {}
unsafe impl Sync for CommitFeedback {}

impl std::fmt::Debug for CommitFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommitFeedback")
            .field("gpu_start_time", &self.gpu_start_time())
            .field("gpu_end_time", &self.gpu_end_time())
            .field("has_error", &self.error().is_some())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_feedback_size() {
        assert_eq!(
            std::mem::size_of::<CommitFeedback>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
