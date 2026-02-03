//! MTL4 Compiler implementation.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, msg_send_2, msg_send_3, msg_send_4, sel};

use crate::mtl4::{
    BinaryFunction, BinaryFunctionDescriptor, CompilerTask, ComputePipelineDescriptor,
    LibraryDescriptor, MachineLearningPipelineDescriptor, MachineLearningPipelineState,
    PipelineDataSetSerializer, PipelineDescriptor, PipelineStageDynamicLinkingDescriptor,
    RenderPipelineDynamicLinkingDescriptor,
};
use crate::{ComputePipelineState, Device, DynamicLibrary, Library, RenderPipelineState};

use super::CompilerTaskOptions;

/// Helper to create a generic error.
fn generic_error() -> metal_foundation::Error {
    metal_foundation::Error::error(std::ptr::null_mut(), -1, std::ptr::null_mut())
        .expect("failed to create error object")
}

/// MTL4 shader compiler.
///
/// C++ equivalent: `MTL4::Compiler`
///
/// Compiler provides methods for compiling shaders, creating pipelines,
/// and managing binary functions.
#[repr(transparent)]
pub struct Compiler(NonNull<c_void>);

impl Compiler {
    /// Create a Compiler from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the device.
    ///
    /// C++ equivalent: `MTL::Device* device() const`
    pub fn device(&self) -> Option<Device> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(device));
            Device::from_raw(ptr)
        }
    }

    /// Get the label.
    ///
    /// C++ equivalent: `NS::String* label() const`
    pub fn label(&self) -> Option<String> {
        unsafe {
            let ns_string: *mut c_void = msg_send_0(self.as_ptr(), sel!(label));
            if ns_string.is_null() {
                return None;
            }
            let c_str: *const i8 = msg_send_0(ns_string, sel!(UTF8String));
            if c_str.is_null() {
                return None;
            }
            Some(
                std::ffi::CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    }

    /// Get the pipeline data set serializer.
    ///
    /// C++ equivalent: `PipelineDataSetSerializer* pipelineDataSetSerializer() const`
    pub fn pipeline_data_set_serializer(&self) -> Option<PipelineDataSetSerializer> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(pipelineDataSetSerializer));
            PipelineDataSetSerializer::from_raw(ptr)
        }
    }

    // ========== Library Creation ==========

    /// Create a new library synchronously.
    ///
    /// C++ equivalent: `MTL::Library* newLibrary(const MTL4::LibraryDescriptor*, NS::Error**)`
    pub fn new_library(
        &self,
        descriptor: &LibraryDescriptor,
    ) -> Result<Library, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newLibraryWithDescriptor:error:),
                descriptor.as_ptr(),
                &mut error as *mut _,
            );
            if !error.is_null() {
                if let Some(err) = metal_foundation::Error::from_ptr(error) {
                    return Err(err);
                }
            }
            Library::from_raw(ptr).ok_or_else(|| generic_error())
        }
    }

    // ========== Binary Function Creation ==========

    /// Create a new binary function synchronously.
    ///
    /// C++ equivalent: `BinaryFunction* newBinaryFunction(..., NS::Error**)`
    pub fn new_binary_function(
        &self,
        descriptor: &BinaryFunctionDescriptor,
        options: Option<&CompilerTaskOptions>,
    ) -> Result<BinaryFunction, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let options_ptr = options.map_or(std::ptr::null(), |o| o.as_ptr());
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newBinaryFunctionWithDescriptor:compilerTaskOptions:error:),
                descriptor.as_ptr(),
                options_ptr,
                &mut error as *mut _,
            );
            if !error.is_null() {
                if let Some(err) = metal_foundation::Error::from_ptr(error) {
                    return Err(err);
                }
            }
            BinaryFunction::from_raw(ptr).ok_or_else(|| generic_error())
        }
    }

    // ========== Compute Pipeline Creation ==========

    /// Create a new compute pipeline state synchronously.
    ///
    /// C++ equivalent: `MTL::ComputePipelineState* newComputePipelineState(..., NS::Error**)`
    pub fn new_compute_pipeline_state(
        &self,
        descriptor: &ComputePipelineDescriptor,
        options: Option<&CompilerTaskOptions>,
    ) -> Result<ComputePipelineState, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let options_ptr = options.map_or(std::ptr::null(), |o| o.as_ptr());
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newComputePipelineStateWithDescriptor:compilerTaskOptions:error:),
                descriptor.as_ptr(),
                options_ptr,
                &mut error as *mut _,
            );
            if !error.is_null() {
                if let Some(err) = metal_foundation::Error::from_ptr(error) {
                    return Err(err);
                }
            }
            ComputePipelineState::from_raw(ptr).ok_or_else(|| generic_error())
        }
    }

    /// Create a new compute pipeline state with dynamic linking synchronously.
    ///
    /// C++ equivalent: `MTL::ComputePipelineState* newComputePipelineState(..., dynamicLinkingDescriptor, ..., NS::Error**)`
    pub fn new_compute_pipeline_state_with_dynamic_linking(
        &self,
        descriptor: &ComputePipelineDescriptor,
        dynamic_linking: &PipelineStageDynamicLinkingDescriptor,
        options: Option<&CompilerTaskOptions>,
    ) -> Result<ComputePipelineState, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let options_ptr = options.map_or(std::ptr::null(), |o| o.as_ptr());
            let ptr: *mut c_void = msg_send_4(
                self.as_ptr(),
                sel!(newComputePipelineStateWithDescriptor:dynamicLinkingDescriptor:compilerTaskOptions:error:),
                descriptor.as_ptr(),
                dynamic_linking.as_ptr(),
                options_ptr,
                &mut error as *mut _,
            );
            if !error.is_null() {
                if let Some(err) = metal_foundation::Error::from_ptr(error) {
                    return Err(err);
                }
            }
            ComputePipelineState::from_raw(ptr).ok_or_else(|| generic_error())
        }
    }

    // ========== Render Pipeline Creation ==========

    /// Create a new render pipeline state synchronously.
    ///
    /// C++ equivalent: `MTL::RenderPipelineState* newRenderPipelineState(..., NS::Error**)`
    pub fn new_render_pipeline_state(
        &self,
        descriptor: &PipelineDescriptor,
        options: Option<&CompilerTaskOptions>,
    ) -> Result<RenderPipelineState, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let options_ptr = options.map_or(std::ptr::null(), |o| o.as_ptr());
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newRenderPipelineStateWithDescriptor:compilerTaskOptions:error:),
                descriptor.as_ptr(),
                options_ptr,
                &mut error as *mut _,
            );
            if !error.is_null() {
                if let Some(err) = metal_foundation::Error::from_ptr(error) {
                    return Err(err);
                }
            }
            RenderPipelineState::from_raw(ptr).ok_or_else(|| generic_error())
        }
    }

    /// Create a new render pipeline state with dynamic linking synchronously.
    ///
    /// C++ equivalent: `MTL::RenderPipelineState* newRenderPipelineState(..., dynamicLinkingDescriptor, ..., NS::Error**)`
    pub fn new_render_pipeline_state_with_dynamic_linking(
        &self,
        descriptor: &PipelineDescriptor,
        dynamic_linking: &RenderPipelineDynamicLinkingDescriptor,
        options: Option<&CompilerTaskOptions>,
    ) -> Result<RenderPipelineState, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let options_ptr = options.map_or(std::ptr::null(), |o| o.as_ptr());
            let ptr: *mut c_void = msg_send_4(
                self.as_ptr(),
                sel!(newRenderPipelineStateWithDescriptor:dynamicLinkingDescriptor:compilerTaskOptions:error:),
                descriptor.as_ptr(),
                dynamic_linking.as_ptr(),
                options_ptr,
                &mut error as *mut _,
            );
            if !error.is_null() {
                if let Some(err) = metal_foundation::Error::from_ptr(error) {
                    return Err(err);
                }
            }
            RenderPipelineState::from_raw(ptr).ok_or_else(|| generic_error())
        }
    }

    /// Create a new render pipeline state by specialization.
    ///
    /// C++ equivalent: `MTL::RenderPipelineState* newRenderPipelineStateBySpecialization(...)`
    pub fn new_render_pipeline_state_by_specialization(
        &self,
        descriptor: &PipelineDescriptor,
        pipeline: &RenderPipelineState,
    ) -> Result<RenderPipelineState, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newRenderPipelineStateBySpecializationWithDescriptor:pipeline:error:),
                descriptor.as_ptr(),
                pipeline.as_ptr(),
                &mut error as *mut _,
            );
            if !error.is_null() {
                if let Some(err) = metal_foundation::Error::from_ptr(error) {
                    return Err(err);
                }
            }
            RenderPipelineState::from_raw(ptr).ok_or_else(|| generic_error())
        }
    }

    // ========== Dynamic Library Creation ==========

    /// Create a new dynamic library from a library.
    ///
    /// C++ equivalent: `MTL::DynamicLibrary* newDynamicLibrary(const MTL::Library*, NS::Error**)`
    pub fn new_dynamic_library_from_library(
        &self,
        library: &Library,
    ) -> Result<*mut c_void, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newDynamicLibrary:error:),
                library.as_ptr(),
                &mut error as *mut _,
            );
            if !error.is_null() {
                if let Some(err) = metal_foundation::Error::from_ptr(error) {
                    return Err(err);
                }
            }
            if ptr.is_null() {
                return Err(generic_error());
            }
            Ok(ptr)
        }
    }

    /// Create a new dynamic library from a URL.
    ///
    /// C++ equivalent: `MTL::DynamicLibrary* newDynamicLibrary(const NS::URL*, NS::Error**)`
    pub fn new_dynamic_library_from_url(
        &self,
        url: *const c_void,
    ) -> Result<*mut c_void, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newDynamicLibraryWithURL:error:),
                url,
                &mut error as *mut _,
            );
            if !error.is_null() {
                if let Some(err) = metal_foundation::Error::from_ptr(error) {
                    return Err(err);
                }
            }
            if ptr.is_null() {
                return Err(generic_error());
            }
            Ok(ptr)
        }
    }

    // ========== Async Library Creation ==========

    /// Create a new library asynchronously with a completion handler.
    ///
    /// C++ equivalent: `CompilerTask* newLibrary(const LibraryDescriptor*, NewLibraryCompletionHandler)`
    pub fn new_library_async<F>(
        &self,
        descriptor: &LibraryDescriptor,
        completion_handler: F,
    ) -> Option<CompilerTask>
    where
        F: Fn(Option<Library>, Option<metal_foundation::Error>) + Send + 'static,
    {
        let block =
            metal_sys::TwoArgBlock::from_fn(move |lib_ptr: *mut c_void, err_ptr: *mut c_void| {
                let library = if lib_ptr.is_null() {
                    None
                } else {
                    unsafe { Library::from_raw(lib_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { metal_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(library, error);
            });

        unsafe {
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newLibraryWithDescriptor:completionHandler:),
                descriptor.as_ptr(),
                block.as_ptr(),
            );

            std::mem::forget(block);
            CompilerTask::from_raw(ptr)
        }
    }

    // ========== Async Binary Function Creation ==========

    /// Create a new binary function asynchronously with a completion handler.
    ///
    /// C++ equivalent: `CompilerTask* newBinaryFunction(const BinaryFunctionDescriptor*, CompilerTaskOptions*, NewBinaryFunctionCompletionHandler)`
    pub fn new_binary_function_async<F>(
        &self,
        descriptor: &BinaryFunctionDescriptor,
        options: Option<&CompilerTaskOptions>,
        completion_handler: F,
    ) -> Option<CompilerTask>
    where
        F: Fn(Option<BinaryFunction>, Option<metal_foundation::Error>) + Send + 'static,
    {
        let block =
            metal_sys::TwoArgBlock::from_fn(move |fn_ptr: *mut c_void, err_ptr: *mut c_void| {
                let function = if fn_ptr.is_null() {
                    None
                } else {
                    unsafe { BinaryFunction::from_raw(fn_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { metal_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(function, error);
            });

        let options_ptr = options.map_or(std::ptr::null(), |o| o.as_ptr());

        unsafe {
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newBinaryFunctionWithDescriptor:compilerTaskOptions:completionHandler:),
                descriptor.as_ptr(),
                options_ptr,
                block.as_ptr(),
            );

            std::mem::forget(block);
            CompilerTask::from_raw(ptr)
        }
    }

    // ========== Async Compute Pipeline Creation ==========

    /// Create a new compute pipeline state asynchronously with a completion handler.
    ///
    /// C++ equivalent: `CompilerTask* newComputePipelineState(const ComputePipelineDescriptor*, CompilerTaskOptions*, NewComputePipelineStateCompletionHandler)`
    pub fn new_compute_pipeline_state_async<F>(
        &self,
        descriptor: &ComputePipelineDescriptor,
        options: Option<&CompilerTaskOptions>,
        completion_handler: F,
    ) -> Option<CompilerTask>
    where
        F: Fn(Option<ComputePipelineState>, Option<metal_foundation::Error>) + Send + 'static,
    {
        let block =
            metal_sys::TwoArgBlock::from_fn(move |state_ptr: *mut c_void, err_ptr: *mut c_void| {
                let state = if state_ptr.is_null() {
                    None
                } else {
                    unsafe { ComputePipelineState::from_raw(state_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { metal_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(state, error);
            });

        let options_ptr = options.map_or(std::ptr::null(), |o| o.as_ptr());

        unsafe {
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newComputePipelineStateWithDescriptor:compilerTaskOptions:completionHandler:),
                descriptor.as_ptr(),
                options_ptr,
                block.as_ptr(),
            );

            std::mem::forget(block);
            CompilerTask::from_raw(ptr)
        }
    }

    // ========== Async Render Pipeline Creation ==========

    /// Create a new render pipeline state asynchronously with a completion handler.
    ///
    /// C++ equivalent: `CompilerTask* newRenderPipelineState(const PipelineDescriptor*, CompilerTaskOptions*, NewRenderPipelineStateCompletionHandler)`
    pub fn new_render_pipeline_state_async<F>(
        &self,
        descriptor: &PipelineDescriptor,
        options: Option<&CompilerTaskOptions>,
        completion_handler: F,
    ) -> Option<CompilerTask>
    where
        F: Fn(Option<RenderPipelineState>, Option<metal_foundation::Error>) + Send + 'static,
    {
        let block =
            metal_sys::TwoArgBlock::from_fn(move |state_ptr: *mut c_void, err_ptr: *mut c_void| {
                let state = if state_ptr.is_null() {
                    None
                } else {
                    unsafe { RenderPipelineState::from_raw(state_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { metal_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(state, error);
            });

        let options_ptr = options.map_or(std::ptr::null(), |o| o.as_ptr());

        unsafe {
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newRenderPipelineStateWithDescriptor:compilerTaskOptions:completionHandler:),
                descriptor.as_ptr(),
                options_ptr,
                block.as_ptr(),
            );

            std::mem::forget(block);
            CompilerTask::from_raw(ptr)
        }
    }

    /// Create a new render pipeline state by specialization asynchronously.
    ///
    /// C++ equivalent: `CompilerTask* newRenderPipelineStateBySpecialization(..., NewRenderPipelineStateCompletionHandler)`
    pub fn new_render_pipeline_state_by_specialization_async<F>(
        &self,
        descriptor: &PipelineDescriptor,
        pipeline: &RenderPipelineState,
        completion_handler: F,
    ) -> Option<CompilerTask>
    where
        F: Fn(Option<RenderPipelineState>, Option<metal_foundation::Error>) + Send + 'static,
    {
        let block =
            metal_sys::TwoArgBlock::from_fn(move |state_ptr: *mut c_void, err_ptr: *mut c_void| {
                let state = if state_ptr.is_null() {
                    None
                } else {
                    unsafe { RenderPipelineState::from_raw(state_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { metal_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(state, error);
            });

        unsafe {
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newRenderPipelineStateBySpecializationWithDescriptor:pipeline:completionHandler:),
                descriptor.as_ptr(),
                pipeline.as_ptr(),
                block.as_ptr(),
            );

            std::mem::forget(block);
            CompilerTask::from_raw(ptr)
        }
    }

    // ========== Async Dynamic Library Creation ==========

    /// Create a new dynamic library from a library asynchronously.
    ///
    /// C++ equivalent: `CompilerTask* newDynamicLibrary(const Library*, NewDynamicLibraryCompletionHandler)`
    pub fn new_dynamic_library_from_library_async<F>(
        &self,
        library: &Library,
        completion_handler: F,
    ) -> Option<CompilerTask>
    where
        F: Fn(Option<DynamicLibrary>, Option<metal_foundation::Error>) + Send + 'static,
    {
        let block =
            metal_sys::TwoArgBlock::from_fn(move |lib_ptr: *mut c_void, err_ptr: *mut c_void| {
                let dynamic_lib = if lib_ptr.is_null() {
                    None
                } else {
                    unsafe { DynamicLibrary::from_raw(lib_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { metal_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(dynamic_lib, error);
            });

        unsafe {
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newDynamicLibrary:completionHandler:),
                library.as_ptr(),
                block.as_ptr(),
            );

            std::mem::forget(block);
            CompilerTask::from_raw(ptr)
        }
    }

    /// Create a new dynamic library from a URL asynchronously.
    ///
    /// C++ equivalent: `CompilerTask* newDynamicLibrary(const NS::URL*, NewDynamicLibraryCompletionHandler)`
    ///
    /// # Safety
    ///
    /// The URL pointer must be valid.
    pub unsafe fn new_dynamic_library_from_url_async<F>(
        &self,
        url: *const c_void,
        completion_handler: F,
    ) -> Option<CompilerTask>
    where
        F: Fn(Option<DynamicLibrary>, Option<metal_foundation::Error>) + Send + 'static,
    {
        let block =
            metal_sys::TwoArgBlock::from_fn(move |lib_ptr: *mut c_void, err_ptr: *mut c_void| {
                let dynamic_lib = if lib_ptr.is_null() {
                    None
                } else {
                    unsafe { DynamicLibrary::from_raw(lib_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { metal_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(dynamic_lib, error);
            });

        unsafe {
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newDynamicLibraryWithURL:completionHandler:),
                url,
                block.as_ptr(),
            );

            std::mem::forget(block);
            CompilerTask::from_raw(ptr)
        }
    }

    // ========== Async Dynamic Linking Pipeline Creation ==========

    /// Create a new compute pipeline state with dynamic linking asynchronously.
    ///
    /// C++ equivalent: `CompilerTask* newComputePipelineState(..., dynamicLinkingDescriptor, ..., completionHandler)`
    pub fn new_compute_pipeline_state_with_dynamic_linking_async<F>(
        &self,
        descriptor: &ComputePipelineDescriptor,
        dynamic_linking: &PipelineStageDynamicLinkingDescriptor,
        options: Option<&CompilerTaskOptions>,
        completion_handler: F,
    ) -> Option<CompilerTask>
    where
        F: Fn(Option<ComputePipelineState>, Option<metal_foundation::Error>) + Send + 'static,
    {
        let block =
            metal_sys::TwoArgBlock::from_fn(move |state_ptr: *mut c_void, err_ptr: *mut c_void| {
                let state = if state_ptr.is_null() {
                    None
                } else {
                    unsafe { ComputePipelineState::from_raw(state_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { metal_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(state, error);
            });

        let options_ptr = options.map_or(std::ptr::null(), |o| o.as_ptr());

        unsafe {
            let ptr: *mut c_void = msg_send_4(
                self.as_ptr(),
                sel!(newComputePipelineStateWithDescriptor:dynamicLinkingDescriptor:compilerTaskOptions:completionHandler:),
                descriptor.as_ptr(),
                dynamic_linking.as_ptr(),
                options_ptr,
                block.as_ptr(),
            );

            std::mem::forget(block);
            CompilerTask::from_raw(ptr)
        }
    }

    /// Create a new render pipeline state with dynamic linking asynchronously.
    ///
    /// C++ equivalent: `CompilerTask* newRenderPipelineState(..., dynamicLinkingDescriptor, ..., completionHandler)`
    pub fn new_render_pipeline_state_with_dynamic_linking_async<F>(
        &self,
        descriptor: &PipelineDescriptor,
        dynamic_linking: &RenderPipelineDynamicLinkingDescriptor,
        options: Option<&CompilerTaskOptions>,
        completion_handler: F,
    ) -> Option<CompilerTask>
    where
        F: Fn(Option<RenderPipelineState>, Option<metal_foundation::Error>) + Send + 'static,
    {
        let block =
            metal_sys::TwoArgBlock::from_fn(move |state_ptr: *mut c_void, err_ptr: *mut c_void| {
                let state = if state_ptr.is_null() {
                    None
                } else {
                    unsafe { RenderPipelineState::from_raw(state_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { metal_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(state, error);
            });

        let options_ptr = options.map_or(std::ptr::null(), |o| o.as_ptr());

        unsafe {
            let ptr: *mut c_void = msg_send_4(
                self.as_ptr(),
                sel!(newRenderPipelineStateWithDescriptor:dynamicLinkingDescriptor:compilerTaskOptions:completionHandler:),
                descriptor.as_ptr(),
                dynamic_linking.as_ptr(),
                options_ptr,
                block.as_ptr(),
            );

            std::mem::forget(block);
            CompilerTask::from_raw(ptr)
        }
    }

    // ========== Machine Learning Pipeline Creation ==========

    /// Create a new machine learning pipeline state synchronously.
    ///
    /// C++ equivalent: `MTL4::MachineLearningPipelineState* newMachineLearningPipelineState(..., NS::Error**)`
    pub fn new_machine_learning_pipeline_state(
        &self,
        descriptor: &MachineLearningPipelineDescriptor,
        options: Option<&CompilerTaskOptions>,
    ) -> Result<MachineLearningPipelineState, metal_foundation::Error> {
        unsafe {
            let mut error: *mut c_void = std::ptr::null_mut();
            let options_ptr = options.map_or(std::ptr::null(), |o| o.as_ptr());
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newMachineLearningPipelineStateWithDescriptor:compilerTaskOptions:error:),
                descriptor.as_ptr(),
                options_ptr,
                &mut error as *mut _,
            );
            if !error.is_null() {
                if let Some(err) = metal_foundation::Error::from_ptr(error) {
                    return Err(err);
                }
            }
            MachineLearningPipelineState::from_raw(ptr).ok_or_else(generic_error)
        }
    }

    /// Create a new machine learning pipeline state asynchronously.
    ///
    /// C++ equivalent: `CompilerTask* newMachineLearningPipelineState(..., completionHandler)`
    pub fn new_machine_learning_pipeline_state_async<F>(
        &self,
        descriptor: &MachineLearningPipelineDescriptor,
        options: Option<&CompilerTaskOptions>,
        completion_handler: F,
    ) -> Option<CompilerTask>
    where
        F: Fn(Option<MachineLearningPipelineState>, Option<metal_foundation::Error>)
            + Send
            + 'static,
    {
        let block =
            metal_sys::TwoArgBlock::from_fn(move |state_ptr: *mut c_void, err_ptr: *mut c_void| {
                let state = if state_ptr.is_null() {
                    None
                } else {
                    unsafe { MachineLearningPipelineState::from_raw(state_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { metal_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(state, error);
            });

        let options_ptr = options.map_or(std::ptr::null(), |o| o.as_ptr());

        unsafe {
            let ptr: *mut c_void = msg_send_3(
                self.as_ptr(),
                sel!(newMachineLearningPipelineStateWithDescriptor:compilerTaskOptions:completionHandler:),
                descriptor.as_ptr(),
                options_ptr,
                block.as_ptr(),
            );

            std::mem::forget(block);
            CompilerTask::from_raw(ptr)
        }
    }
}

impl Clone for Compiler {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for Compiler {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for Compiler {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for Compiler {}
unsafe impl Sync for Compiler {}

impl std::fmt::Debug for Compiler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Compiler")
            .field("label", &self.label())
            .finish()
    }
}
