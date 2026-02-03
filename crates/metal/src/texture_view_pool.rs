//! Metal texture view pool types.
//!
//! Corresponds to `Metal/MTLResourceViewPool.hpp` and `Metal/MTLTextureViewPool.hpp`.
//!
//! Texture view pools allow efficient management of texture views for bindless
//! rendering workflows.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{Class, msg_send_0, msg_send_1, msg_send_2, msg_send_3, sel};

use crate::types::ResourceID;
use crate::{Buffer, Texture, TextureDescriptor};

// ============================================================================
// ResourceViewPoolDescriptor
// ============================================================================

/// Descriptor for creating a resource view pool.
///
/// C++ equivalent: `MTL::ResourceViewPoolDescriptor`
#[repr(transparent)]
pub struct ResourceViewPoolDescriptor(NonNull<c_void>);

impl ResourceViewPoolDescriptor {
    /// Allocate a new resource view pool descriptor.
    ///
    /// C++ equivalent: `static ResourceViewPoolDescriptor* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let cls = Class::get("MTLResourceViewPoolDescriptor")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize an allocated descriptor.
    ///
    /// C++ equivalent: `ResourceViewPoolDescriptor* init()`
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new resource view pool descriptor.
    pub fn new() -> Option<Self> {
        Self::alloc()?.init()
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal resource view pool descriptor.
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

    /// Get the label.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                metal_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Set the label.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the resource view count.
    ///
    /// C++ equivalent: `NS::UInteger resourceViewCount() const`
    #[inline]
    pub fn resource_view_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(resourceViewCount)) }
    }

    /// Set the resource view count.
    ///
    /// C++ equivalent: `void setResourceViewCount(NS::UInteger)`
    #[inline]
    pub fn set_resource_view_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setResourceViewCount:), count);
        }
    }
}

impl Default for ResourceViewPoolDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create ResourceViewPoolDescriptor")
    }
}

impl Clone for ResourceViewPoolDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy ResourceViewPoolDescriptor")
        }
    }
}

impl Drop for ResourceViewPoolDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for ResourceViewPoolDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ResourceViewPoolDescriptor {}
unsafe impl Sync for ResourceViewPoolDescriptor {}

impl std::fmt::Debug for ResourceViewPoolDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResourceViewPoolDescriptor")
            .field("label", &self.label())
            .field("resource_view_count", &self.resource_view_count())
            .finish()
    }
}

// ============================================================================
// TextureViewPool
// ============================================================================

/// A pool for managing texture views.
///
/// C++ equivalent: `MTL::TextureViewPool`
///
/// Texture view pools allow efficient creation and management of texture views
/// for bindless rendering. They inherit from ResourceViewPool.
#[repr(transparent)]
pub struct TextureViewPool(NonNull<c_void>);

impl TextureViewPool {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal texture view pool.
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
    // ResourceViewPool inherited methods
    // =========================================================================

    /// Get the device that created this pool.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("pool has no device")
        }
    }

    /// Get the label.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                metal_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Get the resource view count.
    ///
    /// C++ equivalent: `NS::UInteger resourceViewCount() const`
    #[inline]
    pub fn resource_view_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(resourceViewCount)) }
    }

    /// Get the base resource ID for bindless access.
    ///
    /// C++ equivalent: `ResourceID baseResourceID() const`
    #[inline]
    pub fn base_resource_id(&self) -> ResourceID {
        unsafe { msg_send_0(self.as_ptr(), sel!(baseResourceID)) }
    }

    /// Copy resource views from another pool.
    ///
    /// C++ equivalent: `void copyResourceViewsFromPool(const ResourceViewPool*, NS::Range, NS::UInteger)`
    pub fn copy_resource_views_from_pool(
        &self,
        source: &TextureViewPool,
        source_location: UInteger,
        source_length: UInteger,
        destination_index: UInteger,
    ) {
        unsafe {
            let range = metal_foundation::Range::new(source_location, source_length);
            msg_send_3::<(), *const c_void, metal_foundation::Range, UInteger>(
                self.as_ptr(),
                sel!(copyResourceViewsFromPool:sourceRange:destinationIndex:),
                source.as_ptr(),
                range,
                destination_index,
            );
        }
    }

    // =========================================================================
    // TextureViewPool specific methods
    // =========================================================================

    /// Set a texture view at the specified index.
    ///
    /// C++ equivalent: `void setTextureView(const Texture*, NS::UInteger)`
    pub fn set_texture_view(&self, texture: &Texture, index: UInteger) {
        unsafe {
            msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setTextureView:atIndex:),
                texture.as_ptr(),
                index,
            );
        }
    }

    /// Set a texture view with a descriptor at the specified index.
    ///
    /// C++ equivalent: `void setTextureView(const Texture*, const TextureViewDescriptor*, NS::UInteger)`
    pub fn set_texture_view_with_descriptor(
        &self,
        texture: &Texture,
        descriptor: &crate::TextureViewDescriptor,
        index: UInteger,
    ) {
        unsafe {
            msg_send_3::<(), *const c_void, *const c_void, UInteger>(
                self.as_ptr(),
                sel!(setTextureView:descriptor:atIndex:),
                texture.as_ptr(),
                descriptor.as_ptr(),
                index,
            );
        }
    }

    /// Set a texture view from a buffer.
    ///
    /// C++ equivalent: `void setTextureViewFromBuffer(const Buffer*, const TextureDescriptor*, NS::UInteger, NS::UInteger, NS::UInteger)`
    pub fn set_texture_view_from_buffer(
        &self,
        buffer: &Buffer,
        descriptor: &TextureDescriptor,
        offset: UInteger,
        bytes_per_row: UInteger,
        index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_5::<(), *const c_void, *const c_void, UInteger, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setTextureViewFromBuffer:descriptor:offset:bytesPerRow:atIndex:),
                buffer.as_ptr(),
                descriptor.as_ptr(),
                offset,
                bytes_per_row,
                index,
            );
        }
    }

    /// Set a texture view from a buffer (raw pointer version).
    ///
    /// C++ equivalent: `void setTextureViewFromBuffer(const Buffer*, const TextureDescriptor*, NS::UInteger, NS::UInteger, NS::UInteger)`
    ///
    /// # Safety
    ///
    /// All pointers must be valid Metal objects.
    pub unsafe fn set_texture_view_from_buffer_raw(
        &self,
        buffer: *const c_void,
        descriptor: *const c_void,
        offset: UInteger,
        bytes_per_row: UInteger,
        index: UInteger,
    ) {
        unsafe {
            metal_sys::msg_send_5::<(), *const c_void, *const c_void, UInteger, UInteger, UInteger>(
                self.as_ptr(),
                sel!(setTextureViewFromBuffer:descriptor:offset:bytesPerRow:atIndex:),
                buffer,
                descriptor,
                offset,
                bytes_per_row,
                index,
            );
        }
    }
}

impl Clone for TextureViewPool {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for TextureViewPool {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for TextureViewPool {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for TextureViewPool {}
unsafe impl Sync for TextureViewPool {}

impl std::fmt::Debug for TextureViewPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextureViewPool")
            .field("label", &self.label())
            .field("resource_view_count", &self.resource_view_count())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_view_pool_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<ResourceViewPoolDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_texture_view_pool_size() {
        assert_eq!(
            std::mem::size_of::<TextureViewPool>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
