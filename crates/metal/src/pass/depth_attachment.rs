//! Depth attachment descriptor for render passes.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::Texture;
use crate::enums::{LoadAction, MultisampleDepthResolveFilter, StoreAction};

/// A depth attachment descriptor for a render pass.
///
/// C++ equivalent: `MTL::RenderPassDepthAttachmentDescriptor`
#[repr(transparent)]
pub struct RenderPassDepthAttachmentDescriptor(NonNull<c_void>);

impl RenderPassDepthAttachmentDescriptor {
    /// Create a RenderPassDepthAttachmentDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal render pass depth attachment descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the clear depth value for this attachment.
    ///
    /// C++ equivalent: `double clearDepth() const`
    #[inline]
    pub fn clear_depth(&self) -> f64 {
        unsafe { msg_send_0(self.as_ptr(), sel!(clearDepth)) }
    }

    /// Set the clear depth value for this attachment.
    ///
    /// C++ equivalent: `void setClearDepth(double)`
    #[inline]
    pub fn set_clear_depth(&self, depth: f64) {
        unsafe {
            msg_send_1::<(), f64>(self.as_ptr(), sel!(setClearDepth:), depth);
        }
    }

    /// Get the depth resolve filter for this attachment.
    ///
    /// C++ equivalent: `MultisampleDepthResolveFilter depthResolveFilter() const`
    #[inline]
    pub fn depth_resolve_filter(&self) -> MultisampleDepthResolveFilter {
        unsafe { msg_send_0(self.as_ptr(), sel!(depthResolveFilter)) }
    }

    /// Set the depth resolve filter for this attachment.
    ///
    /// C++ equivalent: `void setDepthResolveFilter(MultisampleDepthResolveFilter)`
    #[inline]
    pub fn set_depth_resolve_filter(&self, filter: MultisampleDepthResolveFilter) {
        unsafe {
            msg_send_1::<(), MultisampleDepthResolveFilter>(
                self.as_ptr(),
                sel!(setDepthResolveFilter:),
                filter,
            );
        }
    }

    // Base attachment methods
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
}

impl Referencing for RenderPassDepthAttachmentDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}
