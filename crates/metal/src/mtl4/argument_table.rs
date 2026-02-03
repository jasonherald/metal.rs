//! MTL4 ArgumentTable implementation.
//!
//! Corresponds to `Metal/MTL4ArgumentTable.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, msg_send_3, sel};

use crate::Device;

// ============================================================
// ArgumentTableDescriptor
// ============================================================

/// Descriptor for creating an argument table.
///
/// C++ equivalent: `MTL4::ArgumentTableDescriptor`
#[repr(transparent)]
pub struct ArgumentTableDescriptor(NonNull<c_void>);

impl ArgumentTableDescriptor {
    /// Create an ArgumentTableDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new argument table descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4ArgumentTableDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the label.
    ///
    /// C++ equivalent: `NS::String* label() const`
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
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the maximum buffer bind count.
    ///
    /// C++ equivalent: `NS::UInteger maxBufferBindCount() const`
    pub fn max_buffer_bind_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxBufferBindCount)) }
    }

    /// Set the maximum buffer bind count.
    ///
    /// C++ equivalent: `void setMaxBufferBindCount(NS::UInteger)`
    pub fn set_max_buffer_bind_count(&self, count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMaxBufferBindCount:), count);
        }
    }

    /// Get the maximum texture bind count.
    ///
    /// C++ equivalent: `NS::UInteger maxTextureBindCount() const`
    pub fn max_texture_bind_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxTextureBindCount)) }
    }

    /// Set the maximum texture bind count.
    ///
    /// C++ equivalent: `void setMaxTextureBindCount(NS::UInteger)`
    pub fn set_max_texture_bind_count(&self, count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMaxTextureBindCount:), count);
        }
    }

    /// Get the maximum sampler state bind count.
    ///
    /// C++ equivalent: `NS::UInteger maxSamplerStateBindCount() const`
    pub fn max_sampler_state_bind_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxSamplerStateBindCount)) }
    }

    /// Set the maximum sampler state bind count.
    ///
    /// C++ equivalent: `void setMaxSamplerStateBindCount(NS::UInteger)`
    pub fn set_max_sampler_state_bind_count(&self, count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMaxSamplerStateBindCount:), count);
        }
    }

    /// Check if bindings should be initialized.
    ///
    /// C++ equivalent: `bool initializeBindings() const`
    pub fn initialize_bindings(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(initializeBindings)) }
    }

    /// Set whether bindings should be initialized.
    ///
    /// C++ equivalent: `void setInitializeBindings(bool)`
    pub fn set_initialize_bindings(&self, initialize: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setInitializeBindings:), initialize);
        }
    }

    /// Check if attribute strides are supported.
    ///
    /// C++ equivalent: `bool supportAttributeStrides() const`
    pub fn support_attribute_strides(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportAttributeStrides)) }
    }

    /// Set whether attribute strides are supported.
    ///
    /// C++ equivalent: `void setSupportAttributeStrides(bool)`
    pub fn set_support_attribute_strides(&self, support: bool) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setSupportAttributeStrides:), support);
        }
    }
}

impl Default for ArgumentTableDescriptor {
    fn default() -> Self {
        Self::new().expect("Failed to create MTL4ArgumentTableDescriptor")
    }
}

impl Clone for ArgumentTableDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ArgumentTableDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for ArgumentTableDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ArgumentTableDescriptor {}
unsafe impl Sync for ArgumentTableDescriptor {}

impl std::fmt::Debug for ArgumentTableDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArgumentTableDescriptor")
            .field("label", &self.label())
            .field("max_buffer_bind_count", &self.max_buffer_bind_count())
            .field("max_texture_bind_count", &self.max_texture_bind_count())
            .field("max_sampler_state_bind_count", &self.max_sampler_state_bind_count())
            .finish()
    }
}

// ============================================================
// ArgumentTable
// ============================================================

/// Argument table for GPU resource binding.
///
/// C++ equivalent: `MTL4::ArgumentTable`
///
/// ArgumentTable provides a way to bind resources (buffers, textures, samplers)
/// at specific indices for use in shaders.
#[repr(transparent)]
pub struct ArgumentTable(NonNull<c_void>);

impl ArgumentTable {
    /// Create an ArgumentTable from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the device.
    ///
    /// C++ equivalent: `MTL::Device* device() const`
    pub fn device(&self) -> Option<Device> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            Device::from_raw(ptr)
        }
    }

    /// Get the label.
    ///
    /// C++ equivalent: `NS::String* label() const`
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

    /// Set a GPU address at the specified binding index.
    ///
    /// C++ equivalent: `void setAddress(MTL::GPUAddress, NS::UInteger)`
    pub fn set_address(&self, gpu_address: u64, binding_index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setAddress:atIndex:),
                gpu_address,
                binding_index,
            );
        }
    }

    /// Set a GPU address with stride at the specified binding index.
    ///
    /// C++ equivalent: `void setAddress(MTL::GPUAddress, NS::UInteger, NS::UInteger)`
    pub fn set_address_with_stride(
        &self,
        gpu_address: u64,
        stride: UInteger,
        binding_index: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(setAddress:attributeStride:atIndex:),
                gpu_address,
                stride,
                binding_index,
            );
        }
    }

    /// Set a resource at the specified buffer index.
    ///
    /// C++ equivalent: `void setResource(MTL::ResourceID, NS::UInteger)`
    pub fn set_resource(&self, resource_id: u64, binding_index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setResource:atBufferIndex:),
                resource_id,
                binding_index,
            );
        }
    }

    /// Set a texture at the specified binding index.
    ///
    /// C++ equivalent: `void setTexture(MTL::ResourceID, NS::UInteger)`
    pub fn set_texture(&self, resource_id: u64, binding_index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setTexture:atIndex:),
                resource_id,
                binding_index,
            );
        }
    }

    /// Set a sampler state at the specified binding index.
    ///
    /// C++ equivalent: `void setSamplerState(MTL::ResourceID, NS::UInteger)`
    pub fn set_sampler_state(&self, resource_id: u64, binding_index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setSamplerState:atIndex:),
                resource_id,
                binding_index,
            );
        }
    }
}

impl Clone for ArgumentTable {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ArgumentTable {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for ArgumentTable {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ArgumentTable {}
unsafe impl Sync for ArgumentTable {}

impl std::fmt::Debug for ArgumentTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArgumentTable")
            .field("label", &self.label())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argument_table_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<ArgumentTableDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_argument_table_size() {
        assert_eq!(
            std::mem::size_of::<ArgumentTable>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
