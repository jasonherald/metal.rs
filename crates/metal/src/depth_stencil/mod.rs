//! Metal depth/stencil state.
//!
//! Corresponds to `Metal/MTLDepthStencil.hpp`.
//!
//! Depth and stencil testing configurations.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{CompareFunction, StencilOperation};
use crate::types::ResourceID;

/// An object that contains depth and stencil test configurations.
///
/// C++ equivalent: `MTL::DepthStencilState`
#[repr(transparent)]
pub struct DepthStencilState(pub(crate) NonNull<c_void>);

impl DepthStencilState {
    /// Create a DepthStencilState from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal depth/stencil state object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the depth/stencil state.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the label for this depth/stencil state.
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

    /// Get the device that created this depth/stencil state.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("depth/stencil state has no device")
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

impl Clone for DepthStencilState {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for DepthStencilState {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for DepthStencilState {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for DepthStencilState {}
unsafe impl Sync for DepthStencilState {}

impl std::fmt::Debug for DepthStencilState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DepthStencilState")
            .field("label", &self.label())
            .finish()
    }
}

// ============================================================================
// Stencil Descriptor
// ============================================================================

/// A configuration for stencil test operations.
///
/// C++ equivalent: `MTL::StencilDescriptor`
#[repr(transparent)]
pub struct StencilDescriptor(pub(crate) NonNull<c_void>);

impl StencilDescriptor {
    /// Create a new stencil descriptor.
    ///
    /// C++ equivalent: `static StencilDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLStencilDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a StencilDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal stencil descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the stencil compare function.
    ///
    /// C++ equivalent: `CompareFunction stencilCompareFunction() const`
    #[inline]
    pub fn stencil_compare_function(&self) -> CompareFunction {
        unsafe { msg_send_0(self.as_ptr(), sel!(stencilCompareFunction)) }
    }

    /// Set the stencil compare function.
    ///
    /// C++ equivalent: `void setStencilCompareFunction(CompareFunction)`
    #[inline]
    pub fn set_stencil_compare_function(&self, func: CompareFunction) {
        unsafe {
            msg_send_1::<(), CompareFunction>(
                self.as_ptr(),
                sel!(setStencilCompareFunction:),
                func,
            );
        }
    }

    /// Get the stencil failure operation.
    ///
    /// C++ equivalent: `StencilOperation stencilFailureOperation() const`
    #[inline]
    pub fn stencil_failure_operation(&self) -> StencilOperation {
        unsafe { msg_send_0(self.as_ptr(), sel!(stencilFailureOperation)) }
    }

    /// Set the stencil failure operation.
    ///
    /// C++ equivalent: `void setStencilFailureOperation(StencilOperation)`
    #[inline]
    pub fn set_stencil_failure_operation(&self, op: StencilOperation) {
        unsafe {
            msg_send_1::<(), StencilOperation>(
                self.as_ptr(),
                sel!(setStencilFailureOperation:),
                op,
            );
        }
    }

    /// Get the depth failure operation.
    ///
    /// C++ equivalent: `StencilOperation depthFailureOperation() const`
    #[inline]
    pub fn depth_failure_operation(&self) -> StencilOperation {
        unsafe { msg_send_0(self.as_ptr(), sel!(depthFailureOperation)) }
    }

    /// Set the depth failure operation.
    ///
    /// C++ equivalent: `void setDepthFailureOperation(StencilOperation)`
    #[inline]
    pub fn set_depth_failure_operation(&self, op: StencilOperation) {
        unsafe {
            msg_send_1::<(), StencilOperation>(self.as_ptr(), sel!(setDepthFailureOperation:), op);
        }
    }

    /// Get the depth/stencil pass operation.
    ///
    /// C++ equivalent: `StencilOperation depthStencilPassOperation() const`
    #[inline]
    pub fn depth_stencil_pass_operation(&self) -> StencilOperation {
        unsafe { msg_send_0(self.as_ptr(), sel!(depthStencilPassOperation)) }
    }

    /// Set the depth/stencil pass operation.
    ///
    /// C++ equivalent: `void setDepthStencilPassOperation(StencilOperation)`
    #[inline]
    pub fn set_depth_stencil_pass_operation(&self, op: StencilOperation) {
        unsafe {
            msg_send_1::<(), StencilOperation>(
                self.as_ptr(),
                sel!(setDepthStencilPassOperation:),
                op,
            );
        }
    }

    /// Get the read mask.
    ///
    /// C++ equivalent: `uint32_t readMask() const`
    #[inline]
    pub fn read_mask(&self) -> u32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(readMask)) }
    }

    /// Set the read mask.
    ///
    /// C++ equivalent: `void setReadMask(uint32_t)`
    #[inline]
    pub fn set_read_mask(&self, mask: u32) {
        unsafe {
            msg_send_1::<(), u32>(self.as_ptr(), sel!(setReadMask:), mask);
        }
    }

    /// Get the write mask.
    ///
    /// C++ equivalent: `uint32_t writeMask() const`
    #[inline]
    pub fn write_mask(&self) -> u32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(writeMask)) }
    }

    /// Set the write mask.
    ///
    /// C++ equivalent: `void setWriteMask(uint32_t)`
    #[inline]
    pub fn set_write_mask(&self, mask: u32) {
        unsafe {
            msg_send_1::<(), u32>(self.as_ptr(), sel!(setWriteMask:), mask);
        }
    }
}

impl Default for StencilDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create stencil descriptor")
    }
}

impl Clone for StencilDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy stencil descriptor")
        }
    }
}

impl Drop for StencilDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for StencilDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for StencilDescriptor {}
unsafe impl Sync for StencilDescriptor {}

// ============================================================================
// Depth Stencil Descriptor
// ============================================================================

/// A configuration for depth and stencil test operations.
///
/// C++ equivalent: `MTL::DepthStencilDescriptor`
#[repr(transparent)]
pub struct DepthStencilDescriptor(pub(crate) NonNull<c_void>);

impl DepthStencilDescriptor {
    /// Create a new depth/stencil descriptor.
    ///
    /// C++ equivalent: `static DepthStencilDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLDepthStencilDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a DepthStencilDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal depth/stencil descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the depth compare function.
    ///
    /// C++ equivalent: `CompareFunction depthCompareFunction() const`
    #[inline]
    pub fn depth_compare_function(&self) -> CompareFunction {
        unsafe { msg_send_0(self.as_ptr(), sel!(depthCompareFunction)) }
    }

    /// Set the depth compare function.
    ///
    /// C++ equivalent: `void setDepthCompareFunction(CompareFunction)`
    #[inline]
    pub fn set_depth_compare_function(&self, func: CompareFunction) {
        unsafe {
            msg_send_1::<(), CompareFunction>(self.as_ptr(), sel!(setDepthCompareFunction:), func);
        }
    }

    /// Check if depth write is enabled.
    ///
    /// C++ equivalent: `bool isDepthWriteEnabled() const`
    #[inline]
    pub fn is_depth_write_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isDepthWriteEnabled)) }
    }

    /// Set whether depth write is enabled.
    ///
    /// C++ equivalent: `void setDepthWriteEnabled(bool)`
    #[inline]
    pub fn set_depth_write_enabled(&self, enabled: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setDepthWriteEnabled:), enabled);
        }
    }

    /// Get the front face stencil descriptor.
    ///
    /// C++ equivalent: `StencilDescriptor* frontFaceStencil() const`
    pub fn front_face_stencil(&self) -> Option<StencilDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(frontFaceStencil));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            StencilDescriptor::from_raw(ptr)
        }
    }

    /// Set the front face stencil descriptor.
    ///
    /// C++ equivalent: `void setFrontFaceStencil(StencilDescriptor*)`
    pub fn set_front_face_stencil(&self, stencil: Option<&StencilDescriptor>) {
        let ptr = stencil.map_or(std::ptr::null(), |s| s.as_ptr());
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setFrontFaceStencil:), ptr);
        }
    }

    /// Get the back face stencil descriptor.
    ///
    /// C++ equivalent: `StencilDescriptor* backFaceStencil() const`
    pub fn back_face_stencil(&self) -> Option<StencilDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(backFaceStencil));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            StencilDescriptor::from_raw(ptr)
        }
    }

    /// Set the back face stencil descriptor.
    ///
    /// C++ equivalent: `void setBackFaceStencil(StencilDescriptor*)`
    pub fn set_back_face_stencil(&self, stencil: Option<&StencilDescriptor>) {
        let ptr = stencil.map_or(std::ptr::null(), |s| s.as_ptr());
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setBackFaceStencil:), ptr);
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
}

impl Default for DepthStencilDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create depth/stencil descriptor")
    }
}

impl Clone for DepthStencilDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy depth/stencil descriptor")
        }
    }
}

impl Drop for DepthStencilDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for DepthStencilDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for DepthStencilDescriptor {}
unsafe impl Sync for DepthStencilDescriptor {}

impl std::fmt::Debug for DepthStencilDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DepthStencilDescriptor")
            .field("depth_compare_function", &self.depth_compare_function())
            .field("is_depth_write_enabled", &self.is_depth_write_enabled())
            .field("label", &self.label())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_stencil_state_size() {
        assert_eq!(
            std::mem::size_of::<DepthStencilState>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_depth_stencil_descriptor_creation() {
        let desc = DepthStencilDescriptor::new();
        assert!(desc.is_some());
    }
}
