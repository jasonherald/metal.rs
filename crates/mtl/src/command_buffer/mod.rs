//! Metal command buffer.
//!
//! Corresponds to `Metal/MTLCommandBuffer.hpp`.
//!
//! A command buffer stores encoded commands that the GPU will execute.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, TimeInterval, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{CommandBufferErrorOption, CommandBufferStatus, DispatchType};

// ============================================================================
// CommandBufferDescriptor
// ============================================================================

/// A descriptor for configuring command buffer creation.
///
/// C++ equivalent: `MTL::CommandBufferDescriptor`
#[repr(transparent)]
pub struct CommandBufferDescriptor(NonNull<c_void>);

impl CommandBufferDescriptor {
    /// Create a new CommandBufferDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal command buffer descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the descriptor.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new command buffer descriptor.
    ///
    /// C++ equivalent: `CommandBufferDescriptor::alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::class!(MTLCommandBufferDescriptor);
            let obj: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if obj.is_null() {
                return None;
            }
            let obj: *mut c_void = msg_send_0(obj, sel!(init));
            Self::from_raw(obj)
        }
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the error options for the command buffer.
    ///
    /// C++ equivalent: `CommandBufferErrorOption errorOptions() const`
    #[inline]
    pub fn error_options(&self) -> CommandBufferErrorOption {
        unsafe { msg_send_0(self.as_ptr(), sel!(errorOptions)) }
    }

    /// Set the error options for the command buffer.
    ///
    /// C++ equivalent: `void setErrorOptions(CommandBufferErrorOption errorOptions)`
    #[inline]
    pub fn set_error_options(&self, error_options: CommandBufferErrorOption) {
        unsafe {
            msg_send_1::<(), CommandBufferErrorOption>(
                self.as_ptr(),
                sel!(setErrorOptions:),
                error_options,
            );
        }
    }

    /// Check if the command buffer retains references to resources.
    ///
    /// C++ equivalent: `bool retainedReferences() const`
    #[inline]
    pub fn retained_references(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(retainedReferences)) }
    }

    /// Set whether the command buffer retains references to resources.
    ///
    /// C++ equivalent: `void setRetainedReferences(bool retainedReferences)`
    #[inline]
    pub fn set_retained_references(&self, retained_references: bool) {
        unsafe {
            msg_send_1::<(), bool>(
                self.as_ptr(),
                sel!(setRetainedReferences:),
                retained_references,
            );
        }
    }

    /// Get the log state for the command buffer.
    ///
    /// C++ equivalent: `LogState* logState() const`
    ///
    /// Returns a raw pointer to the log state object.
    #[inline]
    pub fn log_state(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(logState)) }
    }

    /// Set the log state for the command buffer.
    ///
    /// C++ equivalent: `void setLogState(const LogState* logState)`
    ///
    /// # Safety
    ///
    /// The log_state pointer must be valid or null.
    #[inline]
    pub unsafe fn set_log_state(&self, log_state: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLogState:), log_state);
        }
    }
}

impl Default for CommandBufferDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create CommandBufferDescriptor")
    }
}

impl Clone for CommandBufferDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CommandBufferDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for CommandBufferDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CommandBufferDescriptor {}
unsafe impl Sync for CommandBufferDescriptor {}

impl std::fmt::Debug for CommandBufferDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandBufferDescriptor")
            .field("error_options", &self.error_options())
            .field("retained_references", &self.retained_references())
            .finish()
    }
}

// ============================================================================
// CommandBuffer
// ============================================================================

/// A buffer that stores encoded commands for the GPU to execute.
///
/// C++ equivalent: `MTL::CommandBuffer`
///
/// Command buffers are transient objects - you create them, encode commands,
/// commit them for execution, and then discard them.
#[repr(transparent)]
pub struct CommandBuffer(pub(crate) NonNull<c_void>);

impl CommandBuffer {
    /// Create a CommandBuffer from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal command buffer object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer to the command buffer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    // =========================================================================
    // Command Buffer Lifecycle
    // =========================================================================

    /// Commit the command buffer for execution.
    ///
    /// After calling this, you cannot encode any more commands into the buffer.
    ///
    /// C++ equivalent: `void commit()`
    #[inline]
    pub fn commit(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(commit));
        }
    }

    /// Block until the command buffer completes execution.
    ///
    /// C++ equivalent: `void waitUntilCompleted()`
    #[inline]
    pub fn wait_until_completed(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(waitUntilCompleted));
        }
    }

    /// Block until the command buffer is scheduled for execution.
    ///
    /// C++ equivalent: `void waitUntilScheduled()`
    #[inline]
    pub fn wait_until_scheduled(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(waitUntilScheduled));
        }
    }

    /// Enqueue the command buffer for execution.
    ///
    /// C++ equivalent: `void enqueue()`
    #[inline]
    pub fn enqueue(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(enqueue));
        }
    }

    // =========================================================================
    // Status and Timing
    // =========================================================================

    /// Get the current status of the command buffer.
    ///
    /// C++ equivalent: `CommandBufferStatus status() const`
    #[inline]
    pub fn status(&self) -> CommandBufferStatus {
        unsafe { msg_send_0(self.as_ptr(), sel!(status)) }
    }

    /// Get the error if the command buffer failed.
    ///
    /// C++ equivalent: `NS::Error* error() const`
    pub fn error(&self) -> Option<mtl_foundation::Error> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(error));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            mtl_foundation::Error::from_ptr(ptr)
        }
    }

    /// Get the time when the GPU started executing the command buffer.
    ///
    /// C++ equivalent: `CFTimeInterval GPUStartTime() const`
    #[inline]
    pub fn gpu_start_time(&self) -> TimeInterval {
        unsafe { msg_send_0(self.as_ptr(), sel!(GPUStartTime)) }
    }

    /// Get the time when the GPU finished executing the command buffer.
    ///
    /// C++ equivalent: `CFTimeInterval GPUEndTime() const`
    #[inline]
    pub fn gpu_end_time(&self) -> TimeInterval {
        unsafe { msg_send_0(self.as_ptr(), sel!(GPUEndTime)) }
    }

    /// Get the time when the kernel started executing the command buffer.
    ///
    /// C++ equivalent: `CFTimeInterval kernelStartTime() const`
    #[inline]
    pub fn kernel_start_time(&self) -> TimeInterval {
        unsafe { msg_send_0(self.as_ptr(), sel!(kernelStartTime)) }
    }

    /// Get the time when the kernel finished executing the command buffer.
    ///
    /// C++ equivalent: `CFTimeInterval kernelEndTime() const`
    #[inline]
    pub fn kernel_end_time(&self) -> TimeInterval {
        unsafe { msg_send_0(self.as_ptr(), sel!(kernelEndTime)) }
    }

    /// Check if the command buffer retains resources.
    ///
    /// C++ equivalent: `bool retainedReferences() const`
    #[inline]
    pub fn retained_references(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(retainedReferences)) }
    }

    // =========================================================================
    // Completion Handlers
    // =========================================================================

    /// Add a handler to be called when the command buffer completes.
    ///
    /// C++ equivalent: `void addCompletedHandler(void (^)(CommandBuffer*))`
    pub fn add_completed_handler<F>(&self, handler: F)
    where
        F: Fn(&CommandBuffer) + Send + 'static,
    {
        // Use heap-allocated block to ensure it outlives this function
        let block = mtl_sys::HeapOneArgBlock::from_fn(move |cmd_buf: *mut c_void| {
            unsafe {
                if let Some(buf) = CommandBuffer::from_raw(cmd_buf) {
                    handler(&buf);
                    // Don't drop - we don't own this reference
                    std::mem::forget(buf);
                }
            }
        });

        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(addCompletedHandler:),
                block.as_ptr(),
            );
        }

        // The block is heap-allocated and retained by Metal - don't drop the wrapper
        std::mem::forget(block);
    }

    /// Add a handler to be called when the command buffer is scheduled.
    ///
    /// C++ equivalent: `void addScheduledHandler(void (^)(CommandBuffer*))`
    pub fn add_scheduled_handler<F>(&self, handler: F)
    where
        F: Fn(&CommandBuffer) + Send + 'static,
    {
        // Use heap-allocated block to ensure it outlives this function
        let block = mtl_sys::HeapOneArgBlock::from_fn(move |cmd_buf: *mut c_void| unsafe {
            if let Some(buf) = CommandBuffer::from_raw(cmd_buf) {
                handler(&buf);
                std::mem::forget(buf);
            }
        });

        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(addScheduledHandler:),
                block.as_ptr(),
            );
        }

        // The block is heap-allocated and retained by Metal - don't drop the wrapper
        std::mem::forget(block);
    }

    // =========================================================================
    // Drawable Presentation
    // =========================================================================

    /// Schedule a drawable for presentation at the earliest opportunity.
    ///
    /// C++ equivalent: `void presentDrawable(Drawable*)`
    ///
    /// # Safety
    ///
    /// The drawable pointer must be valid.
    pub unsafe fn present_drawable(&self, drawable: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(presentDrawable:), drawable);
        }
    }

    /// Schedule a drawable for presentation at a specific time.
    ///
    /// C++ equivalent: `void presentDrawable(Drawable*, CFTimeInterval)`
    ///
    /// # Safety
    ///
    /// The drawable pointer must be valid.
    pub unsafe fn present_drawable_at_time(&self, drawable: *const c_void, time: TimeInterval) {
        unsafe {
            mtl_sys::msg_send_2::<(), *const c_void, TimeInterval>(
                self.as_ptr(),
                sel!(presentDrawable: atTime:),
                drawable,
                time,
            );
        }
    }

    /// Schedule a drawable for presentation after a minimum duration.
    ///
    /// C++ equivalent: `void presentDrawableAfterMinimumDuration(Drawable*, CFTimeInterval)`
    ///
    /// # Safety
    ///
    /// The drawable pointer must be valid.
    pub unsafe fn present_drawable_after_minimum_duration(
        &self,
        drawable: *const c_void,
        duration: TimeInterval,
    ) {
        unsafe {
            mtl_sys::msg_send_2::<(), *const c_void, TimeInterval>(
                self.as_ptr(),
                sel!(presentDrawable: afterMinimumDuration:),
                drawable,
                duration,
            );
        }
    }

    // =========================================================================
    // Event Signaling
    // =========================================================================

    /// Encode a signal for an event.
    ///
    /// C++ equivalent: `void encodeSignalEvent(const Event*, uint64_t value)`
    pub fn encode_signal_event(&self, event: &crate::sync::Event, value: u64) {
        unsafe {
            mtl_sys::msg_send_2::<(), *const c_void, u64>(
                self.as_ptr(),
                sel!(encodeSignalEvent: value:),
                event.as_ptr(),
                value,
            );
        }
    }

    /// Encode a wait for an event.
    ///
    /// C++ equivalent: `void encodeWait(const Event*, uint64_t value)`
    pub fn encode_wait_for_event(&self, event: &crate::sync::Event, value: u64) {
        unsafe {
            mtl_sys::msg_send_2::<(), *const c_void, u64>(
                self.as_ptr(),
                sel!(encodeWaitForEvent: value:),
                event.as_ptr(),
                value,
            );
        }
    }

    // =========================================================================
    // Residency Sets
    // =========================================================================

    /// Use a residency set for this command buffer.
    ///
    /// C++ equivalent: `void useResidencySet(const ResidencySet*)`
    ///
    /// # Safety
    ///
    /// The residency_set pointer must be valid.
    pub unsafe fn use_residency_set(&self, residency_set: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(useResidencySet:), residency_set);
        }
    }

    /// Use multiple residency sets for this command buffer.
    ///
    /// C++ equivalent: `void useResidencySets(const ResidencySet* const*, NS::UInteger count)`
    ///
    /// # Safety
    ///
    /// The residency_sets pointer must be valid and point to count valid pointers.
    pub unsafe fn use_residency_sets(&self, residency_sets: *const *const c_void, count: UInteger) {
        unsafe {
            mtl_sys::msg_send_2::<(), *const *const c_void, UInteger>(
                self.as_ptr(),
                sel!(useResidencySets: count:),
                residency_sets,
                count,
            );
        }
    }

    // =========================================================================
    // Error Options
    // =========================================================================

    /// Get the error options for this command buffer.
    ///
    /// C++ equivalent: `CommandBufferErrorOption errorOptions() const`
    #[inline]
    pub fn error_options(&self) -> CommandBufferErrorOption {
        unsafe { msg_send_0(self.as_ptr(), sel!(errorOptions)) }
    }

    // =========================================================================
    // Command Encoders
    // =========================================================================

    /// Create a blit command encoder.
    ///
    /// C++ equivalent: `BlitCommandEncoder* blitCommandEncoder()`
    ///
    /// Returns a raw pointer to the encoder. The caller is responsible for ending encoding.
    pub fn blit_command_encoder(&self) -> *mut c_void {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(blitCommandEncoder));
            if !ptr.is_null() {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            }
            ptr
        }
    }

    /// Create a blit command encoder with a descriptor.
    ///
    /// C++ equivalent: `BlitCommandEncoder* blitCommandEncoder(const BlitPassDescriptor*)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn blit_command_encoder_with_descriptor(
        &self,
        descriptor: *const c_void,
    ) -> *mut c_void {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(blitCommandEncoderWithDescriptor:),
                descriptor,
            );
            if !ptr.is_null() {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            }
            ptr
        }
    }

    /// Create a compute command encoder.
    ///
    /// C++ equivalent: `ComputeCommandEncoder* computeCommandEncoder()`
    ///
    /// Returns a raw pointer to the encoder. The caller is responsible for ending encoding.
    pub fn compute_command_encoder(&self) -> *mut c_void {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(computeCommandEncoder));
            if !ptr.is_null() {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            }
            ptr
        }
    }

    /// Create a compute command encoder with a dispatch type.
    ///
    /// C++ equivalent: `ComputeCommandEncoder* computeCommandEncoder(DispatchType)`
    ///
    /// Returns a raw pointer to the encoder. The caller is responsible for ending encoding.
    pub fn compute_command_encoder_with_dispatch_type(
        &self,
        dispatch_type: DispatchType,
    ) -> *mut c_void {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(computeCommandEncoderWithDispatchType:),
                dispatch_type,
            );
            if !ptr.is_null() {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            }
            ptr
        }
    }

    /// Create a compute command encoder with a descriptor.
    ///
    /// C++ equivalent: `ComputeCommandEncoder* computeCommandEncoder(const ComputePassDescriptor*)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn compute_command_encoder_with_descriptor(
        &self,
        descriptor: *const c_void,
    ) -> *mut c_void {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(computeCommandEncoderWithDescriptor:),
                descriptor,
            );
            if !ptr.is_null() {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            }
            ptr
        }
    }

    /// Create a render command encoder.
    ///
    /// C++ equivalent: `RenderCommandEncoder* renderCommandEncoder(const RenderPassDescriptor*)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn render_command_encoder_with_descriptor(
        &self,
        descriptor: *const c_void,
    ) -> *mut c_void {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(renderCommandEncoderWithDescriptor:),
                descriptor,
            );
            if !ptr.is_null() {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            }
            ptr
        }
    }

    /// Create a parallel render command encoder.
    ///
    /// C++ equivalent: `ParallelRenderCommandEncoder* parallelRenderCommandEncoder(const RenderPassDescriptor*)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn parallel_render_command_encoder_with_descriptor(
        &self,
        descriptor: *const c_void,
    ) -> *mut c_void {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(parallelRenderCommandEncoderWithDescriptor:),
                descriptor,
            );
            if !ptr.is_null() {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            }
            ptr
        }
    }

    /// Create a resource state command encoder.
    ///
    /// C++ equivalent: `ResourceStateCommandEncoder* resourceStateCommandEncoder()`
    ///
    /// Returns a raw pointer to the encoder. The caller is responsible for ending encoding.
    pub fn resource_state_command_encoder(&self) -> *mut c_void {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(resourceStateCommandEncoder));
            if !ptr.is_null() {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            }
            ptr
        }
    }

    /// Create a resource state command encoder with a descriptor.
    ///
    /// C++ equivalent: `ResourceStateCommandEncoder* resourceStateCommandEncoder(const ResourceStatePassDescriptor*)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn resource_state_command_encoder_with_descriptor(
        &self,
        descriptor: *const c_void,
    ) -> *mut c_void {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(resourceStateCommandEncoderWithDescriptor:),
                descriptor,
            );
            if !ptr.is_null() {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            }
            ptr
        }
    }

    /// Create an acceleration structure command encoder.
    ///
    /// C++ equivalent: `AccelerationStructureCommandEncoder* accelerationStructureCommandEncoder()`
    ///
    /// Returns a raw pointer to the encoder. The caller is responsible for ending encoding.
    pub fn acceleration_structure_command_encoder(&self) -> *mut c_void {
        unsafe {
            let ptr: *mut c_void =
                msg_send_0(self.as_ptr(), sel!(accelerationStructureCommandEncoder));
            if !ptr.is_null() {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            }
            ptr
        }
    }

    /// Create an acceleration structure command encoder with a descriptor.
    ///
    /// C++ equivalent: `AccelerationStructureCommandEncoder* accelerationStructureCommandEncoder(const AccelerationStructurePassDescriptor*)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn acceleration_structure_command_encoder_with_descriptor(
        &self,
        descriptor: *const c_void,
    ) -> *mut c_void {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(accelerationStructureCommandEncoderWithDescriptor:),
                descriptor,
            );
            if !ptr.is_null() {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            }
            ptr
        }
    }

    // =========================================================================
    // Safe Encoder Creation (with typed descriptors)
    // =========================================================================

    /// Create a render command encoder with a typed descriptor.
    ///
    /// C++ equivalent: `RenderCommandEncoder* renderCommandEncoder(const RenderPassDescriptor*)`
    ///
    /// Returns a raw pointer to the encoder. The caller is responsible for ending encoding.
    pub fn render_command_encoder(
        &self,
        descriptor: &crate::pass::RenderPassDescriptor,
    ) -> *mut c_void {
        unsafe { self.render_command_encoder_with_descriptor(descriptor.as_ptr()) }
    }

    /// Create a parallel render command encoder with a typed descriptor.
    ///
    /// C++ equivalent: `ParallelRenderCommandEncoder* parallelRenderCommandEncoder(const RenderPassDescriptor*)`
    ///
    /// Returns a raw pointer to the encoder. The caller is responsible for ending encoding.
    pub fn parallel_render_command_encoder(
        &self,
        descriptor: &crate::pass::RenderPassDescriptor,
    ) -> *mut c_void {
        unsafe { self.parallel_render_command_encoder_with_descriptor(descriptor.as_ptr()) }
    }

    /// Create a compute command encoder with a typed descriptor.
    ///
    /// C++ equivalent: `ComputeCommandEncoder* computeCommandEncoder(const ComputePassDescriptor*)`
    ///
    /// Returns a raw pointer to the encoder. The caller is responsible for ending encoding.
    pub fn compute_command_encoder_with_pass_descriptor(
        &self,
        descriptor: &crate::pass::ComputePassDescriptor,
    ) -> *mut c_void {
        unsafe { self.compute_command_encoder_with_descriptor(descriptor.as_ptr()) }
    }

    /// Create a blit command encoder with a typed descriptor.
    ///
    /// C++ equivalent: `BlitCommandEncoder* blitCommandEncoder(const BlitPassDescriptor*)`
    ///
    /// Returns a raw pointer to the encoder. The caller is responsible for ending encoding.
    pub fn blit_command_encoder_with_pass_descriptor(
        &self,
        descriptor: &crate::pass::BlitPassDescriptor,
    ) -> *mut c_void {
        unsafe { self.blit_command_encoder_with_descriptor(descriptor.as_ptr()) }
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Get the label for this command buffer.
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

    /// Set the label for this command buffer.
    ///
    /// C++ equivalent: `void setLabel(const NS::String*)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = mtl_foundation::String::from_str(label) {
            unsafe {
                msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Get the device that created this command buffer.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::Device::from_raw(ptr).expect("command buffer has no device")
        }
    }

    /// Get the command queue that created this command buffer.
    ///
    /// C++ equivalent: `CommandQueue* commandQueue() const`
    pub fn command_queue(&self) -> crate::command_queue::CommandQueue {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(commandQueue));
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            crate::command_queue::CommandQueue::from_raw(ptr)
                .expect("command buffer has no command queue")
        }
    }

    // =========================================================================
    // Debug
    // =========================================================================

    /// Push a debug group.
    ///
    /// C++ equivalent: `void pushDebugGroup(const NS::String*)`
    pub fn push_debug_group(&self, name: &str) {
        if let Some(ns_name) = mtl_foundation::String::from_str(name) {
            unsafe {
                msg_send_1::<(), *const c_void>(
                    self.as_ptr(),
                    sel!(pushDebugGroup:),
                    ns_name.as_ptr(),
                );
            }
        }
    }

    /// Pop a debug group.
    ///
    /// C++ equivalent: `void popDebugGroup()`
    #[inline]
    pub fn pop_debug_group(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(popDebugGroup));
        }
    }

    // =========================================================================
    // Logging
    // =========================================================================

    /// Get the log container for this command buffer.
    ///
    /// C++ equivalent: `LogContainer* logs() const`
    ///
    /// Returns a container of function logs generated during command buffer execution.
    /// The log container conforms to FastEnumeration and can be iterated.
    pub fn logs(&self) -> Option<crate::function_log::LogContainer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(logs));
            crate::function_log::LogContainer::from_raw(ptr)
        }
    }
}

impl Clone for CommandBuffer {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for CommandBuffer {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CommandBuffer {}
unsafe impl Sync for CommandBuffer {}

impl std::fmt::Debug for CommandBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandBuffer")
            .field("status", &self.status())
            .field("label", &self.label())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_buffer_size() {
        assert_eq!(
            std::mem::size_of::<CommandBuffer>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
