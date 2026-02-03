//! MTL4 TileRenderPipeline implementation.
//!
//! Corresponds to `Metal/MTL4TileRenderPipeline.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::Size;
use super::{FunctionDescriptor, PipelineOptions, StaticLinkingDescriptor};

// ============================================================
// TileRenderPipelineDescriptor
// ============================================================

/// Descriptor for MTL4 tile render pipelines.
///
/// C++ equivalent: `MTL4::TileRenderPipelineDescriptor`
///
/// TileRenderPipelineDescriptor configures tile shading pipelines
/// for on-tile compute operations.
#[repr(transparent)]
pub struct TileRenderPipelineDescriptor(NonNull<c_void>);

impl TileRenderPipelineDescriptor {
    /// Create a TileRenderPipelineDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new tile render pipeline descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4TileRenderPipelineDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    // ========== Base Pipeline Properties ==========

    /// Get the label.
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ns_string: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ns_string.is_null() {
                return None;
            }
            let c_str: *const i8 = msg_send_0(ns_string, sel!(UTF8String));
            if c_str.is_null() {
                return None;
            }
            Some(
                std::ffi::CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    }

    /// Set the label.
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the pipeline options.
    pub fn options(&self) -> Option<PipelineOptions> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(options));
            PipelineOptions::from_raw(ptr)
        }
    }

    /// Set the pipeline options.
    pub fn set_options(&self, options: &PipelineOptions) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOptions:), options.as_ptr());
        }
    }

    // ========== Tile Function ==========

    /// Get the tile function descriptor.
    pub fn tile_function_descriptor(&self) -> Option<FunctionDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(tileFunctionDescriptor));
            FunctionDescriptor::from_raw(ptr)
        }
    }

    /// Set the tile function descriptor.
    pub fn set_tile_function_descriptor(&self, descriptor: &FunctionDescriptor) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setTileFunctionDescriptor:),
                descriptor.as_ptr(),
            );
        }
    }

    /// Get the static linking descriptor.
    pub fn static_linking_descriptor(&self) -> Option<StaticLinkingDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(staticLinkingDescriptor));
            StaticLinkingDescriptor::from_raw(ptr)
        }
    }

    /// Set the static linking descriptor.
    pub fn set_static_linking_descriptor(&self, descriptor: &StaticLinkingDescriptor) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setStaticLinkingDescriptor:),
                descriptor.as_ptr(),
            );
        }
    }

    // ========== Color Attachments ==========

    /// Get the color attachments array (as raw pointer to MTL::TileRenderPipelineColorAttachmentDescriptorArray).
    pub fn color_attachments_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(colorAttachments)) }
    }

    // ========== Threadgroup ==========

    /// Get the maximum total threads per threadgroup.
    pub fn max_total_threads_per_threadgroup(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxTotalThreadsPerThreadgroup)) }
    }

    /// Set the maximum total threads per threadgroup.
    pub fn set_max_total_threads_per_threadgroup(&self, max: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMaxTotalThreadsPerThreadgroup:), max);
        }
    }

    /// Get the required threads per threadgroup.
    pub fn required_threads_per_threadgroup(&self) -> Size {
        unsafe { msg_send_0(self.as_ptr(), sel!(requiredThreadsPerThreadgroup)) }
    }

    /// Set the required threads per threadgroup.
    pub fn set_required_threads_per_threadgroup(&self, size: Size) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setRequiredThreadsPerThreadgroup:), size);
        }
    }

    /// Get whether threadgroup size matches tile size.
    pub fn threadgroup_size_matches_tile_size(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(threadgroupSizeMatchesTileSize)) }
    }

    /// Set whether threadgroup size matches tile size.
    pub fn set_threadgroup_size_matches_tile_size(&self, value: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setThreadgroupSizeMatchesTileSize:), value);
        }
    }

    // ========== Rasterization ==========

    /// Get the raster sample count.
    pub fn raster_sample_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(rasterSampleCount)) }
    }

    /// Set the raster sample count.
    pub fn set_raster_sample_count(&self, count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setRasterSampleCount:), count);
        }
    }

    // ========== Binary Linking ==========

    /// Get whether binary linking is supported.
    pub fn support_binary_linking(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportBinaryLinking)) }
    }

    /// Set whether binary linking is supported.
    pub fn set_support_binary_linking(&self, support: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setSupportBinaryLinking:), support);
        }
    }

    /// Reset the descriptor to its default state.
    pub fn reset(&self) {
        unsafe {
            let _: () = msg_send_0(self.as_ptr(), sel!(reset));
        }
    }
}

impl Clone for TileRenderPipelineDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for TileRenderPipelineDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for TileRenderPipelineDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for TileRenderPipelineDescriptor {}
unsafe impl Sync for TileRenderPipelineDescriptor {}

impl std::fmt::Debug for TileRenderPipelineDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TileRenderPipelineDescriptor")
            .field("label", &self.label())
            .field("raster_sample_count", &self.raster_sample_count())
            .field(
                "max_total_threads_per_threadgroup",
                &self.max_total_threads_per_threadgroup(),
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_render_pipeline_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<TileRenderPipelineDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
