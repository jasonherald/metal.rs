//! MTL4 Acceleration Structure implementation.
//!
//! Corresponds to `Metal/MTL4AccelerationStructure.hpp`.

mod bounding_box_geometry;
mod curve_geometry;
mod descriptor;
mod geometry_descriptor;
mod indirect_instance_descriptor;
mod instance_descriptor;
mod motion_bounding_box_geometry;
mod motion_curve_geometry;
mod motion_triangle_geometry;
mod primitive_descriptor;
mod triangle_geometry;

pub use bounding_box_geometry::AccelerationStructureBoundingBoxGeometryDescriptor;
pub use curve_geometry::AccelerationStructureCurveGeometryDescriptor;
pub use descriptor::AccelerationStructureDescriptor;
pub use geometry_descriptor::AccelerationStructureGeometryDescriptor;
pub use indirect_instance_descriptor::IndirectInstanceAccelerationStructureDescriptor;
pub use instance_descriptor::InstanceAccelerationStructureDescriptor;
pub use motion_bounding_box_geometry::AccelerationStructureMotionBoundingBoxGeometryDescriptor;
pub use motion_curve_geometry::AccelerationStructureMotionCurveGeometryDescriptor;
pub use motion_triangle_geometry::AccelerationStructureMotionTriangleGeometryDescriptor;
pub use primitive_descriptor::PrimitiveAccelerationStructureDescriptor;
pub use triangle_geometry::AccelerationStructureTriangleGeometryDescriptor;

// ============================================================
// BufferRange
// ============================================================

/// A buffer range with address and length.
///
/// C++ equivalent: `MTL4::BufferRange`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default)]
pub struct BufferRange {
    /// The GPU address of the buffer.
    pub buffer_address: u64,
    /// The length of the buffer range in bytes.
    pub length: u64,
}

impl BufferRange {
    /// Create a new buffer range.
    pub fn new(buffer_address: u64, length: u64) -> Self {
        Self {
            buffer_address,
            length,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;

    #[test]
    fn test_buffer_range_size() {
        assert_eq!(std::mem::size_of::<BufferRange>(), 16);
    }

    #[test]
    fn test_acceleration_structure_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<AccelerationStructureDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_acceleration_structure_geometry_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<AccelerationStructureGeometryDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_primitive_acceleration_structure_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<PrimitiveAccelerationStructureDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_triangle_geometry_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<AccelerationStructureTriangleGeometryDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_bounding_box_geometry_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<AccelerationStructureBoundingBoxGeometryDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_curve_geometry_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<AccelerationStructureCurveGeometryDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_instance_acceleration_structure_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<InstanceAccelerationStructureDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_indirect_instance_acceleration_structure_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<IndirectInstanceAccelerationStructureDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
