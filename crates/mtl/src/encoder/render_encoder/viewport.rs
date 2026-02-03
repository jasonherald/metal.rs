//! Viewport and scissor configuration methods.

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_1, sel};

use crate::types::{ScissorRect, Viewport};

use super::RenderCommandEncoder;

impl RenderCommandEncoder {
    // =========================================================================
    // Viewport and Scissor
    // =========================================================================

    /// Set the viewport.
    ///
    /// C++ equivalent: `void setViewport(MTL::Viewport)`
    #[inline]
    pub fn set_viewport(&self, viewport: Viewport) {
        unsafe {
            msg_send_1::<(), Viewport>(self.as_ptr(), sel!(setViewport:), viewport);
        }
    }

    /// Set multiple viewports.
    ///
    /// C++ equivalent: `void setViewports(const Viewport*, NS::UInteger)`
    #[inline]
    pub fn set_viewports(&self, viewports: &[Viewport]) {
        unsafe {
            mtl_sys::msg_send_2::<(), *const Viewport, UInteger>(
                self.as_ptr(),
                sel!(setViewports: count:),
                viewports.as_ptr(),
                viewports.len() as UInteger,
            );
        }
    }

    /// Set the scissor rectangle.
    ///
    /// C++ equivalent: `void setScissorRect(MTL::ScissorRect)`
    #[inline]
    pub fn set_scissor_rect(&self, rect: ScissorRect) {
        unsafe {
            msg_send_1::<(), ScissorRect>(self.as_ptr(), sel!(setScissorRect:), rect);
        }
    }

    /// Set multiple scissor rectangles.
    ///
    /// C++ equivalent: `void setScissorRects(const ScissorRect*, NS::UInteger)`
    #[inline]
    pub fn set_scissor_rects(&self, rects: &[ScissorRect]) {
        unsafe {
            mtl_sys::msg_send_2::<(), *const ScissorRect, UInteger>(
                self.as_ptr(),
                sel!(setScissorRects: count:),
                rects.as_ptr(),
                rects.len() as UInteger,
            );
        }
    }
}
