# Metal API Coverage Report

Tracking translation completeness from metal-cpp to Rust.

## Summary

| Category | C++ Count | Rust Mapped | N/A | Coverage |
|----------|-----------|-------------|-----|----------|
| Classes/Structs | 253 | 253 | 15 | **100%** |
| Methods | 3175 | 2963 | — | **93%** |
| Enums | 125 | 125 | — | **100%** |
| Enum Values | 903 | 903 | — | **100%** |

### Method Match Quality

| Match Type | Count | Description |
|------------|-------|-------------|
| Exact | 2299 | Direct name conversion match |
| Convention | 190 | Matched via naming rules (overloads, aliases) |
| Pattern | 286 | Matched via pattern (alloc/init → new) |
| Fuzzy | 188 | Matched via similarity (needs review) |
| Deprecated | 28 | C++ deprecated aliases (not implemented) |
| N/A | 184 | Not applicable (reflection types, etc.) |
| Missing | 0 | No Rust equivalent found |

### Protocol Types (N/A)

These C++ types are Objective-C protocols or abstract base classes.
They are implemented via Rust traits or have their methods on concrete types:

**Rust Traits:**
- `Allocation` → `metal::allocation::Allocation trait`
- `Resource` → `metal::resource::Resource trait`

**Abstract Base Classes:**
- `CommandEncoder` → Methods on concrete encoder types
- `CommandEncoder` → Methods on concrete encoder types
- `ResourceViewPool` → Methods on TextureViewPool
- `FrameInterpolatorBase` → Methods on FrameInterpolator
- `SpatialScalerBase` → Methods on SpatialScaler
- `TemporalDenoisedScalerBase` → Methods on TemporalDenoisedScaler
- `FrameInterpolatableScaler` → Protocol for frame interpolation scalers
- `TemporalScalerBase` → Methods on TemporalScaler

**Objective-C Protocols:**
- `FastEnumeration` → Iterator trait
- `Locking` → Mutex/RwLock patterns
- `Copying` → Clone trait
- `SecureCoding` → Not applicable in Rust

**Foundation Types:**
- `URL` → Use &str/Path directly

### Deprecated C++ Aliases (Not Implemented)

These C++ methods are deprecated aliases. Use the canonical Rust methods instead:

| C++ Deprecated | Rust Canonical |
|----------------|----------------|
| `active()` | `is_active()` |
| `alphaToCoverageEnabled()` | `is_alpha_to_coverage_enabled()` |
| `alphaToOneEnabled()` | `is_alpha_to_one_enabled()` |
| `barycentricCoordsSupported()` | `are_barycentric_coords_supported()` |
| `blendingEnabled()` | `is_blending_enabled()` |
| `depth24Stencil8PixelFormatSupported()` | `is_depth24_stencil8_pixel_format_supported()` |
| `depthWriteEnabled()` | `is_depth_write_enabled()` |
| `framebufferOnly()` | `is_framebuffer_only()` |
| `headless()` | `is_headless()` |
| `lowPower()` | `is_low_power()` |
| `patchControlPointData()` | `is_patch_control_point_data()` |
| `patchData()` | `is_patch_data()` |
| `programmableSamplePositionsSupported()` | `are_programmable_sample_positions_supported()` |
| `rasterOrderGroupsSupported()` | `are_raster_order_groups_supported()` |
| `rasterizationEnabled()` | `is_rasterization_enabled()` |
| `removable()` | `is_removable()` |
| `shareable()` | `is_shareable()` |
| `tessellationFactorScaleEnabled()` | `is_tessellation_factor_scale_enabled()` |

---

## Missing Classes

All classes are mapped!

## Missing Methods by Class

---

## Missing Enums

All enums are mapped!

---

## Fuzzy Matches (Review Needed)

These matches were made via similarity and should be verified:

### ComputeCommandEncoder

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `copyFromBuffer` | `copy_from_buffer_to_buffer` | 68% |
| `copyFromBuffer` | `copy_from_texture_to_buffer` | 67% |
| `copyFromBuffer` | `copy_from_texture_to_buffer` | 67% |
| `copyFromTexture` | `copy_from_texture_to_texture` | 68% |
| `copyFromTexture` | `copy_from_tensor` | 71% |
| `copyFromTexture` | `copy_from_texture_to_buffer` | 69% |
| `copyFromTexture` | `copy_from_texture_to_buffer` | 69% |
| `copyFromTexture` | `copy_from_texture_to_buffer` | 69% |

### CounterHeapDescriptor

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `setType` | `set_heap_type` | 68% |
| `type` | `heap_type` | 57% |

### CounterHeap

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `type` | `heap_type` | 57% |

### RenderCommandEncoder

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `executeCommandsInBuffer` | `execute_commands_in_buffer_ptr` | 74% |
| `executeCommandsInBuffer` | `execute_commands_in_buffer_ptr` | 74% |
| `setArgumentTable` | `set_tile_argument_table` | 78% |
| `setColorAttachmentMap` | `set_color_attachment_map_ptr` | 81% |

### RenderPassDescriptor

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `getSamplePositions` | `get_sample_positions` | 79% |

### InstanceAccelerationStructureDescriptor

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `instancedAccelerationStructures` | `instanced_acceleration_structures_ptr` | 83% |
| `setInstancedAccelerationStructures` | `set_instanced_acceleration_structures_ptr` | 83% |

### AccelerationStructureCommandEncoder

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `sampleCountersInBuffer` | `sample_counters_in_buffer_ptr` | 82% |
| `useHeaps` | `use_heap` | 83% |
| `useResources` | `use_resource` | 84% |

### AccelerationStructurePassSampleBufferAttachmentDescriptor

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `sampleBuffer` | `sample_buffer_ptr` | 77% |
| `setSampleBuffer` | `set_sample_buffer_ptr` | 79% |

### StructType

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `members` | `members_ptr` | 70% |

### TensorReferenceType

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `dimensions` | `dimensions_ptr` | 74% |

### TensorBinding

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `dimensions` | `dimensions_ptr` | 74% |

### ArgumentEncoder

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `setBuffers` | `set_buffer` | 84% |
| `setComputePipelineStates` | `set_compute_pipeline_state` | 86% |
| `setDepthStencilStates` | `set_depth_stencil_state` | 86% |
| `setIndirectCommandBuffer` | `set_indirect_command_buffer_ptr` | 82% |
| `setIndirectCommandBuffers` | `set_indirect_command_buffer_ptr` | 81% |
| `setIntersectionFunctionTable` | `set_intersection_function_table_ptr` | 83% |
| `setIntersectionFunctionTables` | `set_intersection_function_table_ptr` | 82% |
| `setRenderPipelineStates` | `set_render_pipeline_state` | 86% |
| `setSamplerStates` | `set_sampler_state` | 85% |
| `setTextures` | `set_texture` | 84% |
| `setVisibleFunctionTable` | `set_visible_function_table_ptr` | 82% |
| `setVisibleFunctionTables` | `set_visible_function_table_ptr` | 80% |

### BinaryArchive

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `addLibrary` | `add_library_ptr` | 75% |
| `addMeshRenderPipelineFunctions` | `add_mesh_render_pipeline_functions_ptr` | 83% |
| `addTileRenderPipelineFunctions` | `add_tile_render_pipeline_functions_ptr` | 83% |

### BlitCommandEncoder

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `copyFromBuffer` | `copy_from_buffer_to_texture` | 67% |
| `copyFromBuffer` | `copy_from_buffer_to_texture` | 67% |
| `copyFromBuffer` | `copy_from_buffer_to_buffer` | 68% |
| `copyFromTensor` | `copy_from_tensor_ptr` | 79% |
| `copyFromTexture` | `copy_from_buffer_to_texture` | 69% |
| `copyFromTexture` | `copy_from_buffer_to_texture` | 69% |
| `copyFromTexture` | `copy_from_buffer_to_texture` | 69% |
| `copyFromTexture` | `copy_from_tensor_ptr` | 68% |
| `copyFromTexture` | `copy_from_texture_to_texture` | 68% |
| `copyIndirectCommandBuffer` | `copy_indirect_command_buffer_ptr` | 74% |
| `getTextureAccessCounters` | `get_texture_access_counters` | 81% |
| `optimizeIndirectCommandBuffer` | `optimize_indirect_command_buffer_ptr` | 75% |
| `resetCommandsInBuffer` | `reset_commands_in_buffer_ptr` | 73% |
| `resolveCounters` | `resolve_counters_ptr` | 71% |
| `sampleCountersInBuffer` | `sample_counters_in_buffer_ptr` | 82% |
| `synchronizeResource` | `synchronize_resource_ptr` | 80% |

### CaptureManager

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `newCaptureScope` | `default_capture_scope` | 71% |
| `newCaptureScope` | `default_capture_scope` | 71% |
| `newCaptureScope` | `default_capture_scope` | 71% |
| `sharedCaptureManager` | `start_capture` | 58% |

### CommandBufferEncoderInfo

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `debugSignposts` | `debug_signposts_ptr` | 78% |

### CommandBuffer

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `encodeWait` | `encode_wait_for_event` | 63% |

### ComputeCommandEncoder

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `memoryBarrier` | `memory_barrier_with_scope` | 65% |
| `memoryBarrier` | `memory_barrier_with_scope` | 65% |
| `sampleCountersInBuffer` | `sample_counters_in_buffer_ptr` | 82% |
| `setIntersectionFunctionTable` | `set_intersection_function_table_ptr` | 83% |
| `setIntersectionFunctionTables` | `set_intersection_function_table_ptr` | 82% |
| `setSamplerStates` | `set_sampler_state` | 85% |
| `setSamplerStates` | `set_sampler_states_ptr` | 80% |
| `setVisibleFunctionTable` | `set_visible_function_table_ptr` | 82% |
| `setVisibleFunctionTables` | `set_visible_function_table_ptr` | 80% |

### Device

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `accelerationStructureSizes` | `new_acceleration_structure_with_size` | 75% |
| `getDefaultSamplePositions` | `get_default_sample_positions` | 81% |
| `heapAccelerationStructureSizeAndAlign` | `heap_acceleration_structure_size_and_align_with_size` | 79% |
| `heapAccelerationStructureSizeAndAlign` | `heap_acceleration_structure_size_and_align_with_size` | 79% |
| `newArchive` | `new_binary_archive` | 68% |
| `newCommandBuffer` | `new_command_queue` | 72% |
| `newDynamicLibrary` | `new_default_library` | 58% |
| `newDynamicLibrary` | `new_default_library` | 58% |
| `newSharedTexture` | `new_texture` | 68% |
| `newSharedTexture` | `new_texture` | 68% |
| `supports32BitFloatFiltering` | `supports_32bit_float_filtering` | 85% |
| `supports32BitMSAA` | `supports_32bit_msaa` | 83% |
| `tensorSizeAndAlign` | `heap_texture_size_and_align` | 68% |

### SharedEventListener

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `sharedListener` | `shared` | 53% |

### SharedEvent

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `waitUntilSignaledValue` | `set_signaled_value` | 67% |

### FunctionConstantValues

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `setConstantValue` | `set_constant_values` | 77% |
| `setConstantValue` | `set_constant_values` | 77% |

### FunctionLog

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `type` | `log_type` | 61% |

### FunctionStitchingFunctionNode

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `arguments` | `arguments_ptr` | 73% |
| `controlDependencies` | `control_dependencies_ptr` | 80% |
| `setArguments` | `set_arguments_ptr` | 77% |
| `setControlDependencies` | `set_control_dependencies_ptr` | 81% |

### FunctionStitchingGraph

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `attributes` | `attributes_ptr` | 74% |
| `nodes` | `nodes_ptr` | 65% |
| `setAttributes` | `set_attributes_ptr` | 78% |
| `setNodes` | `set_nodes_ptr` | 73% |

### StitchedLibraryDescriptor

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `binaryArchives` | `binary_archives_ptr` | 78% |
| `functionGraphs` | `function_graphs_ptr` | 78% |
| `functions` | `functions_ptr` | 73% |
| `setBinaryArchives` | `set_binary_archives_ptr` | 80% |
| `setFunctionGraphs` | `set_function_graphs_ptr` | 80% |
| `setFunctions` | `set_functions_ptr` | 77% |

### HeapDescriptor

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `setType` | `set_heap_type` | 68% |
| `type` | `heap_type` | 57% |

### Heap

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `type` | `heap_type` | 57% |

### IOCommandQueueDescriptor

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `scratchBufferAllocator` | `scratch_buffer_allocator_ptr` | 81% |
| `setScratchBufferAllocator` | `set_scratch_buffer_allocator_ptr` | 82% |
| `setType` | `set_queue_type` | 66% |
| `type` | `queue_type` | 53% |

### IndirectRenderCommand

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `drawIndexedPatches` | `draw_indexed_primitives` | 63% |
| `drawPatches` | `draw_primitives` | 53% |

### IntersectionFunctionTable

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `setOpaqueCurveIntersectionFunction` | `set_opaque_curve_intersection_function_at_index` | 79% |
| `setOpaqueCurveIntersectionFunction` | `set_opaque_curve_intersection_function_at_index` | 79% |
| `setOpaqueTriangleIntersectionFunction` | `set_opaque_triangle_intersection_function_at_index` | 80% |
| `setOpaqueTriangleIntersectionFunction` | `set_opaque_triangle_intersection_function_at_index` | 80% |

### CompileOptions

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `libraries` | `library_type` | 61% |
| `setLibraries` | `set_library_type` | 68% |

### RenderCommandEncoder

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `executeCommandsInBuffer` | `execute_commands_in_buffer_ptr` | 74% |
| `executeCommandsInBuffer` | `execute_commands_in_buffer_ptr` | 82% |
| `memoryBarrier` | `memory_barrier_with_scope` | 65% |
| `memoryBarrier` | `memory_barrier_with_scope` | 65% |
| `sampleCountersInBuffer` | `sample_counters_in_buffer_ptr` | 82% |
| `setColorAttachmentMap` | `set_color_attachment_map_ptr` | 81% |
| `setFragmentBuffers` | `set_fragment_buffer` | 85% |
| `setFragmentIntersectionFunctionTable` | `set_fragment_intersection_function_table_ptr` | 84% |
| `setFragmentIntersectionFunctionTables` | `set_fragment_intersection_function_table_ptr` | 83% |
| `setFragmentSamplerStates` | `set_fragment_sampler_state` | 86% |
| `setFragmentSamplerStates` | `set_fragment_sampler_states_ptr` | 82% |
| `setFragmentTextures` | `set_fragment_texture` | 86% |
| `setFragmentVisibleFunctionTable` | `set_fragment_visible_function_table_ptr` | 83% |
| `setFragmentVisibleFunctionTables` | `set_fragment_visible_function_table_ptr` | 82% |
| `setMeshBuffers` | `set_mesh_buffer` | 85% |
| `setMeshSamplerStates` | `set_mesh_sampler_state` | 86% |
| `setMeshSamplerStates` | `set_mesh_sampler_states_ptr` | 81% |
| `setMeshTextures` | `set_mesh_texture` | 85% |
| `setObjectBuffers` | `set_object_buffer` | 85% |
| `setObjectSamplerStates` | `set_object_sampler_state` | 86% |
| ... | ... | (23 more) |

### RenderPassColorAttachmentDescriptorArray

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `object` | `object_at` | 72% |
| `setObject` | `set_object_at` | 77% |

### RenderPassDescriptor

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `getSamplePositions` | `get_sample_positions` | 79% |
| `sampleBufferAttachments` | `color_attachments` | 61% |

### RenderPipelineState

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `newRenderPipelineDescriptor` | `new_render_pipeline_state` | 61% |

### ResidencySet

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `addAllocation` | `add_allocation_ptr` | 78% |
| `addAllocations` | `add_allocations_ptr` | 78% |
| `allAllocations` | `all_allocations_ptr` | 78% |
| `containsAllocation` | `contains_allocation_ptr` | 80% |
| `removeAllocation` | `remove_allocation_ptr` | 79% |
| `removeAllocations` | `remove_allocations_ptr` | 80% |

### BufferLayoutDescriptorArray

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `object` | `object_at` | 72% |
| `setObject` | `set_object_at` | 77% |

### AttributeDescriptorArray

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `object` | `object_at` | 72% |
| `setObject` | `set_object_at` | 77% |

### Tensor

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `getBytes` | `get_bytes` | 65% |

### TextureDescriptor

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `texture2DDescriptor` | `texture_2d_descriptor` | 84% |

### Texture

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `getBytes` | `get_bytes` | 65% |
| `getBytes` | `get_bytes` | 57% |
| `rootResource` | `gpu_resource_id` | 59% |

### ProcessInfo

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `isiOSAppOnMac` | `is_ios_app_on_mac` | 83% |

### FrameInterpolatorDescriptor

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `supportsMetal4FX` | `supports_device` | 56% |

### SpatialScalerDescriptor

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `supportsMetal4FX` | `supports_device` | 56% |

### TemporalScalerDescriptor

| C++ Method | Rust Match | Confidence |
|------------|------------|------------|
| `supportedInputContentMinScale` | `set_input_content_min_scale` | 77% |
| `supportedInputContentMaxScale` | `set_input_content_max_scale` | 77% |
| `supportsMetal4FX` | `supports_device` | 56% |
