//! Pipeline state methods for ComputeCommandEncoder.

use std::ffi::c_void;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::DispatchType;

use super::ComputeCommandEncoder;

impl ComputeCommandEncoder {
    /// Set the compute pipeline state.
    ///
    /// C++ equivalent: `void setComputePipelineState(const ComputePipelineState*)`
    #[inline]
    pub fn set_compute_pipeline_state(&self, state: &crate::ComputePipelineState) {
        unsafe {
            msg_send_1::<(), *const c_void>(
                self.as_ptr(),
                sel!(setComputePipelineState:),
                state.as_ptr(),
            );
        }
    }

    /// Get the dispatch type for this encoder.
    ///
    /// C++ equivalent: `DispatchType dispatchType() const`
    #[inline]
    pub fn dispatch_type(&self) -> DispatchType {
        unsafe { msg_send_0(self.as_ptr(), sel!(dispatchType)) }
    }
}
