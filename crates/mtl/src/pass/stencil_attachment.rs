//! Stencil attachment descriptor for render passes.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::Texture;
use crate::enums::{LoadAction, MultisampleStencilResolveFilter, StoreAction};

/// A stencil attachment descriptor for a render pass.
///
/// C++ equivalent: `MTL::RenderPassStencilAttachmentDescriptor`
#[repr(transparent)]
pub struct RenderPassStencilAttachmentDescriptor(NonNull<c_void>);

impl RenderPassStencilAttachmentDescriptor {
    /// Create a RenderPassStencilAttachmentDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal render pass stencil attachment descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the clear stencil value for this attachment.
    ///
    /// C++ equivalent: `uint32_t clearStencil() const`
    #[inline]
    pub fn clear_stencil(&self) -> u32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(clearStencil)) }
    }

    /// Set the clear stencil value for this attachment.
    ///
    /// C++ equivalent: `void setClearStencil(uint32_t)`
    #[inline]
    pub fn set_clear_stencil(&self, stencil: u32) {
        unsafe {
            msg_send_1::<(), u32>(self.as_ptr(), sel!(setClearStencil:), stencil);
        }
    }

    /// Get the stencil resolve filter for this attachment.
    ///
    /// C++ equivalent: `MultisampleStencilResolveFilter stencilResolveFilter() const`
    #[inline]
    pub fn stencil_resolve_filter(&self) -> MultisampleStencilResolveFilter {
        unsafe { msg_send_0(self.as_ptr(), sel!(stencilResolveFilter)) }
    }

    /// Set the stencil resolve filter for this attachment.
    ///
    /// C++ equivalent: `void setStencilResolveFilter(MultisampleStencilResolveFilter)`
    #[inline]
    pub fn set_stencil_resolve_filter(&self, filter: MultisampleStencilResolveFilter) {
        unsafe {
            msg_send_1::<(), MultisampleStencilResolveFilter>(
                self.as_ptr(),
                sel!(setStencilResolveFilter:),
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

impl Referencing for RenderPassStencilAttachmentDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}
