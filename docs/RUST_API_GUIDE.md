# Rust Metal API Guide

Comprehensive Rust API documentation with C++ equivalents for reference.

## Overview

This crate provides safe Rust bindings to Apple's Metal graphics API.
Each type mirrors its C++ metal-cpp counterpart with idiomatic Rust patterns.

## Module Structure

```
metal/
├── device/        # MTL::Device - GPU device management
├── command/       # Command queues, buffers, encoders
├── resource/      # Buffers, textures, heaps
├── pipeline/      # Render and compute pipelines
├── shader/        # Libraries, functions, arguments
├── enums/         # All Metal enumerations
└── mtl4/          # Metal 4 features (macOS 26+)
```

---

## metal

### `AccelerationStructure`

C++ equivalent: `NS::AccelerationStructure`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `gpu_resource_id` | `(&self) → u64` | `gpuResourceID` |
| `label` | `(&self) → Option<String>` | — |
| `size` | `(&self) → UInteger` | `size` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | — |

---

### `AccelerationStructureBoundingBoxGeometryDescriptor`

C++ equivalent: `NS::AccelerationStructureBoundingBoxGeometryDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allow_duplicate_intersection_function_invocation` | `(&self) → bool` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `bounding_box_buffer` | `(&self) → Option<Buffer>` | `boundingBoxBuffer` |
| `bounding_box_buffer_offset` | `(&self) → UInteger` | — |
| `bounding_box_count` | `(&self) → UInteger` | `boundingBoxCount` |
| `bounding_box_stride` | `(&self) → UInteger` | `boundingBoxStride` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `intersection_function_table_offset` | `(&self) → UInteger` | — |
| `label` | `(&self) → Option<String>` | — |
| `opaque` | `(&self) → bool` | — |
| `primitive_data_buffer` | `(&self) → Option<Buffer>` | — |
| `primitive_data_buffer_offset` | `(&self) → UInteger` | — |
| `primitive_data_element_size` | `(&self) → UInteger` | — |
| `primitive_data_stride` | `(&self) → UInteger` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_allow_duplicate_intersection_function_invocation` | `(&self, allow: bool) → void` | — |
| `set_bounding_box_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setBoundingBoxBuffer` |
| `set_bounding_box_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_bounding_box_count` | `(&self, count: UInteger) → void` | `setBoundingBoxCount` |
| `set_bounding_box_stride` | `(&self, stride: UInteger) → void` | `setBoundingBoxStride` |
| `set_intersection_function_table_offset` | `(&self, offset: UInteger) → void` | — |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_opaque` | `(&self, opaque: bool) → void` | — |
| `set_primitive_data_buffer` | `(&self, buffer: Option<&Buffer>) → void` | — |
| `set_primitive_data_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_primitive_data_element_size` | `(&self, size: UInteger) → void` | — |
| `set_primitive_data_stride` | `(&self, stride: UInteger) → void` | — |

---

### `AccelerationStructureBoundingBoxGeometryDescriptor`

C++ equivalent: `NS::AccelerationStructureBoundingBoxGeometryDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allow_duplicate_intersection_function_invocation` | `(&self) → bool` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `bounding_box_buffer` | `(&self) → Option<Buffer>` | `boundingBoxBuffer` |
| `bounding_box_buffer_offset` | `(&self) → UInteger` | `boundingBoxBufferOffset` |
| `bounding_box_count` | `(&self) → UInteger` | `boundingBoxCount` |
| `bounding_box_stride` | `(&self) → UInteger` | `boundingBoxStride` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `intersection_function_table_offset` | `(&self) → UInteger` | — |
| `label` | `(&self) → Option<String>` | — |
| `opaque` | `(&self) → bool` | — |
| `primitive_data_buffer` | `(&self) → Option<Buffer>` | — |
| `primitive_data_buffer_offset` | `(&self) → UInteger` | — |
| `primitive_data_element_size` | `(&self) → UInteger` | — |
| `primitive_data_stride` | `(&self) → UInteger` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_allow_duplicate_intersection_function_invocation` | `(&self, allow: bool) → void` | — |
| `set_bounding_box_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setBoundingBoxBuffer` |
| `set_bounding_box_buffer_offset` | `(&self, offset: UInteger) → void` | `setBoundingBoxBufferOffset` |
| `set_bounding_box_count` | `(&self, count: UInteger) → void` | `setBoundingBoxCount` |
| `set_bounding_box_stride` | `(&self, stride: UInteger) → void` | `setBoundingBoxStride` |
| `set_intersection_function_table_offset` | `(&self, offset: UInteger) → void` | — |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_opaque` | `(&self, opaque: bool) → void` | — |
| `set_primitive_data_buffer` | `(&self, buffer: Option<&Buffer>) → void` | — |
| `set_primitive_data_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_primitive_data_element_size` | `(&self, size: UInteger) → void` | — |
| `set_primitive_data_stride` | `(&self, stride: UInteger) → void` | — |

---

### `AccelerationStructureCommandEncoder`

C++ equivalent: `NS::AccelerationStructureCommandEncoder`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `build_acceleration_structure` | `(&self,
        acceleration...) → void` | `buildAccelerationStructure` |
| `copy_acceleration_structure` | `(&self,
        source: &Acc...) → void` | `copyAccelerationStructure` |
| `copy_and_compact_acceleration_structure` | `(&self,
        source: &Acc...) → void` | `copyAndCompactAccelerationStructure` |
| `end_encoding` | `(&self) → void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `insert_debug_signpost` | `(&self, name: &str) → void` | — |
| `label` | `(&self) → Option<String>` | — |
| `pop_debug_group` | `(&self) → void` | — |
| `push_debug_group` | `(&self, name: &str) → void` | — |
| `refit_acceleration_structure` | `(&self,
        source: &Acc...) → void` | `refitAccelerationStructure` |
| `refit_acceleration_structure_with_options` | `(&self,
        source: &Acc...) → void` | — |
| `sample_counters_in_buffer_ptr` | `(&self,
        sample_buffe...) → void` | `sampleCountersInBuffer` |
| `update_fence` | `(&self, fence: &Fence) → void` | `updateFence` |
| `use_heap` | `(&self, heap: &Heap) → void` | `useHeap` |
| `use_heaps_ptr` | `(&self, heaps: *const *const...) → void` | — |
| `use_resource` | `(&self, resource: &R, usage:...) → void` | `useResource` |
| `use_resources_ptr` | `(&self,
        resources: *...) → void` | — |
| `wait_for_fence` | `(&self, fence: &Fence) → void` | `waitForFence` |
| `write_compacted_acceleration_structure_size` | `(&self,
        acceleration...) → void` | `writeCompactedAccelerationStructureSize` |
| `write_compacted_acceleration_structure_size_with_type` | `(&self,
        acceleration...) → void` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | — |

---

### `AccelerationStructureCurveGeometryDescriptor`

C++ equivalent: `NS::AccelerationStructureCurveGeometryDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allow_duplicate_intersection_function_invocation` | `(&self) → bool` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `control_point_buffer` | `(&self) → Option<Buffer>` | `controlPointBuffer` |
| `control_point_buffer_offset` | `(&self) → UInteger` | — |
| `control_point_count` | `(&self) → UInteger` | `controlPointCount` |
| `control_point_format` | `(&self) → AttributeFormat` | `controlPointFormat` |
| `control_point_stride` | `(&self) → UInteger` | `controlPointStride` |
| `curve_basis` | `(&self) → CurveBasis` | `curveBasis` |
| `curve_end_caps` | `(&self) → CurveEndCaps` | `curveEndCaps` |
| `curve_type` | `(&self) → CurveType` | `curveType` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index_buffer` | `(&self) → Option<Buffer>` | `indexBuffer` |
| `index_buffer_offset` | `(&self) → UInteger` | — |
| `index_type` | `(&self) → IndexType` | `indexType` |
| `intersection_function_table_offset` | `(&self) → UInteger` | — |
| `label` | `(&self) → Option<String>` | — |
| `opaque` | `(&self) → bool` | — |
| `primitive_data_buffer` | `(&self) → Option<Buffer>` | — |
| `primitive_data_buffer_offset` | `(&self) → UInteger` | — |
| `primitive_data_element_size` | `(&self) → UInteger` | — |
| `primitive_data_stride` | `(&self) → UInteger` | — |
| `radius_buffer` | `(&self) → Option<Buffer>` | `radiusBuffer` |
| `radius_buffer_offset` | `(&self) → UInteger` | — |
| `radius_format` | `(&self) → AttributeFormat` | `radiusFormat` |
| `radius_stride` | `(&self) → UInteger` | `radiusStride` |
| `segment_control_point_count` | `(&self) → UInteger` | `segmentControlPointCount` |
| `segment_count` | `(&self) → UInteger` | `segmentCount` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_allow_duplicate_intersection_function_invocation` | `(&self, allow: bool) → void` | — |
| `set_control_point_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setControlPointBuffer` |
| `set_control_point_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_control_point_count` | `(&self, count: UInteger) → void` | `setControlPointCount` |
| `set_control_point_format` | `(&self, format: AttributeFormat) → void` | `setControlPointFormat` |
| `set_control_point_stride` | `(&self, stride: UInteger) → void` | `setControlPointStride` |
| `set_curve_basis` | `(&self, basis: CurveBasis) → void` | `setCurveBasis` |
| `set_curve_end_caps` | `(&self, end_caps: CurveEndCaps) → void` | `setCurveEndCaps` |
| `set_curve_type` | `(&self, curve_type: CurveType) → void` | `setCurveType` |
| `set_index_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setIndexBuffer` |
| `set_index_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_index_type` | `(&self, index_type: IndexType) → void` | `setIndexType` |
| `set_intersection_function_table_offset` | `(&self, offset: UInteger) → void` | — |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_opaque` | `(&self, opaque: bool) → void` | — |
| `set_primitive_data_buffer` | `(&self, buffer: Option<&Buffer>) → void` | — |
| `set_primitive_data_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_primitive_data_element_size` | `(&self, size: UInteger) → void` | — |
| `set_primitive_data_stride` | `(&self, stride: UInteger) → void` | — |
| `set_radius_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setRadiusBuffer` |
| `set_radius_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_radius_format` | `(&self, format: AttributeFormat) → void` | `setRadiusFormat` |
| `set_radius_stride` | `(&self, stride: UInteger) → void` | `setRadiusStride` |
| `set_segment_control_point_count` | `(&self, count: UInteger) → void` | `setSegmentControlPointCount` |
| `set_segment_count` | `(&self, count: UInteger) → void` | `setSegmentCount` |

---

### `AccelerationStructureCurveGeometryDescriptor`

C++ equivalent: `NS::AccelerationStructureCurveGeometryDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allow_duplicate_intersection_function_invocation` | `(&self) → bool` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `control_point_buffer` | `(&self) → Option<Buffer>` | `controlPointBuffer` |
| `control_point_buffer_offset` | `(&self) → UInteger` | `controlPointBufferOffset` |
| `control_point_count` | `(&self) → UInteger` | `controlPointCount` |
| `control_point_format` | `(&self) → AttributeFormat` | `controlPointFormat` |
| `control_point_stride` | `(&self) → UInteger` | `controlPointStride` |
| `curve_basis` | `(&self) → CurveBasis` | `curveBasis` |
| `curve_end_caps` | `(&self) → CurveEndCaps` | `curveEndCaps` |
| `curve_type` | `(&self) → CurveType` | `curveType` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index_buffer` | `(&self) → Option<Buffer>` | `indexBuffer` |
| `index_buffer_offset` | `(&self) → UInteger` | `indexBufferOffset` |
| `index_type` | `(&self) → IndexType` | `indexType` |
| `intersection_function_table_offset` | `(&self) → UInteger` | — |
| `label` | `(&self) → Option<String>` | — |
| `opaque` | `(&self) → bool` | — |
| `primitive_data_buffer` | `(&self) → Option<Buffer>` | — |
| `primitive_data_buffer_offset` | `(&self) → UInteger` | — |
| `primitive_data_element_size` | `(&self) → UInteger` | — |
| `primitive_data_stride` | `(&self) → UInteger` | — |
| `radius_buffer` | `(&self) → Option<Buffer>` | `radiusBuffer` |
| `radius_buffer_offset` | `(&self) → UInteger` | `radiusBufferOffset` |
| `radius_format` | `(&self) → AttributeFormat` | `radiusFormat` |
| `radius_stride` | `(&self) → UInteger` | `radiusStride` |
| `segment_control_point_count` | `(&self) → UInteger` | `segmentControlPointCount` |
| `segment_count` | `(&self) → UInteger` | `segmentCount` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_allow_duplicate_intersection_function_invocation` | `(&self, allow: bool) → void` | — |
| `set_control_point_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setControlPointBuffer` |
| `set_control_point_buffer_offset` | `(&self, offset: UInteger) → void` | `setControlPointBufferOffset` |
| `set_control_point_count` | `(&self, count: UInteger) → void` | `setControlPointCount` |
| `set_control_point_format` | `(&self, format: AttributeFormat) → void` | `setControlPointFormat` |
| `set_control_point_stride` | `(&self, stride: UInteger) → void` | `setControlPointStride` |
| `set_curve_basis` | `(&self, basis: CurveBasis) → void` | `setCurveBasis` |
| `set_curve_end_caps` | `(&self, end_caps: CurveEndCaps) → void` | `setCurveEndCaps` |
| `set_curve_type` | `(&self, curve_type: CurveType) → void` | `setCurveType` |
| `set_index_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setIndexBuffer` |
| `set_index_buffer_offset` | `(&self, offset: UInteger) → void` | `setIndexBufferOffset` |
| `set_index_type` | `(&self, index_type: IndexType) → void` | `setIndexType` |
| `set_intersection_function_table_offset` | `(&self, offset: UInteger) → void` | — |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_opaque` | `(&self, opaque: bool) → void` | — |
| `set_primitive_data_buffer` | `(&self, buffer: Option<&Buffer>) → void` | — |
| `set_primitive_data_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_primitive_data_element_size` | `(&self, size: UInteger) → void` | — |
| `set_primitive_data_stride` | `(&self, stride: UInteger) → void` | — |
| `set_radius_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setRadiusBuffer` |
| `set_radius_buffer_offset` | `(&self, offset: UInteger) → void` | `setRadiusBufferOffset` |
| `set_radius_format` | `(&self, format: AttributeFormat) → void` | `setRadiusFormat` |
| `set_radius_stride` | `(&self, stride: UInteger) → void` | `setRadiusStride` |
| `set_segment_control_point_count` | `(&self, count: UInteger) → void` | `setSegmentControlPointCount` |
| `set_segment_count` | `(&self, count: UInteger) → void` | `setSegmentCount` |

---

### `AccelerationStructureDescriptor`

C++ equivalent: `NS::AccelerationStructureDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `usage` | `(&self) → AccelerationStructureUsage` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_usage` | `(&self, usage: AccelerationS...) → void` | — |

---

### `AccelerationStructureDescriptor`

C++ equivalent: `NS::AccelerationStructureDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `usage` | `(&self) → AccelerationStructureUsage` | `usage` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_usage` | `(&self, usage: AccelerationS...) → void` | `setUsage` |

---

### `AccelerationStructureGeometryDescriptor`

C++ equivalent: `NS::AccelerationStructureGeometryDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allow_duplicate_intersection_function_invocation` | `(&self) → bool` | `allowDuplicateIntersectionFunctionInvocation` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `intersection_function_table_offset` | `(&self) → UInteger` | `intersectionFunctionTableOffset` |
| `label` | `(&self) → Option<String>` | `label` |
| `opaque` | `(&self) → bool` | `opaque` |
| `primitive_data_buffer` | `(&self) → Option<Buffer>` | `primitiveDataBuffer` |
| `primitive_data_buffer_offset` | `(&self) → UInteger` | — |
| `primitive_data_element_size` | `(&self) → UInteger` | `primitiveDataElementSize` |
| `primitive_data_stride` | `(&self) → UInteger` | `primitiveDataStride` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_allow_duplicate_intersection_function_invocation` | `(&self, allow: bool) → void` | `setAllowDuplicateIntersectionFunctionInvocation` |
| `set_intersection_function_table_offset` | `(&self, offset: UInteger) → void` | `setIntersectionFunctionTableOffset` |
| `set_label` | `(&self, label: &str) → void` | `setLabel` |
| `set_opaque` | `(&self, opaque: bool) → void` | `setOpaque` |
| `set_primitive_data_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setPrimitiveDataBuffer` |
| `set_primitive_data_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_primitive_data_element_size` | `(&self, size: UInteger) → void` | `setPrimitiveDataElementSize` |
| `set_primitive_data_stride` | `(&self, stride: UInteger) → void` | `setPrimitiveDataStride` |

---

### `AccelerationStructureGeometryDescriptor`

C++ equivalent: `NS::AccelerationStructureGeometryDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allow_duplicate_intersection_function_invocation` | `(&self) → bool` | `allowDuplicateIntersectionFunctionInvocation` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `intersection_function_table_offset` | `(&self) → UInteger` | `intersectionFunctionTableOffset` |
| `label` | `(&self) → Option<String>` | `label` |
| `opaque` | `(&self) → bool` | `opaque` |
| `primitive_data_buffer` | `(&self) → Option<Buffer>` | `primitiveDataBuffer` |
| `primitive_data_buffer_offset` | `(&self) → UInteger` | `primitiveDataBufferOffset` |
| `primitive_data_element_size` | `(&self) → UInteger` | `primitiveDataElementSize` |
| `primitive_data_stride` | `(&self) → UInteger` | `primitiveDataStride` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_allow_duplicate_intersection_function_invocation` | `(&self, allow: bool) → void` | `setAllowDuplicateIntersectionFunctionInvocation` |
| `set_intersection_function_table_offset` | `(&self, offset: UInteger) → void` | `setIntersectionFunctionTableOffset` |
| `set_label` | `(&self, label: &str) → void` | `setLabel` |
| `set_opaque` | `(&self, opaque: bool) → void` | `setOpaque` |
| `set_primitive_data_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setPrimitiveDataBuffer` |
| `set_primitive_data_buffer_offset` | `(&self, offset: UInteger) → void` | `setPrimitiveDataBufferOffset` |
| `set_primitive_data_element_size` | `(&self, size: UInteger) → void` | `setPrimitiveDataElementSize` |
| `set_primitive_data_stride` | `(&self, stride: UInteger) → void` | `setPrimitiveDataStride` |

---

### `AccelerationStructureMotionBoundingBoxGeometryDescriptor`

C++ equivalent: `NS::AccelerationStructureMotionBoundingBoxGeometryDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allow_duplicate_intersection_function_invocation` | `(&self) → bool` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `bounding_box_buffers` | `(&self) → BufferRange` | `boundingBoxBuffers` |
| `bounding_box_buffers_ptr` | `(&self) → *const c_void` | — |
| `bounding_box_count` | `(&self) → UInteger` | `boundingBoxCount` |
| `bounding_box_stride` | `(&self) → UInteger` | `boundingBoxStride` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `intersection_function_table_offset` | `(&self) → UInteger` | — |
| `label` | `(&self) → Option<String>` | — |
| `opaque` | `(&self) → bool` | — |
| `primitive_data_buffer` | `(&self) → Option<Buffer>` | — |
| `primitive_data_buffer_offset` | `(&self) → UInteger` | — |
| `primitive_data_element_size` | `(&self) → UInteger` | — |
| `primitive_data_stride` | `(&self) → UInteger` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_allow_duplicate_intersection_function_invocation` | `(&self, allow: bool) → void` | — |
| `set_bounding_box_buffers` | `(&self, buffers: BufferRange) → void` | `setBoundingBoxBuffers` |
| `set_bounding_box_buffers_ptr` | `(&self, bounding_box_buffers...) → void` | — |
| `set_bounding_box_count` | `(&self, count: UInteger) → void` | `setBoundingBoxCount` |
| `set_bounding_box_stride` | `(&self, stride: UInteger) → void` | `setBoundingBoxStride` |
| `set_intersection_function_table_offset` | `(&self, offset: UInteger) → void` | — |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_opaque` | `(&self, opaque: bool) → void` | — |
| `set_primitive_data_buffer` | `(&self, buffer: Option<&Buffer>) → void` | — |
| `set_primitive_data_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_primitive_data_element_size` | `(&self, size: UInteger) → void` | — |
| `set_primitive_data_stride` | `(&self, stride: UInteger) → void` | — |

---

### `AccelerationStructureMotionBoundingBoxGeometryDescriptor`

C++ equivalent: `NS::AccelerationStructureMotionBoundingBoxGeometryDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allow_duplicate_intersection_function_invocation` | `(&self) → bool` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `bounding_box_buffers` | `(&self) → BufferRange` | `boundingBoxBuffers` |
| `bounding_box_buffers_ptr` | `(&self) → *const c_void` | — |
| `bounding_box_count` | `(&self) → UInteger` | `boundingBoxCount` |
| `bounding_box_stride` | `(&self) → UInteger` | `boundingBoxStride` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `intersection_function_table_offset` | `(&self) → UInteger` | — |
| `label` | `(&self) → Option<String>` | — |
| `opaque` | `(&self) → bool` | — |
| `primitive_data_buffer` | `(&self) → Option<Buffer>` | — |
| `primitive_data_buffer_offset` | `(&self) → UInteger` | — |
| `primitive_data_element_size` | `(&self) → UInteger` | — |
| `primitive_data_stride` | `(&self) → UInteger` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_allow_duplicate_intersection_function_invocation` | `(&self, allow: bool) → void` | — |
| `set_bounding_box_buffers` | `(&self, buffers: BufferRange) → void` | `setBoundingBoxBuffers` |
| `set_bounding_box_buffers_ptr` | `(&self, bounding_box_buffers...) → void` | — |
| `set_bounding_box_count` | `(&self, count: UInteger) → void` | `setBoundingBoxCount` |
| `set_bounding_box_stride` | `(&self, stride: UInteger) → void` | `setBoundingBoxStride` |
| `set_intersection_function_table_offset` | `(&self, offset: UInteger) → void` | — |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_opaque` | `(&self, opaque: bool) → void` | — |
| `set_primitive_data_buffer` | `(&self, buffer: Option<&Buffer>) → void` | — |
| `set_primitive_data_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_primitive_data_element_size` | `(&self, size: UInteger) → void` | — |
| `set_primitive_data_stride` | `(&self, stride: UInteger) → void` | — |

---

### `AccelerationStructureMotionCurveGeometryDescriptor`

C++ equivalent: `NS::AccelerationStructureMotionCurveGeometryDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allow_duplicate_intersection_function_invocation` | `(&self) → bool` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `control_point_buffers` | `(&self) → BufferRange` | `controlPointBuffers` |
| `control_point_buffers_ptr` | `(&self) → *const c_void` | — |
| `control_point_count` | `(&self) → UInteger` | `controlPointCount` |
| `control_point_format` | `(&self) → AttributeFormat` | `controlPointFormat` |
| `control_point_stride` | `(&self) → UInteger` | `controlPointStride` |
| `curve_basis` | `(&self) → CurveBasis` | `curveBasis` |
| `curve_end_caps` | `(&self) → CurveEndCaps` | `curveEndCaps` |
| `curve_type` | `(&self) → CurveType` | `curveType` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index_buffer` | `(&self) → Option<Buffer>` | `indexBuffer` |
| `index_buffer_offset` | `(&self) → UInteger` | — |
| `index_type` | `(&self) → IndexType` | `indexType` |
| `intersection_function_table_offset` | `(&self) → UInteger` | — |
| `label` | `(&self) → Option<String>` | — |
| `opaque` | `(&self) → bool` | — |
| `primitive_data_buffer` | `(&self) → Option<Buffer>` | — |
| `primitive_data_buffer_offset` | `(&self) → UInteger` | — |
| `primitive_data_element_size` | `(&self) → UInteger` | — |
| `primitive_data_stride` | `(&self) → UInteger` | — |
| `radius_buffers` | `(&self) → BufferRange` | `radiusBuffers` |
| `radius_buffers_ptr` | `(&self) → *const c_void` | — |
| `radius_format` | `(&self) → AttributeFormat` | `radiusFormat` |
| `radius_stride` | `(&self) → UInteger` | `radiusStride` |
| `segment_control_point_count` | `(&self) → UInteger` | `segmentControlPointCount` |
| `segment_count` | `(&self) → UInteger` | `segmentCount` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_allow_duplicate_intersection_function_invocation` | `(&self, allow: bool) → void` | — |
| `set_control_point_buffers` | `(&self, buffers: BufferRange) → void` | `setControlPointBuffers` |
| `set_control_point_buffers_ptr` | `(&self, control_point_buffer...) → void` | — |
| `set_control_point_count` | `(&self, count: UInteger) → void` | `setControlPointCount` |
| `set_control_point_format` | `(&self, format: AttributeFormat) → void` | `setControlPointFormat` |
| `set_control_point_stride` | `(&self, stride: UInteger) → void` | `setControlPointStride` |
| `set_curve_basis` | `(&self, basis: CurveBasis) → void` | `setCurveBasis` |
| `set_curve_end_caps` | `(&self, end_caps: CurveEndCaps) → void` | `setCurveEndCaps` |
| `set_curve_type` | `(&self, curve_type: CurveType) → void` | `setCurveType` |
| `set_index_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setIndexBuffer` |
| `set_index_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_index_type` | `(&self, index_type: IndexType) → void` | `setIndexType` |
| `set_intersection_function_table_offset` | `(&self, offset: UInteger) → void` | — |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_opaque` | `(&self, opaque: bool) → void` | — |
| `set_primitive_data_buffer` | `(&self, buffer: Option<&Buffer>) → void` | — |
| `set_primitive_data_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_primitive_data_element_size` | `(&self, size: UInteger) → void` | — |
| `set_primitive_data_stride` | `(&self, stride: UInteger) → void` | — |
| `set_radius_buffers` | `(&self, buffers: BufferRange) → void` | `setRadiusBuffers` |
| `set_radius_buffers_ptr` | `(&self, radius_buffers: *con...) → void` | — |
| `set_radius_format` | `(&self, format: AttributeFormat) → void` | `setRadiusFormat` |
| `set_radius_stride` | `(&self, stride: UInteger) → void` | `setRadiusStride` |
| `set_segment_control_point_count` | `(&self, count: UInteger) → void` | `setSegmentControlPointCount` |
| `set_segment_count` | `(&self, count: UInteger) → void` | `setSegmentCount` |

---

### `AccelerationStructureMotionCurveGeometryDescriptor`

C++ equivalent: `NS::AccelerationStructureMotionCurveGeometryDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allow_duplicate_intersection_function_invocation` | `(&self) → bool` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `control_point_buffers` | `(&self) → BufferRange` | `controlPointBuffers` |
| `control_point_buffers_ptr` | `(&self) → *const c_void` | — |
| `control_point_count` | `(&self) → UInteger` | `controlPointCount` |
| `control_point_format` | `(&self) → AttributeFormat` | `controlPointFormat` |
| `control_point_stride` | `(&self) → UInteger` | `controlPointStride` |
| `curve_basis` | `(&self) → CurveBasis` | `curveBasis` |
| `curve_end_caps` | `(&self) → CurveEndCaps` | `curveEndCaps` |
| `curve_type` | `(&self) → CurveType` | `curveType` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index_buffer` | `(&self) → Option<Buffer>` | `indexBuffer` |
| `index_buffer_offset` | `(&self) → UInteger` | `indexBufferOffset` |
| `index_type` | `(&self) → IndexType` | `indexType` |
| `intersection_function_table_offset` | `(&self) → UInteger` | — |
| `label` | `(&self) → Option<String>` | — |
| `opaque` | `(&self) → bool` | — |
| `primitive_data_buffer` | `(&self) → Option<Buffer>` | — |
| `primitive_data_buffer_offset` | `(&self) → UInteger` | — |
| `primitive_data_element_size` | `(&self) → UInteger` | — |
| `primitive_data_stride` | `(&self) → UInteger` | — |
| `radius_buffers` | `(&self) → BufferRange` | `radiusBuffers` |
| `radius_buffers_ptr` | `(&self) → *const c_void` | — |
| `radius_format` | `(&self) → AttributeFormat` | `radiusFormat` |
| `radius_stride` | `(&self) → UInteger` | `radiusStride` |
| `segment_control_point_count` | `(&self) → UInteger` | `segmentControlPointCount` |
| `segment_count` | `(&self) → UInteger` | `segmentCount` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_allow_duplicate_intersection_function_invocation` | `(&self, allow: bool) → void` | — |
| `set_control_point_buffers` | `(&self, buffers: BufferRange) → void` | `setControlPointBuffers` |
| `set_control_point_buffers_ptr` | `(&self, control_point_buffer...) → void` | — |
| `set_control_point_count` | `(&self, count: UInteger) → void` | `setControlPointCount` |
| `set_control_point_format` | `(&self, format: AttributeFormat) → void` | `setControlPointFormat` |
| `set_control_point_stride` | `(&self, stride: UInteger) → void` | `setControlPointStride` |
| `set_curve_basis` | `(&self, basis: CurveBasis) → void` | `setCurveBasis` |
| `set_curve_end_caps` | `(&self, end_caps: CurveEndCaps) → void` | `setCurveEndCaps` |
| `set_curve_type` | `(&self, curve_type: CurveType) → void` | `setCurveType` |
| `set_index_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setIndexBuffer` |
| `set_index_buffer_offset` | `(&self, offset: UInteger) → void` | `setIndexBufferOffset` |
| `set_index_type` | `(&self, index_type: IndexType) → void` | `setIndexType` |
| `set_intersection_function_table_offset` | `(&self, offset: UInteger) → void` | — |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_opaque` | `(&self, opaque: bool) → void` | — |
| `set_primitive_data_buffer` | `(&self, buffer: Option<&Buffer>) → void` | — |
| `set_primitive_data_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_primitive_data_element_size` | `(&self, size: UInteger) → void` | — |
| `set_primitive_data_stride` | `(&self, stride: UInteger) → void` | — |
| `set_radius_buffers` | `(&self, buffers: BufferRange) → void` | `setRadiusBuffers` |
| `set_radius_buffers_ptr` | `(&self, radius_buffers: *con...) → void` | — |
| `set_radius_format` | `(&self, format: AttributeFormat) → void` | `setRadiusFormat` |
| `set_radius_stride` | `(&self, stride: UInteger) → void` | `setRadiusStride` |
| `set_segment_control_point_count` | `(&self, count: UInteger) → void` | `setSegmentControlPointCount` |
| `set_segment_count` | `(&self, count: UInteger) → void` | `setSegmentCount` |

---

### `AccelerationStructureMotionTriangleGeometryDescriptor`

C++ equivalent: `NS::AccelerationStructureMotionTriangleGeometryDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allow_duplicate_intersection_function_invocation` | `(&self) → bool` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index_buffer` | `(&self) → Option<Buffer>` | `indexBuffer` |
| `index_buffer_offset` | `(&self) → UInteger` | — |
| `index_type` | `(&self) → IndexType` | `indexType` |
| `intersection_function_table_offset` | `(&self) → UInteger` | — |
| `label` | `(&self) → Option<String>` | — |
| `opaque` | `(&self) → bool` | — |
| `primitive_data_buffer` | `(&self) → Option<Buffer>` | — |
| `primitive_data_buffer_offset` | `(&self) → UInteger` | — |
| `primitive_data_element_size` | `(&self) → UInteger` | — |
| `primitive_data_stride` | `(&self) → UInteger` | — |
| `transformation_matrix_buffer` | `(&self) → Option<Buffer>` | `transformationMatrixBuffer` |
| `transformation_matrix_buffer_offset` | `(&self) → UInteger` | — |
| `transformation_matrix_layout` | `(&self) → MatrixLayout` | `transformationMatrixLayout` |
| `triangle_count` | `(&self) → UInteger` | `triangleCount` |
| `vertex_buffers` | `(&self) → BufferRange` | `vertexBuffers` |
| `vertex_buffers_ptr` | `(&self) → *const c_void` | — |
| `vertex_format` | `(&self) → AttributeFormat` | `vertexFormat` |
| `vertex_stride` | `(&self) → UInteger` | `vertexStride` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_allow_duplicate_intersection_function_invocation` | `(&self, allow: bool) → void` | — |
| `set_index_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setIndexBuffer` |
| `set_index_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_index_type` | `(&self, index_type: IndexType) → void` | `setIndexType` |
| `set_intersection_function_table_offset` | `(&self, offset: UInteger) → void` | — |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_opaque` | `(&self, opaque: bool) → void` | — |
| `set_primitive_data_buffer` | `(&self, buffer: Option<&Buffer>) → void` | — |
| `set_primitive_data_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_primitive_data_element_size` | `(&self, size: UInteger) → void` | — |
| `set_primitive_data_stride` | `(&self, stride: UInteger) → void` | — |
| `set_transformation_matrix_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setTransformationMatrixBuffer` |
| `set_transformation_matrix_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_transformation_matrix_layout` | `(&self, layout: MatrixLayout) → void` | `setTransformationMatrixLayout` |
| `set_triangle_count` | `(&self, count: UInteger) → void` | `setTriangleCount` |
| `set_vertex_buffers` | `(&self, buffers: BufferRange) → void` | `setVertexBuffers` |
| `set_vertex_buffers_ptr` | `(&self, vertex_buffers: *con...) → void` | — |
| `set_vertex_format` | `(&self, format: AttributeFormat) → void` | `setVertexFormat` |
| `set_vertex_stride` | `(&self, stride: UInteger) → void` | `setVertexStride` |

---

### `AccelerationStructureMotionTriangleGeometryDescriptor`

C++ equivalent: `NS::AccelerationStructureMotionTriangleGeometryDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allow_duplicate_intersection_function_invocation` | `(&self) → bool` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index_buffer` | `(&self) → Option<Buffer>` | `indexBuffer` |
| `index_buffer_offset` | `(&self) → UInteger` | `indexBufferOffset` |
| `index_type` | `(&self) → IndexType` | `indexType` |
| `intersection_function_table_offset` | `(&self) → UInteger` | — |
| `label` | `(&self) → Option<String>` | — |
| `opaque` | `(&self) → bool` | — |
| `primitive_data_buffer` | `(&self) → Option<Buffer>` | — |
| `primitive_data_buffer_offset` | `(&self) → UInteger` | — |
| `primitive_data_element_size` | `(&self) → UInteger` | — |
| `primitive_data_stride` | `(&self) → UInteger` | — |
| `transformation_matrix_buffer` | `(&self) → Option<Buffer>` | `transformationMatrixBuffer` |
| `transformation_matrix_buffer_offset` | `(&self) → UInteger` | `transformationMatrixBufferOffset` |
| `transformation_matrix_layout` | `(&self) → MatrixLayout` | `transformationMatrixLayout` |
| `triangle_count` | `(&self) → UInteger` | `triangleCount` |
| `vertex_buffers` | `(&self) → BufferRange` | `vertexBuffers` |
| `vertex_buffers_ptr` | `(&self) → *const c_void` | — |
| `vertex_format` | `(&self) → AttributeFormat` | `vertexFormat` |
| `vertex_stride` | `(&self) → UInteger` | `vertexStride` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_allow_duplicate_intersection_function_invocation` | `(&self, allow: bool) → void` | — |
| `set_index_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setIndexBuffer` |
| `set_index_buffer_offset` | `(&self, offset: UInteger) → void` | `setIndexBufferOffset` |
| `set_index_type` | `(&self, index_type: IndexType) → void` | `setIndexType` |
| `set_intersection_function_table_offset` | `(&self, offset: UInteger) → void` | — |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_opaque` | `(&self, opaque: bool) → void` | — |
| `set_primitive_data_buffer` | `(&self, buffer: Option<&Buffer>) → void` | — |
| `set_primitive_data_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_primitive_data_element_size` | `(&self, size: UInteger) → void` | — |
| `set_primitive_data_stride` | `(&self, stride: UInteger) → void` | — |
| `set_transformation_matrix_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setTransformationMatrixBuffer` |
| `set_transformation_matrix_buffer_offset` | `(&self, offset: UInteger) → void` | `setTransformationMatrixBufferOffset` |
| `set_transformation_matrix_layout` | `(&self, layout: MatrixLayout) → void` | `setTransformationMatrixLayout` |
| `set_triangle_count` | `(&self, count: UInteger) → void` | `setTriangleCount` |
| `set_vertex_buffers` | `(&self, buffers: BufferRange) → void` | `setVertexBuffers` |
| `set_vertex_buffers_ptr` | `(&self, vertex_buffers: *con...) → void` | — |
| `set_vertex_format` | `(&self, format: AttributeFormat) → void` | `setVertexFormat` |
| `set_vertex_stride` | `(&self, stride: UInteger) → void` | `setVertexStride` |

---

### `AccelerationStructurePassDescriptor`

C++ equivalent: `NS::AccelerationStructurePassDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `accelerationStructurePassDescriptor` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `descriptor` | `() → Option<Self>` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `sample_buffer_attachments` | `(&self,) → Option<AccelerationStructurePassSampleBufferAttachmentDescriptorArray>` | `sampleBufferAttachments` |

---

### `AccelerationStructurePassSampleBufferAttachmentDescriptor`

C++ equivalent: `NS::AccelerationStructurePassSampleBufferAttachmentDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `end_of_encoder_sample_index` | `(&self) → UInteger` | `endOfEncoderSampleIndex` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `sample_buffer_ptr` | `(&self) → *mut c_void` | `sampleBuffer` |
| `start_of_encoder_sample_index` | `(&self) → UInteger` | `startOfEncoderSampleIndex` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_end_of_encoder_sample_index` | `(&self, index: UInteger) → void` | `setEndOfEncoderSampleIndex` |
| `set_sample_buffer_ptr` | `(&self, sample_buffer: *cons...) → void` | `setSampleBuffer` |
| `set_start_of_encoder_sample_index` | `(&self, index: UInteger) → void` | `setStartOfEncoderSampleIndex` |

---

### `AccelerationStructurePassSampleBufferAttachmentDescriptorArray`

C++ equivalent: `NS::AccelerationStructurePassSampleBufferAttachmentDescriptorArray`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `object` | `(&self,
        index: UInte...) → Option<AccelerationStructurePassSampleBufferAttachmentDescriptor>` | `object` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_object` | `(&self,
        attachment: ...) → void` | `setObject` |

---

### `AccelerationStructureTriangleGeometryDescriptor`

C++ equivalent: `NS::AccelerationStructureTriangleGeometryDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allow_duplicate_intersection_function_invocation` | `(&self) → bool` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index_buffer` | `(&self) → Option<Buffer>` | `indexBuffer` |
| `index_buffer_offset` | `(&self) → UInteger` | — |
| `index_type` | `(&self) → IndexType` | `indexType` |
| `intersection_function_table_offset` | `(&self) → UInteger` | — |
| `label` | `(&self) → Option<String>` | — |
| `opaque` | `(&self) → bool` | — |
| `primitive_data_buffer` | `(&self) → Option<Buffer>` | — |
| `primitive_data_buffer_offset` | `(&self) → UInteger` | — |
| `primitive_data_element_size` | `(&self) → UInteger` | — |
| `primitive_data_stride` | `(&self) → UInteger` | — |
| `transformation_matrix_buffer` | `(&self) → Option<Buffer>` | `transformationMatrixBuffer` |
| `transformation_matrix_buffer_offset` | `(&self) → UInteger` | — |
| `transformation_matrix_layout` | `(&self) → MatrixLayout` | `transformationMatrixLayout` |
| `triangle_count` | `(&self) → UInteger` | `triangleCount` |
| `vertex_buffer` | `(&self) → Option<Buffer>` | `vertexBuffer` |
| `vertex_buffer_offset` | `(&self) → UInteger` | — |
| `vertex_format` | `(&self) → AttributeFormat` | `vertexFormat` |
| `vertex_stride` | `(&self) → UInteger` | `vertexStride` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_allow_duplicate_intersection_function_invocation` | `(&self, allow: bool) → void` | — |
| `set_index_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setIndexBuffer` |
| `set_index_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_index_type` | `(&self, index_type: IndexType) → void` | `setIndexType` |
| `set_intersection_function_table_offset` | `(&self, offset: UInteger) → void` | — |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_opaque` | `(&self, opaque: bool) → void` | — |
| `set_primitive_data_buffer` | `(&self, buffer: Option<&Buffer>) → void` | — |
| `set_primitive_data_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_primitive_data_element_size` | `(&self, size: UInteger) → void` | — |
| `set_primitive_data_stride` | `(&self, stride: UInteger) → void` | — |
| `set_transformation_matrix_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setTransformationMatrixBuffer` |
| `set_transformation_matrix_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_transformation_matrix_layout` | `(&self, layout: MatrixLayout) → void` | `setTransformationMatrixLayout` |
| `set_triangle_count` | `(&self, count: UInteger) → void` | `setTriangleCount` |
| `set_vertex_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setVertexBuffer` |
| `set_vertex_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_vertex_format` | `(&self, format: AttributeFormat) → void` | `setVertexFormat` |
| `set_vertex_stride` | `(&self, stride: UInteger) → void` | `setVertexStride` |

---

### `AccelerationStructureTriangleGeometryDescriptor`

C++ equivalent: `NS::AccelerationStructureTriangleGeometryDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allow_duplicate_intersection_function_invocation` | `(&self) → bool` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index_buffer` | `(&self) → Option<Buffer>` | `indexBuffer` |
| `index_buffer_offset` | `(&self) → UInteger` | `indexBufferOffset` |
| `index_type` | `(&self) → IndexType` | `indexType` |
| `intersection_function_table_offset` | `(&self) → UInteger` | — |
| `label` | `(&self) → Option<String>` | — |
| `opaque` | `(&self) → bool` | — |
| `primitive_data_buffer` | `(&self) → Option<Buffer>` | — |
| `primitive_data_buffer_offset` | `(&self) → UInteger` | — |
| `primitive_data_element_size` | `(&self) → UInteger` | — |
| `primitive_data_stride` | `(&self) → UInteger` | — |
| `transformation_matrix_buffer` | `(&self) → Option<Buffer>` | `transformationMatrixBuffer` |
| `transformation_matrix_buffer_offset` | `(&self) → UInteger` | `transformationMatrixBufferOffset` |
| `transformation_matrix_layout` | `(&self) → MatrixLayout` | `transformationMatrixLayout` |
| `triangle_count` | `(&self) → UInteger` | `triangleCount` |
| `vertex_buffer` | `(&self) → Option<Buffer>` | `vertexBuffer` |
| `vertex_buffer_offset` | `(&self) → UInteger` | `vertexBufferOffset` |
| `vertex_format` | `(&self) → AttributeFormat` | `vertexFormat` |
| `vertex_stride` | `(&self) → UInteger` | `vertexStride` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_allow_duplicate_intersection_function_invocation` | `(&self, allow: bool) → void` | — |
| `set_index_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setIndexBuffer` |
| `set_index_buffer_offset` | `(&self, offset: UInteger) → void` | `setIndexBufferOffset` |
| `set_index_type` | `(&self, index_type: IndexType) → void` | `setIndexType` |
| `set_intersection_function_table_offset` | `(&self, offset: UInteger) → void` | — |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_opaque` | `(&self, opaque: bool) → void` | — |
| `set_primitive_data_buffer` | `(&self, buffer: Option<&Buffer>) → void` | — |
| `set_primitive_data_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_primitive_data_element_size` | `(&self, size: UInteger) → void` | — |
| `set_primitive_data_stride` | `(&self, stride: UInteger) → void` | — |
| `set_transformation_matrix_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setTransformationMatrixBuffer` |
| `set_transformation_matrix_buffer_offset` | `(&self, offset: UInteger) → void` | `setTransformationMatrixBufferOffset` |
| `set_transformation_matrix_layout` | `(&self, layout: MatrixLayout) → void` | `setTransformationMatrixLayout` |
| `set_triangle_count` | `(&self, count: UInteger) → void` | `setTriangleCount` |
| `set_vertex_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setVertexBuffer` |
| `set_vertex_buffer_offset` | `(&self, offset: UInteger) → void` | `setVertexBufferOffset` |
| `set_vertex_format` | `(&self, format: AttributeFormat) → void` | `setVertexFormat` |
| `set_vertex_stride` | `(&self, stride: UInteger) → void` | `setVertexStride` |

---

### `Architecture`

C++ equivalent: `NS::Architecture`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | `alloc` |
| `init` | `(self) → Option<Self>` | `init` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `name` | `(&self) → &str` | `name` |

---

### `Archive`

C++ equivalent: `NS::Archive`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new_binary_function` | `(&self,
        descriptor: ...) → Result<BinaryFunction, metal_foundation::Error>` | `newBinaryFunction` |
| `new_compute_pipeline_state` | `(&self,
        descriptor: ...) → Result<ComputePipelineState, metal_foundation::Error>` | `newComputePipelineState` |
| `new_compute_pipeline_state_with_dynamic_linking` | `(&self,
        descriptor: ...) → Result<ComputePipelineState, metal_foundation::Error>` | — |
| `new_render_pipeline_state` | `(&self,
        descriptor: ...) → Result<RenderPipelineState, metal_foundation::Error>` | `newRenderPipelineState` |
| `new_render_pipeline_state_with_dynamic_linking` | `(&self,
        descriptor: ...) → Result<RenderPipelineState, metal_foundation::Error>` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |

---

### `Argument`

C++ equivalent: `NS::Argument`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `access` | `(&self) → BindingAccess` | `access` |
| `active` | `(&self) → bool` | — |
| `argument_type` | `(&self) → ArgumentType` | `type` |
| `array_length` | `(&self) → UInteger` | `arrayLength` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `buffer_alignment` | `(&self) → UInteger` | `bufferAlignment` |
| `buffer_data_size` | `(&self) → UInteger` | `bufferDataSize` |
| `buffer_data_type` | `(&self) → DataType` | `bufferDataType` |
| `buffer_pointer_type` | `(&self) → Option<PointerType>` | `bufferPointerType` |
| `buffer_struct_type` | `(&self) → Option<StructType>` | `bufferStructType` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index` | `(&self) → UInteger` | `index` |
| `name` | `(&self) → Option<String>` | `name` |
| `texture_data_type` | `(&self) → DataType` | `textureDataType` |
| `texture_type` | `(&self) → TextureType` | `textureType` |
| `threadgroup_memory_alignment` | `(&self) → UInteger` | `threadgroupMemoryAlignment` |
| `threadgroup_memory_data_size` | `(&self) → UInteger` | `threadgroupMemoryDataSize` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_active` | `(&self) → bool` | `active` |
| `is_depth_texture` | `(&self) → bool` | `isDepthTexture` |

---

### `ArgumentDescriptor`

C++ equivalent: `NS::ArgumentDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `access` | `(&self) → BindingAccess` | `access` |
| `array_length` | `(&self) → UInteger` | `arrayLength` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `constant_block_alignment` | `(&self) → UInteger` | `constantBlockAlignment` |
| `data_type` | `(&self) → DataType` | `dataType` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index` | `(&self) → UInteger` | `index` |
| `texture_type` | `(&self) → TextureType` | `textureType` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_access` | `(&self, access: BindingAccess) → void` | `setAccess` |
| `set_array_length` | `(&self, array_length: UInteger) → void` | `setArrayLength` |
| `set_constant_block_alignment` | `(&self, alignment: UInteger) → void` | `setConstantBlockAlignment` |
| `set_data_type` | `(&self, data_type: DataType) → void` | `setDataType` |
| `set_index` | `(&self, index: UInteger) → void` | `setIndex` |
| `set_texture_type` | `(&self, texture_type: Textur...) → void` | `setTextureType` |

---

### `ArgumentEncoder`

C++ equivalent: `NS::ArgumentEncoder`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new_argument_encoder` | `(&self, index: UInteger) → Option<ArgumentEncoder>` | `newArgumentEncoder` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `alignment` | `(&self) → UInteger` | `alignment` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `constant_data` | `(&self, index: UInteger) → *mut c_void` | `constantData` |
| `device` | `(&self) → crate::Device` | `device` |
| `encoded_length` | `(&self) → UInteger` | `encodedLength` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_acceleration_structure` | `(&self,
        acceleration...) → void` | `setAccelerationStructure` |
| `set_argument_buffer` | `(&self, buffer: &Buffer, off...) → void` | `setArgumentBuffer` |
| `set_argument_buffer_with_array_element` | `(&self,
        buffer: &Buf...) → void` | — |
| `set_buffer` | `(&self, buffer: &Buffer, off...) → void` | `setBuffer` |
| `set_compute_pipeline_state` | `(&self,
        pipeline: &c...) → void` | `setComputePipelineState` |
| `set_depth_stencil_state` | `(&self, state: &crate::Depth...) → void` | `setDepthStencilState` |
| `set_indirect_command_buffer_ptr` | `(&self, buffer: *const c_voi...) → void` | `setIndirectCommandBuffer` |
| `set_intersection_function_table_ptr` | `(&self, table: *const c_void...) → void` | `setIntersectionFunctionTable` |
| `set_label` | `(&self, label: &str) → void` | `setLabel` |
| `set_render_pipeline_state` | `(&self, pipeline: &crate::Re...) → void` | `setRenderPipelineState` |
| `set_sampler_state` | `(&self, sampler: &crate::Sam...) → void` | `setSamplerState` |
| `set_texture` | `(&self, texture: &crate::Tex...) → void` | `setTexture` |
| `set_visible_function_table_ptr` | `(&self, table: *const c_void...) → void` | `setVisibleFunctionTable` |

---

### `ArgumentTable`

C++ equivalent: `NS::ArgumentTable`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `device` | `(&self) → Option<Device>` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_address` | `(&self, gpu_address: u64, bi...) → void` | `setAddress` |
| `set_address_with_stride` | `(&self,
        gpu_address:...) → void` | — |
| `set_resource` | `(&self, resource_id: u64, bi...) → void` | `setResource` |
| `set_sampler_state` | `(&self, resource_id: u64, bi...) → void` | `setSamplerState` |
| `set_texture` | `(&self, resource_id: u64, bi...) → void` | `setTexture` |

---

### `ArgumentTableDescriptor`

C++ equivalent: `NS::ArgumentTableDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `initialize_bindings` | `(&self) → bool` | `initializeBindings` |
| `label` | `(&self) → Option<String>` | `label` |
| `max_buffer_bind_count` | `(&self) → UInteger` | `maxBufferBindCount` |
| `max_sampler_state_bind_count` | `(&self) → UInteger` | `maxSamplerStateBindCount` |
| `max_texture_bind_count` | `(&self) → UInteger` | `maxTextureBindCount` |
| `support_attribute_strides` | `(&self) → bool` | `supportAttributeStrides` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_initialize_bindings` | `(&self, initialize: bool) → void` | `setInitializeBindings` |
| `set_label` | `(&self, label: &str) → void` | `setLabel` |
| `set_max_buffer_bind_count` | `(&self, count: UInteger) → void` | `setMaxBufferBindCount` |
| `set_max_sampler_state_bind_count` | `(&self, count: UInteger) → void` | `setMaxSamplerStateBindCount` |
| `set_max_texture_bind_count` | `(&self, count: UInteger) → void` | `setMaxTextureBindCount` |
| `set_support_attribute_strides` | `(&self, support: bool) → void` | `setSupportAttributeStrides` |

---

### `ArrayType`

C++ equivalent: `NS::ArrayType`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `argument_index_stride` | `(&self) → UInteger` | `argumentIndexStride` |
| `array_length` | `(&self) → UInteger` | `arrayLength` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `data_type` | `(&self) → DataType` | — |
| `element_array_type` | `(&self) → Option<ArrayType>` | `elementArrayType` |
| `element_pointer_type` | `(&self) → Option<PointerType>` | `elementPointerType` |
| `element_struct_type` | `(&self) → Option<StructType>` | `elementStructType` |
| `element_tensor_reference_type` | `(&self) → Option<TensorReferenceType>` | `elementTensorReferenceType` |
| `element_texture_reference_type` | `(&self) → Option<TextureReferenceType>` | `elementTextureReferenceType` |
| `element_type` | `(&self) → DataType` | `elementType` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `stride` | `(&self) → UInteger` | `stride` |

---

### `Attribute`

C++ equivalent: `NS::Attribute`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `attribute_index` | `(&self) → UInteger` | `attributeIndex` |
| `attribute_type` | `(&self) → DataType` | `attributeType` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `name` | `(&self) → Option<String>` | `name` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_active` | `(&self) → bool` | `active` |
| `is_patch_control_point_data` | `(&self) → bool` | `isPatchControlPointData` |
| `is_patch_data` | `(&self) → bool` | `isPatchData` |

---

### `AttributeDescriptor`

C++ equivalent: `NS::AttributeDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `buffer_index` | `(&self) → UInteger` | `bufferIndex` |
| `format` | `(&self) → AttributeFormat` | `format` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `offset` | `(&self) → UInteger` | `offset` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_buffer_index` | `(&self, buffer_index: UInteger) → void` | `setBufferIndex` |
| `set_format` | `(&self, format: AttributeFormat) → void` | `setFormat` |
| `set_offset` | `(&self, offset: UInteger) → void` | `setOffset` |

---

### `AttributeDescriptorArray`

C++ equivalent: `NS::AttributeDescriptorArray`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `object_at` | `(&self, index: UInteger) → Option<AttributeDescriptor>` | `object` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_object_at` | `(&self, descriptor: &Attribu...) → void` | `setObject` |

---

### `BinaryArchive`

C++ equivalent: `NS::BinaryArchive`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `add_compute_pipeline_functions` | `(&self,
        descriptor: ...) → Result<(), metal_foundation::Error>` | `addComputePipelineFunctions` |
| `add_function` | `(&self,
        descriptor: ...) → Result<(), metal_foundation::Error>` | `addFunction` |
| `add_library_ptr` | `(&self, descriptor: *const c...) → Result<(), metal_foundation::Error>` | `addLibrary` |
| `add_mesh_render_pipeline_functions_ptr` | `(&self,
        descriptor: ...) → Result<(), metal_foundation::Error>` | `addMeshRenderPipelineFunctions` |
| `add_render_pipeline_functions` | `(&self,
        descriptor: ...) → Result<(), metal_foundation::Error>` | `addRenderPipelineFunctions` |
| `add_tile_render_pipeline_functions_ptr` | `(&self,
        descriptor: ...) → Result<(), metal_foundation::Error>` | `addTileRenderPipelineFunctions` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `device` | `(&self) → Device` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |
| `serialize_to_url` | `(&self, url: &metal_foundati...) → Result<(), metal_foundation::Error>` | `serializeToURL` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |

---

### `BinaryArchiveDescriptor`

C++ equivalent: `NS::BinaryArchiveDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `url` | `(&self) → Option<metal_foundation::Url>` | `url` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_url` | `(&self, url: &metal_foundati...) → void` | `setUrl` |

---

### `BinaryFunction`

C++ equivalent: `NS::BinaryFunction`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function_type` | `(&self) → FunctionType` | `functionType` |
| `name` | `(&self) → Option<String>` | `name` |

---

### `BinaryFunctionDescriptor`

C++ equivalent: `NS::BinaryFunctionDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function_descriptor` | `(&self) → Option<FunctionDescriptor>` | `functionDescriptor` |
| `name` | `(&self) → Option<String>` | `name` |
| `options` | `(&self) → BinaryFunctionOptions` | `options` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_function_descriptor` | `(&self, descriptor: &Functio...) → void` | `setFunctionDescriptor` |
| `set_name` | `(&self, name: &str) → void` | `setName` |
| `set_options` | `(&self, options: BinaryFunct...) → void` | `setOptions` |

---

### `Binding`

C++ equivalent: `NS::Binding`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `access` | `(&self) → BindingAccess` | `access` |
| `argument` | `(&self) → bool` | `argument` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `binding_type` | `(&self) → BindingType` | `type` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index` | `(&self) → UInteger` | `index` |
| `name` | `(&self) → Option<String>` | `name` |
| `used` | `(&self) → bool` | `used` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_argument` | `(&self) → bool` | `isArgument` |
| `is_used` | `(&self) → bool` | `isUsed` |

---

### `BlitCommandEncoder`

C++ equivalent: `NS::BlitCommandEncoder`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `barrier_after_queue_stages` | `(&self,
        after_stages...) → void` | — |
| `command_buffer` | `(&self) → crate::CommandBuffer` | — |
| `copy_from_buffer_to_buffer` | `(&self,
        source_buffe...) → void` | `copyFromBuffer` |
| `copy_from_buffer_to_texture` | `(&self,
        source_buffe...) → void` | `copyFromBuffer` |
| `copy_from_buffer_to_texture_with_options` | `(&self,
        source_buffe...) → void` | — |
| `copy_from_tensor_ptr` | `(&self,
        source_tenso...) → void` | `copyFromTensor` |
| `copy_from_texture_to_buffer` | `(&self,
        source_textu...) → void` | — |
| `copy_from_texture_to_buffer_with_options` | `(&self,
        source_textu...) → void` | — |
| `copy_from_texture_to_texture` | `(&self,
        source_textu...) → void` | `copyFromTexture` |
| `copy_from_texture_to_texture_region` | `(&self,
        source_textu...) → void` | — |
| `copy_from_texture_to_texture_slices` | `(&self,
        source_textu...) → void` | — |
| `copy_indirect_command_buffer_ptr` | `(&self,
        source: *con...) → void` | `copyIndirectCommandBuffer` |
| `device` | `(&self) → crate::Device` | — |
| `end_encoding` | `(&self) → void` | — |
| `fill_buffer` | `(&self, buffer: &Buffer, off...) → void` | `fillBuffer` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `generate_mipmaps` | `(&self, texture: &Texture) → void` | `generateMipmaps` |
| `get_texture_access_counters` | `(&self,
        texture: &Te...) → void` | `getTextureAccessCounters` |
| `insert_debug_signpost` | `(&self, string: &str) → void` | — |
| `label` | `(&self) → Option<String>` | — |
| `optimize_contents_for_cpu_access` | `(&self, texture: &Texture) → void` | `optimizeContentsForCPUAccess` |
| `optimize_contents_for_cpu_access_slice` | `(&self,
        texture: &Te...) → void` | — |
| `optimize_contents_for_gpu_access` | `(&self, texture: &Texture) → void` | `optimizeContentsForGPUAccess` |
| `optimize_contents_for_gpu_access_slice` | `(&self,
        texture: &Te...) → void` | — |
| `optimize_indirect_command_buffer_ptr` | `(&self,
        indirect_com...) → void` | `optimizeIndirectCommandBuffer` |
| `pop_debug_group` | `(&self) → void` | — |
| `push_debug_group` | `(&self, string: &str) → void` | — |
| `reset_commands_in_buffer_ptr` | `(&self,
        buffer: *con...) → void` | `resetCommandsInBuffer` |
| `reset_texture_access_counters` | `(&self,
        texture: &Te...) → void` | `resetTextureAccessCounters` |
| `resolve_counters_ptr` | `(&self,
        sample_buffe...) → void` | `resolveCounters` |
| `sample_counters_in_buffer_ptr` | `(&self,
        sample_buffe...) → void` | `sampleCountersInBuffer` |
| `synchronize_buffer` | `(&self, buffer: &Buffer) → void` | — |
| `synchronize_resource_ptr` | `(&self, resource: *const c_void) → void` | `synchronizeResource` |
| `synchronize_texture` | `(&self, texture: &Texture) → void` | `synchronizeTexture` |
| `synchronize_texture_slice` | `(&self, texture: &Texture, s...) → void` | — |
| `update_fence` | `(&self, fence: &crate::Fence) → void` | `updateFence` |
| `update_fence_ptr` | `(&self, fence: *const c_void) → void` | — |
| `wait_for_fence` | `(&self, fence: &crate::Fence) → void` | `waitForFence` |
| `wait_for_fence_ptr` | `(&self, fence: *const c_void) → void` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | — |

---

### `BlitPassDescriptor`

C++ equivalent: `NS::BlitPassDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `sample_buffer_attachments` | `(&self) → Option<BlitPassSampleBufferAttachmentDescriptorArray>` | `sampleBufferAttachments` |

---

### `BlitPassSampleBufferAttachmentDescriptor`

C++ equivalent: `NS::BlitPassSampleBufferAttachmentDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `end_of_encoder_sample_index` | `(&self) → UInteger` | `endOfEncoderSampleIndex` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `sample_buffer` | `(&self) → Option<crate::CounterSampleBuffer>` | `sampleBuffer` |
| `start_of_encoder_sample_index` | `(&self) → UInteger` | `startOfEncoderSampleIndex` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_end_of_encoder_sample_index` | `(&self, index: UInteger) → void` | `setEndOfEncoderSampleIndex` |
| `set_sample_buffer` | `(&self, sample_buffer: Optio...) → void` | `setSampleBuffer` |
| `set_start_of_encoder_sample_index` | `(&self, index: UInteger) → void` | `setStartOfEncoderSampleIndex` |

---

### `BlitPassSampleBufferAttachmentDescriptorArray`

C++ equivalent: `NS::BlitPassSampleBufferAttachmentDescriptorArray`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `object` | `(&self, index: UInteger) → Option<BlitPassSampleBufferAttachmentDescriptor>` | `object` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_object` | `(&self,
        attachment: ...) → void` | `setObject` |

---

### `Buffer`

C++ equivalent: `NS::Buffer`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new_remote_buffer_view_for_device` | `(&self, device: &crate::Device) → Option<Buffer>` | `newRemoteBufferViewForDevice` |
| `new_tensor` | `(&self,
        descriptor: ...) → Option<crate::tensor::Tensor>` | `newTensor` |
| `new_texture` | `(&self,
        descriptor: ...) → Option<crate::texture::Texture>` | `newTexture` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `add_debug_marker` | `(&self, marker: &str, locati...) → void` | `addDebugMarker` |
| `allocated_size` | `(&self) → UInteger` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `contents` | `(&self) → Option<*mut c_void>` | `contents` |
| `device` | `(&self) → crate::Device` | — |
| `did_modify_range` | `(&self, location: UInteger, ...) → void` | `didModifyRange` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `gpu_address` | `(&self) → u64` | `gpuAddress` |
| `gpu_resource_id` | `(&self) → ResourceID` | — |
| `label` | `(&self) → Option<String>` | — |
| `length` | `(&self) → UInteger` | `length` |
| `remote_storage_buffer` | `(&self) → Option<Buffer>` | `remoteStorageBuffer` |
| `remove_all_debug_markers` | `(&self) → void` | `removeAllDebugMarkers` |
| `resource_options` | `(&self) → ResourceOptions` | — |
| `sparse_buffer_tier` | `(&self) → BufferSparseTier` | `sparseBufferTier` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | — |

---

### `BufferBinding`

C++ equivalent: `NS::BufferBinding`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `access` | `(&self) → BindingAccess` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `binding_type` | `(&self) → BindingType` | — |
| `buffer_alignment` | `(&self) → UInteger` | `bufferAlignment` |
| `buffer_data_size` | `(&self) → UInteger` | `bufferDataSize` |
| `buffer_data_type` | `(&self) → DataType` | `bufferDataType` |
| `buffer_pointer_type` | `(&self) → Option<PointerType>` | `bufferPointerType` |
| `buffer_struct_type` | `(&self) → Option<StructType>` | `bufferStructType` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index` | `(&self) → UInteger` | — |
| `name` | `(&self) → Option<String>` | — |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_argument` | `(&self) → bool` | — |
| `is_used` | `(&self) → bool` | — |

---

### `BufferLayoutDescriptor`

C++ equivalent: `NS::BufferLayoutDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `step_function` | `(&self) → StepFunction` | `stepFunction` |
| `step_rate` | `(&self) → UInteger` | `stepRate` |
| `stride` | `(&self) → UInteger` | `stride` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_step_function` | `(&self, step_function: StepF...) → void` | `setStepFunction` |
| `set_step_rate` | `(&self, step_rate: UInteger) → void` | `setStepRate` |
| `set_stride` | `(&self, stride: UInteger) → void` | `setStride` |

---

### `BufferLayoutDescriptorArray`

C++ equivalent: `NS::BufferLayoutDescriptorArray`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `object_at` | `(&self, index: UInteger) → Option<BufferLayoutDescriptor>` | `object` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_object_at` | `(&self, descriptor: &BufferL...) → void` | `setObject` |

---

### `CaptureDescriptor`

C++ equivalent: `NS::CaptureDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `capture_object` | `(&self) → Option<*mut c_void>` | `captureObject` |
| `destination` | `(&self) → CaptureDestination` | `destination` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `output_url` | `(&self) → Option<Url>` | `outputURL` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_capture_command_queue` | `(&self, queue: &CommandQueue) → void` | — |
| `set_capture_device` | `(&self, device: &Device) → void` | — |
| `set_capture_object` | `(&self, object: *const c_void) → void` | `setCaptureObject` |
| `set_capture_scope` | `(&self, scope: &CaptureScope) → void` | — |
| `set_destination` | `(&self, destination: Capture...) → void` | `setDestination` |
| `set_output_url` | `(&self, url: &Url) → void` | `setOutputURL` |

---

### `CaptureManager`

C++ equivalent: `NS::CaptureManager`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new_capture_scope_with_command_queue` | `(&self,
        queue: &Comm...) → Option<CaptureScope>` | — |
| `new_capture_scope_with_device` | `(&self, device: &Device) → Option<CaptureScope>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `default_capture_scope` | `(&self) → Option<CaptureScope>` | `defaultCaptureScope` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `shared` | `() → Option<Self>` | — |
| `start_capture` | `(&self, descriptor: &Capture...) → Result<(), CaptureError>` | `sharedCaptureManager` |
| `start_capture_with_command_queue` | `(&self, queue: &CommandQueue) → void` | — |
| `start_capture_with_device` | `(&self, device: &Device) → void` | — |
| `start_capture_with_scope` | `(&self, scope: &CaptureScope) → void` | — |
| `stop_capture` | `(&self) → void` | `stopCapture` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_capturing` | `(&self) → bool` | `isCapturing` |
| `supports_destination` | `(&self, destination: Capture...) → bool` | `supportsDestination` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_default_capture_scope` | `(&self, scope: &CaptureScope) → void` | `setDefaultCaptureScope` |

---

### `CaptureScope`

C++ equivalent: `NS::CaptureScope`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `begin_scope` | `(&self) → void` | `beginScope` |
| `command_queue` | `(&self) → Option<CommandQueue>` | `commandQueue` |
| `device` | `(&self) → Device` | `device` |
| `end_scope` | `(&self) → void` | `endScope` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |

---

### `CommandAllocator`

C++ equivalent: `NS::CommandAllocator`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allocated_size` | `(&self) → u64` | `allocatedSize` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `device` | `(&self) → Option<Device>` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |
| `reset` | `(&self) → void` | `reset` |

---

### `CommandAllocatorDescriptor`

C++ equivalent: `NS::CommandAllocatorDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |

---

### `CommandBuffer`

C++ equivalent: `NS::CommandBuffer`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `acceleration_structure_command_encoder` | `(&self) → *mut c_void` | — |
| `acceleration_structure_command_encoder_with_descriptor` | `(&self,
        descriptor: ...) → *mut c_void` | — |
| `add_completed_handler` | `(&self, handler: F) → void` | — |
| `add_scheduled_handler` | `(&self, handler: F) → void` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `begin_command_buffer` | `(&self, allocator: &CommandA...) → void` | `beginCommandBuffer` |
| `begin_command_buffer_with_options` | `(&self,
        allocator: &...) → void` | — |
| `blit_command_encoder` | `(&self) → *mut c_void` | — |
| `blit_command_encoder_with_descriptor` | `(&self,
        descriptor: ...) → *mut c_void` | — |
| `blit_command_encoder_with_pass_descriptor` | `(&self,
        descriptor: ...) → *mut c_void` | — |
| `command_queue` | `(&self) → crate::command_queue::CommandQueue` | — |
| `commit` | `(&self) → void` | — |
| `compute_command_encoder` | `(&self) → *mut c_void` | `computeCommandEncoder` |
| `compute_command_encoder_with_descriptor` | `(&self,
        descriptor: ...) → *mut c_void` | — |
| `compute_command_encoder_with_dispatch_type` | `(&self,
        dispatch_typ...) → *mut c_void` | — |
| `compute_command_encoder_with_pass_descriptor` | `(&self,
        descriptor: ...) → *mut c_void` | — |
| `device` | `(&self) → crate::Device` | `device` |
| `encode_signal_event` | `(&self, event: &crate::sync:...) → void` | — |
| `encode_wait_for_event` | `(&self, event: &crate::sync:...) → void` | — |
| `end_command_buffer` | `(&self) → void` | `endCommandBuffer` |
| `enqueue` | `(&self) → void` | — |
| `error` | `(&self) → Option<metal_foundation::Error>` | — |
| `error_options` | `(&self) → CommandBufferErrorOption` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `gpu_end_time` | `(&self) → TimeInterval` | — |
| `gpu_start_time` | `(&self) → TimeInterval` | — |
| `kernel_end_time` | `(&self) → TimeInterval` | — |
| `kernel_start_time` | `(&self) → TimeInterval` | — |
| `label` | `(&self) → Option<String>` | `label` |
| `logs` | `(&self) → Option<crate::function_log::LogContainer>` | — |
| `machine_learning_command_encoder` | `(&self,) → Option<super::MachineLearningCommandEncoder>` | `machineLearningCommandEncoder` |
| `parallel_render_command_encoder` | `(&self,
        descriptor: ...) → *mut c_void` | — |
| `parallel_render_command_encoder_with_descriptor` | `(&self,
        descriptor: ...) → *mut c_void` | — |
| `pop_debug_group` | `(&self) → void` | `popDebugGroup` |
| `present_drawable` | `(&self, drawable: *const c_void) → void` | — |
| `present_drawable_after_minimum_duration` | `(&self,
        drawable: *c...) → void` | — |
| `present_drawable_at_time` | `(&self, drawable: *const c_v...) → void` | — |
| `push_debug_group` | `(&self, name: &str) → void` | `pushDebugGroup` |
| `render_command_encoder` | `(&self,
        descriptor: ...) → *mut c_void` | `renderCommandEncoder` |
| `render_command_encoder_with_descriptor` | `(&self,
        descriptor: ...) → *mut c_void` | — |
| `resolve_counter_heap` | `(&self,
        counter_heap...) → void` | `resolveCounterHeap` |
| `resource_state_command_encoder` | `(&self) → *mut c_void` | — |
| `resource_state_command_encoder_with_descriptor` | `(&self,
        descriptor: ...) → *mut c_void` | — |
| `retained_references` | `(&self) → bool` | — |
| `status` | `(&self) → CommandBufferStatus` | — |
| `use_residency_set` | `(&self, residency_set: *cons...) → void` | `useResidencySet` |
| `use_residency_sets` | `(&self, residency_sets: *con...) → void` | `useResidencySets` |
| `wait_until_completed` | `(&self) → void` | — |
| `wait_until_scheduled` | `(&self) → void` | — |
| `write_timestamp_into_heap` | `(&self, counter_heap: *const...) → void` | `writeTimestampIntoHeap` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |

---

### `CommandBuffer`

C++ equivalent: `NS::CommandBuffer`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `acceleration_structure_command_encoder` | `(&self) → *mut c_void` | `accelerationStructureCommandEncoder` |
| `acceleration_structure_command_encoder_with_descriptor` | `(&self,
        descriptor: ...) → *mut c_void` | — |
| `add_completed_handler` | `(&self, handler: F) → void` | `addCompletedHandler` |
| `add_scheduled_handler` | `(&self, handler: F) → void` | `addScheduledHandler` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `begin_command_buffer` | `(&self, allocator: &CommandA...) → void` | — |
| `begin_command_buffer_with_options` | `(&self,
        allocator: &...) → void` | — |
| `blit_command_encoder` | `(&self) → *mut c_void` | `blitCommandEncoder` |
| `blit_command_encoder_with_descriptor` | `(&self,
        descriptor: ...) → *mut c_void` | — |
| `blit_command_encoder_with_pass_descriptor` | `(&self,
        descriptor: ...) → *mut c_void` | — |
| `command_queue` | `(&self) → crate::command_queue::CommandQueue` | `commandQueue` |
| `commit` | `(&self) → void` | `commit` |
| `compute_command_encoder` | `(&self) → *mut c_void` | `computeCommandEncoder` |
| `compute_command_encoder_with_descriptor` | `(&self,
        descriptor: ...) → *mut c_void` | — |
| `compute_command_encoder_with_dispatch_type` | `(&self,
        dispatch_typ...) → *mut c_void` | — |
| `compute_command_encoder_with_pass_descriptor` | `(&self,
        descriptor: ...) → *mut c_void` | — |
| `device` | `(&self) → crate::Device` | `device` |
| `encode_signal_event` | `(&self, event: &crate::sync:...) → void` | `encodeSignalEvent` |
| `encode_wait_for_event` | `(&self, event: &crate::sync:...) → void` | `encodeWait` |
| `end_command_buffer` | `(&self) → void` | — |
| `enqueue` | `(&self) → void` | `enqueue` |
| `error` | `(&self) → Option<metal_foundation::Error>` | `error` |
| `error_options` | `(&self) → CommandBufferErrorOption` | `errorOptions` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `gpu_end_time` | `(&self) → TimeInterval` | `GPUEndTime` |
| `gpu_start_time` | `(&self) → TimeInterval` | `GPUStartTime` |
| `kernel_end_time` | `(&self) → TimeInterval` | `kernelEndTime` |
| `kernel_start_time` | `(&self) → TimeInterval` | `kernelStartTime` |
| `label` | `(&self) → Option<String>` | `label` |
| `logs` | `(&self) → Option<crate::function_log::LogContainer>` | `logs` |
| `machine_learning_command_encoder` | `(&self,) → Option<super::MachineLearningCommandEncoder>` | — |
| `parallel_render_command_encoder` | `(&self,
        descriptor: ...) → *mut c_void` | `parallelRenderCommandEncoder` |
| `parallel_render_command_encoder_with_descriptor` | `(&self,
        descriptor: ...) → *mut c_void` | — |
| `pop_debug_group` | `(&self) → void` | `popDebugGroup` |
| `present_drawable` | `(&self, drawable: *const c_void) → void` | `presentDrawable` |
| `present_drawable_after_minimum_duration` | `(&self,
        drawable: *c...) → void` | `presentDrawableAfterMinimumDuration` |
| `present_drawable_at_time` | `(&self, drawable: *const c_v...) → void` | `presentDrawableAtTime` |
| `push_debug_group` | `(&self, name: &str) → void` | `pushDebugGroup` |
| `render_command_encoder` | `(&self,
        descriptor: ...) → *mut c_void` | `renderCommandEncoder` |
| `render_command_encoder_with_descriptor` | `(&self,
        descriptor: ...) → *mut c_void` | — |
| `resolve_counter_heap` | `(&self,
        counter_heap...) → void` | — |
| `resource_state_command_encoder` | `(&self) → *mut c_void` | `resourceStateCommandEncoder` |
| `resource_state_command_encoder_with_descriptor` | `(&self,
        descriptor: ...) → *mut c_void` | — |
| `retained_references` | `(&self) → bool` | `retainedReferences` |
| `status` | `(&self) → CommandBufferStatus` | `status` |
| `use_residency_set` | `(&self, residency_set: *cons...) → void` | `useResidencySet` |
| `use_residency_sets` | `(&self, residency_sets: *con...) → void` | `useResidencySets` |
| `wait_until_completed` | `(&self) → void` | `waitUntilCompleted` |
| `wait_until_scheduled` | `(&self) → void` | `waitUntilScheduled` |
| `write_timestamp_into_heap` | `(&self, counter_heap: *const...) → void` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |

---

### `CommandBufferDescriptor`

C++ equivalent: `NS::CommandBufferDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `error_options` | `(&self) → CommandBufferErrorOption` | `errorOptions` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `log_state` | `(&self) → *mut c_void` | `logState` |
| `retained_references` | `(&self) → bool` | `retainedReferences` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_error_options` | `(&self, error_options: Comma...) → void` | `setErrorOptions` |
| `set_log_state` | `(&self, log_state: *const c_...) → void` | `setLogState` |
| `set_retained_references` | `(&self, retained_references:...) → void` | `setRetainedReferences` |

---

### `CommandBufferEncoderInfo`

C++ equivalent: `NS::CommandBufferEncoderInfo`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `debug_signposts_ptr` | `(&self) → *mut c_void` | `debugSignposts` |
| `error_state` | `(&self) → CommandEncoderErrorState` | `errorState` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |

---

### `CommandBufferOptions`

C++ equivalent: `NS::CommandBufferOptions`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `log_state` | `(&self) → Option<crate::log_state::LogState>` | `logState` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_log_state` | `(&self, log_state: &crate::l...) → void` | `setLogState` |

---

### `CommandQueue`

C++ equivalent: `NS::CommandQueue`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new_command_buffer` | `(&self) → Option<CommandBuffer>` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `add_residency_set` | `(&self, residency_set: *cons...) → void` | `addResidencySet` |
| `add_residency_sets` | `(&self, residency_sets: *con...) → void` | `addResidencySets` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `command_buffer` | `(&self) → Option<crate::command_buffer::CommandBuffer>` | — |
| `command_buffer_with_descriptor` | `(&self,
        descriptor: ...) → Option<crate::command_buffer::CommandBuffer>` | — |
| `command_buffer_with_descriptor_ptr` | `(&self,
        descriptor: ...) → Option<crate::command_buffer::CommandBuffer>` | — |
| `command_buffer_with_unretained_references` | `(&self,) → Option<crate::command_buffer::CommandBuffer>` | — |
| `commit` | `(&self, command_buffers: &[&...) → void` | `commit` |
| `commit_with_options` | `(&self,
        command_buff...) → void` | — |
| `copy_buffer_mappings_from_buffer` | `(&self,
        source_buffe...) → void` | `copyBufferMappingsFromBuffer` |
| `copy_texture_mappings_from_texture` | `(&self,
        source_textu...) → void` | `copyTextureMappingsFromTexture` |
| `device` | `(&self) → crate::Device` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `insert_debug_capture_boundary` | `(&self) → void` | — |
| `label` | `(&self) → Option<String>` | `label` |
| `remove_residency_set` | `(&self, residency_set: *cons...) → void` | `removeResidencySet` |
| `remove_residency_sets` | `(&self,
        residency_se...) → void` | `removeResidencySets` |
| `signal_drawable` | `(&self, drawable: &Drawable) → void` | `signalDrawable` |
| `signal_event` | `(&self, event: &Event, value...) → void` | `signalEvent` |
| `update_buffer_mappings` | `(&self,
        buffer: *con...) → void` | `updateBufferMappings` |
| `update_texture_mappings` | `(&self,
        texture: *co...) → void` | `updateTextureMappings` |
| `wait_for_drawable` | `(&self, drawable: *const c_void) → void` | — |
| `wait_for_event` | `(&self, event: &crate::Event...) → void` | `wait` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | — |

---

### `CommandQueue`

C++ equivalent: `NS::CommandQueue`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new_command_buffer` | `(&self) → Option<CommandBuffer>` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `add_residency_set` | `(&self, residency_set: *cons...) → void` | `addResidencySet` |
| `add_residency_sets` | `(&self, residency_sets: *con...) → void` | `addResidencySets` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `command_buffer` | `(&self) → Option<crate::command_buffer::CommandBuffer>` | `commandBuffer` |
| `command_buffer_with_descriptor` | `(&self,
        descriptor: ...) → Option<crate::command_buffer::CommandBuffer>` | — |
| `command_buffer_with_descriptor_ptr` | `(&self,
        descriptor: ...) → Option<crate::command_buffer::CommandBuffer>` | — |
| `command_buffer_with_unretained_references` | `(&self,) → Option<crate::command_buffer::CommandBuffer>` | `commandBufferWithUnretainedReferences` |
| `commit` | `(&self, command_buffers: &[&...) → void` | — |
| `commit_with_options` | `(&self,
        command_buff...) → void` | — |
| `copy_buffer_mappings_from_buffer` | `(&self,
        source_buffe...) → void` | — |
| `copy_texture_mappings_from_texture` | `(&self,
        source_textu...) → void` | — |
| `device` | `(&self) → crate::Device` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `insert_debug_capture_boundary` | `(&self) → void` | `insertDebugCaptureBoundary` |
| `label` | `(&self) → Option<String>` | `label` |
| `remove_residency_set` | `(&self, residency_set: *cons...) → void` | `removeResidencySet` |
| `remove_residency_sets` | `(&self,
        residency_se...) → void` | `removeResidencySets` |
| `signal_drawable` | `(&self, drawable: &Drawable) → void` | — |
| `signal_event` | `(&self, event: &Event, value...) → void` | — |
| `update_buffer_mappings` | `(&self,
        buffer: *con...) → void` | — |
| `update_texture_mappings` | `(&self,
        texture: *co...) → void` | — |
| `wait_for_drawable` | `(&self, drawable: *const c_void) → void` | — |
| `wait_for_event` | `(&self, event: &crate::Event...) → void` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |

---

### `CommandQueueDescriptor`

C++ equivalent: `NS::CommandQueueDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `feedback_queue` | `(&self) → DispatchQueue` | `feedbackQueue` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |
| `log_state` | `(&self) → *mut c_void` | — |
| `max_command_buffer_count` | `(&self) → UInteger` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_feedback_queue` | `(&self, queue: DispatchQueue) → void` | `setFeedbackQueue` |
| `set_label` | `(&self, label: &str) → void` | `setLabel` |
| `set_log_state` | `(&self, log_state: *const c_...) → void` | — |
| `set_max_command_buffer_count` | `(&self, count: UInteger) → void` | — |

---

### `CommandQueueDescriptor`

C++ equivalent: `NS::CommandQueueDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `feedback_queue` | `(&self) → DispatchQueue` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | — |
| `log_state` | `(&self) → *mut c_void` | `logState` |
| `max_command_buffer_count` | `(&self) → UInteger` | `maxCommandBufferCount` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_feedback_queue` | `(&self, queue: DispatchQueue) → void` | — |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_log_state` | `(&self, log_state: *const c_...) → void` | `setLogState` |
| `set_max_command_buffer_count` | `(&self, count: UInteger) → void` | `setMaxCommandBufferCount` |

---

### `CommitFeedback`

C++ equivalent: `NS::CommitFeedback`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `error` | `(&self) → Option<metal_foundation::Error>` | `error` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `gpu_duration` | `(&self) → TimeInterval` | — |
| `gpu_end_time` | `(&self) → TimeInterval` | `GPUEndTime` |
| `gpu_start_time` | `(&self) → TimeInterval` | `GPUStartTime` |

---

### `CommitOptions`

C++ equivalent: `NS::CommitOptions`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `add_feedback_handler` | `(&self, handler: F) → void` | `addFeedbackHandler` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |

---

### `CompileOptions`

C++ equivalent: `NS::CompileOptions`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allow_referencing_undefined_symbols` | `(&self) → bool` | `allowReferencingUndefinedSymbols` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `compile_symbol_visibility` | `(&self) → CompileSymbolVisibility` | `compileSymbolVisibility` |
| `enable_logging` | `(&self) → bool` | `enableLogging` |
| `fast_math_enabled` | `(&self) → bool` | `fastMathEnabled` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `install_name` | `(&self) → Option<String>` | `installName` |
| `language_version` | `(&self) → LanguageVersion` | `languageVersion` |
| `library_type` | `(&self) → LibraryType` | `libraries` |
| `math_floating_point_functions` | `(&self) → MathFloatingPointFunctions` | `mathFloatingPointFunctions` |
| `math_mode` | `(&self) → MathMode` | `mathMode` |
| `max_total_threads_per_threadgroup` | `(&self) → UInteger` | `maxTotalThreadsPerThreadgroup` |
| `optimization_level` | `(&self) → LibraryOptimizationLevel` | `optimizationLevel` |
| `preprocessor_macros_raw` | `(&self) → *mut c_void` | `preprocessorMacros` |
| `preserve_invariance` | `(&self) → bool` | `preserveInvariance` |
| `required_threads_per_threadgroup` | `(&self) → Size` | `requiredThreadsPerThreadgroup` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_allow_referencing_undefined_symbols` | `(&self, allow: bool) → void` | `setAllowReferencingUndefinedSymbols` |
| `set_compile_symbol_visibility` | `(&self, visibility: CompileS...) → void` | `setCompileSymbolVisibility` |
| `set_enable_logging` | `(&self, enable: bool) → void` | `setEnableLogging` |
| `set_fast_math_enabled` | `(&self, enabled: bool) → void` | `setFastMathEnabled` |
| `set_install_name` | `(&self, name: &str) → void` | `setInstallName` |
| `set_language_version` | `(&self, version: LanguageVer...) → void` | `setLanguageVersion` |
| `set_library_type` | `(&self, lib_type: LibraryType) → void` | `setLibraries` |
| `set_math_floating_point_functions` | `(&self, funcs: MathFloatingP...) → void` | `setMathFloatingPointFunctions` |
| `set_math_mode` | `(&self, mode: MathMode) → void` | `setMathMode` |
| `set_max_total_threads_per_threadgroup` | `(&self, count: UInteger) → void` | `setMaxTotalThreadsPerThreadgroup` |
| `set_optimization_level` | `(&self, level: LibraryOptimi...) → void` | `setOptimizationLevel` |
| `set_preprocessor_macros_raw` | `(&self, dict: *const c_void) → void` | `setPreprocessorMacros` |
| `set_preserve_invariance` | `(&self, preserve: bool) → void` | `setPreserveInvariance` |
| `set_required_threads_per_threadgroup` | `(&self, size: Size) → void` | `setRequiredThreadsPerThreadgroup` |

---

### `Compiler`

C++ equivalent: `NS::Compiler`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new_binary_function` | `(&self,
        descriptor: ...) → Result<BinaryFunction, metal_foundation::Error>` | `newBinaryFunction` |
| `new_binary_function_async` | `(&self,
        descriptor: ...) → Option<CompilerTask>` | — |
| `new_compute_pipeline_state` | `(&self,
        descriptor: ...) → Result<ComputePipelineState, metal_foundation::Error>` | `newComputePipelineState` |
| `new_compute_pipeline_state_async` | `(&self,
        descriptor: ...) → Option<CompilerTask>` | — |
| `new_compute_pipeline_state_with_dynamic_linking` | `(&self,
        descriptor: ...) → Result<ComputePipelineState, metal_foundation::Error>` | — |
| `new_compute_pipeline_state_with_dynamic_linking_async` | `(&self,
        descriptor: ...) → Option<CompilerTask>` | — |
| `new_dynamic_library_from_library` | `(&self,
        library: &Li...) → Result<*mut c_void, metal_foundation::Error>` | — |
| `new_dynamic_library_from_library_async` | `(&self,
        library: &Li...) → Option<CompilerTask>` | — |
| `new_dynamic_library_from_url` | `(&self,
        url: *const ...) → Result<*mut c_void, metal_foundation::Error>` | `newDynamicLibrary` |
| `new_dynamic_library_from_url_async` | `(&self,
        url: *const ...) → Option<CompilerTask>` | — |
| `new_library` | `(&self,
        descriptor: ...) → Result<Library, metal_foundation::Error>` | `newLibrary` |
| `new_library_async` | `(&self,
        descriptor: ...) → Option<CompilerTask>` | — |
| `new_machine_learning_pipeline_state` | `(&self,
        descriptor: ...) → Result<MachineLearningPipelineState, metal_foundation::Error>` | `newMachineLearningPipelineState` |
| `new_machine_learning_pipeline_state_async` | `(&self,
        descriptor: ...) → Option<CompilerTask>` | — |
| `new_render_pipeline_state` | `(&self,
        descriptor: ...) → Result<RenderPipelineState, metal_foundation::Error>` | `newRenderPipelineState` |
| `new_render_pipeline_state_async` | `(&self,
        descriptor: ...) → Option<CompilerTask>` | — |
| `new_render_pipeline_state_by_specialization` | `(&self,
        descriptor: ...) → Result<RenderPipelineState, metal_foundation::Error>` | `newRenderPipelineStateBySpecialization` |
| `new_render_pipeline_state_by_specialization_async` | `(&self,
        descriptor: ...) → Option<CompilerTask>` | — |
| `new_render_pipeline_state_with_dynamic_linking` | `(&self,
        descriptor: ...) → Result<RenderPipelineState, metal_foundation::Error>` | — |
| `new_render_pipeline_state_with_dynamic_linking_async` | `(&self,
        descriptor: ...) → Option<CompilerTask>` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `device` | `(&self) → Option<Device>` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |
| `pipeline_data_set_serializer` | `(&self) → Option<PipelineDataSetSerializer>` | `pipelineDataSetSerializer` |

---

### `CompilerDescriptor`

C++ equivalent: `NS::CompilerDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |
| `pipeline_data_set_serializer` | `(&self) → Option<PipelineDataSetSerializer>` | `pipelineDataSetSerializer` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |
| `set_pipeline_data_set_serializer` | `(&self, serializer: &Pipelin...) → void` | `setPipelineDataSetSerializer` |

---

### `CompilerTask`

C++ equivalent: `NS::CompilerTask`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `compiler_raw` | `(&self) → *mut c_void` | `compiler` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `status` | `(&self) → CompilerTaskStatus` | `status` |
| `wait_until_completed` | `(&self) → void` | `waitUntilCompleted` |

---

### `CompilerTaskOptions`

C++ equivalent: `NS::CompilerTaskOptions`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `lookup_archives_raw` | `(&self) → *mut c_void` | `lookupArchives` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_lookup_archives_raw` | `(&self, archives: *const c_void) → void` | `setLookupArchives` |

---

### `ComputeCommandEncoder`

C++ equivalent: `NS::ComputeCommandEncoder`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `barrier` | `(&self) → void` | — |
| `barrier_after_queue_stages` | `(&self,
        after_stages...) → void` | — |
| `barrier_buffer` | `(&self, buffer: *const c_voi...) → void` | — |
| `barrier_texture` | `(&self, texture: *const c_vo...) → void` | — |
| `build_acceleration_structure` | `(&self,
        acceleration...) → void` | `buildAccelerationStructure` |
| `command_buffer` | `(&self) → crate::CommandBuffer` | — |
| `copy_acceleration_structure` | `(&self,
        source: &cra...) → void` | `copyAccelerationStructure` |
| `copy_and_compact_acceleration_structure` | `(&self,
        source: &cra...) → void` | `copyAndCompactAccelerationStructure` |
| `copy_from_buffer_to_buffer` | `(&self,
        source_buffe...) → void` | `copyFromBuffer` |
| `copy_from_tensor` | `(&self,
        source_tenso...) → void` | `copyFromTensor` |
| `copy_from_texture_to_buffer` | `(&self,
        source_textu...) → void` | `copyFromBuffer` |
| `copy_from_texture_to_buffer_with_options` | `(&self,
        source_textu...) → void` | — |
| `copy_from_texture_to_texture` | `(&self,
        source_textu...) → void` | `copyFromTexture` |
| `copy_from_texture_with_origin` | `(&self,
        source_textu...) → void` | — |
| `copy_from_texture_with_slices` | `(&self,
        source_textu...) → void` | — |
| `copy_indirect_command_buffer` | `(&self,
        source: &cra...) → void` | `copyIndirectCommandBuffer` |
| `device` | `(&self) → crate::Device` | — |
| `dispatch_threadgroups` | `(&self,
        threadgroups...) → void` | `dispatchThreadgroups` |
| `dispatch_threadgroups_indirect` | `(&self,
        indirect_buf...) → void` | — |
| `dispatch_threadgroups_with_indirect_buffer` | `(&self,
        indirect_buf...) → void` | — |
| `dispatch_threads` | `(&self, threads_per_grid: Si...) → void` | `dispatchThreads` |
| `dispatch_threads_indirect` | `(&self,
        indirect_buf...) → void` | — |
| `dispatch_type` | `(&self) → DispatchType` | — |
| `end_encoding` | `(&self) → void` | — |
| `execute_commands_in_buffer` | `(&self,
        indirect_com...) → void` | `executeCommandsInBuffer` |
| `execute_commands_in_buffer_indirect` | `(&self,
        indirect_com...) → void` | — |
| `execute_commands_in_buffer_ptr` | `(&self,
        indirect_com...) → void` | — |
| `execute_commands_in_buffer_with_indirect_range_ptr` | `(&self,
        indirect_com...) → void` | — |
| `fill_buffer` | `(&self,
        buffer: *con...) → void` | `fillBuffer` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `generate_mipmaps` | `(&self, texture: *const c_void) → void` | `generateMipmaps` |
| `insert_debug_signpost` | `(&self, string: &str) → void` | — |
| `label` | `(&self) → Option<String>` | — |
| `memory_barrier_with_resources_ptr` | `(&self,
        resources: *...) → void` | — |
| `memory_barrier_with_scope` | `(&self, scope: BarrierScope) → void` | — |
| `optimize_contents_for_cpu_access` | `(&self, texture: *const c_void) → void` | `optimizeContentsForCPUAccess` |
| `optimize_contents_for_cpu_access_slice_level` | `(&self,
        texture: *co...) → void` | — |
| `optimize_contents_for_gpu_access` | `(&self, texture: *const c_void) → void` | `optimizeContentsForGPUAccess` |
| `optimize_contents_for_gpu_access_slice_level` | `(&self,
        texture: *co...) → void` | — |
| `optimize_indirect_command_buffer` | `(&self,
        indirect_com...) → void` | `optimizeIndirectCommandBuffer` |
| `pop_debug_group` | `(&self) → void` | — |
| `push_debug_group` | `(&self, string: &str) → void` | — |
| `refit_acceleration_structure` | `(&self,
        source: &cra...) → void` | `refitAccelerationStructure` |
| `refit_acceleration_structure_with_options` | `(&self,
        source: &cra...) → void` | — |
| `reset_commands_in_buffer` | `(&self,
        buffer: &cra...) → void` | `resetCommandsInBuffer` |
| `sample_counters_in_buffer_ptr` | `(&self,
        sample_buffe...) → void` | — |
| `stages` | `(&self) → crate::Stages` | `stages` |
| `update_fence` | `(&self, fence: &crate::Fence) → void` | — |
| `update_fence_ptr` | `(&self, fence: *const c_void) → void` | — |
| `use_buffer` | `(&self, buffer: &Buffer, usa...) → void` | — |
| `use_heap` | `(&self, heap: &crate::Heap) → void` | — |
| `use_heap_ptr` | `(&self, heap: *const c_void) → void` | — |
| `use_heaps` | `(&self, heaps: *const *const...) → void` | — |
| `use_heaps_ptr` | `(&self, heaps: *const *const...) → void` | — |
| `use_resource` | `(&self, resource: *const c_v...) → void` | — |
| `use_resource_ptr` | `(&self, resource: *const c_v...) → void` | — |
| `use_resources` | `(&self,
        resources: *...) → void` | — |
| `use_resources_ptr` | `(&self,
        resources: *...) → void` | — |
| `use_texture` | `(&self, texture: &Texture, u...) → void` | — |
| `wait_for_fence` | `(&self, fence: &crate::Fence) → void` | — |
| `wait_for_fence_ptr` | `(&self, fence: *const c_void) → void` | — |
| `write_compacted_acceleration_structure_size` | `(&self,
        acceleration...) → void` | `writeCompactedAccelerationStructureSize` |
| `write_timestamp` | `(&self,
        granularity:...) → void` | `writeTimestamp` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_acceleration_structure` | `(&self,
        acceleration...) → void` | — |
| `set_acceleration_structure_ptr` | `(&self,
        acceleration...) → void` | — |
| `set_argument_table` | `(&self, table: *const c_void...) → void` | `setArgumentTable` |
| `set_buffer` | `(&self, buffer: &Buffer, off...) → void` | — |
| `set_buffer_offset` | `(&self, offset: UInteger, in...) → void` | — |
| `set_buffer_offset_with_stride` | `(&self,
        offset: UInt...) → void` | — |
| `set_buffer_with_stride` | `(&self,
        buffer: &Buf...) → void` | — |
| `set_buffers` | `(&self,
        buffers: *co...) → void` | — |
| `set_buffers_ptr` | `(&self,
        buffers: *co...) → void` | — |
| `set_buffers_with_strides_ptr` | `(&self,
        buffers: *co...) → void` | — |
| `set_bytes` | `(&self, bytes: &[u8], index:...) → void` | — |
| `set_bytes_with_stride` | `(&self, bytes: &[u8], stride...) → void` | — |
| `set_compute_pipeline_state` | `(&self, state: &crate::Compu...) → void` | `setComputePipelineState` |
| `set_imageblock_width` | `(&self, width: UInteger, hei...) → void` | `setImageblockWidth` |
| `set_intersection_function_table_ptr` | `(&self,
        intersection...) → void` | — |
| `set_intersection_function_tables_ptr` | `(&self,
        intersection...) → void` | — |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_sampler_state` | `(&self, sampler: &crate::Sam...) → void` | — |
| `set_sampler_state_with_lod` | `(&self,
        sampler: *co...) → void` | — |
| `set_sampler_state_with_lod_clamps` | `(&self,
        sampler: &cr...) → void` | — |
| `set_sampler_states_ptr` | `(&self,
        samplers: *c...) → void` | — |
| `set_sampler_states_with_lod_clamps_ptr` | `(&self,
        samplers: *c...) → void` | — |
| `set_stage_in_region` | `(&self, region: Region) → void` | — |
| `set_stage_in_region_with_indirect_buffer` | `(&self,
        indirect_buf...) → void` | — |
| `set_texture` | `(&self, texture: &Texture, i...) → void` | — |
| `set_textures` | `(&self,
        textures: *c...) → void` | — |
| `set_textures_ptr` | `(&self,
        textures: *c...) → void` | — |
| `set_threadgroup_memory_length` | `(&self, length: UInteger, in...) → void` | `setThreadgroupMemoryLength` |
| `set_visible_function_table_ptr` | `(&self,
        visible_func...) → void` | — |
| `set_visible_function_tables_ptr` | `(&self,
        visible_func...) → void` | — |

---

### `ComputeCommandEncoder`

C++ equivalent: `NS::ComputeCommandEncoder`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `barrier` | `(&self) → void` | — |
| `barrier_after_queue_stages` | `(&self,
        after_stages...) → void` | — |
| `barrier_buffer` | `(&self, buffer: *const c_voi...) → void` | — |
| `barrier_texture` | `(&self, texture: *const c_vo...) → void` | — |
| `build_acceleration_structure` | `(&self,
        acceleration...) → void` | — |
| `command_buffer` | `(&self) → crate::CommandBuffer` | — |
| `copy_acceleration_structure` | `(&self,
        source: &cra...) → void` | — |
| `copy_and_compact_acceleration_structure` | `(&self,
        source: &cra...) → void` | — |
| `copy_from_buffer_to_buffer` | `(&self,
        source_buffe...) → void` | — |
| `copy_from_tensor` | `(&self,
        source_tenso...) → void` | — |
| `copy_from_texture_to_buffer` | `(&self,
        source_textu...) → void` | — |
| `copy_from_texture_to_buffer_with_options` | `(&self,
        source_textu...) → void` | — |
| `copy_from_texture_to_texture` | `(&self,
        source_textu...) → void` | — |
| `copy_from_texture_with_origin` | `(&self,
        source_textu...) → void` | — |
| `copy_from_texture_with_slices` | `(&self,
        source_textu...) → void` | — |
| `copy_indirect_command_buffer` | `(&self,
        source: &cra...) → void` | — |
| `device` | `(&self) → crate::Device` | — |
| `dispatch_threadgroups` | `(&self,
        threadgroups...) → void` | `dispatchThreadgroups` |
| `dispatch_threadgroups_indirect` | `(&self,
        indirect_buf...) → void` | — |
| `dispatch_threadgroups_with_indirect_buffer` | `(&self,
        indirect_buf...) → void` | — |
| `dispatch_threads` | `(&self, threads_per_grid: Si...) → void` | `dispatchThreads` |
| `dispatch_threads_indirect` | `(&self,
        indirect_buf...) → void` | — |
| `dispatch_type` | `(&self) → DispatchType` | `dispatchType` |
| `end_encoding` | `(&self) → void` | — |
| `execute_commands_in_buffer` | `(&self,
        indirect_com...) → void` | `executeCommandsInBuffer` |
| `execute_commands_in_buffer_indirect` | `(&self,
        indirect_com...) → void` | — |
| `execute_commands_in_buffer_ptr` | `(&self,
        indirect_com...) → void` | — |
| `execute_commands_in_buffer_with_indirect_range_ptr` | `(&self,
        indirect_com...) → void` | — |
| `fill_buffer` | `(&self,
        buffer: *con...) → void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `generate_mipmaps` | `(&self, texture: *const c_void) → void` | — |
| `insert_debug_signpost` | `(&self, string: &str) → void` | — |
| `label` | `(&self) → Option<String>` | — |
| `memory_barrier_with_resources_ptr` | `(&self,
        resources: *...) → void` | — |
| `memory_barrier_with_scope` | `(&self, scope: BarrierScope) → void` | `memoryBarrier` |
| `optimize_contents_for_cpu_access` | `(&self, texture: *const c_void) → void` | — |
| `optimize_contents_for_cpu_access_slice_level` | `(&self,
        texture: *co...) → void` | — |
| `optimize_contents_for_gpu_access` | `(&self, texture: *const c_void) → void` | — |
| `optimize_contents_for_gpu_access_slice_level` | `(&self,
        texture: *co...) → void` | — |
| `optimize_indirect_command_buffer` | `(&self,
        indirect_com...) → void` | — |
| `pop_debug_group` | `(&self) → void` | — |
| `push_debug_group` | `(&self, string: &str) → void` | — |
| `refit_acceleration_structure` | `(&self,
        source: &cra...) → void` | — |
| `refit_acceleration_structure_with_options` | `(&self,
        source: &cra...) → void` | — |
| `reset_commands_in_buffer` | `(&self,
        buffer: &cra...) → void` | — |
| `sample_counters_in_buffer_ptr` | `(&self,
        sample_buffe...) → void` | `sampleCountersInBuffer` |
| `stages` | `(&self) → crate::Stages` | — |
| `update_fence` | `(&self, fence: &crate::Fence) → void` | `updateFence` |
| `update_fence_ptr` | `(&self, fence: *const c_void) → void` | — |
| `use_buffer` | `(&self, buffer: &Buffer, usa...) → void` | — |
| `use_heap` | `(&self, heap: &crate::Heap) → void` | `useHeap` |
| `use_heap_ptr` | `(&self, heap: *const c_void) → void` | — |
| `use_heaps` | `(&self, heaps: *const *const...) → void` | `useHeaps` |
| `use_heaps_ptr` | `(&self, heaps: *const *const...) → void` | — |
| `use_resource` | `(&self, resource: *const c_v...) → void` | `useResource` |
| `use_resource_ptr` | `(&self, resource: *const c_v...) → void` | — |
| `use_resources` | `(&self,
        resources: *...) → void` | `useResources` |
| `use_resources_ptr` | `(&self,
        resources: *...) → void` | — |
| `use_texture` | `(&self, texture: &Texture, u...) → void` | — |
| `wait_for_fence` | `(&self, fence: &crate::Fence) → void` | `waitForFence` |
| `wait_for_fence_ptr` | `(&self, fence: *const c_void) → void` | — |
| `write_compacted_acceleration_structure_size` | `(&self,
        acceleration...) → void` | — |
| `write_timestamp` | `(&self,
        granularity:...) → void` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_acceleration_structure` | `(&self,
        acceleration...) → void` | `setAccelerationStructure` |
| `set_acceleration_structure_ptr` | `(&self,
        acceleration...) → void` | — |
| `set_argument_table` | `(&self, table: *const c_void...) → void` | — |
| `set_buffer` | `(&self, buffer: &Buffer, off...) → void` | `setBuffer` |
| `set_buffer_offset` | `(&self, offset: UInteger, in...) → void` | `setBufferOffset` |
| `set_buffer_offset_with_stride` | `(&self,
        offset: UInt...) → void` | — |
| `set_buffer_with_stride` | `(&self,
        buffer: &Buf...) → void` | — |
| `set_buffers` | `(&self,
        buffers: *co...) → void` | `setBuffers` |
| `set_buffers_ptr` | `(&self,
        buffers: *co...) → void` | — |
| `set_buffers_with_strides_ptr` | `(&self,
        buffers: *co...) → void` | — |
| `set_bytes` | `(&self, bytes: &[u8], index:...) → void` | `setBytes` |
| `set_bytes_with_stride` | `(&self, bytes: &[u8], stride...) → void` | — |
| `set_compute_pipeline_state` | `(&self, state: &crate::Compu...) → void` | `setComputePipelineState` |
| `set_imageblock_width` | `(&self, width: UInteger, hei...) → void` | `setImageblockWidth` |
| `set_intersection_function_table_ptr` | `(&self,
        intersection...) → void` | `setIntersectionFunctionTable` |
| `set_intersection_function_tables_ptr` | `(&self,
        intersection...) → void` | — |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_sampler_state` | `(&self, sampler: &crate::Sam...) → void` | `setSamplerState` |
| `set_sampler_state_with_lod` | `(&self,
        sampler: *co...) → void` | — |
| `set_sampler_state_with_lod_clamps` | `(&self,
        sampler: &cr...) → void` | — |
| `set_sampler_states_ptr` | `(&self,
        samplers: *c...) → void` | `setSamplerStates` |
| `set_sampler_states_with_lod_clamps_ptr` | `(&self,
        samplers: *c...) → void` | — |
| `set_stage_in_region` | `(&self, region: Region) → void` | `setStageInRegion` |
| `set_stage_in_region_with_indirect_buffer` | `(&self,
        indirect_buf...) → void` | — |
| `set_texture` | `(&self, texture: &Texture, i...) → void` | `setTexture` |
| `set_textures` | `(&self,
        textures: *c...) → void` | `setTextures` |
| `set_textures_ptr` | `(&self,
        textures: *c...) → void` | — |
| `set_threadgroup_memory_length` | `(&self, length: UInteger, in...) → void` | `setThreadgroupMemoryLength` |
| `set_visible_function_table_ptr` | `(&self,
        visible_func...) → void` | `setVisibleFunctionTable` |
| `set_visible_function_tables_ptr` | `(&self,
        visible_func...) → void` | — |

---

### `ComputePassDescriptor`

C++ equivalent: `NS::ComputePassDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `dispatch_type` | `(&self) → DispatchType` | `dispatchType` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `sample_buffer_attachments` | `(&self,) → Option<ComputePassSampleBufferAttachmentDescriptorArray>` | `sampleBufferAttachments` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_dispatch_type` | `(&self, dispatch_type: Dispa...) → void` | `setDispatchType` |

---

### `ComputePassSampleBufferAttachmentDescriptor`

C++ equivalent: `NS::ComputePassSampleBufferAttachmentDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `end_of_encoder_sample_index` | `(&self) → UInteger` | `endOfEncoderSampleIndex` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `sample_buffer` | `(&self) → Option<crate::CounterSampleBuffer>` | `sampleBuffer` |
| `start_of_encoder_sample_index` | `(&self) → UInteger` | `startOfEncoderSampleIndex` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_end_of_encoder_sample_index` | `(&self, index: UInteger) → void` | `setEndOfEncoderSampleIndex` |
| `set_sample_buffer` | `(&self, sample_buffer: Optio...) → void` | `setSampleBuffer` |
| `set_start_of_encoder_sample_index` | `(&self, index: UInteger) → void` | `setStartOfEncoderSampleIndex` |

---

### `ComputePassSampleBufferAttachmentDescriptorArray`

C++ equivalent: `NS::ComputePassSampleBufferAttachmentDescriptorArray`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `object` | `(&self, index: UInteger) → Option<ComputePassSampleBufferAttachmentDescriptor>` | `object` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_object` | `(&self,
        attachment: ...) → void` | `setObject` |

---

### `ComputePipelineDescriptor`

C++ equivalent: `NS::ComputePipelineDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `binary_archives_raw` | `(&self) → *mut c_void` | — |
| `buffers` | `(&self) → Option<PipelineBufferDescriptorArray>` | — |
| `compute_function` | `(&self) → Option<crate::Function>` | — |
| `compute_function_descriptor` | `(&self) → Option<FunctionDescriptor>` | `computeFunctionDescriptor` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `insert_libraries_raw` | `(&self) → *mut c_void` | — |
| `label` | `(&self) → Option<String>` | — |
| `linked_functions` | `(&self) → Option<crate::LinkedFunctions>` | — |
| `max_call_stack_depth` | `(&self) → UInteger` | — |
| `max_total_threads_per_threadgroup` | `(&self) → UInteger` | `maxTotalThreadsPerThreadgroup` |
| `options` | `(&self) → Option<PipelineOptions>` | — |
| `preloaded_libraries_raw` | `(&self) → *mut c_void` | — |
| `required_threads_per_threadgroup` | `(&self) → Size` | `requiredThreadsPerThreadgroup` |
| `reset` | `(&self) → void` | `reset` |
| `shader_validation` | `(&self) → ShaderValidation` | — |
| `stage_input_descriptor_raw` | `(&self) → *mut c_void` | — |
| `static_linking_descriptor` | `(&self) → Option<StaticLinkingDescriptor>` | `staticLinkingDescriptor` |
| `support_adding_binary_functions` | `(&self) → bool` | — |
| `support_binary_linking` | `(&self) → bool` | `supportBinaryLinking` |
| `support_indirect_command_buffers` | `(&self) → IndirectCommandBufferSupportState` | `supportIndirectCommandBuffers` |
| `thread_group_size_is_multiple_of_thread_execution_width` | `(&self) → bool` | `threadGroupSizeIsMultipleOfThreadExecutionWidth` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_binary_archives_raw` | `(&self, archives: *const c_void) → void` | — |
| `set_compute_function` | `(&self, function: Option<&cr...) → void` | — |
| `set_compute_function_descriptor` | `(&self, descriptor: &Functio...) → void` | `setComputeFunctionDescriptor` |
| `set_insert_libraries_raw` | `(&self, libraries: *const c_...) → void` | — |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_linked_functions` | `(&self, functions: Option<&c...) → void` | — |
| `set_max_call_stack_depth` | `(&self, depth: UInteger) → void` | — |
| `set_max_total_threads_per_threadgroup` | `(&self, max_threads: UInteger) → void` | `setMaxTotalThreadsPerThreadgroup` |
| `set_options` | `(&self, options: &PipelineOp...) → void` | — |
| `set_preloaded_libraries_raw` | `(&self, libraries: *const c_...) → void` | — |
| `set_required_threads_per_threadgroup` | `(&self, size: Size) → void` | `setRequiredThreadsPerThreadgroup` |
| `set_shader_validation` | `(&self, validation: ShaderVa...) → void` | — |
| `set_stage_input_descriptor_raw` | `(&self, descriptor: *const c...) → void` | — |
| `set_static_linking_descriptor` | `(&self, descriptor: &StaticL...) → void` | `setStaticLinkingDescriptor` |
| `set_support_adding_binary_functions` | `(&self, support: bool) → void` | — |
| `set_support_binary_linking` | `(&self, support: bool) → void` | `setSupportBinaryLinking` |
| `set_support_indirect_command_buffers` | `(&self, state: IndirectComma...) → void` | `setSupportIndirectCommandBuffers` |
| `set_thread_group_size_is_multiple_of_thread_execution_width` | `(&self, value: bool) → void` | `setThreadGroupSizeIsMultipleOfThreadExecutionWidth` |

---

### `ComputePipelineDescriptor`

C++ equivalent: `NS::ComputePipelineDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `binary_archives_raw` | `(&self) → *mut c_void` | `binaryArchives` |
| `buffers` | `(&self) → Option<PipelineBufferDescriptorArray>` | `buffers` |
| `compute_function` | `(&self) → Option<crate::Function>` | `computeFunction` |
| `compute_function_descriptor` | `(&self) → Option<FunctionDescriptor>` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `insert_libraries_raw` | `(&self) → *mut c_void` | `insertLibraries` |
| `label` | `(&self) → Option<String>` | `label` |
| `linked_functions` | `(&self) → Option<crate::LinkedFunctions>` | `linkedFunctions` |
| `max_call_stack_depth` | `(&self) → UInteger` | `maxCallStackDepth` |
| `max_total_threads_per_threadgroup` | `(&self) → UInteger` | `maxTotalThreadsPerThreadgroup` |
| `options` | `(&self) → Option<PipelineOptions>` | — |
| `preloaded_libraries_raw` | `(&self) → *mut c_void` | `preloadedLibraries` |
| `required_threads_per_threadgroup` | `(&self) → Size` | `requiredThreadsPerThreadgroup` |
| `reset` | `(&self) → void` | `reset` |
| `shader_validation` | `(&self) → ShaderValidation` | `shaderValidation` |
| `stage_input_descriptor_raw` | `(&self) → *mut c_void` | `stageInputDescriptor` |
| `static_linking_descriptor` | `(&self) → Option<StaticLinkingDescriptor>` | — |
| `support_adding_binary_functions` | `(&self) → bool` | `supportAddingBinaryFunctions` |
| `support_binary_linking` | `(&self) → bool` | — |
| `support_indirect_command_buffers` | `(&self) → IndirectCommandBufferSupportState` | `supportIndirectCommandBuffers` |
| `thread_group_size_is_multiple_of_thread_execution_width` | `(&self) → bool` | `threadGroupSizeIsMultipleOfThreadExecutionWidth` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_binary_archives_raw` | `(&self, archives: *const c_void) → void` | `setBinaryArchives` |
| `set_compute_function` | `(&self, function: Option<&cr...) → void` | `setComputeFunction` |
| `set_compute_function_descriptor` | `(&self, descriptor: &Functio...) → void` | — |
| `set_insert_libraries_raw` | `(&self, libraries: *const c_...) → void` | `setInsertLibraries` |
| `set_label` | `(&self, label: &str) → void` | `setLabel` |
| `set_linked_functions` | `(&self, functions: Option<&c...) → void` | `setLinkedFunctions` |
| `set_max_call_stack_depth` | `(&self, depth: UInteger) → void` | `setMaxCallStackDepth` |
| `set_max_total_threads_per_threadgroup` | `(&self, max_threads: UInteger) → void` | `setMaxTotalThreadsPerThreadgroup` |
| `set_options` | `(&self, options: &PipelineOp...) → void` | — |
| `set_preloaded_libraries_raw` | `(&self, libraries: *const c_...) → void` | `setPreloadedLibraries` |
| `set_required_threads_per_threadgroup` | `(&self, size: Size) → void` | `setRequiredThreadsPerThreadgroup` |
| `set_shader_validation` | `(&self, validation: ShaderVa...) → void` | `setShaderValidation` |
| `set_stage_input_descriptor_raw` | `(&self, descriptor: *const c...) → void` | `setStageInputDescriptor` |
| `set_static_linking_descriptor` | `(&self, descriptor: &StaticL...) → void` | — |
| `set_support_adding_binary_functions` | `(&self, support: bool) → void` | `setSupportAddingBinaryFunctions` |
| `set_support_binary_linking` | `(&self, support: bool) → void` | — |
| `set_support_indirect_command_buffers` | `(&self, state: IndirectComma...) → void` | `setSupportIndirectCommandBuffers` |
| `set_thread_group_size_is_multiple_of_thread_execution_width` | `(&self, value: bool) → void` | `setThreadGroupSizeIsMultipleOfThreadExecutionWidth` |

---

### `ComputePipelineReflection`

C++ equivalent: `NS::ComputePipelineReflection`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `arguments_raw` | `(&self) → *mut c_void` | `arguments` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `bindings_raw` | `(&self) → *mut c_void` | `bindings` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |

---

### `ComputePipelineState`

C++ equivalent: `NS::ComputePipelineState`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new_compute_pipeline_state_with_binary_functions` | `(&self,
        binary_funct...) → Result<ComputePipelineState, metal_foundation::Error>` | `newComputePipelineStateWithBinaryFunctions` |
| `new_compute_pipeline_state_with_functions` | `(&self,
        functions: *...) → Result<ComputePipelineState, metal_foundation::Error>` | `newComputePipelineState` |
| `new_intersection_function_table` | `(&self,
        descriptor: ...) → Option<crate::IntersectionFunctionTable>` | `newIntersectionFunctionTable` |
| `new_visible_function_table` | `(&self,
        descriptor: ...) → Option<crate::VisibleFunctionTable>` | `newVisibleFunctionTable` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `device` | `(&self) → crate::Device` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function_handle_with_binary_function` | `(&self,
        binary_funct...) → Option<crate::FunctionHandle>` | — |
| `function_handle_with_function` | `(&self,
        function: &c...) → Option<crate::FunctionHandle>` | — |
| `function_handle_with_name` | `(&self, name: &str) → Option<crate::FunctionHandle>` | `functionHandle` |
| `gpu_resource_id` | `(&self) → ResourceID` | `gpuResourceID` |
| `imageblock_memory_length` | `(&self, dimensions: Size) → UInteger` | `imageblockMemoryLength` |
| `label` | `(&self) → Option<String>` | `label` |
| `max_total_threads_per_threadgroup` | `(&self) → UInteger` | `maxTotalThreadsPerThreadgroup` |
| `reflection` | `(&self) → Option<ComputePipelineReflection>` | `reflection` |
| `required_threads_per_threadgroup` | `(&self) → Size` | `requiredThreadsPerThreadgroup` |
| `shader_validation` | `(&self) → ShaderValidation` | `shaderValidation` |
| `static_threadgroup_memory_length` | `(&self) → UInteger` | `staticThreadgroupMemoryLength` |
| `support_indirect_command_buffers` | `(&self) → bool` | `supportIndirectCommandBuffers` |
| `thread_execution_width` | `(&self) → UInteger` | `threadExecutionWidth` |

---

### `Counter`

C++ equivalent: `NS::Counter`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `name` | `(&self) → Option<String>` | `name` |

---

### `CounterHeap`

C++ equivalent: `NS::CounterHeap`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `count` | `(&self) → UInteger` | `count` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `heap_type` | `(&self) → CounterHeapType` | `type` |
| `invalidate_counter_range` | `(&self, location: UInteger, ...) → void` | `invalidateCounterRange` |
| `label` | `(&self) → Option<String>` | `label` |
| `resolve_counter_range_raw` | `(&self,
        location: UI...) → *mut c_void` | `resolveCounterRange` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |

---

### `CounterHeapDescriptor`

C++ equivalent: `NS::CounterHeapDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `count` | `(&self) → UInteger` | `count` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `heap_type` | `(&self) → CounterHeapType` | `type` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_count` | `(&self, count: UInteger) → void` | `setCount` |
| `set_heap_type` | `(&self, heap_type: CounterHe...) → void` | `setType` |

---

### `CounterSampleBuffer`

C++ equivalent: `NS::CounterSampleBuffer`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `device` | `(&self) → Device` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |
| `resolve_counter_range_raw` | `(&self,
        location: UI...) → Result<*mut c_void, CounterSampleBufferError>` | `resolveCounterRange` |
| `sample_count` | `(&self) → UInteger` | `sampleCount` |

---

### `CounterSampleBufferDescriptor`

C++ equivalent: `NS::CounterSampleBufferDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `counter_set` | `(&self) → Option<CounterSet>` | `counterSet` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |
| `sample_count` | `(&self) → UInteger` | `sampleCount` |
| `storage_mode` | `(&self) → StorageMode` | `storageMode` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_counter_set` | `(&self, counter_set: &Counte...) → void` | `setCounterSet` |
| `set_label` | `(&self, label: &str) → void` | `setLabel` |
| `set_sample_count` | `(&self, count: UInteger) → void` | `setSampleCount` |
| `set_storage_mode` | `(&self, mode: StorageMode) → void` | `setStorageMode` |

---

### `CounterSet`

C++ equivalent: `NS::CounterSet`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `counter_at_index` | `(&self, index: UInteger) → Option<Counter>` | — |
| `counter_count` | `(&self) → UInteger` | — |
| `counters_raw` | `(&self) → *mut c_void` | `counters` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `name` | `(&self) → Option<String>` | `name` |

---

### `DepthStencilDescriptor`

C++ equivalent: `NS::DepthStencilDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `back_face_stencil` | `(&self) → Option<StencilDescriptor>` | `backFaceStencil` |
| `depth_compare_function` | `(&self) → CompareFunction` | `depthCompareFunction` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `front_face_stencil` | `(&self) → Option<StencilDescriptor>` | `frontFaceStencil` |
| `label` | `(&self) → Option<String>` | `label` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_depth_write_enabled` | `(&self) → bool` | `depthWriteEnabled` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_back_face_stencil` | `(&self, stencil: Option<&Ste...) → void` | `setBackFaceStencil` |
| `set_depth_compare_function` | `(&self, func: CompareFunction) → void` | `setDepthCompareFunction` |
| `set_depth_write_enabled` | `(&self, enabled: bool) → void` | `setDepthWriteEnabled` |
| `set_front_face_stencil` | `(&self, stencil: Option<&Ste...) → void` | `setFrontFaceStencil` |
| `set_label` | `(&self, label: &str) → void` | `setLabel` |

---

### `DepthStencilState`

C++ equivalent: `NS::DepthStencilState`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `device` | `(&self) → crate::Device` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `gpu_resource_id` | `(&self) → ResourceID` | `gpuResourceID` |
| `label` | `(&self) → Option<String>` | `label` |

---

### `Device`

C++ equivalent: `NS::Device`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new_acceleration_structure_with_descriptor` | `(&self,
        descriptor: ...) → Option<AccelerationStructure>` | — |
| `new_acceleration_structure_with_size` | `(&self, size: UInteger) → Option<AccelerationStructure>` | `accelerationStructureSizes` |
| `new_archive_with_url` | `(&self,
        url: *const ...) → Result<Archive, metal_foundation::Error>` | — |
| `new_argument_encoder_with_arguments` | `(&self,
        arguments: *...) → Option<ArgumentEncoder>` | `newArgumentEncoder` |
| `new_argument_encoder_with_buffer_binding` | `(&self,
        buffer_bindi...) → Option<ArgumentEncoder>` | — |
| `new_argument_table` | `(&self,
        descriptor: ...) → Result<ArgumentTable, metal_foundation::Error>` | `newArgumentTable` |
| `new_binary_archive` | `(&self,
        descriptor: ...) → Result<BinaryArchive, metal_foundation::Error>` | `newArchive` |
| `new_buffer` | `(&self, length: UInteger, op...) → Option<Buffer>` | `newBuffer` |
| `new_buffer_with_bytes` | `(&self,
        bytes: &[u8]...) → Option<Buffer>` | — |
| `new_buffer_with_bytes_no_copy` | `(&self,
        pointer: *mu...) → Option<Buffer>` | — |
| `new_buffer_with_bytes_ptr` | `(&self,
        pointer: *co...) → Option<Buffer>` | — |
| `new_command_allocator` | `(&self) → Option<CommandAllocator>` | `newCommandAllocator` |
| `new_command_allocator_with_descriptor` | `(&self,
        descriptor: ...) → Result<CommandAllocator, metal_foundation::Error>` | — |
| `new_command_queue` | `(&self) → Option<CommandQueue>` | `newCommandBuffer` |
| `new_command_queue_with_descriptor` | `(&self,
        descriptor: ...) → Option<CommandQueue>` | — |
| `new_command_queue_with_max_command_buffer_count` | `(&self,
        max_command_...) → Option<CommandQueue>` | — |
| `new_compiler` | `(&self,
        descriptor: ...) → Result<Compiler, metal_foundation::Error>` | `newCompiler` |
| `new_compute_pipeline_state_validated` | `(&self,
        descriptor: ...) → Result<ComputePipelineState, ValidationError>` | — |
| `new_compute_pipeline_state_with_descriptor` | `(&self,
        descriptor: ...) → Result<ComputePipelineState, metal_foundation::Error>` | — |
| `new_compute_pipeline_state_with_descriptor_async` | `(&self,
        descriptor: ...) → void` | — |
| `new_compute_pipeline_state_with_function` | `(&self,
        function: &F...) → Result<ComputePipelineState, metal_foundation::Error>` | `newComputePipelineState` |
| `new_compute_pipeline_state_with_function_and_reflection` | `(&self,
        function: &F...) → Result<ComputePipelineState, metal_foundation::Error>` | — |
| `new_compute_pipeline_state_with_function_and_reflection_async` | `(&self,
        function: &F...) → void` | — |
| `new_compute_pipeline_state_with_function_async` | `(&self,
        function: &F...) → void` | — |
| `new_counter_heap` | `(&self,
        descriptor: ...) → Result<CounterHeap, metal_foundation::Error>` | `newCounterHeap` |
| `new_counter_sample_buffer` | `(&self,
        descriptor: ...) → Result<CounterSampleBuffer, metal_foundation::Error>` | `newCounterSampleBuffer` |
| `new_default_library` | `(&self) → Option<Library>` | `newDefaultLibrary` |
| `new_default_library_with_bundle` | `(&self,
        bundle: *con...) → Result<Library, metal_foundation::Error>` | — |
| `new_depth_stencil_state` | `(&self,
        descriptor: ...) → Option<DepthStencilState>` | `newDepthStencilState` |
| `new_depth_stencil_state_with_ptr` | `(&self,
        descriptor: ...) → Option<DepthStencilState>` | — |
| `new_event` | `(&self) → Option<Event>` | `newEvent` |
| `new_fence` | `(&self) → Option<Fence>` | `newFence` |
| `new_heap` | `(&self, descriptor: &HeapDes...) → Option<Heap>` | `newHeap` |
| `new_heap_validated` | `(&self,
        descriptor: ...) → Result<Heap, ValidationError>` | — |
| `new_heap_with_ptr` | `(&self, descriptor: *const c...) → Option<Heap>` | — |
| `new_indirect_command_buffer` | `(&self,
        descriptor: ...) → Option<IndirectCommandBuffer>` | `newIndirectCommandBuffer` |
| `new_io_command_queue` | `(&self,
        descriptor: ...) → Result<IOCommandQueue, metal_foundation::Error>` | `newIOCommandQueue` |
| `new_io_file_handle` | `(&self,
        url: &metal_...) → Result<IOFileHandle, metal_foundation::Error>` | `newIOFileHandle` |
| `new_io_file_handle_with_compression` | `(&self,
        url: &metal_...) → Result<IOFileHandle, metal_foundation::Error>` | — |
| `new_io_handle` | `(&self,
        url: &metal_...) → Result<IOFileHandle, metal_foundation::Error>` | `newIOHandle` |
| `new_io_handle_with_compression` | `(&self,
        url: &metal_...) → Result<IOFileHandle, metal_foundation::Error>` | — |
| `new_library_with_data` | `(&self,
        data: *const...) → Result<Library, metal_foundation::Error>` | — |
| `new_library_with_source` | `(&self,
        source: &str...) → Result<Library, metal_foundation::Error>` | `newLibrary` |
| `new_library_with_source_async` | `(&self,
        source: &str...) → void` | — |
| `new_library_with_stitched_descriptor` | `(&self,
        descriptor: ...) → Result<Library, metal_foundation::Error>` | — |
| `new_library_with_stitched_descriptor_async` | `(&self,
        descriptor: ...) → void` | — |
| `new_library_with_url` | `(&self,
        url: *const ...) → Result<Library, metal_foundation::Error>` | — |
| `new_log_state` | `(&self,
        descriptor: ...) → Result<LogState, metal_foundation::Error>` | `newLogState` |
| `new_mesh_render_pipeline_state_with_reflection_async` | `(&self,
        descriptor: ...) → void` | — |
| `new_mtl4_command_queue` | `(&self) → Option<CommandQueue>` | `newMTL4CommandQueue` |
| `new_mtl4_command_queue_with_descriptor` | `(&self,
        descriptor: ...) → Result<CommandQueue, metal_foundation::Error>` | — |
| `new_pipeline_data_set_serializer` | `(&self,
        descriptor: ...) → Option<PipelineDataSetSerializer>` | `newPipelineDataSetSerializer` |
| `new_rasterization_rate_map` | `(&self,
        descriptor: ...) → Option<RasterizationRateMap>` | `newRasterizationRateMap` |
| `new_render_pipeline_state` | `(&self,
        descriptor: ...) → Result<RenderPipelineState, metal_foundation::Error>` | `newRenderPipelineState` |
| `new_render_pipeline_state_async` | `(&self,
        descriptor: ...) → void` | — |
| `new_render_pipeline_state_with_descriptor` | `(&self,
        descriptor: ...) → Result<RenderPipelineState, ValidationError>` | — |
| `new_render_pipeline_state_with_reflection` | `(&self,
        descriptor: ...) → Result<RenderPipelineState, metal_foundation::Error>` | — |
| `new_render_pipeline_state_with_reflection_async` | `(&self,
        descriptor: ...) → void` | — |
| `new_residency_set` | `(&self,
        descriptor: ...) → Result<ResidencySet, metal_foundation::Error>` | `newResidencySet` |
| `new_sampler_state` | `(&self, descriptor: &Sampler...) → Option<SamplerState>` | `newSamplerState` |
| `new_sampler_state_validated` | `(&self,
        descriptor: ...) → Result<SamplerState, ValidationError>` | — |
| `new_sampler_state_with_ptr` | `(&self,
        descriptor: ...) → Option<SamplerState>` | — |
| `new_shared_event` | `(&self) → Option<SharedEvent>` | `newSharedEvent` |
| `new_shared_event_with_handle` | `(&self, handle: &SharedEvent...) → Option<SharedEvent>` | — |
| `new_shared_texture_with_descriptor` | `(&self,
        descriptor: ...) → Option<Texture>` | — |
| `new_shared_texture_with_handle` | `(&self,
        handle: *con...) → Option<Texture>` | — |
| `new_tensor` | `(&self,
        descriptor: ...) → Result<Tensor, metal_foundation::Error>` | `newTensor` |
| `new_texture` | `(&self, descriptor: *const c...) → Option<Texture>` | `newSharedTexture` |
| `new_texture_view_pool` | `(&self,
        descriptor: ...) → Result<TextureViewPool, metal_foundation::Error>` | `newTextureViewPool` |
| `new_texture_with_descriptor` | `(&self,
        descriptor: ...) → Result<Texture, ValidationError>` | — |
| `new_texture_with_iosurface` | `(&self,
        descriptor: ...) → Option<Texture>` | — |
| `new_tile_render_pipeline_state_with_reflection_async` | `(&self,
        descriptor: ...) → void` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `acceleration_structure_sizes_with_descriptor` | `(&self,
        descriptor: ...) → AccelerationStructureSizes` | — |
| `architecture` | `(&self) → Option<Architecture>` | `architecture` |
| `are_barycentric_coords_supported` | `(&self) → bool` | `areBarycentricCoordsSupported` |
| `are_programmable_sample_positions_supported` | `(&self) → bool` | `areProgrammableSamplePositionsSupported` |
| `are_raster_order_groups_supported` | `(&self) → bool` | `areRasterOrderGroupsSupported` |
| `argument_buffers_support` | `(&self) → ArgumentBuffersTier` | `argumentBuffersSupport` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `barycentric_coords_supported` | `(&self) → bool` | — |
| `convert_sparse_pixel_regions` | `(&self,
        pixel_region...) → void` | `convertSparsePixelRegions` |
| `convert_sparse_tile_regions` | `(&self,
        tile_regions...) → void` | `convertSparseTileRegions` |
| `counter_set_at_index` | `(&self, index: UInteger) → Option<CounterSet>` | — |
| `counter_set_count` | `(&self) → UInteger` | — |
| `counter_sets_raw` | `(&self) → *mut c_void` | `counterSets` |
| `current_allocated_size` | `(&self) → UInteger` | `currentAllocatedSize` |
| `depth24_stencil8_pixel_format_supported` | `(&self) → bool` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function_handle` | `(&self, function: &crate::Fu...) → Option<FunctionHandle>` | — |
| `function_handle_with_binary_function` | `(&self,
        function: &c...) → Option<FunctionHandle>` | `functionHandle` |
| `get_default_sample_positions` | `(&self, positions: &mut [Sam...) → void` | `getDefaultSamplePositions` |
| `headless` | `(&self) → bool` | — |
| `heap_acceleration_structure_size_and_align_with_descriptor` | `(&self,
        descriptor: ...) → SizeAndAlign` | — |
| `heap_acceleration_structure_size_and_align_with_size` | `(&self,
        size: UInteger,) → SizeAndAlign` | `heapAccelerationStructureSizeAndAlign` |
| `heap_buffer_size_and_align` | `(&self,
        length: UInt...) → SizeAndAlign` | `heapBufferSizeAndAlign` |
| `heap_texture_size_and_align` | `(&self, descriptor: *const c...) → SizeAndAlign` | `heapTextureSizeAndAlign` |
| `location` | `(&self) → DeviceLocation` | `location` |
| `location_number` | `(&self) → UInteger` | `locationNumber` |
| `low_power` | `(&self) → bool` | — |
| `max_argument_buffer_sampler_count` | `(&self) → UInteger` | `maxArgumentBufferSamplerCount` |
| `max_buffer_length` | `(&self) → UInteger` | `maxBufferLength` |
| `max_threadgroup_memory_length` | `(&self) → UInteger` | `maxThreadgroupMemoryLength` |
| `max_threads_per_threadgroup` | `(&self) → Size` | `maxThreadsPerThreadgroup` |
| `max_transfer_rate` | `(&self) → u64` | `maxTransferRate` |
| `maximum_concurrent_compilation_task_count` | `(&self) → UInteger` | `maximumConcurrentCompilationTaskCount` |
| `minimum_linear_texture_alignment_for_pixel_format` | `(&self, format: PixelFormat) → UInteger` | `minimumLinearTextureAlignmentForPixelFormat` |
| `minimum_texture_buffer_alignment_for_pixel_format` | `(&self, format: PixelFormat) → UInteger` | `minimumTextureBufferAlignmentForPixelFormat` |
| `name` | `(&self) → &str` | `name` |
| `peer_count` | `(&self) → u32` | `peerCount` |
| `peer_group_id` | `(&self) → u64` | `peerGroupID` |
| `peer_index` | `(&self) → u32` | `peerIndex` |
| `programmable_sample_positions_supported` | `(&self) → bool` | — |
| `query_timestamp_frequency` | `(&self) → u64` | `queryTimestampFrequency` |
| `raster_order_groups_supported` | `(&self) → bool` | — |
| `read_write_texture_support` | `(&self) → ReadWriteTextureTier` | `readWriteTextureSupport` |
| `recommended_max_working_set_size` | `(&self) → u64` | `recommendedMaxWorkingSetSize` |
| `registry_id` | `(&self) → u64` | `registryID` |
| `removable` | `(&self) → bool` | — |
| `sample_timestamps` | `(&self) → (u64, u64)` | `sampleTimestamps` |
| `should_maximize_concurrent_compilation` | `(&self) → bool` | `shouldMaximizeConcurrentCompilation` |
| `size_of_counter_heap_entry` | `(&self, heap_type: CounterHe...) → metal_foundation::UInteger` | `sizeOfCounterHeapEntry` |
| `sparse_tile_size` | `(&self,
        texture_type...) → Size` | `sparseTileSize` |
| `sparse_tile_size_in_bytes` | `(&self) → UInteger` | `sparseTileSizeInBytes` |
| `sparse_tile_size_in_bytes_for_page_size` | `(&self, page_size: SparsePag...) → UInteger` | — |
| `sparse_tile_size_with_page_size` | `(&self,
        texture_type...) → Size` | — |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `has_unified_memory` | `(&self) → bool` | `hasUnifiedMemory` |
| `is_depth24_stencil8_pixel_format_supported` | `(&self) → bool` | `depth24Stencil8PixelFormatSupported` |
| `is_headless` | `(&self) → bool` | `headless` |
| `is_low_power` | `(&self) → bool` | `isLowPower` |
| `is_removable` | `(&self) → bool` | `isRemovable` |
| `supports_32bit_float_filtering` | `(&self) → bool` | `supports32BitFloatFiltering` |
| `supports_32bit_msaa` | `(&self) → bool` | `supports32BitMSAA` |
| `supports_bc_texture_compression` | `(&self) → bool` | `supportsBCTextureCompression` |
| `supports_counter_sampling` | `(&self, sampling_point: Coun...) → bool` | `supportsCounterSampling` |
| `supports_dynamic_libraries` | `(&self) → bool` | `supportsDynamicLibraries` |
| `supports_family` | `(&self, gpu_family: GPUFamily) → bool` | `supportsFamily` |
| `supports_feature_set` | `(&self, feature_set: FeatureSet) → bool` | `supportsFeatureSet` |
| `supports_function_pointers` | `(&self) → bool` | `supportsFunctionPointers` |
| `supports_function_pointers_from_render` | `(&self) → bool` | `supportsFunctionPointersFromRender` |
| `supports_primitive_motion_blur` | `(&self) → bool` | `supportsPrimitiveMotionBlur` |
| `supports_pull_model_interpolation` | `(&self) → bool` | `supportsPullModelInterpolation` |
| `supports_query_texture_lod` | `(&self) → bool` | `supportsQueryTextureLOD` |
| `supports_rasterization_rate_map` | `(&self, layer_count: UInteger) → bool` | `supportsRasterizationRateMap` |
| `supports_raytracing` | `(&self) → bool` | `supportsRaytracing` |
| `supports_raytracing_from_render` | `(&self) → bool` | `supportsRaytracingFromRender` |
| `supports_render_dynamic_libraries` | `(&self) → bool` | `supportsRenderDynamicLibraries` |
| `supports_shader_barycentric_coordinates` | `(&self) → bool` | `supportsShaderBarycentricCoordinates` |
| `supports_texture_sample_count` | `(&self, sample_count: UInteger) → bool` | `supportsTextureSampleCount` |
| `supports_vertex_amplification_count` | `(&self, count: UInteger) → bool` | `supportsVertexAmplificationCount` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_should_maximize_concurrent_compilation` | `(&self, value: bool) → void` | `setShouldMaximizeConcurrentCompilation` |

---

### `Drawable`

C++ equivalent: `NS::Drawable`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `add_presented_handler` | `(&self, handler: F) → void` | `addPresentedHandler` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `drawable_id` | `(&self) → UInteger` | `drawableID` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `present` | `(&self) → void` | `present` |
| `present_after_minimum_duration` | `(&self, duration: TimeInterval) → void` | `presentAfterMinimumDuration` |
| `present_at_time` | `(&self, presentation_time: T...) → void` | `presentAtTime` |
| `presented_time` | `(&self) → TimeInterval` | `presentedTime` |

---

### `DynamicLibrary`

C++ equivalent: `NS::DynamicLibrary`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `device` | `(&self) → crate::Device` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `install_name` | `(&self) → Option<String>` | `installName` |
| `label` | `(&self) → Option<String>` | `label` |
| `serialize_to_url` | `(&self, url: &metal_foundati...) → Result<(), metal_foundation::Error>` | `serializeToURL` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |

---

### `Event`

C++ equivalent: `NS::Event`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `device` | `(&self) → crate::Device` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |

---

### `Fence`

C++ equivalent: `NS::Fence`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `device` | `(&self) → crate::Device` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |

---

### `Function`

C++ equivalent: `NS::Function`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new_argument_encoder` | `(&self, buffer_index: UInteger) → Option<crate::ArgumentEncoder>` | `newArgumentEncoder` |
| `new_argument_encoder_with_reflection` | `(&self,
        buffer_index...) → Option<crate::ArgumentEncoder>` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `device` | `(&self) → crate::Device` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function_constants_dictionary_raw` | `(&self) → *mut c_void` | `functionConstantsDictionary` |
| `function_type` | `(&self) → FunctionType` | `functionType` |
| `label` | `(&self) → Option<String>` | `label` |
| `name` | `(&self) → Option<String>` | `name` |
| `options` | `(&self) → FunctionOptions` | `options` |
| `patch_control_point_count` | `(&self) → metal_foundation::Integer` | `patchControlPointCount` |
| `patch_type` | `(&self) → PatchType` | `patchType` |
| `stage_input_attributes_raw` | `(&self) → *mut c_void` | `stageInputAttributes` |
| `vertex_attributes_raw` | `(&self) → *mut c_void` | `vertexAttributes` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |

---

### `FunctionConstant`

C++ equivalent: `NS::FunctionConstant`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `constant_type` | `(&self) → DataType` | `type` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index` | `(&self) → UInteger` | `index` |
| `name` | `(&self) → Option<String>` | `name` |
| `required` | `(&self) → bool` | `required` |

---

### `FunctionConstantValues`

C++ equivalent: `NS::FunctionConstantValues`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `reset` | `(&self) → void` | `reset` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_bool_at_index` | `(&self, value: bool, index: ...) → void` | — |
| `set_constant_value_at_index` | `(&self,
        value: *cons...) → void` | — |
| `set_constant_value_with_name` | `(&self,
        value: *cons...) → void` | — |
| `set_constant_values` | `(&self,
        values: *con...) → void` | `setConstantValue` |
| `set_float_at_index` | `(&self, value: f32, index: U...) → void` | — |
| `set_int_at_index` | `(&self, value: i32, index: U...) → void` | — |
| `set_uint_at_index` | `(&self, value: u32, index: U...) → void` | — |

---

### `FunctionDescriptor`

C++ equivalent: `NS::FunctionDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `binary_archives_raw` | `(&self) → *mut c_void` | — |
| `constant_values` | `(&self) → Option<FunctionConstantValues>` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function_descriptor` | `() → Option<Self>` | — |
| `name` | `(&self) → Option<String>` | — |
| `options` | `(&self) → FunctionOptions` | — |
| `specialized_name` | `(&self) → Option<String>` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_binary_archives_raw` | `(&self, archives: *const c_void) → void` | — |
| `set_constant_values` | `(&self, values: Option<&Func...) → void` | — |
| `set_name` | `(&self, name: &str) → void` | — |
| `set_options` | `(&self, options: FunctionOpt...) → void` | — |
| `set_specialized_name` | `(&self, name: &str) → void` | — |

---

### `FunctionDescriptor`

C++ equivalent: `NS::FunctionDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `binary_archives_raw` | `(&self) → *mut c_void` | `binaryArchives` |
| `constant_values` | `(&self) → Option<FunctionConstantValues>` | `constantValues` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function_descriptor` | `() → Option<Self>` | — |
| `name` | `(&self) → Option<String>` | `name` |
| `options` | `(&self) → FunctionOptions` | `options` |
| `specialized_name` | `(&self) → Option<String>` | `specializedName` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_binary_archives_raw` | `(&self, archives: *const c_void) → void` | `setBinaryArchives` |
| `set_constant_values` | `(&self, values: Option<&Func...) → void` | `setConstantValues` |
| `set_name` | `(&self, name: &str) → void` | `setName` |
| `set_options` | `(&self, options: FunctionOpt...) → void` | `setOptions` |
| `set_specialized_name` | `(&self, name: &str) → void` | `setSpecializedName` |

---

### `FunctionHandle`

C++ equivalent: `NS::FunctionHandle`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `device` | `(&self) → crate::Device` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function_type` | `(&self) → FunctionType` | `functionType` |
| `gpu_resource_id` | `(&self) → ResourceID` | `gpuResourceID` |
| `name` | `(&self) → Option<String>` | `name` |

---

### `FunctionLog`

C++ equivalent: `NS::FunctionLog`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `debug_location` | `(&self) → Option<FunctionLogDebugLocation>` | `debugLocation` |
| `encoder_label` | `(&self) → Option<String>` | `encoderLabel` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function` | `(&self) → Option<Function>` | `function` |
| `log_type` | `(&self) → FunctionLogType` | `type` |

---

### `FunctionLogDebugLocation`

C++ equivalent: `NS::FunctionLogDebugLocation`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `column` | `(&self) → UInteger` | `column` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function_name` | `(&self) → Option<String>` | `functionName` |
| `line` | `(&self) → UInteger` | `line` |
| `url` | `(&self) → Option<String>` | `URL` |

---

### `FunctionReflection`

C++ equivalent: `NS::FunctionReflection`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `bindings_raw` | `(&self) → *mut c_void` | `bindings` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |

---

### `FunctionStitchingAttribute`

C++ equivalent: `NS::FunctionStitchingAttribute`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |

---

### `FunctionStitchingAttributeAlwaysInline`

C++ equivalent: `NS::FunctionStitchingAttributeAlwaysInline`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |

---

### `FunctionStitchingFunctionNode`

C++ equivalent: `NS::FunctionStitchingFunctionNode`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `arguments_ptr` | `(&self) → *const c_void` | `arguments` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `control_dependencies_ptr` | `(&self) → *const c_void` | `controlDependencies` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `name` | `(&self) → Option<String>` | `name` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_arguments_ptr` | `(&self, arguments: *const c_...) → void` | `setArguments` |
| `set_control_dependencies_ptr` | `(&self, control_dependencies...) → void` | `setControlDependencies` |
| `set_name` | `(&self, name: &str) → void` | `setName` |

---

### `FunctionStitchingGraph`

C++ equivalent: `NS::FunctionStitchingGraph`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `attributes_ptr` | `(&self) → *const c_void` | `attributes` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function_name` | `(&self) → Option<String>` | `functionName` |
| `nodes_ptr` | `(&self) → *const c_void` | `nodes` |
| `output_node` | `(&self) → Option<FunctionStitchingFunctionNode>` | `outputNode` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_attributes_ptr` | `(&self, attributes: *const c...) → void` | `setAttributes` |
| `set_function_name` | `(&self, name: &str) → void` | `setFunctionName` |
| `set_nodes_ptr` | `(&self, nodes: *const c_void) → void` | `setNodes` |
| `set_output_node` | `(&self, output_node: &Functi...) → void` | `setOutputNode` |

---

### `FunctionStitchingInputNode`

C++ equivalent: `NS::FunctionStitchingInputNode`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `argument_index` | `(&self) → UInteger` | `argumentIndex` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `with_argument_index` | `(argument_index: UInteger) → Option<Self>` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_argument_index` | `(&self, argument_index: UInt...) → void` | `setArgumentIndex` |

---

### `FunctionStitchingNode`

C++ equivalent: `NS::FunctionStitchingNode`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |

---

### `Heap`

C++ equivalent: `NS::Heap`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new_acceleration_structure` | `(&self,
        descriptor: ...) → Option<crate::acceleration::AccelerationStructure>` | — |
| `new_acceleration_structure_with_offset` | `(&self,
        descriptor: ...) → Option<crate::acceleration::AccelerationStructure>` | — |
| `new_acceleration_structure_with_size` | `(&self,
        size: UInteger,) → Option<crate::acceleration::AccelerationStructure>` | `newAccelerationStructure` |
| `new_acceleration_structure_with_size_and_offset` | `(&self,
        size: UInteg...) → Option<crate::acceleration::AccelerationStructure>` | — |
| `new_buffer` | `(&self,
        length: UInt...) → Option<crate::buffer::Buffer>` | `newBuffer` |
| `new_buffer_with_offset` | `(&self,
        length: UInt...) → Option<crate::buffer::Buffer>` | — |
| `new_texture` | `(&self, descriptor: *const c...) → Option<crate::texture::Texture>` | `newTexture` |
| `new_texture_with_offset` | `(&self,
        descriptor: ...) → Option<crate::texture::Texture>` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `cpu_cache_mode` | `(&self) → CPUCacheMode` | `cpuCacheMode` |
| `current_allocated_size` | `(&self) → UInteger` | `currentAllocatedSize` |
| `device` | `(&self) → crate::Device` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `hazard_tracking_mode` | `(&self) → HazardTrackingMode` | `hazardTrackingMode` |
| `heap_type` | `(&self) → HeapType` | `type` |
| `label` | `(&self) → Option<String>` | `label` |
| `max_available_size` | `(&self, alignment: UInteger) → UInteger` | `maxAvailableSize` |
| `resource_options` | `(&self) → ResourceOptions` | `resourceOptions` |
| `size` | `(&self) → UInteger` | `size` |
| `storage_mode` | `(&self) → StorageMode` | `storageMode` |
| `used_size` | `(&self) → UInteger` | `usedSize` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |
| `set_purgeable_state` | `(&self, state: PurgeableState) → PurgeableState` | `setPurgeableState` |

---

### `HeapDescriptor`

C++ equivalent: `NS::HeapDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `cpu_cache_mode` | `(&self) → CPUCacheMode` | `cpuCacheMode` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `hazard_tracking_mode` | `(&self) → HazardTrackingMode` | `hazardTrackingMode` |
| `heap_type` | `(&self) → HeapType` | `type` |
| `max_compatible_placement_sparse_page_size` | `(&self) → SparsePageSize` | `maxCompatiblePlacementSparsePageSize` |
| `resource_options` | `(&self) → ResourceOptions` | `resourceOptions` |
| `size` | `(&self) → UInteger` | `size` |
| `sparse_page_size` | `(&self) → SparsePageSize` | `sparsePageSize` |
| `storage_mode` | `(&self) → StorageMode` | `storageMode` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_cpu_cache_mode` | `(&self, mode: CPUCacheMode) → void` | `setCpuCacheMode` |
| `set_hazard_tracking_mode` | `(&self, mode: HazardTracking...) → void` | `setHazardTrackingMode` |
| `set_heap_type` | `(&self, heap_type: HeapType) → void` | `setType` |
| `set_max_compatible_placement_sparse_page_size` | `(&self, size: SparsePageSize) → void` | `setMaxCompatiblePlacementSparsePageSize` |
| `set_resource_options` | `(&self, options: ResourceOpt...) → void` | `setResourceOptions` |
| `set_size` | `(&self, size: UInteger) → void` | `setSize` |
| `set_sparse_page_size` | `(&self, size: SparsePageSize) → void` | `setSparsePageSize` |
| `set_storage_mode` | `(&self, mode: StorageMode) → void` | `setStorageMode` |

---

### `IOCommandBuffer`

C++ equivalent: `NS::IOCommandBuffer`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `add_barrier` | `(&self) → void` | `addBarrier` |
| `add_completed_handler` | `(&self, handler: F) → void` | `addCompletedHandler` |
| `add_completed_handler_ptr` | `(&self, block: *const c_void) → void` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `commit` | `(&self) → void` | `commit` |
| `copy_status_to_buffer` | `(&self, buffer: &Buffer, off...) → void` | `copyStatusToBuffer` |
| `enqueue` | `(&self) → void` | `enqueue` |
| `error` | `(&self) → Option<metal_foundation::Error>` | `error` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |
| `load_buffer` | `(&self,
        buffer: &Buf...) → void` | `loadBuffer` |
| `load_bytes` | `(&self,
        pointer: *mu...) → void` | `loadBytes` |
| `load_texture` | `(&self,
        texture: &Te...) → void` | `loadTexture` |
| `pop_debug_group` | `(&self) → void` | `popDebugGroup` |
| `push_debug_group` | `(&self, name: &str) → void` | `pushDebugGroup` |
| `signal_event` | `(&self, event: &SharedEvent,...) → void` | `signalEvent` |
| `status` | `(&self) → IOStatus` | `status` |
| `try_cancel` | `(&self) → void` | `tryCancel` |
| `wait` | `(&self, event: &SharedEvent,...) → void` | `wait` |
| `wait_until_completed` | `(&self) → void` | `waitUntilCompleted` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |

---

### `IOCommandQueue`

C++ equivalent: `NS::IOCommandQueue`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `command_buffer` | `(&self) → Option<IOCommandBuffer>` | `commandBuffer` |
| `command_buffer_with_unretained_references` | `(&self) → Option<IOCommandBuffer>` | `commandBufferWithUnretainedReferences` |
| `enqueue_barrier` | `(&self) → void` | `enqueueBarrier` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |

---

### `IOCommandQueueDescriptor`

C++ equivalent: `NS::IOCommandQueueDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `max_command_buffer_count` | `(&self) → UInteger` | `maxCommandBufferCount` |
| `max_commands_in_flight` | `(&self) → UInteger` | `maxCommandsInFlight` |
| `priority` | `(&self) → IOPriority` | `priority` |
| `queue_type` | `(&self) → IOCommandQueueType` | `type` |
| `scratch_buffer_allocator_ptr` | `(&self) → *const c_void` | `scratchBufferAllocator` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_max_command_buffer_count` | `(&self, count: UInteger) → void` | `setMaxCommandBufferCount` |
| `set_max_commands_in_flight` | `(&self, count: UInteger) → void` | `setMaxCommandsInFlight` |
| `set_priority` | `(&self, priority: IOPriority) → void` | `setPriority` |
| `set_queue_type` | `(&self, queue_type: IOComman...) → void` | `setType` |
| `set_scratch_buffer_allocator_ptr` | `(&self, allocator: *const c_...) → void` | `setScratchBufferAllocator` |

---

### `IOFileHandle`

C++ equivalent: `NS::IOFileHandle`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |

---

### `IOScratchBuffer`

C++ equivalent: `NS::IOScratchBuffer`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `buffer` | `(&self) → Option<Buffer>` | `buffer` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |

---

### `IOScratchBufferAllocator`

C++ equivalent: `NS::IOScratchBufferAllocator`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new_scratch_buffer` | `(&self, minimum_size: UInteger) → Option<IOScratchBuffer>` | `newScratchBuffer` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |

---

### `IndirectCommandBuffer`

C++ equivalent: `NS::IndirectCommandBuffer`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `gpu_resource_id` | `(&self) → ResourceID` | `gpuResourceID` |
| `indirect_compute_command` | `(&self, index: UInteger) → Option<IndirectComputeCommand>` | `indirectComputeCommand` |
| `indirect_render_command` | `(&self, index: UInteger) → Option<IndirectRenderCommand>` | `indirectRenderCommand` |
| `reset` | `(&self, location: UInteger, ...) → void` | `reset` |
| `size` | `(&self) → UInteger` | `size` |

---

### `IndirectCommandBufferDescriptor`

C++ equivalent: `NS::IndirectCommandBufferDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `command_types` | `(&self) → IndirectCommandType` | `commandTypes` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `inherit_buffers` | `(&self) → bool` | `inheritBuffers` |
| `inherit_cull_mode` | `(&self) → bool` | `inheritCullMode` |
| `inherit_depth_bias` | `(&self) → bool` | `inheritDepthBias` |
| `inherit_depth_clip_mode` | `(&self) → bool` | `inheritDepthClipMode` |
| `inherit_depth_stencil_state` | `(&self) → bool` | `inheritDepthStencilState` |
| `inherit_front_facing_winding` | `(&self) → bool` | `inheritFrontFacingWinding` |
| `inherit_pipeline_state` | `(&self) → bool` | `inheritPipelineState` |
| `inherit_triangle_fill_mode` | `(&self) → bool` | `inheritTriangleFillMode` |
| `max_fragment_buffer_bind_count` | `(&self) → UInteger` | `maxFragmentBufferBindCount` |
| `max_kernel_buffer_bind_count` | `(&self) → UInteger` | `maxKernelBufferBindCount` |
| `max_kernel_threadgroup_memory_bind_count` | `(&self) → UInteger` | `maxKernelThreadgroupMemoryBindCount` |
| `max_mesh_buffer_bind_count` | `(&self) → UInteger` | `maxMeshBufferBindCount` |
| `max_object_buffer_bind_count` | `(&self) → UInteger` | `maxObjectBufferBindCount` |
| `max_object_threadgroup_memory_bind_count` | `(&self) → UInteger` | `maxObjectThreadgroupMemoryBindCount` |
| `max_vertex_buffer_bind_count` | `(&self) → UInteger` | `maxVertexBufferBindCount` |
| `support_color_attachment_mapping` | `(&self) → bool` | `supportColorAttachmentMapping` |
| `support_dynamic_attribute_stride` | `(&self) → bool` | `supportDynamicAttributeStride` |
| `support_ray_tracing` | `(&self) → bool` | `supportRayTracing` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_command_types` | `(&self, types: IndirectComma...) → void` | `setCommandTypes` |
| `set_inherit_buffers` | `(&self, inherit: bool) → void` | `setInheritBuffers` |
| `set_inherit_cull_mode` | `(&self, inherit: bool) → void` | `setInheritCullMode` |
| `set_inherit_depth_bias` | `(&self, inherit: bool) → void` | `setInheritDepthBias` |
| `set_inherit_depth_clip_mode` | `(&self, inherit: bool) → void` | `setInheritDepthClipMode` |
| `set_inherit_depth_stencil_state` | `(&self, inherit: bool) → void` | `setInheritDepthStencilState` |
| `set_inherit_front_facing_winding` | `(&self, inherit: bool) → void` | `setInheritFrontFacingWinding` |
| `set_inherit_pipeline_state` | `(&self, inherit: bool) → void` | `setInheritPipelineState` |
| `set_inherit_triangle_fill_mode` | `(&self, inherit: bool) → void` | `setInheritTriangleFillMode` |
| `set_max_fragment_buffer_bind_count` | `(&self, count: UInteger) → void` | `setMaxFragmentBufferBindCount` |
| `set_max_kernel_buffer_bind_count` | `(&self, count: UInteger) → void` | `setMaxKernelBufferBindCount` |
| `set_max_kernel_threadgroup_memory_bind_count` | `(&self, count: UInteger) → void` | `setMaxKernelThreadgroupMemoryBindCount` |
| `set_max_mesh_buffer_bind_count` | `(&self, count: UInteger) → void` | `setMaxMeshBufferBindCount` |
| `set_max_object_buffer_bind_count` | `(&self, count: UInteger) → void` | `setMaxObjectBufferBindCount` |
| `set_max_object_threadgroup_memory_bind_count` | `(&self, count: UInteger) → void` | `setMaxObjectThreadgroupMemoryBindCount` |
| `set_max_vertex_buffer_bind_count` | `(&self, count: UInteger) → void` | `setMaxVertexBufferBindCount` |
| `set_support_color_attachment_mapping` | `(&self, support: bool) → void` | `setSupportColorAttachmentMapping` |
| `set_support_dynamic_attribute_stride` | `(&self, support: bool) → void` | `setSupportDynamicAttributeStride` |
| `set_support_ray_tracing` | `(&self, support: bool) → void` | `setSupportRayTracing` |

---

### `IndirectComputeCommand`

C++ equivalent: `NS::IndirectComputeCommand`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `clear_barrier` | `(&self) → void` | `clearBarrier` |
| `concurrent_dispatch_threadgroups` | `(&self,
        threadgroups...) → void` | `concurrentDispatchThreadgroups` |
| `concurrent_dispatch_threads` | `(&self,
        threads_per_...) → void` | `concurrentDispatchThreads` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `reset` | `(&self) → void` | `reset` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_barrier` | `(&self) → void` | `setBarrier` |
| `set_compute_pipeline_state` | `(&self, state: &ComputePipel...) → void` | `setComputePipelineState` |
| `set_imageblock_width` | `(&self, width: UInteger, hei...) → void` | `setImageblockWidth` |
| `set_kernel_buffer` | `(&self, buffer: &Buffer, off...) → void` | `setKernelBuffer` |
| `set_kernel_buffer_with_stride` | `(&self,
        buffer: &Buf...) → void` | — |
| `set_stage_in_region` | `(&self, region: Region) → void` | `setStageInRegion` |
| `set_threadgroup_memory_length` | `(&self, length: UInteger, in...) → void` | `setThreadgroupMemoryLength` |

---

### `IndirectInstanceAccelerationStructureDescriptor`

C++ equivalent: `NS::IndirectInstanceAccelerationStructureDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `instance_count_buffer` | `(&self) → Option<Buffer>` | `instanceCountBuffer` |
| `instance_count_buffer_offset` | `(&self) → UInteger` | — |
| `instance_descriptor_buffer` | `(&self) → Option<Buffer>` | `instanceDescriptorBuffer` |
| `instance_descriptor_buffer_offset` | `(&self) → UInteger` | — |
| `instance_descriptor_stride` | `(&self) → UInteger` | `instanceDescriptorStride` |
| `instance_descriptor_type` | `(&self) → AccelerationStructureInstanceDescriptorType` | `instanceDescriptorType` |
| `instance_transformation_matrix_layout` | `(&self) → MatrixLayout` | `instanceTransformationMatrixLayout` |
| `max_instance_count` | `(&self) → UInteger` | `maxInstanceCount` |
| `max_motion_transform_count` | `(&self) → UInteger` | `maxMotionTransformCount` |
| `motion_transform_buffer` | `(&self) → Option<Buffer>` | `motionTransformBuffer` |
| `motion_transform_buffer_offset` | `(&self) → UInteger` | — |
| `motion_transform_count_buffer` | `(&self) → Option<Buffer>` | `motionTransformCountBuffer` |
| `motion_transform_count_buffer_offset` | `(&self) → UInteger` | — |
| `motion_transform_stride` | `(&self) → UInteger` | `motionTransformStride` |
| `motion_transform_type` | `(&self) → TransformType` | `motionTransformType` |
| `usage` | `(&self) → AccelerationStructureUsage` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_instance_count_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setInstanceCountBuffer` |
| `set_instance_count_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_instance_descriptor_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setInstanceDescriptorBuffer` |
| `set_instance_descriptor_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_instance_descriptor_stride` | `(&self, stride: UInteger) → void` | `setInstanceDescriptorStride` |
| `set_instance_descriptor_type` | `(&self,
        descriptor_t...) → void` | `setInstanceDescriptorType` |
| `set_instance_transformation_matrix_layout` | `(&self, layout: MatrixLayout) → void` | `setInstanceTransformationMatrixLayout` |
| `set_max_instance_count` | `(&self, count: UInteger) → void` | `setMaxInstanceCount` |
| `set_max_motion_transform_count` | `(&self, count: UInteger) → void` | `setMaxMotionTransformCount` |
| `set_motion_transform_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setMotionTransformBuffer` |
| `set_motion_transform_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_motion_transform_count_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setMotionTransformCountBuffer` |
| `set_motion_transform_count_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_motion_transform_stride` | `(&self, stride: UInteger) → void` | `setMotionTransformStride` |
| `set_motion_transform_type` | `(&self, transform_type: Tran...) → void` | `setMotionTransformType` |
| `set_usage` | `(&self, usage: AccelerationS...) → void` | — |

---

### `IndirectInstanceAccelerationStructureDescriptor`

C++ equivalent: `NS::IndirectInstanceAccelerationStructureDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `instance_count_buffer` | `(&self) → Option<Buffer>` | `instanceCountBuffer` |
| `instance_count_buffer_offset` | `(&self) → UInteger` | `instanceCountBufferOffset` |
| `instance_descriptor_buffer` | `(&self) → Option<Buffer>` | `instanceDescriptorBuffer` |
| `instance_descriptor_buffer_offset` | `(&self) → UInteger` | `instanceDescriptorBufferOffset` |
| `instance_descriptor_stride` | `(&self) → UInteger` | `instanceDescriptorStride` |
| `instance_descriptor_type` | `(&self) → AccelerationStructureInstanceDescriptorType` | `instanceDescriptorType` |
| `instance_transformation_matrix_layout` | `(&self) → MatrixLayout` | `instanceTransformationMatrixLayout` |
| `max_instance_count` | `(&self) → UInteger` | `maxInstanceCount` |
| `max_motion_transform_count` | `(&self) → UInteger` | `maxMotionTransformCount` |
| `motion_transform_buffer` | `(&self) → Option<Buffer>` | `motionTransformBuffer` |
| `motion_transform_buffer_offset` | `(&self) → UInteger` | `motionTransformBufferOffset` |
| `motion_transform_count_buffer` | `(&self) → Option<Buffer>` | `motionTransformCountBuffer` |
| `motion_transform_count_buffer_offset` | `(&self) → UInteger` | `motionTransformCountBufferOffset` |
| `motion_transform_stride` | `(&self) → UInteger` | `motionTransformStride` |
| `motion_transform_type` | `(&self) → TransformType` | `motionTransformType` |
| `usage` | `(&self) → AccelerationStructureUsage` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_instance_count_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setInstanceCountBuffer` |
| `set_instance_count_buffer_offset` | `(&self, offset: UInteger) → void` | `setInstanceCountBufferOffset` |
| `set_instance_descriptor_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setInstanceDescriptorBuffer` |
| `set_instance_descriptor_buffer_offset` | `(&self, offset: UInteger) → void` | `setInstanceDescriptorBufferOffset` |
| `set_instance_descriptor_stride` | `(&self, stride: UInteger) → void` | `setInstanceDescriptorStride` |
| `set_instance_descriptor_type` | `(&self,
        descriptor_t...) → void` | `setInstanceDescriptorType` |
| `set_instance_transformation_matrix_layout` | `(&self, layout: MatrixLayout) → void` | `setInstanceTransformationMatrixLayout` |
| `set_max_instance_count` | `(&self, count: UInteger) → void` | `setMaxInstanceCount` |
| `set_max_motion_transform_count` | `(&self, count: UInteger) → void` | `setMaxMotionTransformCount` |
| `set_motion_transform_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setMotionTransformBuffer` |
| `set_motion_transform_buffer_offset` | `(&self, offset: UInteger) → void` | `setMotionTransformBufferOffset` |
| `set_motion_transform_count_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setMotionTransformCountBuffer` |
| `set_motion_transform_count_buffer_offset` | `(&self, offset: UInteger) → void` | `setMotionTransformCountBufferOffset` |
| `set_motion_transform_stride` | `(&self, stride: UInteger) → void` | `setMotionTransformStride` |
| `set_motion_transform_type` | `(&self, transform_type: Tran...) → void` | `setMotionTransformType` |
| `set_usage` | `(&self, usage: AccelerationS...) → void` | — |

---

### `IndirectRenderCommand`

C++ equivalent: `NS::IndirectRenderCommand`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `clear_barrier` | `(&self) → void` | `clearBarrier` |
| `draw_indexed_primitives` | `(&self,
        primitive_ty...) → void` | `drawIndexedPatches` |
| `draw_mesh_threadgroups` | `(&self,
        threadgroups...) → void` | `drawMeshThreadgroups` |
| `draw_mesh_threads` | `(&self,
        threads_per_...) → void` | `drawMeshThreads` |
| `draw_primitives` | `(&self,
        primitive_ty...) → void` | `drawPatches` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `reset` | `(&self) → void` | `reset` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_barrier` | `(&self) → void` | `setBarrier` |
| `set_cull_mode` | `(&self, mode: CullMode) → void` | `setCullMode` |
| `set_depth_bias` | `(&self, depth_bias: f32, slo...) → void` | `setDepthBias` |
| `set_depth_clip_mode` | `(&self, mode: DepthClipMode) → void` | `setDepthClipMode` |
| `set_depth_stencil_state` | `(&self, state: &DepthStencil...) → void` | `setDepthStencilState` |
| `set_fragment_buffer` | `(&self, buffer: &Buffer, off...) → void` | `setFragmentBuffer` |
| `set_front_facing_winding` | `(&self, winding: Winding) → void` | `setFrontFacingWinding` |
| `set_mesh_buffer` | `(&self, buffer: &Buffer, off...) → void` | `setMeshBuffer` |
| `set_object_buffer` | `(&self, buffer: &Buffer, off...) → void` | `setObjectBuffer` |
| `set_object_threadgroup_memory_length` | `(&self, length: UInteger, in...) → void` | `setObjectThreadgroupMemoryLength` |
| `set_render_pipeline_state` | `(&self, state: &RenderPipeli...) → void` | `setRenderPipelineState` |
| `set_triangle_fill_mode` | `(&self, mode: TriangleFillMode) → void` | `setTriangleFillMode` |
| `set_vertex_buffer` | `(&self, buffer: &Buffer, off...) → void` | `setVertexBuffer` |
| `set_vertex_buffer_with_stride` | `(&self,
        buffer: &Buf...) → void` | — |

---

### `InstanceAccelerationStructureDescriptor`

C++ equivalent: `NS::InstanceAccelerationStructureDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `instance_count` | `(&self) → UInteger` | `instanceCount` |
| `instance_descriptor_buffer` | `(&self) → Option<Buffer>` | `instanceDescriptorBuffer` |
| `instance_descriptor_buffer_offset` | `(&self) → UInteger` | — |
| `instance_descriptor_stride` | `(&self) → UInteger` | `instanceDescriptorStride` |
| `instance_descriptor_type` | `(&self) → AccelerationStructureInstanceDescriptorType` | `instanceDescriptorType` |
| `instance_transformation_matrix_layout` | `(&self) → MatrixLayout` | `instanceTransformationMatrixLayout` |
| `instanced_acceleration_structures_ptr` | `(&self) → *const c_void` | — |
| `motion_transform_buffer` | `(&self) → Option<Buffer>` | `motionTransformBuffer` |
| `motion_transform_buffer_offset` | `(&self) → UInteger` | — |
| `motion_transform_count` | `(&self) → UInteger` | `motionTransformCount` |
| `motion_transform_stride` | `(&self) → UInteger` | `motionTransformStride` |
| `motion_transform_type` | `(&self) → TransformType` | `motionTransformType` |
| `usage` | `(&self) → AccelerationStructureUsage` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_instance_count` | `(&self, count: UInteger) → void` | `setInstanceCount` |
| `set_instance_descriptor_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setInstanceDescriptorBuffer` |
| `set_instance_descriptor_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_instance_descriptor_stride` | `(&self, stride: UInteger) → void` | `setInstanceDescriptorStride` |
| `set_instance_descriptor_type` | `(&self,
        descriptor_t...) → void` | `setInstanceDescriptorType` |
| `set_instance_transformation_matrix_layout` | `(&self, layout: MatrixLayout) → void` | `setInstanceTransformationMatrixLayout` |
| `set_instanced_acceleration_structures_ptr` | `(&self,
        instanced_ac...) → void` | — |
| `set_motion_transform_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setMotionTransformBuffer` |
| `set_motion_transform_buffer_offset` | `(&self, offset: UInteger) → void` | — |
| `set_motion_transform_count` | `(&self, count: UInteger) → void` | `setMotionTransformCount` |
| `set_motion_transform_stride` | `(&self, stride: UInteger) → void` | `setMotionTransformStride` |
| `set_motion_transform_type` | `(&self, transform_type: Tran...) → void` | `setMotionTransformType` |
| `set_usage` | `(&self, usage: AccelerationS...) → void` | — |

---

### `InstanceAccelerationStructureDescriptor`

C++ equivalent: `NS::InstanceAccelerationStructureDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `instance_count` | `(&self) → UInteger` | `instanceCount` |
| `instance_descriptor_buffer` | `(&self) → Option<Buffer>` | `instanceDescriptorBuffer` |
| `instance_descriptor_buffer_offset` | `(&self) → UInteger` | `instanceDescriptorBufferOffset` |
| `instance_descriptor_stride` | `(&self) → UInteger` | `instanceDescriptorStride` |
| `instance_descriptor_type` | `(&self) → AccelerationStructureInstanceDescriptorType` | `instanceDescriptorType` |
| `instance_transformation_matrix_layout` | `(&self) → MatrixLayout` | `instanceTransformationMatrixLayout` |
| `instanced_acceleration_structures_ptr` | `(&self) → *const c_void` | `instancedAccelerationStructures` |
| `motion_transform_buffer` | `(&self) → Option<Buffer>` | `motionTransformBuffer` |
| `motion_transform_buffer_offset` | `(&self) → UInteger` | `motionTransformBufferOffset` |
| `motion_transform_count` | `(&self) → UInteger` | `motionTransformCount` |
| `motion_transform_stride` | `(&self) → UInteger` | `motionTransformStride` |
| `motion_transform_type` | `(&self) → TransformType` | `motionTransformType` |
| `usage` | `(&self) → AccelerationStructureUsage` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_instance_count` | `(&self, count: UInteger) → void` | `setInstanceCount` |
| `set_instance_descriptor_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setInstanceDescriptorBuffer` |
| `set_instance_descriptor_buffer_offset` | `(&self, offset: UInteger) → void` | `setInstanceDescriptorBufferOffset` |
| `set_instance_descriptor_stride` | `(&self, stride: UInteger) → void` | `setInstanceDescriptorStride` |
| `set_instance_descriptor_type` | `(&self,
        descriptor_t...) → void` | `setInstanceDescriptorType` |
| `set_instance_transformation_matrix_layout` | `(&self, layout: MatrixLayout) → void` | `setInstanceTransformationMatrixLayout` |
| `set_instanced_acceleration_structures_ptr` | `(&self,
        instanced_ac...) → void` | `setInstancedAccelerationStructures` |
| `set_motion_transform_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setMotionTransformBuffer` |
| `set_motion_transform_buffer_offset` | `(&self, offset: UInteger) → void` | `setMotionTransformBufferOffset` |
| `set_motion_transform_count` | `(&self, count: UInteger) → void` | `setMotionTransformCount` |
| `set_motion_transform_stride` | `(&self, stride: UInteger) → void` | `setMotionTransformStride` |
| `set_motion_transform_type` | `(&self, transform_type: Tran...) → void` | `setMotionTransformType` |
| `set_usage` | `(&self, usage: AccelerationS...) → void` | — |

---

### `IntersectionFunctionDescriptor`

C++ equivalent: `NS::IntersectionFunctionDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `binary_archives_raw` | `(&self) → *mut c_void` | — |
| `constant_values` | `(&self) → Option<FunctionConstantValues>` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `name` | `(&self) → Option<String>` | — |
| `options` | `(&self) → FunctionOptions` | — |
| `specialized_name` | `(&self) → Option<String>` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_binary_archives_raw` | `(&self, archives: *const c_void) → void` | — |
| `set_constant_values` | `(&self, values: Option<&Func...) → void` | — |
| `set_name` | `(&self, name: &str) → void` | — |
| `set_options` | `(&self, options: FunctionOpt...) → void` | — |
| `set_specialized_name` | `(&self, name: &str) → void` | — |

---

### `IntersectionFunctionTable`

C++ equivalent: `NS::IntersectionFunctionTable`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `gpu_resource_id` | `(&self) → ResourceID` | `gpuResourceID` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_buffer` | `(&self, buffer: &Buffer, off...) → void` | `setBuffer` |
| `set_buffers` | `(&self, buffers: &[&Buffer],...) → void` | `setBuffers` |
| `set_function` | `(&self, function: &FunctionH...) → void` | `setFunction` |
| `set_functions` | `(&self, functions: &[&Functi...) → void` | `setFunctions` |
| `set_opaque_curve_intersection_function_at_index` | `(&self,
        signature: I...) → void` | `setOpaqueCurveIntersectionFunction` |
| `set_opaque_curve_intersection_function_with_range` | `(&self,
        signature: I...) → void` | — |
| `set_opaque_triangle_intersection_function_at_index` | `(&self,
        signature: I...) → void` | `setOpaqueTriangleIntersectionFunction` |
| `set_opaque_triangle_intersection_function_with_range` | `(&self,
        signature: I...) → void` | — |
| `set_visible_function_table` | `(&self,
        function_tab...) → void` | `setVisibleFunctionTable` |
| `set_visible_function_tables` | `(&self,
        function_tab...) → void` | `setVisibleFunctionTables` |

---

### `IntersectionFunctionTableDescriptor`

C++ equivalent: `NS::IntersectionFunctionTableDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function_count` | `(&self) → UInteger` | `functionCount` |
| `intersection_function_table_descriptor` | `() → Option<Self>` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_function_count` | `(&self, function_count: UInt...) → void` | `setFunctionCount` |

---

### `Library`

C++ equivalent: `NS::Library`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new_function_with_descriptor` | `(&self,
        descriptor: ...) → Result<Function, metal_foundation::Error>` | — |
| `new_function_with_descriptor_async` | `(&self,
        descriptor: ...) → void` | — |
| `new_function_with_name` | `(&self, name: &str) → Option<Function>` | `newFunction` |
| `new_function_with_name_and_constants` | `(&self,
        name: &str,
...) → Result<Function, metal_foundation::Error>` | — |
| `new_function_with_name_and_constants_async` | `(&self,
        name: &str,
...) → void` | — |
| `new_intersection_function` | `(&self,
        descriptor: ...) → Result<Function, metal_foundation::Error>` | `newIntersectionFunction` |
| `new_intersection_function_async` | `(&self,
        descriptor: ...) → void` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `device` | `(&self) → crate::Device` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function_names` | `(&self) → Vec<String>` | `functionNames` |
| `install_name` | `(&self) → Option<String>` | `installName` |
| `label` | `(&self) → Option<String>` | `label` |
| `library_type` | `(&self) → LibraryType` | `type` |
| `reflection_for_function` | `(&self, name: &str) → Option<FunctionReflection>` | `reflectionForFunction` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |

---

### `LibraryDescriptor`

C++ equivalent: `NS::LibraryDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `name` | `(&self) → Option<String>` | `name` |
| `options` | `(&self) → Option<CompileOptions>` | `options` |
| `source` | `(&self) → Option<String>` | `source` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_name` | `(&self, name: &str) → void` | `setName` |
| `set_options` | `(&self, options: &CompileOpt...) → void` | `setOptions` |
| `set_source` | `(&self, source: &str) → void` | `setSource` |

---

### `LibraryFunctionDescriptor`

C++ equivalent: `NS::LibraryFunctionDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `library` | `(&self) → Option<Library>` | `library` |
| `name` | `(&self) → Option<String>` | `name` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_library` | `(&self, library: &Library) → void` | `setLibrary` |
| `set_name` | `(&self, name: &str) → void` | `setName` |

---

### `LinkedFunctions`

C++ equivalent: `NS::LinkedFunctions`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `binary_functions_raw` | `(&self) → *mut c_void` | `binaryFunctions` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `functions_raw` | `(&self) → *mut c_void` | `functions` |
| `groups_raw` | `(&self) → *mut c_void` | `groups` |
| `linked_functions` | `() → Option<Self>` | `linkedFunctions` |
| `private_functions_raw` | `(&self) → *mut c_void` | `privateFunctions` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_binary_functions_raw` | `(&self, functions: *const c_...) → void` | `setBinaryFunctions` |
| `set_functions_raw` | `(&self, functions: *const c_...) → void` | `setFunctions` |
| `set_groups_raw` | `(&self, groups: *const c_void) → void` | `setGroups` |
| `set_private_functions_raw` | `(&self, functions: *const c_...) → void` | `setPrivateFunctions` |

---

### `LogContainer`

C++ equivalent: `NS::LogContainer`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |

---

### `LogState`

C++ equivalent: `NS::LogState`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `add_log_handler` | `(&self, handler: F) → void` | `addLogHandler` |
| `add_log_handler_raw` | `(&self, block: *const c_void) → void` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |

---

### `LogStateDescriptor`

C++ equivalent: `NS::LogStateDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `buffer_size` | `(&self) → UInteger` | `bufferSize` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `level` | `(&self) → LogLevel` | `level` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_buffer_size` | `(&self, size: UInteger) → void` | `setBufferSize` |
| `set_level` | `(&self, level: LogLevel) → void` | `setLevel` |

---

### `LogicalToPhysicalColorAttachmentMap`

C++ equivalent: `NS::LogicalToPhysicalColorAttachmentMap`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `physical_index` | `(&self, logical_index: UInteger) → UInteger` | `getPhysicalIndex` |
| `reset` | `(&self) → void` | `reset` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_physical_index` | `(&self, physical_index: UInt...) → void` | `setPhysicalIndex` |

---

### `MachineLearningCommandEncoder`

C++ equivalent: `NS::MachineLearningCommandEncoder`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `dispatch_network` | `(&self, intermediates_heap: ...) → void` | `dispatchNetwork` |
| `end_encoding` | `(&self) → void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_argument_table` | `(&self, argument_table: &Arg...) → void` | `setArgumentTable` |
| `set_pipeline_state` | `(&self, pipeline_state: &Mac...) → void` | `setPipelineState` |

---

### `MachineLearningPipelineDescriptor`

C++ equivalent: `NS::MachineLearningPipelineDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `input_dimensions_at_buffer_index_raw` | `(&self, buffer_index: Integer) → *mut c_void` | `inputDimensionsAtBufferIndex` |
| `label` | `(&self) → Option<String>` | `label` |
| `machine_learning_function_descriptor` | `(&self) → Option<FunctionDescriptor>` | `machineLearningFunctionDescriptor` |
| `reset` | `(&self) → void` | `reset` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_input_dimensions_raw` | `(&self, dimensions: *const c...) → void` | `setInputDimensions` |
| `set_label` | `(&self, label: &str) → void` | `setLabel` |
| `set_machine_learning_function_descriptor` | `(&self, descriptor: &Functio...) → void` | `setMachineLearningFunctionDescriptor` |

---

### `MachineLearningPipelineReflection`

C++ equivalent: `NS::MachineLearningPipelineReflection`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `bindings_raw` | `(&self) → *mut c_void` | `bindings` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |

---

### `MachineLearningPipelineState`

C++ equivalent: `NS::MachineLearningPipelineState`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `device` | `(&self) → Option<Device>` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `intermediates_heap_size` | `(&self) → UInteger` | `intermediatesHeapSize` |
| `label` | `(&self) → Option<String>` | `label` |
| `reflection` | `(&self) → Option<MachineLearningPipelineReflection>` | `reflection` |

---

### `MeshRenderPipelineDescriptor`

C++ equivalent: `NS::MeshRenderPipelineDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `alpha_to_coverage_state` | `(&self) → AlphaToCoverageState` | `alphaToCoverageState` |
| `alpha_to_one_state` | `(&self) → AlphaToOneState` | `alphaToOneState` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `binary_archives_raw` | `(&self) → *mut c_void` | — |
| `color_attachment_mapping_state` | `(&self) → LogicalToPhysicalColorAttachmentMappingState` | `colorAttachmentMappingState` |
| `color_attachments` | `(&self) → Option<RenderPipelineColorAttachmentDescriptorArray>` | `colorAttachments` |
| `depth_attachment_pixel_format` | `(&self) → PixelFormat` | — |
| `fragment_buffers` | `(&self) → Option<PipelineBufferDescriptorArray>` | — |
| `fragment_function` | `(&self) → Option<crate::Function>` | — |
| `fragment_function_descriptor` | `(&self) → Option<FunctionDescriptor>` | `fragmentFunctionDescriptor` |
| `fragment_linked_functions` | `(&self) → Option<crate::LinkedFunctions>` | — |
| `fragment_static_linking_descriptor` | `(&self) → Option<StaticLinkingDescriptor>` | `fragmentStaticLinkingDescriptor` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | — |
| `max_total_threadgroups_per_mesh_grid` | `(&self) → UInteger` | `maxTotalThreadgroupsPerMeshGrid` |
| `max_total_threads_per_mesh_threadgroup` | `(&self) → UInteger` | `maxTotalThreadsPerMeshThreadgroup` |
| `max_total_threads_per_object_threadgroup` | `(&self) → UInteger` | `maxTotalThreadsPerObjectThreadgroup` |
| `max_vertex_amplification_count` | `(&self) → UInteger` | `maxVertexAmplificationCount` |
| `mesh_buffers` | `(&self) → Option<PipelineBufferDescriptorArray>` | — |
| `mesh_function` | `(&self) → Option<crate::Function>` | — |
| `mesh_function_descriptor` | `(&self) → Option<FunctionDescriptor>` | `meshFunctionDescriptor` |
| `mesh_linked_functions` | `(&self) → Option<crate::LinkedFunctions>` | — |
| `mesh_static_linking_descriptor` | `(&self) → Option<StaticLinkingDescriptor>` | `meshStaticLinkingDescriptor` |
| `mesh_threadgroup_size_is_multiple_of_thread_execution_width` | `(&self) → bool` | `meshThreadgroupSizeIsMultipleOfThreadExecutionWidth` |
| `object_buffers` | `(&self) → Option<PipelineBufferDescriptorArray>` | — |
| `object_function` | `(&self) → Option<crate::Function>` | — |
| `object_function_descriptor` | `(&self) → Option<FunctionDescriptor>` | `objectFunctionDescriptor` |
| `object_linked_functions` | `(&self) → Option<crate::LinkedFunctions>` | — |
| `object_static_linking_descriptor` | `(&self) → Option<StaticLinkingDescriptor>` | `objectStaticLinkingDescriptor` |
| `object_threadgroup_size_is_multiple_of_thread_execution_width` | `(&self) → bool` | `objectThreadgroupSizeIsMultipleOfThreadExecutionWidth` |
| `options` | `(&self) → Option<PipelineOptions>` | — |
| `payload_memory_length` | `(&self) → UInteger` | `payloadMemoryLength` |
| `raster_sample_count` | `(&self) → UInteger` | `rasterSampleCount` |
| `required_threads_per_mesh_threadgroup` | `(&self) → Size` | `requiredThreadsPerMeshThreadgroup` |
| `required_threads_per_object_threadgroup` | `(&self) → Size` | `requiredThreadsPerObjectThreadgroup` |
| `reset` | `(&self) → void` | `reset` |
| `shader_validation` | `(&self) → ShaderValidation` | — |
| `stencil_attachment_pixel_format` | `(&self) → PixelFormat` | — |
| `support_fragment_binary_linking` | `(&self) → bool` | `supportFragmentBinaryLinking` |
| `support_indirect_command_buffers` | `(&self) → IndirectCommandBufferSupportState` | `supportIndirectCommandBuffers` |
| `support_mesh_binary_linking` | `(&self) → bool` | `supportMeshBinaryLinking` |
| `support_object_binary_linking` | `(&self) → bool` | `supportObjectBinaryLinking` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_alpha_to_coverage_enabled` | `(&self) → bool` | — |
| `is_alpha_to_one_enabled` | `(&self) → bool` | — |
| `is_rasterization_enabled` | `(&self) → bool` | `isRasterizationEnabled` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_alpha_to_coverage_enabled` | `(&self, enabled: bool) → void` | — |
| `set_alpha_to_coverage_state` | `(&self, state: AlphaToCovera...) → void` | `setAlphaToCoverageState` |
| `set_alpha_to_one_enabled` | `(&self, enabled: bool) → void` | — |
| `set_alpha_to_one_state` | `(&self, state: AlphaToOneState) → void` | `setAlphaToOneState` |
| `set_binary_archives_raw` | `(&self, archives: *const c_void) → void` | — |
| `set_color_attachment_mapping_state` | `(&self,
        state: Logic...) → void` | `setColorAttachmentMappingState` |
| `set_depth_attachment_pixel_format` | `(&self, format: PixelFormat) → void` | — |
| `set_fragment_function` | `(&self, function: Option<&cr...) → void` | — |
| `set_fragment_function_descriptor` | `(&self, descriptor: &Functio...) → void` | `setFragmentFunctionDescriptor` |
| `set_fragment_linked_functions` | `(&self, functions: Option<&c...) → void` | — |
| `set_fragment_static_linking_descriptor` | `(&self, descriptor: &StaticL...) → void` | `setFragmentStaticLinkingDescriptor` |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_max_total_threadgroups_per_mesh_grid` | `(&self, max: UInteger) → void` | `setMaxTotalThreadgroupsPerMeshGrid` |
| `set_max_total_threads_per_mesh_threadgroup` | `(&self, max: UInteger) → void` | `setMaxTotalThreadsPerMeshThreadgroup` |
| `set_max_total_threads_per_object_threadgroup` | `(&self, max: UInteger) → void` | `setMaxTotalThreadsPerObjectThreadgroup` |
| `set_max_vertex_amplification_count` | `(&self, count: UInteger) → void` | `setMaxVertexAmplificationCount` |
| `set_mesh_function` | `(&self, function: Option<&cr...) → void` | — |
| `set_mesh_function_descriptor` | `(&self, descriptor: &Functio...) → void` | `setMeshFunctionDescriptor` |
| `set_mesh_linked_functions` | `(&self, functions: Option<&c...) → void` | — |
| `set_mesh_static_linking_descriptor` | `(&self, descriptor: &StaticL...) → void` | `setMeshStaticLinkingDescriptor` |
| `set_mesh_threadgroup_size_is_multiple_of_thread_execution_width` | `(&self, value: bool) → void` | `setMeshThreadgroupSizeIsMultipleOfThreadExecutionWidth` |
| `set_object_function` | `(&self, function: Option<&cr...) → void` | — |
| `set_object_function_descriptor` | `(&self, descriptor: &Functio...) → void` | `setObjectFunctionDescriptor` |
| `set_object_linked_functions` | `(&self, functions: Option<&c...) → void` | — |
| `set_object_static_linking_descriptor` | `(&self, descriptor: &StaticL...) → void` | `setObjectStaticLinkingDescriptor` |
| `set_object_threadgroup_size_is_multiple_of_thread_execution_width` | `(&self, value: bool) → void` | `setObjectThreadgroupSizeIsMultipleOfThreadExecutionWidth` |
| `set_options` | `(&self, options: &PipelineOp...) → void` | — |
| `set_payload_memory_length` | `(&self, length: UInteger) → void` | `setPayloadMemoryLength` |
| `set_raster_sample_count` | `(&self, count: UInteger) → void` | `setRasterSampleCount` |
| `set_rasterization_enabled` | `(&self, enabled: bool) → void` | `setRasterizationEnabled` |
| `set_required_threads_per_mesh_threadgroup` | `(&self, size: Size) → void` | `setRequiredThreadsPerMeshThreadgroup` |
| `set_required_threads_per_object_threadgroup` | `(&self, size: Size) → void` | `setRequiredThreadsPerObjectThreadgroup` |
| `set_shader_validation` | `(&self, validation: ShaderVa...) → void` | — |
| `set_stencil_attachment_pixel_format` | `(&self, format: PixelFormat) → void` | — |
| `set_support_fragment_binary_linking` | `(&self, support: bool) → void` | `setSupportFragmentBinaryLinking` |
| `set_support_indirect_command_buffers` | `(&self, state: IndirectComma...) → void` | `setSupportIndirectCommandBuffers` |
| `set_support_mesh_binary_linking` | `(&self, support: bool) → void` | `setSupportMeshBinaryLinking` |
| `set_support_object_binary_linking` | `(&self, support: bool) → void` | `setSupportObjectBinaryLinking` |

---

### `MeshRenderPipelineDescriptor`

C++ equivalent: `NS::MeshRenderPipelineDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `alpha_to_coverage_state` | `(&self) → AlphaToCoverageState` | — |
| `alpha_to_one_state` | `(&self) → AlphaToOneState` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `binary_archives_raw` | `(&self) → *mut c_void` | `binaryArchives` |
| `color_attachment_mapping_state` | `(&self) → LogicalToPhysicalColorAttachmentMappingState` | — |
| `color_attachments` | `(&self) → Option<RenderPipelineColorAttachmentDescriptorArray>` | `colorAttachments` |
| `depth_attachment_pixel_format` | `(&self) → PixelFormat` | `depthAttachmentPixelFormat` |
| `fragment_buffers` | `(&self) → Option<PipelineBufferDescriptorArray>` | `fragmentBuffers` |
| `fragment_function` | `(&self) → Option<crate::Function>` | `fragmentFunction` |
| `fragment_function_descriptor` | `(&self) → Option<FunctionDescriptor>` | — |
| `fragment_linked_functions` | `(&self) → Option<crate::LinkedFunctions>` | `fragmentLinkedFunctions` |
| `fragment_static_linking_descriptor` | `(&self) → Option<StaticLinkingDescriptor>` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |
| `max_total_threadgroups_per_mesh_grid` | `(&self) → UInteger` | `maxTotalThreadgroupsPerMeshGrid` |
| `max_total_threads_per_mesh_threadgroup` | `(&self) → UInteger` | `maxTotalThreadsPerMeshThreadgroup` |
| `max_total_threads_per_object_threadgroup` | `(&self) → UInteger` | `maxTotalThreadsPerObjectThreadgroup` |
| `max_vertex_amplification_count` | `(&self) → UInteger` | `maxVertexAmplificationCount` |
| `mesh_buffers` | `(&self) → Option<PipelineBufferDescriptorArray>` | `meshBuffers` |
| `mesh_function` | `(&self) → Option<crate::Function>` | `meshFunction` |
| `mesh_function_descriptor` | `(&self) → Option<FunctionDescriptor>` | — |
| `mesh_linked_functions` | `(&self) → Option<crate::LinkedFunctions>` | `meshLinkedFunctions` |
| `mesh_static_linking_descriptor` | `(&self) → Option<StaticLinkingDescriptor>` | — |
| `mesh_threadgroup_size_is_multiple_of_thread_execution_width` | `(&self) → bool` | `meshThreadgroupSizeIsMultipleOfThreadExecutionWidth` |
| `object_buffers` | `(&self) → Option<PipelineBufferDescriptorArray>` | `objectBuffers` |
| `object_function` | `(&self) → Option<crate::Function>` | `objectFunction` |
| `object_function_descriptor` | `(&self) → Option<FunctionDescriptor>` | — |
| `object_linked_functions` | `(&self) → Option<crate::LinkedFunctions>` | `objectLinkedFunctions` |
| `object_static_linking_descriptor` | `(&self) → Option<StaticLinkingDescriptor>` | — |
| `object_threadgroup_size_is_multiple_of_thread_execution_width` | `(&self) → bool` | `objectThreadgroupSizeIsMultipleOfThreadExecutionWidth` |
| `options` | `(&self) → Option<PipelineOptions>` | — |
| `payload_memory_length` | `(&self) → UInteger` | `payloadMemoryLength` |
| `raster_sample_count` | `(&self) → UInteger` | `rasterSampleCount` |
| `required_threads_per_mesh_threadgroup` | `(&self) → Size` | `requiredThreadsPerMeshThreadgroup` |
| `required_threads_per_object_threadgroup` | `(&self) → Size` | `requiredThreadsPerObjectThreadgroup` |
| `reset` | `(&self) → void` | `reset` |
| `shader_validation` | `(&self) → ShaderValidation` | `shaderValidation` |
| `stencil_attachment_pixel_format` | `(&self) → PixelFormat` | `stencilAttachmentPixelFormat` |
| `support_fragment_binary_linking` | `(&self) → bool` | — |
| `support_indirect_command_buffers` | `(&self) → IndirectCommandBufferSupportState` | `supportIndirectCommandBuffers` |
| `support_mesh_binary_linking` | `(&self) → bool` | — |
| `support_object_binary_linking` | `(&self) → bool` | — |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_alpha_to_coverage_enabled` | `(&self) → bool` | `alphaToCoverageEnabled` |
| `is_alpha_to_one_enabled` | `(&self) → bool` | `alphaToOneEnabled` |
| `is_rasterization_enabled` | `(&self) → bool` | `isRasterizationEnabled` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_alpha_to_coverage_enabled` | `(&self, enabled: bool) → void` | `setAlphaToCoverageEnabled` |
| `set_alpha_to_coverage_state` | `(&self, state: AlphaToCovera...) → void` | — |
| `set_alpha_to_one_enabled` | `(&self, enabled: bool) → void` | `setAlphaToOneEnabled` |
| `set_alpha_to_one_state` | `(&self, state: AlphaToOneState) → void` | — |
| `set_binary_archives_raw` | `(&self, archives: *const c_void) → void` | `setBinaryArchives` |
| `set_color_attachment_mapping_state` | `(&self,
        state: Logic...) → void` | — |
| `set_depth_attachment_pixel_format` | `(&self, format: PixelFormat) → void` | `setDepthAttachmentPixelFormat` |
| `set_fragment_function` | `(&self, function: Option<&cr...) → void` | `setFragmentFunction` |
| `set_fragment_function_descriptor` | `(&self, descriptor: &Functio...) → void` | — |
| `set_fragment_linked_functions` | `(&self, functions: Option<&c...) → void` | `setFragmentLinkedFunctions` |
| `set_fragment_static_linking_descriptor` | `(&self, descriptor: &StaticL...) → void` | — |
| `set_label` | `(&self, label: &str) → void` | `setLabel` |
| `set_max_total_threadgroups_per_mesh_grid` | `(&self, max: UInteger) → void` | `setMaxTotalThreadgroupsPerMeshGrid` |
| `set_max_total_threads_per_mesh_threadgroup` | `(&self, max: UInteger) → void` | `setMaxTotalThreadsPerMeshThreadgroup` |
| `set_max_total_threads_per_object_threadgroup` | `(&self, max: UInteger) → void` | `setMaxTotalThreadsPerObjectThreadgroup` |
| `set_max_vertex_amplification_count` | `(&self, count: UInteger) → void` | `setMaxVertexAmplificationCount` |
| `set_mesh_function` | `(&self, function: Option<&cr...) → void` | `setMeshFunction` |
| `set_mesh_function_descriptor` | `(&self, descriptor: &Functio...) → void` | — |
| `set_mesh_linked_functions` | `(&self, functions: Option<&c...) → void` | `setMeshLinkedFunctions` |
| `set_mesh_static_linking_descriptor` | `(&self, descriptor: &StaticL...) → void` | — |
| `set_mesh_threadgroup_size_is_multiple_of_thread_execution_width` | `(&self, value: bool) → void` | `setMeshThreadgroupSizeIsMultipleOfThreadExecutionWidth` |
| `set_object_function` | `(&self, function: Option<&cr...) → void` | `setObjectFunction` |
| `set_object_function_descriptor` | `(&self, descriptor: &Functio...) → void` | — |
| `set_object_linked_functions` | `(&self, functions: Option<&c...) → void` | `setObjectLinkedFunctions` |
| `set_object_static_linking_descriptor` | `(&self, descriptor: &StaticL...) → void` | — |
| `set_object_threadgroup_size_is_multiple_of_thread_execution_width` | `(&self, value: bool) → void` | `setObjectThreadgroupSizeIsMultipleOfThreadExecutionWidth` |
| `set_options` | `(&self, options: &PipelineOp...) → void` | — |
| `set_payload_memory_length` | `(&self, length: UInteger) → void` | `setPayloadMemoryLength` |
| `set_raster_sample_count` | `(&self, count: UInteger) → void` | `setRasterSampleCount` |
| `set_rasterization_enabled` | `(&self, enabled: bool) → void` | `setRasterizationEnabled` |
| `set_required_threads_per_mesh_threadgroup` | `(&self, size: Size) → void` | `setRequiredThreadsPerMeshThreadgroup` |
| `set_required_threads_per_object_threadgroup` | `(&self, size: Size) → void` | `setRequiredThreadsPerObjectThreadgroup` |
| `set_shader_validation` | `(&self, validation: ShaderVa...) → void` | `setShaderValidation` |
| `set_stencil_attachment_pixel_format` | `(&self, format: PixelFormat) → void` | `setStencilAttachmentPixelFormat` |
| `set_support_fragment_binary_linking` | `(&self, support: bool) → void` | — |
| `set_support_indirect_command_buffers` | `(&self, state: IndirectComma...) → void` | `setSupportIndirectCommandBuffers` |
| `set_support_mesh_binary_linking` | `(&self, support: bool) → void` | — |
| `set_support_object_binary_linking` | `(&self, support: bool) → void` | — |

---

### `MotionKeyframeData`

C++ equivalent: `NS::MotionKeyframeData`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `buffer` | `(&self) → Option<Buffer>` | `buffer` |
| `data` | `() → Option<Self>` | `data` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `offset` | `(&self) → UInteger` | `offset` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_buffer` | `(&self, buffer: Option<&Buffer>) → void` | `setBuffer` |
| `set_offset` | `(&self, offset: UInteger) → void` | `setOffset` |

---

### `ObjectPayloadBinding`

C++ equivalent: `NS::ObjectPayloadBinding`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `access` | `(&self) → BindingAccess` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `binding_type` | `(&self) → BindingType` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index` | `(&self) → UInteger` | — |
| `name` | `(&self) → Option<String>` | — |
| `object_payload_alignment` | `(&self) → UInteger` | `objectPayloadAlignment` |
| `object_payload_data_size` | `(&self) → UInteger` | `objectPayloadDataSize` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_argument` | `(&self) → bool` | — |
| `is_used` | `(&self) → bool` | — |

---

### `ParallelRenderCommandEncoder`

C++ equivalent: `NS::ParallelRenderCommandEncoder`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `barrier_after_queue_stages` | `(&self,
        after_stages...) → void` | — |
| `command_buffer` | `(&self) → crate::CommandBuffer` | — |
| `device` | `(&self) → crate::Device` | — |
| `end_encoding` | `(&self) → void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `insert_debug_signpost` | `(&self, string: &str) → void` | — |
| `label` | `(&self) → Option<String>` | — |
| `pop_debug_group` | `(&self) → void` | — |
| `push_debug_group` | `(&self, string: &str) → void` | — |
| `render_command_encoder` | `(&self) → Option<crate::RenderCommandEncoder>` | `renderCommandEncoder` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_color_store_action` | `(&self, store_action: StoreA...) → void` | `setColorStoreAction` |
| `set_color_store_action_options` | `(&self,
        store_action...) → void` | `setColorStoreActionOptions` |
| `set_depth_store_action` | `(&self, store_action: StoreA...) → void` | `setDepthStoreAction` |
| `set_depth_store_action_options` | `(&self, store_action_options...) → void` | `setDepthStoreActionOptions` |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_stencil_store_action` | `(&self, store_action: StoreA...) → void` | `setStencilStoreAction` |
| `set_stencil_store_action_options` | `(&self, store_action_options...) → void` | `setStencilStoreActionOptions` |

---

### `PipelineBufferDescriptor`

C++ equivalent: `NS::PipelineBufferDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `mutability` | `(&self) → crate::enums::Mutability` | `mutability` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_mutability` | `(&self, mutability: crate::e...) → void` | `setMutability` |

---

### `PipelineBufferDescriptorArray`

C++ equivalent: `NS::PipelineBufferDescriptorArray`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `object` | `(&self, buffer_index: UInteger) → Option<PipelineBufferDescriptor>` | `object` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_object` | `(&self, buffer: &PipelineBuf...) → void` | `setObject` |

---

### `PipelineDataSetSerializer`

C++ equivalent: `NS::PipelineDataSetSerializer`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `serialize_as_archive_and_flush_to_url` | `(&self,
        url: *const ...) → Result<bool, metal_foundation::Error>` | `serializeAsArchiveAndFlushToURL` |
| `serialize_as_pipelines_script` | `(&self) → Result<*mut c_void, metal_foundation::Error>` | `serializeAsPipelinesScript` |

---

### `PipelineDataSetSerializerDescriptor`

C++ equivalent: `NS::PipelineDataSetSerializerDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `configuration` | `(&self) → PipelineDataSetSerializerConfiguration` | `configuration` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_configuration` | `(&self, configuration: Pipel...) → void` | `setConfiguration` |

---

### `PipelineDescriptor`

C++ equivalent: `NS::PipelineDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |
| `options` | `(&self) → Option<PipelineOptions>` | `options` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |
| `set_options` | `(&self, options: &PipelineOp...) → void` | `setOptions` |

---

### `PipelineOptions`

C++ equivalent: `NS::PipelineOptions`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `shader_reflection` | `(&self) → ShaderReflection` | `shaderReflection` |
| `shader_validation` | `(&self) → ShaderValidation` | `shaderValidation` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_shader_reflection` | `(&self, reflection: ShaderRe...) → void` | `setShaderReflection` |
| `set_shader_validation` | `(&self, validation: ShaderVa...) → void` | `setShaderValidation` |

---

### `PipelineStageDynamicLinkingDescriptor`

C++ equivalent: `NS::PipelineStageDynamicLinkingDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `binary_linked_functions_raw` | `(&self) → *mut c_void` | `binaryLinkedFunctions` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `max_call_stack_depth` | `(&self) → UInteger` | `maxCallStackDepth` |
| `preloaded_libraries_raw` | `(&self) → *mut c_void` | `preloadedLibraries` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_binary_linked_functions_raw` | `(&self, functions: *const c_...) → void` | `setBinaryLinkedFunctions` |
| `set_max_call_stack_depth` | `(&self, depth: UInteger) → void` | `setMaxCallStackDepth` |
| `set_preloaded_libraries_raw` | `(&self, libraries: *const c_...) → void` | `setPreloadedLibraries` |

---

### `PointerType`

C++ equivalent: `NS::PointerType`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `access` | `(&self) → BindingAccess` | `access` |
| `alignment` | `(&self) → UInteger` | `alignment` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `data_size` | `(&self) → UInteger` | `dataSize` |
| `data_type` | `(&self) → DataType` | — |
| `element_array_type` | `(&self) → Option<ArrayType>` | `elementArrayType` |
| `element_is_argument_buffer` | `(&self) → bool` | `elementIsArgumentBuffer` |
| `element_struct_type` | `(&self) → Option<StructType>` | `elementStructType` |
| `element_type` | `(&self) → DataType` | `elementType` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |

---

### `PrimitiveAccelerationStructureDescriptor`

C++ equivalent: `NS::PrimitiveAccelerationStructureDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `geometry_descriptors_ptr` | `(&self) → *const c_void` | — |
| `geometry_descriptors_raw` | `(&self) → *mut c_void` | `geometryDescriptors` |
| `motion_end_border_mode` | `(&self) → MotionBorderMode` | `motionEndBorderMode` |
| `motion_end_time` | `(&self) → f32` | `motionEndTime` |
| `motion_keyframe_count` | `(&self) → UInteger` | `motionKeyframeCount` |
| `motion_start_border_mode` | `(&self) → MotionBorderMode` | `motionStartBorderMode` |
| `motion_start_time` | `(&self) → f32` | `motionStartTime` |
| `usage` | `(&self) → AccelerationStructureUsage` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_geometry_descriptors_ptr` | `(&self, geometry_descriptors...) → void` | — |
| `set_geometry_descriptors_raw` | `(&self, descriptors: *const ...) → void` | `setGeometryDescriptors` |
| `set_motion_end_border_mode` | `(&self, mode: MotionBorderMode) → void` | `setMotionEndBorderMode` |
| `set_motion_end_time` | `(&self, time: f32) → void` | `setMotionEndTime` |
| `set_motion_keyframe_count` | `(&self, count: UInteger) → void` | `setMotionKeyframeCount` |
| `set_motion_start_border_mode` | `(&self, mode: MotionBorderMode) → void` | `setMotionStartBorderMode` |
| `set_motion_start_time` | `(&self, time: f32) → void` | `setMotionStartTime` |
| `set_usage` | `(&self, usage: AccelerationS...) → void` | — |

---

### `PrimitiveAccelerationStructureDescriptor`

C++ equivalent: `NS::PrimitiveAccelerationStructureDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `geometry_descriptors_ptr` | `(&self) → *const c_void` | — |
| `geometry_descriptors_raw` | `(&self) → *mut c_void` | `geometryDescriptors` |
| `motion_end_border_mode` | `(&self) → MotionBorderMode` | `motionEndBorderMode` |
| `motion_end_time` | `(&self) → f32` | `motionEndTime` |
| `motion_keyframe_count` | `(&self) → UInteger` | `motionKeyframeCount` |
| `motion_start_border_mode` | `(&self) → MotionBorderMode` | `motionStartBorderMode` |
| `motion_start_time` | `(&self) → f32` | `motionStartTime` |
| `usage` | `(&self) → AccelerationStructureUsage` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_geometry_descriptors_ptr` | `(&self, geometry_descriptors...) → void` | — |
| `set_geometry_descriptors_raw` | `(&self, descriptors: *const ...) → void` | `setGeometryDescriptors` |
| `set_motion_end_border_mode` | `(&self, mode: MotionBorderMode) → void` | `setMotionEndBorderMode` |
| `set_motion_end_time` | `(&self, time: f32) → void` | `setMotionEndTime` |
| `set_motion_keyframe_count` | `(&self, count: UInteger) → void` | `setMotionKeyframeCount` |
| `set_motion_start_border_mode` | `(&self, mode: MotionBorderMode) → void` | `setMotionStartBorderMode` |
| `set_motion_start_time` | `(&self, time: f32) → void` | `setMotionStartTime` |
| `set_usage` | `(&self, usage: AccelerationS...) → void` | — |

---

### `RasterizationRateLayerArray`

C++ equivalent: `NS::RasterizationRateLayerArray`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `object` | `(&self, layer_index: UInteger) → Option<RasterizationRateLayerDescriptor>` | `object` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_object` | `(&self, layer: &Rasterizatio...) → void` | `setObject` |

---

### `RasterizationRateLayerDescriptor`

C++ equivalent: `NS::RasterizationRateLayerDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `horizontal` | `(&self) → RasterizationRateSampleArray` | `horizontal` |
| `horizontal_sample_storage` | `(&self) → *mut f32` | `horizontalSampleStorage` |
| `max_sample_count` | `(&self) → Size` | `maxSampleCount` |
| `sample_count` | `(&self) → Size` | `sampleCount` |
| `vertical` | `(&self) → RasterizationRateSampleArray` | `vertical` |
| `vertical_sample_storage` | `(&self) → *mut f32` | `verticalSampleStorage` |
| `with_sample_count` | `(sample_count: Size) → Option<Self>` | — |
| `with_sample_count_and_data` | `(sample_count: Size,
       ...) → Option<Self>` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_sample_count` | `(&self, sample_count: Size) → void` | `setSampleCount` |

---

### `RasterizationRateMap`

C++ equivalent: `NS::RasterizationRateMap`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `copy_parameter_data_to_buffer` | `(&self, buffer: &Buffer, off...) → void` | `copyParameterDataToBuffer` |
| `device` | `(&self) → crate::Device` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |
| `layer_count` | `(&self) → UInteger` | `layerCount` |
| `map_physical_to_screen_coordinates` | `(&self,
        physical_coo...) → Coordinate2D` | `mapPhysicalToScreenCoordinates` |
| `map_screen_to_physical_coordinates` | `(&self,
        screen_coord...) → Coordinate2D` | `mapScreenToPhysicalCoordinates` |
| `parameter_buffer_size_and_align` | `(&self) → SizeAndAlign` | `parameterBufferSizeAndAlign` |
| `physical_granularity` | `(&self) → Size` | `physicalGranularity` |
| `physical_size` | `(&self, layer_index: UInteger) → Size` | `physicalSize` |
| `screen_size` | `(&self) → Size` | `screenSize` |

---

### `RasterizationRateMapDescriptor`

C++ equivalent: `NS::RasterizationRateMapDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |
| `layer` | `(&self, layer_index: UInteger) → Option<RasterizationRateLayerDescriptor>` | `layer` |
| `layer_count` | `(&self) → UInteger` | `layerCount` |
| `layers` | `(&self) → RasterizationRateLayerArray` | `layers` |
| `screen_size` | `(&self) → Size` | `screenSize` |
| `with_screen_size` | `(screen_size: Size) → Option<Self>` | — |
| `with_screen_size_and_layer` | `(screen_size: Size,
        ...) → Option<Self>` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |
| `set_layer` | `(&self, layer: &Rasterizatio...) → void` | `setLayer` |
| `set_screen_size` | `(&self, screen_size: Size) → void` | `setScreenSize` |

---

### `RasterizationRateSampleArray`

C++ equivalent: `NS::RasterizationRateSampleArray`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `object_raw` | `(&self, index: UInteger) → *mut c_void` | `object` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_object_raw` | `(&self, value: *const c_void...) → void` | `setObject` |

---

### `RenderCommandEncoder`

C++ equivalent: `NS::RenderCommandEncoder`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `barrier` | `(&self) → void` | — |
| `barrier_after_queue_stages` | `(&self,
        after_stages...) → void` | — |
| `barrier_buffer` | `(&self, buffer: *const c_voi...) → void` | — |
| `barrier_texture` | `(&self, texture: *const c_vo...) → void` | — |
| `command_buffer` | `(&self) → crate::CommandBuffer` | — |
| `device` | `(&self) → crate::Device` | — |
| `dispatch_threads_per_tile` | `(&self, threads_per_tile: Size) → void` | `dispatchThreadsPerTile` |
| `draw_indexed_patches` | `(&self,
        number_of_pa...) → void` | — |
| `draw_indexed_patches_indirect` | `(&self,
        number_of_pa...) → void` | — |
| `draw_indexed_primitives` | `(&self,
        primitive_ty...) → void` | `drawIndexedPrimitives` |
| `draw_indexed_primitives_indirect` | `(&self,
        primitive_ty...) → void` | — |
| `draw_indexed_primitives_instanced` | `(&self,
        primitive_ty...) → void` | — |
| `draw_indexed_primitives_instanced_base_vertex_base_instance` | `(&self,
        primitive_ty...) → void` | — |
| `draw_mesh_threadgroups` | `(&self,
        threadgroups...) → void` | `drawMeshThreadgroups` |
| `draw_mesh_threadgroups_indirect` | `(&self,
        indirect_buf...) → void` | — |
| `draw_mesh_threads` | `(&self,
        threads_per_...) → void` | `drawMeshThreads` |
| `draw_patches` | `(&self,
        number_of_pa...) → void` | — |
| `draw_patches_indirect` | `(&self,
        number_of_pa...) → void` | — |
| `draw_primitives` | `(&self,
        primitive_ty...) → void` | `drawPrimitives` |
| `draw_primitives_indirect` | `(&self,
        primitive_ty...) → void` | — |
| `draw_primitives_instanced` | `(&self,
        primitive_ty...) → void` | — |
| `draw_primitives_instanced_base` | `(&self,
        primitive_ty...) → void` | — |
| `draw_primitives_instanced_base_instance` | `(&self,
        primitive_ty...) → void` | — |
| `end_encoding` | `(&self) → void` | — |
| `execute_commands_in_buffer_ptr` | `(&self,
        indirect_com...) → void` | `executeCommandsInBuffer` |
| `execute_commands_in_buffer_with_indirect_range_ptr` | `(&self,
        indirect_com...) → void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `insert_debug_signpost` | `(&self, string: &str) → void` | — |
| `label` | `(&self) → Option<String>` | — |
| `memory_barrier_with_resources_ptr` | `(&self,
        resources: *...) → void` | — |
| `memory_barrier_with_scope` | `(&self,
        scope: Barri...) → void` | — |
| `pop_debug_group` | `(&self) → void` | — |
| `push_debug_group` | `(&self, string: &str) → void` | — |
| `sample_counters_in_buffer_ptr` | `(&self,
        sample_buffe...) → void` | — |
| `texture_barrier` | `(&self) → void` | — |
| `tile_height` | `(&self) → UInteger` | `tileHeight` |
| `tile_width` | `(&self) → UInteger` | `tileWidth` |
| `update_fence` | `(&self, fence: &crate::Fence...) → void` | — |
| `update_fence_ptr` | `(&self, fence: *const c_void...) → void` | — |
| `use_buffer` | `(&self, buffer: &Buffer, usa...) → void` | — |
| `use_heap` | `(&self, heap: &crate::Heap) → void` | — |
| `use_heap_ptr` | `(&self, heap: *const c_void) → void` | — |
| `use_heap_ptr_with_stages` | `(&self, heap: *const c_void,...) → void` | — |
| `use_heaps_ptr` | `(&self, heaps: *const *const...) → void` | — |
| `use_heaps_with_stages_ptr` | `(&self,
        heaps: *cons...) → void` | — |
| `use_resource` | `(&self, resource: *const c_v...) → void` | — |
| `use_resource_ptr` | `(&self, resource: *const c_v...) → void` | — |
| `use_resource_ptr_with_stages` | `(&self,
        resource: *c...) → void` | — |
| `use_resources_ptr` | `(&self,
        resources: *...) → void` | — |
| `use_resources_with_stages_ptr` | `(&self,
        resources: *...) → void` | — |
| `use_texture` | `(&self, texture: &Texture, u...) → void` | — |
| `wait_for_fence` | `(&self, fence: &crate::Fence...) → void` | — |
| `wait_for_fence_ptr` | `(&self, fence: *const c_void...) → void` | — |
| `write_timestamp` | `(&self,
        granularity:...) → void` | `writeTimestamp` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_blend_color` | `(&self, red: f32, green: f32...) → void` | `setBlendColor` |
| `set_color_attachment_map_ptr` | `(&self, mapping: *const c_void) → void` | `setColorAttachmentMap` |
| `set_color_store_action` | `(&self, store_action: StoreA...) → void` | `setColorStoreAction` |
| `set_color_store_action_options` | `(&self,
        options: Sto...) → void` | — |
| `set_cull_mode` | `(&self, cull_mode: CullMode) → void` | `setCullMode` |
| `set_depth_bias` | `(&self, depth_bias: f32, slo...) → void` | `setDepthBias` |
| `set_depth_clip_mode` | `(&self, mode: DepthClipMode) → void` | `setDepthClipMode` |
| `set_depth_stencil_state` | `(&self, state: &crate::Depth...) → void` | `setDepthStencilState` |
| `set_depth_store_action` | `(&self, store_action: StoreA...) → void` | `setDepthStoreAction` |
| `set_depth_store_action_options` | `(&self, options: StoreAction...) → void` | — |
| `set_depth_test_bounds` | `(&self, min_bound: f32, max_...) → void` | `setDepthTestBounds` |
| `set_fragment_acceleration_structure` | `(&self,
        acceleration...) → void` | — |
| `set_fragment_acceleration_structure_ptr` | `(&self,
        acceleration...) → void` | — |
| `set_fragment_argument_table` | `(&self, table: *const c_void...) → void` | — |
| `set_fragment_buffer` | `(&self, buffer: &Buffer, off...) → void` | — |
| `set_fragment_buffer_offset` | `(&self, offset: UInteger, in...) → void` | — |
| `set_fragment_buffers_ptr` | `(&self,
        buffers: *co...) → void` | — |
| `set_fragment_bytes` | `(&self, bytes: &[u8], index:...) → void` | — |
| `set_fragment_intersection_function_table_ptr` | `(&self,
        function_tab...) → void` | — |
| `set_fragment_intersection_function_tables_ptr` | `(&self,
        function_tab...) → void` | — |
| `set_fragment_sampler_state` | `(&self, sampler: &crate::Sam...) → void` | — |
| `set_fragment_sampler_state_with_lod_clamps` | `(&self,
        sampler: &cr...) → void` | — |
| `set_fragment_sampler_states_ptr` | `(&self,
        samplers: *c...) → void` | — |
| `set_fragment_sampler_states_with_lod_clamps_ptr` | `(&self,
        samplers: *c...) → void` | — |
| `set_fragment_texture` | `(&self, texture: &Texture, i...) → void` | — |
| `set_fragment_textures_ptr` | `(&self,
        textures: *c...) → void` | — |
| `set_fragment_visible_function_table_ptr` | `(&self,
        function_tab...) → void` | — |
| `set_fragment_visible_function_tables_ptr` | `(&self,
        function_tab...) → void` | — |
| `set_front_facing_winding` | `(&self, winding: Winding) → void` | `setFrontFacingWinding` |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_mesh_argument_table` | `(&self, table: *const c_void...) → void` | — |
| `set_mesh_buffer` | `(&self, buffer: &Buffer, off...) → void` | — |
| `set_mesh_buffer_offset` | `(&self, offset: UInteger, in...) → void` | — |
| `set_mesh_buffers_ptr` | `(&self,
        buffers: *co...) → void` | — |
| `set_mesh_bytes` | `(&self, bytes: &[u8], index:...) → void` | — |
| `set_mesh_sampler_state` | `(&self, sampler: &crate::Sam...) → void` | — |
| `set_mesh_sampler_state_with_lod_clamps` | `(&self,
        sampler: &cr...) → void` | — |
| `set_mesh_sampler_states_ptr` | `(&self,
        samplers: *c...) → void` | — |
| `set_mesh_sampler_states_with_lod_clamps_ptr` | `(&self,
        samplers: *c...) → void` | — |
| `set_mesh_texture` | `(&self, texture: &Texture, i...) → void` | — |
| `set_mesh_textures_ptr` | `(&self,
        textures: *c...) → void` | — |
| `set_object_argument_table` | `(&self, table: *const c_void...) → void` | — |
| `set_object_buffer` | `(&self, buffer: &Buffer, off...) → void` | — |
| `set_object_buffer_offset` | `(&self, offset: UInteger, in...) → void` | — |
| `set_object_buffers_ptr` | `(&self,
        buffers: *co...) → void` | — |
| `set_object_bytes` | `(&self, bytes: &[u8], index:...) → void` | — |
| `set_object_sampler_state` | `(&self, sampler: &crate::Sam...) → void` | — |
| `set_object_sampler_state_with_lod_clamps` | `(&self,
        sampler: &cr...) → void` | — |
| `set_object_sampler_states_ptr` | `(&self,
        samplers: *c...) → void` | — |
| `set_object_sampler_states_with_lod_clamps_ptr` | `(&self,
        samplers: *c...) → void` | — |
| `set_object_texture` | `(&self, texture: &Texture, i...) → void` | — |
| `set_object_textures_ptr` | `(&self,
        textures: *c...) → void` | — |
| `set_object_threadgroup_memory_length` | `(&self, length: UInteger, in...) → void` | `setObjectThreadgroupMemoryLength` |
| `set_render_pipeline_state` | `(&self, state: &crate::Rende...) → void` | `setRenderPipelineState` |
| `set_scissor_rect` | `(&self, rect: ScissorRect) → void` | `setScissorRect` |
| `set_scissor_rects` | `(&self, rects: &[ScissorRect]) → void` | `setScissorRects` |
| `set_stencil_reference_value` | `(&self, value: u32) → void` | `setStencilReferenceValue` |
| `set_stencil_reference_values` | `(&self, front: u32, back: u32) → void` | `setStencilReferenceValues` |
| `set_stencil_store_action` | `(&self, store_action: StoreA...) → void` | `setStencilStoreAction` |
| `set_stencil_store_action_options` | `(&self, options: StoreAction...) → void` | — |
| `set_tessellation_factor_buffer` | `(&self,
        buffer: &Buf...) → void` | — |
| `set_tessellation_factor_scale` | `(&self, scale: f32) → void` | — |
| `set_threadgroup_memory_length` | `(&self,
        length: UInt...) → void` | `setThreadgroupMemoryLength` |
| `set_tile_acceleration_structure` | `(&self,
        acceleration...) → void` | — |
| `set_tile_acceleration_structure_ptr` | `(&self,
        acceleration...) → void` | — |
| `set_tile_argument_table` | `(&self, table: *const c_void...) → void` | `setArgumentTable` |
| `set_tile_buffer` | `(&self, buffer: &Buffer, off...) → void` | — |
| `set_tile_buffer_offset` | `(&self, offset: UInteger, in...) → void` | — |
| `set_tile_buffers_ptr` | `(&self,
        buffers: *co...) → void` | — |
| `set_tile_bytes` | `(&self, bytes: &[u8], index:...) → void` | — |
| `set_tile_intersection_function_table_ptr` | `(&self,
        function_tab...) → void` | — |
| `set_tile_intersection_function_tables_ptr` | `(&self,
        function_tab...) → void` | — |
| `set_tile_sampler_state` | `(&self, sampler: &crate::Sam...) → void` | — |
| `set_tile_sampler_state_with_lod_clamps` | `(&self,
        sampler: &cr...) → void` | — |
| `set_tile_sampler_states_ptr` | `(&self,
        samplers: *c...) → void` | — |
| `set_tile_sampler_states_with_lod_clamps_ptr` | `(&self,
        samplers: *c...) → void` | — |
| `set_tile_texture` | `(&self, texture: &Texture, i...) → void` | — |
| `set_tile_textures_ptr` | `(&self,
        textures: *c...) → void` | — |
| `set_tile_visible_function_table_ptr` | `(&self,
        function_tab...) → void` | — |
| `set_tile_visible_function_tables_ptr` | `(&self,
        function_tab...) → void` | — |
| `set_triangle_fill_mode` | `(&self, fill_mode: TriangleF...) → void` | `setTriangleFillMode` |
| `set_vertex_acceleration_structure` | `(&self,
        acceleration...) → void` | — |
| `set_vertex_acceleration_structure_ptr` | `(&self,
        acceleration...) → void` | — |
| `set_vertex_amplification_count` | `(&self,
        count: UInte...) → void` | `setVertexAmplificationCount` |
| `set_vertex_argument_table` | `(&self, table: *const c_void...) → void` | — |
| `set_vertex_buffer` | `(&self, buffer: &Buffer, off...) → void` | — |
| `set_vertex_buffer_offset` | `(&self, offset: UInteger, in...) → void` | — |
| `set_vertex_buffer_offset_with_stride` | `(&self,
        offset: UInt...) → void` | — |
| `set_vertex_buffer_with_stride` | `(&self,
        buffer: &Buf...) → void` | — |
| `set_vertex_buffers_ptr` | `(&self,
        buffers: *co...) → void` | — |
| `set_vertex_buffers_with_strides_ptr` | `(&self,
        buffers: *co...) → void` | — |
| `set_vertex_bytes` | `(&self, bytes: &[u8], index:...) → void` | — |
| `set_vertex_bytes_with_stride` | `(&self, bytes: &[u8], stride...) → void` | — |
| `set_vertex_intersection_function_table_ptr` | `(&self,
        function_tab...) → void` | — |
| `set_vertex_intersection_function_tables_ptr` | `(&self,
        function_tab...) → void` | — |
| `set_vertex_sampler_state` | `(&self, sampler: &crate::Sam...) → void` | — |
| `set_vertex_sampler_state_with_lod_clamps` | `(&self,
        sampler: &cr...) → void` | — |
| `set_vertex_sampler_states_ptr` | `(&self,
        samplers: *c...) → void` | — |
| `set_vertex_sampler_states_with_lod_clamps_ptr` | `(&self,
        samplers: *c...) → void` | — |
| `set_vertex_texture` | `(&self, texture: &Texture, i...) → void` | — |
| `set_vertex_textures_ptr` | `(&self,
        textures: *c...) → void` | — |
| `set_vertex_visible_function_table_ptr` | `(&self,
        function_tab...) → void` | — |
| `set_vertex_visible_function_tables_ptr` | `(&self,
        function_tab...) → void` | — |
| `set_viewport` | `(&self, viewport: Viewport) → void` | `setViewport` |
| `set_viewports` | `(&self, viewports: &[Viewport]) → void` | `setViewports` |
| `set_visibility_result_mode` | `(&self, mode: VisibilityResu...) → void` | `setVisibilityResultMode` |

---

### `RenderCommandEncoder`

C++ equivalent: `NS::RenderCommandEncoder`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `barrier` | `(&self) → void` | — |
| `barrier_after_queue_stages` | `(&self,
        after_stages...) → void` | — |
| `barrier_buffer` | `(&self, buffer: *const c_voi...) → void` | — |
| `barrier_texture` | `(&self, texture: *const c_vo...) → void` | — |
| `command_buffer` | `(&self) → crate::CommandBuffer` | — |
| `device` | `(&self) → crate::Device` | — |
| `dispatch_threads_per_tile` | `(&self, threads_per_tile: Size) → void` | `dispatchThreadsPerTile` |
| `draw_indexed_patches` | `(&self,
        number_of_pa...) → void` | `drawIndexedPatches` |
| `draw_indexed_patches_indirect` | `(&self,
        number_of_pa...) → void` | — |
| `draw_indexed_primitives` | `(&self,
        primitive_ty...) → void` | `drawIndexedPrimitives` |
| `draw_indexed_primitives_indirect` | `(&self,
        primitive_ty...) → void` | — |
| `draw_indexed_primitives_instanced` | `(&self,
        primitive_ty...) → void` | — |
| `draw_indexed_primitives_instanced_base_vertex_base_instance` | `(&self,
        primitive_ty...) → void` | — |
| `draw_mesh_threadgroups` | `(&self,
        threadgroups...) → void` | `drawMeshThreadgroups` |
| `draw_mesh_threadgroups_indirect` | `(&self,
        indirect_buf...) → void` | — |
| `draw_mesh_threads` | `(&self,
        threads_per_...) → void` | `drawMeshThreads` |
| `draw_patches` | `(&self,
        number_of_pa...) → void` | `drawPatches` |
| `draw_patches_indirect` | `(&self,
        number_of_pa...) → void` | — |
| `draw_primitives` | `(&self,
        primitive_ty...) → void` | `drawPrimitives` |
| `draw_primitives_indirect` | `(&self,
        primitive_ty...) → void` | — |
| `draw_primitives_instanced` | `(&self,
        primitive_ty...) → void` | — |
| `draw_primitives_instanced_base` | `(&self,
        primitive_ty...) → void` | — |
| `draw_primitives_instanced_base_instance` | `(&self,
        primitive_ty...) → void` | — |
| `end_encoding` | `(&self) → void` | — |
| `execute_commands_in_buffer_ptr` | `(&self,
        indirect_com...) → void` | `executeCommandsInBuffer` |
| `execute_commands_in_buffer_with_indirect_range_ptr` | `(&self,
        indirect_com...) → void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `insert_debug_signpost` | `(&self, string: &str) → void` | — |
| `label` | `(&self) → Option<String>` | — |
| `memory_barrier_with_resources_ptr` | `(&self,
        resources: *...) → void` | — |
| `memory_barrier_with_scope` | `(&self,
        scope: Barri...) → void` | `memoryBarrier` |
| `pop_debug_group` | `(&self) → void` | — |
| `push_debug_group` | `(&self, string: &str) → void` | — |
| `sample_counters_in_buffer_ptr` | `(&self,
        sample_buffe...) → void` | `sampleCountersInBuffer` |
| `texture_barrier` | `(&self) → void` | `textureBarrier` |
| `tile_height` | `(&self) → UInteger` | `tileHeight` |
| `tile_width` | `(&self) → UInteger` | `tileWidth` |
| `update_fence` | `(&self, fence: &crate::Fence...) → void` | `updateFence` |
| `update_fence_ptr` | `(&self, fence: *const c_void...) → void` | — |
| `use_buffer` | `(&self, buffer: &Buffer, usa...) → void` | — |
| `use_heap` | `(&self, heap: &crate::Heap) → void` | `useHeap` |
| `use_heap_ptr` | `(&self, heap: *const c_void) → void` | — |
| `use_heap_ptr_with_stages` | `(&self, heap: *const c_void,...) → void` | — |
| `use_heaps_ptr` | `(&self, heaps: *const *const...) → void` | — |
| `use_heaps_with_stages_ptr` | `(&self,
        heaps: *cons...) → void` | — |
| `use_resource` | `(&self, resource: *const c_v...) → void` | `useResource` |
| `use_resource_ptr` | `(&self, resource: *const c_v...) → void` | — |
| `use_resource_ptr_with_stages` | `(&self,
        resource: *c...) → void` | — |
| `use_resources_ptr` | `(&self,
        resources: *...) → void` | `useResources` |
| `use_resources_with_stages_ptr` | `(&self,
        resources: *...) → void` | — |
| `use_texture` | `(&self, texture: &Texture, u...) → void` | — |
| `wait_for_fence` | `(&self, fence: &crate::Fence...) → void` | `waitForFence` |
| `wait_for_fence_ptr` | `(&self, fence: *const c_void...) → void` | — |
| `write_timestamp` | `(&self,
        granularity:...) → void` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_blend_color` | `(&self, red: f32, green: f32...) → void` | `setBlendColor` |
| `set_color_attachment_map_ptr` | `(&self, mapping: *const c_void) → void` | `setColorAttachmentMap` |
| `set_color_store_action` | `(&self, store_action: StoreA...) → void` | `setColorStoreAction` |
| `set_color_store_action_options` | `(&self,
        options: Sto...) → void` | `setColorStoreActionOptions` |
| `set_cull_mode` | `(&self, cull_mode: CullMode) → void` | `setCullMode` |
| `set_depth_bias` | `(&self, depth_bias: f32, slo...) → void` | `setDepthBias` |
| `set_depth_clip_mode` | `(&self, mode: DepthClipMode) → void` | `setDepthClipMode` |
| `set_depth_stencil_state` | `(&self, state: &crate::Depth...) → void` | `setDepthStencilState` |
| `set_depth_store_action` | `(&self, store_action: StoreA...) → void` | `setDepthStoreAction` |
| `set_depth_store_action_options` | `(&self, options: StoreAction...) → void` | `setDepthStoreActionOptions` |
| `set_depth_test_bounds` | `(&self, min_bound: f32, max_...) → void` | `setDepthTestBounds` |
| `set_fragment_acceleration_structure` | `(&self,
        acceleration...) → void` | `setFragmentAccelerationStructure` |
| `set_fragment_acceleration_structure_ptr` | `(&self,
        acceleration...) → void` | — |
| `set_fragment_argument_table` | `(&self, table: *const c_void...) → void` | — |
| `set_fragment_buffer` | `(&self, buffer: &Buffer, off...) → void` | `setFragmentBuffer` |
| `set_fragment_buffer_offset` | `(&self, offset: UInteger, in...) → void` | `setFragmentBufferOffset` |
| `set_fragment_buffers_ptr` | `(&self,
        buffers: *co...) → void` | — |
| `set_fragment_bytes` | `(&self, bytes: &[u8], index:...) → void` | `setFragmentBytes` |
| `set_fragment_intersection_function_table_ptr` | `(&self,
        function_tab...) → void` | `setFragmentIntersectionFunctionTable` |
| `set_fragment_intersection_function_tables_ptr` | `(&self,
        function_tab...) → void` | — |
| `set_fragment_sampler_state` | `(&self, sampler: &crate::Sam...) → void` | `setFragmentSamplerState` |
| `set_fragment_sampler_state_with_lod_clamps` | `(&self,
        sampler: &cr...) → void` | — |
| `set_fragment_sampler_states_ptr` | `(&self,
        samplers: *c...) → void` | `setFragmentSamplerStates` |
| `set_fragment_sampler_states_with_lod_clamps_ptr` | `(&self,
        samplers: *c...) → void` | — |
| `set_fragment_texture` | `(&self, texture: &Texture, i...) → void` | `setFragmentTexture` |
| `set_fragment_textures_ptr` | `(&self,
        textures: *c...) → void` | — |
| `set_fragment_visible_function_table_ptr` | `(&self,
        function_tab...) → void` | `setFragmentVisibleFunctionTable` |
| `set_fragment_visible_function_tables_ptr` | `(&self,
        function_tab...) → void` | — |
| `set_front_facing_winding` | `(&self, winding: Winding) → void` | `setFrontFacingWinding` |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_mesh_argument_table` | `(&self, table: *const c_void...) → void` | — |
| `set_mesh_buffer` | `(&self, buffer: &Buffer, off...) → void` | `setMeshBuffer` |
| `set_mesh_buffer_offset` | `(&self, offset: UInteger, in...) → void` | `setMeshBufferOffset` |
| `set_mesh_buffers_ptr` | `(&self,
        buffers: *co...) → void` | — |
| `set_mesh_bytes` | `(&self, bytes: &[u8], index:...) → void` | `setMeshBytes` |
| `set_mesh_sampler_state` | `(&self, sampler: &crate::Sam...) → void` | `setMeshSamplerState` |
| `set_mesh_sampler_state_with_lod_clamps` | `(&self,
        sampler: &cr...) → void` | — |
| `set_mesh_sampler_states_ptr` | `(&self,
        samplers: *c...) → void` | `setMeshSamplerStates` |
| `set_mesh_sampler_states_with_lod_clamps_ptr` | `(&self,
        samplers: *c...) → void` | — |
| `set_mesh_texture` | `(&self, texture: &Texture, i...) → void` | `setMeshTexture` |
| `set_mesh_textures_ptr` | `(&self,
        textures: *c...) → void` | — |
| `set_object_argument_table` | `(&self, table: *const c_void...) → void` | — |
| `set_object_buffer` | `(&self, buffer: &Buffer, off...) → void` | `setObjectBuffer` |
| `set_object_buffer_offset` | `(&self, offset: UInteger, in...) → void` | `setObjectBufferOffset` |
| `set_object_buffers_ptr` | `(&self,
        buffers: *co...) → void` | — |
| `set_object_bytes` | `(&self, bytes: &[u8], index:...) → void` | `setObjectBytes` |
| `set_object_sampler_state` | `(&self, sampler: &crate::Sam...) → void` | `setObjectSamplerState` |
| `set_object_sampler_state_with_lod_clamps` | `(&self,
        sampler: &cr...) → void` | — |
| `set_object_sampler_states_ptr` | `(&self,
        samplers: *c...) → void` | `setObjectSamplerStates` |
| `set_object_sampler_states_with_lod_clamps_ptr` | `(&self,
        samplers: *c...) → void` | — |
| `set_object_texture` | `(&self, texture: &Texture, i...) → void` | `setObjectTexture` |
| `set_object_textures_ptr` | `(&self,
        textures: *c...) → void` | — |
| `set_object_threadgroup_memory_length` | `(&self, length: UInteger, in...) → void` | `setObjectThreadgroupMemoryLength` |
| `set_render_pipeline_state` | `(&self, state: &crate::Rende...) → void` | `setRenderPipelineState` |
| `set_scissor_rect` | `(&self, rect: ScissorRect) → void` | `setScissorRect` |
| `set_scissor_rects` | `(&self, rects: &[ScissorRect]) → void` | `setScissorRects` |
| `set_stencil_reference_value` | `(&self, value: u32) → void` | `setStencilReferenceValue` |
| `set_stencil_reference_values` | `(&self, front: u32, back: u32) → void` | `setStencilReferenceValues` |
| `set_stencil_store_action` | `(&self, store_action: StoreA...) → void` | `setStencilStoreAction` |
| `set_stencil_store_action_options` | `(&self, options: StoreAction...) → void` | `setStencilStoreActionOptions` |
| `set_tessellation_factor_buffer` | `(&self,
        buffer: &Buf...) → void` | `setTessellationFactorBuffer` |
| `set_tessellation_factor_scale` | `(&self, scale: f32) → void` | `setTessellationFactorScale` |
| `set_threadgroup_memory_length` | `(&self,
        length: UInt...) → void` | `setThreadgroupMemoryLength` |
| `set_tile_acceleration_structure` | `(&self,
        acceleration...) → void` | `setTileAccelerationStructure` |
| `set_tile_acceleration_structure_ptr` | `(&self,
        acceleration...) → void` | — |
| `set_tile_argument_table` | `(&self, table: *const c_void...) → void` | — |
| `set_tile_buffer` | `(&self, buffer: &Buffer, off...) → void` | `setTileBuffer` |
| `set_tile_buffer_offset` | `(&self, offset: UInteger, in...) → void` | `setTileBufferOffset` |
| `set_tile_buffers_ptr` | `(&self,
        buffers: *co...) → void` | — |
| `set_tile_bytes` | `(&self, bytes: &[u8], index:...) → void` | `setTileBytes` |
| `set_tile_intersection_function_table_ptr` | `(&self,
        function_tab...) → void` | `setTileIntersectionFunctionTable` |
| `set_tile_intersection_function_tables_ptr` | `(&self,
        function_tab...) → void` | — |
| `set_tile_sampler_state` | `(&self, sampler: &crate::Sam...) → void` | `setTileSamplerState` |
| `set_tile_sampler_state_with_lod_clamps` | `(&self,
        sampler: &cr...) → void` | — |
| `set_tile_sampler_states_ptr` | `(&self,
        samplers: *c...) → void` | `setTileSamplerStates` |
| `set_tile_sampler_states_with_lod_clamps_ptr` | `(&self,
        samplers: *c...) → void` | — |
| `set_tile_texture` | `(&self, texture: &Texture, i...) → void` | `setTileTexture` |
| `set_tile_textures_ptr` | `(&self,
        textures: *c...) → void` | — |
| `set_tile_visible_function_table_ptr` | `(&self,
        function_tab...) → void` | `setTileVisibleFunctionTable` |
| `set_tile_visible_function_tables_ptr` | `(&self,
        function_tab...) → void` | — |
| `set_triangle_fill_mode` | `(&self, fill_mode: TriangleF...) → void` | `setTriangleFillMode` |
| `set_vertex_acceleration_structure` | `(&self,
        acceleration...) → void` | `setVertexAccelerationStructure` |
| `set_vertex_acceleration_structure_ptr` | `(&self,
        acceleration...) → void` | — |
| `set_vertex_amplification_count` | `(&self,
        count: UInte...) → void` | `setVertexAmplificationCount` |
| `set_vertex_argument_table` | `(&self, table: *const c_void...) → void` | — |
| `set_vertex_buffer` | `(&self, buffer: &Buffer, off...) → void` | `setVertexBuffer` |
| `set_vertex_buffer_offset` | `(&self, offset: UInteger, in...) → void` | `setVertexBufferOffset` |
| `set_vertex_buffer_offset_with_stride` | `(&self,
        offset: UInt...) → void` | — |
| `set_vertex_buffer_with_stride` | `(&self,
        buffer: &Buf...) → void` | — |
| `set_vertex_buffers_ptr` | `(&self,
        buffers: *co...) → void` | — |
| `set_vertex_buffers_with_strides_ptr` | `(&self,
        buffers: *co...) → void` | — |
| `set_vertex_bytes` | `(&self, bytes: &[u8], index:...) → void` | `setVertexBytes` |
| `set_vertex_bytes_with_stride` | `(&self, bytes: &[u8], stride...) → void` | — |
| `set_vertex_intersection_function_table_ptr` | `(&self,
        function_tab...) → void` | `setVertexIntersectionFunctionTable` |
| `set_vertex_intersection_function_tables_ptr` | `(&self,
        function_tab...) → void` | — |
| `set_vertex_sampler_state` | `(&self, sampler: &crate::Sam...) → void` | `setVertexSamplerState` |
| `set_vertex_sampler_state_with_lod_clamps` | `(&self,
        sampler: &cr...) → void` | — |
| `set_vertex_sampler_states_ptr` | `(&self,
        samplers: *c...) → void` | `setVertexSamplerStates` |
| `set_vertex_sampler_states_with_lod_clamps_ptr` | `(&self,
        samplers: *c...) → void` | — |
| `set_vertex_texture` | `(&self, texture: &Texture, i...) → void` | `setVertexTexture` |
| `set_vertex_textures_ptr` | `(&self,
        textures: *c...) → void` | — |
| `set_vertex_visible_function_table_ptr` | `(&self,
        function_tab...) → void` | `setVertexVisibleFunctionTable` |
| `set_vertex_visible_function_tables_ptr` | `(&self,
        function_tab...) → void` | — |
| `set_viewport` | `(&self, viewport: Viewport) → void` | `setViewport` |
| `set_viewports` | `(&self, viewports: &[Viewport]) → void` | `setViewports` |
| `set_visibility_result_mode` | `(&self, mode: VisibilityResu...) → void` | `setVisibilityResultMode` |

---

### `RenderPassAttachmentDescriptor`

C++ equivalent: `NS::RenderPassAttachmentDescriptor`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `depth_plane` | `(&self) → UInteger` | `depthPlane` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `level` | `(&self) → UInteger` | `level` |
| `load_action` | `(&self) → LoadAction` | `loadAction` |
| `resolve_depth_plane` | `(&self) → UInteger` | `resolveDepthPlane` |
| `resolve_level` | `(&self) → UInteger` | `resolveLevel` |
| `resolve_slice` | `(&self) → UInteger` | `resolveSlice` |
| `resolve_texture` | `(&self) → Option<Texture>` | `resolveTexture` |
| `slice` | `(&self) → UInteger` | `slice` |
| `store_action` | `(&self) → StoreAction` | `storeAction` |
| `store_action_options` | `(&self) → StoreActionOptions` | `storeActionOptions` |
| `texture` | `(&self) → Option<Texture>` | `texture` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_depth_plane` | `(&self, depth_plane: UInteger) → void` | `setDepthPlane` |
| `set_level` | `(&self, level: UInteger) → void` | `setLevel` |
| `set_load_action` | `(&self, load_action: LoadAction) → void` | `setLoadAction` |
| `set_resolve_depth_plane` | `(&self, depth_plane: UInteger) → void` | `setResolveDepthPlane` |
| `set_resolve_level` | `(&self, level: UInteger) → void` | `setResolveLevel` |
| `set_resolve_slice` | `(&self, slice: UInteger) → void` | `setResolveSlice` |
| `set_resolve_texture` | `(&self, texture: Option<&Tex...) → void` | `setResolveTexture` |
| `set_slice` | `(&self, slice: UInteger) → void` | `setSlice` |
| `set_store_action` | `(&self, store_action: StoreA...) → void` | `setStoreAction` |
| `set_store_action_options` | `(&self, options: StoreAction...) → void` | `setStoreActionOptions` |
| `set_texture` | `(&self, texture: Option<&Tex...) → void` | `setTexture` |

---

### `RenderPassColorAttachmentDescriptor`

C++ equivalent: `NS::RenderPassColorAttachmentDescriptor`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `clear_color` | `(&self) → ClearColor` | `clearColor` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `level` | `(&self) → UInteger` | — |
| `load_action` | `(&self) → LoadAction` | — |
| `slice` | `(&self) → UInteger` | — |
| `store_action` | `(&self) → StoreAction` | — |
| `texture` | `(&self) → Option<Texture>` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_clear_color` | `(&self, color: ClearColor) → void` | `setClearColor` |
| `set_level` | `(&self, level: UInteger) → void` | — |
| `set_load_action` | `(&self, load_action: LoadAction) → void` | — |
| `set_slice` | `(&self, slice: UInteger) → void` | — |
| `set_store_action` | `(&self, store_action: StoreA...) → void` | — |
| `set_texture` | `(&self, texture: Option<&Tex...) → void` | — |

---

### `RenderPassColorAttachmentDescriptorArray`

C++ equivalent: `NS::RenderPassColorAttachmentDescriptorArray`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `object_at` | `(&self, index: UInteger) → Option<RenderPassColorAttachmentDescriptor>` | `object` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_object_at` | `(&self,
        attachment: ...) → void` | `setObject` |

---

### `RenderPassDepthAttachmentDescriptor`

C++ equivalent: `NS::RenderPassDepthAttachmentDescriptor`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `clear_depth` | `(&self) → f64` | `clearDepth` |
| `depth_resolve_filter` | `(&self) → MultisampleDepthResolveFilter` | `depthResolveFilter` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `load_action` | `(&self) → LoadAction` | — |
| `store_action` | `(&self) → StoreAction` | — |
| `texture` | `(&self) → Option<Texture>` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_clear_depth` | `(&self, depth: f64) → void` | `setClearDepth` |
| `set_depth_resolve_filter` | `(&self, filter: MultisampleD...) → void` | `setDepthResolveFilter` |
| `set_load_action` | `(&self, load_action: LoadAction) → void` | — |
| `set_store_action` | `(&self, store_action: StoreA...) → void` | — |
| `set_texture` | `(&self, texture: Option<&Tex...) → void` | — |

---

### `RenderPassDescriptor`

C++ equivalent: `NS::RenderPassDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `color_attachments` | `(&self) → Option<RenderPassColorAttachmentDescriptorArray>` | `colorAttachments` |
| `default_raster_sample_count` | `(&self) → UInteger` | `defaultRasterSampleCount` |
| `depth_attachment` | `(&self) → Option<RenderPassDepthAttachmentDescriptor>` | `depthAttachment` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `get_sample_positions` | `(&self, positions: &mut [Sam...) → UInteger` | `getSamplePositions` |
| `imageblock_sample_length` | `(&self) → UInteger` | `imageblockSampleLength` |
| `rasterization_rate_map` | `(&self) → *mut c_void` | `rasterizationRateMap` |
| `rasterization_rate_map_raw` | `(&self) → *mut c_void` | — |
| `render_target_array_length` | `(&self) → UInteger` | `renderTargetArrayLength` |
| `render_target_height` | `(&self) → UInteger` | `renderTargetHeight` |
| `render_target_width` | `(&self) → UInteger` | `renderTargetWidth` |
| `stencil_attachment` | `(&self) → Option<RenderPassStencilAttachmentDescriptor>` | `stencilAttachment` |
| `support_color_attachment_mapping` | `(&self) → bool` | `supportColorAttachmentMapping` |
| `threadgroup_memory_length` | `(&self) → UInteger` | `threadgroupMemoryLength` |
| `tile_height` | `(&self) → UInteger` | `tileHeight` |
| `tile_width` | `(&self) → UInteger` | `tileWidth` |
| `visibility_result_buffer` | `(&self) → Option<Buffer>` | `visibilityResultBuffer` |
| `visibility_result_type` | `(&self) → VisibilityResultType` | `visibilityResultType` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_default_raster_sample_count` | `(&self, count: UInteger) → void` | `setDefaultRasterSampleCount` |
| `set_depth_attachment` | `(&self, attachment: &RenderP...) → void` | `setDepthAttachment` |
| `set_imageblock_sample_length` | `(&self, length: UInteger) → void` | `setImageblockSampleLength` |
| `set_rasterization_rate_map` | `(&self, rate_map: *const c_void) → void` | `setRasterizationRateMap` |
| `set_rasterization_rate_map_raw` | `(&self, map: *const c_void) → void` | — |
| `set_render_target_array_length` | `(&self, length: UInteger) → void` | `setRenderTargetArrayLength` |
| `set_render_target_height` | `(&self, height: UInteger) → void` | `setRenderTargetHeight` |
| `set_render_target_width` | `(&self, width: UInteger) → void` | `setRenderTargetWidth` |
| `set_sample_positions` | `(&self, positions: &[SampleP...) → void` | `setSamplePositions` |
| `set_stencil_attachment` | `(&self, attachment: &RenderP...) → void` | `setStencilAttachment` |
| `set_support_color_attachment_mapping` | `(&self, support: bool) → void` | `setSupportColorAttachmentMapping` |
| `set_threadgroup_memory_length` | `(&self, length: UInteger) → void` | `setThreadgroupMemoryLength` |
| `set_tile_height` | `(&self, height: UInteger) → void` | `setTileHeight` |
| `set_tile_width` | `(&self, width: UInteger) → void` | `setTileWidth` |
| `set_visibility_result_buffer` | `(&self, buffer: &Buffer) → void` | `setVisibilityResultBuffer` |
| `set_visibility_result_type` | `(&self, result_type: Visibil...) → void` | `setVisibilityResultType` |

---

### `RenderPassDescriptor`

C++ equivalent: `NS::RenderPassDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `color_attachments` | `(&self) → Option<RenderPassColorAttachmentDescriptorArray>` | `colorAttachments` |
| `default_raster_sample_count` | `(&self) → UInteger` | `defaultRasterSampleCount` |
| `depth_attachment` | `(&self) → Option<RenderPassDepthAttachmentDescriptor>` | `depthAttachment` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `get_sample_positions` | `(&self, positions: &mut [Sam...) → UInteger` | `getSamplePositions` |
| `imageblock_sample_length` | `(&self) → UInteger` | `imageblockSampleLength` |
| `rasterization_rate_map` | `(&self) → *mut c_void` | `rasterizationRateMap` |
| `rasterization_rate_map_raw` | `(&self) → *mut c_void` | — |
| `render_target_array_length` | `(&self) → UInteger` | `renderTargetArrayLength` |
| `render_target_height` | `(&self) → UInteger` | `renderTargetHeight` |
| `render_target_width` | `(&self) → UInteger` | `renderTargetWidth` |
| `stencil_attachment` | `(&self) → Option<RenderPassStencilAttachmentDescriptor>` | `stencilAttachment` |
| `support_color_attachment_mapping` | `(&self) → bool` | `supportColorAttachmentMapping` |
| `threadgroup_memory_length` | `(&self) → UInteger` | `threadgroupMemoryLength` |
| `tile_height` | `(&self) → UInteger` | `tileHeight` |
| `tile_width` | `(&self) → UInteger` | `tileWidth` |
| `visibility_result_buffer` | `(&self) → Option<Buffer>` | `visibilityResultBuffer` |
| `visibility_result_type` | `(&self) → VisibilityResultType` | `visibilityResultType` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_default_raster_sample_count` | `(&self, count: UInteger) → void` | `setDefaultRasterSampleCount` |
| `set_depth_attachment` | `(&self, attachment: &RenderP...) → void` | `setDepthAttachment` |
| `set_imageblock_sample_length` | `(&self, length: UInteger) → void` | `setImageblockSampleLength` |
| `set_rasterization_rate_map` | `(&self, rate_map: *const c_void) → void` | `setRasterizationRateMap` |
| `set_rasterization_rate_map_raw` | `(&self, map: *const c_void) → void` | — |
| `set_render_target_array_length` | `(&self, length: UInteger) → void` | `setRenderTargetArrayLength` |
| `set_render_target_height` | `(&self, height: UInteger) → void` | `setRenderTargetHeight` |
| `set_render_target_width` | `(&self, width: UInteger) → void` | `setRenderTargetWidth` |
| `set_sample_positions` | `(&self, positions: &[SampleP...) → void` | `setSamplePositions` |
| `set_stencil_attachment` | `(&self, attachment: &RenderP...) → void` | `setStencilAttachment` |
| `set_support_color_attachment_mapping` | `(&self, support: bool) → void` | `setSupportColorAttachmentMapping` |
| `set_threadgroup_memory_length` | `(&self, length: UInteger) → void` | `setThreadgroupMemoryLength` |
| `set_tile_height` | `(&self, height: UInteger) → void` | `setTileHeight` |
| `set_tile_width` | `(&self, width: UInteger) → void` | `setTileWidth` |
| `set_visibility_result_buffer` | `(&self, buffer: &Buffer) → void` | `setVisibilityResultBuffer` |
| `set_visibility_result_type` | `(&self, result_type: Visibil...) → void` | `setVisibilityResultType` |

---

### `RenderPassSampleBufferAttachmentDescriptor`

C++ equivalent: `NS::RenderPassSampleBufferAttachmentDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `end_of_fragment_sample_index` | `(&self) → UInteger` | `endOfFragmentSampleIndex` |
| `end_of_vertex_sample_index` | `(&self) → UInteger` | `endOfVertexSampleIndex` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `sample_buffer` | `(&self) → Option<crate::CounterSampleBuffer>` | `sampleBuffer` |
| `start_of_fragment_sample_index` | `(&self) → UInteger` | `startOfFragmentSampleIndex` |
| `start_of_vertex_sample_index` | `(&self) → UInteger` | `startOfVertexSampleIndex` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_end_of_fragment_sample_index` | `(&self, index: UInteger) → void` | `setEndOfFragmentSampleIndex` |
| `set_end_of_vertex_sample_index` | `(&self, index: UInteger) → void` | `setEndOfVertexSampleIndex` |
| `set_sample_buffer` | `(&self, sample_buffer: Optio...) → void` | `setSampleBuffer` |
| `set_start_of_fragment_sample_index` | `(&self, index: UInteger) → void` | `setStartOfFragmentSampleIndex` |
| `set_start_of_vertex_sample_index` | `(&self, index: UInteger) → void` | `setStartOfVertexSampleIndex` |

---

### `RenderPassSampleBufferAttachmentDescriptorArray`

C++ equivalent: `NS::RenderPassSampleBufferAttachmentDescriptorArray`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `object` | `(&self, index: UInteger) → Option<RenderPassSampleBufferAttachmentDescriptor>` | `object` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_object` | `(&self,
        attachment: ...) → void` | `setObject` |

---

### `RenderPassStencilAttachmentDescriptor`

C++ equivalent: `NS::RenderPassStencilAttachmentDescriptor`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `clear_stencil` | `(&self) → u32` | `clearStencil` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `load_action` | `(&self) → LoadAction` | — |
| `stencil_resolve_filter` | `(&self) → MultisampleStencilResolveFilter` | `stencilResolveFilter` |
| `store_action` | `(&self) → StoreAction` | — |
| `texture` | `(&self) → Option<Texture>` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_clear_stencil` | `(&self, stencil: u32) → void` | `setClearStencil` |
| `set_load_action` | `(&self, load_action: LoadAction) → void` | — |
| `set_stencil_resolve_filter` | `(&self, filter: MultisampleS...) → void` | `setStencilResolveFilter` |
| `set_store_action` | `(&self, store_action: StoreA...) → void` | — |
| `set_texture` | `(&self, texture: Option<&Tex...) → void` | — |

---

### `RenderPipelineBinaryFunctionsDescriptor`

C++ equivalent: `NS::RenderPipelineBinaryFunctionsDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `fragment_additional_binary_functions_raw` | `(&self) → *mut c_void` | `fragmentAdditionalBinaryFunctions` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `mesh_additional_binary_functions_raw` | `(&self) → *mut c_void` | `meshAdditionalBinaryFunctions` |
| `object_additional_binary_functions_raw` | `(&self) → *mut c_void` | `objectAdditionalBinaryFunctions` |
| `reset` | `(&self) → void` | `reset` |
| `tile_additional_binary_functions_raw` | `(&self) → *mut c_void` | `tileAdditionalBinaryFunctions` |
| `vertex_additional_binary_functions_raw` | `(&self) → *mut c_void` | `vertexAdditionalBinaryFunctions` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_fragment_additional_binary_functions_raw` | `(&self, functions: *const c_...) → void` | `setFragmentAdditionalBinaryFunctions` |
| `set_mesh_additional_binary_functions_raw` | `(&self, functions: *const c_...) → void` | `setMeshAdditionalBinaryFunctions` |
| `set_object_additional_binary_functions_raw` | `(&self, functions: *const c_...) → void` | `setObjectAdditionalBinaryFunctions` |
| `set_tile_additional_binary_functions_raw` | `(&self, functions: *const c_...) → void` | `setTileAdditionalBinaryFunctions` |
| `set_vertex_additional_binary_functions_raw` | `(&self, functions: *const c_...) → void` | `setVertexAdditionalBinaryFunctions` |

---

### `RenderPipelineColorAttachmentDescriptor`

C++ equivalent: `NS::RenderPipelineColorAttachmentDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `alpha_blend_operation` | `(&self) → BlendOperation` | `alphaBlendOperation` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `blending_state` | `(&self) → BlendState` | `blendingState` |
| `destination_alpha_blend_factor` | `(&self) → BlendFactor` | `destinationAlphaBlendFactor` |
| `destination_rgb_blend_factor` | `(&self) → BlendFactor` | `destinationRGBBlendFactor` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `pixel_format` | `(&self) → PixelFormat` | `pixelFormat` |
| `reset` | `(&self) → void` | `reset` |
| `rgb_blend_operation` | `(&self) → BlendOperation` | `rgbBlendOperation` |
| `source_alpha_blend_factor` | `(&self) → BlendFactor` | `sourceAlphaBlendFactor` |
| `source_rgb_blend_factor` | `(&self) → BlendFactor` | `sourceRGBBlendFactor` |
| `write_mask` | `(&self) → ColorWriteMask` | `writeMask` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_blending_enabled` | `(&self) → bool` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_alpha_blend_operation` | `(&self, operation: BlendOper...) → void` | `setAlphaBlendOperation` |
| `set_blending_enabled` | `(&self, enabled: bool) → void` | — |
| `set_blending_state` | `(&self, state: BlendState) → void` | `setBlendingState` |
| `set_destination_alpha_blend_factor` | `(&self, factor: BlendFactor) → void` | `setDestinationAlphaBlendFactor` |
| `set_destination_rgb_blend_factor` | `(&self, factor: BlendFactor) → void` | `setDestinationRGBBlendFactor` |
| `set_pixel_format` | `(&self, format: PixelFormat) → void` | `setPixelFormat` |
| `set_rgb_blend_operation` | `(&self, operation: BlendOper...) → void` | `setRgbBlendOperation` |
| `set_source_alpha_blend_factor` | `(&self, factor: BlendFactor) → void` | `setSourceAlphaBlendFactor` |
| `set_source_rgb_blend_factor` | `(&self, factor: BlendFactor) → void` | `setSourceRGBBlendFactor` |
| `set_write_mask` | `(&self, mask: ColorWriteMask) → void` | `setWriteMask` |

---

### `RenderPipelineColorAttachmentDescriptor`

C++ equivalent: `NS::RenderPipelineColorAttachmentDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `alpha_blend_operation` | `(&self) → BlendOperation` | `alphaBlendOperation` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `blending_state` | `(&self) → BlendState` | — |
| `destination_alpha_blend_factor` | `(&self) → BlendFactor` | `destinationAlphaBlendFactor` |
| `destination_rgb_blend_factor` | `(&self) → BlendFactor` | `destinationRGBBlendFactor` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `pixel_format` | `(&self) → PixelFormat` | `pixelFormat` |
| `reset` | `(&self) → void` | — |
| `rgb_blend_operation` | `(&self) → BlendOperation` | `rgbBlendOperation` |
| `source_alpha_blend_factor` | `(&self) → BlendFactor` | `sourceAlphaBlendFactor` |
| `source_rgb_blend_factor` | `(&self) → BlendFactor` | `sourceRGBBlendFactor` |
| `write_mask` | `(&self) → ColorWriteMask` | `writeMask` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_blending_enabled` | `(&self) → bool` | `blendingEnabled` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_alpha_blend_operation` | `(&self, operation: BlendOper...) → void` | `setAlphaBlendOperation` |
| `set_blending_enabled` | `(&self, enabled: bool) → void` | `setBlendingEnabled` |
| `set_blending_state` | `(&self, state: BlendState) → void` | — |
| `set_destination_alpha_blend_factor` | `(&self, factor: BlendFactor) → void` | `setDestinationAlphaBlendFactor` |
| `set_destination_rgb_blend_factor` | `(&self, factor: BlendFactor) → void` | `setDestinationRGBBlendFactor` |
| `set_pixel_format` | `(&self, format: PixelFormat) → void` | `setPixelFormat` |
| `set_rgb_blend_operation` | `(&self, operation: BlendOper...) → void` | `setRgbBlendOperation` |
| `set_source_alpha_blend_factor` | `(&self, factor: BlendFactor) → void` | `setSourceAlphaBlendFactor` |
| `set_source_rgb_blend_factor` | `(&self, factor: BlendFactor) → void` | `setSourceRGBBlendFactor` |
| `set_write_mask` | `(&self, mask: ColorWriteMask) → void` | `setWriteMask` |

---

### `RenderPipelineColorAttachmentDescriptorArray`

C++ equivalent: `NS::RenderPipelineColorAttachmentDescriptorArray`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `object` | `(&self, index: UInteger) → Option<RenderPipelineColorAttachmentDescriptor>` | `object` |
| `reset` | `(&self) → void` | `reset` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_object` | `(&self,
        attachment: ...) → void` | `setObject` |

---

### `RenderPipelineColorAttachmentDescriptorArray`

C++ equivalent: `NS::RenderPipelineColorAttachmentDescriptorArray`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `object` | `(&self, index: UInteger) → Option<RenderPipelineColorAttachmentDescriptor>` | `object` |
| `reset` | `(&self) → void` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_object` | `(&self,
        attachment: ...) → void` | `setObject` |

---

### `RenderPipelineDescriptor`

C++ equivalent: `NS::RenderPipelineDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `alpha_to_coverage_state` | `(&self) → AlphaToCoverageState` | `alphaToCoverageState` |
| `alpha_to_one_state` | `(&self) → AlphaToOneState` | `alphaToOneState` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `binary_archives_raw` | `(&self) → *mut c_void` | — |
| `color_attachment_mapping_state` | `(&self) → LogicalToPhysicalColorAttachmentMappingState` | `colorAttachmentMappingState` |
| `color_attachments` | `(&self) → Option<RenderPipelineColorAttachmentDescriptorArray>` | `colorAttachments` |
| `depth_attachment_pixel_format` | `(&self) → PixelFormat` | — |
| `fragment_buffers` | `(&self) → Option<PipelineBufferDescriptorArray>` | — |
| `fragment_function` | `(&self) → Option<crate::Function>` | — |
| `fragment_function_descriptor` | `(&self) → Option<FunctionDescriptor>` | `fragmentFunctionDescriptor` |
| `fragment_linked_functions` | `(&self) → Option<crate::LinkedFunctions>` | — |
| `fragment_preloaded_libraries_raw` | `(&self) → *mut c_void` | — |
| `fragment_static_linking_descriptor` | `(&self) → Option<StaticLinkingDescriptor>` | `fragmentStaticLinkingDescriptor` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `input_primitive_topology` | `(&self) → PrimitiveTopologyClass` | `inputPrimitiveTopology` |
| `label` | `(&self) → Option<String>` | — |
| `max_fragment_call_stack_depth` | `(&self) → UInteger` | — |
| `max_tessellation_factor` | `(&self) → UInteger` | — |
| `max_vertex_amplification_count` | `(&self) → UInteger` | `maxVertexAmplificationCount` |
| `max_vertex_call_stack_depth` | `(&self) → UInteger` | — |
| `options` | `(&self) → Option<PipelineOptions>` | — |
| `raster_sample_count` | `(&self) → UInteger` | `rasterSampleCount` |
| `reset` | `(&self) → void` | `reset` |
| `sample_count` | `(&self) → UInteger` | — |
| `shader_validation` | `(&self) → ShaderValidation` | — |
| `stencil_attachment_pixel_format` | `(&self) → PixelFormat` | — |
| `support_adding_fragment_binary_functions` | `(&self) → bool` | — |
| `support_adding_vertex_binary_functions` | `(&self) → bool` | — |
| `support_fragment_binary_linking` | `(&self) → bool` | `supportFragmentBinaryLinking` |
| `support_indirect_command_buffers` | `(&self) → IndirectCommandBufferSupportState` | `supportIndirectCommandBuffers` |
| `support_vertex_binary_linking` | `(&self) → bool` | `supportVertexBinaryLinking` |
| `tessellation_control_point_index_type` | `(&self) → TessellationControlPointIndexType` | — |
| `tessellation_factor_format` | `(&self) → TessellationFactorFormat` | — |
| `tessellation_factor_step_function` | `(&self) → TessellationFactorStepFunction` | — |
| `tessellation_output_winding_order` | `(&self) → Winding` | — |
| `tessellation_partition_mode` | `(&self) → TessellationPartitionMode` | — |
| `vertex_buffers` | `(&self) → Option<PipelineBufferDescriptorArray>` | — |
| `vertex_descriptor` | `(&self) → Option<crate::VertexDescriptor>` | `vertexDescriptor` |
| `vertex_descriptor_raw` | `(&self) → *mut c_void` | — |
| `vertex_function` | `(&self) → Option<crate::Function>` | — |
| `vertex_function_descriptor` | `(&self) → Option<FunctionDescriptor>` | `vertexFunctionDescriptor` |
| `vertex_linked_functions` | `(&self) → Option<crate::LinkedFunctions>` | — |
| `vertex_preloaded_libraries_raw` | `(&self) → *mut c_void` | — |
| `vertex_static_linking_descriptor` | `(&self) → Option<StaticLinkingDescriptor>` | `vertexStaticLinkingDescriptor` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_alpha_to_coverage_enabled` | `(&self) → bool` | — |
| `is_alpha_to_one_enabled` | `(&self) → bool` | — |
| `is_rasterization_enabled` | `(&self) → bool` | `isRasterizationEnabled` |
| `is_tessellation_factor_scale_enabled` | `(&self) → bool` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_alpha_to_coverage_enabled` | `(&self, enabled: bool) → void` | — |
| `set_alpha_to_coverage_state` | `(&self, state: AlphaToCovera...) → void` | `setAlphaToCoverageState` |
| `set_alpha_to_one_enabled` | `(&self, enabled: bool) → void` | — |
| `set_alpha_to_one_state` | `(&self, state: AlphaToOneState) → void` | `setAlphaToOneState` |
| `set_binary_archives_raw` | `(&self, archives: *const c_void) → void` | — |
| `set_color_attachment_mapping_state` | `(&self,
        state: Logic...) → void` | `setColorAttachmentMappingState` |
| `set_depth_attachment_pixel_format` | `(&self, format: PixelFormat) → void` | — |
| `set_fragment_function` | `(&self, function: Option<&cr...) → void` | — |
| `set_fragment_function_descriptor` | `(&self, descriptor: &Functio...) → void` | `setFragmentFunctionDescriptor` |
| `set_fragment_linked_functions` | `(&self, functions: Option<&c...) → void` | — |
| `set_fragment_preloaded_libraries_raw` | `(&self, libraries: *const c_...) → void` | — |
| `set_fragment_static_linking_descriptor` | `(&self, descriptor: &StaticL...) → void` | `setFragmentStaticLinkingDescriptor` |
| `set_input_primitive_topology` | `(&self, topology: PrimitiveT...) → void` | `setInputPrimitiveTopology` |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_max_fragment_call_stack_depth` | `(&self, depth: UInteger) → void` | — |
| `set_max_tessellation_factor` | `(&self, factor: UInteger) → void` | — |
| `set_max_vertex_amplification_count` | `(&self, count: UInteger) → void` | `setMaxVertexAmplificationCount` |
| `set_max_vertex_call_stack_depth` | `(&self, depth: UInteger) → void` | — |
| `set_options` | `(&self, options: &PipelineOp...) → void` | — |
| `set_raster_sample_count` | `(&self, count: UInteger) → void` | `setRasterSampleCount` |
| `set_rasterization_enabled` | `(&self, enabled: bool) → void` | `setRasterizationEnabled` |
| `set_sample_count` | `(&self, count: UInteger) → void` | — |
| `set_shader_validation` | `(&self, validation: ShaderVa...) → void` | — |
| `set_stencil_attachment_pixel_format` | `(&self, format: PixelFormat) → void` | — |
| `set_support_adding_fragment_binary_functions` | `(&self, support: bool) → void` | — |
| `set_support_adding_vertex_binary_functions` | `(&self, support: bool) → void` | — |
| `set_support_fragment_binary_linking` | `(&self, support: bool) → void` | `setSupportFragmentBinaryLinking` |
| `set_support_indirect_command_buffers` | `(&self, state: IndirectComma...) → void` | `setSupportIndirectCommandBuffers` |
| `set_support_vertex_binary_linking` | `(&self, support: bool) → void` | `setSupportVertexBinaryLinking` |
| `set_tessellation_control_point_index_type` | `(&self,
        index_type: ...) → void` | — |
| `set_tessellation_factor_format` | `(&self, format: Tessellation...) → void` | — |
| `set_tessellation_factor_scale_enabled` | `(&self, enabled: bool) → void` | — |
| `set_tessellation_factor_step_function` | `(&self, func: TessellationFa...) → void` | — |
| `set_tessellation_output_winding_order` | `(&self, winding: Winding) → void` | — |
| `set_tessellation_partition_mode` | `(&self, mode: TessellationPa...) → void` | — |
| `set_vertex_descriptor` | `(&self, descriptor: Option<&...) → void` | `setVertexDescriptor` |
| `set_vertex_descriptor_raw` | `(&self, descriptor: *const c...) → void` | — |
| `set_vertex_function` | `(&self, function: Option<&cr...) → void` | — |
| `set_vertex_function_descriptor` | `(&self, descriptor: &Functio...) → void` | `setVertexFunctionDescriptor` |
| `set_vertex_linked_functions` | `(&self, functions: Option<&c...) → void` | — |
| `set_vertex_preloaded_libraries_raw` | `(&self, libraries: *const c_...) → void` | — |
| `set_vertex_static_linking_descriptor` | `(&self, descriptor: &StaticL...) → void` | `setVertexStaticLinkingDescriptor` |

---

### `RenderPipelineDescriptor`

C++ equivalent: `NS::RenderPipelineDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `alpha_to_coverage_state` | `(&self) → AlphaToCoverageState` | — |
| `alpha_to_one_state` | `(&self) → AlphaToOneState` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `binary_archives_raw` | `(&self) → *mut c_void` | `binaryArchives` |
| `color_attachment_mapping_state` | `(&self) → LogicalToPhysicalColorAttachmentMappingState` | — |
| `color_attachments` | `(&self) → Option<RenderPipelineColorAttachmentDescriptorArray>` | `colorAttachments` |
| `depth_attachment_pixel_format` | `(&self) → PixelFormat` | `depthAttachmentPixelFormat` |
| `fragment_buffers` | `(&self) → Option<PipelineBufferDescriptorArray>` | `fragmentBuffers` |
| `fragment_function` | `(&self) → Option<crate::Function>` | `fragmentFunction` |
| `fragment_function_descriptor` | `(&self) → Option<FunctionDescriptor>` | — |
| `fragment_linked_functions` | `(&self) → Option<crate::LinkedFunctions>` | `fragmentLinkedFunctions` |
| `fragment_preloaded_libraries_raw` | `(&self) → *mut c_void` | `fragmentPreloadedLibraries` |
| `fragment_static_linking_descriptor` | `(&self) → Option<StaticLinkingDescriptor>` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `input_primitive_topology` | `(&self) → PrimitiveTopologyClass` | `inputPrimitiveTopology` |
| `label` | `(&self) → Option<String>` | `label` |
| `max_fragment_call_stack_depth` | `(&self) → UInteger` | `maxFragmentCallStackDepth` |
| `max_tessellation_factor` | `(&self) → UInteger` | `maxTessellationFactor` |
| `max_vertex_amplification_count` | `(&self) → UInteger` | `maxVertexAmplificationCount` |
| `max_vertex_call_stack_depth` | `(&self) → UInteger` | `maxVertexCallStackDepth` |
| `options` | `(&self) → Option<PipelineOptions>` | — |
| `raster_sample_count` | `(&self) → UInteger` | `rasterSampleCount` |
| `reset` | `(&self) → void` | `reset` |
| `sample_count` | `(&self) → UInteger` | `sampleCount` |
| `shader_validation` | `(&self) → ShaderValidation` | `shaderValidation` |
| `stencil_attachment_pixel_format` | `(&self) → PixelFormat` | `stencilAttachmentPixelFormat` |
| `support_adding_fragment_binary_functions` | `(&self) → bool` | `supportAddingFragmentBinaryFunctions` |
| `support_adding_vertex_binary_functions` | `(&self) → bool` | `supportAddingVertexBinaryFunctions` |
| `support_fragment_binary_linking` | `(&self) → bool` | — |
| `support_indirect_command_buffers` | `(&self) → IndirectCommandBufferSupportState` | `supportIndirectCommandBuffers` |
| `support_vertex_binary_linking` | `(&self) → bool` | — |
| `tessellation_control_point_index_type` | `(&self) → TessellationControlPointIndexType` | `tessellationControlPointIndexType` |
| `tessellation_factor_format` | `(&self) → TessellationFactorFormat` | `tessellationFactorFormat` |
| `tessellation_factor_step_function` | `(&self) → TessellationFactorStepFunction` | `tessellationFactorStepFunction` |
| `tessellation_output_winding_order` | `(&self) → Winding` | `tessellationOutputWindingOrder` |
| `tessellation_partition_mode` | `(&self) → TessellationPartitionMode` | `tessellationPartitionMode` |
| `vertex_buffers` | `(&self) → Option<PipelineBufferDescriptorArray>` | `vertexBuffers` |
| `vertex_descriptor` | `(&self) → Option<crate::VertexDescriptor>` | `vertexDescriptor` |
| `vertex_descriptor_raw` | `(&self) → *mut c_void` | — |
| `vertex_function` | `(&self) → Option<crate::Function>` | `vertexFunction` |
| `vertex_function_descriptor` | `(&self) → Option<FunctionDescriptor>` | — |
| `vertex_linked_functions` | `(&self) → Option<crate::LinkedFunctions>` | `vertexLinkedFunctions` |
| `vertex_preloaded_libraries_raw` | `(&self) → *mut c_void` | `vertexPreloadedLibraries` |
| `vertex_static_linking_descriptor` | `(&self) → Option<StaticLinkingDescriptor>` | — |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_alpha_to_coverage_enabled` | `(&self) → bool` | `alphaToCoverageEnabled` |
| `is_alpha_to_one_enabled` | `(&self) → bool` | `alphaToOneEnabled` |
| `is_rasterization_enabled` | `(&self) → bool` | `isRasterizationEnabled` |
| `is_tessellation_factor_scale_enabled` | `(&self) → bool` | `isTessellationFactorScaleEnabled` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_alpha_to_coverage_enabled` | `(&self, enabled: bool) → void` | `setAlphaToCoverageEnabled` |
| `set_alpha_to_coverage_state` | `(&self, state: AlphaToCovera...) → void` | — |
| `set_alpha_to_one_enabled` | `(&self, enabled: bool) → void` | `setAlphaToOneEnabled` |
| `set_alpha_to_one_state` | `(&self, state: AlphaToOneState) → void` | — |
| `set_binary_archives_raw` | `(&self, archives: *const c_void) → void` | `setBinaryArchives` |
| `set_color_attachment_mapping_state` | `(&self,
        state: Logic...) → void` | — |
| `set_depth_attachment_pixel_format` | `(&self, format: PixelFormat) → void` | `setDepthAttachmentPixelFormat` |
| `set_fragment_function` | `(&self, function: Option<&cr...) → void` | `setFragmentFunction` |
| `set_fragment_function_descriptor` | `(&self, descriptor: &Functio...) → void` | — |
| `set_fragment_linked_functions` | `(&self, functions: Option<&c...) → void` | `setFragmentLinkedFunctions` |
| `set_fragment_preloaded_libraries_raw` | `(&self, libraries: *const c_...) → void` | `setFragmentPreloadedLibraries` |
| `set_fragment_static_linking_descriptor` | `(&self, descriptor: &StaticL...) → void` | — |
| `set_input_primitive_topology` | `(&self, topology: PrimitiveT...) → void` | `setInputPrimitiveTopology` |
| `set_label` | `(&self, label: &str) → void` | `setLabel` |
| `set_max_fragment_call_stack_depth` | `(&self, depth: UInteger) → void` | `setMaxFragmentCallStackDepth` |
| `set_max_tessellation_factor` | `(&self, factor: UInteger) → void` | `setMaxTessellationFactor` |
| `set_max_vertex_amplification_count` | `(&self, count: UInteger) → void` | `setMaxVertexAmplificationCount` |
| `set_max_vertex_call_stack_depth` | `(&self, depth: UInteger) → void` | `setMaxVertexCallStackDepth` |
| `set_options` | `(&self, options: &PipelineOp...) → void` | — |
| `set_raster_sample_count` | `(&self, count: UInteger) → void` | `setRasterSampleCount` |
| `set_rasterization_enabled` | `(&self, enabled: bool) → void` | `setRasterizationEnabled` |
| `set_sample_count` | `(&self, count: UInteger) → void` | `setSampleCount` |
| `set_shader_validation` | `(&self, validation: ShaderVa...) → void` | `setShaderValidation` |
| `set_stencil_attachment_pixel_format` | `(&self, format: PixelFormat) → void` | `setStencilAttachmentPixelFormat` |
| `set_support_adding_fragment_binary_functions` | `(&self, support: bool) → void` | `setSupportAddingFragmentBinaryFunctions` |
| `set_support_adding_vertex_binary_functions` | `(&self, support: bool) → void` | `setSupportAddingVertexBinaryFunctions` |
| `set_support_fragment_binary_linking` | `(&self, support: bool) → void` | — |
| `set_support_indirect_command_buffers` | `(&self, state: IndirectComma...) → void` | `setSupportIndirectCommandBuffers` |
| `set_support_vertex_binary_linking` | `(&self, support: bool) → void` | — |
| `set_tessellation_control_point_index_type` | `(&self,
        index_type: ...) → void` | `setTessellationControlPointIndexType` |
| `set_tessellation_factor_format` | `(&self, format: Tessellation...) → void` | `setTessellationFactorFormat` |
| `set_tessellation_factor_scale_enabled` | `(&self, enabled: bool) → void` | `setTessellationFactorScaleEnabled` |
| `set_tessellation_factor_step_function` | `(&self, func: TessellationFa...) → void` | `setTessellationFactorStepFunction` |
| `set_tessellation_output_winding_order` | `(&self, winding: Winding) → void` | `setTessellationOutputWindingOrder` |
| `set_tessellation_partition_mode` | `(&self, mode: TessellationPa...) → void` | `setTessellationPartitionMode` |
| `set_vertex_descriptor` | `(&self, descriptor: Option<&...) → void` | `setVertexDescriptor` |
| `set_vertex_descriptor_raw` | `(&self, descriptor: *const c...) → void` | — |
| `set_vertex_function` | `(&self, function: Option<&cr...) → void` | `setVertexFunction` |
| `set_vertex_function_descriptor` | `(&self, descriptor: &Functio...) → void` | — |
| `set_vertex_linked_functions` | `(&self, functions: Option<&c...) → void` | `setVertexLinkedFunctions` |
| `set_vertex_preloaded_libraries_raw` | `(&self, libraries: *const c_...) → void` | `setVertexPreloadedLibraries` |
| `set_vertex_static_linking_descriptor` | `(&self, descriptor: &StaticL...) → void` | — |

---

### `RenderPipelineDynamicLinkingDescriptor`

C++ equivalent: `NS::RenderPipelineDynamicLinkingDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `fragment_linking_descriptor` | `(&self) → Option<PipelineStageDynamicLinkingDescriptor>` | `fragmentLinkingDescriptor` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `mesh_linking_descriptor` | `(&self) → Option<PipelineStageDynamicLinkingDescriptor>` | `meshLinkingDescriptor` |
| `object_linking_descriptor` | `(&self) → Option<PipelineStageDynamicLinkingDescriptor>` | `objectLinkingDescriptor` |
| `tile_linking_descriptor` | `(&self) → Option<PipelineStageDynamicLinkingDescriptor>` | `tileLinkingDescriptor` |
| `vertex_linking_descriptor` | `(&self) → Option<PipelineStageDynamicLinkingDescriptor>` | `vertexLinkingDescriptor` |

---

### `RenderPipelineFunctionsDescriptor`

C++ equivalent: `NS::RenderPipelineFunctionsDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `fragment_additional_binary_functions_raw` | `(&self) → *mut c_void` | `fragmentAdditionalBinaryFunctions` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `tile_additional_binary_functions_raw` | `(&self) → *mut c_void` | `tileAdditionalBinaryFunctions` |
| `vertex_additional_binary_functions_raw` | `(&self) → *mut c_void` | `vertexAdditionalBinaryFunctions` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_fragment_additional_binary_functions_raw` | `(&self, functions: *const c_...) → void` | `setFragmentAdditionalBinaryFunctions` |
| `set_tile_additional_binary_functions_raw` | `(&self, functions: *const c_...) → void` | `setTileAdditionalBinaryFunctions` |
| `set_vertex_additional_binary_functions_raw` | `(&self, functions: *const c_...) → void` | `setVertexAdditionalBinaryFunctions` |

---

### `RenderPipelineReflection`

C++ equivalent: `NS::RenderPipelineReflection`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `fragment_arguments_raw` | `(&self) → *mut c_void` | `fragmentArguments` |
| `fragment_bindings_raw` | `(&self) → *mut c_void` | `fragmentBindings` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `mesh_bindings_raw` | `(&self) → *mut c_void` | `meshBindings` |
| `object_bindings_raw` | `(&self) → *mut c_void` | `objectBindings` |
| `tile_arguments_raw` | `(&self) → *mut c_void` | `tileArguments` |
| `tile_bindings_raw` | `(&self) → *mut c_void` | `tileBindings` |
| `vertex_arguments_raw` | `(&self) → *mut c_void` | `vertexArguments` |
| `vertex_bindings_raw` | `(&self) → *mut c_void` | `vertexBindings` |

---

### `RenderPipelineState`

C++ equivalent: `NS::RenderPipelineState`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new_intersection_function_table` | `(&self,
        descriptor: ...) → Option<crate::IntersectionFunctionTable>` | `newIntersectionFunctionTable` |
| `new_render_pipeline_state` | `(&self,
        additional_f...) → Result<RenderPipelineState, metal_foundation::Error>` | `newRenderPipelineDescriptor` |
| `new_visible_function_table` | `(&self,
        descriptor: ...) → Option<crate::VisibleFunctionTable>` | `newVisibleFunctionTable` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `device` | `(&self) → crate::Device` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function_handle_with_function` | `(&self,
        function: &c...) → Option<crate::FunctionHandle>` | — |
| `function_handle_with_name` | `(&self,
        name: &str,
...) → Option<crate::FunctionHandle>` | `functionHandle` |
| `gpu_resource_id` | `(&self) → ResourceID` | `gpuResourceID` |
| `imageblock_memory_length` | `(&self, dimensions: Size) → UInteger` | `imageblockMemoryLength` |
| `imageblock_sample_length` | `(&self) → UInteger` | `imageblockSampleLength` |
| `label` | `(&self) → Option<String>` | `label` |
| `max_total_threadgroups_per_mesh_grid` | `(&self) → UInteger` | `maxTotalThreadgroupsPerMeshGrid` |
| `max_total_threads_per_mesh_threadgroup` | `(&self) → UInteger` | `maxTotalThreadsPerMeshThreadgroup` |
| `max_total_threads_per_object_threadgroup` | `(&self) → UInteger` | `maxTotalThreadsPerObjectThreadgroup` |
| `max_total_threads_per_threadgroup` | `(&self) → UInteger` | `maxTotalThreadsPerThreadgroup` |
| `mesh_thread_execution_width` | `(&self) → UInteger` | `meshThreadExecutionWidth` |
| `object_thread_execution_width` | `(&self) → UInteger` | `objectThreadExecutionWidth` |
| `reflection` | `(&self) → Option<RenderPipelineReflection>` | `reflection` |
| `required_threads_per_mesh_threadgroup` | `(&self) → Size` | `requiredThreadsPerMeshThreadgroup` |
| `required_threads_per_object_threadgroup` | `(&self) → Size` | `requiredThreadsPerObjectThreadgroup` |
| `required_threads_per_tile_threadgroup` | `(&self) → Size` | `requiredThreadsPerTileThreadgroup` |
| `shader_validation` | `(&self) → ShaderValidation` | `shaderValidation` |
| `support_indirect_command_buffers` | `(&self) → bool` | `supportIndirectCommandBuffers` |
| `threadgroup_size_matches_tile_size` | `(&self) → bool` | `threadgroupSizeMatchesTileSize` |

---

### `ResidencySet`

C++ equivalent: `NS::ResidencySet`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `add_allocation_ptr` | `(&self, allocation: *const c...) → void` | `addAllocation` |
| `add_allocations_ptr` | `(&self, allocations: *const ...) → void` | `addAllocations` |
| `all_allocations_ptr` | `(&self) → *const c_void` | `allAllocations` |
| `allocated_size` | `(&self) → u64` | `allocatedSize` |
| `allocation_count` | `(&self) → UInteger` | `allocationCount` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `commit` | `(&self) → void` | `commit` |
| `contains_allocation_ptr` | `(&self, allocation: *const c...) → bool` | `containsAllocation` |
| `device` | `(&self) → Device` | `device` |
| `end_residency` | `(&self) → void` | `endResidency` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |
| `remove_all_allocations` | `(&self) → void` | `removeAllAllocations` |
| `remove_allocation_ptr` | `(&self, allocation: *const c...) → void` | `removeAllocation` |
| `remove_allocations_ptr` | `(&self, allocations: *const ...) → void` | `removeAllocations` |
| `request_residency` | `(&self) → void` | `requestResidency` |

---

### `ResidencySetDescriptor`

C++ equivalent: `NS::ResidencySetDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `initial_capacity` | `(&self) → UInteger` | `initialCapacity` |
| `label` | `(&self) → Option<String>` | `label` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_initial_capacity` | `(&self, capacity: UInteger) → void` | `setInitialCapacity` |
| `set_label` | `(&self, label: &str) → void` | `setLabel` |

---

### `ResourceStateCommandEncoder`

C++ equivalent: `NS::ResourceStateCommandEncoder`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `barrier_after_queue_stages` | `(&self,
        after_stages...) → void` | — |
| `command_buffer` | `(&self) → crate::CommandBuffer` | — |
| `device` | `(&self) → crate::Device` | — |
| `end_encoding` | `(&self) → void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `insert_debug_signpost` | `(&self, string: &str) → void` | — |
| `label` | `(&self) → Option<String>` | — |
| `move_texture_mappings_from_texture` | `(&self,
        source_textu...) → void` | `moveTextureMappingsFromTexture` |
| `pop_debug_group` | `(&self) → void` | — |
| `push_debug_group` | `(&self, string: &str) → void` | — |
| `update_fence` | `(&self, fence: &Fence) → void` | `updateFence` |
| `update_texture_mapping` | `(&self,
        texture: &Te...) → void` | `updateTextureMapping` |
| `update_texture_mapping_indirect` | `(&self,
        texture: &Te...) → void` | — |
| `update_texture_mappings` | `(&self,
        texture: &Te...) → void` | `updateTextureMappings` |
| `wait_for_fence` | `(&self, fence: &Fence) → void` | `waitForFence` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | — |

---

### `ResourceStatePassDescriptor`

C++ equivalent: `NS::ResourceStatePassDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `sample_buffer_attachments` | `(&self) → Option<ResourceStatePassSampleBufferAttachmentDescriptorArray>` | `sampleBufferAttachments` |

---

### `ResourceStatePassSampleBufferAttachmentDescriptor`

C++ equivalent: `NS::ResourceStatePassSampleBufferAttachmentDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `end_of_encoder_sample_index` | `(&self) → UInteger` | `endOfEncoderSampleIndex` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `sample_buffer` | `(&self) → Option<CounterSampleBuffer>` | `sampleBuffer` |
| `start_of_encoder_sample_index` | `(&self) → UInteger` | `startOfEncoderSampleIndex` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_end_of_encoder_sample_index` | `(&self, index: UInteger) → void` | `setEndOfEncoderSampleIndex` |
| `set_sample_buffer` | `(&self, sample_buffer: Optio...) → void` | `setSampleBuffer` |
| `set_start_of_encoder_sample_index` | `(&self, index: UInteger) → void` | `setStartOfEncoderSampleIndex` |

---

### `ResourceStatePassSampleBufferAttachmentDescriptorArray`

C++ equivalent: `NS::ResourceStatePassSampleBufferAttachmentDescriptorArray`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `object` | `(&self, index: UInteger) → Option<ResourceStatePassSampleBufferAttachmentDescriptor>` | `object` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_object` | `(&self, descriptor: Option<&...) → void` | `setObject` |

---

### `ResourceViewPoolDescriptor`

C++ equivalent: `NS::ResourceViewPoolDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |
| `resource_view_count` | `(&self) → UInteger` | `resourceViewCount` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | `setLabel` |
| `set_resource_view_count` | `(&self, count: UInteger) → void` | `setResourceViewCount` |

---

### `SamplerDescriptor`

C++ equivalent: `NS::SamplerDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `border_color` | `(&self) → SamplerBorderColor` | `borderColor` |
| `compare_function` | `(&self) → CompareFunction` | `compareFunction` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |
| `lod_average` | `(&self) → bool` | `lodAverage` |
| `lod_bias` | `(&self) → f32` | `lodBias` |
| `lod_max_clamp` | `(&self) → f32` | `lodMaxClamp` |
| `lod_min_clamp` | `(&self) → f32` | `lodMinClamp` |
| `mag_filter` | `(&self) → SamplerMinMagFilter` | `magFilter` |
| `max_anisotropy` | `(&self) → UInteger` | `maxAnisotropy` |
| `min_filter` | `(&self) → SamplerMinMagFilter` | `minFilter` |
| `mip_filter` | `(&self) → SamplerMipFilter` | `mipFilter` |
| `normalized_coordinates` | `(&self) → bool` | `normalizedCoordinates` |
| `r_address_mode` | `(&self) → SamplerAddressMode` | `rAddressMode` |
| `reduction_mode` | `(&self) → SamplerReductionMode` | `reductionMode` |
| `s_address_mode` | `(&self) → SamplerAddressMode` | `sAddressMode` |
| `support_argument_buffers` | `(&self) → bool` | `supportArgumentBuffers` |
| `t_address_mode` | `(&self) → SamplerAddressMode` | `tAddressMode` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_border_color` | `(&self, color: SamplerBorder...) → void` | `setBorderColor` |
| `set_compare_function` | `(&self, func: CompareFunction) → void` | `setCompareFunction` |
| `set_label` | `(&self, label: &str) → void` | `setLabel` |
| `set_lod_average` | `(&self, average: bool) → void` | `setLodAverage` |
| `set_lod_bias` | `(&self, bias: f32) → void` | `setLodBias` |
| `set_lod_max_clamp` | `(&self, clamp: f32) → void` | `setLodMaxClamp` |
| `set_lod_min_clamp` | `(&self, clamp: f32) → void` | `setLodMinClamp` |
| `set_mag_filter` | `(&self, filter: SamplerMinMa...) → void` | `setMagFilter` |
| `set_max_anisotropy` | `(&self, max: UInteger) → void` | `setMaxAnisotropy` |
| `set_min_filter` | `(&self, filter: SamplerMinMa...) → void` | `setMinFilter` |
| `set_mip_filter` | `(&self, filter: SamplerMipFi...) → void` | `setMipFilter` |
| `set_normalized_coordinates` | `(&self, normalized: bool) → void` | `setNormalizedCoordinates` |
| `set_r_address_mode` | `(&self, mode: SamplerAddress...) → void` | `setRAddressMode` |
| `set_reduction_mode` | `(&self, mode: SamplerReducti...) → void` | `setReductionMode` |
| `set_s_address_mode` | `(&self, mode: SamplerAddress...) → void` | `setSAddressMode` |
| `set_support_argument_buffers` | `(&self, support: bool) → void` | `setSupportArgumentBuffers` |
| `set_t_address_mode` | `(&self, mode: SamplerAddress...) → void` | `setTAddressMode` |

---

### `SamplerState`

C++ equivalent: `NS::SamplerState`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `device` | `(&self) → crate::Device` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `gpu_resource_id` | `(&self) → ResourceID` | `gpuResourceID` |
| `label` | `(&self) → Option<String>` | `label` |

---

### `SharedEvent`

C++ equivalent: `NS::SharedEvent`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new_shared_event_handle` | `(&self) → Option<SharedEventHandle>` | `newSharedEventHandle` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `device` | `(&self) → crate::Device` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | — |
| `notify_listener` | `(&self,
        listener: *c...) → void` | `notifyListener` |
| `signaled_value` | `(&self) → u64` | `signaledValue` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | — |
| `set_signaled_value` | `(&self, value: u64) → void` | `setSignaledValue` |

---

### `SharedEventHandle`

C++ equivalent: `NS::SharedEventHandle`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |

---

### `SharedEventListener`

C++ equivalent: `NS::SharedEventListener`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `dispatch_queue_raw` | `(&self) → *mut c_void` | `dispatchQueue` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `shared` | `() → Option<Self>` | `sharedListener` |

---

### `SharedTextureHandle`

C++ equivalent: `NS::SharedTextureHandle`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `device` | `(&self) → crate::Device` | `device` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |

---

### `SpecializedFunctionDescriptor`

C++ equivalent: `NS::SpecializedFunctionDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `constant_values_raw` | `(&self) → *mut c_void` | `constantValues` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function_descriptor` | `(&self) → Option<FunctionDescriptor>` | `functionDescriptor` |
| `specialized_name` | `(&self) → Option<String>` | `specializedName` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_constant_values_raw` | `(&self, values: *const c_void) → void` | `setConstantValues` |
| `set_function_descriptor` | `(&self, descriptor: &Functio...) → void` | `setFunctionDescriptor` |
| `set_specialized_name` | `(&self, name: &str) → void` | `setSpecializedName` |

---

### `StageInputOutputDescriptor`

C++ equivalent: `NS::StageInputOutputDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `attributes` | `(&self) → Option<AttributeDescriptorArray>` | `attributes` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index_buffer_index` | `(&self) → UInteger` | `indexBufferIndex` |
| `index_type` | `(&self) → IndexType` | `indexType` |
| `layouts` | `(&self) → Option<BufferLayoutDescriptorArray>` | `layouts` |
| `reset` | `(&self) → void` | `reset` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_index_buffer_index` | `(&self, index: UInteger) → void` | `setIndexBufferIndex` |
| `set_index_type` | `(&self, index_type: IndexType) → void` | `setIndexType` |

---

### `StaticLinkingDescriptor`

C++ equivalent: `NS::StaticLinkingDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function_descriptors_raw` | `(&self) → *mut c_void` | `functionDescriptors` |
| `groups_raw` | `(&self) → *mut c_void` | `groups` |
| `private_function_descriptors_raw` | `(&self) → *mut c_void` | `privateFunctionDescriptors` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_function_descriptors_raw` | `(&self, descriptors: *const ...) → void` | `setFunctionDescriptors` |
| `set_groups_raw` | `(&self, groups: *const c_void) → void` | `setGroups` |
| `set_private_function_descriptors_raw` | `(&self, descriptors: *const ...) → void` | `setPrivateFunctionDescriptors` |

---

### `StencilDescriptor`

C++ equivalent: `NS::StencilDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `depth_failure_operation` | `(&self) → StencilOperation` | `depthFailureOperation` |
| `depth_stencil_pass_operation` | `(&self) → StencilOperation` | `depthStencilPassOperation` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `read_mask` | `(&self) → u32` | `readMask` |
| `stencil_compare_function` | `(&self) → CompareFunction` | `stencilCompareFunction` |
| `stencil_failure_operation` | `(&self) → StencilOperation` | `stencilFailureOperation` |
| `write_mask` | `(&self) → u32` | `writeMask` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_depth_failure_operation` | `(&self, op: StencilOperation) → void` | `setDepthFailureOperation` |
| `set_depth_stencil_pass_operation` | `(&self, op: StencilOperation) → void` | `setDepthStencilPassOperation` |
| `set_read_mask` | `(&self, mask: u32) → void` | `setReadMask` |
| `set_stencil_compare_function` | `(&self, func: CompareFunction) → void` | `setStencilCompareFunction` |
| `set_stencil_failure_operation` | `(&self, op: StencilOperation) → void` | `setStencilFailureOperation` |
| `set_write_mask` | `(&self, mask: u32) → void` | `setWriteMask` |

---

### `StitchedFunctionDescriptor`

C++ equivalent: `NS::StitchedFunctionDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function_descriptors_raw` | `(&self) → *mut c_void` | `functionDescriptors` |
| `function_graph` | `(&self) → Option<FunctionStitchingGraph>` | `functionGraph` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_function_descriptors_raw` | `(&self, descriptors: *const ...) → void` | `setFunctionDescriptors` |
| `set_function_graph` | `(&self, graph: &FunctionStit...) → void` | `setFunctionGraph` |

---

### `StitchedLibraryDescriptor`

C++ equivalent: `NS::StitchedLibraryDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `binary_archives_ptr` | `(&self) → *const c_void` | `binaryArchives` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function_graphs_ptr` | `(&self) → *const c_void` | `functionGraphs` |
| `functions_ptr` | `(&self) → *const c_void` | `functions` |
| `options` | `(&self) → StitchedLibraryOptions` | `options` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_binary_archives_ptr` | `(&self, binary_archives: *co...) → void` | `setBinaryArchives` |
| `set_function_graphs_ptr` | `(&self, function_graphs: *co...) → void` | `setFunctionGraphs` |
| `set_functions_ptr` | `(&self, functions: *const c_...) → void` | `setFunctions` |
| `set_options` | `(&self, options: StitchedLib...) → void` | `setOptions` |

---

### `StructMember`

C++ equivalent: `NS::StructMember`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `argument_index` | `(&self) → UInteger` | `argumentIndex` |
| `array_type` | `(&self) → Option<ArrayType>` | `arrayType` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `data_type` | `(&self) → DataType` | `dataType` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `name` | `(&self) → Option<String>` | `name` |
| `offset` | `(&self) → UInteger` | `offset` |
| `pointer_type` | `(&self) → Option<PointerType>` | `pointerType` |
| `struct_type` | `(&self) → Option<StructType>` | `structType` |
| `tensor_reference_type` | `(&self) → Option<TensorReferenceType>` | `tensorReferenceType` |
| `texture_reference_type` | `(&self) → Option<TextureReferenceType>` | `textureReferenceType` |

---

### `StructType`

C++ equivalent: `NS::StructType`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `data_type` | `(&self) → DataType` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `member_by_name` | `(&self, name: &str) → Option<StructMember>` | `memberByName` |
| `members_ptr` | `(&self) → *const c_void` | `members` |

---

### `Tensor`

C++ equivalent: `NS::Tensor`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `buffer` | `(&self) → Option<Buffer>` | `buffer` |
| `buffer_offset` | `(&self) → UInteger` | `bufferOffset` |
| `data_type` | `(&self) → TensorDataType` | `dataType` |
| `dimensions` | `(&self) → Option<TensorExtents>` | `dimensions` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `get_bytes` | `(&self,
        bytes: *mut ...) → void` | `getBytes` |
| `gpu_resource_id` | `(&self) → ResourceID` | `gpuResourceID` |
| `replace_slice_origin` | `(&self,
        slice_origin...) → void` | `replaceSliceOrigin` |
| `strides` | `(&self) → Option<TensorExtents>` | `strides` |
| `usage` | `(&self) → TensorUsage` | `usage` |

---

### `TensorBinding`

C++ equivalent: `NS::TensorBinding`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `access` | `(&self) → BindingAccess` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `binding_type` | `(&self) → BindingType` | — |
| `dimensions_ptr` | `(&self) → *const c_void` | `dimensions` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index` | `(&self) → UInteger` | — |
| `index_type` | `(&self) → DataType` | `indexType` |
| `name` | `(&self) → Option<String>` | — |
| `tensor_data_type` | `(&self) → TensorDataType` | `tensorDataType` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_argument` | `(&self) → bool` | — |
| `is_used` | `(&self) → bool` | — |

---

### `TensorDescriptor`

C++ equivalent: `NS::TensorDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `cpu_cache_mode` | `(&self) → CPUCacheMode` | `cpuCacheMode` |
| `data_type` | `(&self) → TensorDataType` | `dataType` |
| `dimensions` | `(&self) → Option<TensorExtents>` | `dimensions` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `hazard_tracking_mode` | `(&self) → HazardTrackingMode` | `hazardTrackingMode` |
| `resource_options` | `(&self) → ResourceOptions` | `resourceOptions` |
| `storage_mode` | `(&self) → StorageMode` | `storageMode` |
| `strides` | `(&self) → Option<TensorExtents>` | `strides` |
| `usage` | `(&self) → TensorUsage` | `usage` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_cpu_cache_mode` | `(&self, cpu_cache_mode: CPUC...) → void` | `setCpuCacheMode` |
| `set_data_type` | `(&self, data_type: TensorDat...) → void` | `setDataType` |
| `set_dimensions` | `(&self, dimensions: &TensorE...) → void` | `setDimensions` |
| `set_hazard_tracking_mode` | `(&self, hazard_tracking_mode...) → void` | `setHazardTrackingMode` |
| `set_resource_options` | `(&self, resource_options: Re...) → void` | `setResourceOptions` |
| `set_storage_mode` | `(&self, storage_mode: Storag...) → void` | `setStorageMode` |
| `set_strides` | `(&self, strides: &TensorExtents) → void` | `setStrides` |
| `set_usage` | `(&self, usage: TensorUsage) → void` | `setUsage` |

---

### `TensorExtents`

C++ equivalent: `NS::TensorExtents`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `extent_at_dimension_index` | `(&self, dimension_index: UIn...) → Integer` | `extentAtDimensionIndex` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `rank` | `(&self) → UInteger` | `rank` |
| `with_values` | `(values: &[Integer]) → Option<Self>` | — |

---

### `TensorReferenceType`

C++ equivalent: `NS::TensorReferenceType`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `access` | `(&self) → BindingAccess` | `access` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `data_type` | `(&self) → DataType` | — |
| `dimensions_ptr` | `(&self) → *const c_void` | `dimensions` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index_type` | `(&self) → DataType` | `indexType` |
| `tensor_data_type` | `(&self) → TensorDataType` | `tensorDataType` |

---

### `Texture`

C++ equivalent: `NS::Texture`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new_remote_texture_view_for_device` | `(&self, device: &crate::Device) → Option<Texture>` | `newRemoteTextureViewForDevice` |
| `new_shared_texture_handle` | `(&self) → Option<SharedTextureHandle>` | `newSharedTextureHandle` |
| `new_texture_view` | `(&self,
        pixel_format...) → Option<Texture>` | `newTextureView` |
| `new_texture_view_with_descriptor` | `(&self,
        descriptor: ...) → Option<Texture>` | — |
| `new_texture_view_with_pixel_format` | `(&self, pixel_format: PixelF...) → Option<Texture>` | — |
| `new_texture_view_with_swizzle` | `(&self,
        pixel_format...) → Option<Texture>` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allocated_size` | `(&self) → UInteger` | — |
| `allow_gpu_optimized_contents` | `(&self) → bool` | `allowGPUOptimizedContents` |
| `array_length` | `(&self) → UInteger` | `arrayLength` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `buffer` | `(&self) → Option<crate::buffer::Buffer>` | `buffer` |
| `buffer_bytes_per_row` | `(&self) → UInteger` | `bufferBytesPerRow` |
| `buffer_offset` | `(&self) → UInteger` | `bufferOffset` |
| `compression_type` | `(&self) → TextureCompressionType` | `compressionType` |
| `depth` | `(&self) → UInteger` | `depth` |
| `device` | `(&self) → crate::Device` | — |
| `first_mipmap_in_tail` | `(&self) → UInteger` | `firstMipmapInTail` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `get_bytes` | `(&self,
        pixel_bytes:...) → void` | `getBytes` |
| `get_bytes_simple` | `(&self,
        pixel_bytes:...) → void` | — |
| `gpu_resource_id` | `(&self) → ResourceID` | `gpuResourceID` |
| `height` | `(&self) → UInteger` | `height` |
| `iosurface` | `(&self) → Option<*mut c_void>` | `iosurface` |
| `iosurface_plane` | `(&self) → UInteger` | `iosurfacePlane` |
| `label` | `(&self) → Option<String>` | — |
| `mipmap_level_count` | `(&self) → UInteger` | `mipmapLevelCount` |
| `parent_relative_level` | `(&self) → UInteger` | `parentRelativeLevel` |
| `parent_relative_slice` | `(&self) → UInteger` | `parentRelativeSlice` |
| `parent_texture` | `(&self) → Option<Texture>` | `parentTexture` |
| `pixel_format` | `(&self) → PixelFormat` | `pixelFormat` |
| `remote_storage_texture` | `(&self) → Option<Texture>` | `remoteStorageTexture` |
| `replace_region` | `(&self,
        region: crat...) → void` | `replaceRegion` |
| `replace_region_simple` | `(&self,
        region: crat...) → void` | — |
| `resource_options` | `(&self) → ResourceOptions` | — |
| `sample_count` | `(&self) → UInteger` | `sampleCount` |
| `sparse_texture_tier` | `(&self) → TextureSparseTier` | `sparseTextureTier` |
| `swizzle` | `(&self) → TextureSwizzleChannels` | `swizzle` |
| `tail_size_in_bytes` | `(&self) → UInteger` | `tailSizeInBytes` |
| `texture_type` | `(&self) → TextureType` | `textureType` |
| `usage` | `(&self) → TextureUsage` | `usage` |
| `width` | `(&self) → UInteger` | `width` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_framebuffer_only` | `(&self) → bool` | `framebufferOnly` |
| `is_shareable` | `(&self) → bool` | `isShareable` |
| `is_sparse` | `(&self) → bool` | `isSparse` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_label` | `(&self, label: &str) → void` | — |

---

### `TextureBinding`

C++ equivalent: `NS::TextureBinding`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `access` | `(&self) → BindingAccess` | — |
| `array_length` | `(&self) → UInteger` | `arrayLength` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `binding_type` | `(&self) → BindingType` | — |
| `depth_texture` | `(&self) → bool` | `depthTexture` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index` | `(&self) → UInteger` | — |
| `name` | `(&self) → Option<String>` | — |
| `texture_data_type` | `(&self) → DataType` | `textureDataType` |
| `texture_type` | `(&self) → TextureType` | `textureType` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_argument` | `(&self) → bool` | — |
| `is_depth_texture` | `(&self) → bool` | `isDepthTexture` |
| `is_used` | `(&self) → bool` | — |

---

### `TextureDescriptor`

C++ equivalent: `NS::TextureDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allow_gpu_optimized_contents` | `(&self) → bool` | `allowGPUOptimizedContents` |
| `array_length` | `(&self) → UInteger` | `arrayLength` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `compression_type` | `(&self) → TextureCompressionType` | `compressionType` |
| `cpu_cache_mode` | `(&self) → CPUCacheMode` | `cpuCacheMode` |
| `depth` | `(&self) → UInteger` | `depth` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `hazard_tracking_mode` | `(&self) → HazardTrackingMode` | `hazardTrackingMode` |
| `height` | `(&self) → UInteger` | `height` |
| `mipmap_level_count` | `(&self) → UInteger` | `mipmapLevelCount` |
| `pixel_format` | `(&self) → PixelFormat` | `pixelFormat` |
| `placement_sparse_page_size` | `(&self) → SparsePageSize` | `placementSparsePageSize` |
| `resource_options` | `(&self) → ResourceOptions` | `resourceOptions` |
| `sample_count` | `(&self) → UInteger` | `sampleCount` |
| `storage_mode` | `(&self) → StorageMode` | `storageMode` |
| `swizzle` | `(&self) → TextureSwizzleChannels` | `swizzle` |
| `texture_2d_descriptor` | `(pixel_format: PixelFormat,
...) → Option<Self>` | `texture2DDescriptor` |
| `texture_buffer_descriptor` | `(pixel_format: PixelFormat,
...) → Option<Self>` | `textureBufferDescriptor` |
| `texture_cube_descriptor` | `(pixel_format: PixelFormat,
...) → Option<Self>` | `textureCubeDescriptor` |
| `texture_type` | `(&self) → TextureType` | `textureType` |
| `usage` | `(&self) → TextureUsage` | `usage` |
| `width` | `(&self) → UInteger` | `width` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_allow_gpu_optimized_contents` | `(&self, allow: bool) → void` | `setAllowGPUOptimizedContents` |
| `set_array_length` | `(&self, length: UInteger) → void` | `setArrayLength` |
| `set_compression_type` | `(&self, compression_type: Te...) → void` | `setCompressionType` |
| `set_cpu_cache_mode` | `(&self, mode: CPUCacheMode) → void` | `setCpuCacheMode` |
| `set_depth` | `(&self, depth: UInteger) → void` | `setDepth` |
| `set_hazard_tracking_mode` | `(&self, mode: HazardTracking...) → void` | `setHazardTrackingMode` |
| `set_height` | `(&self, height: UInteger) → void` | `setHeight` |
| `set_mipmap_level_count` | `(&self, count: UInteger) → void` | `setMipmapLevelCount` |
| `set_pixel_format` | `(&self, pixel_format: PixelF...) → void` | `setPixelFormat` |
| `set_placement_sparse_page_size` | `(&self, size: SparsePageSize) → void` | `setPlacementSparsePageSize` |
| `set_resource_options` | `(&self, options: ResourceOpt...) → void` | `setResourceOptions` |
| `set_sample_count` | `(&self, count: UInteger) → void` | `setSampleCount` |
| `set_storage_mode` | `(&self, mode: StorageMode) → void` | `setStorageMode` |
| `set_swizzle` | `(&self, swizzle: TextureSwiz...) → void` | `setSwizzle` |
| `set_texture_type` | `(&self, texture_type: Textur...) → void` | `setTextureType` |
| `set_usage` | `(&self, usage: TextureUsage) → void` | `setUsage` |
| `set_width` | `(&self, width: UInteger) → void` | `setWidth` |

---

### `TextureReferenceType`

C++ equivalent: `NS::TextureReferenceType`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `access` | `(&self) → BindingAccess` | `access` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `data_type` | `(&self) → DataType` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `texture_data_type` | `(&self) → DataType` | `textureDataType` |
| `texture_type` | `(&self) → TextureType` | `textureType` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_depth_texture` | `(&self) → bool` | `isDepthTexture` |

---

### `TextureViewDescriptor`

C++ equivalent: `NS::TextureViewDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `level_range` | `(&self) → metal_foundation::Range` | `levelRange` |
| `pixel_format` | `(&self) → PixelFormat` | `pixelFormat` |
| `slice_range` | `(&self) → metal_foundation::Range` | `sliceRange` |
| `swizzle` | `(&self) → TextureSwizzleChannels` | `swizzle` |
| `texture_type` | `(&self) → TextureType` | `textureType` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_level_range` | `(&self, range: metal_foundat...) → void` | `setLevelRange` |
| `set_pixel_format` | `(&self, pixel_format: PixelF...) → void` | `setPixelFormat` |
| `set_slice_range` | `(&self, range: metal_foundat...) → void` | `setSliceRange` |
| `set_swizzle` | `(&self, swizzle: TextureSwiz...) → void` | `setSwizzle` |
| `set_texture_type` | `(&self, texture_type: Textur...) → void` | `setTextureType` |

---

### `TextureViewPool`

C++ equivalent: `NS::TextureViewPool`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `base_resource_id` | `(&self) → ResourceID` | — |
| `copy_resource_views_from_pool` | `(&self,
        source: &Tex...) → void` | — |
| `device` | `(&self) → crate::Device` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | — |
| `resource_view_count` | `(&self) → UInteger` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_texture_view` | `(&self, texture: &Texture, i...) → void` | `setTextureView` |
| `set_texture_view_from_buffer` | `(&self,
        buffer: &Buf...) → void` | `setTextureViewFromBuffer` |
| `set_texture_view_from_buffer_raw` | `(&self,
        buffer: *con...) → void` | — |
| `set_texture_view_with_descriptor` | `(&self,
        texture: &Te...) → void` | — |

---

### `ThreadgroupBinding`

C++ equivalent: `NS::ThreadgroupBinding`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `access` | `(&self) → BindingAccess` | — |
| `as_raw` | `(&self) → *mut c_void` | — |
| `binding_type` | `(&self) → BindingType` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `index` | `(&self) → UInteger` | — |
| `name` | `(&self) → Option<String>` | — |
| `threadgroup_memory_alignment` | `(&self) → UInteger` | `threadgroupMemoryAlignment` |
| `threadgroup_memory_data_size` | `(&self) → UInteger` | `threadgroupMemoryDataSize` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_argument` | `(&self) → bool` | — |
| `is_used` | `(&self) → bool` | — |

---

### `TileRenderPipelineColorAttachmentDescriptor`

C++ equivalent: `NS::TileRenderPipelineColorAttachmentDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `pixel_format` | `(&self) → PixelFormat` | `pixelFormat` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_pixel_format` | `(&self, format: PixelFormat) → void` | `setPixelFormat` |

---

### `TileRenderPipelineColorAttachmentDescriptorArray`

C++ equivalent: `NS::TileRenderPipelineColorAttachmentDescriptorArray`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `object` | `(&self, index: UInteger) → Option<TileRenderPipelineColorAttachmentDescriptor>` | `object` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_object` | `(&self,
        descriptor: ...) → void` | `setObject` |

---

### `TileRenderPipelineDescriptor`

C++ equivalent: `NS::TileRenderPipelineDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `binary_archives_raw` | `(&self) → *mut c_void` | — |
| `color_attachments` | `(&self) → Option<TileRenderPipelineColorAttachmentDescriptorArray>` | `colorAttachments` |
| `color_attachments_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | — |
| `linked_functions` | `(&self) → Option<crate::LinkedFunctions>` | — |
| `max_call_stack_depth` | `(&self) → UInteger` | — |
| `max_total_threads_per_threadgroup` | `(&self) → UInteger` | `maxTotalThreadsPerThreadgroup` |
| `options` | `(&self) → Option<PipelineOptions>` | — |
| `preloaded_libraries_raw` | `(&self) → *mut c_void` | — |
| `raster_sample_count` | `(&self) → UInteger` | `rasterSampleCount` |
| `required_threads_per_threadgroup` | `(&self) → Size` | `requiredThreadsPerThreadgroup` |
| `reset` | `(&self) → void` | `reset` |
| `shader_validation` | `(&self) → ShaderValidation` | — |
| `static_linking_descriptor` | `(&self) → Option<StaticLinkingDescriptor>` | `staticLinkingDescriptor` |
| `support_adding_binary_functions` | `(&self) → bool` | — |
| `support_binary_linking` | `(&self) → bool` | `supportBinaryLinking` |
| `threadgroup_size_matches_tile_size` | `(&self) → bool` | `threadgroupSizeMatchesTileSize` |
| `tile_buffers` | `(&self) → Option<PipelineBufferDescriptorArray>` | — |
| `tile_function` | `(&self) → Option<crate::Function>` | — |
| `tile_function_descriptor` | `(&self) → Option<FunctionDescriptor>` | `tileFunctionDescriptor` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_binary_archives_raw` | `(&self, archives: *const c_void) → void` | — |
| `set_label` | `(&self, label: &str) → void` | — |
| `set_linked_functions` | `(&self, functions: Option<&c...) → void` | — |
| `set_max_call_stack_depth` | `(&self, depth: UInteger) → void` | — |
| `set_max_total_threads_per_threadgroup` | `(&self, max: UInteger) → void` | `setMaxTotalThreadsPerThreadgroup` |
| `set_options` | `(&self, options: &PipelineOp...) → void` | — |
| `set_preloaded_libraries_raw` | `(&self, libraries: *const c_...) → void` | — |
| `set_raster_sample_count` | `(&self, count: UInteger) → void` | `setRasterSampleCount` |
| `set_required_threads_per_threadgroup` | `(&self, size: Size) → void` | `setRequiredThreadsPerThreadgroup` |
| `set_shader_validation` | `(&self, validation: ShaderVa...) → void` | — |
| `set_static_linking_descriptor` | `(&self, descriptor: &StaticL...) → void` | `setStaticLinkingDescriptor` |
| `set_support_adding_binary_functions` | `(&self, support: bool) → void` | — |
| `set_support_binary_linking` | `(&self, support: bool) → void` | `setSupportBinaryLinking` |
| `set_threadgroup_size_matches_tile_size` | `(&self, value: bool) → void` | `setThreadgroupSizeMatchesTileSize` |
| `set_tile_function` | `(&self, function: Option<&cr...) → void` | — |
| `set_tile_function_descriptor` | `(&self, descriptor: &Functio...) → void` | `setTileFunctionDescriptor` |

---

### `TileRenderPipelineDescriptor`

C++ equivalent: `NS::TileRenderPipelineDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `binary_archives_raw` | `(&self) → *mut c_void` | `binaryArchives` |
| `color_attachments` | `(&self) → Option<TileRenderPipelineColorAttachmentDescriptorArray>` | `colorAttachments` |
| `color_attachments_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `label` | `(&self) → Option<String>` | `label` |
| `linked_functions` | `(&self) → Option<crate::LinkedFunctions>` | `linkedFunctions` |
| `max_call_stack_depth` | `(&self) → UInteger` | `maxCallStackDepth` |
| `max_total_threads_per_threadgroup` | `(&self) → UInteger` | `maxTotalThreadsPerThreadgroup` |
| `options` | `(&self) → Option<PipelineOptions>` | — |
| `preloaded_libraries_raw` | `(&self) → *mut c_void` | `preloadedLibraries` |
| `raster_sample_count` | `(&self) → UInteger` | `rasterSampleCount` |
| `required_threads_per_threadgroup` | `(&self) → Size` | `requiredThreadsPerThreadgroup` |
| `reset` | `(&self) → void` | `reset` |
| `shader_validation` | `(&self) → ShaderValidation` | `shaderValidation` |
| `static_linking_descriptor` | `(&self) → Option<StaticLinkingDescriptor>` | — |
| `support_adding_binary_functions` | `(&self) → bool` | `supportAddingBinaryFunctions` |
| `support_binary_linking` | `(&self) → bool` | — |
| `threadgroup_size_matches_tile_size` | `(&self) → bool` | `threadgroupSizeMatchesTileSize` |
| `tile_buffers` | `(&self) → Option<PipelineBufferDescriptorArray>` | `tileBuffers` |
| `tile_function` | `(&self) → Option<crate::Function>` | `tileFunction` |
| `tile_function_descriptor` | `(&self) → Option<FunctionDescriptor>` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_binary_archives_raw` | `(&self, archives: *const c_void) → void` | `setBinaryArchives` |
| `set_label` | `(&self, label: &str) → void` | `setLabel` |
| `set_linked_functions` | `(&self, functions: Option<&c...) → void` | `setLinkedFunctions` |
| `set_max_call_stack_depth` | `(&self, depth: UInteger) → void` | `setMaxCallStackDepth` |
| `set_max_total_threads_per_threadgroup` | `(&self, max: UInteger) → void` | `setMaxTotalThreadsPerThreadgroup` |
| `set_options` | `(&self, options: &PipelineOp...) → void` | — |
| `set_preloaded_libraries_raw` | `(&self, libraries: *const c_...) → void` | `setPreloadedLibraries` |
| `set_raster_sample_count` | `(&self, count: UInteger) → void` | `setRasterSampleCount` |
| `set_required_threads_per_threadgroup` | `(&self, size: Size) → void` | `setRequiredThreadsPerThreadgroup` |
| `set_shader_validation` | `(&self, validation: ShaderVa...) → void` | `setShaderValidation` |
| `set_static_linking_descriptor` | `(&self, descriptor: &StaticL...) → void` | — |
| `set_support_adding_binary_functions` | `(&self, support: bool) → void` | `setSupportAddingBinaryFunctions` |
| `set_support_binary_linking` | `(&self, support: bool) → void` | — |
| `set_threadgroup_size_matches_tile_size` | `(&self, value: bool) → void` | `setThreadgroupSizeMatchesTileSize` |
| `set_tile_function` | `(&self, function: Option<&cr...) → void` | `setTileFunction` |
| `set_tile_function_descriptor` | `(&self, descriptor: &Functio...) → void` | — |

---

### `Type`

C++ equivalent: `NS::Type`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `data_type` | `(&self) → DataType` | `dataType` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |

---

### `VertexAttribute`

C++ equivalent: `NS::VertexAttribute`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `attribute_index` | `(&self) → UInteger` | `attributeIndex` |
| `attribute_type` | `(&self) → DataType` | `attributeType` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `name` | `(&self) → Option<String>` | `name` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_active` | `(&self) → bool` | `active` |
| `is_patch_control_point_data` | `(&self) → bool` | `isPatchControlPointData` |
| `is_patch_data` | `(&self) → bool` | `isPatchData` |

---

### `VertexAttributeDescriptor`

C++ equivalent: `NS::VertexAttributeDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `buffer_index` | `(&self) → UInteger` | `bufferIndex` |
| `format` | `(&self) → VertexFormat` | `format` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `offset` | `(&self) → UInteger` | `offset` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_buffer_index` | `(&self, buffer_index: UInteger) → void` | `setBufferIndex` |
| `set_format` | `(&self, format: VertexFormat) → void` | `setFormat` |
| `set_offset` | `(&self, offset: UInteger) → void` | `setOffset` |

---

### `VertexAttributeDescriptorArray`

C++ equivalent: `NS::VertexAttributeDescriptorArray`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `object` | `(&self, index: UInteger) → Option<VertexAttributeDescriptor>` | `object` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_object` | `(&self, attribute_desc: &Ver...) → void` | `setObject` |

---

### `VertexBufferLayoutDescriptor`

C++ equivalent: `NS::VertexBufferLayoutDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `step_function` | `(&self) → VertexStepFunction` | `stepFunction` |
| `step_rate` | `(&self) → UInteger` | `stepRate` |
| `stride` | `(&self) → UInteger` | `stride` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_step_function` | `(&self, step_function: Verte...) → void` | `setStepFunction` |
| `set_step_rate` | `(&self, step_rate: UInteger) → void` | `setStepRate` |
| `set_stride` | `(&self, stride: UInteger) → void` | `setStride` |

---

### `VertexBufferLayoutDescriptorArray`

C++ equivalent: `NS::VertexBufferLayoutDescriptorArray`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `object` | `(&self, index: UInteger) → Option<VertexBufferLayoutDescriptor>` | `object` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_object` | `(&self, buffer_desc: &Vertex...) → void` | `setObject` |

---

### `VertexDescriptor`

C++ equivalent: `NS::VertexDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `attributes` | `(&self) → VertexAttributeDescriptorArray` | `attributes` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `layouts` | `(&self) → VertexBufferLayoutDescriptorArray` | `layouts` |
| `reset` | `(&self) → void` | `reset` |
| `vertex_descriptor` | `() → Option<Self>` | — |

---

### `VisibleFunctionTable`

C++ equivalent: `NS::VisibleFunctionTable`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `gpu_resource_id` | `(&self) → ResourceID` | `gpuResourceID` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_function` | `(&self, function: &FunctionH...) → void` | `setFunction` |
| `set_functions` | `(&self, functions: &[&Functi...) → void` | `setFunctions` |

---

### `VisibleFunctionTableDescriptor`

C++ equivalent: `NS::VisibleFunctionTableDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `function_count` | `(&self) → UInteger` | `functionCount` |
| `visible_function_table_descriptor` | `() → Option<Self>` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_function_count` | `(&self, function_count: UInt...) → void` | `setFunctionCount` |

---

## metal-foundation

### `Array`

C++ equivalent: `MTL::Array`

---

### `AutoreleasePool`

C++ equivalent: `MTL::AutoreleasePool`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `add_object` | `(&self, object: &Object) → void` | `addObject` |
| `drain` | `(&self) → void` | `drain` |
| `from_ptr` | `(ptr: *mut c_void) → Option<Self>` | — |
| `show_pools` | `() → void` | `showPools` |

---

### `Bundle`

C++ equivalent: `MTL::Bundle`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `all_bundles` | `() → *mut Array<Bundle>` | — |
| `all_frameworks` | `() → *mut Array<Bundle>` | — |
| `app_store_receipt_url` | `(&self) → *mut Url` | — |
| `built_in_plug_ins_path` | `(&self) → *mut String` | — |
| `built_in_plug_ins_url` | `(&self) → *mut Url` | — |
| `bundle_identifier` | `(&self) → *mut String` | — |
| `bundle_path` | `(&self) → *mut String` | — |
| `bundle_url` | `(&self) → *mut Url` | — |
| `bundle_with_path` | `(path: &String) → Option<Self>` | — |
| `bundle_with_url` | `(url: &Url) → Option<Self>` | — |
| `executable_path` | `(&self) → *mut String` | — |
| `executable_url` | `(&self) → *mut Url` | — |
| `from_ptr` | `(ptr: *mut c_void) → Option<Self>` | — |
| `info_dictionary` | `(&self) → *mut Dictionary` | — |
| `init_with_path` | `(&self, path: &String) → Option<Self>` | — |
| `init_with_url` | `(&self, url: &Url) → Option<Self>` | — |
| `load` | `(&self) → bool` | — |
| `load_and_return_error` | `(&self, error: *mut *mut Error) → bool` | — |
| `localized_info_dictionary` | `(&self) → *mut Dictionary` | — |
| `localized_string` | `(&self,
        key: &String...) → *mut String` | — |
| `main_bundle` | `() → Option<Self>` | — |
| `object_for_info_dictionary_key` | `(&self, key: &String) → *mut Object` | — |
| `path_for_auxiliary_executable` | `(&self, name: &String) → *mut String` | — |
| `preflight_and_return_error` | `(&self, error: *mut *mut Error) → bool` | — |
| `private_frameworks_path` | `(&self) → *mut String` | — |
| `private_frameworks_url` | `(&self) → *mut Url` | — |
| `resource_path` | `(&self) → *mut String` | — |
| `resource_url` | `(&self) → *mut Url` | — |
| `shared_frameworks_path` | `(&self) → *mut String` | — |
| `shared_frameworks_url` | `(&self) → *mut Url` | — |
| `shared_support_path` | `(&self) → *mut String` | — |
| `shared_support_url` | `(&self) → *mut Url` | — |
| `unload` | `(&self) → bool` | — |
| `url_for_auxiliary_executable` | `(&self, name: &String) → *mut Url` | — |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_loaded` | `(&self) → bool` | — |

---

### `Condition`

C++ equivalent: `MTL::Condition`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |
| `init` | `(&self) → Option<Self>` | — |
| `new` | `() → Option<Self>` | `alloc` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `broadcast` | `(&self) → void` | `broadcast` |
| `from_ptr` | `(ptr: *mut c_void) → Option<Self>` | — |
| `signal` | `(&self) → void` | `signal` |
| `wait` | `(&self) → void` | `wait` |
| `wait_until_date` | `(&self, limit: &Date) → bool` | `waitUntilDate` |

---

### `Data`

C++ equivalent: `MTL::Data`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `from_ptr` | `(ptr: *mut c_void) → Option<Self>` | — |
| `length` | `(&self) → UInteger` | `length` |
| `mutable_bytes` | `(&self) → *mut c_void` | `mutableBytes` |

---

### `Date`

C++ equivalent: `MTL::Date`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `date_with_time_interval_since_now` | `(secs: TimeInterval) → Option<Self>` | `dateWithTimeIntervalSinceNow` |
| `from_ptr` | `(ptr: *mut c_void) → Option<Self>` | — |

---

### `Dictionary`

C++ equivalent: `NS::Dictionary`

---

### `Enumerator`

C++ equivalent: `MTL::Enumerator`

---

### `Error`

C++ equivalent: `MTL::Error`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | `alloc` |
| `init` | `(&self) → Option<Self>` | `init` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `code` | `(&self) → Integer` | `code` |
| `domain` | `(&self) → ErrorDomain` | `domain` |
| `error` | `(domain: ErrorDomain,
      ...) → Option<Self>` | `error` |
| `from_ptr` | `(ptr: *mut c_void) → Option<Self>` | — |
| `init_with_domain` | `(&self,
        domain: Erro...) → Option<Self>` | — |
| `localized_description` | `(&self) → *mut String` | `localizedDescription` |
| `localized_failure_reason` | `(&self) → *mut String` | `localizedFailureReason` |
| `localized_recovery_options` | `(&self) → *mut Array<String>` | `localizedRecoveryOptions` |
| `localized_recovery_suggestion` | `(&self) → *mut String` | `localizedRecoverySuggestion` |
| `user_info` | `(&self) → *mut Dictionary` | `userInfo` |

---

### `Notification`

C++ equivalent: `NS::Notification`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `from_ptr` | `(ptr: *mut c_void) → Option<Self>` | — |
| `name` | `(&self) → *mut String` | `name` |
| `object` | `(&self) → *mut Object` | `object` |
| `user_info` | `(&self) → *mut Dictionary` | `userInfo` |

---

### `NotificationCenter`

C++ equivalent: `NS::NotificationCenter`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `add_observer` | `(&self,
        name: Notifi...) → *mut Object` | `addObserver` |
| `default_center` | `() → Option<Self>` | `defaultCenter` |
| `from_ptr` | `(ptr: *mut c_void) → Option<Self>` | — |
| `remove_observer` | `(&self, observer: &Object) → void` | `removeObserver` |

---

### `Number`

C++ equivalent: `MTL::Number`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `bool_value` | `(&self) → bool` | — |
| `char_value` | `(&self) → i8` | — |
| `compare` | `(&self, other: &Number) → ComparisonResult` | — |
| `description_with_locale` | `(&self, locale: *const c_void) → *mut String` | — |
| `double_value` | `(&self) → f64` | — |
| `float_value` | `(&self) → f32` | — |
| `from_ptr` | `(ptr: *mut c_void) → Option<Self>` | — |
| `init_with_bool` | `(&self, value: bool) → Option<Self>` | — |
| `init_with_char` | `(&self, value: i8) → Option<Self>` | — |
| `init_with_coder` | `(&self, coder: *const c_void) → Option<Self>` | — |
| `init_with_double` | `(&self, value: f64) → Option<Self>` | — |
| `init_with_float` | `(&self, value: f32) → Option<Self>` | — |
| `init_with_int` | `(&self, value: i32) → Option<Self>` | — |
| `init_with_long` | `(&self, value: std::ffi::c_long) → Option<Self>` | — |
| `init_with_long_long` | `(&self, value: i64) → Option<Self>` | — |
| `init_with_short` | `(&self, value: i16) → Option<Self>` | — |
| `init_with_unsigned_char` | `(&self, value: u8) → Option<Self>` | — |
| `init_with_unsigned_int` | `(&self, value: u32) → Option<Self>` | — |
| `init_with_unsigned_long` | `(&self, value: std::ffi::c_u...) → Option<Self>` | — |
| `init_with_unsigned_long_long` | `(&self, value: u64) → Option<Self>` | — |
| `init_with_unsigned_short` | `(&self, value: u16) → Option<Self>` | — |
| `int_value` | `(&self) → i32` | — |
| `integer_value` | `(&self) → Integer` | — |
| `long_long_value` | `(&self) → i64` | — |
| `long_value` | `(&self) → std::ffi::c_long` | — |
| `number_with_bool` | `(value: bool) → Option<Self>` | — |
| `number_with_char` | `(value: i8) → Option<Self>` | — |
| `number_with_double` | `(value: f64) → Option<Self>` | — |
| `number_with_float` | `(value: f32) → Option<Self>` | — |
| `number_with_int` | `(value: i32) → Option<Self>` | — |
| `number_with_long` | `(value: std::ffi::c_long) → Option<Self>` | — |
| `number_with_long_long` | `(value: i64) → Option<Self>` | — |
| `number_with_short` | `(value: i16) → Option<Self>` | — |
| `number_with_unsigned_char` | `(value: u8) → Option<Self>` | — |
| `number_with_unsigned_int` | `(value: u32) → Option<Self>` | — |
| `number_with_unsigned_long` | `(value: std::ffi::c_ulong) → Option<Self>` | — |
| `number_with_unsigned_long_long` | `(value: u64) → Option<Self>` | — |
| `number_with_unsigned_short` | `(value: u16) → Option<Self>` | — |
| `short_value` | `(&self) → i16` | — |
| `string_value` | `(&self) → *mut String` | — |
| `unsigned_char_value` | `(&self) → u8` | — |
| `unsigned_int_value` | `(&self) → u32` | — |
| `unsigned_integer_value` | `(&self) → UInteger` | — |
| `unsigned_long_long_value` | `(&self) → u64` | — |
| `unsigned_long_value` | `(&self) → std::ffi::c_ulong` | — |
| `unsigned_short_value` | `(&self) → u16` | — |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_equal_to_number` | `(&self, number: &Number) → bool` | — |

---

### `Object`

C++ equivalent: `MTL::Object`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `init` | `(&self) → Option<Self>` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc_with_class` | `(cls: Class) → Option<Self>` | — |
| `debug_description` | `(&self) → *mut c_void` | — |
| `description` | `(&self) → *mut c_void` | — |
| `from_ptr` | `(ptr: *mut c_void) → Option<Self>` | — |
| `hash` | `(&self) → UInteger` | — |
| `responds_to_selector` | `(&self, selector: Sel) → bool` | — |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_equal` | `(&self, other: &Object) → bool` | — |

---

### `ProcessInfo`

C++ equivalent: `MTL::ProcessInfo`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `active_processor_count` | `(&self) → UInteger` | `activeProcessorCount` |
| `arguments` | `(&self) → *mut Array<String>` | `arguments` |
| `automatic_termination_support_enabled` | `(&self) → bool` | `automaticTerminationSupportEnabled` |
| `begin_activity` | `(&self, options: ActivityOpt...) → *mut Object` | `beginActivity` |
| `disable_automatic_termination` | `(&self, reason: &String) → void` | `disableAutomaticTermination` |
| `disable_sudden_termination` | `(&self) → void` | `disableSuddenTermination` |
| `enable_automatic_termination` | `(&self, reason: &String) → void` | `enableAutomaticTermination` |
| `enable_sudden_termination` | `(&self) → void` | `enableSuddenTermination` |
| `end_activity` | `(&self, activity: &Object) → void` | `endActivity` |
| `environment` | `(&self) → *mut Dictionary` | `environment` |
| `from_ptr` | `(ptr: *mut c_void) → Option<Self>` | — |
| `full_user_name` | `(&self) → *mut String` | `fullUserName` |
| `globally_unique_string` | `(&self) → *mut String` | `globallyUniqueString` |
| `host_name` | `(&self) → *mut String` | `hostName` |
| `operating_system` | `(&self) → UInteger` | `operatingSystem` |
| `operating_system_version` | `(&self) → OperatingSystemVersion` | `operatingSystemVersion` |
| `operating_system_version_string` | `(&self) → *mut String` | `operatingSystemVersionString` |
| `physical_memory` | `(&self) → u64` | `physicalMemory` |
| `process_identifier` | `(&self) → i32` | `processIdentifier` |
| `process_info` | `() → Option<Self>` | `processInfo` |
| `process_name` | `(&self) → *mut String` | `processName` |
| `processor_count` | `(&self) → UInteger` | `processorCount` |
| `system_uptime` | `(&self) → TimeInterval` | `systemUptime` |
| `thermal_state` | `(&self) → ProcessInfoThermalState` | `thermalState` |
| `user_name` | `(&self) → *mut String` | `userName` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `has_performance_profile` | `(&self, performance_profile:...) → bool` | `hasPerformanceProfile` |
| `is_device_certified` | `(&self, performance_tier: De...) → bool` | `isDeviceCertified` |
| `is_ios_app_on_mac` | `(&self) → bool` | `isiOSAppOnMac` |
| `is_low_power_mode_enabled` | `(&self) → bool` | `isLowPowerModeEnabled` |
| `is_mac_catalyst_app` | `(&self) → bool` | `isMacCatalystApp` |
| `is_operating_system_at_least_version` | `(&self, version: OperatingSy...) → bool` | `isOperatingSystemAtLeastVersion` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_automatic_termination_support_enabled` | `(&self, enabled: bool) → void` | `setAutomaticTerminationSupportEnabled` |
| `set_process_name` | `(&self, name: &String) → void` | `setProcessName` |

---

### `Set`

C++ equivalent: `NS::Set`

---

### `String`

C++ equivalent: `MTL::String`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | `alloc` |
| `init` | `(&self) → Option<Self>` | `init` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `c_string` | `(&self, encoding: StringEnco...) → *const c_char` | `cString` |
| `case_insensitive_compare` | `(&self, string: &String) → ComparisonResult` | `caseInsensitiveCompare` |
| `character` | `(&self, index: UInteger) → Unichar` | `character` |
| `file_system_representation` | `(&self) → *const c_char` | `fileSystemRepresentation` |
| `from_ptr` | `(ptr: *mut c_void) → Option<Self>` | — |
| `from_str` | `(s: &str) → Option<Self>` | — |
| `init_with_bytes_no_copy` | `(&self,
        bytes: *mut ...) → Option<Self>` | — |
| `init_with_cstring` | `(&self, cstring: *const c_ch...) → Option<Self>` | — |
| `init_with_string` | `(&self, string: &String) → Option<Self>` | — |
| `length` | `(&self) → UInteger` | `length` |
| `length_of_bytes` | `(&self, encoding: StringEnco...) → UInteger` | `lengthOfBytes` |
| `maximum_length_of_bytes` | `(&self, encoding: StringEnco...) → UInteger` | `maximumLengthOfBytes` |
| `range_of_string` | `(&self, string: &String, opt...) → Range` | `rangeOfString` |
| `string` | `() → Option<Self>` | `string` |
| `string_by_appending_string` | `(&self, string: &String) → Option<Self>` | `stringByAppendingString` |
| `string_with_cstring` | `(cstring: *const c_char, enc...) → Option<Self>` | — |
| `string_with_string` | `(string: &String) → Option<Self>` | — |
| `to_string` | `(&self) → Option<std::string::String>` | — |
| `utf8_string` | `(&self) → *const c_char` | `utf8String` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_equal_to_string` | `(&self, string: &String) → bool` | `isEqualToString` |

---

### `Value`

C++ equivalent: `MTL::Value`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `alloc` | `() → Option<Self>` | — |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `from_ptr` | `(ptr: *mut c_void) → Option<Self>` | — |
| `get_value` | `(&self, value: *mut c_void, ...) → void` | — |
| `init_with_bytes` | `(&self, value: *const c_void...) → Option<Self>` | — |
| `init_with_coder` | `(&self, coder: *const c_void) → Option<Self>` | — |
| `objc_type` | `(&self) → *const c_char` | — |
| `pointer_value` | `(&self) → *mut c_void` | — |
| `value` | `(value: *const c_void, obj_c...) → Option<Self>` | — |
| `value_with_pointer` | `(pointer: *const c_void) → Option<Self>` | — |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_equal_to_value` | `(&self, value: &Value) → bool` | — |

---

## metal-fx

### `FrameInterpolator`

C++ equivalent: `NS::FrameInterpolator`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `aspect_ratio` | `(&self) → f32` | — |
| `color_texture` | `(&self) → Option<metal::Texture>` | — |
| `color_texture_format` | `(&self) → metal::PixelFormat` | — |
| `color_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `delta_time` | `(&self) → f32` | — |
| `depth_texture` | `(&self) → Option<metal::Texture>` | — |
| `depth_texture_format` | `(&self) → metal::PixelFormat` | — |
| `depth_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `encode_to_command_buffer` | `(&self, command_buffer: &met...) → void` | `encodeToCommandBuffer` |
| `encode_to_mtl4_command_buffer` | `(&self, command_buffer: &met...) → void` | — |
| `far_plane` | `(&self) → f32` | — |
| `fence` | `(&self) → Option<metal::Fence>` | — |
| `field_of_view` | `(&self) → f32` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `input_height` | `(&self) → UInteger` | — |
| `input_width` | `(&self) → UInteger` | — |
| `jitter_offset_x` | `(&self) → f32` | — |
| `jitter_offset_y` | `(&self) → f32` | — |
| `motion_texture` | `(&self) → Option<metal::Texture>` | — |
| `motion_texture_format` | `(&self) → metal::PixelFormat` | — |
| `motion_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `motion_vector_scale_x` | `(&self) → f32` | — |
| `motion_vector_scale_y` | `(&self) → f32` | — |
| `near_plane` | `(&self) → f32` | — |
| `output_height` | `(&self) → UInteger` | — |
| `output_texture` | `(&self) → Option<metal::Texture>` | — |
| `output_texture_format` | `(&self) → metal::PixelFormat` | — |
| `output_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `output_width` | `(&self) → UInteger` | — |
| `prev_color_texture` | `(&self) → Option<metal::Texture>` | — |
| `should_reset_history` | `(&self) → bool` | — |
| `ui_texture` | `(&self) → Option<metal::Texture>` | — |
| `ui_texture_format` | `(&self) → metal::PixelFormat` | — |
| `ui_texture_usage` | `(&self) → metal::TextureUsage` | — |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_depth_reversed` | `(&self) → bool` | — |
| `is_ui_texture_composited` | `(&self) → bool` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_aspect_ratio` | `(&self, aspect_ratio: f32) → void` | — |
| `set_color_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_delta_time` | `(&self, delta_time: f32) → void` | — |
| `set_depth_reversed` | `(&self, reversed: bool) → void` | — |
| `set_depth_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_far_plane` | `(&self, far_plane: f32) → void` | — |
| `set_fence` | `(&self, fence: &metal::Fence) → void` | — |
| `set_field_of_view` | `(&self, fov: f32) → void` | — |
| `set_is_ui_texture_composited` | `(&self, composited: bool) → void` | — |
| `set_jitter_offset_x` | `(&self, offset: f32) → void` | — |
| `set_jitter_offset_y` | `(&self, offset: f32) → void` | — |
| `set_motion_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_motion_vector_scale_x` | `(&self, scale: f32) → void` | — |
| `set_motion_vector_scale_y` | `(&self, scale: f32) → void` | — |
| `set_near_plane` | `(&self, near_plane: f32) → void` | — |
| `set_output_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_prev_color_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_should_reset_history` | `(&self, reset: bool) → void` | — |
| `set_ui_texture` | `(&self, texture: &metal::Tex...) → void` | — |

---

### `FrameInterpolator`

C++ equivalent: `NS::FrameInterpolator`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `aspect_ratio` | `(&self) → f32` | — |
| `color_texture` | `(&self) → Option<metal::Texture>` | — |
| `color_texture_format` | `(&self) → metal::PixelFormat` | — |
| `color_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `delta_time` | `(&self) → f32` | — |
| `depth_texture` | `(&self) → Option<metal::Texture>` | — |
| `depth_texture_format` | `(&self) → metal::PixelFormat` | — |
| `depth_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `encode_to_command_buffer` | `(&self, command_buffer: &met...) → void` | `encodeToCommandBuffer` |
| `encode_to_mtl4_command_buffer` | `(&self, command_buffer: &met...) → void` | — |
| `far_plane` | `(&self) → f32` | — |
| `fence` | `(&self) → Option<metal::Fence>` | — |
| `field_of_view` | `(&self) → f32` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `input_height` | `(&self) → UInteger` | — |
| `input_width` | `(&self) → UInteger` | — |
| `jitter_offset_x` | `(&self) → f32` | — |
| `jitter_offset_y` | `(&self) → f32` | — |
| `motion_texture` | `(&self) → Option<metal::Texture>` | — |
| `motion_texture_format` | `(&self) → metal::PixelFormat` | — |
| `motion_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `motion_vector_scale_x` | `(&self) → f32` | — |
| `motion_vector_scale_y` | `(&self) → f32` | — |
| `near_plane` | `(&self) → f32` | — |
| `output_height` | `(&self) → UInteger` | — |
| `output_texture` | `(&self) → Option<metal::Texture>` | — |
| `output_texture_format` | `(&self) → metal::PixelFormat` | — |
| `output_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `output_width` | `(&self) → UInteger` | — |
| `prev_color_texture` | `(&self) → Option<metal::Texture>` | — |
| `should_reset_history` | `(&self) → bool` | — |
| `ui_texture` | `(&self) → Option<metal::Texture>` | — |
| `ui_texture_format` | `(&self) → metal::PixelFormat` | — |
| `ui_texture_usage` | `(&self) → metal::TextureUsage` | — |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_depth_reversed` | `(&self) → bool` | — |
| `is_ui_texture_composited` | `(&self) → bool` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_aspect_ratio` | `(&self, aspect_ratio: f32) → void` | — |
| `set_color_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_delta_time` | `(&self, delta_time: f32) → void` | — |
| `set_depth_reversed` | `(&self, reversed: bool) → void` | — |
| `set_depth_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_far_plane` | `(&self, far_plane: f32) → void` | — |
| `set_fence` | `(&self, fence: &metal::Fence) → void` | — |
| `set_field_of_view` | `(&self, fov: f32) → void` | — |
| `set_is_ui_texture_composited` | `(&self, composited: bool) → void` | — |
| `set_jitter_offset_x` | `(&self, offset: f32) → void` | — |
| `set_jitter_offset_y` | `(&self, offset: f32) → void` | — |
| `set_motion_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_motion_vector_scale_x` | `(&self, scale: f32) → void` | — |
| `set_motion_vector_scale_y` | `(&self, scale: f32) → void` | — |
| `set_near_plane` | `(&self, near_plane: f32) → void` | — |
| `set_output_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_prev_color_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_should_reset_history` | `(&self, reset: bool) → void` | — |
| `set_ui_texture` | `(&self, texture: &metal::Tex...) → void` | — |

---

### `FrameInterpolatorDescriptor`

C++ equivalent: `NS::FrameInterpolatorDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |
| `new_frame_interpolator` | `(&self, device: &metal::Device) → Option<FrameInterpolator>` | `newFrameInterpolator` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `color_texture_format` | `(&self) → metal::PixelFormat` | `colorTextureFormat` |
| `depth_texture_format` | `(&self) → metal::PixelFormat` | `depthTextureFormat` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `input_height` | `(&self) → UInteger` | `inputHeight` |
| `input_width` | `(&self) → UInteger` | `inputWidth` |
| `motion_texture_format` | `(&self) → metal::PixelFormat` | `motionTextureFormat` |
| `output_height` | `(&self) → UInteger` | `outputHeight` |
| `output_texture_format` | `(&self) → metal::PixelFormat` | `outputTextureFormat` |
| `output_width` | `(&self) → UInteger` | `outputWidth` |
| `scaler_raw` | `(&self) → *mut c_void` | `scaler` |
| `ui_texture_format` | `(&self) → metal::PixelFormat` | `uiTextureFormat` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `supports_device` | `(device: &metal::Device) → bool` | `supportsMetal4FX` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_color_texture_format` | `(&self, format: metal::Pixel...) → void` | `setColorTextureFormat` |
| `set_depth_texture_format` | `(&self, format: metal::Pixel...) → void` | `setDepthTextureFormat` |
| `set_input_height` | `(&self, height: UInteger) → void` | `setInputHeight` |
| `set_input_width` | `(&self, width: UInteger) → void` | `setInputWidth` |
| `set_motion_texture_format` | `(&self, format: metal::Pixel...) → void` | `setMotionTextureFormat` |
| `set_output_height` | `(&self, height: UInteger) → void` | `setOutputHeight` |
| `set_output_texture_format` | `(&self, format: metal::Pixel...) → void` | `setOutputTextureFormat` |
| `set_output_width` | `(&self, width: UInteger) → void` | `setOutputWidth` |
| `set_scaler_raw` | `(&self, scaler: *const c_void) → void` | `setScaler` |
| `set_ui_texture_format` | `(&self, format: metal::Pixel...) → void` | `setUITextureFormat` |

---

### `SpatialScaler`

C++ equivalent: `NS::SpatialScaler`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `color_processing_mode` | `(&self) → SpatialScalerColorProcessingMode` | — |
| `color_texture` | `(&self) → Option<metal::Texture>` | — |
| `color_texture_format` | `(&self) → metal::PixelFormat` | — |
| `color_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `encode_to_command_buffer` | `(&self, command_buffer: &met...) → void` | `encodeToCommandBuffer` |
| `encode_to_mtl4_command_buffer` | `(&self, command_buffer: &met...) → void` | — |
| `fence` | `(&self) → Option<metal::Fence>` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `input_content_height` | `(&self) → UInteger` | — |
| `input_content_width` | `(&self) → UInteger` | — |
| `input_height` | `(&self) → UInteger` | — |
| `input_width` | `(&self) → UInteger` | — |
| `output_height` | `(&self) → UInteger` | — |
| `output_texture` | `(&self) → Option<metal::Texture>` | — |
| `output_texture_format` | `(&self) → metal::PixelFormat` | — |
| `output_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `output_width` | `(&self) → UInteger` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_color_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_fence` | `(&self, fence: &metal::Fence) → void` | — |
| `set_input_content_height` | `(&self, height: UInteger) → void` | — |
| `set_input_content_width` | `(&self, width: UInteger) → void` | — |
| `set_output_texture` | `(&self, texture: &metal::Tex...) → void` | — |

---

### `SpatialScaler`

C++ equivalent: `NS::SpatialScaler`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `color_processing_mode` | `(&self) → SpatialScalerColorProcessingMode` | — |
| `color_texture` | `(&self) → Option<metal::Texture>` | — |
| `color_texture_format` | `(&self) → metal::PixelFormat` | — |
| `color_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `encode_to_command_buffer` | `(&self, command_buffer: &met...) → void` | `encodeToCommandBuffer` |
| `encode_to_mtl4_command_buffer` | `(&self, command_buffer: &met...) → void` | — |
| `fence` | `(&self) → Option<metal::Fence>` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `input_content_height` | `(&self) → UInteger` | — |
| `input_content_width` | `(&self) → UInteger` | — |
| `input_height` | `(&self) → UInteger` | — |
| `input_width` | `(&self) → UInteger` | — |
| `output_height` | `(&self) → UInteger` | — |
| `output_texture` | `(&self) → Option<metal::Texture>` | — |
| `output_texture_format` | `(&self) → metal::PixelFormat` | — |
| `output_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `output_width` | `(&self) → UInteger` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_color_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_fence` | `(&self, fence: &metal::Fence) → void` | — |
| `set_input_content_height` | `(&self, height: UInteger) → void` | — |
| `set_input_content_width` | `(&self, width: UInteger) → void` | — |
| `set_output_texture` | `(&self, texture: &metal::Tex...) → void` | — |

---

### `SpatialScalerDescriptor`

C++ equivalent: `NS::SpatialScalerDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |
| `new_spatial_scaler` | `(&self, device: &metal::Device) → Option<SpatialScaler>` | `newSpatialScaler` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `color_processing_mode` | `(&self) → SpatialScalerColorProcessingMode` | `colorProcessingMode` |
| `color_texture_format` | `(&self) → metal::PixelFormat` | `colorTextureFormat` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `input_height` | `(&self) → UInteger` | `inputHeight` |
| `input_width` | `(&self) → UInteger` | `inputWidth` |
| `output_height` | `(&self) → UInteger` | `outputHeight` |
| `output_texture_format` | `(&self) → metal::PixelFormat` | `outputTextureFormat` |
| `output_width` | `(&self) → UInteger` | `outputWidth` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `supports_device` | `(device: &metal::Device) → bool` | `supportsDevice` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_color_processing_mode` | `(&self, mode: SpatialScalerC...) → void` | `setColorProcessingMode` |
| `set_color_texture_format` | `(&self, format: metal::Pixel...) → void` | `setColorTextureFormat` |
| `set_input_height` | `(&self, height: UInteger) → void` | `setInputHeight` |
| `set_input_width` | `(&self, width: UInteger) → void` | `setInputWidth` |
| `set_output_height` | `(&self, height: UInteger) → void` | `setOutputHeight` |
| `set_output_texture_format` | `(&self, format: metal::Pixel...) → void` | `setOutputTextureFormat` |
| `set_output_width` | `(&self, width: UInteger) → void` | `setOutputWidth` |

---

### `TemporalDenoisedScaler`

C++ equivalent: `NS::TemporalDenoisedScaler`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `color_texture` | `(&self) → Option<Texture>` | — |
| `color_texture_format` | `(&self) → PixelFormat` | — |
| `color_texture_usage` | `(&self) → TextureUsage` | — |
| `denoise_strength_mask_texture` | `(&self) → Option<Texture>` | — |
| `denoise_strength_mask_texture_format` | `(&self) → PixelFormat` | — |
| `denoise_strength_mask_texture_usage` | `(&self) → TextureUsage` | — |
| `depth_texture` | `(&self) → Option<Texture>` | — |
| `depth_texture_format` | `(&self) → PixelFormat` | — |
| `depth_texture_usage` | `(&self) → TextureUsage` | — |
| `diffuse_albedo_texture` | `(&self) → Option<Texture>` | — |
| `diffuse_albedo_texture_format` | `(&self) → PixelFormat` | — |
| `diffuse_albedo_texture_usage` | `(&self) → TextureUsage` | — |
| `encode_to_command_buffer` | `(&self, command_buffer: &met...) → void` | `encodeToCommandBuffer` |
| `encode_to_mtl4_command_buffer` | `(&self, command_buffer: &met...) → void` | — |
| `exposure_texture` | `(&self) → Option<Texture>` | — |
| `fence` | `(&self) → Option<Fence>` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `input_content_max_scale` | `(&self) → f32` | — |
| `input_content_min_scale` | `(&self) → f32` | — |
| `input_height` | `(&self) → UInteger` | — |
| `input_width` | `(&self) → UInteger` | — |
| `jitter_offset_x` | `(&self) → f32` | — |
| `jitter_offset_y` | `(&self) → f32` | — |
| `motion_texture` | `(&self) → Option<Texture>` | — |
| `motion_texture_format` | `(&self) → PixelFormat` | — |
| `motion_texture_usage` | `(&self) → TextureUsage` | — |
| `motion_vector_scale_x` | `(&self) → f32` | — |
| `motion_vector_scale_y` | `(&self) → f32` | — |
| `normal_texture` | `(&self) → Option<Texture>` | — |
| `normal_texture_format` | `(&self) → PixelFormat` | — |
| `normal_texture_usage` | `(&self) → TextureUsage` | — |
| `output_height` | `(&self) → UInteger` | — |
| `output_texture` | `(&self) → Option<Texture>` | — |
| `output_texture_format` | `(&self) → PixelFormat` | — |
| `output_texture_usage` | `(&self) → TextureUsage` | — |
| `output_width` | `(&self) → UInteger` | — |
| `pre_exposure` | `(&self) → f32` | — |
| `reactive_mask_texture` | `(&self) → Option<Texture>` | — |
| `reactive_mask_texture_format` | `(&self) → PixelFormat` | — |
| `reactive_texture_usage` | `(&self) → TextureUsage` | — |
| `roughness_texture` | `(&self) → Option<Texture>` | — |
| `roughness_texture_format` | `(&self) → PixelFormat` | — |
| `roughness_texture_usage` | `(&self) → TextureUsage` | — |
| `should_reset_history` | `(&self) → bool` | — |
| `specular_albedo_texture` | `(&self) → Option<Texture>` | — |
| `specular_albedo_texture_format` | `(&self) → PixelFormat` | — |
| `specular_albedo_texture_usage` | `(&self) → TextureUsage` | — |
| `specular_hit_distance_texture` | `(&self) → Option<Texture>` | — |
| `specular_hit_distance_texture_format` | `(&self) → PixelFormat` | — |
| `specular_hit_distance_texture_usage` | `(&self) → TextureUsage` | — |
| `transparency_overlay_texture` | `(&self) → Option<Texture>` | — |
| `transparency_overlay_texture_format` | `(&self) → PixelFormat` | — |
| `transparency_overlay_texture_usage` | `(&self) → TextureUsage` | — |
| `view_to_clip_matrix_raw` | `(&self) → [f32; 16]` | — |
| `world_to_view_matrix_raw` | `(&self) → [f32; 16]` | — |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_depth_reversed` | `(&self) → bool` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_color_texture` | `(&self, texture: &Texture) → void` | — |
| `set_denoise_strength_mask_texture` | `(&self, texture: &Texture) → void` | — |
| `set_depth_reversed` | `(&self, reversed: bool) → void` | — |
| `set_depth_texture` | `(&self, texture: &Texture) → void` | — |
| `set_diffuse_albedo_texture` | `(&self, texture: &Texture) → void` | — |
| `set_exposure_texture` | `(&self, texture: &Texture) → void` | — |
| `set_fence` | `(&self, fence: &Fence) → void` | — |
| `set_jitter_offset_x` | `(&self, offset: f32) → void` | — |
| `set_jitter_offset_y` | `(&self, offset: f32) → void` | — |
| `set_motion_texture` | `(&self, texture: &Texture) → void` | — |
| `set_motion_vector_scale_x` | `(&self, scale: f32) → void` | — |
| `set_motion_vector_scale_y` | `(&self, scale: f32) → void` | — |
| `set_normal_texture` | `(&self, texture: &Texture) → void` | — |
| `set_output_texture` | `(&self, texture: &Texture) → void` | — |
| `set_pre_exposure` | `(&self, pre_exposure: f32) → void` | — |
| `set_reactive_mask_texture` | `(&self, texture: &Texture) → void` | — |
| `set_roughness_texture` | `(&self, texture: &Texture) → void` | — |
| `set_should_reset_history` | `(&self, reset: bool) → void` | — |
| `set_specular_albedo_texture` | `(&self, texture: &Texture) → void` | — |
| `set_specular_hit_distance_texture` | `(&self, texture: &Texture) → void` | — |
| `set_transparency_overlay_texture` | `(&self, texture: &Texture) → void` | — |
| `set_view_to_clip_matrix_raw` | `(&self, matrix: &[f32; 16]) → void` | — |
| `set_world_to_view_matrix_raw` | `(&self, matrix: &[f32; 16]) → void` | — |

---

### `TemporalDenoisedScaler`

C++ equivalent: `NS::TemporalDenoisedScaler`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `color_texture` | `(&self) → Option<Texture>` | — |
| `color_texture_format` | `(&self) → PixelFormat` | — |
| `color_texture_usage` | `(&self) → TextureUsage` | — |
| `denoise_strength_mask_texture` | `(&self) → Option<Texture>` | — |
| `denoise_strength_mask_texture_format` | `(&self) → PixelFormat` | — |
| `denoise_strength_mask_texture_usage` | `(&self) → TextureUsage` | — |
| `depth_texture` | `(&self) → Option<Texture>` | — |
| `depth_texture_format` | `(&self) → PixelFormat` | — |
| `depth_texture_usage` | `(&self) → TextureUsage` | — |
| `diffuse_albedo_texture` | `(&self) → Option<Texture>` | — |
| `diffuse_albedo_texture_format` | `(&self) → PixelFormat` | — |
| `diffuse_albedo_texture_usage` | `(&self) → TextureUsage` | — |
| `encode_to_command_buffer` | `(&self, command_buffer: &met...) → void` | `encodeToCommandBuffer` |
| `encode_to_mtl4_command_buffer` | `(&self, command_buffer: &met...) → void` | — |
| `exposure_texture` | `(&self) → Option<Texture>` | — |
| `fence` | `(&self) → Option<Fence>` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `input_content_max_scale` | `(&self) → f32` | — |
| `input_content_min_scale` | `(&self) → f32` | — |
| `input_height` | `(&self) → UInteger` | — |
| `input_width` | `(&self) → UInteger` | — |
| `jitter_offset_x` | `(&self) → f32` | — |
| `jitter_offset_y` | `(&self) → f32` | — |
| `motion_texture` | `(&self) → Option<Texture>` | — |
| `motion_texture_format` | `(&self) → PixelFormat` | — |
| `motion_texture_usage` | `(&self) → TextureUsage` | — |
| `motion_vector_scale_x` | `(&self) → f32` | — |
| `motion_vector_scale_y` | `(&self) → f32` | — |
| `normal_texture` | `(&self) → Option<Texture>` | — |
| `normal_texture_format` | `(&self) → PixelFormat` | — |
| `normal_texture_usage` | `(&self) → TextureUsage` | — |
| `output_height` | `(&self) → UInteger` | — |
| `output_texture` | `(&self) → Option<Texture>` | — |
| `output_texture_format` | `(&self) → PixelFormat` | — |
| `output_texture_usage` | `(&self) → TextureUsage` | — |
| `output_width` | `(&self) → UInteger` | — |
| `pre_exposure` | `(&self) → f32` | — |
| `reactive_mask_texture` | `(&self) → Option<Texture>` | — |
| `reactive_mask_texture_format` | `(&self) → PixelFormat` | — |
| `reactive_texture_usage` | `(&self) → TextureUsage` | — |
| `roughness_texture` | `(&self) → Option<Texture>` | — |
| `roughness_texture_format` | `(&self) → PixelFormat` | — |
| `roughness_texture_usage` | `(&self) → TextureUsage` | — |
| `should_reset_history` | `(&self) → bool` | — |
| `specular_albedo_texture` | `(&self) → Option<Texture>` | — |
| `specular_albedo_texture_format` | `(&self) → PixelFormat` | — |
| `specular_albedo_texture_usage` | `(&self) → TextureUsage` | — |
| `specular_hit_distance_texture` | `(&self) → Option<Texture>` | — |
| `specular_hit_distance_texture_format` | `(&self) → PixelFormat` | — |
| `specular_hit_distance_texture_usage` | `(&self) → TextureUsage` | — |
| `transparency_overlay_texture` | `(&self) → Option<Texture>` | — |
| `transparency_overlay_texture_format` | `(&self) → PixelFormat` | — |
| `transparency_overlay_texture_usage` | `(&self) → TextureUsage` | — |
| `view_to_clip_matrix_raw` | `(&self) → [f32; 16]` | — |
| `world_to_view_matrix_raw` | `(&self) → [f32; 16]` | — |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_depth_reversed` | `(&self) → bool` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_color_texture` | `(&self, texture: &Texture) → void` | — |
| `set_denoise_strength_mask_texture` | `(&self, texture: &Texture) → void` | — |
| `set_depth_reversed` | `(&self, reversed: bool) → void` | — |
| `set_depth_texture` | `(&self, texture: &Texture) → void` | — |
| `set_diffuse_albedo_texture` | `(&self, texture: &Texture) → void` | — |
| `set_exposure_texture` | `(&self, texture: &Texture) → void` | — |
| `set_fence` | `(&self, fence: &Fence) → void` | — |
| `set_jitter_offset_x` | `(&self, offset: f32) → void` | — |
| `set_jitter_offset_y` | `(&self, offset: f32) → void` | — |
| `set_motion_texture` | `(&self, texture: &Texture) → void` | — |
| `set_motion_vector_scale_x` | `(&self, scale: f32) → void` | — |
| `set_motion_vector_scale_y` | `(&self, scale: f32) → void` | — |
| `set_normal_texture` | `(&self, texture: &Texture) → void` | — |
| `set_output_texture` | `(&self, texture: &Texture) → void` | — |
| `set_pre_exposure` | `(&self, pre_exposure: f32) → void` | — |
| `set_reactive_mask_texture` | `(&self, texture: &Texture) → void` | — |
| `set_roughness_texture` | `(&self, texture: &Texture) → void` | — |
| `set_should_reset_history` | `(&self, reset: bool) → void` | — |
| `set_specular_albedo_texture` | `(&self, texture: &Texture) → void` | — |
| `set_specular_hit_distance_texture` | `(&self, texture: &Texture) → void` | — |
| `set_transparency_overlay_texture` | `(&self, texture: &Texture) → void` | — |
| `set_view_to_clip_matrix_raw` | `(&self, matrix: &[f32; 16]) → void` | — |
| `set_world_to_view_matrix_raw` | `(&self, matrix: &[f32; 16]) → void` | — |

---

### `TemporalDenoisedScalerDescriptor`

C++ equivalent: `NS::TemporalDenoisedScalerDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |
| `new_mtl4_temporal_denoised_scaler` | `(&self,
        device: &Dev...) → Option<TemporalDenoisedScaler>` | — |
| `new_temporal_denoised_scaler` | `(&self, device: &Device) → Option<TemporalDenoisedScaler>` | `newTemporalDenoisedScaler` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `color_texture_format` | `(&self) → PixelFormat` | `colorTextureFormat` |
| `denoise_strength_mask_texture_format` | `(&self) → PixelFormat` | `denoiseStrengthMaskTextureFormat` |
| `depth_texture_format` | `(&self) → PixelFormat` | `depthTextureFormat` |
| `diffuse_albedo_texture_format` | `(&self) → PixelFormat` | `diffuseAlbedoTextureFormat` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `input_content_max_scale` | `(&self) → f32` | `inputContentMaxScale` |
| `input_content_min_scale` | `(&self) → f32` | `inputContentMinScale` |
| `input_height` | `(&self) → UInteger` | `inputHeight` |
| `input_width` | `(&self) → UInteger` | `inputWidth` |
| `motion_texture_format` | `(&self) → PixelFormat` | `motionTextureFormat` |
| `normal_texture_format` | `(&self) → PixelFormat` | `normalTextureFormat` |
| `output_height` | `(&self) → UInteger` | `outputHeight` |
| `output_texture_format` | `(&self) → PixelFormat` | `outputTextureFormat` |
| `output_width` | `(&self) → UInteger` | `outputWidth` |
| `reactive_mask_texture_format` | `(&self) → PixelFormat` | `reactiveMaskTextureFormat` |
| `requires_synchronous_initialization` | `(&self) → bool` | `requiresSynchronousInitialization` |
| `roughness_texture_format` | `(&self) → PixelFormat` | `roughnessTextureFormat` |
| `specular_albedo_texture_format` | `(&self) → PixelFormat` | `specularAlbedoTextureFormat` |
| `specular_hit_distance_texture_format` | `(&self) → PixelFormat` | `specularHitDistanceTextureFormat` |
| `supported_input_content_max_scale` | `(device: &Device) → f32` | `supportedInputContentMaxScale` |
| `supported_input_content_min_scale` | `(device: &Device) → f32` | `supportedInputContentMinScale` |
| `transparency_overlay_texture_format` | `(&self) → PixelFormat` | `transparencyOverlayTextureFormat` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_auto_exposure_enabled` | `(&self) → bool` | `isAutoExposureEnabled` |
| `is_denoise_strength_mask_texture_enabled` | `(&self) → bool` | `isDenoiseStrengthMaskTextureEnabled` |
| `is_input_content_properties_enabled` | `(&self) → bool` | `isInputContentPropertiesEnabled` |
| `is_reactive_mask_texture_enabled` | `(&self) → bool` | `isReactiveMaskTextureEnabled` |
| `is_specular_hit_distance_texture_enabled` | `(&self) → bool` | `isSpecularHitDistanceTextureEnabled` |
| `is_transparency_overlay_texture_enabled` | `(&self) → bool` | `isTransparencyOverlayTextureEnabled` |
| `supports_device` | `(device: &Device) → bool` | `supportsDevice` |
| `supports_metal4_fx` | `(device: &Device) → bool` | `supportsMetal4FX` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_auto_exposure_enabled` | `(&self, enabled: bool) → void` | `setAutoExposureEnabled` |
| `set_color_texture_format` | `(&self, format: PixelFormat) → void` | `setColorTextureFormat` |
| `set_denoise_strength_mask_texture_enabled` | `(&self, enabled: bool) → void` | `setDenoiseStrengthMaskTextureEnabled` |
| `set_denoise_strength_mask_texture_format` | `(&self, format: PixelFormat) → void` | `setDenoiseStrengthMaskTextureFormat` |
| `set_depth_texture_format` | `(&self, format: PixelFormat) → void` | `setDepthTextureFormat` |
| `set_diffuse_albedo_texture_format` | `(&self, format: PixelFormat) → void` | `setDiffuseAlbedoTextureFormat` |
| `set_input_content_max_scale` | `(&self, scale: f32) → void` | `setInputContentMaxScale` |
| `set_input_content_min_scale` | `(&self, scale: f32) → void` | `setInputContentMinScale` |
| `set_input_content_properties_enabled` | `(&self, enabled: bool) → void` | `setInputContentPropertiesEnabled` |
| `set_input_height` | `(&self, height: UInteger) → void` | `setInputHeight` |
| `set_input_width` | `(&self, width: UInteger) → void` | `setInputWidth` |
| `set_motion_texture_format` | `(&self, format: PixelFormat) → void` | `setMotionTextureFormat` |
| `set_normal_texture_format` | `(&self, format: PixelFormat) → void` | `setNormalTextureFormat` |
| `set_output_height` | `(&self, height: UInteger) → void` | `setOutputHeight` |
| `set_output_texture_format` | `(&self, format: PixelFormat) → void` | `setOutputTextureFormat` |
| `set_output_width` | `(&self, width: UInteger) → void` | `setOutputWidth` |
| `set_reactive_mask_texture_enabled` | `(&self, enabled: bool) → void` | `setReactiveMaskTextureEnabled` |
| `set_reactive_mask_texture_format` | `(&self, format: PixelFormat) → void` | `setReactiveMaskTextureFormat` |
| `set_requires_synchronous_initialization` | `(&self, required: bool) → void` | `setRequiresSynchronousInitialization` |
| `set_roughness_texture_format` | `(&self, format: PixelFormat) → void` | `setRoughnessTextureFormat` |
| `set_specular_albedo_texture_format` | `(&self, format: PixelFormat) → void` | `setSpecularAlbedoTextureFormat` |
| `set_specular_hit_distance_texture_enabled` | `(&self, enabled: bool) → void` | `setSpecularHitDistanceTextureEnabled` |
| `set_specular_hit_distance_texture_format` | `(&self, format: PixelFormat) → void` | `setSpecularHitDistanceTextureFormat` |
| `set_transparency_overlay_texture_enabled` | `(&self, enabled: bool) → void` | `setTransparencyOverlayTextureEnabled` |
| `set_transparency_overlay_texture_format` | `(&self, format: PixelFormat) → void` | `setTransparencyOverlayTextureFormat` |

---

### `TemporalScaler`

C++ equivalent: `NS::TemporalScaler`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `color_texture` | `(&self) → Option<metal::Texture>` | — |
| `color_texture_format` | `(&self) → metal::PixelFormat` | — |
| `color_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `depth_texture` | `(&self) → Option<metal::Texture>` | — |
| `depth_texture_format` | `(&self) → metal::PixelFormat` | — |
| `depth_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `encode_to_command_buffer` | `(&self, command_buffer: &met...) → void` | `encodeToCommandBuffer` |
| `encode_to_mtl4_command_buffer` | `(&self, command_buffer: &met...) → void` | — |
| `exposure_texture` | `(&self) → Option<metal::Texture>` | — |
| `fence` | `(&self) → Option<metal::Fence>` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `input_content_height` | `(&self) → UInteger` | — |
| `input_content_max_scale` | `(&self) → f32` | — |
| `input_content_min_scale` | `(&self) → f32` | — |
| `input_content_width` | `(&self) → UInteger` | — |
| `input_height` | `(&self) → UInteger` | — |
| `input_width` | `(&self) → UInteger` | — |
| `jitter_offset_x` | `(&self) → f32` | — |
| `jitter_offset_y` | `(&self) → f32` | — |
| `motion_texture` | `(&self) → Option<metal::Texture>` | — |
| `motion_texture_format` | `(&self) → metal::PixelFormat` | — |
| `motion_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `motion_vector_scale_x` | `(&self) → f32` | — |
| `motion_vector_scale_y` | `(&self) → f32` | — |
| `output_height` | `(&self) → UInteger` | — |
| `output_texture` | `(&self) → Option<metal::Texture>` | — |
| `output_texture_format` | `(&self) → metal::PixelFormat` | — |
| `output_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `output_width` | `(&self) → UInteger` | — |
| `pre_exposure` | `(&self) → f32` | — |
| `reactive_mask_texture` | `(&self) → Option<metal::Texture>` | — |
| `reactive_texture_format` | `(&self) → metal::PixelFormat` | — |
| `reactive_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `reset` | `(&self) → bool` | — |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_depth_reversed` | `(&self) → bool` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_color_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_depth_reversed` | `(&self, reversed: bool) → void` | — |
| `set_depth_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_exposure_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_fence` | `(&self, fence: &metal::Fence) → void` | — |
| `set_input_content_height` | `(&self, height: UInteger) → void` | — |
| `set_input_content_width` | `(&self, width: UInteger) → void` | — |
| `set_jitter_offset_x` | `(&self, offset: f32) → void` | — |
| `set_jitter_offset_y` | `(&self, offset: f32) → void` | — |
| `set_motion_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_motion_vector_scale_x` | `(&self, scale: f32) → void` | — |
| `set_motion_vector_scale_y` | `(&self, scale: f32) → void` | — |
| `set_output_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_pre_exposure` | `(&self, value: f32) → void` | — |
| `set_reactive_mask_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_reset` | `(&self, reset: bool) → void` | — |

---

### `TemporalScaler`

C++ equivalent: `NS::TemporalScaler`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `color_texture` | `(&self) → Option<metal::Texture>` | — |
| `color_texture_format` | `(&self) → metal::PixelFormat` | — |
| `color_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `depth_texture` | `(&self) → Option<metal::Texture>` | — |
| `depth_texture_format` | `(&self) → metal::PixelFormat` | — |
| `depth_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `encode_to_command_buffer` | `(&self, command_buffer: &met...) → void` | `encodeToCommandBuffer` |
| `encode_to_mtl4_command_buffer` | `(&self, command_buffer: &met...) → void` | — |
| `exposure_texture` | `(&self) → Option<metal::Texture>` | — |
| `fence` | `(&self) → Option<metal::Fence>` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `input_content_height` | `(&self) → UInteger` | — |
| `input_content_max_scale` | `(&self) → f32` | — |
| `input_content_min_scale` | `(&self) → f32` | — |
| `input_content_width` | `(&self) → UInteger` | — |
| `input_height` | `(&self) → UInteger` | — |
| `input_width` | `(&self) → UInteger` | — |
| `jitter_offset_x` | `(&self) → f32` | — |
| `jitter_offset_y` | `(&self) → f32` | — |
| `motion_texture` | `(&self) → Option<metal::Texture>` | — |
| `motion_texture_format` | `(&self) → metal::PixelFormat` | — |
| `motion_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `motion_vector_scale_x` | `(&self) → f32` | — |
| `motion_vector_scale_y` | `(&self) → f32` | — |
| `output_height` | `(&self) → UInteger` | — |
| `output_texture` | `(&self) → Option<metal::Texture>` | — |
| `output_texture_format` | `(&self) → metal::PixelFormat` | — |
| `output_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `output_width` | `(&self) → UInteger` | — |
| `pre_exposure` | `(&self) → f32` | — |
| `reactive_mask_texture` | `(&self) → Option<metal::Texture>` | — |
| `reactive_texture_format` | `(&self) → metal::PixelFormat` | — |
| `reactive_texture_usage` | `(&self) → metal::TextureUsage` | — |
| `reset` | `(&self) → bool` | — |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_depth_reversed` | `(&self) → bool` | — |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_color_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_depth_reversed` | `(&self, reversed: bool) → void` | — |
| `set_depth_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_exposure_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_fence` | `(&self, fence: &metal::Fence) → void` | — |
| `set_input_content_height` | `(&self, height: UInteger) → void` | — |
| `set_input_content_width` | `(&self, width: UInteger) → void` | — |
| `set_jitter_offset_x` | `(&self, offset: f32) → void` | — |
| `set_jitter_offset_y` | `(&self, offset: f32) → void` | — |
| `set_motion_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_motion_vector_scale_x` | `(&self, scale: f32) → void` | — |
| `set_motion_vector_scale_y` | `(&self, scale: f32) → void` | — |
| `set_output_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_pre_exposure` | `(&self, value: f32) → void` | — |
| `set_reactive_mask_texture` | `(&self, texture: &metal::Tex...) → void` | — |
| `set_reset` | `(&self, reset: bool) → void` | — |

---

### `TemporalScalerDescriptor`

C++ equivalent: `NS::TemporalScalerDescriptor`

#### Creation

| Method | Signature | C++ |
|--------|-----------|-----|
| `new` | `() → Option<Self>` | `alloc` |
| `new_temporal_scaler` | `(&self, device: &metal::Device) → Option<TemporalScaler>` | `newTemporalScaler` |

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `color_texture_format` | `(&self) → metal::PixelFormat` | `colorTextureFormat` |
| `depth_texture_format` | `(&self) → metal::PixelFormat` | `depthTextureFormat` |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `input_content_max_scale` | `(&self) → f32` | `inputContentMaxScale` |
| `input_content_min_scale` | `(&self) → f32` | `inputContentMinScale` |
| `input_height` | `(&self) → UInteger` | `inputHeight` |
| `input_width` | `(&self) → UInteger` | `inputWidth` |
| `motion_texture_format` | `(&self) → metal::PixelFormat` | `motionTextureFormat` |
| `output_height` | `(&self) → UInteger` | `outputHeight` |
| `output_texture_format` | `(&self) → metal::PixelFormat` | `outputTextureFormat` |
| `output_width` | `(&self) → UInteger` | `outputWidth` |
| `reactive_mask_texture_format` | `(&self) → metal::PixelFormat` | `reactiveMaskTextureFormat` |
| `requires_synchronous_initialization` | `(&self) → bool` | `requiresSynchronousInitialization` |

#### Queries

| Method | Signature | C++ |
|--------|-----------|-----|
| `is_auto_exposure_enabled` | `(&self) → bool` | `isAutoExposureEnabled` |
| `is_input_content_properties_enabled` | `(&self) → bool` | `isInputContentPropertiesEnabled` |
| `is_reactive_mask_texture_enabled` | `(&self) → bool` | `isReactiveMaskTextureEnabled` |
| `supports_device` | `(device: &metal::Device) → bool` | `supportsDevice` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_auto_exposure_enabled` | `(&self, enabled: bool) → void` | `setAutoExposureEnabled` |
| `set_color_texture_format` | `(&self, format: metal::Pixel...) → void` | `setColorTextureFormat` |
| `set_depth_texture_format` | `(&self, format: metal::Pixel...) → void` | `setDepthTextureFormat` |
| `set_input_content_max_scale` | `(&self, scale: f32) → void` | `setInputContentMaxScale` |
| `set_input_content_min_scale` | `(&self, scale: f32) → void` | `setInputContentMinScale` |
| `set_input_content_properties_enabled` | `(&self, enabled: bool) → void` | `setInputContentPropertiesEnabled` |
| `set_input_height` | `(&self, height: UInteger) → void` | `setInputHeight` |
| `set_input_width` | `(&self, width: UInteger) → void` | `setInputWidth` |
| `set_motion_texture_format` | `(&self, format: metal::Pixel...) → void` | `setMotionTextureFormat` |
| `set_output_height` | `(&self, height: UInteger) → void` | `setOutputHeight` |
| `set_output_texture_format` | `(&self, format: metal::Pixel...) → void` | `setOutputTextureFormat` |
| `set_output_width` | `(&self, width: UInteger) → void` | `setOutputWidth` |
| `set_reactive_mask_texture_enabled` | `(&self, enabled: bool) → void` | `setReactiveMaskTextureEnabled` |
| `set_reactive_mask_texture_format` | `(&self, format: metal::Pixel...) → void` | `setReactiveMaskTextureFormat` |
| `set_requires_synchronous_initialization` | `(&self, required: bool) → void` | `setRequiresSynchronousInitialization` |

---

## quartz-core

### `MetalDrawable`

C++ equivalent: `NS::MetalDrawable`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `as_raw` | `(&self) → *mut c_void` | — |
| `drawable_id` | `(&self) → UInteger` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `layer` | `(&self) → Option<MetalLayer>` | `layer` |
| `present` | `(&self) → void` | — |
| `present_after_minimum_duration` | `(&self, duration: TimeInterval) → void` | — |
| `present_at_time` | `(&self, presentation_time: T...) → void` | — |
| `presented_time` | `(&self) → TimeInterval` | — |
| `texture` | `(&self) → Option<metal::Texture>` | `texture` |

---

### `MetalLayer`

C++ equivalent: `NS::MetalLayer`

#### Methods

| Method | Signature | C++ |
|--------|-----------|-----|
| `allows_next_drawable_timeout` | `(&self) → bool` | `allowsNextDrawableTimeout` |
| `as_raw` | `(&self) → *mut c_void` | — |
| `colorspace` | `(&self) → CGColorSpaceRef` | `colorspace` |
| `device` | `(&self) → Option<metal::Device>` | `device` |
| `display_sync_enabled` | `(&self) → bool` | `displaySyncEnabled` |
| `drawable_size` | `(&self) → CGSize` | `drawableSize` |
| `framebuffer_only` | `(&self) → bool` | — |
| `from_raw` | `(ptr: *mut c_void) → Option<Self>` | — |
| `layer` | `() → Option<Self>` | `layer` |
| `maximum_drawable_count` | `(&self) → UInteger` | `maximumDrawableCount` |
| `next_drawable` | `(&self) → Option<MetalDrawable>` | `nextDrawable` |
| `pixel_format` | `(&self) → metal::PixelFormat` | `pixelFormat` |
| `residency_set` | `(&self) → Option<metal::ResidencySet>` | `residencySet` |

#### Setters

| Method | Signature | C++ |
|--------|-----------|-----|
| `set_allows_next_drawable_timeout` | `(&self, allows: bool) → void` | `setAllowsNextDrawableTimeout` |
| `set_colorspace` | `(&self, colorspace: CGColorS...) → void` | `setColorspace` |
| `set_device` | `(&self, device: &metal::Device) → void` | `setDevice` |
| `set_display_sync_enabled` | `(&self, enabled: bool) → void` | `setDisplaySyncEnabled` |
| `set_drawable_size` | `(&self, drawable_size: CGSize) → void` | `setDrawableSize` |
| `set_framebuffer_only` | `(&self, framebuffer_only: bool) → void` | `setFramebufferOnly` |
| `set_maximum_drawable_count` | `(&self, count: UInteger) → void` | `setMaximumDrawableCount` |
| `set_pixel_format` | `(&self, pixel_format: metal:...) → void` | `setPixelFormat` |

---
