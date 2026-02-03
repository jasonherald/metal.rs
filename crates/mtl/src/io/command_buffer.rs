//! IO command buffer for Metal.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, msg_send_2, msg_send_5, msg_send_9, sel};

use crate::buffer::Buffer;
use crate::enums::IOStatus;
use crate::sync::SharedEvent;
use crate::texture::Texture;
use crate::types::{Origin, Size};

use super::IOFileHandle;

/// Command buffer for IO operations.
///
/// C++ equivalent: `MTL::IOCommandBuffer`
#[repr(transparent)]
pub struct IOCommandBuffer(pub(crate) NonNull<c_void>);

impl IOCommandBuffer {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal IO command buffer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Add a barrier.
    ///
    /// C++ equivalent: `void addBarrier()`
    #[inline]
    pub fn add_barrier(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(addBarrier));
        }
    }

    /// Commit the command buffer.
    ///
    /// C++ equivalent: `void commit()`
    #[inline]
    pub fn commit(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(commit));
        }
    }

    /// Copy the status to a buffer.
    ///
    /// C++ equivalent: `void copyStatusToBuffer(const Buffer*, NS::UInteger)`
    pub fn copy_status_to_buffer(&self, buffer: &Buffer, offset: UInteger) {
        unsafe {
            msg_send_2::<(), *const c_void, UInteger>(
                self.as_ptr(),
                sel!(copyStatusToBuffer:offset:),
                buffer.as_ptr(),
                offset,
            );
        }
    }

    /// Enqueue the command buffer.
    ///
    /// C++ equivalent: `void enqueue()`
    #[inline]
    pub fn enqueue(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(enqueue));
        }
    }

    /// Get the error, if any.
    ///
    /// C++ equivalent: `NS::Error* error() const`
    pub fn error(&self) -> Option<mtl_foundation::Error> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(error));
            mtl_foundation::Error::from_ptr(ptr)
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

    /// Load data from a file into a buffer.
    ///
    /// C++ equivalent: `void loadBuffer(const Buffer*, NS::UInteger, NS::UInteger, const IOFileHandle*, NS::UInteger)`
    pub fn load_buffer(
        &self,
        buffer: &Buffer,
        offset: UInteger,
        size: UInteger,
        source_handle: &IOFileHandle,
        source_handle_offset: UInteger,
    ) {
        unsafe {
            msg_send_5::<(), *const c_void, UInteger, UInteger, *const c_void, UInteger>(
                self.as_ptr(),
                sel!(loadBuffer:offset:size:sourceHandle:sourceHandleOffset:),
                buffer.as_ptr(),
                offset,
                size,
                source_handle.as_ptr(),
                source_handle_offset,
            );
        }
    }

    /// Load data from a file into memory.
    ///
    /// # Safety
    ///
    /// The pointer must point to valid memory of at least `size` bytes.
    ///
    /// C++ equivalent: `void loadBytes(const void*, NS::UInteger, const IOFileHandle*, NS::UInteger)`
    pub unsafe fn load_bytes(
        &self,
        pointer: *mut c_void,
        size: UInteger,
        source_handle: &IOFileHandle,
        source_handle_offset: UInteger,
    ) {
        unsafe {
            mtl_sys::msg_send_4::<(), *mut c_void, UInteger, *const c_void, UInteger>(
                self.as_ptr(),
                sel!(loadBytes:size:sourceHandle:sourceHandleOffset:),
                pointer,
                size,
                source_handle.as_ptr(),
                source_handle_offset,
            );
        }
    }

    /// Load data from a file into a texture.
    ///
    /// C++ equivalent: `void loadTexture(const Texture*, NS::UInteger, NS::UInteger, Size, NS::UInteger, NS::UInteger, Origin, const IOFileHandle*, NS::UInteger)`
    #[allow(clippy::too_many_arguments)]
    pub fn load_texture(
        &self,
        texture: &Texture,
        slice: UInteger,
        level: UInteger,
        size: Size,
        source_bytes_per_row: UInteger,
        source_bytes_per_image: UInteger,
        destination_origin: Origin,
        source_handle: &IOFileHandle,
        source_handle_offset: UInteger,
    ) {
        unsafe {
            msg_send_9::<
                (),
                *const c_void,
                UInteger,
                UInteger,
                Size,
                UInteger,
                UInteger,
                Origin,
                *const c_void,
                UInteger,
            >(
                self.as_ptr(),
                sel!(loadTexture:slice:level:size:sourceBytesPerRow:sourceBytesPerImage:destinationOrigin:sourceHandle:sourceHandleOffset:),
                texture.as_ptr(),
                slice,
                level,
                size,
                source_bytes_per_row,
                source_bytes_per_image,
                destination_origin,
                source_handle.as_ptr(),
                source_handle_offset,
            );
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

    /// Signal a shared event.
    ///
    /// C++ equivalent: `void signalEvent(const SharedEvent*, uint64_t)`
    pub fn signal_event(&self, event: &SharedEvent, value: u64) {
        unsafe {
            msg_send_2::<(), *const c_void, u64>(
                self.as_ptr(),
                sel!(signalEvent:value:),
                event.as_ptr(),
                value,
            );
        }
    }

    /// Get the status.
    ///
    /// C++ equivalent: `IOStatus status() const`
    #[inline]
    pub fn status(&self) -> IOStatus {
        unsafe { msg_send_0(self.as_ptr(), sel!(status)) }
    }

    /// Try to cancel the command buffer.
    ///
    /// C++ equivalent: `void tryCancel()`
    #[inline]
    pub fn try_cancel(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(tryCancel));
        }
    }

    /// Wait for a shared event to reach a value.
    ///
    /// C++ equivalent: `void wait(const SharedEvent*, uint64_t)`
    pub fn wait(&self, event: &SharedEvent, value: u64) {
        unsafe {
            msg_send_2::<(), *const c_void, u64>(
                self.as_ptr(),
                sel!(waitForEvent:value:),
                event.as_ptr(),
                value,
            );
        }
    }

    /// Wait until the command buffer completes.
    ///
    /// C++ equivalent: `void waitUntilCompleted()`
    #[inline]
    pub fn wait_until_completed(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(waitUntilCompleted));
        }
    }

    /// Add a completed handler (raw block pointer).
    ///
    /// # Safety
    ///
    /// The block pointer must be a valid Objective-C block.
    ///
    /// C++ equivalent: `void addCompletedHandler(const IOCommandBufferHandler)`
    pub unsafe fn add_completed_handler_ptr(&self, block: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(addCompletedHandler:), block);
        }
    }

    /// Add a handler to be called when the IO command buffer completes.
    ///
    /// C++ equivalent: `void addCompletedHandler(const IOCommandBufferHandler)`
    ///
    /// The handler is called with a reference to the completed IO command buffer.
    pub fn add_completed_handler<F>(&self, handler: F)
    where
        F: Fn(&IOCommandBuffer) + Send + 'static,
    {
        let block = mtl_sys::OneArgBlock::from_fn(move |cmd_buf: *mut c_void| {
            unsafe {
                if let Some(buf) = IOCommandBuffer::from_raw(cmd_buf) {
                    handler(&buf);
                    // Don't drop - Metal owns this reference
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

        // The block is retained by Metal
        std::mem::forget(block);
    }
}

impl Clone for IOCommandBuffer {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for IOCommandBuffer {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for IOCommandBuffer {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for IOCommandBuffer {}
unsafe impl Sync for IOCommandBuffer {}
