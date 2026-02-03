//! Metal sampler state.
//!
//! Corresponds to `Metal/MTLSampler.hpp`.
//!
//! Sampler states define how textures are sampled in shaders.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{
    CompareFunction, SamplerAddressMode, SamplerBorderColor, SamplerMinMagFilter, SamplerMipFilter,
    SamplerReductionMode,
};
use crate::types::ResourceID;

/// An object that defines how texture coordinates map to texels.
///
/// C++ equivalent: `MTL::SamplerState`
#[repr(transparent)]
pub struct SamplerState(pub(crate) NonNull<c_void>);

impl SamplerState {
    /// Create a SamplerState from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal sampler state object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the sampler state.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the label for this sampler state.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                mtl_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Get the device that created this sampler state.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("sampler state has no device")
        }
    }

    /// Get the GPU resource ID for bindless access.
    ///
    /// C++ equivalent: `ResourceID gpuResourceID() const`
    #[inline]
    pub fn gpu_resource_id(&self) -> ResourceID {
        unsafe { msg_send_0(self.as_ptr(), sel!(gpuResourceID)) }
    }
}

impl Clone for SamplerState {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for SamplerState {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for SamplerState {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for SamplerState {}
unsafe impl Sync for SamplerState {}

impl std::fmt::Debug for SamplerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SamplerState")
            .field("label", &self.label())
            .finish()
    }
}

// ============================================================================
// Sampler Descriptor
// ============================================================================

/// A configuration for a sampler state.
///
/// C++ equivalent: `MTL::SamplerDescriptor`
#[repr(transparent)]
pub struct SamplerDescriptor(pub(crate) NonNull<c_void>);

impl SamplerDescriptor {
    /// Create a new sampler descriptor.
    ///
    /// C++ equivalent: `static SamplerDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLSamplerDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a SamplerDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal sampler descriptor object.
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
    // Filter Properties
    // =========================================================================

    /// Get the minification filter.
    ///
    /// C++ equivalent: `SamplerMinMagFilter minFilter() const`
    #[inline]
    pub fn min_filter(&self) -> SamplerMinMagFilter {
        unsafe { msg_send_0(self.as_ptr(), sel!(minFilter)) }
    }

    /// Set the minification filter.
    ///
    /// C++ equivalent: `void setMinFilter(SamplerMinMagFilter)`
    #[inline]
    pub fn set_min_filter(&self, filter: SamplerMinMagFilter) {
        unsafe {
            msg_send_1::<(), SamplerMinMagFilter>(self.as_ptr(), sel!(setMinFilter:), filter);
        }
    }

    /// Get the magnification filter.
    ///
    /// C++ equivalent: `SamplerMinMagFilter magFilter() const`
    #[inline]
    pub fn mag_filter(&self) -> SamplerMinMagFilter {
        unsafe { msg_send_0(self.as_ptr(), sel!(magFilter)) }
    }

    /// Set the magnification filter.
    ///
    /// C++ equivalent: `void setMagFilter(SamplerMinMagFilter)`
    #[inline]
    pub fn set_mag_filter(&self, filter: SamplerMinMagFilter) {
        unsafe {
            msg_send_1::<(), SamplerMinMagFilter>(self.as_ptr(), sel!(setMagFilter:), filter);
        }
    }

    /// Get the mipmap filter.
    ///
    /// C++ equivalent: `SamplerMipFilter mipFilter() const`
    #[inline]
    pub fn mip_filter(&self) -> SamplerMipFilter {
        unsafe { msg_send_0(self.as_ptr(), sel!(mipFilter)) }
    }

    /// Set the mipmap filter.
    ///
    /// C++ equivalent: `void setMipFilter(SamplerMipFilter)`
    #[inline]
    pub fn set_mip_filter(&self, filter: SamplerMipFilter) {
        unsafe {
            msg_send_1::<(), SamplerMipFilter>(self.as_ptr(), sel!(setMipFilter:), filter);
        }
    }

    // =========================================================================
    // Address Mode Properties
    // =========================================================================

    /// Get the S (horizontal) address mode.
    ///
    /// C++ equivalent: `SamplerAddressMode sAddressMode() const`
    #[inline]
    pub fn s_address_mode(&self) -> SamplerAddressMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(sAddressMode)) }
    }

    /// Set the S (horizontal) address mode.
    ///
    /// C++ equivalent: `void setSAddressMode(SamplerAddressMode)`
    #[inline]
    pub fn set_s_address_mode(&self, mode: SamplerAddressMode) {
        unsafe {
            msg_send_1::<(), SamplerAddressMode>(self.as_ptr(), sel!(setSAddressMode:), mode);
        }
    }

    /// Get the T (vertical) address mode.
    ///
    /// C++ equivalent: `SamplerAddressMode tAddressMode() const`
    #[inline]
    pub fn t_address_mode(&self) -> SamplerAddressMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(tAddressMode)) }
    }

    /// Set the T (vertical) address mode.
    ///
    /// C++ equivalent: `void setTAddressMode(SamplerAddressMode)`
    #[inline]
    pub fn set_t_address_mode(&self, mode: SamplerAddressMode) {
        unsafe {
            msg_send_1::<(), SamplerAddressMode>(self.as_ptr(), sel!(setTAddressMode:), mode);
        }
    }

    /// Get the R (depth) address mode.
    ///
    /// C++ equivalent: `SamplerAddressMode rAddressMode() const`
    #[inline]
    pub fn r_address_mode(&self) -> SamplerAddressMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(rAddressMode)) }
    }

    /// Set the R (depth) address mode.
    ///
    /// C++ equivalent: `void setRAddressMode(SamplerAddressMode)`
    #[inline]
    pub fn set_r_address_mode(&self, mode: SamplerAddressMode) {
        unsafe {
            msg_send_1::<(), SamplerAddressMode>(self.as_ptr(), sel!(setRAddressMode:), mode);
        }
    }

    /// Get the border color.
    ///
    /// C++ equivalent: `SamplerBorderColor borderColor() const`
    #[inline]
    pub fn border_color(&self) -> SamplerBorderColor {
        unsafe { msg_send_0(self.as_ptr(), sel!(borderColor)) }
    }

    /// Set the border color.
    ///
    /// C++ equivalent: `void setBorderColor(SamplerBorderColor)`
    #[inline]
    pub fn set_border_color(&self, color: SamplerBorderColor) {
        unsafe {
            msg_send_1::<(), SamplerBorderColor>(self.as_ptr(), sel!(setBorderColor:), color);
        }
    }

    // =========================================================================
    // LOD Properties
    // =========================================================================

    /// Get the minimum LOD clamp.
    ///
    /// C++ equivalent: `float lodMinClamp() const`
    #[inline]
    pub fn lod_min_clamp(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(lodMinClamp)) }
    }

    /// Set the minimum LOD clamp.
    ///
    /// C++ equivalent: `void setLodMinClamp(float)`
    #[inline]
    pub fn set_lod_min_clamp(&self, clamp: f32) {
        unsafe {
            msg_send_1::<(), f32>(self.as_ptr(), sel!(setLodMinClamp:), clamp);
        }
    }

    /// Get the maximum LOD clamp.
    ///
    /// C++ equivalent: `float lodMaxClamp() const`
    #[inline]
    pub fn lod_max_clamp(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(lodMaxClamp)) }
    }

    /// Set the maximum LOD clamp.
    ///
    /// C++ equivalent: `void setLodMaxClamp(float)`
    #[inline]
    pub fn set_lod_max_clamp(&self, clamp: f32) {
        unsafe {
            msg_send_1::<(), f32>(self.as_ptr(), sel!(setLodMaxClamp:), clamp);
        }
    }

    /// Get the LOD bias.
    ///
    /// C++ equivalent: `float lodBias() const`
    #[inline]
    pub fn lod_bias(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(lodBias)) }
    }

    /// Set the LOD bias.
    ///
    /// C++ equivalent: `void setLodBias(float)`
    #[inline]
    pub fn set_lod_bias(&self, bias: f32) {
        unsafe {
            msg_send_1::<(), f32>(self.as_ptr(), sel!(setLodBias:), bias);
        }
    }

    /// Get whether LOD averaging is enabled.
    ///
    /// C++ equivalent: `bool lodAverage() const`
    #[inline]
    pub fn lod_average(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(lodAverage)) }
    }

    /// Set whether LOD averaging is enabled.
    ///
    /// C++ equivalent: `void setLodAverage(bool)`
    #[inline]
    pub fn set_lod_average(&self, average: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setLodAverage:), average);
        }
    }

    // =========================================================================
    // Anisotropy
    // =========================================================================

    /// Get the maximum anisotropy.
    ///
    /// C++ equivalent: `NS::UInteger maxAnisotropy() const`
    #[inline]
    pub fn max_anisotropy(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxAnisotropy)) }
    }

    /// Set the maximum anisotropy.
    ///
    /// C++ equivalent: `void setMaxAnisotropy(NS::UInteger)`
    #[inline]
    pub fn set_max_anisotropy(&self, max: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxAnisotropy:), max);
        }
    }

    // =========================================================================
    // Compare Function
    // =========================================================================

    /// Get the compare function.
    ///
    /// C++ equivalent: `CompareFunction compareFunction() const`
    #[inline]
    pub fn compare_function(&self) -> CompareFunction {
        unsafe { msg_send_0(self.as_ptr(), sel!(compareFunction)) }
    }

    /// Set the compare function.
    ///
    /// C++ equivalent: `void setCompareFunction(CompareFunction)`
    #[inline]
    pub fn set_compare_function(&self, func: CompareFunction) {
        unsafe {
            msg_send_1::<(), CompareFunction>(self.as_ptr(), sel!(setCompareFunction:), func);
        }
    }

    // =========================================================================
    // Reduction Mode
    // =========================================================================

    /// Get the reduction mode.
    ///
    /// C++ equivalent: `SamplerReductionMode reductionMode() const`
    #[inline]
    pub fn reduction_mode(&self) -> SamplerReductionMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(reductionMode)) }
    }

    /// Set the reduction mode.
    ///
    /// C++ equivalent: `void setReductionMode(SamplerReductionMode)`
    #[inline]
    pub fn set_reduction_mode(&self, mode: SamplerReductionMode) {
        unsafe {
            msg_send_1::<(), SamplerReductionMode>(self.as_ptr(), sel!(setReductionMode:), mode);
        }
    }

    // =========================================================================
    // Normalized Coordinates
    // =========================================================================

    /// Check if normalized coordinates are used.
    ///
    /// C++ equivalent: `bool normalizedCoordinates() const`
    #[inline]
    pub fn normalized_coordinates(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(normalizedCoordinates)) }
    }

    /// Set whether normalized coordinates are used.
    ///
    /// C++ equivalent: `void setNormalizedCoordinates(bool)`
    #[inline]
    pub fn set_normalized_coordinates(&self, normalized: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setNormalizedCoordinates:), normalized);
        }
    }

    // =========================================================================
    // Label
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
                mtl_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
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
        if let Some(ns_label) = mtl_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Check if LOD average is supported.
    ///
    /// C++ equivalent: `bool supportArgumentBuffers() const`
    #[inline]
    pub fn support_argument_buffers(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportArgumentBuffers)) }
    }

    /// Set whether argument buffers are supported.
    ///
    /// C++ equivalent: `void setSupportArgumentBuffers(bool)`
    #[inline]
    pub fn set_support_argument_buffers(&self, support: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setSupportArgumentBuffers:), support);
        }
    }
}

impl Default for SamplerDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create sampler descriptor")
    }
}

impl Clone for SamplerDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy sampler descriptor")
        }
    }
}

impl Drop for SamplerDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for SamplerDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for SamplerDescriptor {}
unsafe impl Sync for SamplerDescriptor {}

impl std::fmt::Debug for SamplerDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SamplerDescriptor")
            .field("min_filter", &self.min_filter())
            .field("mag_filter", &self.mag_filter())
            .field("mip_filter", &self.mip_filter())
            .field("label", &self.label())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sampler_state_size() {
        assert_eq!(
            std::mem::size_of::<SamplerState>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_sampler_descriptor_creation() {
        let desc = SamplerDescriptor::new();
        assert!(desc.is_some());
    }
}
