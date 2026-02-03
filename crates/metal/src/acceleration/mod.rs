//! Metal acceleration structures for ray tracing.
//!
//! This module contains types for building and managing acceleration structures
//! used in ray tracing operations.
//!
//! Corresponds to `Metal/MTLAccelerationStructure.hpp` and related headers.

mod curve;
mod descriptors;
mod encoder;
mod geometry;
mod instance;
mod motion_geometry;
mod motion_keyframe;
mod pass;
mod structure;

// Re-export all public types
pub use curve::AccelerationStructureCurveGeometryDescriptor;
pub use descriptors::{AccelerationStructureDescriptor, PrimitiveAccelerationStructureDescriptor};
pub use encoder::AccelerationStructureCommandEncoder;
pub use geometry::{
    AccelerationStructureBoundingBoxGeometryDescriptor, AccelerationStructureGeometryDescriptor,
    AccelerationStructureTriangleGeometryDescriptor,
};
pub use instance::{
    IndirectInstanceAccelerationStructureDescriptor, InstanceAccelerationStructureDescriptor,
};
pub use motion_geometry::{
    AccelerationStructureMotionBoundingBoxGeometryDescriptor,
    AccelerationStructureMotionCurveGeometryDescriptor,
    AccelerationStructureMotionTriangleGeometryDescriptor,
};
pub use motion_keyframe::MotionKeyframeData;
pub use pass::{
    AccelerationStructurePassDescriptor, AccelerationStructurePassSampleBufferAttachmentDescriptor,
    AccelerationStructurePassSampleBufferAttachmentDescriptorArray,
};
pub use structure::{AccelerationStructure, AccelerationStructureSizes};
