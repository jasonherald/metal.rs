//! Motion keyframe data types.
//!
//! Contains `MotionKeyframeData`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::Buffer;

pub struct MotionKeyframeData(pub(crate) NonNull<c_void>);

impl MotionKeyframeData {
    /// Create new motion keyframe data.
    ///
    /// C++ equivalent: `static MotionKeyframeData* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLMotionKeyframeData")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create motion keyframe data using the class method.
    ///
    /// C++ equivalent: `static MotionKeyframeData* data()`
    pub fn data() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLMotionKeyframeData")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(data));
            if ptr.is_null() {
                return None;
            }
            // The returned object is autoreleased, so retain it
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal motion keyframe data object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the buffer.
    ///
    /// C++ equivalent: `Buffer* buffer() const`
    pub fn buffer(&self) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(buffer));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Buffer::from_raw(ptr)
        }
    }

    /// Set the buffer.
    ///
    /// C++ equivalent: `void setBuffer(Buffer*)`
    pub fn set_buffer(&self, buffer: Option<&Buffer>) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setBuffer:),
                buffer.map_or(std::ptr::null(), |b| b.as_ptr()),
            );
        }
    }

    /// Get the offset.
    ///
    /// C++ equivalent: `NS::UInteger offset() const`
    #[inline]
    pub fn offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(offset)) }
    }

    /// Set the offset.
    ///
    /// C++ equivalent: `void setOffset(NS::UInteger)`
    #[inline]
    pub fn set_offset(&self, offset: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setOffset:), offset);
        }
    }
}

impl Default for MotionKeyframeData {
    fn default() -> Self {
        Self::new().expect("failed to create motion keyframe data")
    }
}

impl Clone for MotionKeyframeData {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for MotionKeyframeData {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for MotionKeyframeData {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for MotionKeyframeData {}
unsafe impl Sync for MotionKeyframeData {}

