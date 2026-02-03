//! Device pipeline state creation methods.
//!
//! Corresponds to pipeline state creation methods in `Metal/MTLDevice.hpp`.

use std::ffi::c_void;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_0, msg_send_2, msg_send_3, sel};

use super::Device;
use crate::error::ValidationError;
use crate::library::Function;
use crate::pipeline::{
    ComputePipelineDescriptor, ComputePipelineReflection, ComputePipelineState,
    MeshRenderPipelineDescriptor, RenderPipelineDescriptor, RenderPipelineReflection,
    RenderPipelineState, TileRenderPipelineDescriptor,
};

impl Device {
    // =========================================================================
    // Render Pipeline State Creation
    // =========================================================================

    /// Create a render pipeline state from a descriptor.
    ///
    /// C++ equivalent: `RenderPipelineState* newRenderPipelineState(const RenderPipelineDescriptor*, NS::Error**)`
    ///
    /// # Safety
    ///
    /// The descriptor pointer must be valid.
    pub unsafe fn new_render_pipeline_state(
        &self,
        descriptor: *const c_void,
    ) -> Result<RenderPipelineState, mtl_foundation::Error> {
        let mut error: *mut c_void = std::ptr::null_mut();
        unsafe {
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newRenderPipelineStateWithDescriptor: error:),
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

            Ok(RenderPipelineState::from_raw(ptr).expect("render pipeline state should be valid"))
        }
    }

    /// Create a render pipeline state with validation.
    ///
    /// This safe method validates the descriptor before calling Metal APIs:
    /// - Ensures a vertex function is set (required)
    /// - Validates raster sample count is supported by the device
    ///
    /// Use this method instead of `new_render_pipeline_state` to avoid process
    /// aborts from Metal's validation layer.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let desc = RenderPipelineDescriptor::new().unwrap();
    /// desc.set_vertex_function(Some(&vertex_fn));
    /// desc.set_fragment_function(Some(&fragment_fn));
    ///
    /// match device.new_render_pipeline_state_with_descriptor(&desc) {
    ///     Ok(pipeline) => { /* use pipeline */ }
    ///     Err(ValidationError::MissingVertexFunction) => { /* handle error */ }
    ///     Err(e) => { /* handle other errors */ }
    /// }
    /// ```
    pub fn new_render_pipeline_state_with_descriptor(
        &self,
        descriptor: &RenderPipelineDescriptor,
    ) -> Result<RenderPipelineState, ValidationError> {
        // Validate vertex function is set
        if descriptor.vertex_function().is_none() {
            return Err(ValidationError::MissingVertexFunction);
        }

        // Validate raster sample count if > 1
        let sample_count = descriptor.raster_sample_count();
        if sample_count > 1 && !self.supports_texture_sample_count(sample_count) {
            return Err(ValidationError::UnsupportedRasterSampleCount(sample_count));
        }

        // Call unsafe implementation
        unsafe {
            self.new_render_pipeline_state(descriptor.as_ptr())
                .map_err(ValidationError::from)
        }
    }

    /// Create a render pipeline state with options.
    ///
    /// C++ equivalent: `RenderPipelineState* newRenderPipelineState(const RenderPipelineDescriptor*, PipelineOption, RenderPipelineReflection**, NS::Error**)`
    ///
    /// # Safety
    ///
    /// The descriptor and reflection pointers must be valid.
    pub unsafe fn new_render_pipeline_state_with_reflection(
        &self,
        descriptor: *const c_void,
        options: crate::enums::PipelineOption,
        reflection: *mut *mut c_void,
    ) -> Result<RenderPipelineState, mtl_foundation::Error> {
        let mut error: *mut c_void = std::ptr::null_mut();
        unsafe {
            let ptr: *mut c_void = mtl_sys::msg_send_4(
                self.as_ptr(),
                sel!(newRenderPipelineStateWithDescriptor: options: reflection: error:),
                descriptor,
                options,
                reflection,
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

            Ok(RenderPipelineState::from_raw(ptr).expect("render pipeline state should be valid"))
        }
    }

    // =========================================================================
    // Compute Pipeline State Creation
    // =========================================================================

    /// Create a compute pipeline state from a function.
    ///
    /// C++ equivalent: `ComputePipelineState* newComputePipelineState(const Function*, NS::Error**)`
    pub fn new_compute_pipeline_state_with_function(
        &self,
        function: &Function,
    ) -> Result<ComputePipelineState, mtl_foundation::Error> {
        let mut error: *mut c_void = std::ptr::null_mut();
        unsafe {
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(newComputePipelineStateWithFunction: error:),
                function.as_ptr(),
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

            Ok(
                ComputePipelineState::from_raw(ptr)
                    .expect("compute pipeline state should be valid"),
            )
        }
    }

    /// Create a compute pipeline state with options.
    ///
    /// C++ equivalent: `ComputePipelineState* newComputePipelineState(const Function*, PipelineOption, ComputePipelineReflection**, NS::Error**)`
    ///
    /// # Safety
    ///
    /// The reflection pointer must be valid if not null.
    pub unsafe fn new_compute_pipeline_state_with_function_and_reflection(
        &self,
        function: &Function,
        options: crate::enums::PipelineOption,
        reflection: *mut *mut c_void,
    ) -> Result<ComputePipelineState, mtl_foundation::Error> {
        let mut error: *mut c_void = std::ptr::null_mut();
        unsafe {
            let ptr: *mut c_void = mtl_sys::msg_send_4(
                self.as_ptr(),
                sel!(newComputePipelineStateWithFunction: options: reflection: error:),
                function.as_ptr(),
                options,
                reflection,
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

            Ok(
                ComputePipelineState::from_raw(ptr)
                    .expect("compute pipeline state should be valid"),
            )
        }
    }

    /// Create a compute pipeline state with validation.
    ///
    /// This safe method validates the descriptor before calling Metal APIs:
    /// - Ensures a compute function is set (required)
    ///
    /// Use this method instead of the unsafe `new_compute_pipeline_state_with_descriptor`
    /// to avoid process aborts from Metal's validation layer.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let desc = ComputePipelineDescriptor::new().unwrap();
    /// desc.set_compute_function(Some(&compute_fn));
    ///
    /// match device.new_compute_pipeline_state_validated(&desc) {
    ///     Ok(pipeline) => { /* use pipeline */ }
    ///     Err(ValidationError::MissingComputeFunction) => { /* handle error */ }
    ///     Err(e) => { /* handle other errors */ }
    /// }
    /// ```
    pub fn new_compute_pipeline_state_validated(
        &self,
        descriptor: &ComputePipelineDescriptor,
    ) -> Result<ComputePipelineState, ValidationError> {
        // Validate compute function is set
        if descriptor.compute_function().is_none() {
            return Err(ValidationError::MissingComputeFunction);
        }

        // Call the unsafe implementation with default options and no reflection
        unsafe {
            self.new_compute_pipeline_state_with_descriptor(
                descriptor.as_ptr(),
                crate::enums::PipelineOption::NONE,
                std::ptr::null_mut(),
            )
            .map_err(ValidationError::from)
        }
    }

    /// Create a compute pipeline state from a descriptor.
    ///
    /// C++ equivalent: `ComputePipelineState* newComputePipelineState(const ComputePipelineDescriptor*, PipelineOption, ComputePipelineReflection**, NS::Error**)`
    ///
    /// # Safety
    ///
    /// The descriptor and reflection pointers must be valid.
    pub unsafe fn new_compute_pipeline_state_with_descriptor(
        &self,
        descriptor: *const c_void,
        options: crate::enums::PipelineOption,
        reflection: *mut *mut c_void,
    ) -> Result<ComputePipelineState, mtl_foundation::Error> {
        let mut error: *mut c_void = std::ptr::null_mut();
        unsafe {
            let ptr: *mut c_void = mtl_sys::msg_send_4(
                self.as_ptr(),
                sel!(newComputePipelineStateWithDescriptor: options: reflection: error:),
                descriptor,
                options,
                reflection,
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

            Ok(
                ComputePipelineState::from_raw(ptr)
                    .expect("compute pipeline state should be valid"),
            )
        }
    }

    // =========================================================================
    // Async Render Pipeline State Creation
    // =========================================================================

    /// Create a render pipeline state asynchronously.
    ///
    /// C++ equivalent: `void newRenderPipelineState(const RenderPipelineDescriptor*, NewRenderPipelineStateCompletionHandler)`
    ///
    /// The completion handler is called with the pipeline state and any error that occurred.
    pub fn new_render_pipeline_state_async<F>(
        &self,
        descriptor: &RenderPipelineDescriptor,
        completion_handler: F,
    ) where
        F: Fn(Option<RenderPipelineState>, Option<mtl_foundation::Error>) + Send + 'static,
    {
        let block =
            mtl_sys::TwoArgBlock::from_fn(move |state_ptr: *mut c_void, err_ptr: *mut c_void| {
                let state = if state_ptr.is_null() {
                    None
                } else {
                    unsafe { RenderPipelineState::from_raw(state_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { mtl_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(state, error);
            });

        unsafe {
            msg_send_2::<(), *const c_void, *const c_void>(
                self.as_ptr(),
                sel!(newRenderPipelineStateWithDescriptor:completionHandler:),
                descriptor.as_ptr(),
                block.as_ptr(),
            );
        }

        std::mem::forget(block);
    }

    /// Create a render pipeline state with reflection asynchronously.
    ///
    /// C++ equivalent: `void newRenderPipelineState(const RenderPipelineDescriptor*, PipelineOption, NewRenderPipelineStateWithReflectionCompletionHandler)`
    ///
    /// The completion handler is called with the pipeline state, reflection data, and any error.
    pub fn new_render_pipeline_state_with_reflection_async<F>(
        &self,
        descriptor: &RenderPipelineDescriptor,
        options: crate::enums::PipelineOption,
        completion_handler: F,
    ) where
        F: Fn(
                Option<RenderPipelineState>,
                Option<RenderPipelineReflection>,
                Option<mtl_foundation::Error>,
            ) + Send
            + 'static,
    {
        let block = mtl_sys::ThreeArgBlock::from_fn(
            move |state_ptr: *mut c_void, reflection_ptr: *mut c_void, err_ptr: *mut c_void| {
                let state = if state_ptr.is_null() {
                    None
                } else {
                    unsafe { RenderPipelineState::from_raw(state_ptr) }
                };

                let reflection = if reflection_ptr.is_null() {
                    None
                } else {
                    unsafe { RenderPipelineReflection::from_raw(reflection_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { mtl_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(state, reflection, error);
            },
        );

        unsafe {
            msg_send_3::<(), *const c_void, crate::enums::PipelineOption, *const c_void>(
                self.as_ptr(),
                sel!(newRenderPipelineStateWithDescriptor:options:completionHandler:),
                descriptor.as_ptr(),
                options,
                block.as_ptr(),
            );
        }

        std::mem::forget(block);
    }

    /// Create a tile render pipeline state with reflection asynchronously.
    ///
    /// C++ equivalent: `void newRenderPipelineState(const TileRenderPipelineDescriptor*, PipelineOption, NewRenderPipelineStateWithReflectionCompletionHandler)`
    pub fn new_tile_render_pipeline_state_with_reflection_async<F>(
        &self,
        descriptor: &TileRenderPipelineDescriptor,
        options: crate::enums::PipelineOption,
        completion_handler: F,
    ) where
        F: Fn(
                Option<RenderPipelineState>,
                Option<RenderPipelineReflection>,
                Option<mtl_foundation::Error>,
            ) + Send
            + 'static,
    {
        let block = mtl_sys::ThreeArgBlock::from_fn(
            move |state_ptr: *mut c_void, reflection_ptr: *mut c_void, err_ptr: *mut c_void| {
                let state = if state_ptr.is_null() {
                    None
                } else {
                    unsafe { RenderPipelineState::from_raw(state_ptr) }
                };

                let reflection = if reflection_ptr.is_null() {
                    None
                } else {
                    unsafe { RenderPipelineReflection::from_raw(reflection_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { mtl_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(state, reflection, error);
            },
        );

        unsafe {
            msg_send_3::<(), *const c_void, crate::enums::PipelineOption, *const c_void>(
                self.as_ptr(),
                sel!(newRenderPipelineStateWithTileDescriptor:options:completionHandler:),
                descriptor.as_ptr(),
                options,
                block.as_ptr(),
            );
        }

        std::mem::forget(block);
    }

    /// Create a mesh render pipeline state with reflection asynchronously.
    ///
    /// C++ equivalent: `void newRenderPipelineState(const MeshRenderPipelineDescriptor*, PipelineOption, NewRenderPipelineStateWithReflectionCompletionHandler)`
    pub fn new_mesh_render_pipeline_state_with_reflection_async<F>(
        &self,
        descriptor: &MeshRenderPipelineDescriptor,
        options: crate::enums::PipelineOption,
        completion_handler: F,
    ) where
        F: Fn(
                Option<RenderPipelineState>,
                Option<RenderPipelineReflection>,
                Option<mtl_foundation::Error>,
            ) + Send
            + 'static,
    {
        let block = mtl_sys::ThreeArgBlock::from_fn(
            move |state_ptr: *mut c_void, reflection_ptr: *mut c_void, err_ptr: *mut c_void| {
                let state = if state_ptr.is_null() {
                    None
                } else {
                    unsafe { RenderPipelineState::from_raw(state_ptr) }
                };

                let reflection = if reflection_ptr.is_null() {
                    None
                } else {
                    unsafe { RenderPipelineReflection::from_raw(reflection_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { mtl_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(state, reflection, error);
            },
        );

        unsafe {
            msg_send_3::<(), *const c_void, crate::enums::PipelineOption, *const c_void>(
                self.as_ptr(),
                sel!(newRenderPipelineStateWithMeshDescriptor:options:completionHandler:),
                descriptor.as_ptr(),
                options,
                block.as_ptr(),
            );
        }

        std::mem::forget(block);
    }

    // =========================================================================
    // Async Compute Pipeline State Creation
    // =========================================================================

    /// Create a compute pipeline state from a function asynchronously.
    ///
    /// C++ equivalent: `void newComputePipelineState(const Function*, NewComputePipelineStateCompletionHandler)`
    pub fn new_compute_pipeline_state_with_function_async<F>(
        &self,
        function: &Function,
        completion_handler: F,
    ) where
        F: Fn(Option<ComputePipelineState>, Option<mtl_foundation::Error>) + Send + 'static,
    {
        let block =
            mtl_sys::TwoArgBlock::from_fn(move |state_ptr: *mut c_void, err_ptr: *mut c_void| {
                let state = if state_ptr.is_null() {
                    None
                } else {
                    unsafe { ComputePipelineState::from_raw(state_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { mtl_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(state, error);
            });

        unsafe {
            msg_send_2::<(), *const c_void, *const c_void>(
                self.as_ptr(),
                sel!(newComputePipelineStateWithFunction:completionHandler:),
                function.as_ptr(),
                block.as_ptr(),
            );
        }

        std::mem::forget(block);
    }

    /// Create a compute pipeline state with reflection asynchronously.
    ///
    /// C++ equivalent: `void newComputePipelineState(const Function*, PipelineOption, NewComputePipelineStateWithReflectionCompletionHandler)`
    pub fn new_compute_pipeline_state_with_function_and_reflection_async<F>(
        &self,
        function: &Function,
        options: crate::enums::PipelineOption,
        completion_handler: F,
    ) where
        F: Fn(
                Option<ComputePipelineState>,
                Option<ComputePipelineReflection>,
                Option<mtl_foundation::Error>,
            ) + Send
            + 'static,
    {
        let block = mtl_sys::ThreeArgBlock::from_fn(
            move |state_ptr: *mut c_void, reflection_ptr: *mut c_void, err_ptr: *mut c_void| {
                let state = if state_ptr.is_null() {
                    None
                } else {
                    unsafe { ComputePipelineState::from_raw(state_ptr) }
                };

                let reflection = if reflection_ptr.is_null() {
                    None
                } else {
                    unsafe { ComputePipelineReflection::from_raw(reflection_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { mtl_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(state, reflection, error);
            },
        );

        unsafe {
            msg_send_3::<(), *const c_void, crate::enums::PipelineOption, *const c_void>(
                self.as_ptr(),
                sel!(newComputePipelineStateWithFunction:options:completionHandler:),
                function.as_ptr(),
                options,
                block.as_ptr(),
            );
        }

        std::mem::forget(block);
    }

    /// Create a compute pipeline state from a descriptor asynchronously.
    ///
    /// C++ equivalent: `void newComputePipelineState(const ComputePipelineDescriptor*, PipelineOption, NewComputePipelineStateWithReflectionCompletionHandler)`
    pub fn new_compute_pipeline_state_with_descriptor_async<F>(
        &self,
        descriptor: &ComputePipelineDescriptor,
        options: crate::enums::PipelineOption,
        completion_handler: F,
    ) where
        F: Fn(
                Option<ComputePipelineState>,
                Option<ComputePipelineReflection>,
                Option<mtl_foundation::Error>,
            ) + Send
            + 'static,
    {
        let block = mtl_sys::ThreeArgBlock::from_fn(
            move |state_ptr: *mut c_void, reflection_ptr: *mut c_void, err_ptr: *mut c_void| {
                let state = if state_ptr.is_null() {
                    None
                } else {
                    unsafe { ComputePipelineState::from_raw(state_ptr) }
                };

                let reflection = if reflection_ptr.is_null() {
                    None
                } else {
                    unsafe { ComputePipelineReflection::from_raw(reflection_ptr) }
                };

                let error = if err_ptr.is_null() {
                    None
                } else {
                    unsafe { mtl_foundation::Error::from_ptr(err_ptr) }
                };

                completion_handler(state, reflection, error);
            },
        );

        unsafe {
            msg_send_3::<(), *const c_void, crate::enums::PipelineOption, *const c_void>(
                self.as_ptr(),
                sel!(newComputePipelineStateWithDescriptor:options:completionHandler:),
                descriptor.as_ptr(),
                options,
                block.as_ptr(),
            );
        }

        std::mem::forget(block);
    }
}

#[cfg(test)]
mod tests {
    use crate::device::system_default;

    #[test]
    fn test_new_compute_pipeline_state() {
        let device = system_default().expect("no Metal device");

        let source = r#"
            #include <metal_stdlib>
            using namespace metal;

            kernel void test_kernel(device float* data [[buffer(0)]],
                                   uint id [[thread_position_in_grid]]) {
                data[id] = data[id] * 2.0;
            }
        "#;

        let library = device
            .new_library_with_source(source, None)
            .expect("failed to compile shader");

        let function = library
            .new_function_with_name("test_kernel")
            .expect("function not found");

        let pipeline = device.new_compute_pipeline_state_with_function(&function);
        assert!(pipeline.is_ok());

        let pipeline = pipeline.unwrap();
        assert!(pipeline.max_total_threads_per_threadgroup() > 0);
        assert!(pipeline.thread_execution_width() > 0);
    }
}
