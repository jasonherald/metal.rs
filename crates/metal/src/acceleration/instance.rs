//! Instance acceleration structure descriptors.
//!
//! Contains `InstanceAccelerationStructureDescriptor` and 
//! `IndirectInstanceAccelerationStructureDescriptor`.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{AccelerationStructureInstanceDescriptorType, AccelerationStructureUsage, MatrixLayout, TransformType};
use crate::Buffer;

pub struct InstanceAccelerationStructureDescriptor(pub(crate) NonNull<c_void>);

impl InstanceAccelerationStructureDescriptor {
    /// Create a new instance acceleration structure descriptor.
    ///
    /// C++ equivalent: `static InstanceAccelerationStructureDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTLInstanceAccelerationStructureDescriptor")?;
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
    /// The pointer must be a valid Metal instance acceleration structure descriptor.
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

    /// Get the instance count.
    ///
    /// C++ equivalent: `NS::UInteger instanceCount() const`
    #[inline]
    pub fn instance_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(instanceCount)) }
    }

    /// Set the instance count.
    ///
    /// C++ equivalent: `void setInstanceCount(NS::UInteger)`
    #[inline]
    pub fn set_instance_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setInstanceCount:), count);
        }
    }

    /// Get the instance descriptor buffer.
    ///
    /// C++ equivalent: `Buffer* instanceDescriptorBuffer() const`
    pub fn instance_descriptor_buffer(&self) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(instanceDescriptorBuffer));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Buffer::from_raw(ptr)
        }
    }

    /// Set the instance descriptor buffer.
    ///
    /// C++ equivalent: `void setInstanceDescriptorBuffer(Buffer*)`
    pub fn set_instance_descriptor_buffer(&self, buffer: Option<&Buffer>) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setInstanceDescriptorBuffer:),
                buffer.map_or(std::ptr::null(), |b| b.as_ptr()),
            );
        }
    }

    /// Get the instance descriptor buffer offset.
    ///
    /// C++ equivalent: `NS::UInteger instanceDescriptorBufferOffset() const`
    #[inline]
    pub fn instance_descriptor_buffer_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(instanceDescriptorBufferOffset)) }
    }

    /// Set the instance descriptor buffer offset.
    ///
    /// C++ equivalent: `void setInstanceDescriptorBufferOffset(NS::UInteger)`
    #[inline]
    pub fn set_instance_descriptor_buffer_offset(&self, offset: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(
                self.as_ptr(),
                sel!(setInstanceDescriptorBufferOffset:),
                offset,
            );
        }
    }

    /// Get the instance descriptor stride.
    ///
    /// C++ equivalent: `NS::UInteger instanceDescriptorStride() const`
    #[inline]
    pub fn instance_descriptor_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(instanceDescriptorStride)) }
    }

    /// Set the instance descriptor stride.
    ///
    /// C++ equivalent: `void setInstanceDescriptorStride(NS::UInteger)`
    #[inline]
    pub fn set_instance_descriptor_stride(&self, stride: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setInstanceDescriptorStride:), stride);
        }
    }

    /// Get the instance descriptor type.
    ///
    /// C++ equivalent: `AccelerationStructureInstanceDescriptorType instanceDescriptorType() const`
    #[inline]
    pub fn instance_descriptor_type(&self) -> AccelerationStructureInstanceDescriptorType {
        unsafe { msg_send_0(self.as_ptr(), sel!(instanceDescriptorType)) }
    }

    /// Set the instance descriptor type.
    ///
    /// C++ equivalent: `void setInstanceDescriptorType(AccelerationStructureInstanceDescriptorType)`
    #[inline]
    pub fn set_instance_descriptor_type(
        &self,
        descriptor_type: AccelerationStructureInstanceDescriptorType,
    ) {
        unsafe {
            msg_send_1::<(), AccelerationStructureInstanceDescriptorType>(
                self.as_ptr(),
                sel!(setInstanceDescriptorType:),
                descriptor_type,
            );
        }
    }

    /// Get the instance transformation matrix layout.
    ///
    /// C++ equivalent: `MatrixLayout instanceTransformationMatrixLayout() const`
    #[inline]
    pub fn instance_transformation_matrix_layout(&self) -> MatrixLayout {
        unsafe { msg_send_0(self.as_ptr(), sel!(instanceTransformationMatrixLayout)) }
    }

    /// Set the instance transformation matrix layout.
    ///
    /// C++ equivalent: `void setInstanceTransformationMatrixLayout(MatrixLayout)`
    #[inline]
    pub fn set_instance_transformation_matrix_layout(&self, layout: MatrixLayout) {
        unsafe {
            msg_send_1::<(), MatrixLayout>(
                self.as_ptr(),
                sel!(setInstanceTransformationMatrixLayout:),
                layout,
            );
        }
    }

    /// Get the motion transform buffer.
    ///
    /// C++ equivalent: `Buffer* motionTransformBuffer() const`
    pub fn motion_transform_buffer(&self) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(motionTransformBuffer));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Buffer::from_raw(ptr)
        }
    }

    /// Set the motion transform buffer.
    ///
    /// C++ equivalent: `void setMotionTransformBuffer(Buffer*)`
    pub fn set_motion_transform_buffer(&self, buffer: Option<&Buffer>) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setMotionTransformBuffer:),
                buffer.map_or(std::ptr::null(), |b| b.as_ptr()),
            );
        }
    }

    /// Get the motion transform buffer offset.
    ///
    /// C++ equivalent: `NS::UInteger motionTransformBufferOffset() const`
    #[inline]
    pub fn motion_transform_buffer_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTransformBufferOffset)) }
    }

    /// Set the motion transform buffer offset.
    ///
    /// C++ equivalent: `void setMotionTransformBufferOffset(NS::UInteger)`
    #[inline]
    pub fn set_motion_transform_buffer_offset(&self, offset: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(
                self.as_ptr(),
                sel!(setMotionTransformBufferOffset:),
                offset,
            );
        }
    }

    /// Get the motion transform count.
    ///
    /// C++ equivalent: `NS::UInteger motionTransformCount() const`
    #[inline]
    pub fn motion_transform_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTransformCount)) }
    }

    /// Set the motion transform count.
    ///
    /// C++ equivalent: `void setMotionTransformCount(NS::UInteger)`
    #[inline]
    pub fn set_motion_transform_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMotionTransformCount:), count);
        }
    }

    /// Get the motion transform stride.
    ///
    /// C++ equivalent: `NS::UInteger motionTransformStride() const`
    #[inline]
    pub fn motion_transform_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTransformStride)) }
    }

    /// Set the motion transform stride.
    ///
    /// C++ equivalent: `void setMotionTransformStride(NS::UInteger)`
    #[inline]
    pub fn set_motion_transform_stride(&self, stride: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMotionTransformStride:), stride);
        }
    }

    /// Get the motion transform type.
    ///
    /// C++ equivalent: `TransformType motionTransformType() const`
    #[inline]
    pub fn motion_transform_type(&self) -> TransformType {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTransformType)) }
    }

    /// Set the motion transform type.
    ///
    /// C++ equivalent: `void setMotionTransformType(TransformType)`
    #[inline]
    pub fn set_motion_transform_type(&self, transform_type: TransformType) {
        unsafe {
            msg_send_1::<(), TransformType>(
                self.as_ptr(),
                sel!(setMotionTransformType:),
                transform_type,
            );
        }
    }

    /// Get the instanced acceleration structures as a raw NS::Array pointer.
    ///
    /// C++ equivalent: `NS::Array* instancedAccelerationStructures() const`
    ///
    /// # Safety
    ///
    /// The returned pointer is an NS::Array containing AccelerationStructure objects.
    /// The caller must manage the memory appropriately.
    #[inline]
    pub fn instanced_acceleration_structures_ptr(&self) -> *const c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(instancedAccelerationStructures)) }
    }

    /// Set the instanced acceleration structures from a raw NS::Array pointer.
    ///
    /// C++ equivalent: `void setInstancedAccelerationStructures(const NS::Array*)`
    ///
    /// # Safety
    ///
    /// The instanced_acceleration_structures pointer must be a valid NS::Array or null.
    pub unsafe fn set_instanced_acceleration_structures_ptr(
        &self,
        instanced_acceleration_structures: *const c_void,
    ) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setInstancedAccelerationStructures:),
                instanced_acceleration_structures,
            );
        }
    }
}

impl Default for InstanceAccelerationStructureDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create instance acceleration structure descriptor")
    }
}

impl Clone for InstanceAccelerationStructureDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy descriptor")
        }
    }
}

impl Drop for InstanceAccelerationStructureDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
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

// ============================================================================
// IndirectInstanceAccelerationStructureDescriptor
// ============================================================================

/// Descriptor for indirect instance (top-level) acceleration structures.
///
/// C++ equivalent: `MTL::IndirectInstanceAccelerationStructureDescriptor`
#[repr(transparent)]
pub struct IndirectInstanceAccelerationStructureDescriptor(pub(crate) NonNull<c_void>);

impl IndirectInstanceAccelerationStructureDescriptor {
    /// Create a new indirect instance acceleration structure descriptor.
    ///
    /// C++ equivalent: `static IndirectInstanceAccelerationStructureDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class =
                metal_sys::Class::get("MTLIndirectInstanceAccelerationStructureDescriptor")?;
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
    /// The pointer must be a valid Metal indirect instance acceleration structure descriptor.
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

    /// Get the maximum instance count.
    ///
    /// C++ equivalent: `NS::UInteger maxInstanceCount() const`
    #[inline]
    pub fn max_instance_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxInstanceCount)) }
    }

    /// Set the maximum instance count.
    ///
    /// C++ equivalent: `void setMaxInstanceCount(NS::UInteger)`
    #[inline]
    pub fn set_max_instance_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxInstanceCount:), count);
        }
    }

    /// Get the instance count buffer.
    ///
    /// C++ equivalent: `Buffer* instanceCountBuffer() const`
    pub fn instance_count_buffer(&self) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(instanceCountBuffer));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Buffer::from_raw(ptr)
        }
    }

    /// Set the instance count buffer.
    ///
    /// C++ equivalent: `void setInstanceCountBuffer(Buffer*)`
    pub fn set_instance_count_buffer(&self, buffer: Option<&Buffer>) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setInstanceCountBuffer:),
                buffer.map_or(std::ptr::null(), |b| b.as_ptr()),
            );
        }
    }

    /// Get the instance count buffer offset.
    ///
    /// C++ equivalent: `NS::UInteger instanceCountBufferOffset() const`
    #[inline]
    pub fn instance_count_buffer_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(instanceCountBufferOffset)) }
    }

    /// Set the instance count buffer offset.
    ///
    /// C++ equivalent: `void setInstanceCountBufferOffset(NS::UInteger)`
    #[inline]
    pub fn set_instance_count_buffer_offset(&self, offset: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setInstanceCountBufferOffset:), offset);
        }
    }

    /// Get the instance descriptor buffer.
    ///
    /// C++ equivalent: `Buffer* instanceDescriptorBuffer() const`
    pub fn instance_descriptor_buffer(&self) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(instanceDescriptorBuffer));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Buffer::from_raw(ptr)
        }
    }

    /// Set the instance descriptor buffer.
    ///
    /// C++ equivalent: `void setInstanceDescriptorBuffer(Buffer*)`
    pub fn set_instance_descriptor_buffer(&self, buffer: Option<&Buffer>) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setInstanceDescriptorBuffer:),
                buffer.map_or(std::ptr::null(), |b| b.as_ptr()),
            );
        }
    }

    /// Get the instance descriptor buffer offset.
    ///
    /// C++ equivalent: `NS::UInteger instanceDescriptorBufferOffset() const`
    #[inline]
    pub fn instance_descriptor_buffer_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(instanceDescriptorBufferOffset)) }
    }

    /// Set the instance descriptor buffer offset.
    ///
    /// C++ equivalent: `void setInstanceDescriptorBufferOffset(NS::UInteger)`
    #[inline]
    pub fn set_instance_descriptor_buffer_offset(&self, offset: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(
                self.as_ptr(),
                sel!(setInstanceDescriptorBufferOffset:),
                offset,
            );
        }
    }

    /// Get the instance descriptor stride.
    ///
    /// C++ equivalent: `NS::UInteger instanceDescriptorStride() const`
    #[inline]
    pub fn instance_descriptor_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(instanceDescriptorStride)) }
    }

    /// Set the instance descriptor stride.
    ///
    /// C++ equivalent: `void setInstanceDescriptorStride(NS::UInteger)`
    #[inline]
    pub fn set_instance_descriptor_stride(&self, stride: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setInstanceDescriptorStride:), stride);
        }
    }

    /// Get the instance descriptor type.
    ///
    /// C++ equivalent: `AccelerationStructureInstanceDescriptorType instanceDescriptorType() const`
    #[inline]
    pub fn instance_descriptor_type(&self) -> AccelerationStructureInstanceDescriptorType {
        unsafe { msg_send_0(self.as_ptr(), sel!(instanceDescriptorType)) }
    }

    /// Set the instance descriptor type.
    ///
    /// C++ equivalent: `void setInstanceDescriptorType(AccelerationStructureInstanceDescriptorType)`
    #[inline]
    pub fn set_instance_descriptor_type(
        &self,
        descriptor_type: AccelerationStructureInstanceDescriptorType,
    ) {
        unsafe {
            msg_send_1::<(), AccelerationStructureInstanceDescriptorType>(
                self.as_ptr(),
                sel!(setInstanceDescriptorType:),
                descriptor_type,
            );
        }
    }

    /// Get the instance transformation matrix layout.
    ///
    /// C++ equivalent: `MatrixLayout instanceTransformationMatrixLayout() const`
    #[inline]
    pub fn instance_transformation_matrix_layout(&self) -> MatrixLayout {
        unsafe { msg_send_0(self.as_ptr(), sel!(instanceTransformationMatrixLayout)) }
    }

    /// Set the instance transformation matrix layout.
    ///
    /// C++ equivalent: `void setInstanceTransformationMatrixLayout(MatrixLayout)`
    #[inline]
    pub fn set_instance_transformation_matrix_layout(&self, layout: MatrixLayout) {
        unsafe {
            msg_send_1::<(), MatrixLayout>(
                self.as_ptr(),
                sel!(setInstanceTransformationMatrixLayout:),
                layout,
            );
        }
    }

    /// Get the maximum motion transform count.
    ///
    /// C++ equivalent: `NS::UInteger maxMotionTransformCount() const`
    #[inline]
    pub fn max_motion_transform_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxMotionTransformCount)) }
    }

    /// Set the maximum motion transform count.
    ///
    /// C++ equivalent: `void setMaxMotionTransformCount(NS::UInteger)`
    #[inline]
    pub fn set_max_motion_transform_count(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMaxMotionTransformCount:), count);
        }
    }

    /// Get the motion transform buffer.
    ///
    /// C++ equivalent: `Buffer* motionTransformBuffer() const`
    pub fn motion_transform_buffer(&self) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(motionTransformBuffer));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Buffer::from_raw(ptr)
        }
    }

    /// Set the motion transform buffer.
    ///
    /// C++ equivalent: `void setMotionTransformBuffer(Buffer*)`
    pub fn set_motion_transform_buffer(&self, buffer: Option<&Buffer>) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setMotionTransformBuffer:),
                buffer.map_or(std::ptr::null(), |b| b.as_ptr()),
            );
        }
    }

    /// Get the motion transform buffer offset.
    ///
    /// C++ equivalent: `NS::UInteger motionTransformBufferOffset() const`
    #[inline]
    pub fn motion_transform_buffer_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTransformBufferOffset)) }
    }

    /// Set the motion transform buffer offset.
    ///
    /// C++ equivalent: `void setMotionTransformBufferOffset(NS::UInteger)`
    #[inline]
    pub fn set_motion_transform_buffer_offset(&self, offset: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(
                self.as_ptr(),
                sel!(setMotionTransformBufferOffset:),
                offset,
            );
        }
    }

    /// Get the motion transform count buffer.
    ///
    /// C++ equivalent: `Buffer* motionTransformCountBuffer() const`
    pub fn motion_transform_count_buffer(&self) -> Option<Buffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(motionTransformCountBuffer));
            if ptr.is_null() {
                return None;
            }
            let _: *mut c_void = msg_send_0(ptr, sel!(retain));
            Buffer::from_raw(ptr)
        }
    }

    /// Set the motion transform count buffer.
    ///
    /// C++ equivalent: `void setMotionTransformCountBuffer(Buffer*)`
    pub fn set_motion_transform_count_buffer(&self, buffer: Option<&Buffer>) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setMotionTransformCountBuffer:),
                buffer.map_or(std::ptr::null(), |b| b.as_ptr()),
            );
        }
    }

    /// Get the motion transform count buffer offset.
    ///
    /// C++ equivalent: `NS::UInteger motionTransformCountBufferOffset() const`
    #[inline]
    pub fn motion_transform_count_buffer_offset(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTransformCountBufferOffset)) }
    }

    /// Set the motion transform count buffer offset.
    ///
    /// C++ equivalent: `void setMotionTransformCountBufferOffset(NS::UInteger)`
    #[inline]
    pub fn set_motion_transform_count_buffer_offset(&self, offset: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(
                self.as_ptr(),
                sel!(setMotionTransformCountBufferOffset:),
                offset,
            );
        }
    }

    /// Get the motion transform stride.
    ///
    /// C++ equivalent: `NS::UInteger motionTransformStride() const`
    #[inline]
    pub fn motion_transform_stride(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTransformStride)) }
    }

    /// Set the motion transform stride.
    ///
    /// C++ equivalent: `void setMotionTransformStride(NS::UInteger)`
    #[inline]
    pub fn set_motion_transform_stride(&self, stride: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(self.as_ptr(), sel!(setMotionTransformStride:), stride);
        }
    }

    /// Get the motion transform type.
    ///
    /// C++ equivalent: `TransformType motionTransformType() const`
    #[inline]
    pub fn motion_transform_type(&self) -> TransformType {
        unsafe { msg_send_0(self.as_ptr(), sel!(motionTransformType)) }
    }

    /// Set the motion transform type.
    ///
    /// C++ equivalent: `void setMotionTransformType(TransformType)`
    #[inline]
    pub fn set_motion_transform_type(&self, transform_type: TransformType) {
        unsafe {
            msg_send_1::<(), TransformType>(
                self.as_ptr(),
                sel!(setMotionTransformType:),
                transform_type,
            );
        }
    }
}

impl Default for IndirectInstanceAccelerationStructureDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create indirect instance acceleration structure descriptor")
    }
}

impl Clone for IndirectInstanceAccelerationStructureDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy descriptor")
        }
    }
}

impl Drop for IndirectInstanceAccelerationStructureDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for IndirectInstanceAccelerationStructureDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for IndirectInstanceAccelerationStructureDescriptor {}
unsafe impl Sync for IndirectInstanceAccelerationStructureDescriptor {}

