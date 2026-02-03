//! Geometry descriptors for acceleration structures.
//!
//! Contains `AccelerationStructureGeometryDescriptor`,
//! `AccelerationStructureTriangleGeometryDescriptor`, and
//! `AccelerationStructureBoundingBoxGeometryDescriptor`.

mod base;
mod bounding_box;
mod triangle;

pub use base::AccelerationStructureGeometryDescriptor;
pub use bounding_box::AccelerationStructureBoundingBoxGeometryDescriptor;
pub use triangle::AccelerationStructureTriangleGeometryDescriptor;
