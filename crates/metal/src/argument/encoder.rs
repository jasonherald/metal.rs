//! An encoder for encoding resources into argument buffers.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, msg_send_3, sel};

use crate::Buffer;

/// An encoder for encoding resources into argument buffers.
///
/// C++ equivalent: `MTL::ArgumentEncoder`
#[repr(transparent)]
pub struct ArgumentEncoder(pub(crate) NonNull<c_void>);

impl ArgumentEncoder {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal ArgumentEncoder.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the alignment of the encoded data.
    ///
    /// C++ equivalent: `NS::UInteger alignment() const`
    #[inline]
    pub fn alignment(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(alignment)) }
    }

    /// Get a pointer to the constant data at the given index.
    ///
    /// C++ equivalent: `void* constantData(NS::UInteger index)`
    #[inline]
    pub fn constant_data(&self, index: UInteger) -> *mut c_void {
        unsafe { msg_send_1(self.as_ptr(), sel!(constantDataAtIndex:), index) }
    }

    /// Get the device that created this encoder.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> crate::Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            // Device is retained by the encoder, retain it for our reference
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            crate::Device::from_raw(ptr).expect("device should be valid")
        }
    }

    /// Get the encoded length in bytes.
    ///
    /// C++ equivalent: `NS::UInteger encodedLength() const`
    #[inline]
    pub fn encoded_length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(encodedLength)) }
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
    /// C++ equivalent: `void setLabel(const NS::String* label)`
    pub fn set_label(&self, label: &str) {
        if let Some(ns_label) = metal_foundation::String::from_str(label) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Create a new argument encoder for a nested buffer at the given index.
    ///
    /// C++ equivalent: `ArgumentEncoder* newArgumentEncoder(NS::UInteger index)`
    pub fn new_argument_encoder(&self, index: UInteger) -> Option<ArgumentEncoder> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newArgumentEncoderForBufferAtIndex:),
                index,
            );
            ArgumentEncoder::from_raw(ptr)
        }
    }

    /// Set the argument buffer to encode into.
    ///
    /// C++ equivalent: `void setArgumentBuffer(const MTL::Buffer* argumentBuffer, NS::UInteger offset)`
    pub fn set_argument_buffer(&self, buffer: &Buffer, offset: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setArgumentBuffer:offset:),
                buffer.as_ptr(),
                offset,
            );
        }
    }

    /// Set the argument buffer with array element.
    ///
    /// C++ equivalent: `void setArgumentBuffer(const MTL::Buffer*, NS::UInteger, NS::UInteger)`
    pub fn set_argument_buffer_with_array_element(
        &self,
        buffer: &Buffer,
        start_offset: UInteger,
        array_element: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(setArgumentBuffer:startOffset:arrayElement:),
                buffer.as_ptr(),
                start_offset,
                array_element,
            );
        }
    }

    /// Set a buffer at the given index.
    ///
    /// C++ equivalent: `void setBuffer(const MTL::Buffer*, NS::UInteger, NS::UInteger)`
    pub fn set_buffer(&self, buffer: &Buffer, offset: UInteger, index: UInteger) {
        unsafe {
            let _: () = msg_send_3(
                self.as_ptr(),
                sel!(setBuffer:offset:atIndex:),
                buffer.as_ptr(),
                offset,
                index,
            );
        }
    }

    /// Set a texture at the given index.
    ///
    /// C++ equivalent: `void setTexture(const MTL::Texture*, NS::UInteger)`
    pub fn set_texture(&self, texture: &crate::Texture, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setTexture:atIndex:),
                texture.as_ptr(),
                index,
            );
        }
    }

    /// Set a sampler state at the given index.
    ///
    /// C++ equivalent: `void setSamplerState(const MTL::SamplerState*, NS::UInteger)`
    pub fn set_sampler_state(&self, sampler: &crate::SamplerState, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setSamplerState:atIndex:),
                sampler.as_ptr(),
                index,
            );
        }
    }

    /// Set a render pipeline state at the given index.
    ///
    /// C++ equivalent: `void setRenderPipelineState(const MTL::RenderPipelineState*, NS::UInteger)`
    pub fn set_render_pipeline_state(
        &self,
        pipeline: &crate::RenderPipelineState,
        index: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setRenderPipelineState:atIndex:),
                pipeline.as_ptr(),
                index,
            );
        }
    }

    /// Set a compute pipeline state at the given index.
    ///
    /// C++ equivalent: `void setComputePipelineState(const MTL::ComputePipelineState*, NS::UInteger)`
    pub fn set_compute_pipeline_state(
        &self,
        pipeline: &crate::ComputePipelineState,
        index: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setComputePipelineState:atIndex:),
                pipeline.as_ptr(),
                index,
            );
        }
    }

    /// Set a depth stencil state at the given index.
    ///
    /// C++ equivalent: `void setDepthStencilState(const MTL::DepthStencilState*, NS::UInteger)`
    pub fn set_depth_stencil_state(&self, state: &crate::DepthStencilState, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setDepthStencilState:atIndex:),
                state.as_ptr(),
                index,
            );
        }
    }

    /// Set an acceleration structure at the given index.
    ///
    /// C++ equivalent: `void setAccelerationStructure(const MTL::AccelerationStructure*, NS::UInteger)`
    pub fn set_acceleration_structure(
        &self,
        acceleration_structure: &crate::AccelerationStructure,
        index: UInteger,
    ) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setAccelerationStructure:atIndex:),
                acceleration_structure.as_ptr(),
                index,
            );
        }
    }

    /// Set an indirect command buffer at the given index.
    ///
    /// C++ equivalent: `void setIndirectCommandBuffer(const MTL::IndirectCommandBuffer*, NS::UInteger)`
    pub fn set_indirect_command_buffer_ptr(&self, buffer: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setIndirectCommandBuffer:atIndex:),
                buffer,
                index,
            );
        }
    }

    /// Set a visible function table at the given index.
    ///
    /// C++ equivalent: `void setVisibleFunctionTable(const MTL::VisibleFunctionTable*, NS::UInteger)`
    pub fn set_visible_function_table_ptr(&self, table: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setVisibleFunctionTable:atIndex:),
                table,
                index,
            );
        }
    }

    /// Set an intersection function table at the given index.
    ///
    /// C++ equivalent: `void setIntersectionFunctionTable(const MTL::IntersectionFunctionTable*, NS::UInteger)`
    pub fn set_intersection_function_table_ptr(&self, table: *const c_void, index: UInteger) {
        unsafe {
            let _: () = msg_send_2(
                self.as_ptr(),
                sel!(setIntersectionFunctionTable:atIndex:),
                table,
                index,
            );
        }
    }
}

impl Clone for ArgumentEncoder {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for ArgumentEncoder {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for ArgumentEncoder {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ArgumentEncoder {}
unsafe impl Sync for ArgumentEncoder {}
