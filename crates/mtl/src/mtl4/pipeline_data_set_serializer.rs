//! MTL4 PipelineDataSetSerializer implementation.
//!
//! Corresponds to `Metal/MTL4PipelineDataSetSerializer.hpp`.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_0, msg_send_1, msg_send_2, sel};

use super::enums::PipelineDataSetSerializerConfiguration;

// ============================================================
// PipelineDataSetSerializerDescriptor
// ============================================================

/// Descriptor for creating a pipeline data set serializer.
///
/// C++ equivalent: `MTL4::PipelineDataSetSerializerDescriptor`
#[repr(transparent)]
pub struct PipelineDataSetSerializerDescriptor(NonNull<c_void>);

impl PipelineDataSetSerializerDescriptor {
    /// Create a PipelineDataSetSerializerDescriptor from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create a new pipeline data set serializer descriptor.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTL4PipelineDataSetSerializerDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the configuration.
    ///
    /// C++ equivalent: `PipelineDataSetSerializerConfiguration configuration() const`
    pub fn configuration(&self) -> PipelineDataSetSerializerConfiguration {
        unsafe { msg_send_0(self.as_ptr(), sel!(configuration)) }
    }

    /// Set the configuration.
    ///
    /// C++ equivalent: `void setConfiguration(MTL4::PipelineDataSetSerializerConfiguration)`
    pub fn set_configuration(&self, configuration: PipelineDataSetSerializerConfiguration) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setConfiguration:), configuration);
        }
    }
}

impl Clone for PipelineDataSetSerializerDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for PipelineDataSetSerializerDescriptor {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for PipelineDataSetSerializerDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for PipelineDataSetSerializerDescriptor {}
unsafe impl Sync for PipelineDataSetSerializerDescriptor {}

impl std::fmt::Debug for PipelineDataSetSerializerDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipelineDataSetSerializerDescriptor")
            .field("configuration", &self.configuration())
            .finish()
    }
}

// ============================================================
// PipelineDataSetSerializer
// ============================================================

/// Serializes pipeline data sets for caching.
///
/// C++ equivalent: `MTL4::PipelineDataSetSerializer`
///
/// PipelineDataSetSerializer captures pipeline creation data for
/// later serialization to disk or as a pipelines script.
#[repr(transparent)]
pub struct PipelineDataSetSerializer(NonNull<c_void>);

impl PipelineDataSetSerializer {
    /// Create a PipelineDataSetSerializer from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Serialize as an archive and flush to a URL.
    ///
    /// C++ equivalent: `bool serializeAsArchiveAndFlushToURL(const NS::URL*, NS::Error**)`
    ///
    /// Returns `Ok(true)` on success, `Err` with the error on failure.
    pub fn serialize_as_archive_and_flush_to_url(
        &self,
        url: *const c_void,
    ) -> Result<bool, mtl_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let result: bool = msg_send_2(
                self.as_ptr(),
                sel!(serializeAsArchiveAndFlushToURL:error:),
                url,
                &mut error as *mut _,
            );
            if !error.is_null() {
                if let Some(err) = mtl_foundation::Error::from_ptr(error) {
                    return Err(err);
                }
            }
            Ok(result)
        }
    }

    /// Serialize as a pipelines script.
    ///
    /// C++ equivalent: `NS::Data* serializeAsPipelinesScript(NS::Error**)`
    ///
    /// Returns the serialized data on success, or an error on failure.
    pub fn serialize_as_pipelines_script(&self) -> Result<*mut c_void, mtl_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let data: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(serializeAsPipelinesScriptWithError:),
                &mut error as *mut _,
            );
            if !error.is_null() {
                if let Some(err) = mtl_foundation::Error::from_ptr(error) {
                    return Err(err);
                }
            }
            Ok(data)
        }
    }
}

impl Clone for PipelineDataSetSerializer {
    fn clone(&self) -> Self {
        unsafe {
            mtl_sys::msg_send_0::<*mut c_void>(self.as_ptr(), mtl_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for PipelineDataSetSerializer {
    fn drop(&mut self) {
        unsafe {
            mtl_sys::msg_send_0::<()>(self.as_ptr(), mtl_sys::sel!(release));
        }
    }
}

impl Referencing for PipelineDataSetSerializer {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for PipelineDataSetSerializer {}
unsafe impl Sync for PipelineDataSetSerializer {}

impl std::fmt::Debug for PipelineDataSetSerializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipelineDataSetSerializer").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_data_set_serializer_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<PipelineDataSetSerializerDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_pipeline_data_set_serializer_size() {
        assert_eq!(
            std::mem::size_of::<PipelineDataSetSerializer>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
