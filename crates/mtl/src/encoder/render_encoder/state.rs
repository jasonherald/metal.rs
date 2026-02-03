//! Render state configuration methods.
//!
//! This module contains methods for configuring render state including:
//! - Rasterizer state (cull mode, winding, fill mode, depth clip)
//! - Depth/stencil state
//! - Blend color
//! - Visibility results
//! - Store actions

use std::ffi::c_void;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_1, sel};

use crate::enums::{
    CullMode, DepthClipMode, StoreAction, StoreActionOptions, TriangleFillMode,
    VisibilityResultMode, Winding,
};

use super::RenderCommandEncoder;

impl RenderCommandEncoder {
    // =========================================================================
    // Rasterizer State
    // =========================================================================

    /// Set the cull mode.
    ///
    /// C++ equivalent: `void setCullMode(MTL::CullMode)`
    #[inline]
    pub fn set_cull_mode(&self, cull_mode: CullMode) {
        unsafe {
            msg_send_1::<(), CullMode>(self.as_ptr(), sel!(setCullMode:), cull_mode);
        }
    }

    /// Set the front-facing winding order.
    ///
    /// C++ equivalent: `void setFrontFacingWinding(MTL::Winding)`
    #[inline]
    pub fn set_front_facing_winding(&self, winding: Winding) {
        unsafe {
            msg_send_1::<(), Winding>(self.as_ptr(), sel!(setFrontFacingWinding:), winding);
        }
    }

    /// Set the triangle fill mode.
    ///
    /// C++ equivalent: `void setTriangleFillMode(MTL::TriangleFillMode)`
    #[inline]
    pub fn set_triangle_fill_mode(&self, fill_mode: TriangleFillMode) {
        unsafe {
            msg_send_1::<(), TriangleFillMode>(
                self.as_ptr(),
                sel!(setTriangleFillMode:),
                fill_mode,
            );
        }
    }

    /// Set the depth clip mode.
    ///
    /// C++ equivalent: `void setDepthClipMode(MTL::DepthClipMode)`
    #[inline]
    pub fn set_depth_clip_mode(&self, mode: DepthClipMode) {
        unsafe {
            msg_send_1::<(), DepthClipMode>(self.as_ptr(), sel!(setDepthClipMode:), mode);
        }
    }

    /// Set the depth bias.
    ///
    /// C++ equivalent: `void setDepthBias(float, float, float)`
    #[inline]
    pub fn set_depth_bias(&self, depth_bias: f32, slope_scale: f32, clamp: f32) {
        unsafe {
            mtl_sys::msg_send_3::<(), f32, f32, f32>(
                self.as_ptr(),
                sel!(setDepthBias: slopeScale: clamp:),
                depth_bias,
                slope_scale,
                clamp,
            );
        }
    }

    /// Set the depth test bounds.
    ///
    /// C++ equivalent: `void setDepthTestBounds(float, float)`
    #[inline]
    pub fn set_depth_test_bounds(&self, min_bound: f32, max_bound: f32) {
        unsafe {
            mtl_sys::msg_send_2::<(), f32, f32>(
                self.as_ptr(),
                sel!(setDepthTestMinBound: maxBound:),
                min_bound,
                max_bound,
            );
        }
    }

    // =========================================================================
    // Depth Stencil State
    // =========================================================================

    /// Set the depth stencil state.
    ///
    /// C++ equivalent: `void setDepthStencilState(const DepthStencilState*)`
    #[inline]
    pub fn set_depth_stencil_state(&self, state: &crate::DepthStencilState) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setDepthStencilState:),
                state.as_ptr(),
            );
        }
    }

    /// Set the stencil reference value (same for front and back).
    ///
    /// C++ equivalent: `void setStencilReferenceValue(uint32_t)`
    #[inline]
    pub fn set_stencil_reference_value(&self, value: u32) {
        unsafe {
            msg_send_1::<(), u32>(self.as_ptr(), sel!(setStencilReferenceValue:), value);
        }
    }

    /// Set the stencil reference values (separate front and back).
    ///
    /// C++ equivalent: `void setStencilReferenceValues(uint32_t, uint32_t)`
    #[inline]
    pub fn set_stencil_reference_values(&self, front: u32, back: u32) {
        unsafe {
            mtl_sys::msg_send_2::<(), u32, u32>(
                self.as_ptr(),
                sel!(setStencilFrontReferenceValue: backReferenceValue:),
                front,
                back,
            );
        }
    }

    // =========================================================================
    // Blend Color
    // =========================================================================

    /// Set the blend color.
    ///
    /// C++ equivalent: `void setBlendColor(float, float, float, float)`
    #[inline]
    pub fn set_blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            mtl_sys::msg_send_4::<(), f32, f32, f32, f32>(
                self.as_ptr(),
                sel!(setBlendColorRed: green: blue: alpha:),
                red,
                green,
                blue,
                alpha,
            );
        }
    }

    // =========================================================================
    // Visibility Result
    // =========================================================================

    /// Set the visibility result mode.
    ///
    /// C++ equivalent: `void setVisibilityResultMode(MTL::VisibilityResultMode, NS::UInteger)`
    #[inline]
    pub fn set_visibility_result_mode(&self, mode: VisibilityResultMode, offset: UInteger) {
        unsafe {
            mtl_sys::msg_send_2::<(), VisibilityResultMode, UInteger>(
                self.as_ptr(),
                sel!(setVisibilityResultMode: offset:),
                mode,
                offset,
            );
        }
    }

    // =========================================================================
    // Store Actions
    // =========================================================================

    /// Set the color store action for an attachment.
    ///
    /// C++ equivalent: `void setColorStoreAction(MTL::StoreAction, NS::UInteger)`
    #[inline]
    pub fn set_color_store_action(&self, store_action: StoreAction, attachment_index: UInteger) {
        unsafe {
            mtl_sys::msg_send_2::<(), StoreAction, UInteger>(
                self.as_ptr(),
                sel!(setColorStoreAction: atIndex:),
                store_action,
                attachment_index,
            );
        }
    }

    /// Set the color store action options for an attachment.
    ///
    /// C++ equivalent: `void setColorStoreActionOptions(MTL::StoreActionOptions, NS::UInteger)`
    #[inline]
    pub fn set_color_store_action_options(
        &self,
        options: StoreActionOptions,
        attachment_index: UInteger,
    ) {
        unsafe {
            mtl_sys::msg_send_2::<(), StoreActionOptions, UInteger>(
                self.as_ptr(),
                sel!(setColorStoreActionOptions: atIndex:),
                options,
                attachment_index,
            );
        }
    }

    /// Set the depth store action.
    ///
    /// C++ equivalent: `void setDepthStoreAction(MTL::StoreAction)`
    #[inline]
    pub fn set_depth_store_action(&self, store_action: StoreAction) {
        unsafe {
            msg_send_1::<(), StoreAction>(self.as_ptr(), sel!(setDepthStoreAction:), store_action);
        }
    }

    /// Set the depth store action options.
    ///
    /// C++ equivalent: `void setDepthStoreActionOptions(MTL::StoreActionOptions)`
    #[inline]
    pub fn set_depth_store_action_options(&self, options: StoreActionOptions) {
        unsafe {
            msg_send_1::<(), StoreActionOptions>(
                self.as_ptr(),
                sel!(setDepthStoreActionOptions:),
                options,
            );
        }
    }

    /// Set the stencil store action.
    ///
    /// C++ equivalent: `void setStencilStoreAction(MTL::StoreAction)`
    #[inline]
    pub fn set_stencil_store_action(&self, store_action: StoreAction) {
        unsafe {
            msg_send_1::<(), StoreAction>(
                self.as_ptr(),
                sel!(setStencilStoreAction:),
                store_action,
            );
        }
    }

    /// Set the stencil store action options.
    ///
    /// C++ equivalent: `void setStencilStoreActionOptions(MTL::StoreActionOptions)`
    #[inline]
    pub fn set_stencil_store_action_options(&self, options: StoreActionOptions) {
        unsafe {
            msg_send_1::<(), StoreActionOptions>(
                self.as_ptr(),
                sel!(setStencilStoreActionOptions:),
                options,
            );
        }
    }
}
