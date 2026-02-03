//! MTL4-related Device methods.
//!
//! Corresponds to Metal 4 factory methods in `Metal/MTLDevice.hpp`.

use std::ffi::c_void;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, msg_send_1, msg_send_2, sel};

use super::Device;
use crate::function_table::FunctionHandle;
use crate::mtl4::{
    Archive, ArgumentTable, ArgumentTableDescriptor, CommandAllocator,
    CommandAllocatorDescriptor, CommandQueue, CommandQueueDescriptor, Compiler,
    CompilerDescriptor, CounterHeap, CounterHeapDescriptor, CounterHeapType,
    PipelineDataSetSerializer, PipelineDataSetSerializerDescriptor,
};

impl Device {
    // =========================================================================
    // Command Allocator
    // =========================================================================

    /// Create a new MTL4 command allocator with default settings.
    ///
    /// C++ equivalent: `MTL4::CommandAllocator* newCommandAllocator()`
    pub fn new_command_allocator(&self) -> Option<CommandAllocator> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(newCommandAllocator));
            if ptr.is_null() {
                None
            } else {
                CommandAllocator::from_raw(ptr)
            }
        }
    }

    /// Create a new MTL4 command allocator with a descriptor.
    ///
    /// C++ equivalent: `MTL4::CommandAllocator* newCommandAllocator(const MTL4::CommandAllocatorDescriptor*, NS::Error**)`
    pub fn new_command_allocator_with_descriptor(
        &self,
        descriptor: &CommandAllocatorDescriptor,
    ) -> Result<CommandAllocator, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newCommandAllocatorWithDescriptor:error:),
                descriptor.as_ptr(),
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    return Err(
                        metal_foundation::Error::from_ptr(error).expect("error should be valid")
                    );
                }
                return Err(metal_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error"));
            }

            Ok(CommandAllocator::from_raw(ptr).expect("failed to create command allocator"))
        }
    }

    // =========================================================================
    // Command Queue
    // =========================================================================

    /// Create a new MTL4 command queue with default settings.
    ///
    /// C++ equivalent: `MTL4::CommandQueue* newMTL4CommandQueue()`
    pub fn new_mtl4_command_queue(&self) -> Option<CommandQueue> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(newMTL4CommandQueue));
            if ptr.is_null() {
                None
            } else {
                CommandQueue::from_raw(ptr)
            }
        }
    }

    /// Create a new MTL4 command queue with a descriptor.
    ///
    /// C++ equivalent: `MTL4::CommandQueue* newMTL4CommandQueue(const MTL4::CommandQueueDescriptor*, NS::Error**)`
    pub fn new_mtl4_command_queue_with_descriptor(
        &self,
        descriptor: &CommandQueueDescriptor,
    ) -> Result<CommandQueue, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newMTL4CommandQueueWithDescriptor:error:),
                descriptor.as_ptr(),
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    return Err(
                        metal_foundation::Error::from_ptr(error).expect("error should be valid")
                    );
                }
                return Err(metal_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error"));
            }

            Ok(CommandQueue::from_raw(ptr).expect("failed to create command queue"))
        }
    }

    // =========================================================================
    // Argument Table
    // =========================================================================

    /// Create a new MTL4 argument table.
    ///
    /// C++ equivalent: `MTL4::ArgumentTable* newArgumentTable(const MTL4::ArgumentTableDescriptor*, NS::Error**)`
    pub fn new_argument_table(
        &self,
        descriptor: &ArgumentTableDescriptor,
    ) -> Result<ArgumentTable, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newArgumentTableWithDescriptor:error:),
                descriptor.as_ptr(),
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    return Err(
                        metal_foundation::Error::from_ptr(error).expect("error should be valid")
                    );
                }
                return Err(metal_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error"));
            }

            Ok(ArgumentTable::from_raw(ptr).expect("failed to create argument table"))
        }
    }

    // =========================================================================
    // Compiler
    // =========================================================================

    /// Create a new MTL4 compiler.
    ///
    /// C++ equivalent: `MTL4::Compiler* newCompiler(const MTL4::CompilerDescriptor*, NS::Error**)`
    pub fn new_compiler(
        &self,
        descriptor: &CompilerDescriptor,
    ) -> Result<Compiler, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newCompilerWithDescriptor:error:),
                descriptor.as_ptr(),
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    return Err(
                        metal_foundation::Error::from_ptr(error).expect("error should be valid")
                    );
                }
                return Err(metal_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error"));
            }

            Ok(Compiler::from_raw(ptr).expect("failed to create compiler"))
        }
    }

    // =========================================================================
    // Counter Heap
    // =========================================================================

    /// Create a new MTL4 counter heap.
    ///
    /// C++ equivalent: `MTL4::CounterHeap* newCounterHeap(const MTL4::CounterHeapDescriptor*, NS::Error**)`
    pub fn new_counter_heap(
        &self,
        descriptor: &CounterHeapDescriptor,
    ) -> Result<CounterHeap, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newCounterHeapWithDescriptor:error:),
                descriptor.as_ptr(),
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    return Err(
                        metal_foundation::Error::from_ptr(error).expect("error should be valid")
                    );
                }
                return Err(metal_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error"));
            }

            Ok(CounterHeap::from_raw(ptr).expect("failed to create counter heap"))
        }
    }

    /// Get the size of a counter heap entry.
    ///
    /// C++ equivalent: `NS::UInteger sizeOfCounterHeapEntry(MTL4::CounterHeapType type)`
    pub fn size_of_counter_heap_entry(&self, heap_type: CounterHeapType) -> metal_foundation::UInteger {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(sizeOfCounterHeapEntry:),
                heap_type,
            )
        }
    }

    // =========================================================================
    // Pipeline Data Set Serializer
    // =========================================================================

    /// Create a new MTL4 pipeline data set serializer.
    ///
    /// C++ equivalent: `MTL4::PipelineDataSetSerializer* newPipelineDataSetSerializer(const MTL4::PipelineDataSetSerializerDescriptor*)`
    pub fn new_pipeline_data_set_serializer(
        &self,
        descriptor: &PipelineDataSetSerializerDescriptor,
    ) -> Option<PipelineDataSetSerializer> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(newPipelineDataSetSerializerWithDescriptor:),
                descriptor.as_ptr(),
            );
            if ptr.is_null() {
                None
            } else {
                PipelineDataSetSerializer::from_raw(ptr)
            }
        }
    }

    // =========================================================================
    // Archive
    // =========================================================================

    /// Create a new MTL4 archive from a URL.
    ///
    /// C++ equivalent: `MTL4::Archive* newArchive(const NS::URL*, NS::Error**)`
    ///
    /// # Safety
    ///
    /// The url pointer must be a valid NS::URL object.
    pub unsafe fn new_archive_with_url(
        &self,
        url: *const c_void,
    ) -> Result<Archive, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newArchiveWithURL:error:),
                url,
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    return Err(
                        metal_foundation::Error::from_ptr(error).expect("error should be valid")
                    );
                }
                return Err(metal_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error"));
            }

            Ok(Archive::from_raw(ptr).expect("failed to create archive"))
        }
    }

    // =========================================================================
    // Function Handles
    // =========================================================================

    /// Get a function handle from a compiled function.
    ///
    /// C++ equivalent: `FunctionHandle* functionHandle(const MTL::Function*)`
    pub fn function_handle(&self, function: &crate::Function) -> Option<FunctionHandle> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(functionHandleWithFunction:),
                function.as_ptr(),
            );
            if ptr.is_null() {
                None
            } else {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
                FunctionHandle::from_raw(ptr)
            }
        }
    }

    /// Get a function handle from a binary function.
    ///
    /// C++ equivalent: `FunctionHandle* functionHandle(const MTL4::BinaryFunction*)`
    pub fn function_handle_with_binary_function(
        &self,
        function: &crate::mtl4::BinaryFunction,
    ) -> Option<FunctionHandle> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(functionHandleWithFunction:),
                function.as_ptr(),
            );
            if ptr.is_null() {
                None
            } else {
                let _: *mut c_void = msg_send_0(ptr, sel!(retain));
                FunctionHandle::from_raw(ptr)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mtl4_device_methods_exist() {
        // This test just verifies compilation - actual MTL4 tests require hardware support
    }
}
