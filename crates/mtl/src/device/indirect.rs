//! Indirect command buffer and related Device factory methods.
//!
//! Corresponds to indirect resource creation methods in `Metal/MTLDevice.hpp`.

use std::ffi::c_void;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_1, msg_send_2, msg_send_3, sel};

use super::Device;
use crate::argument::{ArgumentEncoder, BufferBinding};
use crate::counter::{CounterSampleBuffer, CounterSampleBufferDescriptor, CounterSet};
use crate::enums::ResourceOptions;
use crate::indirect::{IndirectCommandBuffer, IndirectCommandBufferDescriptor};
use crate::log_state::{LogState, LogStateDescriptor};
use crate::rasterization_rate::{RasterizationRateMap, RasterizationRateMapDescriptor};
use crate::tensor::{Tensor, TensorDescriptor};
use crate::texture_view_pool::{ResourceViewPoolDescriptor, TextureViewPool};

impl Device {
    // =========================================================================
    // Indirect Command Buffer
    // =========================================================================

    /// Create a new indirect command buffer.
    ///
    /// C++ equivalent: `IndirectCommandBuffer* newIndirectCommandBuffer(const IndirectCommandBufferDescriptor*, NS::UInteger, ResourceOptions)`
    pub fn new_indirect_command_buffer(
        &self,
        descriptor: &IndirectCommandBufferDescriptor,
        max_count: UInteger,
        options: ResourceOptions,
    ) -> Option<IndirectCommandBuffer> {
        unsafe {
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newIndirectCommandBufferWithDescriptor:maxCommandCount:options:),
                descriptor.as_ptr(),
                max_count,
                options,
            );
            if ptr.is_null() {
                None
            } else {
                IndirectCommandBuffer::from_raw(ptr)
            }
        }
    }

    // =========================================================================
    // Rasterization Rate Map
    // =========================================================================

    /// Create a new rasterization rate map.
    ///
    /// C++ equivalent: `RasterizationRateMap* newRasterizationRateMap(const RasterizationRateMapDescriptor*)`
    pub fn new_rasterization_rate_map(
        &self,
        descriptor: &RasterizationRateMapDescriptor,
    ) -> Option<RasterizationRateMap> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newRasterizationRateMapWithDescriptor:),
                descriptor.as_ptr(),
            );
            if ptr.is_null() {
                None
            } else {
                RasterizationRateMap::from_raw(ptr)
            }
        }
    }

    // =========================================================================
    // Counter Sample Buffer
    // =========================================================================

    /// Create a new counter sample buffer.
    ///
    /// C++ equivalent: `CounterSampleBuffer* newCounterSampleBuffer(const CounterSampleBufferDescriptor*, NS::Error**)`
    pub fn new_counter_sample_buffer(
        &self,
        descriptor: &CounterSampleBufferDescriptor,
    ) -> Result<CounterSampleBuffer, mtl_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newCounterSampleBufferWithDescriptor:error:),
                descriptor.as_ptr(),
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    return Err(
                        mtl_foundation::Error::from_ptr(error).expect("error should be valid")
                    );
                }
                return Err(mtl_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error"));
            }

            Ok(CounterSampleBuffer::from_raw(ptr).expect("failed to create counter sample buffer"))
        }
    }

    /// Get the available counter sets for this device.
    ///
    /// Returns a raw pointer to an NSArray of CounterSet objects.
    ///
    /// C++ equivalent: `NS::Array* counterSets() const`
    #[inline]
    pub fn counter_sets_raw(&self) -> *mut c_void {
        unsafe { mtl_sys::msg_send_0(self.as_ptr(), sel!(counterSets)) }
    }

    /// Get the number of counter sets available.
    pub fn counter_set_count(&self) -> UInteger {
        unsafe {
            let array = self.counter_sets_raw();
            if array.is_null() {
                return 0;
            }
            mtl_sys::msg_send_0(array as *const c_void, sel!(count))
        }
    }

    /// Get a counter set at the specified index.
    pub fn counter_set_at_index(&self, index: UInteger) -> Option<CounterSet> {
        unsafe {
            let array = self.counter_sets_raw();
            if array.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_1(array as *const c_void, sel!(objectAtIndex:), index);
            if ptr.is_null() {
                return None;
            }
            mtl_sys::msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            CounterSet::from_raw(ptr)
        }
    }

    // =========================================================================
    // Argument Encoder
    // =========================================================================

    /// Create a new argument encoder from an array of arguments.
    ///
    /// C++ equivalent: `ArgumentEncoder* newArgumentEncoder(const NS::Array* arguments)`
    ///
    /// # Safety
    ///
    /// The arguments pointer must be a valid NSArray of Argument objects.
    pub unsafe fn new_argument_encoder_with_arguments(
        &self,
        arguments: *const c_void,
    ) -> Option<ArgumentEncoder> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newArgumentEncoderWithArguments:),
                arguments,
            );
            if ptr.is_null() {
                None
            } else {
                ArgumentEncoder::from_raw(ptr)
            }
        }
    }

    /// Create a new argument encoder from a buffer binding.
    ///
    /// C++ equivalent: `ArgumentEncoder* newArgumentEncoder(const BufferBinding*)`
    pub fn new_argument_encoder_with_buffer_binding(
        &self,
        buffer_binding: &BufferBinding,
    ) -> Option<ArgumentEncoder> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newArgumentEncoderWithBufferBinding:),
                buffer_binding.as_ptr(),
            );
            if ptr.is_null() {
                None
            } else {
                ArgumentEncoder::from_raw(ptr)
            }
        }
    }

    // =========================================================================
    // Log State
    // =========================================================================

    /// Create a new log state.
    ///
    /// C++ equivalent: `LogState* newLogState(const LogStateDescriptor*, NS::Error**)`
    pub fn new_log_state(
        &self,
        descriptor: &LogStateDescriptor,
    ) -> Result<LogState, mtl_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newLogStateWithDescriptor:error:),
                descriptor.as_ptr(),
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    return Err(
                        mtl_foundation::Error::from_ptr(error).expect("error should be valid")
                    );
                }
                return Err(mtl_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error"));
            }

            Ok(LogState::from_raw(ptr).expect("failed to create log state"))
        }
    }

    // =========================================================================
    // Texture View Pool
    // =========================================================================

    /// Create a new texture view pool.
    ///
    /// C++ equivalent: `TextureViewPool* newTextureViewPool(const ResourceViewPoolDescriptor*, NS::Error**)`
    pub fn new_texture_view_pool(
        &self,
        descriptor: &ResourceViewPoolDescriptor,
    ) -> Result<TextureViewPool, mtl_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newTextureViewPoolWithDescriptor:error:),
                descriptor.as_ptr(),
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    return Err(
                        mtl_foundation::Error::from_ptr(error).expect("error should be valid")
                    );
                }
                return Err(mtl_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error"));
            }

            Ok(TextureViewPool::from_raw(ptr).expect("failed to create texture view pool"))
        }
    }

    // =========================================================================
    // Tensor
    // =========================================================================

    /// Create a new tensor.
    ///
    /// C++ equivalent: `Tensor* newTensor(const TensorDescriptor*, NS::Error**)`
    pub fn new_tensor(
        &self,
        descriptor: &TensorDescriptor,
    ) -> Result<Tensor, mtl_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newTensorWithDescriptor:error:),
                descriptor.as_ptr(),
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    return Err(
                        mtl_foundation::Error::from_ptr(error).expect("error should be valid")
                    );
                }
                return Err(mtl_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error"));
            }

            Ok(Tensor::from_raw(ptr).expect("failed to create tensor"))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_device_indirect_methods_exist() {
        // This test just verifies compilation - actual tests require hardware
    }
}
