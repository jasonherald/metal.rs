//! Acceleration structure descriptors.
//!
//! Contains base `AccelerationStructureDescriptor` and `PrimitiveAccelerationStructureDescriptor`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{AccelerationStructureUsage, MotionBorderMode};

pub struct AccelerationStructureDescriptor(pub(crate) NonNull<c_void>);

impl AccelerationStructureDescriptor {
    /// Create an AccelerationStructureDescriptor from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal acceleration structure descriptor object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the usage flags.
    ///
    /// C++ equivalent: `AccelerationStructureUsage usage() const`
    #[inline]
    pub fn usage(&self) -> AccelerationStructureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(usage)) }
    }

    /// Set the usage flags.
    ///
    /// C++ equivalent: `void setUsage(AccelerationStructureUsage)`
    #[inline]
    pub fn set_usage(&self, usage: AccelerationStructureUsage) {
        unsafe {
            msg_send_1::<(), AccelerationStructureUsage>(self.as_ptr(), sel!(setUsage:), usage);
        }
    }
}

impl Clone for AccelerationStructureDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy acceleration structure descriptor")
        }
    }
}

impl Drop for AccelerationStructureDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for AccelerationStructureDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for AccelerationStructureDescriptor {}
unsafe impl Sync for AccelerationStructureDescriptor {}

// ============================================================================
// PrimitiveAccelerationStructureDescriptor
// ============================================================================

/// Descriptor for primitive (bottom-level) acceleration structures.
///
/// C++ equivalent: `MTL::PrimitiveAccelerationStructureDescriptor`
#[repr(transparent)]
pub struct PrimitiveAccelerationStructureDescriptor(pub(crate) NonNull<c_void>);

impl PrimitiveAccelerationStructureDescriptor {
    /// Create a new primitive acceleration structure descriptor.
    ///
    /// C++ equivalent: `static PrimitiveAccelerationStructureDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLPrimitiveAccelerationStructureDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal primitive acceleration structure descriptor.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the usage flags.
    ///
    /// C++ equivalent: `AccelerationStructureUsage usage() const`
    #[inline]
    pub fn usage(&self) -> AccelerationStructureUsage {
        unsafe { msg_send_0(self.as_ptr(), sel!(usage)) }
    }

    /// Set the usage flags.
    ///
    /// C++ equivalent: `void setUsage(AccelerationStructureUsage)`
    #[inline]
    pub fn set_usage(&self, usage: AccelerationStructureUsage) {
        unsafe {
            msg_send_1::<(), AccelerationStructureUsage>(self.as_ptr(), sel!(setUsage:), usage);
        }
    }

    /// Get the motion start border mode.
    ///
    /// C++ equivalent: `MotionBorderMode motionStartBorderMode() const`
    #[inline]
    pub fn motion_start_border_mode(&self) -> MotionBorderMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionStartBorderMode)) }
    }

    /// Set the motion start border mode.
    ///
    /// C++ equivalent: `void setMotionStartBorderMode(MotionBorderMode)`
    #[inline]
    pub fn set_motion_start_border_mode(&self, mode: MotionBorderMode) {
        unsafe {
            msg_send_1::<(), MotionBorderMode>(
                self.as_ptr(),
                sel!(setMotionStartBorderMode:),
                mode,
            );
        }
    }

    /// Get the motion end border mode.
    ///
    /// C++ equivalent: `MotionBorderMode motionEndBorderMode() const`
    #[inline]
    pub fn motion_end_border_mode(&self) -> MotionBorderMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionEndBorderMode)) }
    }

    /// Set the motion end border mode.
    ///
    /// C++ equivalent: `void setMotionEndBorderMode(MotionBorderMode)`
    #[inline]
    pub fn set_motion_end_border_mode(&self, mode: MotionBorderMode) {
        unsafe {
            msg_send_1::<(), MotionBorderMode>(self.as_ptr(), sel!(setMotionEndBorderMode:), mode);
        }
    }

    /// Get the motion start time.
    ///
    /// C++ equivalent: `float motionStartTime() const`
    #[inline]
    pub fn motion_start_time(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionStartTime)) }
    }

    /// Set the motion start time.
    ///
    /// C++ equivalent: `void setMotionStartTime(float)`
    #[inline]
    pub fn set_motion_start_time(&self, time: f32) {
        unsafe {
            msg_send_1::<(), f32>(self.as_ptr(), sel!(setMotionStartTime:), time);
        }
    }

    /// Get the motion end time.
    ///
    /// C++ equivalent: `float motionEndTime() const`
    #[inline]
    pub fn motion_end_time(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionEndTime)) }
    }

    /// Set the motion end time.
    ///
    /// C++ equivalent: `void setMotionEndTime(float)`
    #[inline]
    pub fn set_motion_end_time(&self, time: f32) {
        unsafe {
            msg_send_1::<(), f32>(self.as_ptr(), sel!(setMotionEndTime:), time);
        }
    }

    /// Get the motion keyframe count.
    ///
    /// C++ equivalent: `NS::UInteger motionKeyframeCount() const`
    #[inline]
    pub fn motion_keyframe_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionKeyframeCount)) }
    }

    /// Set the motion keyframe count.
    ///
    /// C++ equivalent: `void setMotionKeyframeCount(NS::UInteger)`
    #[inline]
    pub fn set_motion_keyframe_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMotionKeyframeCount:), count);
        }
    }

    /// Get the geometry descriptors as a raw NS::Array pointer.
    ///
    /// C++ equivalent: `NS::Array* geometryDescriptors() const`
    ///
    /// # Safety
    ///
    /// The returned pointer is an NS::Array containing geometry descriptors.
    /// The caller must manage the memory appropriately.
    #[inline]
    pub fn geometry_descriptors_ptr(&self) -> *const c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(geometryDescriptors)) }
    }

    /// Set the geometry descriptors from a raw NS::Array pointer.
    ///
    /// C++ equivalent: `void setGeometryDescriptors(const NS::Array*)`
    ///
    /// # Safety
    ///
    /// The geometry_descriptors pointer must be a valid NS::Array or null.
    pub unsafe fn set_geometry_descriptors_ptr(&self, geometry_descriptors: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setGeometryDescriptors:),
                geometry_descriptors,
            );
        }
    }
}

impl Default for PrimitiveAccelerationStructureDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create primitive acceleration structure descriptor")
    }
}

impl Clone for PrimitiveAccelerationStructureDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy descriptor")
        }
    }
}

impl Drop for PrimitiveAccelerationStructureDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
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
