//! Motion geometry descriptors for acceleration structures.
//!
//! Contains motion versions of triangle, bounding box, and curve geometry descriptors.

mod bounding_box;
mod curve;
mod triangle;

pub use bounding_box::AccelerationStructureMotionBoundingBoxGeometryDescriptor;
pub use curve::AccelerationStructureMotionCurveGeometryDescriptor;
pub use triangle::AccelerationStructureMotionTriangleGeometryDescriptor;
