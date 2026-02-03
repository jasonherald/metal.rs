//! Render enumerations.
//!
//! Corresponds to `Metal/MTLRenderCommandEncoder.hpp` and `Metal/MTLRenderPass.hpp`.

use metal_foundation::{Integer, UInteger};

/// Primitive type for rendering.
///
/// C++ equivalent: `MTL::PrimitiveType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PrimitiveType(pub UInteger);

impl PrimitiveType {
    pub const POINT: Self = Self(0);
    pub const LINE: Self = Self(1);
    pub const LINE_STRIP: Self = Self(2);
    pub const TRIANGLE: Self = Self(3);
    pub const TRIANGLE_STRIP: Self = Self(4);
}

/// Visibility result mode.
///
/// C++ equivalent: `MTL::VisibilityResultMode`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VisibilityResultMode(pub UInteger);

impl VisibilityResultMode {
    pub const DISABLED: Self = Self(0);
    pub const BOOLEAN: Self = Self(1);
    pub const COUNTING: Self = Self(2);
}

/// Cull mode for rendering.
///
/// C++ equivalent: `MTL::CullMode`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CullMode(pub UInteger);

impl CullMode {
    pub const NONE: Self = Self(0);
    pub const FRONT: Self = Self(1);
    pub const BACK: Self = Self(2);
}

/// Winding order for front-facing triangles.
///
/// C++ equivalent: `MTL::Winding`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Winding(pub UInteger);

impl Winding {
    pub const CLOCKWISE: Self = Self(0);
    pub const COUNTER_CLOCKWISE: Self = Self(1);
}

/// Depth clip mode.
///
/// C++ equivalent: `MTL::DepthClipMode`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DepthClipMode(pub UInteger);

impl DepthClipMode {
    pub const CLIP: Self = Self(0);
    pub const CLAMP: Self = Self(1);
}

/// Triangle fill mode.
///
/// C++ equivalent: `MTL::TriangleFillMode`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TriangleFillMode(pub UInteger);

impl TriangleFillMode {
    pub const FILL: Self = Self(0);
    pub const LINES: Self = Self(1);
}

/// Render stages (bitflags).
///
/// C++ equivalent: `MTL::RenderStages`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct RenderStages(pub UInteger);

impl RenderStages {
    pub const VERTEX: Self = Self(1);
    pub const FRAGMENT: Self = Self(1 << 1);
    pub const TILE: Self = Self(1 << 2);
    pub const OBJECT: Self = Self(1 << 3);
    pub const MESH: Self = Self(1 << 4);

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

impl std::ops::BitOr for RenderStages {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for RenderStages {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for RenderStages {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Load action for render pass attachments.
///
/// C++ equivalent: `MTL::LoadAction`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct LoadAction(pub UInteger);

impl LoadAction {
    pub const DONT_CARE: Self = Self(0);
    pub const LOAD: Self = Self(1);
    pub const CLEAR: Self = Self(2);
}

/// Store action for render pass attachments.
///
/// C++ equivalent: `MTL::StoreAction`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StoreAction(pub UInteger);

impl StoreAction {
    pub const DONT_CARE: Self = Self(0);
    pub const STORE: Self = Self(1);
    pub const MULTISAMPLE_RESOLVE: Self = Self(2);
    pub const STORE_AND_MULTISAMPLE_RESOLVE: Self = Self(3);
    pub const UNKNOWN: Self = Self(4);
    pub const CUSTOM_SAMPLE_DEPTH_STORE: Self = Self(5);
}

/// Store action options (bitflags).
///
/// C++ equivalent: `MTL::StoreActionOptions`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StoreActionOptions(pub UInteger);

impl StoreActionOptions {
    pub const NONE: Self = Self(0);
    pub const CUSTOM_SAMPLE_POSITIONS: Self = Self(1);
    pub const VALID_MASK: Self = Self(1);

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

impl std::ops::BitOr for StoreActionOptions {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// Visibility result type for render passes.
///
/// C++ equivalent: `MTL::VisibilityResultType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VisibilityResultType(pub Integer);

impl VisibilityResultType {
    pub const RESET: Self = Self(0);
    pub const ACCUMULATE: Self = Self(1);
}

/// Multisample depth resolve filter.
///
/// C++ equivalent: `MTL::MultisampleDepthResolveFilter`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct MultisampleDepthResolveFilter(pub UInteger);

impl MultisampleDepthResolveFilter {
    pub const SAMPLE0: Self = Self(0);
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(2);
}

/// Multisample stencil resolve filter.
///
/// C++ equivalent: `MTL::MultisampleStencilResolveFilter`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct MultisampleStencilResolveFilter(pub UInteger);

impl MultisampleStencilResolveFilter {
    pub const SAMPLE0: Self = Self(0);
    pub const DEPTH_RESOLVED_SAMPLE: Self = Self(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_type_values() {
        assert_eq!(PrimitiveType::POINT.0, 0);
        assert_eq!(PrimitiveType::TRIANGLE.0, 3);
    }

    #[test]
    fn test_cull_mode_values() {
        assert_eq!(CullMode::NONE.0, 0);
        assert_eq!(CullMode::FRONT.0, 1);
        assert_eq!(CullMode::BACK.0, 2);
    }

    #[test]
    fn test_render_stages_bitor() {
        let stages = RenderStages::VERTEX | RenderStages::FRAGMENT;
        assert!(stages.contains(RenderStages::VERTEX));
        assert!(stages.contains(RenderStages::FRAGMENT));
    }
}
