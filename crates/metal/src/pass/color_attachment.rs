//! Color attachment descriptors for render passes.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{LoadAction, StoreAction};
use crate::types::ClearColor;
use crate::Texture;

/// A color attachment descriptor for a render pass.
///
/// C++ equivalent: `MTL::RenderPassColorAttachmentDescriptor`
#[repr(transparent)]
pub struct RenderPassColorAttachmentDescriptor(NonNull<c_void>);

impl RenderPassColorAttachmentDescriptor {
    /// Create a RenderPassColorAttachmentDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal render pass color attachment descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the clear color for this attachment.
    ///
    /// C++ equivalent: `ClearColor clearColor() const`
    #[inline]
    pub fn clear_color(&self) -> ClearColor {
        unsafe { msg_send_0(self.as_ptr(), sel!(clearColor)) }
    }

    /// Set the clear color for this attachment.
    ///
    /// C++ equivalent: `void setClearColor(ClearColor)`
    #[inline]
    pub fn set_clear_color(&self, color: ClearColor) {
        unsafe {
            msg_send_1::<(), ClearColor>(self.as_ptr(), sel!(setClearColor:), color);
        }
    }

    // Inherit all base attachment methods
    /// Get the texture for this attachment.
    pub fn texture(&self) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(texture));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Texture::from_raw(ptr)
        }
    }

    /// Set the texture for this attachment.
    pub fn set_texture(&self, texture: Option<&Texture>) {
        unsafe {
            let ptr = texture.map_or(std::ptr::null(), |t| t.as_ptr());
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setTexture:), ptr);
        }
    }

    /// Get the load action for this attachment.
    #[inline]
    pub fn load_action(&self) -> LoadAction {
        unsafe { msg_send_0(self.as_ptr(), sel!(loadAction)) }
    }

    /// Set the load action for this attachment.
    #[inline]
    pub fn set_load_action(&self, load_action: LoadAction) {
        unsafe {
            msg_send_1::<(), LoadAction>(self.as_ptr(), sel!(setLoadAction:), load_action);
        }
    }

    /// Get the store action for this attachment.
    #[inline]
    pub fn store_action(&self) -> StoreAction {
        unsafe { msg_send_0(self.as_ptr(), sel!(storeAction)) }
    }

    /// Set the store action for this attachment.
    #[inline]
    pub fn set_store_action(&self, store_action: StoreAction) {
        unsafe {
            msg_send_1::<(), StoreAction>(self.as_ptr(), sel!(setStoreAction:), store_action);
        }
    }

    /// Get the mipmap level for this attachment.
    #[inline]
    pub fn level(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(level)) }
    }

    /// Set the mipmap level for this attachment.
    #[inline]
    pub fn set_level(&self, level: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setLevel:), level);
        }
    }

    /// Get the slice for this attachment.
    #[inline]
    pub fn slice(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(slice)) }
    }

    /// Set the slice for this attachment.
    #[inline]
    pub fn set_slice(&self, slice: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setSlice:), slice);
        }
    }
}

impl Referencing for RenderPassColorAttachmentDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

/// An array of color attachment descriptors.
///
/// C++ equivalent: `MTL::RenderPassColorAttachmentDescriptorArray`
#[repr(transparent)]
pub struct RenderPassColorAttachmentDescriptorArray(NonNull<c_void>);

impl RenderPassColorAttachmentDescriptorArray {
    /// Create a RenderPassColorAttachmentDescriptorArray from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal render pass color attachment descriptor array object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the color attachment descriptor at the specified index.
    ///
    /// C++ equivalent: `RenderPassColorAttachmentDescriptor* object(NS::UInteger)`
    pub fn object_at(&self, index: UInteger) -> Option<RenderPassColorAttachmentDescriptor> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(objectAtIndexedSubscript:), index);
            RenderPassColorAttachmentDescriptor::from_raw(ptr)
        }
    }

    /// Set the color attachment descriptor at the specified index.
    ///
    /// C++ equivalent: `void setObject(const RenderPassColorAttachmentDescriptor*, NS::UInteger)`
    pub fn set_object_at(
        &self,
        attachment: Option<&RenderPassColorAttachmentDescriptor>,
        index: UInteger,
    ) {
        unsafe {
            let ptr = attachment.map_or(std::ptr::null(), |a| a.as_ptr());
            metal_sys::msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setObject: atIndexedSubscript:),
                ptr,
                index,
            );
        }
    }
}

impl Referencing for RenderPassColorAttachmentDescriptorArray {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}
