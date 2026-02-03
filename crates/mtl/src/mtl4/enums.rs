//! Metal 4 enumerations.
//!
//! Corresponds to enums in MTL4 headers.

use mtl_foundation::{Integer, UInteger};

/// Error codes for MTL4 command queue operations.
///
/// C++ equivalent: `MTL4::CommandQueueError`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CommandQueueError(pub Integer);

impl CommandQueueError {
    /// No error.
    pub const NONE: Self = Self(0);

    /// Operation timed out.
    pub const TIMEOUT: Self = Self(1);

    /// Operation not permitted.
    pub const NOT_PERMITTED: Self = Self(2);

    /// Out of memory.
    pub const OUT_OF_MEMORY: Self = Self(3);

    /// Device was removed.
    pub const DEVICE_REMOVED: Self = Self(4);

    /// Access was revoked.
    pub const ACCESS_REVOKED: Self = Self(5);

    /// Internal error.
    pub const INTERNAL: Self = Self(6);
}

// ============================================================
// Pipeline State Enums
// ============================================================

/// Alpha to one state for pipeline.
///
/// C++ equivalent: `MTL4::AlphaToOneState`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AlphaToOneState(pub Integer);

impl AlphaToOneState {
    /// Alpha to one is disabled.
    pub const DISABLED: Self = Self(0);

    /// Alpha to one is enabled.
    pub const ENABLED: Self = Self(1);
}

/// Alpha to coverage state for pipeline.
///
/// C++ equivalent: `MTL4::AlphaToCoverageState`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AlphaToCoverageState(pub Integer);

impl AlphaToCoverageState {
    /// Alpha to coverage is disabled.
    pub const DISABLED: Self = Self(0);

    /// Alpha to coverage is enabled.
    pub const ENABLED: Self = Self(1);
}

/// Blend state for pipeline color attachments.
///
/// C++ equivalent: `MTL4::BlendState`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BlendState(pub Integer);

impl BlendState {
    /// Blending is disabled.
    pub const DISABLED: Self = Self(0);

    /// Blending is enabled.
    pub const ENABLED: Self = Self(1);

    /// Blending state is unspecialized (runtime configurable).
    pub const UNSPECIALIZED: Self = Self(2);
}

/// Indirect command buffer support state.
///
/// C++ equivalent: `MTL4::IndirectCommandBufferSupportState`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IndirectCommandBufferSupportState(pub Integer);

impl IndirectCommandBufferSupportState {
    /// Indirect command buffers are not supported.
    pub const DISABLED: Self = Self(0);

    /// Indirect command buffers are supported.
    pub const ENABLED: Self = Self(1);
}

/// Shader reflection options.
///
/// C++ equivalent: `MTL4::ShaderReflection`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ShaderReflection(pub UInteger);

impl ShaderReflection {
    /// No reflection.
    pub const NONE: Self = Self(0);

    /// Include binding info.
    pub const BINDING_INFO: Self = Self(1);

    /// Include buffer type info.
    pub const BUFFER_TYPE_INFO: Self = Self(1 << 1);

    /// Combine flags.
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Check if contains flag.
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for ShaderReflection {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for ShaderReflection {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

/// Logical to physical color attachment mapping state.
///
/// C++ equivalent: `MTL4::LogicalToPhysicalColorAttachmentMappingState`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct LogicalToPhysicalColorAttachmentMappingState(pub Integer);

impl LogicalToPhysicalColorAttachmentMappingState {
    /// Identity mapping (logical index equals physical index).
    pub const IDENTITY: Self = Self(0);

    /// Mapping is inherited from render pass.
    pub const INHERITED: Self = Self(1);
}

// ============================================================
// Compiler Enums
// ============================================================

/// Compiler task status.
///
/// C++ equivalent: `MTL4::CompilerTaskStatus`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CompilerTaskStatus(pub Integer);

impl CompilerTaskStatus {
    /// No status.
    pub const NONE: Self = Self(0);

    /// Task is scheduled.
    pub const SCHEDULED: Self = Self(1);

    /// Task is compiling.
    pub const COMPILING: Self = Self(2);

    /// Task is finished.
    pub const FINISHED: Self = Self(3);
}

/// Binary function options.
///
/// C++ equivalent: `MTL4::BinaryFunctionOptions`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BinaryFunctionOptions(pub UInteger);

impl BinaryFunctionOptions {
    /// No options.
    pub const NONE: Self = Self(0);

    /// Function is pipeline independent.
    pub const PIPELINE_INDEPENDENT: Self = Self(1 << 1);

    /// Combine flags.
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Check if contains flag.
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for BinaryFunctionOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for BinaryFunctionOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

/// Pipeline data set serializer configuration.
///
/// C++ equivalent: `MTL4::PipelineDataSetSerializerConfiguration`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PipelineDataSetSerializerConfiguration(pub UInteger);

impl PipelineDataSetSerializerConfiguration {
    /// Capture pipeline descriptors.
    pub const CAPTURE_DESCRIPTORS: Self = Self(1);

    /// Capture pipeline binaries.
    pub const CAPTURE_BINARIES: Self = Self(1 << 1);

    /// Combine flags.
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Check if contains flag.
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for PipelineDataSetSerializerConfiguration {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for PipelineDataSetSerializerConfiguration {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

// ============================================================
// Command Encoder Enums
// ============================================================

/// Visibility options for command encoder operations.
///
/// C++ equivalent: `MTL4::VisibilityOptions`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VisibilityOptions(pub UInteger);

impl VisibilityOptions {
    /// No visibility options.
    pub const NONE: Self = Self(0);

    /// Make visible to device.
    pub const DEVICE: Self = Self(1);

    /// Make visible for resource aliasing.
    pub const RESOURCE_ALIAS: Self = Self(1 << 1);

    /// Combine flags.
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Check if contains flag.
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for VisibilityOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for VisibilityOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

/// Render encoder options.
///
/// C++ equivalent: `MTL4::RenderEncoderOptions`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct RenderEncoderOptions(pub UInteger);

impl RenderEncoderOptions {
    /// No options.
    pub const NONE: Self = Self(0);

    /// Suspending render pass (can be resumed later).
    pub const SUSPENDING: Self = Self(1);

    /// Resuming a previously suspended render pass.
    pub const RESUMING: Self = Self(1 << 1);

    /// Combine flags.
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Check if contains flag.
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for RenderEncoderOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for RenderEncoderOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_queue_error_values() {
        assert_eq!(CommandQueueError::NONE.0, 0);
        assert_eq!(CommandQueueError::TIMEOUT.0, 1);
        assert_eq!(CommandQueueError::NOT_PERMITTED.0, 2);
        assert_eq!(CommandQueueError::OUT_OF_MEMORY.0, 3);
        assert_eq!(CommandQueueError::DEVICE_REMOVED.0, 4);
        assert_eq!(CommandQueueError::ACCESS_REVOKED.0, 5);
        assert_eq!(CommandQueueError::INTERNAL.0, 6);
    }

    #[test]
    fn test_alpha_to_one_state_values() {
        assert_eq!(AlphaToOneState::DISABLED.0, 0);
        assert_eq!(AlphaToOneState::ENABLED.0, 1);
    }

    #[test]
    fn test_alpha_to_coverage_state_values() {
        assert_eq!(AlphaToCoverageState::DISABLED.0, 0);
        assert_eq!(AlphaToCoverageState::ENABLED.0, 1);
    }

    #[test]
    fn test_blend_state_values() {
        assert_eq!(BlendState::DISABLED.0, 0);
        assert_eq!(BlendState::ENABLED.0, 1);
        assert_eq!(BlendState::UNSPECIALIZED.0, 2);
    }

    #[test]
    fn test_indirect_command_buffer_support_state_values() {
        assert_eq!(IndirectCommandBufferSupportState::DISABLED.0, 0);
        assert_eq!(IndirectCommandBufferSupportState::ENABLED.0, 1);
    }

    #[test]
    fn test_shader_reflection_values() {
        assert_eq!(ShaderReflection::NONE.0, 0);
        assert_eq!(ShaderReflection::BINDING_INFO.0, 1);
        assert_eq!(ShaderReflection::BUFFER_TYPE_INFO.0, 2);

        let combined = ShaderReflection::BINDING_INFO | ShaderReflection::BUFFER_TYPE_INFO;
        assert_eq!(combined.0, 3);
        assert!(combined.contains(ShaderReflection::BINDING_INFO));
        assert!(combined.contains(ShaderReflection::BUFFER_TYPE_INFO));
    }

    #[test]
    fn test_color_attachment_mapping_state_values() {
        assert_eq!(LogicalToPhysicalColorAttachmentMappingState::IDENTITY.0, 0);
        assert_eq!(LogicalToPhysicalColorAttachmentMappingState::INHERITED.0, 1);
    }

    #[test]
    fn test_compiler_task_status_values() {
        assert_eq!(CompilerTaskStatus::NONE.0, 0);
        assert_eq!(CompilerTaskStatus::SCHEDULED.0, 1);
        assert_eq!(CompilerTaskStatus::COMPILING.0, 2);
        assert_eq!(CompilerTaskStatus::FINISHED.0, 3);
    }

    #[test]
    fn test_binary_function_options_values() {
        assert_eq!(BinaryFunctionOptions::NONE.0, 0);
        assert_eq!(BinaryFunctionOptions::PIPELINE_INDEPENDENT.0, 2);
    }

    #[test]
    fn test_pipeline_data_set_serializer_configuration_values() {
        assert_eq!(
            PipelineDataSetSerializerConfiguration::CAPTURE_DESCRIPTORS.0,
            1
        );
        assert_eq!(
            PipelineDataSetSerializerConfiguration::CAPTURE_BINARIES.0,
            2
        );

        let combined = PipelineDataSetSerializerConfiguration::CAPTURE_DESCRIPTORS
            | PipelineDataSetSerializerConfiguration::CAPTURE_BINARIES;
        assert_eq!(combined.0, 3);
    }

    #[test]
    fn test_visibility_options_values() {
        assert_eq!(VisibilityOptions::NONE.0, 0);
        assert_eq!(VisibilityOptions::DEVICE.0, 1);
        assert_eq!(VisibilityOptions::RESOURCE_ALIAS.0, 2);

        let combined = VisibilityOptions::DEVICE | VisibilityOptions::RESOURCE_ALIAS;
        assert_eq!(combined.0, 3);
        assert!(combined.contains(VisibilityOptions::DEVICE));
        assert!(combined.contains(VisibilityOptions::RESOURCE_ALIAS));
    }

    #[test]
    fn test_render_encoder_options_values() {
        assert_eq!(RenderEncoderOptions::NONE.0, 0);
        assert_eq!(RenderEncoderOptions::SUSPENDING.0, 1);
        assert_eq!(RenderEncoderOptions::RESUMING.0, 2);

        let combined = RenderEncoderOptions::SUSPENDING | RenderEncoderOptions::RESUMING;
        assert_eq!(combined.0, 3);
        assert!(combined.contains(RenderEncoderOptions::SUSPENDING));
        assert!(combined.contains(RenderEncoderOptions::RESUMING));
    }
}
