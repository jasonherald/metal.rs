//! Color attachment descriptors for render pipelines.
//!
//! Corresponds to `MTL::RenderPipelineColorAttachmentDescriptor`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{BlendFactor, BlendOperation, ColorWriteMask, PixelFormat};

/// Describes the color attachment for a render pipeline.
///
/// C++ equivalent: `MTL::RenderPipelineColorAttachmentDescriptor`
#[repr(transparent)]
pub struct RenderPipelineColorAttachmentDescriptor(pub(crate) NonNull<c_void>);

impl RenderPipelineColorAttachmentDescriptor {
    /// Allocate a new color attachment descriptor.
    ///
    /// C++ equivalent: `static RenderPipelineColorAttachmentDescriptor* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let class = mtl_sys::class!(MTLRenderPipelineColorAttachmentDescriptor);
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize the descriptor.
    ///
    /// C++ equivalent: `RenderPipelineColorAttachmentDescriptor* init()`
    pub fn init(self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            std::mem::forget(self);
            Self::from_raw(ptr)
        }
    }

    /// Create a new color attachment descriptor.
    pub fn new() -> Option<Self> {
        Self::alloc().and_then(|d| d.init())
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid color attachment descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the pixel format.
    ///
    /// C++ equivalent: `PixelFormat pixelFormat() const`
    #[inline]
    pub fn pixel_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(pixelFormat)) }
    }

    /// Set the pixel format.
    ///
    /// C++ equivalent: `void setPixelFormat(MTL::PixelFormat pixelFormat)`
    #[inline]
    pub fn set_pixel_format(&self, format: PixelFormat) {
        unsafe {
            msg_send_1::<(), PixelFormat>(self.as_ptr(), sel!(setPixelFormat:), format);
        }
    }

    /// Check if blending is enabled.
    ///
    /// C++ equivalent: `bool isBlendingEnabled() const`
    #[inline]
    pub fn is_blending_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isBlendingEnabled)) }
    }

    /// Set blending enabled.
    ///
    /// C++ equivalent: `void setBlendingEnabled(bool blendingEnabled)`
    #[inline]
    pub fn set_blending_enabled(&self, enabled: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setBlendingEnabled:), enabled);
        }
    }

    /// Get the source RGB blend factor.
    ///
    /// C++ equivalent: `BlendFactor sourceRGBBlendFactor() const`
    #[inline]
    pub fn source_rgb_blend_factor(&self) -> BlendFactor {
        unsafe { msg_send_0(self.as_ptr(), sel!(sourceRGBBlendFactor)) }
    }

    /// Set the source RGB blend factor.
    ///
    /// C++ equivalent: `void setSourceRGBBlendFactor(MTL::BlendFactor sourceRGBBlendFactor)`
    #[inline]
    pub fn set_source_rgb_blend_factor(&self, factor: BlendFactor) {
        unsafe {
            msg_send_1::<(), BlendFactor>(self.as_ptr(), sel!(setSourceRGBBlendFactor:), factor);
        }
    }

    /// Get the destination RGB blend factor.
    ///
    /// C++ equivalent: `BlendFactor destinationRGBBlendFactor() const`
    #[inline]
    pub fn destination_rgb_blend_factor(&self) -> BlendFactor {
        unsafe { msg_send_0(self.as_ptr(), sel!(destinationRGBBlendFactor)) }
    }

    /// Set the destination RGB blend factor.
    ///
    /// C++ equivalent: `void setDestinationRGBBlendFactor(MTL::BlendFactor destinationRGBBlendFactor)`
    #[inline]
    pub fn set_destination_rgb_blend_factor(&self, factor: BlendFactor) {
        unsafe {
            msg_send_1::<(), BlendFactor>(
                self.as_ptr(),
                sel!(setDestinationRGBBlendFactor:),
                factor,
            );
        }
    }

    /// Get the RGB blend operation.
    ///
    /// C++ equivalent: `BlendOperation rgbBlendOperation() const`
    #[inline]
    pub fn rgb_blend_operation(&self) -> BlendOperation {
        unsafe { msg_send_0(self.as_ptr(), sel!(rgbBlendOperation)) }
    }

    /// Set the RGB blend operation.
    ///
    /// C++ equivalent: `void setRgbBlendOperation(MTL::BlendOperation rgbBlendOperation)`
    #[inline]
    pub fn set_rgb_blend_operation(&self, operation: BlendOperation) {
        unsafe {
            msg_send_1::<(), BlendOperation>(self.as_ptr(), sel!(setRgbBlendOperation:), operation);
        }
    }

    /// Get the source alpha blend factor.
    ///
    /// C++ equivalent: `BlendFactor sourceAlphaBlendFactor() const`
    #[inline]
    pub fn source_alpha_blend_factor(&self) -> BlendFactor {
        unsafe { msg_send_0(self.as_ptr(), sel!(sourceAlphaBlendFactor)) }
    }

    /// Set the source alpha blend factor.
    ///
    /// C++ equivalent: `void setSourceAlphaBlendFactor(MTL::BlendFactor sourceAlphaBlendFactor)`
    #[inline]
    pub fn set_source_alpha_blend_factor(&self, factor: BlendFactor) {
        unsafe {
            msg_send_1::<(), BlendFactor>(self.as_ptr(), sel!(setSourceAlphaBlendFactor:), factor);
        }
    }

    /// Get the destination alpha blend factor.
    ///
    /// C++ equivalent: `BlendFactor destinationAlphaBlendFactor() const`
    #[inline]
    pub fn destination_alpha_blend_factor(&self) -> BlendFactor {
        unsafe { msg_send_0(self.as_ptr(), sel!(destinationAlphaBlendFactor)) }
    }

    /// Set the destination alpha blend factor.
    ///
    /// C++ equivalent: `void setDestinationAlphaBlendFactor(MTL::BlendFactor destinationAlphaBlendFactor)`
    #[inline]
    pub fn set_destination_alpha_blend_factor(&self, factor: BlendFactor) {
        unsafe {
            msg_send_1::<(), BlendFactor>(
                self.as_ptr(),
                sel!(setDestinationAlphaBlendFactor:),
                factor,
            );
        }
    }

    /// Get the alpha blend operation.
    ///
    /// C++ equivalent: `BlendOperation alphaBlendOperation() const`
    #[inline]
    pub fn alpha_blend_operation(&self) -> BlendOperation {
        unsafe { msg_send_0(self.as_ptr(), sel!(alphaBlendOperation)) }
    }

    /// Set the alpha blend operation.
    ///
    /// C++ equivalent: `void setAlphaBlendOperation(MTL::BlendOperation alphaBlendOperation)`
    #[inline]
    pub fn set_alpha_blend_operation(&self, operation: BlendOperation) {
        unsafe {
            msg_send_1::<(), BlendOperation>(
                self.as_ptr(),
                sel!(setAlphaBlendOperation:),
                operation,
            );
        }
    }

    /// Get the color write mask.
    ///
    /// C++ equivalent: `ColorWriteMask writeMask() const`
    #[inline]
    pub fn write_mask(&self) -> ColorWriteMask {
        unsafe { msg_send_0(self.as_ptr(), sel!(writeMask)) }
    }

    /// Set the color write mask.
    ///
    /// C++ equivalent: `void setWriteMask(MTL::ColorWriteMask writeMask)`
    #[inline]
    pub fn set_write_mask(&self, mask: ColorWriteMask) {
        unsafe {
            msg_send_1::<(), ColorWriteMask>(self.as_ptr(), sel!(setWriteMask:), mask);
        }
    }
}

impl Clone for RenderPipelineColorAttachmentDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("copy returned null")
        }
    }
}

impl Drop for RenderPipelineColorAttachmentDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Default for RenderPipelineColorAttachmentDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create color attachment descriptor")
    }
}

impl Referencing for RenderPipelineColorAttachmentDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RenderPipelineColorAttachmentDescriptor {}
unsafe impl Sync for RenderPipelineColorAttachmentDescriptor {}

impl std::fmt::Debug for RenderPipelineColorAttachmentDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderPipelineColorAttachmentDescriptor")
            .field("pixel_format", &self.pixel_format())
            .field("is_blending_enabled", &self.is_blending_enabled())
            .finish()
    }
}

// ============================================================================
// Render Pipeline Color Attachment Descriptor Array
// ============================================================================

/// Array of color attachment descriptors.
///
/// C++ equivalent: `MTL::RenderPipelineColorAttachmentDescriptorArray`
#[repr(transparent)]
pub struct RenderPipelineColorAttachmentDescriptorArray(pub(crate) NonNull<c_void>);

impl RenderPipelineColorAttachmentDescriptorArray {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid color attachment descriptor array object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the color attachment descriptor at the given index.
    ///
    /// C++ equivalent: `RenderPipelineColorAttachmentDescriptor* object(NS::UInteger attachmentIndex)`
    pub fn object(&self, index: UInteger) -> Option<RenderPipelineColorAttachmentDescriptor> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(objectAtIndexedSubscript:), index);
            if ptr.is_null() {
                return None;
            }
            // Retain because it's an internal reference
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            RenderPipelineColorAttachmentDescriptor::from_raw(ptr)
        }
    }

    /// Set the color attachment descriptor at the given index.
    ///
    /// C++ equivalent: `void setObject(const MTL::RenderPipelineColorAttachmentDescriptor* attachment, NS::UInteger attachmentIndex)`
    pub fn set_object(
        &self,
        attachment: &RenderPipelineColorAttachmentDescriptor,
        index: UInteger,
    ) {
        unsafe {
            mtl_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setObject: atIndexedSubscript:),
                attachment.as_ptr(),
                index,
            );
        }
    }
}

impl Referencing for RenderPipelineColorAttachmentDescriptorArray {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RenderPipelineColorAttachmentDescriptorArray {}
unsafe impl Sync for RenderPipelineColorAttachmentDescriptorArray {}
