//! Tile render pipeline types.
//!
//! Corresponds to tile rendering pipeline descriptors and attachments.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{PixelFormat, ShaderValidation};
use crate::types::Size;

use super::PipelineBufferDescriptorArray;

pub struct TileRenderPipelineColorAttachmentDescriptor(pub(crate) NonNull<c_void>);

impl TileRenderPipelineColorAttachmentDescriptor {
    /// Allocate a new tile render pipeline color attachment descriptor.
    ///
    /// C++ equivalent: `static TileRenderPipelineColorAttachmentDescriptor* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let cls = metal_sys::Class::get("MTLTileRenderPipelineColorAttachmentDescriptor")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize an allocated descriptor.
    ///
    /// C++ equivalent: `TileRenderPipelineColorAttachmentDescriptor* init()`
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new tile render pipeline color attachment descriptor.
    pub fn new() -> Option<Self> {
        Self::alloc()?.init()
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal tile render pipeline color attachment descriptor.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the pixel format.
    ///
    /// C++ equivalent: `PixelFormat pixelFormat() const`
    #[inline]
    pub fn pixel_format(&self) -> PixelFormat {
        unsafe { msg_send_0(self.as_ptr(), sel!(pixelFormat)) }
    }

    /// Set the pixel format.
    ///
    /// C++ equivalent: `void setPixelFormat(PixelFormat)`
    #[inline]
    pub fn set_pixel_format(&self, format: PixelFormat) {
        unsafe {
            msg_send_1::<(), PixelFormat>(self.as_ptr(), sel!(setPixelFormat:), format);
        }
    }
}

impl Default for TileRenderPipelineColorAttachmentDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create TileRenderPipelineColorAttachmentDescriptor")
    }
}

impl Clone for TileRenderPipelineColorAttachmentDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("copy returned null")
        }
    }
}

impl Drop for TileRenderPipelineColorAttachmentDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for TileRenderPipelineColorAttachmentDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for TileRenderPipelineColorAttachmentDescriptor {}
unsafe impl Sync for TileRenderPipelineColorAttachmentDescriptor {}

impl std::fmt::Debug for TileRenderPipelineColorAttachmentDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TileRenderPipelineColorAttachmentDescriptor")
            .field("pixel_format", &self.pixel_format())
            .finish()
    }
}

// ============================================================================
// TileRenderPipelineColorAttachmentDescriptorArray
// ============================================================================

/// An array of tile render pipeline color attachment descriptors.
///
/// C++ equivalent: `MTL::TileRenderPipelineColorAttachmentDescriptorArray`
#[repr(transparent)]
pub struct TileRenderPipelineColorAttachmentDescriptorArray(pub(crate) NonNull<c_void>);

impl TileRenderPipelineColorAttachmentDescriptorArray {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal tile render pipeline color attachment descriptor array.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the descriptor at the specified index.
    ///
    /// C++ equivalent: `TileRenderPipelineColorAttachmentDescriptor* object(NS::UInteger attachmentIndex)`
    pub fn object(&self, index: UInteger) -> Option<TileRenderPipelineColorAttachmentDescriptor> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(objectAtIndexedSubscript:), index);
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            TileRenderPipelineColorAttachmentDescriptor::from_raw(ptr)
        }
    }

    /// Set the descriptor at the specified index.
    ///
    /// C++ equivalent: `void setObject(const TileRenderPipelineColorAttachmentDescriptor*, NS::UInteger)`
    pub fn set_object(
        &self,
        descriptor: Option<&TileRenderPipelineColorAttachmentDescriptor>,
        index: UInteger,
    ) {
        let ptr = descriptor.map(|d| d.as_ptr()).unwrap_or(std::ptr::null());
        unsafe {
            let _: () = metal_sys::msg_send_2(
                self.as_ptr(),
                sel!(setObject: atIndexedSubscript:),
                ptr,
                index,
            );
        }
    }
}

impl Clone for TileRenderPipelineColorAttachmentDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for TileRenderPipelineColorAttachmentDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for TileRenderPipelineColorAttachmentDescriptorArray {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for TileRenderPipelineColorAttachmentDescriptorArray {}
unsafe impl Sync for TileRenderPipelineColorAttachmentDescriptorArray {}

impl std::fmt::Debug for TileRenderPipelineColorAttachmentDescriptorArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TileRenderPipelineColorAttachmentDescriptorArray").finish()
    }
}

// ============================================================================
// TileRenderPipelineDescriptor
// ============================================================================

/// Describes a tile render pipeline configuration.
///
/// C++ equivalent: `MTL::TileRenderPipelineDescriptor`
#[repr(transparent)]
pub struct TileRenderPipelineDescriptor(pub(crate) NonNull<c_void>);

impl TileRenderPipelineDescriptor {
    /// Allocate a new tile render pipeline descriptor.
    ///
    /// C++ equivalent: `static TileRenderPipelineDescriptor* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let cls = metal_sys::Class::get("MTLTileRenderPipelineDescriptor")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize an allocated descriptor.
    ///
    /// C++ equivalent: `TileRenderPipelineDescriptor* init()`
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new tile render pipeline descriptor.
    pub fn new() -> Option<Self> {
        Self::alloc()?.init()
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal tile render pipeline descriptor.
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

    /// Get the tile function.
    ///
    /// C++ equivalent: `Function* tileFunction() const`
    pub fn tile_function(&self) -> Option<crate::Function> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(tileFunction));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            crate::Function::from_raw(ptr)
        }
    }

    /// Set the tile function.
    ///
    /// C++ equivalent: `void setTileFunction(const Function*)`
    pub fn set_tile_function(&self, function: Option<&crate::Function>) {
        let ptr = function.map(|f| f.as_ptr()).unwrap_or(std::ptr::null());
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setTileFunction:), ptr);
        }
    }

    /// Get the raster sample count.
    ///
    /// C++ equivalent: `NS::UInteger rasterSampleCount() const`
    #[inline]
    pub fn raster_sample_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(rasterSampleCount)) }
    }

    /// Set the raster sample count.
    ///
    /// C++ equivalent: `void setRasterSampleCount(NS::UInteger)`
    #[inline]
    pub fn set_raster_sample_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setRasterSampleCount:), count);
        }
    }

    /// Get the color attachments.
    ///
    /// C++ equivalent: `TileRenderPipelineColorAttachmentDescriptorArray* colorAttachments() const`
    pub fn color_attachments(&self) -> Option<TileRenderPipelineColorAttachmentDescriptorArray> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(colorAttachments));
            TileRenderPipelineColorAttachmentDescriptorArray::from_raw(ptr)
        }
    }

    /// Get whether threadgroup size matches tile size.
    ///
    /// C++ equivalent: `bool threadgroupSizeMatchesTileSize() const`
    #[inline]
    pub fn threadgroup_size_matches_tile_size(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(threadgroupSizeMatchesTileSize)) }
    }

    /// Set whether threadgroup size matches tile size.
    ///
    /// C++ equivalent: `void setThreadgroupSizeMatchesTileSize(bool)`
    #[inline]
    pub fn set_threadgroup_size_matches_tile_size(&self, matches: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setThreadgroupSizeMatchesTileSize:), matches);
        }
    }

    /// Get the tile buffers.
    ///
    /// C++ equivalent: `PipelineBufferDescriptorArray* tileBuffers() const`
    pub fn tile_buffers(&self) -> Option<PipelineBufferDescriptorArray> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(tileBuffers));
            PipelineBufferDescriptorArray::from_raw(ptr)
        }
    }

    /// Get the max total threads per threadgroup.
    ///
    /// C++ equivalent: `NS::UInteger maxTotalThreadsPerThreadgroup() const`
    #[inline]
    pub fn max_total_threads_per_threadgroup(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxTotalThreadsPerThreadgroup)) }
    }

    /// Set the max total threads per threadgroup.
    ///
    /// C++ equivalent: `void setMaxTotalThreadsPerThreadgroup(NS::UInteger)`
    #[inline]
    pub fn set_max_total_threads_per_threadgroup(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxTotalThreadsPerThreadgroup:), count);
        }
    }

    /// Get the required threads per threadgroup.
    ///
    /// C++ equivalent: `Size requiredThreadsPerThreadgroup() const`
    #[inline]
    pub fn required_threads_per_threadgroup(&self) -> Size {
        unsafe { msg_send_0(self.as_ptr(), sel!(requiredThreadsPerThreadgroup)) }
    }

    /// Set the required threads per threadgroup.
    ///
    /// C++ equivalent: `void setRequiredThreadsPerThreadgroup(Size)`
    #[inline]
    pub fn set_required_threads_per_threadgroup(&self, size: Size) {
        unsafe {
            msg_send_1::<(), Size>(self.as_ptr(), sel!(setRequiredThreadsPerThreadgroup:), size);
        }
    }

    /// Get the max call stack depth.
    ///
    /// C++ equivalent: `NS::UInteger maxCallStackDepth() const`
    #[inline]
    pub fn max_call_stack_depth(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxCallStackDepth)) }
    }

    /// Set the max call stack depth.
    ///
    /// C++ equivalent: `void setMaxCallStackDepth(NS::UInteger)`
    #[inline]
    pub fn set_max_call_stack_depth(&self, depth: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxCallStackDepth:), depth);
        }
    }

    /// Get the linked functions.
    ///
    /// C++ equivalent: `LinkedFunctions* linkedFunctions() const`
    pub fn linked_functions(&self) -> Option<crate::LinkedFunctions> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(linkedFunctions));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            crate::LinkedFunctions::from_raw(ptr)
        }
    }

    /// Set the linked functions.
    ///
    /// C++ equivalent: `void setLinkedFunctions(const LinkedFunctions*)`
    pub fn set_linked_functions(&self, functions: Option<&crate::LinkedFunctions>) {
        let ptr = functions.map(|f| f.as_ptr()).unwrap_or(std::ptr::null());
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLinkedFunctions:), ptr);
        }
    }

    /// Get whether adding binary functions is supported.
    ///
    /// C++ equivalent: `bool supportAddingBinaryFunctions() const`
    #[inline]
    pub fn support_adding_binary_functions(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportAddingBinaryFunctions)) }
    }

    /// Set whether adding binary functions is supported.
    ///
    /// C++ equivalent: `void setSupportAddingBinaryFunctions(bool)`
    #[inline]
    pub fn set_support_adding_binary_functions(&self, support: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setSupportAddingBinaryFunctions:), support);
        }
    }

    /// Get the shader validation mode.
    ///
    /// C++ equivalent: `ShaderValidation shaderValidation() const`
    #[inline]
    pub fn shader_validation(&self) -> ShaderValidation {
        unsafe { msg_send_0(self.as_ptr(), sel!(shaderValidation)) }
    }

    /// Set the shader validation mode.
    ///
    /// C++ equivalent: `void setShaderValidation(ShaderValidation)`
    #[inline]
    pub fn set_shader_validation(&self, validation: ShaderValidation) {
        unsafe {
            msg_send_1::<(), ShaderValidation>(self.as_ptr(), sel!(setShaderValidation:), validation);
        }
    }

    /// Get the binary archives (raw NSArray pointer).
    ///
    /// C++ equivalent: `NS::Array* binaryArchives() const`
    pub fn binary_archives_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(binaryArchives)) }
    }

    /// Set the binary archives.
    ///
    /// C++ equivalent: `void setBinaryArchives(const NS::Array*)`
    ///
    /// # Safety
    ///
    /// The pointer must be a valid NSArray of BinaryArchive objects.
    pub unsafe fn set_binary_archives_raw(&self, archives: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setBinaryArchives:), archives);
        }
    }

    /// Get the preloaded libraries (raw NSArray pointer).
    ///
    /// C++ equivalent: `NS::Array* preloadedLibraries() const`
    pub fn preloaded_libraries_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(preloadedLibraries)) }
    }

    /// Set the preloaded libraries.
    ///
    /// C++ equivalent: `void setPreloadedLibraries(const NS::Array*)`
    ///
    /// # Safety
    ///
    /// The pointer must be a valid NSArray of DynamicLibrary objects.
    pub unsafe fn set_preloaded_libraries_raw(&self, libraries: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setPreloadedLibraries:), libraries);
        }
    }

    /// Reset the descriptor to default values.
    ///
    /// C++ equivalent: `void reset()`
    #[inline]
    pub fn reset(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(reset));
        }
    }
}

impl Default for TileRenderPipelineDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create TileRenderPipelineDescriptor")
    }
}

impl Clone for TileRenderPipelineDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("copy returned null")
        }
    }
}

impl Drop for TileRenderPipelineDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for TileRenderPipelineDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for TileRenderPipelineDescriptor {}
unsafe impl Sync for TileRenderPipelineDescriptor {}

impl std::fmt::Debug for TileRenderPipelineDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TileRenderPipelineDescriptor")
            .field("label", &self.label())
            .field("raster_sample_count", &self.raster_sample_count())
            .finish()
    }
}

