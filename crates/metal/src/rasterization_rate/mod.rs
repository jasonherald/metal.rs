//! Rasterization rate types for variable rate shading.
//!
//! Corresponds to `Metal/MTLRasterizationRate.hpp`.
//!
//! Rasterization rate maps allow you to vary the shading rate across the render target,
//! enabling foveated rendering and other optimization techniques.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, sel, Class};

use crate::types::{Coordinate2D, Size, SizeAndAlign};
use crate::Buffer;

// ============================================================================
// RasterizationRateSampleArray
// ============================================================================

/// An array of rasterization rate samples.
///
/// C++ equivalent: `MTL::RasterizationRateSampleArray`
#[repr(transparent)]
pub struct RasterizationRateSampleArray(pub(crate) NonNull<c_void>);

impl RasterizationRateSampleArray {
    /// Create a new rasterization rate sample array.
    ///
    /// C++ equivalent: `static RasterizationRateSampleArray* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = Class::get("MTLRasterizationRateSampleArray")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the sample value at the specified index as a raw NS::Number pointer.
    ///
    /// C++ equivalent: `NS::Number* object(NS::UInteger index)`
    pub fn object_raw(&self, index: UInteger) -> *mut c_void {
        unsafe { msg_send_1(self.as_ptr(), sel!(objectAtIndexedSubscript:), index) }
    }

    /// Set the sample value at the specified index.
    ///
    /// C++ equivalent: `void setObject(const NS::Number* value, NS::UInteger index)`
    pub fn set_object_raw(&self, value: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(self.as_ptr(), sel!(setObject:atIndexedSubscript:), value, index);
        }
    }
}

impl Clone for RasterizationRateSampleArray {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for RasterizationRateSampleArray {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for RasterizationRateSampleArray {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RasterizationRateSampleArray {}
unsafe impl Sync for RasterizationRateSampleArray {}

impl std::fmt::Debug for RasterizationRateSampleArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RasterizationRateSampleArray").finish()
    }
}

// ============================================================================
// RasterizationRateLayerDescriptor
// ============================================================================

/// A descriptor for a rasterization rate layer.
///
/// C++ equivalent: `MTL::RasterizationRateLayerDescriptor`
#[repr(transparent)]
pub struct RasterizationRateLayerDescriptor(pub(crate) NonNull<c_void>);

impl RasterizationRateLayerDescriptor {
    /// Create a new rasterization rate layer descriptor.
    ///
    /// C++ equivalent: `static RasterizationRateLayerDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = Class::get("MTLRasterizationRateLayerDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new rasterization rate layer descriptor with the specified sample count.
    ///
    /// C++ equivalent: `RasterizationRateLayerDescriptor* init(MTL::Size sampleCount)`
    pub fn with_sample_count(sample_count: Size) -> Option<Self> {
        unsafe {
            let class = Class::get("MTLRasterizationRateLayerDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_1(ptr, sel!(initWithSampleCount:), sample_count);
            Self::from_raw(ptr)
        }
    }

    /// Create a new rasterization rate layer descriptor with sample count and data.
    ///
    /// C++ equivalent: `RasterizationRateLayerDescriptor* init(MTL::Size sampleCount, const float* horizontal, const float* vertical)`
    pub fn with_sample_count_and_data(
        sample_count: Size,
        horizontal: *const f32,
        vertical: *const f32,
    ) -> Option<Self> {
        unsafe {
            let class = Class::get("MTLRasterizationRateLayerDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = metal_sys::msg_send_3(
                ptr,
                sel!(initWithSampleCount:horizontal:vertical:),
                sample_count,
                horizontal,
                vertical,
            );
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
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

    /// Get the sample count.
    ///
    /// C++ equivalent: `Size sampleCount() const`
    #[inline]
    pub fn sample_count(&self) -> Size {
        unsafe { msg_send_0(self.as_ptr(), sel!(sampleCount)) }
    }

    /// Set the sample count.
    ///
    /// C++ equivalent: `void setSampleCount(MTL::Size sampleCount)`
    #[inline]
    pub fn set_sample_count(&self, sample_count: Size) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setSampleCount:), sample_count);
        }
    }

    /// Get the maximum sample count.
    ///
    /// C++ equivalent: `Size maxSampleCount() const`
    #[inline]
    pub fn max_sample_count(&self) -> Size {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxSampleCount)) }
    }

    /// Get the horizontal sample array.
    ///
    /// C++ equivalent: `RasterizationRateSampleArray* horizontal() const`
    pub fn horizontal(&self) -> RasterizationRateSampleArray {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(horizontal));
            msg_send_0::<*mut c_void>(ptr, sel!(retain));
            RasterizationRateSampleArray::from_raw(ptr)
                .expect("horizontal sample array should not be null")
        }
    }

    /// Get the vertical sample array.
    ///
    /// C++ equivalent: `RasterizationRateSampleArray* vertical() const`
    pub fn vertical(&self) -> RasterizationRateSampleArray {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(vertical));
            msg_send_0::<*mut c_void>(ptr, sel!(retain));
            RasterizationRateSampleArray::from_raw(ptr)
                .expect("vertical sample array should not be null")
        }
    }

    /// Get direct access to the horizontal sample storage.
    ///
    /// C++ equivalent: `float* horizontalSampleStorage() const`
    #[inline]
    pub fn horizontal_sample_storage(&self) -> *mut f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(horizontalSampleStorage)) }
    }

    /// Get direct access to the vertical sample storage.
    ///
    /// C++ equivalent: `float* verticalSampleStorage() const`
    #[inline]
    pub fn vertical_sample_storage(&self) -> *mut f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(verticalSampleStorage)) }
    }
}

impl Clone for RasterizationRateLayerDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for RasterizationRateLayerDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for RasterizationRateLayerDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RasterizationRateLayerDescriptor {}
unsafe impl Sync for RasterizationRateLayerDescriptor {}

impl std::fmt::Debug for RasterizationRateLayerDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RasterizationRateLayerDescriptor")
            .field("sample_count", &self.sample_count())
            .finish()
    }
}

// ============================================================================
// RasterizationRateLayerArray
// ============================================================================

/// An array of rasterization rate layer descriptors.
///
/// C++ equivalent: `MTL::RasterizationRateLayerArray`
#[repr(transparent)]
pub struct RasterizationRateLayerArray(pub(crate) NonNull<c_void>);

impl RasterizationRateLayerArray {
    /// Create a new rasterization rate layer array.
    ///
    /// C++ equivalent: `static RasterizationRateLayerArray* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = Class::get("MTLRasterizationRateLayerArray")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the layer descriptor at the specified index.
    ///
    /// C++ equivalent: `RasterizationRateLayerDescriptor* object(NS::UInteger layerIndex)`
    pub fn object(&self, layer_index: UInteger) -> Option<RasterizationRateLayerDescriptor> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(objectAtIndexedSubscript:), layer_index);
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr, sel!(retain));
            RasterizationRateLayerDescriptor::from_raw(ptr)
        }
    }

    /// Set the layer descriptor at the specified index.
    ///
    /// C++ equivalent: `void setObject(const MTL::RasterizationRateLayerDescriptor* layer, NS::UInteger layerIndex)`
    pub fn set_object(&self, layer: &RasterizationRateLayerDescriptor, layer_index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setObject:atIndexedSubscript:),
                layer.as_ptr(),
                layer_index,
            );
        }
    }
}

impl Clone for RasterizationRateLayerArray {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for RasterizationRateLayerArray {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for RasterizationRateLayerArray {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RasterizationRateLayerArray {}
unsafe impl Sync for RasterizationRateLayerArray {}

impl std::fmt::Debug for RasterizationRateLayerArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RasterizationRateLayerArray").finish()
    }
}

// ============================================================================
// RasterizationRateMapDescriptor
// ============================================================================

/// A descriptor for creating a rasterization rate map.
///
/// C++ equivalent: `MTL::RasterizationRateMapDescriptor`
#[repr(transparent)]
pub struct RasterizationRateMapDescriptor(pub(crate) NonNull<c_void>);

impl RasterizationRateMapDescriptor {
    /// Create a new rasterization rate map descriptor.
    ///
    /// C++ equivalent: `static RasterizationRateMapDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = Class::get("MTLRasterizationRateMapDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a rasterization rate map descriptor with screen size.
    ///
    /// C++ equivalent: `static RasterizationRateMapDescriptor* rasterizationRateMapDescriptor(MTL::Size screenSize)`
    pub fn with_screen_size(screen_size: Size) -> Option<Self> {
        unsafe {
            let class = Class::get("MTLRasterizationRateMapDescriptor")?;
            let ptr: *mut c_void = msg_send_1(
                class.as_ptr(),
                sel!(rasterizationRateMapDescriptorWithScreenSize:),
                screen_size,
            );
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr, sel!(retain));
            Self::from_raw(ptr)
        }
    }

    /// Create a rasterization rate map descriptor with screen size and a single layer.
    ///
    /// C++ equivalent: `static RasterizationRateMapDescriptor* rasterizationRateMapDescriptor(MTL::Size screenSize, const MTL::RasterizationRateLayerDescriptor* layer)`
    pub fn with_screen_size_and_layer(
        screen_size: Size,
        layer: &RasterizationRateLayerDescriptor,
    ) -> Option<Self> {
        unsafe {
            let class = Class::get("MTLRasterizationRateMapDescriptor")?;
            let ptr: *mut c_void = msg_send_2(
                class.as_ptr(),
                sel!(rasterizationRateMapDescriptorWithScreenSize:layer:),
                screen_size,
                layer.as_ptr(),
            );
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr, sel!(retain));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
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
    /// C++ equivalent: `void setLabel(const NS::String* label)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the screen size.
    ///
    /// C++ equivalent: `Size screenSize() const`
    #[inline]
    pub fn screen_size(&self) -> Size {
        unsafe { msg_send_0(self.as_ptr(), sel!(screenSize)) }
    }

    /// Set the screen size.
    ///
    /// C++ equivalent: `void setScreenSize(MTL::Size screenSize)`
    #[inline]
    pub fn set_screen_size(&self, screen_size: Size) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setScreenSize:), screen_size);
        }
    }

    /// Get the layer count.
    ///
    /// C++ equivalent: `NS::UInteger layerCount() const`
    #[inline]
    pub fn layer_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(layerCount)) }
    }

    /// Get the layers array.
    ///
    /// C++ equivalent: `RasterizationRateLayerArray* layers() const`
    pub fn layers(&self) -> RasterizationRateLayerArray {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(layers));
            msg_send_0::<*mut c_void>(ptr, sel!(retain));
            RasterizationRateLayerArray::from_raw(ptr).expect("layers should not be null")
        }
    }

    /// Get a layer at the specified index.
    ///
    /// C++ equivalent: `RasterizationRateLayerDescriptor* layer(NS::UInteger layerIndex)`
    pub fn layer(&self, layer_index: UInteger) -> Option<RasterizationRateLayerDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(layerAtIndex:), layer_index);
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr, sel!(retain));
            RasterizationRateLayerDescriptor::from_raw(ptr)
        }
    }

    /// Set a layer at the specified index.
    ///
    /// C++ equivalent: `void setLayer(const MTL::RasterizationRateLayerDescriptor* layer, NS::UInteger layerIndex)`
    pub fn set_layer(&self, layer: &RasterizationRateLayerDescriptor, layer_index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setLayer:atIndex:),
                layer.as_ptr(),
                layer_index,
            );
        }
    }
}

impl Clone for RasterizationRateMapDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for RasterizationRateMapDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for RasterizationRateMapDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RasterizationRateMapDescriptor {}
unsafe impl Sync for RasterizationRateMapDescriptor {}

impl std::fmt::Debug for RasterizationRateMapDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RasterizationRateMapDescriptor")
            .field("label", &self.label())
            .field("screen_size", &self.screen_size())
            .field("layer_count", &self.layer_count())
            .finish()
    }
}

// ============================================================================
// RasterizationRateMap
// ============================================================================

/// A rasterization rate map for variable rate shading.
///
/// C++ equivalent: `MTL::RasterizationRateMap`
#[repr(transparent)]
pub struct RasterizationRateMap(pub(crate) NonNull<c_void>);

impl RasterizationRateMap {
    /// Create from a raw pointer.
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

    /// Get the device that created this rasterization rate map.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("rasterization rate map has no device")
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

    /// Get the screen size.
    ///
    /// C++ equivalent: `Size screenSize() const`
    #[inline]
    pub fn screen_size(&self) -> Size {
        unsafe { msg_send_0(self.as_ptr(), sel!(screenSize)) }
    }

    /// Get the physical granularity.
    ///
    /// C++ equivalent: `Size physicalGranularity() const`
    #[inline]
    pub fn physical_granularity(&self) -> Size {
        unsafe { msg_send_0(self.as_ptr(), sel!(physicalGranularity)) }
    }

    /// Get the layer count.
    ///
    /// C++ equivalent: `NS::UInteger layerCount() const`
    #[inline]
    pub fn layer_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(layerCount)) }
    }

    /// Get the parameter buffer size and alignment.
    ///
    /// C++ equivalent: `SizeAndAlign parameterBufferSizeAndAlign() const`
    #[inline]
    pub fn parameter_buffer_size_and_align(&self) -> SizeAndAlign {
        unsafe { msg_send_0(self.as_ptr(), sel!(parameterBufferSizeAndAlign)) }
    }

    // =========================================================================
    // Methods
    // =========================================================================

    /// Get the physical size for a layer.
    ///
    /// C++ equivalent: `Size physicalSize(NS::UInteger layerIndex)`
    #[inline]
    pub fn physical_size(&self, layer_index: UInteger) -> Size {
        unsafe { msg_send_1(self.as_ptr(), sel!(physicalSizeForLayer:), layer_index) }
    }

    /// Map physical coordinates to screen coordinates.
    ///
    /// C++ equivalent: `Coordinate2D mapPhysicalToScreenCoordinates(MTL::Coordinate2D physicalCoordinates, NS::UInteger layerIndex)`
    #[inline]
    pub fn map_physical_to_screen_coordinates(
        &self,
        physical_coordinates: Coordinate2D,
        layer_index: UInteger,
    ) -> Coordinate2D {
        unsafe {
            msg_send_2(
                self.as_ptr(),
                sel!(mapPhysicalToScreenCoordinates:forLayer:),
                physical_coordinates,
                layer_index,
            )
        }
    }

    /// Map screen coordinates to physical coordinates.
    ///
    /// C++ equivalent: `Coordinate2D mapScreenToPhysicalCoordinates(MTL::Coordinate2D screenCoordinates, NS::UInteger layerIndex)`
    #[inline]
    pub fn map_screen_to_physical_coordinates(
        &self,
        screen_coordinates: Coordinate2D,
        layer_index: UInteger,
    ) -> Coordinate2D {
        unsafe {
            msg_send_2(
                self.as_ptr(),
                sel!(mapScreenToPhysicalCoordinates:forLayer:),
                screen_coordinates,
                layer_index,
            )
        }
    }

    /// Copy parameter data to a buffer.
    ///
    /// C++ equivalent: `void copyParameterDataToBuffer(const MTL::Buffer* buffer, NS::UInteger offset)`
    pub fn copy_parameter_data_to_buffer(&self, buffer: &Buffer, offset: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(copyParameterDataToBuffer:offset:),
                buffer.as_ptr(),
                offset,
            );
        }
    }
}

impl Clone for RasterizationRateMap {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for RasterizationRateMap {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for RasterizationRateMap {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for RasterizationRateMap {}
unsafe impl Sync for RasterizationRateMap {}

impl std::fmt::Debug for RasterizationRateMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RasterizationRateMap")
            .field("label", &self.label())
            .field("screen_size", &self.screen_size())
            .field("layer_count", &self.layer_count())
            .finish()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rasterization_rate_map_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<RasterizationRateMapDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_rasterization_rate_map_size() {
        assert_eq!(
            std::mem::size_of::<RasterizationRateMap>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_rasterization_rate_layer_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<RasterizationRateLayerDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
