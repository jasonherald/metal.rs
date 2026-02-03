//! Descriptor for creating an instance acceleration structure.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::{AccelerationStructureInstanceDescriptorType, MatrixLayout, TransformType};

use super::BufferRange;

/// Descriptor for creating an instance acceleration structure.
///
/// C++ equivalent: `MTL4::InstanceAccelerationStructureDescriptor`
#[repr(transparent)]
pub struct InstanceAccelerationStructureDescriptor(NonNull<c_void>);

impl InstanceAccelerationStructureDescriptor {
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

    /// Create a new instance acceleration structure descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4InstanceAccelerationStructureDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the instance count.
    pub fn instance_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(instanceCount)) }
    }

    /// Set the instance count.
    pub fn set_instance_count(&self, count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setInstanceCount:), count);
        }
    }

    /// Get the instance descriptor buffer.
    pub fn instance_descriptor_buffer(&self) -> BufferRange {
        unsafe { msg_send_0(self.as_ptr(), sel!(instanceDescriptorBuffer)) }
    }

    /// Set the instance descriptor buffer.
    pub fn set_instance_descriptor_buffer(&self, buffer: BufferRange) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setInstanceDescriptorBuffer:), buffer);
        }
    }

    /// Get the instance descriptor stride.
    pub fn instance_descriptor_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(instanceDescriptorStride)) }
    }

    /// Set the instance descriptor stride.
    pub fn set_instance_descriptor_stride(&self, stride: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setInstanceDescriptorStride:), stride);
        }
    }

    /// Get the instance descriptor type.
    pub fn instance_descriptor_type(&self) -> AccelerationStructureInstanceDescriptorType {
        unsafe { msg_send_0(self.as_ptr(), sel!(instanceDescriptorType)) }
    }

    /// Set the instance descriptor type.
    pub fn set_instance_descriptor_type(
        &self,
        descriptor_type: AccelerationStructureInstanceDescriptorType,
    ) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setInstanceDescriptorType:),
                descriptor_type,
            );
        }
    }

    /// Get the instance transformation matrix layout.
    pub fn instance_transformation_matrix_layout(&self) -> MatrixLayout {
        unsafe { msg_send_0(self.as_ptr(), sel!(instanceTransformationMatrixLayout)) }
    }

    /// Set the instance transformation matrix layout.
    pub fn set_instance_transformation_matrix_layout(&self, layout: MatrixLayout) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setInstanceTransformationMatrixLayout:),
                layout,
            );
        }
    }

    /// Get the motion transform buffer.
    pub fn motion_transform_buffer(&self) -> BufferRange {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTransformBuffer)) }
    }

    /// Set the motion transform buffer.
    pub fn set_motion_transform_buffer(&self, buffer: BufferRange) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMotionTransformBuffer:), buffer);
        }
    }

    /// Get the motion transform count.
    pub fn motion_transform_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTransformCount)) }
    }

    /// Set the motion transform count.
    pub fn set_motion_transform_count(&self, count: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMotionTransformCount:), count);
        }
    }

    /// Get the motion transform stride.
    pub fn motion_transform_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTransformStride)) }
    }

    /// Set the motion transform stride.
    pub fn set_motion_transform_stride(&self, stride: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMotionTransformStride:), stride);
        }
    }

    /// Get the motion transform type.
    pub fn motion_transform_type(&self) -> TransformType {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTransformType)) }
    }

    /// Set the motion transform type.
    pub fn set_motion_transform_type(&self, transform_type: TransformType) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setMotionTransformType:), transform_type);
        }
    }
}

impl Default for InstanceAccelerationStructureDescriptor {
    fn default() -> Self {
        Self::new().expect("Failed to create MTL4InstanceAccelerationStructureDescriptor")
    }
}

impl Clone for InstanceAccelerationStructureDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for InstanceAccelerationStructureDescriptor {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for InstanceAccelerationStructureDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for InstanceAccelerationStructureDescriptor {}
unsafe impl Sync for InstanceAccelerationStructureDescriptor {}

impl std::fmt::Debug for InstanceAccelerationStructureDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstanceAccelerationStructureDescriptor")
            .field("instance_count", &self.instance_count())
            .finish()
    }
}
