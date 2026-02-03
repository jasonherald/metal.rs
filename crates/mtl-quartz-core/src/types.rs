//! CoreGraphics types used by QuartzCore.
//!
//! These types are defined here to avoid external dependencies on CoreGraphics bindings.

use std::ffi::c_void;

/// A floating-point type for CoreGraphics.
///
/// On 64-bit systems, this is f64. On 32-bit systems, it would be f32.
/// Since Metal only runs on 64-bit systems, we use f64.
#[cfg(target_pointer_width = "64")]
pub type CGFloat = f64;

#[cfg(target_pointer_width = "32")]
pub type CGFloat = f32;

/// A structure that contains width and height values.
///
/// C equivalent: `CGSize`
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct CGSize {
    /// The width value.
    pub width: CGFloat,
    /// The height value.
    pub height: CGFloat,
}

impl CGSize {
    /// Create a new CGSize with the given dimensions.
    #[inline]
    pub const fn new(width: CGFloat, height: CGFloat) -> Self {
        Self { width, height }
    }

    /// A size with zero width and height.
    pub const ZERO: Self = Self {
        width: 0.0,
        height: 0.0,
    };
}

/// An opaque type that represents a color space.
///
/// C equivalent: `CGColorSpaceRef`
///
/// This is an opaque Core Foundation type. Use CoreGraphics APIs to create
/// and manage color spaces.
pub type CGColorSpaceRef = *mut c_void;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cgsize_new() {
        let size = CGSize::new(100.0, 200.0);
        assert_eq!(size.width, 100.0);
        assert_eq!(size.height, 200.0);
    }

    #[test]
    fn test_cgsize_zero() {
        assert_eq!(CGSize::ZERO.width, 0.0);
        assert_eq!(CGSize::ZERO.height, 0.0);
    }

    #[test]
    fn test_cgsize_layout() {
        // CGSize should be two CGFloats
        assert_eq!(
            std::mem::size_of::<CGSize>(),
            std::mem::size_of::<CGFloat>() * 2
        );
    }
}
