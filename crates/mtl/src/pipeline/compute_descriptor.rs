//! Compute pipeline descriptor.
//!
//! Corresponds to `MTL::ComputePipelineDescriptor`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::ShaderValidation;
use crate::types::Size;

use super::PipelineBufferDescriptorArray;

pub struct ComputePipelineDescriptor(pub(crate) NonNull<c_void>);

impl ComputePipelineDescriptor {
    /// Allocate a new compute pipeline descriptor.
    ///
    /// C++ equivalent: `static ComputePipelineDescriptor* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let class = mtl_sys::class!(MTLComputePipelineDescriptor);
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize the descriptor.
    ///
    /// C++ equivalent: `ComputePipelineDescriptor* init()`
    pub fn init(self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            std::mem::forget(self);
            Self::from_raw(ptr)
        }
    }

    /// Create a new compute pipeline descriptor.
    pub fn new() -> Option<Self> {
        Self::alloc().and_then(|d| d.init())
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid compute pipeline descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
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

    // =========================================================================
    // Basic Properties
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
    /// C++ equivalent: `void setLabel(const NS::String* label)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = mtl_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    // =========================================================================
    // Compute Function
    // =========================================================================

    /// Get the compute function.
    ///
    /// C++ equivalent: `Function* computeFunction() const`
    pub fn compute_function(&self) -> Option<crate::Function> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(computeFunction));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Function::from_raw(ptr)
        }
    }

    /// Set the compute function.
    ///
    /// C++ equivalent: `void setComputeFunction(const MTL::Function* computeFunction)`
    pub fn set_compute_function(&self, function: Option<&crate::Function>) {
        unsafe {
            let ptr = function.map_or(std::ptr::null(), |f| f.as_ptr());
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setComputeFunction:), ptr);
        }
    }

    // =========================================================================
    // Threadgroup Configuration
    // =========================================================================

    /// Get the maximum total threads per threadgroup.
    ///
    /// C++ equivalent: `NS::UInteger maxTotalThreadsPerThreadgroup() const`
    #[inline]
    pub fn max_total_threads_per_threadgroup(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxTotalThreadsPerThreadgroup)) }
    }

    /// Set the maximum total threads per threadgroup.
    ///
    /// C++ equivalent: `void setMaxTotalThreadsPerThreadgroup(NS::UInteger maxTotalThreadsPerThreadgroup)`
    #[inline]
    pub fn set_max_total_threads_per_threadgroup(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(
                self.as_ptr(),
                sel!(setMaxTotalThreadsPerThreadgroup:),
                count,
            );
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
    /// C++ equivalent: `void setRequiredThreadsPerThreadgroup(MTL::Size requiredThreadsPerThreadgroup)`
    #[inline]
    pub fn set_required_threads_per_threadgroup(&self, size: Size) {
        unsafe {
            msg_send_1::<(), Size>(self.as_ptr(), sel!(setRequiredThreadsPerThreadgroup:), size);
        }
    }

    /// Check if thread group size is multiple of thread execution width.
    ///
    /// C++ equivalent: `bool threadGroupSizeIsMultipleOfThreadExecutionWidth() const`
    #[inline]
    pub fn thread_group_size_is_multiple_of_thread_execution_width(&self) -> bool {
        unsafe {
            msg_send_0(
                self.as_ptr(),
                sel!(threadGroupSizeIsMultipleOfThreadExecutionWidth),
            )
        }
    }

    /// Set thread group size is multiple of thread execution width.
    ///
    /// C++ equivalent: `void setThreadGroupSizeIsMultipleOfThreadExecutionWidth(bool threadGroupSizeIsMultipleOfThreadExecutionWidth)`
    #[inline]
    pub fn set_thread_group_size_is_multiple_of_thread_execution_width(&self, value: bool) {
        unsafe {
            msg_send_1::<(), bool>(
                self.as_ptr(),
                sel!(setThreadGroupSizeIsMultipleOfThreadExecutionWidth:),
                value,
            );
        }
    }

    // =========================================================================
    // Call Stack Depth
    // =========================================================================

    /// Get the maximum call stack depth.
    ///
    /// C++ equivalent: `NS::UInteger maxCallStackDepth() const`
    #[inline]
    pub fn max_call_stack_depth(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxCallStackDepth)) }
    }

    /// Set the maximum call stack depth.
    ///
    /// C++ equivalent: `void setMaxCallStackDepth(NS::UInteger maxCallStackDepth)`
    #[inline]
    pub fn set_max_call_stack_depth(&self, depth: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxCallStackDepth:), depth);
        }
    }

    // =========================================================================
    // Indirect Command Buffers
    // =========================================================================

    /// Check if the pipeline supports indirect command buffers.
    ///
    /// C++ equivalent: `bool supportIndirectCommandBuffers() const`
    #[inline]
    pub fn support_indirect_command_buffers(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportIndirectCommandBuffers)) }
    }

    /// Set indirect command buffer support.
    ///
    /// C++ equivalent: `void setSupportIndirectCommandBuffers(bool supportIndirectCommandBuffers)`
    #[inline]
    pub fn set_support_indirect_command_buffers(&self, support: bool) {
        unsafe {
            msg_send_1::<(), bool>(
                self.as_ptr(),
                sel!(setSupportIndirectCommandBuffers:),
                support,
            );
        }
    }

    /// Check if support adding binary functions is enabled.
    ///
    /// C++ equivalent: `bool supportAddingBinaryFunctions() const`
    #[inline]
    pub fn support_adding_binary_functions(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportAddingBinaryFunctions)) }
    }

    /// Set support adding binary functions.
    ///
    /// C++ equivalent: `void setSupportAddingBinaryFunctions(bool supportAddingBinaryFunctions)`
    #[inline]
    pub fn set_support_adding_binary_functions(&self, support: bool) {
        unsafe {
            msg_send_1::<(), bool>(
                self.as_ptr(),
                sel!(setSupportAddingBinaryFunctions:),
                support,
            );
        }
    }

    // =========================================================================
    // Shader Validation
    // =========================================================================

    /// Get the shader validation mode.
    ///
    /// C++ equivalent: `ShaderValidation shaderValidation() const`
    #[inline]
    pub fn shader_validation(&self) -> ShaderValidation {
        unsafe { msg_send_0(self.as_ptr(), sel!(shaderValidation)) }
    }

    /// Set the shader validation mode.
    ///
    /// C++ equivalent: `void setShaderValidation(MTL::ShaderValidation shaderValidation)`
    #[inline]
    pub fn set_shader_validation(&self, validation: ShaderValidation) {
        unsafe {
            msg_send_1::<(), ShaderValidation>(
                self.as_ptr(),
                sel!(setShaderValidation:),
                validation,
            );
        }
    }

    // =========================================================================
    // Buffer Descriptors
    // =========================================================================

    /// Get the buffer descriptors array.
    ///
    /// C++ equivalent: `PipelineBufferDescriptorArray* buffers() const`
    pub fn buffers(&self) -> Option<PipelineBufferDescriptorArray> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(buffers));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            PipelineBufferDescriptorArray::from_raw(ptr)
        }
    }

    // =========================================================================
    // Stage Input Descriptor
    // =========================================================================

    /// Get the stage input descriptor.
    ///
    /// C++ equivalent: `StageInputOutputDescriptor* stageInputDescriptor() const`
    pub fn stage_input_descriptor_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(stageInputDescriptor)) }
    }

    /// Set the stage input descriptor.
    ///
    /// C++ equivalent: `void setStageInputDescriptor(const StageInputOutputDescriptor*)`
    ///
    /// # Safety
    ///
    /// The pointer must be a valid StageInputOutputDescriptor object.
    pub unsafe fn set_stage_input_descriptor_raw(&self, descriptor: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setStageInputDescriptor:),
                descriptor,
            );
        }
    }

    // =========================================================================
    // Linked Functions
    // =========================================================================

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

    // =========================================================================
    // Binary Archives
    // =========================================================================

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

    // =========================================================================
    // Preloaded Libraries
    // =========================================================================

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

    // =========================================================================
    // Insert Libraries
    // =========================================================================

    /// Get the insert libraries (raw NSArray pointer).
    ///
    /// C++ equivalent: `NS::Array* insertLibraries() const`
    pub fn insert_libraries_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(insertLibraries)) }
    }

    /// Set the insert libraries.
    ///
    /// C++ equivalent: `void setInsertLibraries(const NS::Array*)`
    ///
    /// # Safety
    ///
    /// The pointer must be a valid NSArray of DynamicLibrary objects.
    pub unsafe fn set_insert_libraries_raw(&self, libraries: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setInsertLibraries:), libraries);
        }
    }
}

impl Clone for ComputePipelineDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("copy returned null")
        }
    }
}

impl Drop for ComputePipelineDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Default for ComputePipelineDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create compute pipeline descriptor")
    }
}

impl Referencing for ComputePipelineDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ComputePipelineDescriptor {}
unsafe impl Sync for ComputePipelineDescriptor {}

impl std::fmt::Debug for ComputePipelineDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComputePipelineDescriptor")
            .field("label", &self.label())
            .field(
                "max_total_threads_per_threadgroup",
                &self.max_total_threads_per_threadgroup(),
            )
            .finish()
    }
}
