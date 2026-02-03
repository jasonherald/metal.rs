//! IO-related Device methods.
//!
//! Corresponds to IO-related methods in `Metal/MTLDevice.hpp`.

use std::ffi::c_void;

use metal_foundation::Referencing;
use metal_sys::{msg_send_2, msg_send_3, sel};

use super::Device;
use crate::enums::IOCompressionMethod;
use crate::io::{IOCommandQueue, IOCommandQueueDescriptor, IOFileHandle};

impl Device {
    /// Create a new IO command queue.
    ///
    /// C++ equivalent: `IOCommandQueue* newIOCommandQueue(const IOCommandQueueDescriptor*, NS::Error**)`
    pub fn new_io_command_queue(
        &self,
        descriptor: &IOCommandQueueDescriptor,
    ) -> Result<IOCommandQueue, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newIOCommandQueueWithDescriptor:error:),
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

            Ok(IOCommandQueue::from_raw(ptr).expect("failed to create IO command queue"))
        }
    }

    /// Create a new IO file handle.
    ///
    /// C++ equivalent: `IOFileHandle* newIOFileHandle(const NS::URL*, NS::Error**)`
    pub fn new_io_file_handle(
        &self,
        url: &metal_foundation::Url,
    ) -> Result<IOFileHandle, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newIOFileHandleWithURL:error:),
                url.as_ptr(),
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

            Ok(IOFileHandle::from_raw(ptr).expect("failed to create IO file handle"))
        }
    }

    /// Create a new IO file handle with compression.
    ///
    /// C++ equivalent: `IOFileHandle* newIOFileHandle(const NS::URL*, IOCompressionMethod, NS::Error**)`
    pub fn new_io_file_handle_with_compression(
        &self,
        url: &metal_foundation::Url,
        compression_method: IOCompressionMethod,
    ) -> Result<IOFileHandle, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newIOFileHandleWithURL:compressionMethod:error:),
                url.as_ptr(),
                compression_method,
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

            Ok(IOFileHandle::from_raw(ptr).expect("failed to create IO file handle"))
        }
    }

    /// Create a new IO handle (deprecated, use new_io_file_handle).
    ///
    /// C++ equivalent: `IOFileHandle* newIOHandle(const NS::URL*, NS::Error**)`
    #[deprecated(note = "Use new_io_file_handle instead")]
    pub fn new_io_handle(
        &self,
        url: &metal_foundation::Url,
    ) -> Result<IOFileHandle, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newIOHandleWithURL:error:),
                url.as_ptr(),
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

            Ok(IOFileHandle::from_raw(ptr).expect("failed to create IO handle"))
        }
    }

    /// Create a new IO handle with compression (deprecated, use new_io_file_handle_with_compression).
    ///
    /// C++ equivalent: `IOFileHandle* newIOHandle(const NS::URL*, IOCompressionMethod, NS::Error**)`
    #[deprecated(note = "Use new_io_file_handle_with_compression instead")]
    pub fn new_io_handle_with_compression(
        &self,
        url: &metal_foundation::Url,
        compression_method: IOCompressionMethod,
    ) -> Result<IOFileHandle, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newIOHandleWithURL:compressionMethod:error:),
                url.as_ptr(),
                compression_method,
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

            Ok(IOFileHandle::from_raw(ptr).expect("failed to create IO handle"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::system_default;

    #[test]
    fn test_new_io_command_queue() {
        let device = system_default().expect("no Metal device");
        let descriptor = IOCommandQueueDescriptor::new().expect("failed to create descriptor");

        // This may fail on some systems that don't support IO commands
        let _result = device.new_io_command_queue(&descriptor);
    }
}
