//! Base class for render pass attachment descriptors.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::Texture;
use crate::enums::{LoadAction, StoreAction, StoreActionOptions};

/// Base class for render pass attachment descriptors.
///
/// C++ equivalent: `MTL::RenderPassAttachmentDescriptor`
#[repr(transparent)]
pub struct RenderPassAttachmentDescriptor(pub(crate) NonNull<c_void>);

impl RenderPassAttachmentDescriptor {
    /// Create a RenderPassAttachmentDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal render pass attachment descriptor object.
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

    /// Get the texture for this attachment.
    ///
    /// C++ equivalent: `Texture* texture() const`
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
    ///
    /// C++ equivalent: `void setTexture(const Texture*)`
    pub fn set_texture(&self, texture: Option<&Texture>) {
        unsafe {
            let ptr = texture.map_or(std::ptr::null(), |t| t.as_ptr());
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setTexture:), ptr);
        }
    }

    /// Get the mipmap level for this attachment.
    ///
    /// C++ equivalent: `NS::UInteger level() const`
    #[inline]
    pub fn level(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(level)) }
    }

    /// Set the mipmap level for this attachment.
    ///
    /// C++ equivalent: `void setLevel(NS::UInteger)`
    #[inline]
    pub fn set_level(&self, level: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setLevel:), level);
        }
    }

    /// Get the slice for this attachment.
    ///
    /// C++ equivalent: `NS::UInteger slice() const`
    #[inline]
    pub fn slice(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(slice)) }
    }

    /// Set the slice for this attachment.
    ///
    /// C++ equivalent: `void setSlice(NS::UInteger)`
    #[inline]
    pub fn set_slice(&self, slice: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setSlice:), slice);
        }
    }

    /// Get the depth plane for this attachment.
    ///
    /// C++ equivalent: `NS::UInteger depthPlane() const`
    #[inline]
    pub fn depth_plane(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(depthPlane)) }
    }

    /// Set the depth plane for this attachment.
    ///
    /// C++ equivalent: `void setDepthPlane(NS::UInteger)`
    #[inline]
    pub fn set_depth_plane(&self, depth_plane: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setDepthPlane:), depth_plane);
        }
    }

    /// Get the load action for this attachment.
    ///
    /// C++ equivalent: `LoadAction loadAction() const`
    #[inline]
    pub fn load_action(&self) -> LoadAction {
        unsafe { msg_send_0(self.as_ptr(), sel!(loadAction)) }
    }

    /// Set the load action for this attachment.
    ///
    /// C++ equivalent: `void setLoadAction(LoadAction)`
    #[inline]
    pub fn set_load_action(&self, load_action: LoadAction) {
        unsafe {
            msg_send_1::<(), LoadAction>(self.as_ptr(), sel!(setLoadAction:), load_action);
        }
    }

    /// Get the store action for this attachment.
    ///
    /// C++ equivalent: `StoreAction storeAction() const`
    #[inline]
    pub fn store_action(&self) -> StoreAction {
        unsafe { msg_send_0(self.as_ptr(), sel!(storeAction)) }
    }

    /// Set the store action for this attachment.
    ///
    /// C++ equivalent: `void setStoreAction(StoreAction)`
    #[inline]
    pub fn set_store_action(&self, store_action: StoreAction) {
        unsafe {
            msg_send_1::<(), StoreAction>(self.as_ptr(), sel!(setStoreAction:), store_action);
        }
    }

    /// Get the store action options for this attachment.
    ///
    /// C++ equivalent: `StoreActionOptions storeActionOptions() const`
    #[inline]
    pub fn store_action_options(&self) -> StoreActionOptions {
        unsafe { msg_send_0(self.as_ptr(), sel!(storeActionOptions)) }
    }

    /// Set the store action options for this attachment.
    ///
    /// C++ equivalent: `void setStoreActionOptions(StoreActionOptions)`
    #[inline]
    pub fn set_store_action_options(&self, options: StoreActionOptions) {
        unsafe {
            msg_send_1::<(), StoreActionOptions>(
                self.as_ptr(),
                sel!(setStoreActionOptions:),
                options,
            );
        }
    }

    /// Get the resolve texture for this attachment.
    ///
    /// C++ equivalent: `Texture* resolveTexture() const`
    pub fn resolve_texture(&self) -> Option<Texture> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(resolveTexture));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Texture::from_raw(ptr)
        }
    }

    /// Set the resolve texture for this attachment.
    ///
    /// C++ equivalent: `void setResolveTexture(const Texture*)`
    pub fn set_resolve_texture(&self, texture: Option<&Texture>) {
        unsafe {
            let ptr = texture.map_or(std::ptr::null(), |t| t.as_ptr());
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setResolveTexture:), ptr);
        }
    }

    /// Get the resolve level for this attachment.
    ///
    /// C++ equivalent: `NS::UInteger resolveLevel() const`
    #[inline]
    pub fn resolve_level(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(resolveLevel)) }
    }

    /// Set the resolve level for this attachment.
    ///
    /// C++ equivalent: `void setResolveLevel(NS::UInteger)`
    #[inline]
    pub fn set_resolve_level(&self, level: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setResolveLevel:), level);
        }
    }

    /// Get the resolve slice for this attachment.
    ///
    /// C++ equivalent: `NS::UInteger resolveSlice() const`
    #[inline]
    pub fn resolve_slice(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(resolveSlice)) }
    }

    /// Set the resolve slice for this attachment.
    ///
    /// C++ equivalent: `void setResolveSlice(NS::UInteger)`
    #[inline]
    pub fn set_resolve_slice(&self, slice: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setResolveSlice:), slice);
        }
    }

    /// Get the resolve depth plane for this attachment.
    ///
    /// C++ equivalent: `NS::UInteger resolveDepthPlane() const`
    #[inline]
    pub fn resolve_depth_plane(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(resolveDepthPlane)) }
    }

    /// Set the resolve depth plane for this attachment.
    ///
    /// C++ equivalent: `void setResolveDepthPlane(NS::UInteger)`
    #[inline]
    pub fn set_resolve_depth_plane(&self, depth_plane: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setResolveDepthPlane:), depth_plane);
        }
    }
}

impl Referencing for RenderPassAttachmentDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}
