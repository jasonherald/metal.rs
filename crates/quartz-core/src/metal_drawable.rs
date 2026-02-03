//! CAMetalDrawable implementation.
//!
//! Corresponds to `QuartzCore/CAMetalDrawable.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::metal_layer::MetalLayer;

/// Time interval type (CFTimeInterval is a double).
pub type TimeInterval = f64;

/// A Metal drawable associated with a CAMetalLayer.
///
/// C++ equivalent: `CA::MetalDrawable`
///
/// MetalDrawable extends the Metal Drawable protocol with access to the
/// backing texture and the layer that created it.
#[repr(transparent)]
pub struct MetalDrawable(NonNull<c_void>);

impl MetalDrawable {
    /// Create a MetalDrawable from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid CAMetalDrawable object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the drawable.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // ============================================================
    // CAMetalDrawable-specific methods
    // ============================================================

    /// Get the layer that created this drawable.
    ///
    /// C++ equivalent: `MetalLayer* layer() const`
    pub fn layer(&self) -> Option<MetalLayer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(layer));
            MetalLayer::from_raw(ptr)
        }
    }

    /// Get the texture for this drawable.
    ///
    /// C++ equivalent: `MTL::Texture* texture() const`
    ///
    /// This is the texture to render into. Once rendering is complete,
    /// call present() to display the content.
    pub fn texture(&self) -> Option<metal::Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(texture));
            metal::Texture::from_raw(ptr)
        }
    }

    // ============================================================
    // MTLDrawable protocol methods
    // ============================================================

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

    // Note: addPresentedHandler is not implemented here to avoid requiring block support.
    // Users can implement block callbacks using the metal-sys block infrastructure if needed.
}

impl Clone for MetalDrawable {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for MetalDrawable {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for MetalDrawable {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

// SAFETY: MetalDrawable is a reference-counted object that is thread-safe
unsafe impl Send for MetalDrawable {}
unsafe impl Sync for MetalDrawable {}

impl std::fmt::Debug for MetalDrawable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetalDrawable")
            .field("drawable_id", &self.drawable_id())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metal_drawable_size() {
        // MetalDrawable should be pointer-sized
        assert_eq!(
            std::mem::size_of::<MetalDrawable>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
