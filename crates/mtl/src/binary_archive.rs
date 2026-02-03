//! Metal binary archive for caching compiled pipeline state.
//!
//! Corresponds to `Metal/MTLBinaryArchive.hpp`.
//!
//! Binary archives store compiled pipeline functions for faster loading.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, msg_send_2, msg_send_3, sel};

use crate::Device;

// ============================================================================
// BinaryArchiveError enum
// ============================================================================

/// Binary archive error codes.
///
/// C++ equivalent: `MTL::BinaryArchiveError`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BinaryArchiveError(pub UInteger);

impl BinaryArchiveError {
    pub const NONE: Self = Self(0);
    pub const INVALID_FILE: Self = Self(1);
    pub const UNEXPECTED_ELEMENT: Self = Self(2);
    pub const COMPILATION_FAILURE: Self = Self(3);
    pub const INTERNAL_ERROR: Self = Self(4);
}

// ============================================================================
// BinaryArchiveDescriptor
// ============================================================================

/// Descriptor for creating a binary archive.
///
/// C++ equivalent: `MTL::BinaryArchiveDescriptor`
#[repr(transparent)]
pub struct BinaryArchiveDescriptor(pub(crate) NonNull<c_void>);

impl BinaryArchiveDescriptor {
    /// Create a new binary archive descriptor.
    ///
    /// C++ equivalent: `BinaryArchiveDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLBinaryArchiveDescriptor")?;
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
    /// The pointer must be a valid Metal BinaryArchiveDescriptor.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the URL to load the archive from.
    ///
    /// C++ equivalent: `NS::URL* url() const`
    pub fn url(&self) -> Option<mtl_foundation::Url> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(url));
            if ptr.is_null() {
                return None;
            }
            // Retain for our reference
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            mtl_foundation::Url::from_ptr(ptr)
        }
    }

    /// Set the URL to load the archive from.
    ///
    /// C++ equivalent: `void setUrl(const NS::URL* url)`
    pub fn set_url(&self, url: &mtl_foundation::Url) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setUrl:), url.as_ptr());
        }
    }
}

impl Default for BinaryArchiveDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create BinaryArchiveDescriptor")
    }
}

impl Clone for BinaryArchiveDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("copy should succeed")
        }
    }
}

impl Drop for BinaryArchiveDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for BinaryArchiveDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for BinaryArchiveDescriptor {}
unsafe impl Sync for BinaryArchiveDescriptor {}

// ============================================================================
// BinaryArchive
// ============================================================================

/// A binary archive for caching compiled pipeline state.
///
/// C++ equivalent: `MTL::BinaryArchive`
#[repr(transparent)]
pub struct BinaryArchive(pub(crate) NonNull<c_void>);

impl BinaryArchive {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal BinaryArchive.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the device that created this archive.
    ///
    /// C++ equivalent: `Device* device() const`
    pub fn device(&self) -> Device {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            // Retain for our reference
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            Device::from_raw(ptr).expect("device should be valid")
        }
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
                mtl_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
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
        if let Some(ns_label) = mtl_foundation::String::from_str(label) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setLabel:), ns_label.as_ptr());
            }
        }
    }

    /// Add compute pipeline functions to the archive.
    ///
    /// C++ equivalent: `bool addComputePipelineFunctions(const MTL::ComputePipelineDescriptor*, NS::Error**)`
    pub fn add_compute_pipeline_functions(
        &self,
        descriptor: &crate::ComputePipelineDescriptor,
    ) -> Result<(), mtl_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let result: bool = msg_send_2(
                self.as_ptr(),
                sel!(addComputePipelineFunctionsWithDescriptor:error:),
                descriptor.as_ptr(),
                &mut error as *mut _,
            );
            if !result {
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
            Ok(())
        }
    }

    /// Add render pipeline functions to the archive.
    ///
    /// C++ equivalent: `bool addRenderPipelineFunctions(const MTL::RenderPipelineDescriptor*, NS::Error**)`
    pub fn add_render_pipeline_functions(
        &self,
        descriptor: &crate::RenderPipelineDescriptor,
    ) -> Result<(), mtl_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let result: bool = msg_send_2(
                self.as_ptr(),
                sel!(addRenderPipelineFunctionsWithDescriptor:error:),
                descriptor.as_ptr(),
                &mut error as *mut _,
            );
            if !result {
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
            Ok(())
        }
    }

    /// Add a function to the archive.
    ///
    /// C++ equivalent: `bool addFunction(const MTL::FunctionDescriptor*, const MTL::Library*, NS::Error**)`
    pub fn add_function(
        &self,
        descriptor: *const c_void, // FunctionDescriptor not yet implemented
        library: &crate::Library,
    ) -> Result<(), mtl_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let result: bool = msg_send_3(
                self.as_ptr(),
                sel!(addFunctionWithDescriptor:library:error:),
                descriptor,
                library.as_ptr(),
                &mut error as *mut _,
            );
            if !result {
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
            Ok(())
        }
    }

    /// Add a stitched library to the archive.
    ///
    /// C++ equivalent: `bool addLibrary(const MTL::StitchedLibraryDescriptor*, NS::Error**)`
    pub fn add_library_ptr(
        &self,
        descriptor: *const c_void,
    ) -> Result<(), mtl_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let result: bool = msg_send_2(
                self.as_ptr(),
                sel!(addLibraryWithDescriptor:error:),
                descriptor,
                &mut error as *mut _,
            );
            if !result {
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
            Ok(())
        }
    }

    /// Add mesh render pipeline functions to the archive.
    ///
    /// C++ equivalent: `bool addMeshRenderPipelineFunctions(const MTL::MeshRenderPipelineDescriptor*, NS::Error**)`
    pub fn add_mesh_render_pipeline_functions_ptr(
        &self,
        descriptor: *const c_void,
    ) -> Result<(), mtl_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let result: bool = msg_send_2(
                self.as_ptr(),
                sel!(addMeshRenderPipelineFunctionsWithDescriptor:error:),
                descriptor,
                &mut error as *mut _,
            );
            if !result {
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
            Ok(())
        }
    }

    /// Add tile render pipeline functions to the archive.
    ///
    /// C++ equivalent: `bool addTileRenderPipelineFunctions(const MTL::TileRenderPipelineDescriptor*, NS::Error**)`
    pub fn add_tile_render_pipeline_functions_ptr(
        &self,
        descriptor: *const c_void,
    ) -> Result<(), mtl_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let result: bool = msg_send_2(
                self.as_ptr(),
                sel!(addTileRenderPipelineFunctionsWithDescriptor:error:),
                descriptor,
                &mut error as *mut _,
            );
            if !result {
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
            Ok(())
        }
    }

    /// Serialize the archive to a URL.
    ///
    /// C++ equivalent: `bool serializeToURL(const NS::URL*, NS::Error**)`
    pub fn serialize_to_url(
        &self,
        url: &mtl_foundation::Url,
    ) -> Result<(), mtl_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let result: bool = msg_send_2(
                self.as_ptr(),
                sel!(serializeToURL:error:),
                url.as_ptr(),
                &mut error as *mut _,
            );
            if !result {
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
            Ok(())
        }
    }
}

impl Clone for BinaryArchive {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for BinaryArchive {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for BinaryArchive {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for BinaryArchive {}
unsafe impl Sync for BinaryArchive {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_archive_descriptor_creation() {
        let descriptor = BinaryArchiveDescriptor::new();
        assert!(descriptor.is_some());
    }

    #[test]
    fn test_binary_archive_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<BinaryArchiveDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_binary_archive_size() {
        assert_eq!(
            std::mem::size_of::<BinaryArchive>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_binary_archive_error_values() {
        assert_eq!(BinaryArchiveError::NONE.0, 0);
        assert_eq!(BinaryArchiveError::INVALID_FILE.0, 1);
        assert_eq!(BinaryArchiveError::UNEXPECTED_ELEMENT.0, 2);
        assert_eq!(BinaryArchiveError::COMPILATION_FAILURE.0, 3);
        assert_eq!(BinaryArchiveError::INTERNAL_ERROR.0, 4);
    }
}
