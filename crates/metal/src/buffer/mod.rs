//! Metal buffer resources.
//!
//! Corresponds to `Metal/MTLBuffer.hpp`.
//!
//! Buffers store data that can be accessed by shaders. They are the most basic
//! type of Metal resource.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{BufferSparseTier, ResourceOptions};
use crate::types::ResourceID;

/// A buffer resource that stores data for shader access.
///
/// C++ equivalent: `MTL::Buffer`
///
/// Buffers can contain any type of data: vertex data, uniform data, texture data,
/// or any other data that shaders need to access.
#[repr(transparent)]
pub struct Buffer(pub(crate) NonNull<c_void>);

impl Buffer {
    /// Create a Buffer from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal buffer object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the buffer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Buffer Properties
    // =========================================================================

    /// Get the length of the buffer in bytes.
    ///
    /// C++ equivalent: `NS::UInteger length() const`
    #[inline]
    pub fn length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(length)) }
    }

    /// Get a CPU-accessible pointer to the buffer's contents.
    ///
    /// Returns `None` if the buffer is not CPU-accessible (e.g., private storage mode).
    ///
    /// C++ equivalent: `void* contents()`
    #[inline]
    pub fn contents(&self) -> Option<*mut c_void> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(contents));
            if ptr.is_null() {
                None
            } else {
                Some(ptr)
            }
        }
    }

    /// Notify Metal that the CPU has modified the contents of the buffer.
    ///
    /// Call this after modifying the buffer's contents on the CPU to ensure
    /// the GPU sees the updated data.
    ///
    /// C++ equivalent: `void didModifyRange(NS::Range range)`
    #[inline]
    pub fn did_modify_range(&self, location: UInteger, length: UInteger) {
        unsafe {
            let range = metal_foundation::Range::new(location, length);
            msg_send_1::<(), metal_foundation::Range>(
                self.as_ptr(),
                sel!(didModifyRange:),
                range,
            );
        }
    }

    /// Get the GPU address for bindless access.
    ///
    /// This returns the virtual address of the buffer in GPU memory, which
    /// can be used for bindless resource access in shaders.
    ///
    /// C++ equivalent: `uint64_t gpuAddress() const`
    #[inline]
    pub fn gpu_address(&self) -> u64 {
        unsafe { msg_send_0(self.as_ptr(), sel!(gpuAddress)) }
    }

    /// Create a texture that shares the buffer's storage.
    ///
    /// C++ equivalent: `Texture* newTexture(const TextureDescriptor*, NS::UInteger offset, NS::UInteger bytesPerRow)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn new_texture(
        &self,
        descriptor: *const c_void,
        offset: UInteger,
        bytes_per_row: UInteger,
    ) -> Option<crate::texture::Texture> {
        unsafe {
            let ptr: *mut c_void = metal_sys::msg_send_3(
                self.as_ptr(),
                sel!(newTextureWithDescriptor: offset: bytesPerRow:),
                descriptor,
                offset,
                bytes_per_row,
            );
            crate::texture::Texture::from_raw(ptr)
        }
    }

    /// Create a tensor that shares the buffer's storage.
    ///
    /// C++ equivalent: `Tensor* newTensor(const TensorDescriptor*, NS::UInteger offset, NS::UInteger bytesPerRow)`
    pub fn new_tensor(
        &self,
        descriptor: &crate::tensor::TensorDescriptor,
        offset: UInteger,
        bytes_per_row: UInteger,
    ) -> Option<crate::tensor::Tensor> {
        unsafe {
            let ptr: *mut c_void = metal_sys::msg_send_3(
                self.as_ptr(),
                sel!(newTensorWithDescriptor: offset: bytesPerRow:),
                descriptor.as_ptr(),
                offset,
                bytes_per_row,
            );
            crate::tensor::Tensor::from_raw(ptr)
        }
    }

    /// Add a debug marker to the buffer.
    ///
    /// C++ equivalent: `void addDebugMarker(const NS::String*, NS::Range)`
    pub fn add_debug_marker(&self, marker: &str, location: UInteger, length: UInteger) {
        if let Some(ns_marker) = metal_foundation::String::from_str(marker) {
            let range = metal_foundation::Range::new(location, length);
            unsafe {
                metal_sys::msg_send_2::<(), *const c_void, metal_foundation::Range>(
                    self.as_ptr(),
                    sel!(addDebugMarker: range:),
                    ns_marker.as_ptr(),
                    range,
                );
            }
        }
    }

    /// Remove all debug markers from the buffer.
    ///
    /// C++ equivalent: `void removeAllDebugMarkers()`
    #[inline]
    pub fn remove_all_debug_markers(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(removeAllDebugMarkers));
        }
    }

    /// Get a remote storage buffer (for cross-process sharing).
    ///
    /// C++ equivalent: `Buffer* remoteStorageBuffer() const`
    #[inline]
    pub fn remote_storage_buffer(&self) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(remoteStorageBuffer));
            if ptr.is_null() {
                return None;
            }
            // Retain the autoreleased object
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Buffer::from_raw(ptr)
        }
    }

    /// Create a remote buffer for a remote device.
    ///
    /// C++ equivalent: `Buffer* newRemoteBufferViewForDevice(Device*)`
    pub fn new_remote_buffer_view_for_device(&self, device: &crate::Device) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newRemoteBufferViewForDevice:),
                device.as_ptr(),
            );
            Buffer::from_raw(ptr)
        }
    }

    /// Get the GPU resource ID for bindless access.
    ///
    /// C++ equivalent: `ResourceID gpuResourceID() const`
    #[inline]
    pub fn gpu_resource_id(&self) -> ResourceID {
        unsafe { msg_send_0(self.as_ptr(), sel!(gpuResourceID)) }
    }

    /// Get the sparse buffer tier for this buffer.
    ///
    /// C++ equivalent: `BufferSparseTier sparseBufferTier() const`
    #[inline]
    pub fn sparse_buffer_tier(&self) -> BufferSparseTier {
        unsafe { msg_send_0(self.as_ptr(), sel!(sparseBufferTier)) }
    }

    // =========================================================================
    // Resource Properties (inherited from MTLResource)
    // =========================================================================

    /// Get the label for this buffer.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ptr.is_null() {
                return None;
            }
            // Get the UTF-8 string directly from the NSString
            let utf8_ptr: *const std::ffi::c_char =
                metal_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Set the label for this buffer.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the device that created this buffer.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            // Retain the autoreleased object
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("buffer has no device")
        }
    }

    /// Get the resource options used to create this buffer.
    ///
    /// C++ equivalent: `ResourceOptions resourceOptions() const`
    #[inline]
    pub fn resource_options(&self) -> ResourceOptions {
        unsafe { msg_send_0(self.as_ptr(), sel!(resourceOptions)) }
    }

    /// Get the allocated size of this buffer.
    ///
    /// C++ equivalent: `NS::UInteger allocatedSize() const`
    #[inline]
    pub fn allocated_size(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(allocatedSize)) }
    }
}

impl Clone for Buffer {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for Buffer {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

// SAFETY: Buffer is a reference-counted object that is thread-safe
unsafe impl Send for Buffer {}
unsafe impl Sync for Buffer {}

impl std::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buffer")
            .field("length", &self.length())
            .field("label", &self.label())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_size() {
        assert_eq!(
            std::mem::size_of::<Buffer>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
