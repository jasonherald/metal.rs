//! Command buffer enumerations.
//!
//! Corresponds to `Metal/MTLCommandBuffer.hpp`.

use metal_foundation::{Integer, UInteger};

/// Command buffer status.
///
/// C++ equivalent: `MTL::CommandBufferStatus`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CommandBufferStatus(pub UInteger);

impl CommandBufferStatus {
    pub const NOT_ENQUEUED: Self = Self(0);
    pub const ENQUEUED: Self = Self(1);
    pub const COMMITTED: Self = Self(2);
    pub const SCHEDULED: Self = Self(3);
    pub const COMPLETED: Self = Self(4);
    pub const ERROR: Self = Self(5);
}

/// Command buffer error codes.
///
/// C++ equivalent: `MTL::CommandBufferError`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CommandBufferError(pub UInteger);

impl CommandBufferError {
    pub const NONE: Self = Self(0);
    pub const INTERNAL: Self = Self(1);
    pub const TIMEOUT: Self = Self(2);
    pub const PAGE_FAULT: Self = Self(3);
    pub const BLACKLISTED: Self = Self(4);
    pub const ACCESS_REVOKED: Self = Self(4);
    pub const NOT_PERMITTED: Self = Self(7);
    pub const OUT_OF_MEMORY: Self = Self(8);
    pub const INVALID_RESOURCE: Self = Self(9);
    pub const MEMORYLESS: Self = Self(10);
    pub const DEVICE_REMOVED: Self = Self(11);
    pub const STACK_OVERFLOW: Self = Self(12);
}

/// Command encoder error state.
///
/// C++ equivalent: `MTL::CommandEncoderErrorState`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CommandEncoderErrorState(pub Integer);

impl CommandEncoderErrorState {
    pub const UNKNOWN: Self = Self(0);
    pub const COMPLETED: Self = Self(1);
    pub const AFFECTED: Self = Self(2);
    pub const PENDING: Self = Self(3);
    pub const FAULTED: Self = Self(4);
}

/// Dispatch type for compute commands.
///
/// C++ equivalent: `MTL::DispatchType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DispatchType(pub UInteger);

impl DispatchType {
    pub const SERIAL: Self = Self(0);
    pub const CONCURRENT: Self = Self(1);
}

/// Command buffer error options (bitflags).
///
/// C++ equivalent: `MTL::CommandBufferErrorOption`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CommandBufferErrorOption(pub UInteger);

impl CommandBufferErrorOption {
    pub const NONE: Self = Self(0);
    pub const ENCODER_EXECUTION_STATUS: Self = Self(1);

    /// Returns the raw bits.
    #[inline]
    pub const fn bits(&self) -> UInteger {
        self.0
    }

    /// Creates from raw bits.
    #[inline]
    pub const fn from_bits(bits: UInteger) -> Self {
        Self(bits)
    }
}

impl std::ops::BitOr for CommandBufferErrorOption {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// Resource usage for command encoders (bitflags).
///
/// C++ equivalent: `MTL::ResourceUsage`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ResourceUsage(pub UInteger);

impl ResourceUsage {
    pub const READ: Self = Self(1);
    pub const WRITE: Self = Self(1 << 1);
    pub const SAMPLE: Self = Self(1 << 2);

    /// Returns the raw bits.
    #[inline]
    pub const fn bits(&self) -> UInteger {
        self.0
    }

    /// Creates from raw bits.
    #[inline]
    pub const fn from_bits(bits: UInteger) -> Self {
        Self(bits)
    }

    /// Check if empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Check if contains all flags in other.
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for ResourceUsage {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for ResourceUsage {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for ResourceUsage {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Barrier scope for command encoders (bitflags).
///
/// C++ equivalent: `MTL::BarrierScope`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BarrierScope(pub UInteger);

impl BarrierScope {
    pub const BUFFERS: Self = Self(1);
    pub const TEXTURES: Self = Self(1 << 1);
    pub const RENDER_TARGETS: Self = Self(1 << 2);

    /// Returns the raw bits.
    #[inline]
    pub const fn bits(&self) -> UInteger {
        self.0
    }

    /// Creates from raw bits.
    #[inline]
    pub const fn from_bits(bits: UInteger) -> Self {
        Self(bits)
    }

    /// Check if empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Check if contains all flags in other.
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for BarrierScope {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for BarrierScope {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for BarrierScope {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Pipeline stages for barrier operations (bitflags).
///
/// C++ equivalent: `MTL::Stages`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Stages(pub UInteger);

impl Stages {
    pub const VERTEX: Self = Self(1);
    pub const FRAGMENT: Self = Self(1 << 1);
    pub const TILE: Self = Self(1 << 2);
    pub const OBJECT: Self = Self(1 << 3);
    pub const MESH: Self = Self(1 << 4);
    pub const RESOURCE_STATE: Self = Self(1 << 26);
    pub const DISPATCH: Self = Self(1 << 27);
    pub const BLIT: Self = Self(1 << 28);
    pub const ACCELERATION_STRUCTURE: Self = Self(1 << 29);
    pub const MACHINE_LEARNING: Self = Self(1 << 30);
    pub const ALL: Self = Self(9223372036854775807);

    /// Returns the raw bits.
    #[inline]
    pub const fn bits(&self) -> UInteger {
        self.0
    }

    /// Creates from raw bits.
    #[inline]
    pub const fn from_bits(bits: UInteger) -> Self {
        Self(bits)
    }

    /// Check if empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Check if contains all flags in other.
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for Stages {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for Stages {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for Stages {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_buffer_status_values() {
        assert_eq!(CommandBufferStatus::NOT_ENQUEUED.0, 0);
        assert_eq!(CommandBufferStatus::COMPLETED.0, 4);
        assert_eq!(CommandBufferStatus::ERROR.0, 5);
    }

    #[test]
    fn test_command_buffer_error_values() {
        assert_eq!(CommandBufferError::NONE.0, 0);
        assert_eq!(CommandBufferError::TIMEOUT.0, 2);
        assert_eq!(
            CommandBufferError::BLACKLISTED.0,
            CommandBufferError::ACCESS_REVOKED.0
        );
    }

    #[test]
    fn test_resource_usage_bitor() {
        let usage = ResourceUsage::READ | ResourceUsage::WRITE;
        assert!(usage.contains(ResourceUsage::READ));
        assert!(usage.contains(ResourceUsage::WRITE));
        assert!(!usage.contains(ResourceUsage::SAMPLE));
    }

    #[test]
    fn test_barrier_scope_values() {
        assert_eq!(BarrierScope::BUFFERS.0, 1);
        assert_eq!(BarrierScope::TEXTURES.0, 2);
        assert_eq!(BarrierScope::RENDER_TARGETS.0, 4);
    }

    #[test]
    fn test_stages_values() {
        assert_eq!(Stages::VERTEX.0, 1);
        assert_eq!(Stages::FRAGMENT.0, 2);
        assert_eq!(Stages::BLIT.0, 1 << 28);
    }
}
