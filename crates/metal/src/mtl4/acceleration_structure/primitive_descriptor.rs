//! Descriptor for creating a primitive acceleration structure.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::MotionBorderMode;

/// Descriptor for creating a primitive acceleration structure.
///
/// C++ equivalent: `MTL4::PrimitiveAccelerationStructureDescriptor`
#[repr(transparent)]
pub struct PrimitiveAccelerationStructureDescriptor(NonNull<c_void>);

impl PrimitiveAccelerationStructureDescriptor {
    /// Create a PrimitiveAccelerationStructureDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new primitive acceleration structure descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4PrimitiveAccelerationStructureDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the geometry descriptors (as raw pointer to NSArray).
    ///
    /// C++ equivalent: `NS::Array* geometryDescriptors() const`
    pub fn geometry_descriptors_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(geometryDescriptors)) }
    }

    /// Set the geometry descriptors (from raw pointer to NSArray).
    ///
    /// C++ equivalent: `void setGeometryDescriptors(const NS::Array*)`
    pub fn set_geometry_descriptors_raw(&self, descriptors: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setGeometryDescriptors:), descriptors);
        }
    }

    /// Get the motion end border mode.
    ///
    /// C++ equivalent: `MTL::MotionBorderMode motionEndBorderMode() const`
    pub fn motion_end_border_mode(&self) -> MotionBorderMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionEndBorderMode)) }
    }

    /// Set the motion end border mode.
    ///
    /// C++ equivalent: `void setMotionEndBorderMode(MTL::MotionBorderMode)`
    pub fn set_motion_end_border_mode(&self, mode: MotionBorderMode) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMotionEndBorderMode:), mode);
        }
    }

    /// Get the motion end time.
    ///
    /// C++ equivalent: `float motionEndTime() const`
    pub fn motion_end_time(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionEndTime)) }
    }

    /// Set the motion end time.
    ///
    /// C++ equivalent: `void setMotionEndTime(float)`
    pub fn set_motion_end_time(&self, time: f32) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMotionEndTime:), time);
        }
    }

    /// Get the motion keyframe count.
    ///
    /// C++ equivalent: `NS::UInteger motionKeyframeCount() const`
    pub fn motion_keyframe_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionKeyframeCount)) }
    }

    /// Set the motion keyframe count.
    ///
    /// C++ equivalent: `void setMotionKeyframeCount(NS::UInteger)`
    pub fn set_motion_keyframe_count(&self, count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMotionKeyframeCount:), count);
        }
    }

    /// Get the motion start border mode.
    ///
    /// C++ equivalent: `MTL::MotionBorderMode motionStartBorderMode() const`
    pub fn motion_start_border_mode(&self) -> MotionBorderMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionStartBorderMode)) }
    }

    /// Set the motion start border mode.
    ///
    /// C++ equivalent: `void setMotionStartBorderMode(MTL::MotionBorderMode)`
    pub fn set_motion_start_border_mode(&self, mode: MotionBorderMode) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMotionStartBorderMode:), mode);
        }
    }

    /// Get the motion start time.
    ///
    /// C++ equivalent: `float motionStartTime() const`
    pub fn motion_start_time(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionStartTime)) }
    }

    /// Set the motion start time.
    ///
    /// C++ equivalent: `void setMotionStartTime(float)`
    pub fn set_motion_start_time(&self, time: f32) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMotionStartTime:), time);
        }
    }
}

impl Default for PrimitiveAccelerationStructureDescriptor {
    fn default() -> Self {
        Self::new().expect("Failed to create MTL4PrimitiveAccelerationStructureDescriptor")
    }
}

impl Clone for PrimitiveAccelerationStructureDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for PrimitiveAccelerationStructureDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for PrimitiveAccelerationStructureDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for PrimitiveAccelerationStructureDescriptor {}
unsafe impl Sync for PrimitiveAccelerationStructureDescriptor {}

impl std::fmt::Debug for PrimitiveAccelerationStructureDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimitiveAccelerationStructureDescriptor")
            .field("motion_keyframe_count", &self.motion_keyframe_count())
            .finish()
    }
}
