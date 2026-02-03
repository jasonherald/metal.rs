//! Metal drawable interface.
//!
//! Corresponds to `Metal/MTLDrawable.hpp`.
//!
//! The `Drawable` protocol represents a displayable resource that can be rendered to.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

/// Time interval type (CFTimeInterval is a double).
pub type TimeInterval = f64;

/// A displayable resource that can be rendered to.
///
/// C++ equivalent: `MTL::Drawable`
///
/// The drawable protocol represents a displayable resource. Drawables are typically
/// obtained from a CAMetalLayer and are used as render targets.
#[repr(transparent)]
pub struct Drawable(pub(crate) NonNull<c_void>);

impl Drawable {
    /// Create a Drawable from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal drawable object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the drawable.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Present the drawable to the screen.
    ///
    /// C++ equivalent: `void present()`
    pub fn present(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(present));
        }
    }

    /// Present the drawable at the specified time.
    ///
    /// C++ equivalent: `void presentAtTime(CFTimeInterval presentationTime)`
    ///
    /// # Arguments
    ///
    /// * `presentation_time` - The host time at which to present the drawable.
    pub fn present_at_time(&self, presentation_time: TimeInterval) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(presentAtTime:), presentation_time);
        }
    }

    /// Present the drawable after a minimum duration.
    ///
    /// C++ equivalent: `void presentAfterMinimumDuration(CFTimeInterval duration)`
    ///
    /// # Arguments
    ///
    /// * `duration` - The minimum duration before presenting.
    pub fn present_after_minimum_duration(&self, duration: TimeInterval) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(presentAfterMinimumDuration:), duration);
        }
    }

    /// Get the unique identifier for this drawable.
    ///
    /// C++ equivalent: `NS::UInteger drawableID() const`
    pub fn drawable_id(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(drawableID)) }
    }

    /// Get the time at which this drawable was presented.
    ///
    /// C++ equivalent: `CFTimeInterval presentedTime() const`
    ///
    /// Returns 0.0 if the drawable has not been presented yet.
    pub fn presented_time(&self) -> TimeInterval {
        unsafe { msg_send_0(self.as_ptr(), sel!(presentedTime)) }
    }

    /// Add a handler to be called when the drawable is presented.
    ///
    /// C++ equivalent: `void addPresentedHandler(void (^)(Drawable*))`
    ///
    /// The handler is called when the drawable has been presented to the display.
    pub fn add_presented_handler<F>(&self, handler: F)
    where
        F: Fn(&Drawable) + Send + 'static,
    {
        let block = metal_sys::OneArgBlock::from_fn(move |drawable_ptr: *mut c_void| {
            unsafe {
                if let Some(drawable) = Drawable::from_raw(drawable_ptr) {
                    handler(&drawable);
                    // Don't drop - Metal owns this reference
                    std::mem::forget(drawable);
                }
            }
        });

        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(addPresentedHandler:),
                block.as_ptr(),
            );
        }

        // The block is retained by Metal
        std::mem::forget(block);
    }
}

impl Clone for Drawable {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for Drawable {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for Drawable {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

// SAFETY: Drawable is a reference-counted object that is thread-safe
unsafe impl Send for Drawable {}
unsafe impl Sync for Drawable {}

impl std::fmt::Debug for Drawable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Drawable")
            .field("drawable_id", &self.drawable_id())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drawable_size() {
        // Drawable should be pointer-sized
        assert_eq!(
            std::mem::size_of::<Drawable>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
