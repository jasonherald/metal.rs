//! Metal types for Rust.
//!
//! Corresponds to `Metal/MTLTypes.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace MTL {
//! struct Origin { NS::UInteger x, y, z; };
//! struct Size { NS::UInteger width, height, depth; };
//! struct Region { MTL::Origin origin; MTL::Size size; };
//! struct SamplePosition { float x, y; };
//! struct ResourceID { uint64_t _impl; };
//! using Coordinate2D = MTL::SamplePosition;
//! }
//! ```

use metal_foundation::UInteger;

/// 3D origin coordinates.
///
/// C++ equivalent: `MTL::Origin`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Origin {
    pub x: UInteger,
    pub y: UInteger,
    pub z: UInteger,
}

impl Origin {
    /// Create a new Origin.
    #[inline]
    pub const fn new(x: UInteger, y: UInteger, z: UInteger) -> Self {
        Self { x, y, z }
    }

    /// Create a new Origin (C++ style factory method).
    ///
    /// C++ equivalent: `static Origin Make(NS::UInteger x, NS::UInteger y, NS::UInteger z)`
    #[inline]
    pub const fn make(x: UInteger, y: UInteger, z: UInteger) -> Self {
        Self::new(x, y, z)
    }
}

/// 3D size dimensions.
///
/// C++ equivalent: `MTL::Size`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Size {
    pub width: UInteger,
    pub height: UInteger,
    pub depth: UInteger,
}

impl Size {
    /// Create a new Size.
    #[inline]
    pub const fn new(width: UInteger, height: UInteger, depth: UInteger) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }

    /// Create a new Size (C++ style factory method).
    ///
    /// C++ equivalent: `static Size Make(NS::UInteger width, NS::UInteger height, NS::UInteger depth)`
    #[inline]
    pub const fn make(width: UInteger, height: UInteger, depth: UInteger) -> Self {
        Self::new(width, height, depth)
    }
}

/// 3D rectangular region (origin + size).
///
/// C++ equivalent: `MTL::Region`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Region {
    pub origin: Origin,
    pub size: Size,
}

impl Region {
    /// Create a new Region from origin and size.
    #[inline]
    pub const fn new(origin: Origin, size: Size) -> Self {
        Self { origin, size }
    }

    /// Create a 1D region.
    ///
    /// C++ equivalent: `Region(NS::UInteger x, NS::UInteger width)`
    #[inline]
    pub const fn new_1d(x: UInteger, width: UInteger) -> Self {
        Self {
            origin: Origin::new(x, 0, 0),
            size: Size::new(width, 1, 1),
        }
    }

    /// Create a 2D region.
    ///
    /// C++ equivalent: `Region(NS::UInteger x, NS::UInteger y, NS::UInteger width, NS::UInteger height)`
    #[inline]
    pub const fn new_2d(x: UInteger, y: UInteger, width: UInteger, height: UInteger) -> Self {
        Self {
            origin: Origin::new(x, y, 0),
            size: Size::new(width, height, 1),
        }
    }

    /// Create a 3D region.
    ///
    /// C++ equivalent: `Region(NS::UInteger x, NS::UInteger y, NS::UInteger z, NS::UInteger width, NS::UInteger height, NS::UInteger depth)`
    #[inline]
    pub const fn new_3d(
        x: UInteger,
        y: UInteger,
        z: UInteger,
        width: UInteger,
        height: UInteger,
        depth: UInteger,
    ) -> Self {
        Self {
            origin: Origin::new(x, y, z),
            size: Size::new(width, height, depth),
        }
    }

    /// Create a 1D region (C++ style factory method).
    ///
    /// C++ equivalent: `static Region Make1D(NS::UInteger x, NS::UInteger width)`
    #[inline]
    pub const fn make_1d(x: UInteger, width: UInteger) -> Self {
        Self::new_1d(x, width)
    }

    /// Create a 2D region (C++ style factory method).
    ///
    /// C++ equivalent: `static Region Make2D(NS::UInteger x, NS::UInteger y, NS::UInteger width, NS::UInteger height)`
    #[inline]
    pub const fn make_2d(x: UInteger, y: UInteger, width: UInteger, height: UInteger) -> Self {
        Self::new_2d(x, y, width, height)
    }

    /// Create a 3D region (C++ style factory method).
    ///
    /// C++ equivalent: `static Region Make3D(NS::UInteger x, NS::UInteger y, NS::UInteger z, NS::UInteger width, NS::UInteger height, NS::UInteger depth)`
    #[inline]
    pub const fn make_3d(
        x: UInteger,
        y: UInteger,
        z: UInteger,
        width: UInteger,
        height: UInteger,
        depth: UInteger,
    ) -> Self {
        Self::new_3d(x, y, z, width, height, depth)
    }
}

/// 2D sample position.
///
/// C++ equivalent: `MTL::SamplePosition`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct SamplePosition {
    pub x: f32,
    pub y: f32,
}

impl SamplePosition {
    /// Create a new SamplePosition.
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Create a new SamplePosition (C++ style factory method).
    ///
    /// C++ equivalent: `static SamplePosition Make(float x, float y)`
    #[inline]
    pub const fn make(x: f32, y: f32) -> Self {
        Self::new(x, y)
    }
}

/// 2D coordinate (alias for SamplePosition).
///
/// C++ equivalent: `using Coordinate2D = MTL::SamplePosition`
pub type Coordinate2D = SamplePosition;

/// GPU resource identifier.
///
/// C++ equivalent: `MTL::ResourceID`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ResourceID {
    pub _impl: u64,
}

impl ResourceID {
    /// Create a new ResourceID.
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self { _impl: value }
    }
}

/// Scissor rectangle for rendering.
///
/// C++ equivalent: `MTL::ScissorRect` (from MTLRenderCommandEncoder.hpp)
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ScissorRect {
    pub x: UInteger,
    pub y: UInteger,
    pub width: UInteger,
    pub height: UInteger,
}

impl ScissorRect {
    /// Create a new ScissorRect.
    #[inline]
    pub const fn new(x: UInteger, y: UInteger, width: UInteger, height: UInteger) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// Rendering viewport.
///
/// C++ equivalent: `MTL::Viewport` (from MTLRenderCommandEncoder.hpp)
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Viewport {
    pub origin_x: f64,
    pub origin_y: f64,
    pub width: f64,
    pub height: f64,
    pub znear: f64,
    pub zfar: f64,
}

impl Viewport {
    /// Create a new Viewport.
    #[inline]
    pub const fn new(
        origin_x: f64,
        origin_y: f64,
        width: f64,
        height: f64,
        znear: f64,
        zfar: f64,
    ) -> Self {
        Self {
            origin_x,
            origin_y,
            width,
            height,
            znear,
            zfar,
        }
    }
}

/// Clear color for render passes.
///
/// C++ equivalent: `MTL::ClearColor` (from MTLRenderPass.hpp)
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ClearColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl ClearColor {
    /// Create a new ClearColor.
    #[inline]
    pub const fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Create a new ClearColor (C++ style factory method).
    ///
    /// C++ equivalent: `static ClearColor Make(double red, double green, double blue, double alpha)`
    #[inline]
    pub const fn make(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Self::new(red, green, blue, alpha)
    }
}

/// Indirect arguments for drawing primitives.
///
/// C++ equivalent: `MTL::DrawPrimitivesIndirectArguments`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DrawPrimitivesIndirectArguments {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub vertex_start: u32,
    pub base_instance: u32,
}

/// Indirect arguments for drawing indexed primitives.
///
/// C++ equivalent: `MTL::DrawIndexedPrimitivesIndirectArguments`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DrawIndexedPrimitivesIndirectArguments {
    pub index_count: u32,
    pub instance_count: u32,
    pub index_start: u32,
    pub base_vertex: i32,
    pub base_instance: u32,
}

/// Vertex amplification view mapping.
///
/// C++ equivalent: `MTL::VertexAmplificationViewMapping`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VertexAmplificationViewMapping {
    pub viewport_array_index_offset: u32,
    pub render_target_array_index_offset: u32,
}

/// Indirect arguments for drawing patches.
///
/// C++ equivalent: `MTL::DrawPatchIndirectArguments`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DrawPatchIndirectArguments {
    pub patch_count: u32,
    pub instance_count: u32,
    pub patch_start: u32,
    pub base_instance: u32,
}

/// Quad tessellation factors (half precision).
///
/// C++ equivalent: `MTL::QuadTessellationFactorsHalf`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct QuadTessellationFactorsHalf {
    pub edge_tessellation_factor: [u16; 4],
    pub inside_tessellation_factor: [u16; 2],
}

/// Triangle tessellation factors (half precision).
///
/// C++ equivalent: `MTL::TriangleTessellationFactorsHalf`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TriangleTessellationFactorsHalf {
    pub edge_tessellation_factor: [u16; 3],
    pub inside_tessellation_factor: u16,
}

// ============================================================================
// Acceleration Structure Types (from MTLAccelerationStructureTypes.hpp)
// ============================================================================

use crate::enums::{AccelerationStructureInstanceOptions, MotionBorderMode};

/// Packed 3-component float vector for acceleration structures.
///
/// C++ equivalent: `MTL::PackedFloat3`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PackedFloat3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PackedFloat3 {
    /// Create a new PackedFloat3.
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Create from an array.
    #[inline]
    pub const fn from_array(arr: [f32; 3]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }

    /// Convert to an array.
    #[inline]
    pub const fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl PackedFloat3 {
    /// Get element at index (0=x, 1=y, 2=z).
    ///
    /// Returns `None` if index is out of bounds.
    ///
    /// Note: Cannot implement Index trait for packed structs due to alignment.
    #[inline]
    pub fn get(&self, idx: usize) -> Option<f32> {
        match idx {
            0 => Some(self.x),
            1 => Some(self.y),
            2 => Some(self.z),
            _ => None,
        }
    }
}

/// Packed 4x3 matrix for acceleration structure transforms.
///
/// This is a column-major 4x3 matrix (4 columns, 3 rows).
///
/// C++ equivalent: `MTL::PackedFloat4x3`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PackedFloat4x3 {
    pub columns: [PackedFloat3; 4],
}

impl PackedFloat4x3 {
    /// Create a new PackedFloat4x3.
    #[inline]
    pub const fn new(
        col0: PackedFloat3,
        col1: PackedFloat3,
        col2: PackedFloat3,
        col3: PackedFloat3,
    ) -> Self {
        Self {
            columns: [col0, col1, col2, col3],
        }
    }

    /// Create an identity-like transform (3x3 identity + zero translation).
    #[inline]
    pub const fn identity() -> Self {
        Self {
            columns: [
                PackedFloat3::new(1.0, 0.0, 0.0),
                PackedFloat3::new(0.0, 1.0, 0.0),
                PackedFloat3::new(0.0, 0.0, 1.0),
                PackedFloat3::new(0.0, 0.0, 0.0),
            ],
        }
    }
}

impl std::ops::Index<usize> for PackedFloat4x3 {
    type Output = PackedFloat3;
    #[inline]
    fn index(&self, idx: usize) -> &Self::Output {
        &self.columns[idx]
    }
}

impl std::ops::IndexMut<usize> for PackedFloat4x3 {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.columns[idx]
    }
}

/// Axis-aligned bounding box.
///
/// C++ equivalent: `MTL::AxisAlignedBoundingBox`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct AxisAlignedBoundingBox {
    pub min: PackedFloat3,
    pub max: PackedFloat3,
}

impl AxisAlignedBoundingBox {
    /// Create a new bounding box.
    #[inline]
    pub const fn new(min: PackedFloat3, max: PackedFloat3) -> Self {
        Self { min, max }
    }

    /// Create a bounding box containing a single point.
    #[inline]
    pub const fn from_point(p: PackedFloat3) -> Self {
        Self { min: p, max: p }
    }
}

/// Packed quaternion for rotations.
///
/// C++ equivalent: `MTL::PackedFloatQuaternion`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PackedFloatQuaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl PackedFloatQuaternion {
    /// Create a new quaternion.
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Create an identity quaternion (no rotation).
    #[inline]
    pub const fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    /// Create from an array.
    #[inline]
    pub const fn from_array(arr: [f32; 4]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
            w: arr[3],
        }
    }

    /// Convert to an array.
    #[inline]
    pub const fn to_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl PackedFloatQuaternion {
    /// Get element at index (0=x, 1=y, 2=z, 3=w).
    ///
    /// Returns `None` if index is out of bounds.
    ///
    /// Note: Cannot implement Index trait for packed structs due to alignment.
    #[inline]
    pub fn get(&self, idx: usize) -> Option<f32> {
        match idx {
            0 => Some(self.x),
            1 => Some(self.y),
            2 => Some(self.z),
            3 => Some(self.w),
            _ => None,
        }
    }
}

/// Component-based transform for acceleration structures.
///
/// C++ equivalent: `MTL::ComponentTransform`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ComponentTransform {
    pub scale: PackedFloat3,
    pub shear: PackedFloat3,
    pub pivot: PackedFloat3,
    pub rotation: PackedFloatQuaternion,
    pub translation: PackedFloat3,
}

impl ComponentTransform {
    /// Create an identity transform.
    #[inline]
    pub const fn identity() -> Self {
        Self {
            scale: PackedFloat3::new(1.0, 1.0, 1.0),
            shear: PackedFloat3::new(0.0, 0.0, 0.0),
            pivot: PackedFloat3::new(0.0, 0.0, 0.0),
            rotation: PackedFloatQuaternion::identity(),
            translation: PackedFloat3::new(0.0, 0.0, 0.0),
        }
    }
}

// ============================================================================
// Device Types (from MTLDevice.hpp)
// ============================================================================

/// Acceleration structure sizes returned by the device.
///
/// C++ equivalent: `MTL::AccelerationStructureSizes`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AccelerationStructureSizes {
    pub acceleration_structure_size: UInteger,
    pub build_scratch_buffer_size: UInteger,
    pub refit_scratch_buffer_size: UInteger,
}

/// Size and alignment for resource allocation.
///
/// C++ equivalent: `MTL::SizeAndAlign`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SizeAndAlign {
    pub size: UInteger,
    pub align: UInteger,
}

// ============================================================================
// Acceleration Structure Instance Descriptors (from MTLAccelerationStructure.hpp)
// ============================================================================

/// Default instance descriptor for acceleration structures.
///
/// C++ equivalent: `MTL::AccelerationStructureInstanceDescriptor`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AccelerationStructureInstanceDescriptor {
    pub transformation_matrix: PackedFloat4x3,
    pub options: AccelerationStructureInstanceOptions,
    pub mask: u32,
    pub intersection_function_table_offset: u32,
    pub acceleration_structure_index: u32,
}

impl Default for AccelerationStructureInstanceDescriptor {
    fn default() -> Self {
        Self {
            transformation_matrix: PackedFloat4x3::identity(),
            options: AccelerationStructureInstanceOptions::NONE,
            mask: 0xFFFFFFFF,
            intersection_function_table_offset: 0,
            acceleration_structure_index: 0,
        }
    }
}

/// Instance descriptor with user ID for acceleration structures.
///
/// C++ equivalent: `MTL::AccelerationStructureUserIDInstanceDescriptor`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AccelerationStructureUserIDInstanceDescriptor {
    pub transformation_matrix: PackedFloat4x3,
    pub options: AccelerationStructureInstanceOptions,
    pub mask: u32,
    pub intersection_function_table_offset: u32,
    pub acceleration_structure_index: u32,
    pub user_id: u32,
}

impl Default for AccelerationStructureUserIDInstanceDescriptor {
    fn default() -> Self {
        Self {
            transformation_matrix: PackedFloat4x3::identity(),
            options: AccelerationStructureInstanceOptions::NONE,
            mask: 0xFFFFFFFF,
            intersection_function_table_offset: 0,
            acceleration_structure_index: 0,
            user_id: 0,
        }
    }
}

/// Motion instance descriptor for acceleration structures.
///
/// C++ equivalent: `MTL::AccelerationStructureMotionInstanceDescriptor`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AccelerationStructureMotionInstanceDescriptor {
    pub options: AccelerationStructureInstanceOptions,
    pub mask: u32,
    pub intersection_function_table_offset: u32,
    pub acceleration_structure_index: u32,
    pub user_id: u32,
    pub motion_transforms_start_index: u32,
    pub motion_transforms_count: u32,
    pub motion_start_border_mode: MotionBorderMode,
    pub motion_end_border_mode: MotionBorderMode,
    pub motion_start_time: f32,
    pub motion_end_time: f32,
}

impl Default for AccelerationStructureMotionInstanceDescriptor {
    fn default() -> Self {
        Self {
            options: AccelerationStructureInstanceOptions::NONE,
            mask: 0xFFFFFFFF,
            intersection_function_table_offset: 0,
            acceleration_structure_index: 0,
            user_id: 0,
            motion_transforms_start_index: 0,
            motion_transforms_count: 0,
            motion_start_border_mode: MotionBorderMode::CLAMP,
            motion_end_border_mode: MotionBorderMode::CLAMP,
            motion_start_time: 0.0,
            motion_end_time: 1.0,
        }
    }
}

/// Indirect instance descriptor for acceleration structures.
///
/// C++ equivalent: `MTL::IndirectAccelerationStructureInstanceDescriptor`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IndirectAccelerationStructureInstanceDescriptor {
    pub transformation_matrix: PackedFloat4x3,
    pub options: AccelerationStructureInstanceOptions,
    pub mask: u32,
    pub intersection_function_table_offset: u32,
    pub user_id: u32,
    pub acceleration_structure_id: ResourceID,
}

impl Default for IndirectAccelerationStructureInstanceDescriptor {
    fn default() -> Self {
        Self {
            transformation_matrix: PackedFloat4x3::identity(),
            options: AccelerationStructureInstanceOptions::NONE,
            mask: 0xFFFFFFFF,
            intersection_function_table_offset: 0,
            user_id: 0,
            acceleration_structure_id: ResourceID::default(),
        }
    }
}

/// Indirect motion instance descriptor for acceleration structures.
///
/// C++ equivalent: `MTL::IndirectAccelerationStructureMotionInstanceDescriptor`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IndirectAccelerationStructureMotionInstanceDescriptor {
    pub options: AccelerationStructureInstanceOptions,
    pub mask: u32,
    pub intersection_function_table_offset: u32,
    pub user_id: u32,
    pub acceleration_structure_id: ResourceID,
    pub motion_transforms_start_index: u32,
    pub motion_transforms_count: u32,
    pub motion_start_border_mode: MotionBorderMode,
    pub motion_end_border_mode: MotionBorderMode,
    pub motion_start_time: f32,
    pub motion_end_time: f32,
}

impl Default for IndirectAccelerationStructureMotionInstanceDescriptor {
    fn default() -> Self {
        Self {
            options: AccelerationStructureInstanceOptions::NONE,
            mask: 0xFFFFFFFF,
            intersection_function_table_offset: 0,
            user_id: 0,
            acceleration_structure_id: ResourceID::default(),
            motion_transforms_start_index: 0,
            motion_transforms_count: 0,
            motion_start_border_mode: MotionBorderMode::CLAMP,
            motion_end_border_mode: MotionBorderMode::CLAMP,
            motion_start_time: 0.0,
            motion_end_time: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin_size() {
        assert_eq!(
            std::mem::size_of::<Origin>(),
            3 * std::mem::size_of::<UInteger>()
        );
    }

    #[test]
    fn test_size_size() {
        assert_eq!(
            std::mem::size_of::<Size>(),
            3 * std::mem::size_of::<UInteger>()
        );
    }

    #[test]
    fn test_region_size() {
        assert_eq!(
            std::mem::size_of::<Region>(),
            6 * std::mem::size_of::<UInteger>()
        );
    }

    #[test]
    fn test_sample_position_size() {
        assert_eq!(std::mem::size_of::<SamplePosition>(), 2 * 4);
    }

    #[test]
    fn test_resource_id_size() {
        assert_eq!(std::mem::size_of::<ResourceID>(), 8);
    }

    #[test]
    fn test_scissor_rect_size() {
        assert_eq!(
            std::mem::size_of::<ScissorRect>(),
            4 * std::mem::size_of::<UInteger>()
        );
    }

    #[test]
    fn test_viewport_size() {
        assert_eq!(std::mem::size_of::<Viewport>(), 6 * 8);
    }

    #[test]
    fn test_clear_color_size() {
        assert_eq!(std::mem::size_of::<ClearColor>(), 4 * 8);
    }

    #[test]
    fn test_origin_make() {
        let origin = Origin::make(1, 2, 3);
        let x = { origin.x };
        let y = { origin.y };
        let z = { origin.z };
        assert_eq!(x, 1);
        assert_eq!(y, 2);
        assert_eq!(z, 3);
    }

    #[test]
    fn test_region_make_2d() {
        let region = Region::make_2d(10, 20, 100, 200);
        let ox = { region.origin.x };
        let oy = { region.origin.y };
        let oz = { region.origin.z };
        let sw = { region.size.width };
        let sh = { region.size.height };
        let sd = { region.size.depth };
        assert_eq!(ox, 10);
        assert_eq!(oy, 20);
        assert_eq!(oz, 0);
        assert_eq!(sw, 100);
        assert_eq!(sh, 200);
        assert_eq!(sd, 1);
    }

    // Acceleration Structure Types Tests

    #[test]
    fn test_packed_float3_size() {
        assert_eq!(std::mem::size_of::<PackedFloat3>(), 12); // 3 * 4 bytes
    }

    #[test]
    fn test_packed_float4x3_size() {
        assert_eq!(std::mem::size_of::<PackedFloat4x3>(), 48); // 4 * 12 bytes
    }

    #[test]
    fn test_axis_aligned_bounding_box_size() {
        assert_eq!(std::mem::size_of::<AxisAlignedBoundingBox>(), 24); // 2 * 12 bytes
    }

    #[test]
    fn test_packed_float_quaternion_size() {
        assert_eq!(std::mem::size_of::<PackedFloatQuaternion>(), 16); // 4 * 4 bytes
    }

    #[test]
    fn test_component_transform_size() {
        // 3*PackedFloat3 + 1*PackedFloatQuaternion + 1*PackedFloat3 = 4*12 + 16 = 64
        assert_eq!(std::mem::size_of::<ComponentTransform>(), 64);
    }

    #[test]
    fn test_acceleration_structure_sizes_size() {
        assert_eq!(
            std::mem::size_of::<AccelerationStructureSizes>(),
            3 * std::mem::size_of::<UInteger>()
        );
    }

    #[test]
    fn test_size_and_align_size() {
        assert_eq!(
            std::mem::size_of::<SizeAndAlign>(),
            2 * std::mem::size_of::<UInteger>()
        );
    }

    #[test]
    fn test_acceleration_structure_instance_descriptor_size() {
        // PackedFloat4x3 (48) + options (4) + mask (4) + offset (4) + index (4) = 64
        assert_eq!(
            std::mem::size_of::<AccelerationStructureInstanceDescriptor>(),
            64
        );
    }

    #[test]
    fn test_acceleration_structure_user_id_instance_descriptor_size() {
        // PackedFloat4x3 (48) + options (4) + mask (4) + offset (4) + index (4) + user_id (4) = 68
        assert_eq!(
            std::mem::size_of::<AccelerationStructureUserIDInstanceDescriptor>(),
            68
        );
    }

    #[test]
    fn test_packed_float3_new() {
        let v = PackedFloat3::new(1.0, 2.0, 3.0);
        let x = { v.x };
        let y = { v.y };
        let z = { v.z };
        assert_eq!(x, 1.0);
        assert_eq!(y, 2.0);
        assert_eq!(z, 3.0);
    }

    #[test]
    fn test_packed_float4x3_identity() {
        let m = PackedFloat4x3::identity();
        // Check first column
        let c0x = { m.columns[0].x };
        let c0y = { m.columns[0].y };
        let c0z = { m.columns[0].z };
        assert_eq!(c0x, 1.0);
        assert_eq!(c0y, 0.0);
        assert_eq!(c0z, 0.0);
        // Check translation column (last column should be zero)
        let c3x = { m.columns[3].x };
        let c3y = { m.columns[3].y };
        let c3z = { m.columns[3].z };
        assert_eq!(c3x, 0.0);
        assert_eq!(c3y, 0.0);
        assert_eq!(c3z, 0.0);
    }

    #[test]
    fn test_packed_float_quaternion_identity() {
        let q = PackedFloatQuaternion::identity();
        let x = { q.x };
        let y = { q.y };
        let z = { q.z };
        let w = { q.w };
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(z, 0.0);
        assert_eq!(w, 1.0);
    }
}
