//! Device library creation methods.
//!
//! Corresponds to library creation methods in `Metal/MTLDevice.hpp`.

use std::ffi::c_void;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_0, msg_send_2, msg_send_3, sel};

use super::Device;
use crate::library::{CompileOptions, Library};

impl Device {
    // =========================================================================
    // Library Creation
    // =========================================================================

    /// Create the default library from the app's main bundle.
    ///
    /// C++ equivalent: `Library* newDefaultLibrary()`
    pub fn new_default_library(&self) -> Option<Library> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(newDefaultLibrary));
            Library::from_raw(ptr)
        }
    }

    /// Create the default library from a bundle.
    ///
    /// C++ equivalent: `Library* newDefaultLibrary(NS::Bundle*, NS::Error**)`
    ///
    /// # Safety
    ///
    /// The bundle pointer must be valid.
    pub unsafe fn new_default_library_with_bundle(
        &self,
        bundle: *const c_void,
    ) -> Result<Library, mtl_foundation::Error> {
        let mut error: *mut c_void = std::ptr::null_mut();
        unsafe {
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newDefaultLibraryWithBundle: error:),
                bundle,
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    let _: *mut c_void = msg_send_0(error, sel!(retain));
                    return Err(mtl_foundation::Error::from_ptr(error)
                        .expect("error pointer should be valid"));
                }
                return Err(mtl_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error object"));
            }

            Ok(Library::from_raw(ptr).expect("library should be valid"))
        }
    }

    /// Create a library from source code.
    ///
    /// C++ equivalent: `Library* newLibrary(const NS::String* source, const CompileOptions* options, NS::Error** error)`
    pub fn new_library_with_source(
        &self,
        source: &str,
        options: Option<&CompileOptions>,
    ) -> Result<Library, mtl_foundation::Error> {
        let ns_source = mtl_foundation::String::from_str(source).ok_or_else(|| {
            mtl_foundation::Error::error(std::ptr::null_mut(), -1, std::ptr::null_mut())
                .expect("failed to create error for invalid string")
        })?;

        let mut error: *mut c_void = std::ptr::null_mut();
        unsafe {
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newLibraryWithSource: options: error:),
                ns_source.as_ptr(),
                options.map_or(std::ptr::null(), |o| o.as_ptr()),
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    let _: *mut c_void = msg_send_0(error, sel!(retain));
                    return Err(mtl_foundation::Error::from_ptr(error)
                        .expect("error pointer should be valid"));
                }
                return Err(mtl_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error object"));
            }

            Ok(Library::from_raw(ptr).expect("library should be valid"))
        }
    }

    /// Create a library from pre-compiled binary data.
    ///
    /// C++ equivalent: `Library* newLibrary(dispatch_data_t data, NS::Error** error)`
    ///
    /// # Safety
    ///
    /// The data pointer must be valid dispatch_data_t.
    pub unsafe fn new_library_with_data(
        &self,
        data: *const c_void,
    ) -> Result<Library, mtl_foundation::Error> {
        let mut error: *mut c_void = std::ptr::null_mut();
        unsafe {
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newLibraryWithData: error:),
                data,
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    let _: *mut c_void = msg_send_0(error, sel!(retain));
                    return Err(mtl_foundation::Error::from_ptr(error)
                        .expect("error pointer should be valid"));
                }
                return Err(mtl_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error object"));
            }

            Ok(Library::from_raw(ptr).expect("library should be valid"))
        }
    }

    /// Create a library from a file URL.
    ///
    /// C++ equivalent: `Library* newLibrary(const NS::URL* url, NS::Error** error)`
    ///
    /// # Safety
    ///
    /// The URL pointer must be valid.
    pub unsafe fn new_library_with_url(
        &self,
        url: *const c_void,
    ) -> Result<Library, mtl_foundation::Error> {
        let mut error: *mut c_void = std::ptr::null_mut();
        unsafe {
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newLibraryWithURL: error:),
                url,
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    let _: *mut c_void = msg_send_0(error, sel!(retain));
                    return Err(mtl_foundation::Error::from_ptr(error)
                        .expect("error pointer should be valid"));
                }
                return Err(mtl_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error object"));
            }

            Ok(Library::from_raw(ptr).expect("library should be valid"))
        }
    }

    /// Create a library from a stitched library descriptor.
    ///
    /// C++ equivalent: `Library* newLibrary(const StitchedLibraryDescriptor*, NS::Error**)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn new_library_with_stitched_descriptor(
        &self,
        descriptor: *const c_void,
    ) -> Result<Library, mtl_foundation::Error> {
        let mut error: *mut c_void = std::ptr::null_mut();
        unsafe {
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newLibraryWithStitchedDescriptor: error:),
                descriptor,
                &mut error as *mut _,
            );

            if ptr.is_null() {
                if !error.is_null() {
                    let _: *mut c_void = msg_send_0(error, sel!(retain));
                    return Err(mtl_foundation::Error::from_ptr(error)
                        .expect("error pointer should be valid"));
                }
                return Err(mtl_foundation::Error::error(
                    std::ptr::null_mut(),
                    -1,
                    std::ptr::null_mut(),
                )
                .expect("failed to create error object"));
            }

            Ok(Library::from_raw(ptr).expect("library should be valid"))
        }
    }

    // =========================================================================
    // Async Library Creation
    // =========================================================================

    /// Create a library from source code asynchronously.
    ///
    /// C++ equivalent: `void newLibrary(const NS::String* source, const CompileOptions* options, NewLibraryCompletionHandler)`
    ///
    /// The completion handler is called with the library and any error that occurred.
    pub fn new_library_with_source_async<F>(
        &self,
        source: &str,
        options: Option<&CompileOptions>,
        completion_handler: F,
    ) where
        F: Fn(Option<Library>, Option<mtl_foundation::Error>) + Send + 'static,
    {
        let Some(ns_source) = mtl_foundation::String::from_str(source) else {
            // Call completion handler with error
            completion_handler(
                None,
                mtl_foundation::Error::error(std::ptr::null_mut(), -1, std::ptr::null_mut()),
            );
            return;
        };

        let block =
            mtl_sys::TwoArgBlock::from_fn(move |lib_ptr: *mut c_void, err_ptr: *mut c_void| {
                let library = if lib_ptr.is_null() {
                    None
                } else {
                    unsafe { Library::from_raw(lib_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { mtl_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(library, error);
            });

        unsafe {
            msg_send_3::<(), *const c_void, *const c_void, *const c_void>(
                self.as_ptr(),
                sel!(newLibraryWithSource:options:completionHandler:),
                ns_source.as_ptr(),
                options.map_or(std::ptr::null(), |o| o.as_ptr()),
                block.as_ptr(),
            );
        }

        std::mem::forget(block);
    }

    /// Create a library from a stitched library descriptor asynchronously.
    ///
    /// C++ equivalent: `void newLibrary(const StitchedLibraryDescriptor*, NewLibraryCompletionHandler)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn new_library_with_stitched_descriptor_async<F>(
        &self,
        descriptor: *const c_void,
        completion_handler: F,
    ) where
        F: Fn(Option<Library>, Option<mtl_foundation::Error>) + Send + 'static,
    {
        let block =
            mtl_sys::TwoArgBlock::from_fn(move |lib_ptr: *mut c_void, err_ptr: *mut c_void| {
                let library = if lib_ptr.is_null() {
                    None
                } else {
                    unsafe { Library::from_raw(lib_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { mtl_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(library, error);
            });

        unsafe {
            msg_send_2::<(), *const c_void, *const c_void>(
                self.as_ptr(),
                sel!(newLibraryWithStitchedDescriptor:completionHandler:),
                descriptor,
                block.as_ptr(),
            );
        }

        std::mem::forget(block);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::system_default;

    #[test]
    fn test_new_library_with_source() {
        let device = system_default().expect("no Metal device");

        let source = r#"
            #include <metal_stdlib>
            using namespace metal;

            kernel void test_kernel(device float* data [[buffer(0)]],
                                   uint id [[thread_position_in_grid]]) {
                data[id] = data[id] * 2.0;
            }
        "#;

        let result = device.new_library_with_source(source, None);
        assert!(
            result.is_ok(),
            "Failed to compile shader: {:?}",
            result.err()
        );

        let library = result.unwrap();
        let names = library.function_names();
        assert!(names.contains(&"test_kernel".to_string()));
    }

    #[test]
    fn test_new_library_with_options() {
        let device = system_default().expect("no Metal device");

        let source = r#"
            #include <metal_stdlib>
            using namespace metal;

            vertex float4 vertex_main(uint vid [[vertex_id]]) {
                return float4(0.0);
            }
        "#;

        let options = CompileOptions::new().expect("failed to create options");
        options.set_fast_math_enabled(true);

        let result = device.new_library_with_source(source, Some(&options));
        assert!(result.is_ok());
    }
}
