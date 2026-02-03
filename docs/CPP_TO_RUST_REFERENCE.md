# Metal C++ to Rust API Reference

Quick reference for C++ Metal developers using these Rust bindings.

## Naming Conventions

| C++ Pattern | Rust Pattern | Example |
|-------------|--------------|---------|
| `newXxxWithYyy(...)` | `new_xxx(...)` | `newBufferWithLength` → `new_buffer` |
| `setXxx(value)` | `set_xxx(value)` | `setLabel` → `set_label` |
| `xxx()` (getter) | `xxx()` | `label()` → `label()` |
| `isXxx()` | `is_xxx()` | `isHeadless` → `is_headless` |
| `hasXxx()` | `has_xxx()` | `hasUnifiedMemory` → `has_unified_memory` |
| `xxxAtIndex(i)` | `xxx_at_index(i)` | `bufferAtIndex` → `buffer_at_index` |
| `supportsXxx()` | `supports_xxx()` | `supportsFamily` → `supports_family` |
| `alloc()` + `init()` | `new()` | `Descriptor::alloc()->init()` → `Descriptor::new()` |

## Type Mappings

| C++ Type | Rust Type |
|----------|-----------|
| `MTL::BlitCommandEncoder*` | `&BlitCommandEncoder` |
| `MTL::Buffer*` | `&Buffer / Buffer` |
| `MTL::CommandBuffer*` | `&CommandBuffer / CommandBuffer` |
| `MTL::CommandQueue*` | `&CommandQueue / CommandQueue` |
| `MTL::ComputeCommandEncoder*` | `&ComputeCommandEncoder` |
| `MTL::Device*` | `&Device / Device` |
| `MTL::RenderCommandEncoder*` | `&RenderCommandEncoder` |
| `MTL::Texture*` | `&Texture / Texture` |
| `NS::Array*` | `Vec<T> / &[T]` |
| `NS::Error*` | `Error` |
| `NS::Error**` | `Result<T, Error>` |
| `NS::Integer` | `Integer` |
| `NS::Object*` | `*mut c_void` |
| `NS::Range` | `Range` |
| `NS::String const*` | `&str` |
| `NS::String*` | `&str / String` |
| `NS::UInteger` | `UInteger` |
| `NS::URL*` | `&str (path)` |
| `bool` | `bool` |
| `double` | `f64` |
| `float` | `f32` |
| `int16_t` | `i16` |
| `int32_t` | `i32` |
| `int64_t` | `i64` |
| `int8_t` | `i8` |
| `size_t` | `usize` |
| `uint16_t` | `u16` |
| `uint32_t` | `u32` |
| `uint64_t` | `u64` |
| `uint8_t` | `u8` |
| `void` | `()` |

## Memory Management

| C++ Pattern | Rust Pattern |
|-------------|--------------|
| `Type* obj = ...` | `let obj: Option<Type> = ...` |
| `obj->method()` | `obj.method()` |
| `obj->release()` | (automatic via `Drop`) |
| `obj->retain()` | `obj.clone()` |
| Error via `NS::Error**` | `Result<T, Error>` |

---

## API Reference by Class

## MTL Namespace

### Array

**Rust:** `Array` in `metal-foundation::src::array`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `array()` | `n/a()` |
| `array()` | `n/a()` |
| `array()` | `n/a()` |
| `count()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `object()` | `n/a()` |
| `objectEnumerator()` | `n/a()` |

### AutoreleasePool

**Rust:** `AutoreleasePool` in `metal-foundation::src::autorelease`

| C++ | Rust |
|-----|------|
| `addObject()` | `add_object()` |
| `alloc()` | `new()` |
| `drain()` | `drain()` |
| `init()` | `new()` |
| `showPools()` | `show_pools()` |

### Bundle

**Rust:** `Bundle` in `metal-foundation::src::bundle`

| C++ | Rust |
|-----|------|
| `URLForAuxiliaryExecutable()` | `n/a()` |
| `allBundles()` | `n/a()` |
| `allFrameworks()` | `n/a()` |
| `alloc()` | `n/a()` |
| `appStoreReceiptURL()` | `n/a()` |
| `builtInPlugInsPath()` | `n/a()` |
| `builtInPlugInsURL()` | `n/a()` |
| `bundle()` | `n/a()` |
| `bundle()` | `n/a()` |
| `bundleIdentifier()` | `n/a()` |
| `bundlePath()` | `n/a()` |
| `bundleURL()` | `n/a()` |
| `executablePath()` | `n/a()` |
| `executableURL()` | `n/a()` |
| `infoDictionary()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `isLoaded()` | `n/a()` |
| `load()` | `n/a()` |
| `loadAndReturnError()` | `n/a()` |
| `localizedInfoDictionary()` | `n/a()` |
| `localizedString()` | `n/a()` |
| `mainBundle()` | `n/a()` |
| `objectForInfoDictionaryKey()` | `n/a()` |
| `pathForAuxiliaryExecutable()` | `n/a()` |
| `preflightAndReturnError()` | `n/a()` |
| `privateFrameworksPath()` | `n/a()` |
| `privateFrameworksURL()` | `n/a()` |
| `resourcePath()` | `n/a()` |
| `resourceURL()` | `n/a()` |
| `sharedFrameworksPath()` | `n/a()` |
| `sharedFrameworksURL()` | `n/a()` |
| `sharedSupportPath()` | `n/a()` |
| `sharedSupportURL()` | `n/a()` |
| `unload()` | `n/a()` |

### Condition

**Rust:** `Condition` in `metal-foundation::src::lock`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `broadcast()` | `broadcast()` |
| `init()` | `new()` |
| `signal()` | `signal()` |
| `wait()` | `wait()` |
| `waitUntilDate()` | `wait_until_date()` |

### Data

**Rust:** `Data` in `metal-foundation::src::data`

| C++ | Rust |
|-----|------|
| `length()` | `length()` |
| `mutableBytes()` | `mutable_bytes()` |

### Date

**Rust:** `Date` in `metal-foundation::src::date`

| C++ | Rust |
|-----|------|
| `dateWithTimeIntervalSinceNow()` | `date_with_time_interval_since_now()` |

### Enumerator

**Rust:** `Enumerator` in `metal-foundation::src::enumerator`

| C++ | Rust |
|-----|------|
| `allObjects()` | `n/a()` |
| `nextObject()` | `n/a()` |

### Error

**Rust:** `Error` in `metal-foundation::src::error`

| C++ | Rust |
|-----|------|
| `alloc()` | `alloc()` |
| `code()` | `code()` |
| `domain()` | `domain()` |
| `error()` | `error()` |
| `init()` | `init()` |
| `init()` | `init()` |
| `localizedDescription()` | `localized_description()` |
| `localizedFailureReason()` | `localized_failure_reason()` |
| `localizedRecoveryOptions()` | `localized_recovery_options()` |
| `localizedRecoverySuggestion()` | `localized_recovery_suggestion()` |
| `userInfo()` | `user_info()` |

### Number

**Rust:** `Number` in `metal-foundation::src::number`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `boolValue()` | `n/a()` |
| `charValue()` | `n/a()` |
| `compare()` | `n/a()` |
| `descriptionWithLocale()` | `n/a()` |
| `doubleValue()` | `n/a()` |
| `floatValue()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `intValue()` | `n/a()` |
| `integerValue()` | `n/a()` |
| `isEqualToNumber()` | `n/a()` |
| `longLongValue()` | `n/a()` |
| `longValue()` | `n/a()` |
| `number()` | `n/a()` |
| `number()` | `n/a()` |
| `number()` | `n/a()` |
| `number()` | `n/a()` |
| `number()` | `n/a()` |
| `number()` | `n/a()` |
| `number()` | `n/a()` |
| `number()` | `n/a()` |
| `number()` | `n/a()` |
| `number()` | `n/a()` |
| `number()` | `n/a()` |
| `number()` | `n/a()` |
| `number()` | `n/a()` |
| `shortValue()` | `n/a()` |
| `stringValue()` | `n/a()` |
| `unsignedCharValue()` | `n/a()` |
| `unsignedIntValue()` | `n/a()` |
| `unsignedIntegerValue()` | `n/a()` |
| `unsignedLongLongValue()` | `n/a()` |
| `unsignedLongValue()` | `n/a()` |
| `unsignedShortValue()` | `n/a()` |

### Object

**Rust:** `Object` in `metal-foundation::src::object`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `alloc()` | `n/a()` |
| `bridgingCast()` | `n/a()` |
| `debugDescription()` | `n/a()` |
| `description()` | `n/a()` |
| `doesRequireMsgSendStret()` | `n/a()` |
| `hash()` | `n/a()` |
| `init()` | `n/a()` |
| `isEqual()` | `n/a()` |
| `methodSignatureForSelector()` | `n/a()` |
| `respondsToSelector()` | `n/a()` |
| `sendMessage()` | `n/a()` |
| `sendMessageSafe()` | `n/a()` |

### ProcessInfo

**Rust:** `ProcessInfo` in `metal-foundation::src::process_info`

| C++ | Rust |
|-----|------|
| `activeProcessorCount()` | `active_processor_count()` |
| `arguments()` | `arguments()` |
| `automaticTerminationSupportEnabled()` | `automatic_termination_support_enabled()` |
| `beginActivity()` | `begin_activity()` |
| `disableAutomaticTermination()` | `disable_automatic_termination()` |
| `disableSuddenTermination()` | `disable_sudden_termination()` |
| `enableAutomaticTermination()` | `enable_automatic_termination()` |
| `enableSuddenTermination()` | `enable_sudden_termination()` |
| `endActivity()` | `end_activity()` |
| `environment()` | `environment()` |
| `fullUserName()` | `full_user_name()` |
| `globallyUniqueString()` | `globally_unique_string()` |
| `hasPerformanceProfile()` | `has_performance_profile()` |
| `hostName()` | `host_name()` |
| `isDeviceCertified()` | `is_device_certified()` |
| `isLowPowerModeEnabled()` | `is_low_power_mode_enabled()` |
| `isMacCatalystApp()` | `is_mac_catalyst_app()` |
| `isOperatingSystemAtLeastVersion()` | `is_operating_system_at_least_version()` |
| `isiOSAppOnMac()` | `is_ios_app_on_mac()` |
| `operatingSystem()` | `operating_system()` |
| `operatingSystemVersion()` | `operating_system_version()` |
| `operatingSystemVersionString()` | `operating_system_version_string()` |
| `physicalMemory()` | `physical_memory()` |
| `processIdentifier()` | `process_identifier()` |
| `processInfo()` | `process_info()` |
| `processName()` | `process_name()` |
| `processorCount()` | `processor_count()` |
| `setAutomaticTerminationSupportEnabled()` | `set_automatic_termination_support_enabled()` |
| `setProcessName()` | `set_process_name()` |
| `systemUptime()` | `system_uptime()` |
| `thermalState()` | `thermal_state()` |
| `userName()` | `user_name()` |

### String

**Rust:** `String` in `metal-foundation::src::string`

| C++ | Rust |
|-----|------|
| `alloc()` | `alloc()` |
| `cString()` | `c_string()` |
| `caseInsensitiveCompare()` | `case_insensitive_compare()` |
| `character()` | `character()` |
| `fileSystemRepresentation()` | `file_system_representation()` |
| `init()` | `init()` |
| `init()` | `init()` |
| `init()` | `init()` |
| `init()` | `init()` |
| `isEqualToString()` | `is_equal_to_string()` |
| `length()` | `length()` |
| `lengthOfBytes()` | `length_of_bytes()` |
| `maximumLengthOfBytes()` | `maximum_length_of_bytes()` |
| `rangeOfString()` | `range_of_string()` |
| `string()` | `string()` |
| `string()` | `string()` |
| `string()` | `string()` |
| `stringByAppendingString()` | `string_by_appending_string()` |
| `utf8String()` | `utf8_string()` |

### Value

**Rust:** `Value` in `metal-foundation::src::number`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `getValue()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `isEqualToValue()` | `n/a()` |
| `objCType()` | `n/a()` |
| `pointerValue()` | `n/a()` |
| `value()` | `n/a()` |
| `value()` | `n/a()` |

## NS Namespace

### AccelerationStructure

**Rust:** `AccelerationStructure` in `metal::src::acceleration::structure`

| C++ | Rust |
|-----|------|
| `gpuResourceID()` | `gpu_resource_id()` |
| `size()` | `size()` |

### AccelerationStructureBoundingBoxGeometryDescriptor

**Rust:** `AccelerationStructureBoundingBoxGeometryDescriptor` in `metal::src::acceleration::geometry::bounding_box`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `boundingBoxBuffer()` | `bounding_box_buffer()` |
| `boundingBoxCount()` | `bounding_box_count()` |
| `boundingBoxStride()` | `bounding_box_stride()` |
| `init()` | `new()` |
| `setBoundingBoxBuffer()` | `set_bounding_box_buffer()` |
| `setBoundingBoxCount()` | `set_bounding_box_count()` |
| `setBoundingBoxStride()` | `set_bounding_box_stride()` |

### AccelerationStructureBoundingBoxGeometryDescriptor

**Rust:** `AccelerationStructureBoundingBoxGeometryDescriptor` in `metal::src::acceleration::geometry::bounding_box`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `boundingBoxBuffer()` | `bounding_box_buffer()` |
| `boundingBoxBufferOffset()` | `bounding_box_buffer_offset()` |
| `boundingBoxCount()` | `bounding_box_count()` |
| `boundingBoxStride()` | `bounding_box_stride()` |
| `descriptor()` | `new()` |
| `init()` | `new()` |
| `setBoundingBoxBuffer()` | `set_bounding_box_buffer()` |
| `setBoundingBoxBufferOffset()` | `set_bounding_box_buffer_offset()` |
| `setBoundingBoxCount()` | `set_bounding_box_count()` |
| `setBoundingBoxStride()` | `set_bounding_box_stride()` |

### AccelerationStructureCommandEncoder

**Rust:** `AccelerationStructureCommandEncoder` in `metal::src::acceleration::encoder`

| C++ | Rust |
|-----|------|
| `buildAccelerationStructure()` | `build_acceleration_structure()` |
| `copyAccelerationStructure()` | `copy_acceleration_structure()` |
| `copyAndCompactAccelerationStructure()` | `copy_and_compact_acceleration_structure()` |
| `refitAccelerationStructure()` | `refit_acceleration_structure()` |
| `refitAccelerationStructure()` | `refit_acceleration_structure()` |
| `sampleCountersInBuffer()` | `sample_counters_in_buffer_ptr()` |
| `updateFence()` | `update_fence()` |
| `useHeap()` | `use_heap()` |
| `useHeaps()` | `use_heap()` |
| `useResource()` | `use_resource()` |
| `useResources()` | `use_resource()` |
| `waitForFence()` | `wait_for_fence()` |
| `writeCompactedAccelerationStructureSize()` | `write_compacted_acceleration_structure_size()` |
| `writeCompactedAccelerationStructureSize()` | `write_compacted_acceleration_structure_size()` |

### AccelerationStructureCurveGeometryDescriptor

**Rust:** `AccelerationStructureCurveGeometryDescriptor` in `metal::src::acceleration::curve`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `controlPointBuffer()` | `control_point_buffer()` |
| `controlPointCount()` | `control_point_count()` |
| `controlPointFormat()` | `control_point_format()` |
| `controlPointStride()` | `control_point_stride()` |
| `curveBasis()` | `curve_basis()` |
| `curveEndCaps()` | `curve_end_caps()` |
| `curveType()` | `curve_type()` |
| `indexBuffer()` | `index_buffer()` |
| `indexType()` | `index_type()` |
| `init()` | `new()` |
| `radiusBuffer()` | `radius_buffer()` |
| `radiusFormat()` | `radius_format()` |
| `radiusStride()` | `radius_stride()` |
| `segmentControlPointCount()` | `segment_control_point_count()` |
| `segmentCount()` | `segment_count()` |
| `setControlPointBuffer()` | `set_control_point_buffer()` |
| `setControlPointCount()` | `set_control_point_count()` |
| `setControlPointFormat()` | `set_control_point_format()` |
| `setControlPointStride()` | `set_control_point_stride()` |
| `setCurveBasis()` | `set_curve_basis()` |
| `setCurveEndCaps()` | `set_curve_end_caps()` |
| `setCurveType()` | `set_curve_type()` |
| `setIndexBuffer()` | `set_index_buffer()` |
| `setIndexType()` | `set_index_type()` |
| `setRadiusBuffer()` | `set_radius_buffer()` |
| `setRadiusFormat()` | `set_radius_format()` |
| `setRadiusStride()` | `set_radius_stride()` |
| `setSegmentControlPointCount()` | `set_segment_control_point_count()` |
| `setSegmentCount()` | `set_segment_count()` |

### AccelerationStructureCurveGeometryDescriptor

**Rust:** `AccelerationStructureCurveGeometryDescriptor` in `metal::src::acceleration::curve`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `controlPointBuffer()` | `control_point_buffer()` |
| `controlPointBufferOffset()` | `control_point_buffer_offset()` |
| `controlPointCount()` | `control_point_count()` |
| `controlPointFormat()` | `control_point_format()` |
| `controlPointStride()` | `control_point_stride()` |
| `curveBasis()` | `curve_basis()` |
| `curveEndCaps()` | `curve_end_caps()` |
| `curveType()` | `curve_type()` |
| `descriptor()` | `new()` |
| `indexBuffer()` | `index_buffer()` |
| `indexBufferOffset()` | `index_buffer_offset()` |
| `indexType()` | `index_type()` |
| `init()` | `new()` |
| `radiusBuffer()` | `radius_buffer()` |
| `radiusBufferOffset()` | `radius_buffer_offset()` |
| `radiusFormat()` | `radius_format()` |
| `radiusStride()` | `radius_stride()` |
| `segmentControlPointCount()` | `segment_control_point_count()` |
| `segmentCount()` | `segment_count()` |
| `setControlPointBuffer()` | `set_control_point_buffer()` |
| `setControlPointBufferOffset()` | `set_control_point_buffer_offset()` |
| `setControlPointCount()` | `set_control_point_count()` |
| `setControlPointFormat()` | `set_control_point_format()` |
| `setControlPointStride()` | `set_control_point_stride()` |
| `setCurveBasis()` | `set_curve_basis()` |
| `setCurveEndCaps()` | `set_curve_end_caps()` |
| `setCurveType()` | `set_curve_type()` |
| `setIndexBuffer()` | `set_index_buffer()` |
| `setIndexBufferOffset()` | `set_index_buffer_offset()` |
| `setIndexType()` | `set_index_type()` |
| `setRadiusBuffer()` | `set_radius_buffer()` |
| `setRadiusBufferOffset()` | `set_radius_buffer_offset()` |
| `setRadiusFormat()` | `set_radius_format()` |
| `setRadiusStride()` | `set_radius_stride()` |
| `setSegmentControlPointCount()` | `set_segment_control_point_count()` |
| `setSegmentCount()` | `set_segment_count()` |

### AccelerationStructureDescriptor

**Rust:** `AccelerationStructureDescriptor` in `metal::src::acceleration::descriptors`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |

### AccelerationStructureDescriptor

**Rust:** `AccelerationStructureDescriptor` in `metal::src::acceleration::descriptors`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `setUsage()` | `set_usage()` |
| `usage()` | `usage()` |

### AccelerationStructureGeometryDescriptor

**Rust:** `AccelerationStructureGeometryDescriptor` in `metal::src::acceleration::geometry::base`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `allowDuplicateIntersectionFunctionInvocation()` | `allow_duplicate_intersection_function_invocation()` |
| `init()` | `new()` |
| `intersectionFunctionTableOffset()` | `intersection_function_table_offset()` |
| `label()` | `label()` |
| `opaque()` | `opaque()` |
| `primitiveDataBuffer()` | `primitive_data_buffer()` |
| `primitiveDataElementSize()` | `primitive_data_element_size()` |
| `primitiveDataStride()` | `primitive_data_stride()` |
| `setAllowDuplicateIntersectionFunctionInvocation()` | `set_allow_duplicate_intersection_function_invocation()` |
| `setIntersectionFunctionTableOffset()` | `set_intersection_function_table_offset()` |
| `setLabel()` | `set_label()` |
| `setOpaque()` | `set_opaque()` |
| `setPrimitiveDataBuffer()` | `set_primitive_data_buffer()` |
| `setPrimitiveDataElementSize()` | `set_primitive_data_element_size()` |
| `setPrimitiveDataStride()` | `set_primitive_data_stride()` |

### AccelerationStructureGeometryDescriptor

**Rust:** `AccelerationStructureGeometryDescriptor` in `metal::src::acceleration::geometry::base`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `allowDuplicateIntersectionFunctionInvocation()` | `allow_duplicate_intersection_function_invocation()` |
| `init()` | `new()` |
| `intersectionFunctionTableOffset()` | `intersection_function_table_offset()` |
| `label()` | `label()` |
| `opaque()` | `opaque()` |
| `primitiveDataBuffer()` | `primitive_data_buffer()` |
| `primitiveDataBufferOffset()` | `primitive_data_buffer_offset()` |
| `primitiveDataElementSize()` | `primitive_data_element_size()` |
| `primitiveDataStride()` | `primitive_data_stride()` |
| `setAllowDuplicateIntersectionFunctionInvocation()` | `set_allow_duplicate_intersection_function_invocation()` |
| `setIntersectionFunctionTableOffset()` | `set_intersection_function_table_offset()` |
| `setLabel()` | `set_label()` |
| `setOpaque()` | `set_opaque()` |
| `setPrimitiveDataBuffer()` | `set_primitive_data_buffer()` |
| `setPrimitiveDataBufferOffset()` | `set_primitive_data_buffer_offset()` |
| `setPrimitiveDataElementSize()` | `set_primitive_data_element_size()` |
| `setPrimitiveDataStride()` | `set_primitive_data_stride()` |

### AccelerationStructureMotionBoundingBoxGeometryDescriptor

**Rust:** `AccelerationStructureMotionBoundingBoxGeometryDescriptor` in `metal::src::acceleration::motion_geometry::bounding_box`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `boundingBoxBuffers()` | `bounding_box_buffers()` |
| `boundingBoxCount()` | `bounding_box_count()` |
| `boundingBoxStride()` | `bounding_box_stride()` |
| `init()` | `new()` |
| `setBoundingBoxBuffers()` | `set_bounding_box_buffers()` |
| `setBoundingBoxCount()` | `set_bounding_box_count()` |
| `setBoundingBoxStride()` | `set_bounding_box_stride()` |

### AccelerationStructureMotionBoundingBoxGeometryDescriptor

**Rust:** `AccelerationStructureMotionBoundingBoxGeometryDescriptor` in `metal::src::acceleration::motion_geometry::bounding_box`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `boundingBoxBuffers()` | `bounding_box_buffers()` |
| `boundingBoxCount()` | `bounding_box_count()` |
| `boundingBoxStride()` | `bounding_box_stride()` |
| `descriptor()` | `new()` |
| `init()` | `new()` |
| `setBoundingBoxBuffers()` | `set_bounding_box_buffers()` |
| `setBoundingBoxCount()` | `set_bounding_box_count()` |
| `setBoundingBoxStride()` | `set_bounding_box_stride()` |

### AccelerationStructureMotionCurveGeometryDescriptor

**Rust:** `AccelerationStructureMotionCurveGeometryDescriptor` in `metal::src::acceleration::motion_geometry::curve`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `controlPointBuffers()` | `control_point_buffers()` |
| `controlPointCount()` | `control_point_count()` |
| `controlPointFormat()` | `control_point_format()` |
| `controlPointStride()` | `control_point_stride()` |
| `curveBasis()` | `curve_basis()` |
| `curveEndCaps()` | `curve_end_caps()` |
| `curveType()` | `curve_type()` |
| `indexBuffer()` | `index_buffer()` |
| `indexType()` | `index_type()` |
| `init()` | `new()` |
| `radiusBuffers()` | `radius_buffers()` |
| `radiusFormat()` | `radius_format()` |
| `radiusStride()` | `radius_stride()` |
| `segmentControlPointCount()` | `segment_control_point_count()` |
| `segmentCount()` | `segment_count()` |
| `setControlPointBuffers()` | `set_control_point_buffers()` |
| `setControlPointCount()` | `set_control_point_count()` |
| `setControlPointFormat()` | `set_control_point_format()` |
| `setControlPointStride()` | `set_control_point_stride()` |
| `setCurveBasis()` | `set_curve_basis()` |
| `setCurveEndCaps()` | `set_curve_end_caps()` |
| `setCurveType()` | `set_curve_type()` |
| `setIndexBuffer()` | `set_index_buffer()` |
| `setIndexType()` | `set_index_type()` |
| `setRadiusBuffers()` | `set_radius_buffers()` |
| `setRadiusFormat()` | `set_radius_format()` |
| `setRadiusStride()` | `set_radius_stride()` |
| `setSegmentControlPointCount()` | `set_segment_control_point_count()` |
| `setSegmentCount()` | `set_segment_count()` |

### AccelerationStructureMotionCurveGeometryDescriptor

**Rust:** `AccelerationStructureMotionCurveGeometryDescriptor` in `metal::src::acceleration::motion_geometry::curve`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `controlPointBuffers()` | `control_point_buffers()` |
| `controlPointCount()` | `control_point_count()` |
| `controlPointFormat()` | `control_point_format()` |
| `controlPointStride()` | `control_point_stride()` |
| `curveBasis()` | `curve_basis()` |
| `curveEndCaps()` | `curve_end_caps()` |
| `curveType()` | `curve_type()` |
| `descriptor()` | `new()` |
| `indexBuffer()` | `index_buffer()` |
| `indexBufferOffset()` | `index_buffer_offset()` |
| `indexType()` | `index_type()` |
| `init()` | `new()` |
| `radiusBuffers()` | `radius_buffers()` |
| `radiusFormat()` | `radius_format()` |
| `radiusStride()` | `radius_stride()` |
| `segmentControlPointCount()` | `segment_control_point_count()` |
| `segmentCount()` | `segment_count()` |
| `setControlPointBuffers()` | `set_control_point_buffers()` |
| `setControlPointCount()` | `set_control_point_count()` |
| `setControlPointFormat()` | `set_control_point_format()` |
| `setControlPointStride()` | `set_control_point_stride()` |
| `setCurveBasis()` | `set_curve_basis()` |
| `setCurveEndCaps()` | `set_curve_end_caps()` |
| `setCurveType()` | `set_curve_type()` |
| `setIndexBuffer()` | `set_index_buffer()` |
| `setIndexBufferOffset()` | `set_index_buffer_offset()` |
| `setIndexType()` | `set_index_type()` |
| `setRadiusBuffers()` | `set_radius_buffers()` |
| `setRadiusFormat()` | `set_radius_format()` |
| `setRadiusStride()` | `set_radius_stride()` |
| `setSegmentControlPointCount()` | `set_segment_control_point_count()` |
| `setSegmentCount()` | `set_segment_count()` |

### AccelerationStructureMotionTriangleGeometryDescriptor

**Rust:** `AccelerationStructureMotionTriangleGeometryDescriptor` in `metal::src::acceleration::motion_geometry::triangle`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `indexBuffer()` | `index_buffer()` |
| `indexType()` | `index_type()` |
| `init()` | `new()` |
| `setIndexBuffer()` | `set_index_buffer()` |
| `setIndexType()` | `set_index_type()` |
| `setTransformationMatrixBuffer()` | `set_transformation_matrix_buffer()` |
| `setTransformationMatrixLayout()` | `set_transformation_matrix_layout()` |
| `setTriangleCount()` | `set_triangle_count()` |
| `setVertexBuffers()` | `set_vertex_buffers()` |
| `setVertexFormat()` | `set_vertex_format()` |
| `setVertexStride()` | `set_vertex_stride()` |
| `transformationMatrixBuffer()` | `transformation_matrix_buffer()` |
| `transformationMatrixLayout()` | `transformation_matrix_layout()` |
| `triangleCount()` | `triangle_count()` |
| `vertexBuffers()` | `vertex_buffers()` |
| `vertexFormat()` | `vertex_format()` |
| `vertexStride()` | `vertex_stride()` |

### AccelerationStructureMotionTriangleGeometryDescriptor

**Rust:** `AccelerationStructureMotionTriangleGeometryDescriptor` in `metal::src::acceleration::motion_geometry::triangle`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `descriptor()` | `new()` |
| `indexBuffer()` | `index_buffer()` |
| `indexBufferOffset()` | `index_buffer_offset()` |
| `indexType()` | `index_type()` |
| `init()` | `new()` |
| `setIndexBuffer()` | `set_index_buffer()` |
| `setIndexBufferOffset()` | `set_index_buffer_offset()` |
| `setIndexType()` | `set_index_type()` |
| `setTransformationMatrixBuffer()` | `set_transformation_matrix_buffer()` |
| `setTransformationMatrixBufferOffset()` | `set_transformation_matrix_buffer_offset()` |
| `setTransformationMatrixLayout()` | `set_transformation_matrix_layout()` |
| `setTriangleCount()` | `set_triangle_count()` |
| `setVertexBuffers()` | `set_vertex_buffers()` |
| `setVertexFormat()` | `set_vertex_format()` |
| `setVertexStride()` | `set_vertex_stride()` |
| `transformationMatrixBuffer()` | `transformation_matrix_buffer()` |
| `transformationMatrixBufferOffset()` | `transformation_matrix_buffer_offset()` |
| `transformationMatrixLayout()` | `transformation_matrix_layout()` |
| `triangleCount()` | `triangle_count()` |
| `vertexBuffers()` | `vertex_buffers()` |
| `vertexFormat()` | `vertex_format()` |
| `vertexStride()` | `vertex_stride()` |

### AccelerationStructurePassDescriptor

**Rust:** `AccelerationStructurePassDescriptor` in `metal::src::acceleration::pass`

| C++ | Rust |
|-----|------|
| `accelerationStructurePassDescriptor()` | `new()` |
| `alloc()` | `new()` |
| `init()` | `new()` |
| `sampleBufferAttachments()` | `sample_buffer_attachments()` |

### AccelerationStructurePassSampleBufferAttachmentDescriptor

**Rust:** `AccelerationStructurePassSampleBufferAttachmentDescriptor` in `metal::src::acceleration::pass`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `endOfEncoderSampleIndex()` | `end_of_encoder_sample_index()` |
| `init()` | `new()` |
| `sampleBuffer()` | `sample_buffer_ptr()` |
| `setEndOfEncoderSampleIndex()` | `set_end_of_encoder_sample_index()` |
| `setSampleBuffer()` | `set_sample_buffer_ptr()` |
| `setStartOfEncoderSampleIndex()` | `set_start_of_encoder_sample_index()` |
| `startOfEncoderSampleIndex()` | `start_of_encoder_sample_index()` |

### AccelerationStructurePassSampleBufferAttachmentDescriptorArray

**Rust:** `AccelerationStructurePassSampleBufferAttachmentDescriptorArray` in `metal::src::acceleration::pass`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `object()` | `object()` |
| `setObject()` | `set_object()` |

### AccelerationStructureTriangleGeometryDescriptor

**Rust:** `AccelerationStructureTriangleGeometryDescriptor` in `metal::src::acceleration::geometry::triangle`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `indexBuffer()` | `index_buffer()` |
| `indexType()` | `index_type()` |
| `init()` | `new()` |
| `setIndexBuffer()` | `set_index_buffer()` |
| `setIndexType()` | `set_index_type()` |
| `setTransformationMatrixBuffer()` | `set_transformation_matrix_buffer()` |
| `setTransformationMatrixLayout()` | `set_transformation_matrix_layout()` |
| `setTriangleCount()` | `set_triangle_count()` |
| `setVertexBuffer()` | `set_vertex_buffer()` |
| `setVertexFormat()` | `set_vertex_format()` |
| `setVertexStride()` | `set_vertex_stride()` |
| `transformationMatrixBuffer()` | `transformation_matrix_buffer()` |
| `transformationMatrixLayout()` | `transformation_matrix_layout()` |
| `triangleCount()` | `triangle_count()` |
| `vertexBuffer()` | `vertex_buffer()` |
| `vertexFormat()` | `vertex_format()` |
| `vertexStride()` | `vertex_stride()` |

### AccelerationStructureTriangleGeometryDescriptor

**Rust:** `AccelerationStructureTriangleGeometryDescriptor` in `metal::src::acceleration::geometry::triangle`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `descriptor()` | `new()` |
| `indexBuffer()` | `index_buffer()` |
| `indexBufferOffset()` | `index_buffer_offset()` |
| `indexType()` | `index_type()` |
| `init()` | `new()` |
| `setIndexBuffer()` | `set_index_buffer()` |
| `setIndexBufferOffset()` | `set_index_buffer_offset()` |
| `setIndexType()` | `set_index_type()` |
| `setTransformationMatrixBuffer()` | `set_transformation_matrix_buffer()` |
| `setTransformationMatrixBufferOffset()` | `set_transformation_matrix_buffer_offset()` |
| `setTransformationMatrixLayout()` | `set_transformation_matrix_layout()` |
| `setTriangleCount()` | `set_triangle_count()` |
| `setVertexBuffer()` | `set_vertex_buffer()` |
| `setVertexBufferOffset()` | `set_vertex_buffer_offset()` |
| `setVertexFormat()` | `set_vertex_format()` |
| `setVertexStride()` | `set_vertex_stride()` |
| `transformationMatrixBuffer()` | `transformation_matrix_buffer()` |
| `transformationMatrixBufferOffset()` | `transformation_matrix_buffer_offset()` |
| `transformationMatrixLayout()` | `transformation_matrix_layout()` |
| `triangleCount()` | `triangle_count()` |
| `vertexBuffer()` | `vertex_buffer()` |
| `vertexBufferOffset()` | `vertex_buffer_offset()` |
| `vertexFormat()` | `vertex_format()` |
| `vertexStride()` | `vertex_stride()` |

### Architecture

**Rust:** `Architecture` in `metal::src::device::architecture`

| C++ | Rust |
|-----|------|
| `alloc()` | `alloc()` |
| `init()` | `init()` |
| `name()` | `name()` |

### Archive

**Rust:** `Archive` in `metal::src::mtl4::archive`

| C++ | Rust |
|-----|------|
| `label()` | `label()` |
| `newBinaryFunction()` | `new_binary_function()` |
| `newComputePipelineState()` | `new_compute_pipeline_state()` |
| `newComputePipelineState()` | `new_compute_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `setLabel()` | `set_label()` |

### Argument

**Rust:** `Argument` in `metal::src::argument::argument`

| C++ | Rust |
|-----|------|
| `access()` | `access()` |
| `active()` | `is_active()` |
| `alloc()` | `n/a()` |
| `arrayLength()` | `array_length()` |
| `bufferAlignment()` | `buffer_alignment()` |
| `bufferDataSize()` | `buffer_data_size()` |
| `bufferDataType()` | `buffer_data_type()` |
| `bufferPointerType()` | `buffer_pointer_type()` |
| `bufferStructType()` | `buffer_struct_type()` |
| `index()` | `index()` |
| `init()` | `n/a()` |
| `isActive()` | `is_active()` |
| `isDepthTexture()` | `is_depth_texture()` |
| `name()` | `name()` |
| `textureDataType()` | `texture_data_type()` |
| `textureType()` | `texture_type()` |
| `threadgroupMemoryAlignment()` | `threadgroup_memory_alignment()` |
| `threadgroupMemoryDataSize()` | `threadgroup_memory_data_size()` |
| `type()` | `argument_type()` |

### ArgumentDescriptor

**Rust:** `ArgumentDescriptor` in `metal::src::argument_descriptor`

| C++ | Rust |
|-----|------|
| `access()` | `access()` |
| `alloc()` | `new()` |
| `argumentDescriptor()` | `new()` |
| `arrayLength()` | `array_length()` |
| `constantBlockAlignment()` | `constant_block_alignment()` |
| `dataType()` | `data_type()` |
| `index()` | `index()` |
| `init()` | `new()` |
| `setAccess()` | `set_access()` |
| `setArrayLength()` | `set_array_length()` |
| `setConstantBlockAlignment()` | `set_constant_block_alignment()` |
| `setDataType()` | `set_data_type()` |
| `setIndex()` | `set_index()` |
| `setTextureType()` | `set_texture_type()` |
| `textureType()` | `texture_type()` |

### ArgumentEncoder

**Rust:** `ArgumentEncoder` in `metal::src::argument::encoder`

| C++ | Rust |
|-----|------|
| `alignment()` | `alignment()` |
| `constantData()` | `constant_data()` |
| `device()` | `device()` |
| `encodedLength()` | `encoded_length()` |
| `label()` | `label()` |
| `newArgumentEncoder()` | `new_argument_encoder()` |
| `setAccelerationStructure()` | `set_acceleration_structure()` |
| `setArgumentBuffer()` | `set_argument_buffer()` |
| `setArgumentBuffer()` | `set_argument_buffer()` |
| `setBuffer()` | `set_buffer()` |
| `setBuffers()` | `set_buffer()` |
| `setComputePipelineState()` | `set_compute_pipeline_state()` |
| `setComputePipelineStates()` | `set_compute_pipeline_state()` |
| `setDepthStencilState()` | `set_depth_stencil_state()` |
| `setDepthStencilStates()` | `set_depth_stencil_state()` |
| `setIndirectCommandBuffer()` | `set_indirect_command_buffer_ptr()` |
| `setIndirectCommandBuffers()` | `set_indirect_command_buffer_ptr()` |
| `setIntersectionFunctionTable()` | `set_intersection_function_table_ptr()` |
| `setIntersectionFunctionTables()` | `set_intersection_function_table_ptr()` |
| `setLabel()` | `set_label()` |
| `setRenderPipelineState()` | `set_render_pipeline_state()` |
| `setRenderPipelineStates()` | `set_render_pipeline_state()` |
| `setSamplerState()` | `set_sampler_state()` |
| `setSamplerStates()` | `set_sampler_state()` |
| `setTexture()` | `set_texture()` |
| `setTextures()` | `set_texture()` |
| `setVisibleFunctionTable()` | `set_visible_function_table_ptr()` |
| `setVisibleFunctionTables()` | `set_visible_function_table_ptr()` |

### ArgumentTable

**Rust:** `ArgumentTable` in `metal::src::mtl4::argument_table`

| C++ | Rust |
|-----|------|
| `device()` | `device()` |
| `label()` | `label()` |
| `setAddress()` | `set_address()` |
| `setAddress()` | `set_address()` |
| `setResource()` | `set_resource()` |
| `setSamplerState()` | `set_sampler_state()` |
| `setTexture()` | `set_texture()` |

### ArgumentTableDescriptor

**Rust:** `ArgumentTableDescriptor` in `metal::src::mtl4::argument_table`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `initializeBindings()` | `initialize_bindings()` |
| `label()` | `label()` |
| `maxBufferBindCount()` | `max_buffer_bind_count()` |
| `maxSamplerStateBindCount()` | `max_sampler_state_bind_count()` |
| `maxTextureBindCount()` | `max_texture_bind_count()` |
| `setInitializeBindings()` | `set_initialize_bindings()` |
| `setLabel()` | `set_label()` |
| `setMaxBufferBindCount()` | `set_max_buffer_bind_count()` |
| `setMaxSamplerStateBindCount()` | `set_max_sampler_state_bind_count()` |
| `setMaxTextureBindCount()` | `set_max_texture_bind_count()` |
| `setSupportAttributeStrides()` | `set_support_attribute_strides()` |
| `supportAttributeStrides()` | `support_attribute_strides()` |

### ArrayType

**Rust:** `ArrayType` in `metal::src::argument::array_type`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `argumentIndexStride()` | `argument_index_stride()` |
| `arrayLength()` | `array_length()` |
| `elementArrayType()` | `element_array_type()` |
| `elementPointerType()` | `element_pointer_type()` |
| `elementStructType()` | `element_struct_type()` |
| `elementTensorReferenceType()` | `element_tensor_reference_type()` |
| `elementTextureReferenceType()` | `element_texture_reference_type()` |
| `elementType()` | `element_type()` |
| `init()` | `n/a()` |
| `stride()` | `stride()` |

### Attribute

**Rust:** `Attribute` in `metal::src::library::attribute`

| C++ | Rust |
|-----|------|
| `active()` | `is_active()` |
| `alloc()` | `new()` |
| `attributeIndex()` | `attribute_index()` |
| `attributeType()` | `attribute_type()` |
| `init()` | `new()` |
| `isActive()` | `is_active()` |
| `isPatchControlPointData()` | `is_patch_control_point_data()` |
| `isPatchData()` | `is_patch_data()` |
| `name()` | `name()` |
| `patchControlPointData()` | `is_patch_control_point_data()` |
| `patchData()` | `is_patch_data()` |

### AttributeDescriptor

**Rust:** `AttributeDescriptor` in `metal::src::stage_input_output`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `bufferIndex()` | `buffer_index()` |
| `format()` | `format()` |
| `init()` | `new()` |
| `offset()` | `offset()` |
| `setBufferIndex()` | `set_buffer_index()` |
| `setFormat()` | `set_format()` |
| `setOffset()` | `set_offset()` |

### AttributeDescriptorArray

**Rust:** `AttributeDescriptorArray` in `metal::src::stage_input_output`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `init()` | `n/a()` |
| `object()` | `object_at()` |
| `setObject()` | `set_object_at()` |

### BinaryArchive

**Rust:** `BinaryArchive` in `metal::src::binary_archive`

| C++ | Rust |
|-----|------|
| `addComputePipelineFunctions()` | `add_compute_pipeline_functions()` |
| `addFunction()` | `add_function()` |
| `addLibrary()` | `add_library_ptr()` |
| `addMeshRenderPipelineFunctions()` | `add_mesh_render_pipeline_functions_ptr()` |
| `addRenderPipelineFunctions()` | `add_render_pipeline_functions()` |
| `addTileRenderPipelineFunctions()` | `add_tile_render_pipeline_functions_ptr()` |
| `device()` | `device()` |
| `label()` | `label()` |
| `serializeToURL()` | `serialize_to_url()` |
| `setLabel()` | `set_label()` |

### BinaryArchiveDescriptor

**Rust:** `BinaryArchiveDescriptor` in `metal::src::binary_archive`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `setUrl()` | `set_url()` |
| `url()` | `url()` |

### BinaryFunction

**Rust:** `BinaryFunction` in `metal::src::mtl4::binary_function`

| C++ | Rust |
|-----|------|
| `functionType()` | `function_type()` |
| `name()` | `name()` |

### BinaryFunctionDescriptor

**Rust:** `BinaryFunctionDescriptor` in `metal::src::mtl4::binary_function`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `functionDescriptor()` | `function_descriptor()` |
| `init()` | `new()` |
| `name()` | `name()` |
| `options()` | `options()` |
| `setFunctionDescriptor()` | `set_function_descriptor()` |
| `setName()` | `set_name()` |
| `setOptions()` | `set_options()` |

### Binding

**Rust:** `Binding` in `metal::src::argument::binding`

| C++ | Rust |
|-----|------|
| `access()` | `access()` |
| `argument()` | `argument()` |
| `index()` | `index()` |
| `isArgument()` | `is_argument()` |
| `isUsed()` | `is_used()` |
| `name()` | `name()` |
| `type()` | `binding_type()` |
| `used()` | `used()` |

### BlitCommandEncoder

**Rust:** `BlitCommandEncoder` in `metal::src::encoder::blit_encoder`

| C++ | Rust |
|-----|------|
| `copyFromBuffer()` | `copy_from_buffer_to_texture()` |
| `copyFromBuffer()` | `copy_from_buffer_to_texture()` |
| `copyFromBuffer()` | `copy_from_buffer_to_buffer()` |
| `copyFromTensor()` | `copy_from_tensor_ptr()` |
| `copyFromTexture()` | `copy_from_buffer_to_texture()` |
| `copyFromTexture()` | `copy_from_buffer_to_texture()` |
| `copyFromTexture()` | `copy_from_buffer_to_texture()` |
| `copyFromTexture()` | `copy_from_tensor_ptr()` |
| `copyFromTexture()` | `copy_from_texture_to_texture()` |
| `copyIndirectCommandBuffer()` | `copy_indirect_command_buffer_ptr()` |
| `fillBuffer()` | `fill_buffer()` |
| `generateMipmaps()` | `generate_mipmaps()` |
| `getTextureAccessCounters()` | `get_texture_access_counters()` |
| `optimizeContentsForCPUAccess()` | `optimize_contents_for_cpu_access()` |
| `optimizeContentsForCPUAccess()` | `optimize_contents_for_cpu_access()` |
| `optimizeContentsForGPUAccess()` | `optimize_contents_for_gpu_access()` |
| `optimizeContentsForGPUAccess()` | `optimize_contents_for_gpu_access()` |
| `optimizeIndirectCommandBuffer()` | `optimize_indirect_command_buffer_ptr()` |
| `resetCommandsInBuffer()` | `reset_commands_in_buffer_ptr()` |
| `resetTextureAccessCounters()` | `reset_texture_access_counters()` |
| `resolveCounters()` | `resolve_counters_ptr()` |
| `sampleCountersInBuffer()` | `sample_counters_in_buffer_ptr()` |
| `synchronizeResource()` | `synchronize_resource_ptr()` |
| `synchronizeTexture()` | `synchronize_texture()` |
| `updateFence()` | `update_fence()` |
| `waitForFence()` | `wait_for_fence()` |

### BlitPassDescriptor

**Rust:** `BlitPassDescriptor` in `metal::src::pass::blit_pass`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `blitPassDescriptor()` | `new()` |
| `init()` | `new()` |
| `sampleBufferAttachments()` | `sample_buffer_attachments()` |

### BlitPassSampleBufferAttachmentDescriptor

**Rust:** `BlitPassSampleBufferAttachmentDescriptor` in `metal::src::pass::blit_sample_buffer`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `endOfEncoderSampleIndex()` | `end_of_encoder_sample_index()` |
| `init()` | `new()` |
| `sampleBuffer()` | `sample_buffer()` |
| `setEndOfEncoderSampleIndex()` | `set_end_of_encoder_sample_index()` |
| `setSampleBuffer()` | `set_sample_buffer()` |
| `setStartOfEncoderSampleIndex()` | `set_start_of_encoder_sample_index()` |
| `startOfEncoderSampleIndex()` | `start_of_encoder_sample_index()` |

### BlitPassSampleBufferAttachmentDescriptorArray

**Rust:** `BlitPassSampleBufferAttachmentDescriptorArray` in `metal::src::pass::blit_sample_buffer`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `object()` | `object()` |
| `setObject()` | `set_object()` |

### Buffer

**Rust:** `Buffer` in `metal::src::buffer`

| C++ | Rust |
|-----|------|
| `addDebugMarker()` | `add_debug_marker()` |
| `contents()` | `contents()` |
| `didModifyRange()` | `did_modify_range()` |
| `gpuAddress()` | `gpu_address()` |
| `length()` | `length()` |
| `newRemoteBufferViewForDevice()` | `new_remote_buffer_view_for_device()` |
| `newTensor()` | `new_tensor()` |
| `newTexture()` | `new_texture()` |
| `remoteStorageBuffer()` | `remote_storage_buffer()` |
| `removeAllDebugMarkers()` | `remove_all_debug_markers()` |
| `sparseBufferTier()` | `sparse_buffer_tier()` |

### BufferBinding

**Rust:** `BufferBinding` in `metal::src::argument::buffer_binding`

| C++ | Rust |
|-----|------|
| `bufferAlignment()` | `buffer_alignment()` |
| `bufferDataSize()` | `buffer_data_size()` |
| `bufferDataType()` | `buffer_data_type()` |
| `bufferPointerType()` | `buffer_pointer_type()` |
| `bufferStructType()` | `buffer_struct_type()` |

### BufferLayoutDescriptor

**Rust:** `BufferLayoutDescriptor` in `metal::src::stage_input_output`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `setStepFunction()` | `set_step_function()` |
| `setStepRate()` | `set_step_rate()` |
| `setStride()` | `set_stride()` |
| `stepFunction()` | `step_function()` |
| `stepRate()` | `step_rate()` |
| `stride()` | `stride()` |

### BufferLayoutDescriptorArray

**Rust:** `BufferLayoutDescriptorArray` in `metal::src::stage_input_output`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `init()` | `n/a()` |
| `object()` | `object_at()` |
| `setObject()` | `set_object_at()` |

### CaptureDescriptor

**Rust:** `CaptureDescriptor` in `metal::src::capture`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `captureObject()` | `capture_object()` |
| `destination()` | `destination()` |
| `init()` | `new()` |
| `outputURL()` | `output_url()` |
| `setCaptureObject()` | `set_capture_object()` |
| `setDestination()` | `set_destination()` |
| `setOutputURL()` | `set_output_url()` |

### CaptureManager

**Rust:** `CaptureManager` in `metal::src::capture`

| C++ | Rust |
|-----|------|
| `alloc()` | `new_capture_scope_with_device()` |
| `defaultCaptureScope()` | `default_capture_scope()` |
| `init()` | `new_capture_scope_with_device()` |
| `isCapturing()` | `is_capturing()` |
| `newCaptureScope()` | `default_capture_scope()` |
| `newCaptureScope()` | `default_capture_scope()` |
| `newCaptureScope()` | `default_capture_scope()` |
| `setDefaultCaptureScope()` | `set_default_capture_scope()` |
| `sharedCaptureManager()` | `start_capture()` |
| `startCapture()` | `start_capture()` |
| `startCapture()` | `start_capture()` |
| `startCapture()` | `start_capture()` |
| `startCapture()` | `start_capture()` |
| `stopCapture()` | `stop_capture()` |
| `supportsDestination()` | `supports_destination()` |

### CaptureScope

**Rust:** `CaptureScope` in `metal::src::capture`

| C++ | Rust |
|-----|------|
| `beginScope()` | `begin_scope()` |
| `commandQueue()` | `command_queue()` |
| `device()` | `device()` |
| `endScope()` | `end_scope()` |
| `label()` | `label()` |
| `setLabel()` | `set_label()` |

### CommandAllocator

**Rust:** `CommandAllocator` in `metal::src::mtl4::command_allocator`

| C++ | Rust |
|-----|------|
| `allocatedSize()` | `allocated_size()` |
| `device()` | `device()` |
| `label()` | `label()` |
| `reset()` | `reset()` |

### CommandAllocatorDescriptor

**Rust:** `CommandAllocatorDescriptor` in `metal::src::mtl4::command_allocator`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `label()` | `label()` |
| `setLabel()` | `set_label()` |

### CommandBuffer

**Rust:** `CommandBuffer` in `metal::src::command_buffer`

| C++ | Rust |
|-----|------|
| `beginCommandBuffer()` | `begin_command_buffer()` |
| `beginCommandBuffer()` | `begin_command_buffer()` |
| `computeCommandEncoder()` | `compute_command_encoder()` |
| `device()` | `device()` |
| `endCommandBuffer()` | `end_command_buffer()` |
| `label()` | `label()` |
| `machineLearningCommandEncoder()` | `machine_learning_command_encoder()` |
| `popDebugGroup()` | `pop_debug_group()` |
| `pushDebugGroup()` | `push_debug_group()` |
| `renderCommandEncoder()` | `render_command_encoder()` |
| `renderCommandEncoder()` | `render_command_encoder()` |
| `resolveCounterHeap()` | `resolve_counter_heap()` |
| `setLabel()` | `set_label()` |
| `useResidencySet()` | `use_residency_set()` |
| `useResidencySets()` | `use_residency_sets()` |
| `writeTimestampIntoHeap()` | `write_timestamp_into_heap()` |

### CommandBuffer

**Rust:** `CommandBuffer` in `metal::src::command_buffer`

| C++ | Rust |
|-----|------|
| `GPUEndTime()` | `gpu_end_time()` |
| `GPUStartTime()` | `gpu_start_time()` |
| `accelerationStructureCommandEncoder()` | `acceleration_structure_command_encoder()` |
| `accelerationStructureCommandEncoder()` | `acceleration_structure_command_encoder()` |
| `addCompletedHandler()` | `add_completed_handler()` |
| `addCompletedHandler()` | `add_completed_handler()` |
| `addScheduledHandler()` | `add_scheduled_handler()` |
| `addScheduledHandler()` | `add_scheduled_handler()` |
| `blitCommandEncoder()` | `blit_command_encoder()` |
| `blitCommandEncoder()` | `blit_command_encoder()` |
| `commandQueue()` | `command_queue()` |
| `commit()` | `commit()` |
| `computeCommandEncoder()` | `compute_command_encoder()` |
| `computeCommandEncoder()` | `compute_command_encoder()` |
| `computeCommandEncoder()` | `compute_command_encoder()` |
| `device()` | `device()` |
| `encodeSignalEvent()` | `encode_signal_event()` |
| `encodeWait()` | `encode_wait_for_event()` |
| `enqueue()` | `enqueue()` |
| `error()` | `error()` |
| `errorOptions()` | `error_options()` |
| `kernelEndTime()` | `kernel_end_time()` |
| `kernelStartTime()` | `kernel_start_time()` |
| `label()` | `label()` |
| `logs()` | `logs()` |
| `parallelRenderCommandEncoder()` | `parallel_render_command_encoder()` |
| `popDebugGroup()` | `pop_debug_group()` |
| `presentDrawable()` | `present_drawable()` |
| `presentDrawableAfterMinimumDuration()` | `present_drawable_after_minimum_duration()` |
| `presentDrawableAtTime()` | `present_drawable_at_time()` |
| `pushDebugGroup()` | `push_debug_group()` |
| `renderCommandEncoder()` | `render_command_encoder()` |
| `resourceStateCommandEncoder()` | `resource_state_command_encoder()` |
| `resourceStateCommandEncoder()` | `resource_state_command_encoder()` |
| `retainedReferences()` | `retained_references()` |
| `setLabel()` | `set_label()` |
| `status()` | `status()` |
| `useResidencySet()` | `use_residency_set()` |
| `useResidencySets()` | `use_residency_sets()` |
| `waitUntilCompleted()` | `wait_until_completed()` |
| `waitUntilScheduled()` | `wait_until_scheduled()` |

### CommandBufferDescriptor

**Rust:** `CommandBufferDescriptor` in `metal::src::command_buffer`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `errorOptions()` | `error_options()` |
| `init()` | `new()` |
| `logState()` | `log_state()` |
| `retainedReferences()` | `retained_references()` |
| `setErrorOptions()` | `set_error_options()` |
| `setLogState()` | `set_log_state()` |
| `setRetainedReferences()` | `set_retained_references()` |

### CommandBufferEncoderInfo

**Rust:** `CommandBufferEncoderInfo` in `metal::src::command_buffer_encoder_info`

| C++ | Rust |
|-----|------|
| `debugSignposts()` | `debug_signposts_ptr()` |
| `errorState()` | `error_state()` |
| `label()` | `label()` |

### CommandBufferOptions

**Rust:** `CommandBufferOptions` in `metal::src::mtl4::command_buffer`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `logState()` | `log_state()` |
| `setLogState()` | `set_log_state()` |

### CommandQueue

**Rust:** `CommandQueue` in `metal::src::command_queue`

| C++ | Rust |
|-----|------|
| `addResidencySet()` | `add_residency_set()` |
| `addResidencySets()` | `add_residency_sets()` |
| `commit()` | `commit()` |
| `commit()` | `commit()` |
| `copyBufferMappingsFromBuffer()` | `copy_buffer_mappings_from_buffer()` |
| `copyTextureMappingsFromTexture()` | `copy_texture_mappings_from_texture()` |
| `device()` | `device()` |
| `label()` | `label()` |
| `removeResidencySet()` | `remove_residency_set()` |
| `removeResidencySets()` | `remove_residency_sets()` |
| `signalDrawable()` | `signal_drawable()` |
| `signalEvent()` | `signal_event()` |
| `updateBufferMappings()` | `update_buffer_mappings()` |
| `updateTextureMappings()` | `update_texture_mappings()` |
| `wait()` | `wait_for_event()` |
| `wait()` | `wait_for_event()` |

### CommandQueue

**Rust:** `CommandQueue` in `metal::src::command_queue`

| C++ | Rust |
|-----|------|
| `addResidencySet()` | `add_residency_set()` |
| `addResidencySets()` | `add_residency_sets()` |
| `commandBuffer()` | `command_buffer()` |
| `commandBuffer()` | `command_buffer()` |
| `commandBufferWithUnretainedReferences()` | `command_buffer_with_unretained_references()` |
| `device()` | `device()` |
| `insertDebugCaptureBoundary()` | `insert_debug_capture_boundary()` |
| `label()` | `label()` |
| `removeResidencySet()` | `remove_residency_set()` |
| `removeResidencySets()` | `remove_residency_sets()` |
| `setLabel()` | `set_label()` |

### CommandQueueDescriptor

**Rust:** `CommandQueueDescriptor` in `metal::src::command_queue`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `feedbackQueue()` | `feedback_queue()` |
| `init()` | `new()` |
| `label()` | `label()` |
| `setFeedbackQueue()` | `set_feedback_queue()` |
| `setLabel()` | `set_label()` |

### CommandQueueDescriptor

**Rust:** `CommandQueueDescriptor` in `metal::src::command_queue`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `logState()` | `log_state()` |
| `maxCommandBufferCount()` | `max_command_buffer_count()` |
| `setLogState()` | `set_log_state()` |
| `setMaxCommandBufferCount()` | `set_max_command_buffer_count()` |

### CommitFeedback

**Rust:** `CommitFeedback` in `metal::src::mtl4::commit_feedback`

| C++ | Rust |
|-----|------|
| `GPUEndTime()` | `gpu_end_time()` |
| `GPUStartTime()` | `gpu_start_time()` |
| `error()` | `error()` |

### CommitOptions

**Rust:** `CommitOptions` in `metal::src::mtl4::command_queue`

| C++ | Rust |
|-----|------|
| `addFeedbackHandler()` | `add_feedback_handler()` |
| `addFeedbackHandler()` | `add_feedback_handler()` |
| `alloc()` | `new()` |
| `init()` | `new()` |

### CompileOptions

**Rust:** `CompileOptions` in `metal::src::library::compile_options`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `allowReferencingUndefinedSymbols()` | `allow_referencing_undefined_symbols()` |
| `compileSymbolVisibility()` | `compile_symbol_visibility()` |
| `enableLogging()` | `enable_logging()` |
| `fastMathEnabled()` | `fast_math_enabled()` |
| `init()` | `new()` |
| `installName()` | `install_name()` |
| `languageVersion()` | `language_version()` |
| `libraries()` | `library_type()` |
| `libraryType()` | `library_type()` |
| `mathFloatingPointFunctions()` | `math_floating_point_functions()` |
| `mathMode()` | `math_mode()` |
| `maxTotalThreadsPerThreadgroup()` | `max_total_threads_per_threadgroup()` |
| `optimizationLevel()` | `optimization_level()` |
| `preprocessorMacros()` | `preprocessor_macros_raw()` |
| `preserveInvariance()` | `preserve_invariance()` |
| `requiredThreadsPerThreadgroup()` | `required_threads_per_threadgroup()` |
| `setAllowReferencingUndefinedSymbols()` | `set_allow_referencing_undefined_symbols()` |
| `setCompileSymbolVisibility()` | `set_compile_symbol_visibility()` |
| `setEnableLogging()` | `set_enable_logging()` |
| `setFastMathEnabled()` | `set_fast_math_enabled()` |
| `setInstallName()` | `set_install_name()` |
| `setLanguageVersion()` | `set_language_version()` |
| `setLibraries()` | `set_library_type()` |
| `setLibraryType()` | `set_library_type()` |
| `setMathFloatingPointFunctions()` | `set_math_floating_point_functions()` |
| `setMathMode()` | `set_math_mode()` |
| `setMaxTotalThreadsPerThreadgroup()` | `set_max_total_threads_per_threadgroup()` |
| `setOptimizationLevel()` | `set_optimization_level()` |
| `setPreprocessorMacros()` | `set_preprocessor_macros_raw()` |
| `setPreserveInvariance()` | `set_preserve_invariance()` |
| `setRequiredThreadsPerThreadgroup()` | `set_required_threads_per_threadgroup()` |

### Compiler

**Rust:** `Compiler` in `metal::src::mtl4::compiler::compiler`

| C++ | Rust |
|-----|------|
| `device()` | `device()` |
| `label()` | `label()` |
| `newBinaryFunction()` | `new_binary_function()` |
| `newBinaryFunction()` | `new_binary_function()` |
| `newComputePipelineState()` | `new_compute_pipeline_state()` |
| `newComputePipelineState()` | `new_compute_pipeline_state()` |
| `newComputePipelineState()` | `new_compute_pipeline_state()` |
| `newComputePipelineState()` | `new_compute_pipeline_state()` |
| `newComputePipelineState()` | `new_compute_pipeline_state()` |
| `newDynamicLibrary()` | `new_dynamic_library_from_url()` |
| `newDynamicLibrary()` | `new_dynamic_library_from_url()` |
| `newDynamicLibrary()` | `new_dynamic_library_from_url()` |
| `newDynamicLibrary()` | `new_dynamic_library_from_url()` |
| `newDynamicLibrary()` | `new_dynamic_library_from_url()` |
| `newDynamicLibrary()` | `new_dynamic_library_from_url()` |
| `newLibrary()` | `new_library()` |
| `newLibrary()` | `new_library()` |
| `newLibrary()` | `new_library()` |
| `newMachineLearningPipelineState()` | `new_machine_learning_pipeline_state()` |
| `newMachineLearningPipelineState()` | `new_machine_learning_pipeline_state()` |
| `newMachineLearningPipelineState()` | `new_machine_learning_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newRenderPipelineStateBySpecialization()` | `new_render_pipeline_state_by_specialization()` |
| `newRenderPipelineStateBySpecialization()` | `new_render_pipeline_state_by_specialization()` |
| `newRenderPipelineStateBySpecialization()` | `new_render_pipeline_state_by_specialization()` |
| `pipelineDataSetSerializer()` | `pipeline_data_set_serializer()` |

### CompilerDescriptor

**Rust:** `CompilerDescriptor` in `metal::src::mtl4::compiler::descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `label()` | `label()` |
| `pipelineDataSetSerializer()` | `pipeline_data_set_serializer()` |
| `setLabel()` | `set_label()` |
| `setPipelineDataSetSerializer()` | `set_pipeline_data_set_serializer()` |

### CompilerTask

**Rust:** `CompilerTask` in `metal::src::mtl4::compiler_task`

| C++ | Rust |
|-----|------|
| `compiler()` | `compiler_raw()` |
| `status()` | `status()` |
| `waitUntilCompleted()` | `wait_until_completed()` |

### CompilerTaskOptions

**Rust:** `CompilerTaskOptions` in `metal::src::mtl4::compiler::task_options`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `lookupArchives()` | `lookup_archives_raw()` |
| `setLookupArchives()` | `set_lookup_archives_raw()` |

### ComputeCommandEncoder

**Rust:** `ComputeCommandEncoder` in `metal::src::encoder::compute_encoder`

| C++ | Rust |
|-----|------|
| `buildAccelerationStructure()` | `build_acceleration_structure()` |
| `copyAccelerationStructure()` | `copy_acceleration_structure()` |
| `copyAndCompactAccelerationStructure()` | `copy_and_compact_acceleration_structure()` |
| `copyFromBuffer()` | `copy_from_buffer_to_buffer()` |
| `copyFromBuffer()` | `copy_from_texture_to_buffer()` |
| `copyFromBuffer()` | `copy_from_texture_to_buffer()` |
| `copyFromTensor()` | `copy_from_tensor()` |
| `copyFromTexture()` | `copy_from_texture_to_texture()` |
| `copyFromTexture()` | `copy_from_tensor()` |
| `copyFromTexture()` | `copy_from_texture_to_buffer()` |
| `copyFromTexture()` | `copy_from_texture_to_buffer()` |
| `copyFromTexture()` | `copy_from_texture_to_buffer()` |
| `copyIndirectCommandBuffer()` | `copy_indirect_command_buffer()` |
| `dispatchThreadgroups()` | `dispatch_threadgroups()` |
| `dispatchThreadgroups()` | `dispatch_threadgroups()` |
| `dispatchThreads()` | `dispatch_threads()` |
| `dispatchThreads()` | `dispatch_threads()` |
| `executeCommandsInBuffer()` | `execute_commands_in_buffer()` |
| `executeCommandsInBuffer()` | `execute_commands_in_buffer()` |
| `fillBuffer()` | `fill_buffer()` |
| `generateMipmaps()` | `generate_mipmaps()` |
| `optimizeContentsForCPUAccess()` | `optimize_contents_for_cpu_access()` |
| `optimizeContentsForCPUAccess()` | `optimize_contents_for_cpu_access()` |
| `optimizeContentsForGPUAccess()` | `optimize_contents_for_gpu_access()` |
| `optimizeContentsForGPUAccess()` | `optimize_contents_for_gpu_access()` |
| `optimizeIndirectCommandBuffer()` | `optimize_indirect_command_buffer()` |
| `refitAccelerationStructure()` | `refit_acceleration_structure()` |
| `refitAccelerationStructure()` | `refit_acceleration_structure()` |
| `resetCommandsInBuffer()` | `reset_commands_in_buffer()` |
| `setArgumentTable()` | `set_argument_table()` |
| `setComputePipelineState()` | `set_compute_pipeline_state()` |
| `setImageblockWidth()` | `set_imageblock_width()` |
| `setThreadgroupMemoryLength()` | `set_threadgroup_memory_length()` |
| `stages()` | `stages()` |
| `writeCompactedAccelerationStructureSize()` | `write_compacted_acceleration_structure_size()` |
| `writeTimestamp()` | `write_timestamp()` |

### ComputeCommandEncoder

**Rust:** `ComputeCommandEncoder` in `metal::src::encoder::compute_encoder`

| C++ | Rust |
|-----|------|
| `dispatchThreadgroups()` | `dispatch_threadgroups()` |
| `dispatchThreadgroups()` | `dispatch_threadgroups()` |
| `dispatchThreads()` | `dispatch_threads()` |
| `dispatchType()` | `dispatch_type()` |
| `executeCommandsInBuffer()` | `execute_commands_in_buffer()` |
| `executeCommandsInBuffer()` | `execute_commands_in_buffer()` |
| `memoryBarrier()` | `memory_barrier_with_scope()` |
| `memoryBarrier()` | `memory_barrier_with_scope()` |
| `sampleCountersInBuffer()` | `sample_counters_in_buffer_ptr()` |
| `setAccelerationStructure()` | `set_acceleration_structure()` |
| `setBuffer()` | `set_buffer()` |
| `setBuffer()` | `set_buffer()` |
| `setBufferOffset()` | `set_buffer_offset()` |
| `setBufferOffset()` | `set_buffer_offset()` |
| `setBuffers()` | `set_buffers()` |
| `setBuffers()` | `set_buffers()` |
| `setBytes()` | `set_bytes()` |
| `setBytes()` | `set_bytes()` |
| `setComputePipelineState()` | `set_compute_pipeline_state()` |
| `setImageblockWidth()` | `set_imageblock_width()` |
| `setIntersectionFunctionTable()` | `set_intersection_function_table_ptr()` |
| `setIntersectionFunctionTables()` | `set_intersection_function_table_ptr()` |
| `setSamplerState()` | `set_sampler_state()` |
| `setSamplerState()` | `set_sampler_state()` |
| `setSamplerStates()` | `set_sampler_state()` |
| `setSamplerStates()` | `set_sampler_states_ptr()` |
| `setStageInRegion()` | `set_stage_in_region()` |
| `setStageInRegion()` | `set_stage_in_region()` |
| `setTexture()` | `set_texture()` |
| `setTextures()` | `set_textures()` |
| `setThreadgroupMemoryLength()` | `set_threadgroup_memory_length()` |
| `setVisibleFunctionTable()` | `set_visible_function_table_ptr()` |
| `setVisibleFunctionTables()` | `set_visible_function_table_ptr()` |
| `updateFence()` | `update_fence()` |
| `useHeap()` | `use_heap()` |
| `useHeaps()` | `use_heaps()` |
| `useResource()` | `use_resource()` |
| `useResources()` | `use_resources()` |
| `waitForFence()` | `wait_for_fence()` |

### ComputePassDescriptor

**Rust:** `ComputePassDescriptor` in `metal::src::pass::compute_pass`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `computePassDescriptor()` | `new()` |
| `dispatchType()` | `dispatch_type()` |
| `init()` | `new()` |
| `sampleBufferAttachments()` | `sample_buffer_attachments()` |
| `setDispatchType()` | `set_dispatch_type()` |

### ComputePassSampleBufferAttachmentDescriptor

**Rust:** `ComputePassSampleBufferAttachmentDescriptor` in `metal::src::pass::compute_sample_buffer`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `endOfEncoderSampleIndex()` | `end_of_encoder_sample_index()` |
| `init()` | `new()` |
| `sampleBuffer()` | `sample_buffer()` |
| `setEndOfEncoderSampleIndex()` | `set_end_of_encoder_sample_index()` |
| `setSampleBuffer()` | `set_sample_buffer()` |
| `setStartOfEncoderSampleIndex()` | `set_start_of_encoder_sample_index()` |
| `startOfEncoderSampleIndex()` | `start_of_encoder_sample_index()` |

### ComputePassSampleBufferAttachmentDescriptorArray

**Rust:** `ComputePassSampleBufferAttachmentDescriptorArray` in `metal::src::pass::compute_sample_buffer`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `object()` | `object()` |
| `setObject()` | `set_object()` |

### ComputePipelineDescriptor

**Rust:** `ComputePipelineDescriptor` in `metal::src::mtl4::compute_pipeline`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `computeFunctionDescriptor()` | `compute_function_descriptor()` |
| `init()` | `new()` |
| `maxTotalThreadsPerThreadgroup()` | `max_total_threads_per_threadgroup()` |
| `requiredThreadsPerThreadgroup()` | `required_threads_per_threadgroup()` |
| `reset()` | `reset()` |
| `setComputeFunctionDescriptor()` | `set_compute_function_descriptor()` |
| `setMaxTotalThreadsPerThreadgroup()` | `set_max_total_threads_per_threadgroup()` |
| `setRequiredThreadsPerThreadgroup()` | `set_required_threads_per_threadgroup()` |
| `setStaticLinkingDescriptor()` | `set_static_linking_descriptor()` |
| `setSupportBinaryLinking()` | `set_support_binary_linking()` |
| `setSupportIndirectCommandBuffers()` | `set_support_indirect_command_buffers()` |
| `setThreadGroupSizeIsMultipleOfThreadExecutionWidth()` | `set_thread_group_size_is_multiple_of_thread_execution_width()` |
| `staticLinkingDescriptor()` | `static_linking_descriptor()` |
| `supportBinaryLinking()` | `support_binary_linking()` |
| `supportIndirectCommandBuffers()` | `support_indirect_command_buffers()` |
| `threadGroupSizeIsMultipleOfThreadExecutionWidth()` | `thread_group_size_is_multiple_of_thread_execution_width()` |

### ComputePipelineDescriptor

**Rust:** `ComputePipelineDescriptor` in `metal::src::mtl4::compute_pipeline`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `binaryArchives()` | `binary_archives_raw()` |
| `buffers()` | `buffers()` |
| `computeFunction()` | `compute_function()` |
| `init()` | `new()` |
| `insertLibraries()` | `insert_libraries_raw()` |
| `label()` | `label()` |
| `linkedFunctions()` | `linked_functions()` |
| `maxCallStackDepth()` | `max_call_stack_depth()` |
| `maxTotalThreadsPerThreadgroup()` | `max_total_threads_per_threadgroup()` |
| `preloadedLibraries()` | `preloaded_libraries_raw()` |
| `requiredThreadsPerThreadgroup()` | `required_threads_per_threadgroup()` |
| `reset()` | `reset()` |
| `setBinaryArchives()` | `set_binary_archives_raw()` |
| `setComputeFunction()` | `set_compute_function()` |
| `setInsertLibraries()` | `set_insert_libraries_raw()` |
| `setLabel()` | `set_label()` |
| `setLinkedFunctions()` | `set_linked_functions()` |
| `setMaxCallStackDepth()` | `set_max_call_stack_depth()` |
| `setMaxTotalThreadsPerThreadgroup()` | `set_max_total_threads_per_threadgroup()` |
| `setPreloadedLibraries()` | `set_preloaded_libraries_raw()` |
| `setRequiredThreadsPerThreadgroup()` | `set_required_threads_per_threadgroup()` |
| `setShaderValidation()` | `set_shader_validation()` |
| `setStageInputDescriptor()` | `set_stage_input_descriptor_raw()` |
| `setSupportAddingBinaryFunctions()` | `set_support_adding_binary_functions()` |
| `setSupportIndirectCommandBuffers()` | `set_support_indirect_command_buffers()` |
| `setThreadGroupSizeIsMultipleOfThreadExecutionWidth()` | `set_thread_group_size_is_multiple_of_thread_execution_width()` |
| `shaderValidation()` | `shader_validation()` |
| `stageInputDescriptor()` | `stage_input_descriptor_raw()` |
| `supportAddingBinaryFunctions()` | `support_adding_binary_functions()` |
| `supportIndirectCommandBuffers()` | `support_indirect_command_buffers()` |
| `threadGroupSizeIsMultipleOfThreadExecutionWidth()` | `thread_group_size_is_multiple_of_thread_execution_width()` |

### ComputePipelineReflection

**Rust:** `ComputePipelineReflection` in `metal::src::pipeline::reflection`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `arguments()` | `arguments_raw()` |
| `bindings()` | `bindings_raw()` |
| `init()` | `new()` |

### ComputePipelineState

**Rust:** `ComputePipelineState` in `metal::src::pipeline::compute_state`

| C++ | Rust |
|-----|------|
| `device()` | `device()` |
| `functionHandle()` | `function_handle_with_name()` |
| `functionHandle()` | `function_handle_with_name()` |
| `functionHandle()` | `function_handle_with_name()` |
| `gpuResourceID()` | `gpu_resource_id()` |
| `imageblockMemoryLength()` | `imageblock_memory_length()` |
| `label()` | `label()` |
| `maxTotalThreadsPerThreadgroup()` | `max_total_threads_per_threadgroup()` |
| `newComputePipelineState()` | `new_compute_pipeline_state_with_functions()` |
| `newComputePipelineStateWithBinaryFunctions()` | `new_compute_pipeline_state_with_binary_functions()` |
| `newIntersectionFunctionTable()` | `new_intersection_function_table()` |
| `newVisibleFunctionTable()` | `new_visible_function_table()` |
| `reflection()` | `reflection()` |
| `requiredThreadsPerThreadgroup()` | `required_threads_per_threadgroup()` |
| `shaderValidation()` | `shader_validation()` |
| `staticThreadgroupMemoryLength()` | `static_threadgroup_memory_length()` |
| `supportIndirectCommandBuffers()` | `support_indirect_command_buffers()` |
| `threadExecutionWidth()` | `thread_execution_width()` |

### Counter

**Rust:** `Counter` in `metal::src::counter`

| C++ | Rust |
|-----|------|
| `name()` | `name()` |

### CounterHeap

**Rust:** `CounterHeap` in `metal::src::mtl4::counters`

| C++ | Rust |
|-----|------|
| `count()` | `count()` |
| `invalidateCounterRange()` | `invalidate_counter_range()` |
| `label()` | `label()` |
| `resolveCounterRange()` | `resolve_counter_range_raw()` |
| `setLabel()` | `set_label()` |
| `type()` | `heap_type()` |

### CounterHeapDescriptor

**Rust:** `CounterHeapDescriptor` in `metal::src::mtl4::counters`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `count()` | `count()` |
| `init()` | `new()` |
| `setCount()` | `set_count()` |
| `setType()` | `set_heap_type()` |
| `type()` | `heap_type()` |

### CounterSampleBuffer

**Rust:** `CounterSampleBuffer` in `metal::src::counter`

| C++ | Rust |
|-----|------|
| `device()` | `device()` |
| `label()` | `label()` |
| `resolveCounterRange()` | `resolve_counter_range_raw()` |
| `sampleCount()` | `sample_count()` |

### CounterSampleBufferDescriptor

**Rust:** `CounterSampleBufferDescriptor` in `metal::src::counter`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `counterSet()` | `counter_set()` |
| `init()` | `new()` |
| `label()` | `label()` |
| `sampleCount()` | `sample_count()` |
| `setCounterSet()` | `set_counter_set()` |
| `setLabel()` | `set_label()` |
| `setSampleCount()` | `set_sample_count()` |
| `setStorageMode()` | `set_storage_mode()` |
| `storageMode()` | `storage_mode()` |

### CounterSet

**Rust:** `CounterSet` in `metal::src::counter`

| C++ | Rust |
|-----|------|
| `counters()` | `counters_raw()` |
| `name()` | `name()` |

### DepthStencilDescriptor

**Rust:** `DepthStencilDescriptor` in `metal::src::depth_stencil`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `backFaceStencil()` | `back_face_stencil()` |
| `depthCompareFunction()` | `depth_compare_function()` |
| `depthWriteEnabled()` | `is_depth_write_enabled()` |
| `frontFaceStencil()` | `front_face_stencil()` |
| `init()` | `new()` |
| `isDepthWriteEnabled()` | `is_depth_write_enabled()` |
| `label()` | `label()` |
| `setBackFaceStencil()` | `set_back_face_stencil()` |
| `setDepthCompareFunction()` | `set_depth_compare_function()` |
| `setDepthWriteEnabled()` | `set_depth_write_enabled()` |
| `setFrontFaceStencil()` | `set_front_face_stencil()` |
| `setLabel()` | `set_label()` |

### DepthStencilState

**Rust:** `DepthStencilState` in `metal::src::depth_stencil`

| C++ | Rust |
|-----|------|
| `device()` | `device()` |
| `gpuResourceID()` | `gpu_resource_id()` |
| `label()` | `label()` |

### Device

**Rust:** `Device` in `metal::src::device`

| C++ | Rust |
|-----|------|
| `accelerationStructureSizes()` | `new_acceleration_structure_with_size()` |
| `architecture()` | `architecture()` |
| `areBarycentricCoordsSupported()` | `are_barycentric_coords_supported()` |
| `areProgrammableSamplePositionsSupported()` | `are_programmable_sample_positions_supported()` |
| `areRasterOrderGroupsSupported()` | `are_raster_order_groups_supported()` |
| `argumentBuffersSupport()` | `argument_buffers_support()` |
| `barycentricCoordsSupported()` | `are_barycentric_coords_supported()` |
| `convertSparsePixelRegions()` | `convert_sparse_pixel_regions()` |
| `convertSparseTileRegions()` | `convert_sparse_tile_regions()` |
| `counterSets()` | `counter_sets_raw()` |
| `currentAllocatedSize()` | `current_allocated_size()` |
| `depth24Stencil8PixelFormatSupported()` | `is_depth24_stencil8_pixel_format_supported()` |
| `functionHandle()` | `function_handle_with_binary_function()` |
| `functionHandle()` | `function_handle_with_binary_function()` |
| `getDefaultSamplePositions()` | `get_default_sample_positions()` |
| `hasUnifiedMemory()` | `has_unified_memory()` |
| `headless()` | `is_headless()` |
| `heapAccelerationStructureSizeAndAlign()` | `heap_acceleration_structure_size_and_align_with_size()` |
| `heapAccelerationStructureSizeAndAlign()` | `heap_acceleration_structure_size_and_align_with_size()` |
| `heapBufferSizeAndAlign()` | `heap_buffer_size_and_align()` |
| `heapTextureSizeAndAlign()` | `heap_texture_size_and_align()` |
| `isDepth24Stencil8PixelFormatSupported()` | `is_depth24_stencil8_pixel_format_supported()` |
| `isHeadless()` | `is_headless()` |
| `isLowPower()` | `is_low_power()` |
| `isRemovable()` | `is_removable()` |
| `location()` | `location()` |
| `locationNumber()` | `location_number()` |
| `lowPower()` | `is_low_power()` |
| `maxArgumentBufferSamplerCount()` | `max_argument_buffer_sampler_count()` |
| `maxBufferLength()` | `max_buffer_length()` |
| `maxThreadgroupMemoryLength()` | `max_threadgroup_memory_length()` |
| `maxThreadsPerThreadgroup()` | `max_threads_per_threadgroup()` |
| `maxTransferRate()` | `max_transfer_rate()` |
| `maximumConcurrentCompilationTaskCount()` | `maximum_concurrent_compilation_task_count()` |
| `minimumLinearTextureAlignmentForPixelFormat()` | `minimum_linear_texture_alignment_for_pixel_format()` |
| `minimumTextureBufferAlignmentForPixelFormat()` | `minimum_texture_buffer_alignment_for_pixel_format()` |
| `name()` | `name()` |
| `newAccelerationStructure()` | `new_acceleration_structure_with_size()` |
| `newAccelerationStructure()` | `new_acceleration_structure_with_size()` |
| `newArchive()` | `new_binary_archive()` |
| `newArgumentEncoder()` | `new_argument_encoder_with_arguments()` |
| `newArgumentEncoder()` | `new_argument_encoder_with_arguments()` |
| `newArgumentTable()` | `new_argument_table()` |
| `newBinaryArchive()` | `new_binary_archive()` |
| `newBuffer()` | `new_buffer()` |
| `newBuffer()` | `new_buffer()` |
| `newBuffer()` | `new_buffer()` |
| `newCommandAllocator()` | `new_command_allocator()` |
| `newCommandAllocator()` | `new_command_allocator()` |
| `newCommandBuffer()` | `new_command_queue()` |
| `newCommandQueue()` | `new_command_queue()` |
| `newCommandQueue()` | `new_command_queue()` |
| `newCommandQueue()` | `new_command_queue()` |
| `newCompiler()` | `new_compiler()` |
| `newComputePipelineState()` | `new_compute_pipeline_state_with_function()` |
| `newComputePipelineState()` | `new_compute_pipeline_state_with_function()` |
| `newComputePipelineState()` | `new_compute_pipeline_state_with_function()` |
| `newComputePipelineState()` | `new_compute_pipeline_state_with_function()` |
| `newComputePipelineState()` | `new_compute_pipeline_state_with_function()` |
| `newComputePipelineState()` | `new_compute_pipeline_state_with_function()` |
| `newComputePipelineState()` | `new_compute_pipeline_state_with_function()` |
| `newComputePipelineState()` | `new_compute_pipeline_state_with_function()` |
| `newComputePipelineState()` | `new_compute_pipeline_state_with_function()` |
| `newCounterHeap()` | `new_counter_heap()` |
| `newCounterSampleBuffer()` | `new_counter_sample_buffer()` |
| `newDefaultLibrary()` | `new_default_library()` |
| `newDefaultLibrary()` | `new_default_library()` |
| `newDepthStencilState()` | `new_depth_stencil_state()` |
| `newDynamicLibrary()` | `new_default_library()` |
| `newDynamicLibrary()` | `new_default_library()` |
| `newEvent()` | `new_event()` |
| `newFence()` | `new_fence()` |
| `newHeap()` | `new_heap()` |
| `newIOCommandQueue()` | `new_io_command_queue()` |
| `newIOFileHandle()` | `new_io_file_handle()` |
| `newIOFileHandle()` | `new_io_file_handle()` |
| `newIOHandle()` | `new_io_handle()` |
| `newIOHandle()` | `new_io_handle()` |
| `newIndirectCommandBuffer()` | `new_indirect_command_buffer()` |
| `newLibrary()` | `new_library_with_source()` |
| `newLibrary()` | `new_library_with_source()` |
| `newLibrary()` | `new_library_with_source()` |
| `newLibrary()` | `new_library_with_source()` |
| `newLibrary()` | `new_library_with_source()` |
| `newLibrary()` | `new_library_with_source()` |
| `newLibrary()` | `new_library_with_source()` |
| `newLibrary()` | `new_library_with_source()` |
| `newLibrary()` | `new_library_with_source()` |
| `newLogState()` | `new_log_state()` |
| `newMTL4CommandQueue()` | `new_mtl4_command_queue()` |
| `newMTL4CommandQueue()` | `new_mtl4_command_queue()` |
| `newPipelineDataSetSerializer()` | `new_pipeline_data_set_serializer()` |
| `newRasterizationRateMap()` | `new_rasterization_rate_map()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newResidencySet()` | `new_residency_set()` |
| `newSamplerState()` | `new_sampler_state()` |
| `newSharedEvent()` | `new_shared_event()` |
| `newSharedEvent()` | `new_shared_event()` |
| `newSharedTexture()` | `new_texture()` |
| `newSharedTexture()` | `new_texture()` |
| `newTensor()` | `new_tensor()` |
| `newTexture()` | `new_texture()` |
| `newTexture()` | `new_texture()` |
| `newTextureViewPool()` | `new_texture_view_pool()` |
| `peerCount()` | `peer_count()` |
| `peerGroupID()` | `peer_group_id()` |
| `peerIndex()` | `peer_index()` |
| `programmableSamplePositionsSupported()` | `are_programmable_sample_positions_supported()` |
| `queryTimestampFrequency()` | `query_timestamp_frequency()` |
| `rasterOrderGroupsSupported()` | `are_raster_order_groups_supported()` |
| `readWriteTextureSupport()` | `read_write_texture_support()` |
| `recommendedMaxWorkingSetSize()` | `recommended_max_working_set_size()` |
| `registryID()` | `registry_id()` |
| `removable()` | `is_removable()` |
| `sampleTimestamps()` | `sample_timestamps()` |
| `setShouldMaximizeConcurrentCompilation()` | `set_should_maximize_concurrent_compilation()` |
| `shouldMaximizeConcurrentCompilation()` | `should_maximize_concurrent_compilation()` |
| `sizeOfCounterHeapEntry()` | `size_of_counter_heap_entry()` |
| `sparseTileSize()` | `sparse_tile_size()` |
| `sparseTileSize()` | `sparse_tile_size()` |
| `sparseTileSizeInBytes()` | `sparse_tile_size_in_bytes()` |
| `sparseTileSizeInBytes()` | `sparse_tile_size_in_bytes()` |
| `supports32BitFloatFiltering()` | `supports_32bit_float_filtering()` |
| `supports32BitMSAA()` | `supports_32bit_msaa()` |
| `supportsBCTextureCompression()` | `supports_bc_texture_compression()` |
| `supportsCounterSampling()` | `supports_counter_sampling()` |
| `supportsDynamicLibraries()` | `supports_dynamic_libraries()` |
| `supportsFamily()` | `supports_family()` |
| `supportsFeatureSet()` | `supports_feature_set()` |
| `supportsFunctionPointers()` | `supports_function_pointers()` |
| `supportsFunctionPointersFromRender()` | `supports_function_pointers_from_render()` |
| `supportsPrimitiveMotionBlur()` | `supports_primitive_motion_blur()` |
| `supportsPullModelInterpolation()` | `supports_pull_model_interpolation()` |
| `supportsQueryTextureLOD()` | `supports_query_texture_lod()` |
| `supportsRasterizationRateMap()` | `supports_rasterization_rate_map()` |
| `supportsRaytracing()` | `supports_raytracing()` |
| `supportsRaytracingFromRender()` | `supports_raytracing_from_render()` |
| `supportsRenderDynamicLibraries()` | `supports_render_dynamic_libraries()` |
| `supportsShaderBarycentricCoordinates()` | `supports_shader_barycentric_coordinates()` |
| `supportsTextureSampleCount()` | `supports_texture_sample_count()` |
| `supportsVertexAmplificationCount()` | `supports_vertex_amplification_count()` |
| `tensorSizeAndAlign()` | `heap_texture_size_and_align()` |

### Dictionary

**Rust:** `Dictionary` in `metal-foundation::src::dictionary`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `count()` | `n/a()` |
| `dictionary()` | `n/a()` |
| `dictionary()` | `n/a()` |
| `dictionary()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `keyEnumerator()` | `n/a()` |
| `object()` | `n/a()` |

### Drawable

**Rust:** `Drawable` in `metal::src::drawable`

| C++ | Rust |
|-----|------|
| `addPresentedHandler()` | `add_presented_handler()` |
| `addPresentedHandler()` | `add_presented_handler()` |
| `drawableID()` | `drawable_id()` |
| `present()` | `present()` |
| `presentAfterMinimumDuration()` | `present_after_minimum_duration()` |
| `presentAtTime()` | `present_at_time()` |
| `presentedTime()` | `presented_time()` |

### DynamicLibrary

**Rust:** `DynamicLibrary` in `metal::src::library::dynamic_library`

| C++ | Rust |
|-----|------|
| `device()` | `device()` |
| `installName()` | `install_name()` |
| `label()` | `label()` |
| `serializeToURL()` | `serialize_to_url()` |
| `setLabel()` | `set_label()` |

### Event

**Rust:** `Event` in `metal::src::sync`

| C++ | Rust |
|-----|------|
| `device()` | `device()` |
| `label()` | `label()` |
| `setLabel()` | `set_label()` |

### Fence

**Rust:** `Fence` in `metal::src::sync`

| C++ | Rust |
|-----|------|
| `device()` | `device()` |
| `label()` | `label()` |
| `setLabel()` | `set_label()` |

### FrameInterpolator

**Rust:** `FrameInterpolator` in `metal-fx::src::frame_interpolator`

| C++ | Rust |
|-----|------|
| `encodeToCommandBuffer()` | `encode_to_command_buffer()` |

### FrameInterpolator

**Rust:** `FrameInterpolator` in `metal-fx::src::frame_interpolator`

| C++ | Rust |
|-----|------|
| `encodeToCommandBuffer()` | `encode_to_command_buffer()` |

### FrameInterpolatorDescriptor

**Rust:** `FrameInterpolatorDescriptor` in `metal-fx::src::frame_interpolator`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `colorTextureFormat()` | `color_texture_format()` |
| `depthTextureFormat()` | `depth_texture_format()` |
| `init()` | `new()` |
| `inputHeight()` | `input_height()` |
| `inputWidth()` | `input_width()` |
| `motionTextureFormat()` | `motion_texture_format()` |
| `newFrameInterpolator()` | `new_frame_interpolator()` |
| `newFrameInterpolator()` | `new_frame_interpolator()` |
| `outputHeight()` | `output_height()` |
| `outputTextureFormat()` | `output_texture_format()` |
| `outputWidth()` | `output_width()` |
| `scaler()` | `scaler_raw()` |
| `setColorTextureFormat()` | `set_color_texture_format()` |
| `setDepthTextureFormat()` | `set_depth_texture_format()` |
| `setInputHeight()` | `set_input_height()` |
| `setInputWidth()` | `set_input_width()` |
| `setMotionTextureFormat()` | `set_motion_texture_format()` |
| `setOutputHeight()` | `set_output_height()` |
| `setOutputTextureFormat()` | `set_output_texture_format()` |
| `setOutputWidth()` | `set_output_width()` |
| `setScaler()` | `set_scaler_raw()` |
| `setUITextureFormat()` | `set_ui_texture_format()` |
| `supportsDevice()` | `supports_device()` |
| `supportsMetal4FX()` | `supports_device()` |
| `uiTextureFormat()` | `ui_texture_format()` |

### Function

**Rust:** `Function` in `metal::src::library::function`

| C++ | Rust |
|-----|------|
| `device()` | `device()` |
| `functionConstantsDictionary()` | `function_constants_dictionary_raw()` |
| `functionType()` | `function_type()` |
| `label()` | `label()` |
| `name()` | `name()` |
| `newArgumentEncoder()` | `new_argument_encoder()` |
| `newArgumentEncoder()` | `new_argument_encoder()` |
| `options()` | `options()` |
| `patchControlPointCount()` | `patch_control_point_count()` |
| `patchType()` | `patch_type()` |
| `setLabel()` | `set_label()` |
| `stageInputAttributes()` | `stage_input_attributes_raw()` |
| `vertexAttributes()` | `vertex_attributes_raw()` |

### FunctionConstant

**Rust:** `FunctionConstant` in `metal::src::library::function_constant`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `index()` | `index()` |
| `init()` | `n/a()` |
| `name()` | `name()` |
| `required()` | `required()` |
| `type()` | `constant_type()` |

### FunctionConstantValues

**Rust:** `FunctionConstantValues` in `metal::src::library::function_constant_values`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `reset()` | `reset()` |
| `setConstantValue()` | `set_constant_values()` |
| `setConstantValue()` | `set_constant_values()` |
| `setConstantValues()` | `set_constant_values()` |

### FunctionDescriptor

**Rust:** `FunctionDescriptor` in `metal::src::library::function_descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |

### FunctionDescriptor

**Rust:** `FunctionDescriptor` in `metal::src::library::function_descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `binaryArchives()` | `binary_archives_raw()` |
| `constantValues()` | `constant_values()` |
| `functionDescriptor()` | `new()` |
| `init()` | `new()` |
| `name()` | `name()` |
| `options()` | `options()` |
| `setBinaryArchives()` | `set_binary_archives_raw()` |
| `setConstantValues()` | `set_constant_values()` |
| `setName()` | `set_name()` |
| `setOptions()` | `set_options()` |
| `setSpecializedName()` | `set_specialized_name()` |
| `specializedName()` | `specialized_name()` |

### FunctionHandle

**Rust:** `FunctionHandle` in `metal::src::function_table`

| C++ | Rust |
|-----|------|
| `device()` | `device()` |
| `functionType()` | `function_type()` |
| `gpuResourceID()` | `gpu_resource_id()` |
| `name()` | `name()` |

### FunctionLog

**Rust:** `FunctionLog` in `metal::src::function_log`

| C++ | Rust |
|-----|------|
| `debugLocation()` | `debug_location()` |
| `encoderLabel()` | `encoder_label()` |
| `function()` | `function()` |
| `type()` | `log_type()` |

### FunctionLogDebugLocation

**Rust:** `FunctionLogDebugLocation` in `metal::src::function_log`

| C++ | Rust |
|-----|------|
| `URL()` | `url()` |
| `column()` | `column()` |
| `functionName()` | `function_name()` |
| `line()` | `line()` |

### FunctionReflection

**Rust:** `FunctionReflection` in `metal::src::library::function_reflection`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `bindings()` | `bindings_raw()` |
| `init()` | `new()` |

### FunctionStitchingAttribute

**Rust:** `FunctionStitchingAttribute` in `metal::src::function_stitching`

### FunctionStitchingAttributeAlwaysInline

**Rust:** `FunctionStitchingAttributeAlwaysInline` in `metal::src::function_stitching`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |

### FunctionStitchingFunctionNode

**Rust:** `FunctionStitchingFunctionNode` in `metal::src::function_stitching`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `arguments()` | `arguments_ptr()` |
| `controlDependencies()` | `control_dependencies_ptr()` |
| `init()` | `new()` |
| `init()` | `new()` |
| `name()` | `name()` |
| `setArguments()` | `set_arguments_ptr()` |
| `setControlDependencies()` | `set_control_dependencies_ptr()` |
| `setName()` | `set_name()` |

### FunctionStitchingGraph

**Rust:** `FunctionStitchingGraph` in `metal::src::function_stitching`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `attributes()` | `attributes_ptr()` |
| `functionName()` | `function_name()` |
| `init()` | `new()` |
| `init()` | `new()` |
| `nodes()` | `nodes_ptr()` |
| `outputNode()` | `output_node()` |
| `setAttributes()` | `set_attributes_ptr()` |
| `setFunctionName()` | `set_function_name()` |
| `setNodes()` | `set_nodes_ptr()` |
| `setOutputNode()` | `set_output_node()` |

### FunctionStitchingInputNode

**Rust:** `FunctionStitchingInputNode` in `metal::src::function_stitching`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `argumentIndex()` | `argument_index()` |
| `init()` | `new()` |
| `init()` | `new()` |
| `setArgumentIndex()` | `set_argument_index()` |

### FunctionStitchingNode

**Rust:** `FunctionStitchingNode` in `metal::src::function_stitching`

### Heap

**Rust:** `Heap` in `metal::src::heap`

| C++ | Rust |
|-----|------|
| `cpuCacheMode()` | `cpu_cache_mode()` |
| `currentAllocatedSize()` | `current_allocated_size()` |
| `device()` | `device()` |
| `hazardTrackingMode()` | `hazard_tracking_mode()` |
| `label()` | `label()` |
| `maxAvailableSize()` | `max_available_size()` |
| `newAccelerationStructure()` | `new_acceleration_structure_with_size()` |
| `newAccelerationStructure()` | `new_acceleration_structure_with_size()` |
| `newAccelerationStructure()` | `new_acceleration_structure_with_size()` |
| `newAccelerationStructure()` | `new_acceleration_structure_with_size()` |
| `newBuffer()` | `new_buffer()` |
| `newBuffer()` | `new_buffer()` |
| `newTexture()` | `new_texture()` |
| `newTexture()` | `new_texture()` |
| `resourceOptions()` | `resource_options()` |
| `setLabel()` | `set_label()` |
| `setPurgeableState()` | `set_purgeable_state()` |
| `size()` | `size()` |
| `storageMode()` | `storage_mode()` |
| `type()` | `heap_type()` |
| `usedSize()` | `used_size()` |

### HeapDescriptor

**Rust:** `HeapDescriptor` in `metal::src::heap`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `cpuCacheMode()` | `cpu_cache_mode()` |
| `hazardTrackingMode()` | `hazard_tracking_mode()` |
| `init()` | `new()` |
| `maxCompatiblePlacementSparsePageSize()` | `max_compatible_placement_sparse_page_size()` |
| `resourceOptions()` | `resource_options()` |
| `setCpuCacheMode()` | `set_cpu_cache_mode()` |
| `setHazardTrackingMode()` | `set_hazard_tracking_mode()` |
| `setMaxCompatiblePlacementSparsePageSize()` | `set_max_compatible_placement_sparse_page_size()` |
| `setResourceOptions()` | `set_resource_options()` |
| `setSize()` | `set_size()` |
| `setSparsePageSize()` | `set_sparse_page_size()` |
| `setStorageMode()` | `set_storage_mode()` |
| `setType()` | `set_heap_type()` |
| `size()` | `size()` |
| `sparsePageSize()` | `sparse_page_size()` |
| `storageMode()` | `storage_mode()` |
| `type()` | `heap_type()` |

### IOCommandBuffer

**Rust:** `IOCommandBuffer` in `metal::src::io::command_buffer`

| C++ | Rust |
|-----|------|
| `addBarrier()` | `add_barrier()` |
| `addCompletedHandler()` | `add_completed_handler()` |
| `addCompletedHandler()` | `add_completed_handler()` |
| `commit()` | `commit()` |
| `copyStatusToBuffer()` | `copy_status_to_buffer()` |
| `enqueue()` | `enqueue()` |
| `error()` | `error()` |
| `label()` | `label()` |
| `loadBuffer()` | `load_buffer()` |
| `loadBytes()` | `load_bytes()` |
| `loadTexture()` | `load_texture()` |
| `popDebugGroup()` | `pop_debug_group()` |
| `pushDebugGroup()` | `push_debug_group()` |
| `setLabel()` | `set_label()` |
| `signalEvent()` | `signal_event()` |
| `status()` | `status()` |
| `tryCancel()` | `try_cancel()` |
| `wait()` | `wait()` |
| `waitUntilCompleted()` | `wait_until_completed()` |

### IOCommandQueue

**Rust:** `IOCommandQueue` in `metal::src::io::command_queue`

| C++ | Rust |
|-----|------|
| `commandBuffer()` | `command_buffer()` |
| `commandBufferWithUnretainedReferences()` | `command_buffer_with_unretained_references()` |
| `enqueueBarrier()` | `enqueue_barrier()` |
| `label()` | `label()` |
| `setLabel()` | `set_label()` |

### IOCommandQueueDescriptor

**Rust:** `IOCommandQueueDescriptor` in `metal::src::io::command_queue_descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `maxCommandBufferCount()` | `max_command_buffer_count()` |
| `maxCommandsInFlight()` | `max_commands_in_flight()` |
| `priority()` | `priority()` |
| `scratchBufferAllocator()` | `scratch_buffer_allocator_ptr()` |
| `setMaxCommandBufferCount()` | `set_max_command_buffer_count()` |
| `setMaxCommandsInFlight()` | `set_max_commands_in_flight()` |
| `setPriority()` | `set_priority()` |
| `setScratchBufferAllocator()` | `set_scratch_buffer_allocator_ptr()` |
| `setType()` | `set_queue_type()` |
| `type()` | `queue_type()` |

### IOFileHandle

**Rust:** `IOFileHandle` in `metal::src::io::file_handle`

| C++ | Rust |
|-----|------|
| `label()` | `label()` |
| `setLabel()` | `set_label()` |

### IOScratchBuffer

**Rust:** `IOScratchBuffer` in `metal::src::io::scratch_buffer`

| C++ | Rust |
|-----|------|
| `buffer()` | `buffer()` |

### IOScratchBufferAllocator

**Rust:** `IOScratchBufferAllocator` in `metal::src::io::scratch_buffer_allocator`

| C++ | Rust |
|-----|------|
| `newScratchBuffer()` | `new_scratch_buffer()` |

### IndirectCommandBuffer

**Rust:** `IndirectCommandBuffer` in `metal::src::indirect::buffer`

| C++ | Rust |
|-----|------|
| `gpuResourceID()` | `gpu_resource_id()` |
| `indirectComputeCommand()` | `indirect_compute_command()` |
| `indirectRenderCommand()` | `indirect_render_command()` |
| `reset()` | `reset()` |
| `size()` | `size()` |

### IndirectCommandBufferDescriptor

**Rust:** `IndirectCommandBufferDescriptor` in `metal::src::indirect::buffer_descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `commandTypes()` | `command_types()` |
| `inheritBuffers()` | `inherit_buffers()` |
| `inheritCullMode()` | `inherit_cull_mode()` |
| `inheritDepthBias()` | `inherit_depth_bias()` |
| `inheritDepthClipMode()` | `inherit_depth_clip_mode()` |
| `inheritDepthStencilState()` | `inherit_depth_stencil_state()` |
| `inheritFrontFacingWinding()` | `inherit_front_facing_winding()` |
| `inheritPipelineState()` | `inherit_pipeline_state()` |
| `inheritTriangleFillMode()` | `inherit_triangle_fill_mode()` |
| `init()` | `new()` |
| `maxFragmentBufferBindCount()` | `max_fragment_buffer_bind_count()` |
| `maxKernelBufferBindCount()` | `max_kernel_buffer_bind_count()` |
| `maxKernelThreadgroupMemoryBindCount()` | `max_kernel_threadgroup_memory_bind_count()` |
| `maxMeshBufferBindCount()` | `max_mesh_buffer_bind_count()` |
| `maxObjectBufferBindCount()` | `max_object_buffer_bind_count()` |
| `maxObjectThreadgroupMemoryBindCount()` | `max_object_threadgroup_memory_bind_count()` |
| `maxVertexBufferBindCount()` | `max_vertex_buffer_bind_count()` |
| `setCommandTypes()` | `set_command_types()` |
| `setInheritBuffers()` | `set_inherit_buffers()` |
| `setInheritCullMode()` | `set_inherit_cull_mode()` |
| `setInheritDepthBias()` | `set_inherit_depth_bias()` |
| `setInheritDepthClipMode()` | `set_inherit_depth_clip_mode()` |
| `setInheritDepthStencilState()` | `set_inherit_depth_stencil_state()` |
| `setInheritFrontFacingWinding()` | `set_inherit_front_facing_winding()` |
| `setInheritPipelineState()` | `set_inherit_pipeline_state()` |
| `setInheritTriangleFillMode()` | `set_inherit_triangle_fill_mode()` |
| `setMaxFragmentBufferBindCount()` | `set_max_fragment_buffer_bind_count()` |
| `setMaxKernelBufferBindCount()` | `set_max_kernel_buffer_bind_count()` |
| `setMaxKernelThreadgroupMemoryBindCount()` | `set_max_kernel_threadgroup_memory_bind_count()` |
| `setMaxMeshBufferBindCount()` | `set_max_mesh_buffer_bind_count()` |
| `setMaxObjectBufferBindCount()` | `set_max_object_buffer_bind_count()` |
| `setMaxObjectThreadgroupMemoryBindCount()` | `set_max_object_threadgroup_memory_bind_count()` |
| `setMaxVertexBufferBindCount()` | `set_max_vertex_buffer_bind_count()` |
| `setSupportColorAttachmentMapping()` | `set_support_color_attachment_mapping()` |
| `setSupportDynamicAttributeStride()` | `set_support_dynamic_attribute_stride()` |
| `setSupportRayTracing()` | `set_support_ray_tracing()` |
| `supportColorAttachmentMapping()` | `support_color_attachment_mapping()` |
| `supportDynamicAttributeStride()` | `support_dynamic_attribute_stride()` |
| `supportRayTracing()` | `support_ray_tracing()` |

### IndirectComputeCommand

**Rust:** `IndirectComputeCommand` in `metal::src::indirect::compute_command`

| C++ | Rust |
|-----|------|
| `clearBarrier()` | `clear_barrier()` |
| `concurrentDispatchThreadgroups()` | `concurrent_dispatch_threadgroups()` |
| `concurrentDispatchThreads()` | `concurrent_dispatch_threads()` |
| `reset()` | `reset()` |
| `setBarrier()` | `set_barrier()` |
| `setComputePipelineState()` | `set_compute_pipeline_state()` |
| `setImageblockWidth()` | `set_imageblock_width()` |
| `setKernelBuffer()` | `set_kernel_buffer()` |
| `setKernelBuffer()` | `set_kernel_buffer()` |
| `setStageInRegion()` | `set_stage_in_region()` |
| `setThreadgroupMemoryLength()` | `set_threadgroup_memory_length()` |

### IndirectInstanceAccelerationStructureDescriptor

**Rust:** `IndirectInstanceAccelerationStructureDescriptor` in `metal::src::acceleration::instance`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `instanceCountBuffer()` | `instance_count_buffer()` |
| `instanceDescriptorBuffer()` | `instance_descriptor_buffer()` |
| `instanceDescriptorStride()` | `instance_descriptor_stride()` |
| `instanceDescriptorType()` | `instance_descriptor_type()` |
| `instanceTransformationMatrixLayout()` | `instance_transformation_matrix_layout()` |
| `maxInstanceCount()` | `max_instance_count()` |
| `maxMotionTransformCount()` | `max_motion_transform_count()` |
| `motionTransformBuffer()` | `motion_transform_buffer()` |
| `motionTransformCountBuffer()` | `motion_transform_count_buffer()` |
| `motionTransformStride()` | `motion_transform_stride()` |
| `motionTransformType()` | `motion_transform_type()` |
| `setInstanceCountBuffer()` | `set_instance_count_buffer()` |
| `setInstanceDescriptorBuffer()` | `set_instance_descriptor_buffer()` |
| `setInstanceDescriptorStride()` | `set_instance_descriptor_stride()` |
| `setInstanceDescriptorType()` | `set_instance_descriptor_type()` |
| `setInstanceTransformationMatrixLayout()` | `set_instance_transformation_matrix_layout()` |
| `setMaxInstanceCount()` | `set_max_instance_count()` |
| `setMaxMotionTransformCount()` | `set_max_motion_transform_count()` |
| `setMotionTransformBuffer()` | `set_motion_transform_buffer()` |
| `setMotionTransformCountBuffer()` | `set_motion_transform_count_buffer()` |
| `setMotionTransformStride()` | `set_motion_transform_stride()` |
| `setMotionTransformType()` | `set_motion_transform_type()` |

### IndirectInstanceAccelerationStructureDescriptor

**Rust:** `IndirectInstanceAccelerationStructureDescriptor` in `metal::src::acceleration::instance`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `descriptor()` | `new()` |
| `init()` | `new()` |
| `instanceCountBuffer()` | `instance_count_buffer()` |
| `instanceCountBufferOffset()` | `instance_count_buffer_offset()` |
| `instanceDescriptorBuffer()` | `instance_descriptor_buffer()` |
| `instanceDescriptorBufferOffset()` | `instance_descriptor_buffer_offset()` |
| `instanceDescriptorStride()` | `instance_descriptor_stride()` |
| `instanceDescriptorType()` | `instance_descriptor_type()` |
| `instanceTransformationMatrixLayout()` | `instance_transformation_matrix_layout()` |
| `maxInstanceCount()` | `max_instance_count()` |
| `maxMotionTransformCount()` | `max_motion_transform_count()` |
| `motionTransformBuffer()` | `motion_transform_buffer()` |
| `motionTransformBufferOffset()` | `motion_transform_buffer_offset()` |
| `motionTransformCountBuffer()` | `motion_transform_count_buffer()` |
| `motionTransformCountBufferOffset()` | `motion_transform_count_buffer_offset()` |
| `motionTransformStride()` | `motion_transform_stride()` |
| `motionTransformType()` | `motion_transform_type()` |
| `setInstanceCountBuffer()` | `set_instance_count_buffer()` |
| `setInstanceCountBufferOffset()` | `set_instance_count_buffer_offset()` |
| `setInstanceDescriptorBuffer()` | `set_instance_descriptor_buffer()` |
| `setInstanceDescriptorBufferOffset()` | `set_instance_descriptor_buffer_offset()` |
| `setInstanceDescriptorStride()` | `set_instance_descriptor_stride()` |
| `setInstanceDescriptorType()` | `set_instance_descriptor_type()` |
| `setInstanceTransformationMatrixLayout()` | `set_instance_transformation_matrix_layout()` |
| `setMaxInstanceCount()` | `set_max_instance_count()` |
| `setMaxMotionTransformCount()` | `set_max_motion_transform_count()` |
| `setMotionTransformBuffer()` | `set_motion_transform_buffer()` |
| `setMotionTransformBufferOffset()` | `set_motion_transform_buffer_offset()` |
| `setMotionTransformCountBuffer()` | `set_motion_transform_count_buffer()` |
| `setMotionTransformCountBufferOffset()` | `set_motion_transform_count_buffer_offset()` |
| `setMotionTransformStride()` | `set_motion_transform_stride()` |
| `setMotionTransformType()` | `set_motion_transform_type()` |

### IndirectRenderCommand

**Rust:** `IndirectRenderCommand` in `metal::src::indirect::render_command`

| C++ | Rust |
|-----|------|
| `clearBarrier()` | `clear_barrier()` |
| `drawIndexedPatches()` | `draw_indexed_primitives()` |
| `drawIndexedPrimitives()` | `draw_indexed_primitives()` |
| `drawMeshThreadgroups()` | `draw_mesh_threadgroups()` |
| `drawMeshThreads()` | `draw_mesh_threads()` |
| `drawPatches()` | `draw_primitives()` |
| `drawPrimitives()` | `draw_primitives()` |
| `reset()` | `reset()` |
| `setBarrier()` | `set_barrier()` |
| `setCullMode()` | `set_cull_mode()` |
| `setDepthBias()` | `set_depth_bias()` |
| `setDepthClipMode()` | `set_depth_clip_mode()` |
| `setDepthStencilState()` | `set_depth_stencil_state()` |
| `setFragmentBuffer()` | `set_fragment_buffer()` |
| `setFrontFacingWinding()` | `set_front_facing_winding()` |
| `setMeshBuffer()` | `set_mesh_buffer()` |
| `setObjectBuffer()` | `set_object_buffer()` |
| `setObjectThreadgroupMemoryLength()` | `set_object_threadgroup_memory_length()` |
| `setRenderPipelineState()` | `set_render_pipeline_state()` |
| `setTriangleFillMode()` | `set_triangle_fill_mode()` |
| `setVertexBuffer()` | `set_vertex_buffer()` |
| `setVertexBuffer()` | `set_vertex_buffer()` |

### InstanceAccelerationStructureDescriptor

**Rust:** `InstanceAccelerationStructureDescriptor` in `metal::src::acceleration::instance`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `instanceCount()` | `instance_count()` |
| `instanceDescriptorBuffer()` | `instance_descriptor_buffer()` |
| `instanceDescriptorStride()` | `instance_descriptor_stride()` |
| `instanceDescriptorType()` | `instance_descriptor_type()` |
| `instanceTransformationMatrixLayout()` | `instance_transformation_matrix_layout()` |
| `motionTransformBuffer()` | `motion_transform_buffer()` |
| `motionTransformCount()` | `motion_transform_count()` |
| `motionTransformStride()` | `motion_transform_stride()` |
| `motionTransformType()` | `motion_transform_type()` |
| `setInstanceCount()` | `set_instance_count()` |
| `setInstanceDescriptorBuffer()` | `set_instance_descriptor_buffer()` |
| `setInstanceDescriptorStride()` | `set_instance_descriptor_stride()` |
| `setInstanceDescriptorType()` | `set_instance_descriptor_type()` |
| `setInstanceTransformationMatrixLayout()` | `set_instance_transformation_matrix_layout()` |
| `setMotionTransformBuffer()` | `set_motion_transform_buffer()` |
| `setMotionTransformCount()` | `set_motion_transform_count()` |
| `setMotionTransformStride()` | `set_motion_transform_stride()` |
| `setMotionTransformType()` | `set_motion_transform_type()` |

### InstanceAccelerationStructureDescriptor

**Rust:** `InstanceAccelerationStructureDescriptor` in `metal::src::acceleration::instance`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `descriptor()` | `new()` |
| `init()` | `new()` |
| `instanceCount()` | `instance_count()` |
| `instanceDescriptorBuffer()` | `instance_descriptor_buffer()` |
| `instanceDescriptorBufferOffset()` | `instance_descriptor_buffer_offset()` |
| `instanceDescriptorStride()` | `instance_descriptor_stride()` |
| `instanceDescriptorType()` | `instance_descriptor_type()` |
| `instanceTransformationMatrixLayout()` | `instance_transformation_matrix_layout()` |
| `instancedAccelerationStructures()` | `instanced_acceleration_structures_ptr()` |
| `motionTransformBuffer()` | `motion_transform_buffer()` |
| `motionTransformBufferOffset()` | `motion_transform_buffer_offset()` |
| `motionTransformCount()` | `motion_transform_count()` |
| `motionTransformStride()` | `motion_transform_stride()` |
| `motionTransformType()` | `motion_transform_type()` |
| `setInstanceCount()` | `set_instance_count()` |
| `setInstanceDescriptorBuffer()` | `set_instance_descriptor_buffer()` |
| `setInstanceDescriptorBufferOffset()` | `set_instance_descriptor_buffer_offset()` |
| `setInstanceDescriptorStride()` | `set_instance_descriptor_stride()` |
| `setInstanceDescriptorType()` | `set_instance_descriptor_type()` |
| `setInstanceTransformationMatrixLayout()` | `set_instance_transformation_matrix_layout()` |
| `setInstancedAccelerationStructures()` | `set_instanced_acceleration_structures_ptr()` |
| `setMotionTransformBuffer()` | `set_motion_transform_buffer()` |
| `setMotionTransformBufferOffset()` | `set_motion_transform_buffer_offset()` |
| `setMotionTransformCount()` | `set_motion_transform_count()` |
| `setMotionTransformStride()` | `set_motion_transform_stride()` |
| `setMotionTransformType()` | `set_motion_transform_type()` |

### IntersectionFunctionDescriptor

**Rust:** `IntersectionFunctionDescriptor` in `metal::src::library::intersection_function_descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |

### IntersectionFunctionTable

**Rust:** `IntersectionFunctionTable` in `metal::src::function_table`

| C++ | Rust |
|-----|------|
| `gpuResourceID()` | `gpu_resource_id()` |
| `setBuffer()` | `set_buffer()` |
| `setBuffers()` | `set_buffers()` |
| `setFunction()` | `set_function()` |
| `setFunctions()` | `set_functions()` |
| `setOpaqueCurveIntersectionFunction()` | `set_opaque_curve_intersection_function_at_index()` |
| `setOpaqueCurveIntersectionFunction()` | `set_opaque_curve_intersection_function_at_index()` |
| `setOpaqueTriangleIntersectionFunction()` | `set_opaque_triangle_intersection_function_at_index()` |
| `setOpaqueTriangleIntersectionFunction()` | `set_opaque_triangle_intersection_function_at_index()` |
| `setVisibleFunctionTable()` | `set_visible_function_table()` |
| `setVisibleFunctionTables()` | `set_visible_function_tables()` |

### IntersectionFunctionTableDescriptor

**Rust:** `IntersectionFunctionTableDescriptor` in `metal::src::function_table`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `functionCount()` | `function_count()` |
| `init()` | `new()` |
| `intersectionFunctionTableDescriptor()` | `new()` |
| `setFunctionCount()` | `set_function_count()` |

### Library

**Rust:** `Library` in `metal::src::library::library`

| C++ | Rust |
|-----|------|
| `device()` | `device()` |
| `functionNames()` | `function_names()` |
| `installName()` | `install_name()` |
| `label()` | `label()` |
| `newFunction()` | `new_function_with_name()` |
| `newFunction()` | `new_function_with_name()` |
| `newFunction()` | `new_function_with_name()` |
| `newFunction()` | `new_function_with_name()` |
| `newFunction()` | `new_function_with_name()` |
| `newIntersectionFunction()` | `new_intersection_function()` |
| `newIntersectionFunction()` | `new_intersection_function()` |
| `reflectionForFunction()` | `reflection_for_function()` |
| `setLabel()` | `set_label()` |
| `type()` | `library_type()` |

### LibraryDescriptor

**Rust:** `LibraryDescriptor` in `metal::src::mtl4::library_descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `name()` | `name()` |
| `options()` | `options()` |
| `setName()` | `set_name()` |
| `setOptions()` | `set_options()` |
| `setSource()` | `set_source()` |
| `source()` | `source()` |

### LibraryFunctionDescriptor

**Rust:** `LibraryFunctionDescriptor` in `metal::src::mtl4::library_function_descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `library()` | `library()` |
| `name()` | `name()` |
| `setLibrary()` | `set_library()` |
| `setName()` | `set_name()` |

### LinkedFunctions

**Rust:** `LinkedFunctions` in `metal::src::library::linked_functions`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `binaryFunctions()` | `binary_functions_raw()` |
| `functions()` | `functions_raw()` |
| `groups()` | `groups_raw()` |
| `init()` | `new()` |
| `linkedFunctions()` | `linked_functions()` |
| `privateFunctions()` | `private_functions_raw()` |
| `setBinaryFunctions()` | `set_binary_functions_raw()` |
| `setFunctions()` | `set_functions_raw()` |
| `setGroups()` | `set_groups_raw()` |
| `setPrivateFunctions()` | `set_private_functions_raw()` |

### LogContainer

**Rust:** `LogContainer` in `metal::src::function_log`

### LogState

**Rust:** `LogState` in `metal::src::log_state`

| C++ | Rust |
|-----|------|
| `addLogHandler()` | `add_log_handler()` |

### LogStateDescriptor

**Rust:** `LogStateDescriptor` in `metal::src::log_state`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `bufferSize()` | `buffer_size()` |
| `init()` | `new()` |
| `level()` | `level()` |
| `setBufferSize()` | `set_buffer_size()` |
| `setLevel()` | `set_level()` |

### LogicalToPhysicalColorAttachmentMap

**Rust:** `LogicalToPhysicalColorAttachmentMap` in `metal::src::pipeline::functions_descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `getPhysicalIndex()` | `physical_index()` |
| `init()` | `new()` |
| `reset()` | `reset()` |
| `setPhysicalIndex()` | `set_physical_index()` |

### MachineLearningCommandEncoder

**Rust:** `MachineLearningCommandEncoder` in `metal::src::mtl4::machine_learning`

| C++ | Rust |
|-----|------|
| `dispatchNetwork()` | `dispatch_network()` |
| `setArgumentTable()` | `set_argument_table()` |
| `setPipelineState()` | `set_pipeline_state()` |

### MachineLearningPipelineDescriptor

**Rust:** `MachineLearningPipelineDescriptor` in `metal::src::mtl4::machine_learning`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `inputDimensionsAtBufferIndex()` | `input_dimensions_at_buffer_index_raw()` |
| `label()` | `label()` |
| `machineLearningFunctionDescriptor()` | `machine_learning_function_descriptor()` |
| `reset()` | `reset()` |
| `setInputDimensions()` | `set_input_dimensions_raw()` |
| `setInputDimensions()` | `set_input_dimensions_raw()` |
| `setLabel()` | `set_label()` |
| `setMachineLearningFunctionDescriptor()` | `set_machine_learning_function_descriptor()` |

### MachineLearningPipelineReflection

**Rust:** `MachineLearningPipelineReflection` in `metal::src::mtl4::machine_learning`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `bindings()` | `bindings_raw()` |
| `init()` | `n/a()` |

### MachineLearningPipelineState

**Rust:** `MachineLearningPipelineState` in `metal::src::mtl4::machine_learning`

| C++ | Rust |
|-----|------|
| `device()` | `device()` |
| `intermediatesHeapSize()` | `intermediates_heap_size()` |
| `label()` | `label()` |
| `reflection()` | `reflection()` |

### MeshRenderPipelineDescriptor

**Rust:** `MeshRenderPipelineDescriptor` in `metal::src::mtl4::mesh_render_pipeline`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `alphaToCoverageState()` | `alpha_to_coverage_state()` |
| `alphaToOneState()` | `alpha_to_one_state()` |
| `colorAttachmentMappingState()` | `color_attachment_mapping_state()` |
| `colorAttachments()` | `color_attachments()` |
| `fragmentFunctionDescriptor()` | `fragment_function_descriptor()` |
| `fragmentStaticLinkingDescriptor()` | `fragment_static_linking_descriptor()` |
| `init()` | `new()` |
| `isRasterizationEnabled()` | `is_rasterization_enabled()` |
| `maxTotalThreadgroupsPerMeshGrid()` | `max_total_threadgroups_per_mesh_grid()` |
| `maxTotalThreadsPerMeshThreadgroup()` | `max_total_threads_per_mesh_threadgroup()` |
| `maxTotalThreadsPerObjectThreadgroup()` | `max_total_threads_per_object_threadgroup()` |
| `maxVertexAmplificationCount()` | `max_vertex_amplification_count()` |
| `meshFunctionDescriptor()` | `mesh_function_descriptor()` |
| `meshStaticLinkingDescriptor()` | `mesh_static_linking_descriptor()` |
| `meshThreadgroupSizeIsMultipleOfThreadExecutionWidth()` | `mesh_threadgroup_size_is_multiple_of_thread_execution_width()` |
| `objectFunctionDescriptor()` | `object_function_descriptor()` |
| `objectStaticLinkingDescriptor()` | `object_static_linking_descriptor()` |
| `objectThreadgroupSizeIsMultipleOfThreadExecutionWidth()` | `object_threadgroup_size_is_multiple_of_thread_execution_width()` |
| `payloadMemoryLength()` | `payload_memory_length()` |
| `rasterSampleCount()` | `raster_sample_count()` |
| `rasterizationEnabled()` | `is_rasterization_enabled()` |
| `requiredThreadsPerMeshThreadgroup()` | `required_threads_per_mesh_threadgroup()` |
| `requiredThreadsPerObjectThreadgroup()` | `required_threads_per_object_threadgroup()` |
| `reset()` | `reset()` |
| `setAlphaToCoverageState()` | `set_alpha_to_coverage_state()` |
| `setAlphaToOneState()` | `set_alpha_to_one_state()` |
| `setColorAttachmentMappingState()` | `set_color_attachment_mapping_state()` |
| `setFragmentFunctionDescriptor()` | `set_fragment_function_descriptor()` |
| `setFragmentStaticLinkingDescriptor()` | `set_fragment_static_linking_descriptor()` |
| `setMaxTotalThreadgroupsPerMeshGrid()` | `set_max_total_threadgroups_per_mesh_grid()` |
| `setMaxTotalThreadsPerMeshThreadgroup()` | `set_max_total_threads_per_mesh_threadgroup()` |
| `setMaxTotalThreadsPerObjectThreadgroup()` | `set_max_total_threads_per_object_threadgroup()` |
| `setMaxVertexAmplificationCount()` | `set_max_vertex_amplification_count()` |
| `setMeshFunctionDescriptor()` | `set_mesh_function_descriptor()` |
| `setMeshStaticLinkingDescriptor()` | `set_mesh_static_linking_descriptor()` |
| `setMeshThreadgroupSizeIsMultipleOfThreadExecutionWidth()` | `set_mesh_threadgroup_size_is_multiple_of_thread_execution_width()` |
| `setObjectFunctionDescriptor()` | `set_object_function_descriptor()` |
| `setObjectStaticLinkingDescriptor()` | `set_object_static_linking_descriptor()` |
| `setObjectThreadgroupSizeIsMultipleOfThreadExecutionWidth()` | `set_object_threadgroup_size_is_multiple_of_thread_execution_width()` |
| `setPayloadMemoryLength()` | `set_payload_memory_length()` |
| `setRasterSampleCount()` | `set_raster_sample_count()` |
| `setRasterizationEnabled()` | `set_rasterization_enabled()` |
| `setRequiredThreadsPerMeshThreadgroup()` | `set_required_threads_per_mesh_threadgroup()` |
| `setRequiredThreadsPerObjectThreadgroup()` | `set_required_threads_per_object_threadgroup()` |
| `setSupportFragmentBinaryLinking()` | `set_support_fragment_binary_linking()` |
| `setSupportIndirectCommandBuffers()` | `set_support_indirect_command_buffers()` |
| `setSupportMeshBinaryLinking()` | `set_support_mesh_binary_linking()` |
| `setSupportObjectBinaryLinking()` | `set_support_object_binary_linking()` |
| `supportFragmentBinaryLinking()` | `support_fragment_binary_linking()` |
| `supportIndirectCommandBuffers()` | `support_indirect_command_buffers()` |
| `supportMeshBinaryLinking()` | `support_mesh_binary_linking()` |
| `supportObjectBinaryLinking()` | `support_object_binary_linking()` |

### MeshRenderPipelineDescriptor

**Rust:** `MeshRenderPipelineDescriptor` in `metal::src::mtl4::mesh_render_pipeline`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `alphaToCoverageEnabled()` | `is_alpha_to_coverage_enabled()` |
| `alphaToOneEnabled()` | `is_alpha_to_one_enabled()` |
| `binaryArchives()` | `binary_archives_raw()` |
| `colorAttachments()` | `color_attachments()` |
| `depthAttachmentPixelFormat()` | `depth_attachment_pixel_format()` |
| `fragmentBuffers()` | `fragment_buffers()` |
| `fragmentFunction()` | `fragment_function()` |
| `fragmentLinkedFunctions()` | `fragment_linked_functions()` |
| `init()` | `new()` |
| `isAlphaToCoverageEnabled()` | `is_alpha_to_coverage_enabled()` |
| `isAlphaToOneEnabled()` | `is_alpha_to_one_enabled()` |
| `isRasterizationEnabled()` | `is_rasterization_enabled()` |
| `label()` | `label()` |
| `maxTotalThreadgroupsPerMeshGrid()` | `max_total_threadgroups_per_mesh_grid()` |
| `maxTotalThreadsPerMeshThreadgroup()` | `max_total_threads_per_mesh_threadgroup()` |
| `maxTotalThreadsPerObjectThreadgroup()` | `max_total_threads_per_object_threadgroup()` |
| `maxVertexAmplificationCount()` | `max_vertex_amplification_count()` |
| `meshBuffers()` | `mesh_buffers()` |
| `meshFunction()` | `mesh_function()` |
| `meshLinkedFunctions()` | `mesh_linked_functions()` |
| `meshThreadgroupSizeIsMultipleOfThreadExecutionWidth()` | `mesh_threadgroup_size_is_multiple_of_thread_execution_width()` |
| `objectBuffers()` | `object_buffers()` |
| `objectFunction()` | `object_function()` |
| `objectLinkedFunctions()` | `object_linked_functions()` |
| `objectThreadgroupSizeIsMultipleOfThreadExecutionWidth()` | `object_threadgroup_size_is_multiple_of_thread_execution_width()` |
| `payloadMemoryLength()` | `payload_memory_length()` |
| `rasterSampleCount()` | `raster_sample_count()` |
| `rasterizationEnabled()` | `is_rasterization_enabled()` |
| `requiredThreadsPerMeshThreadgroup()` | `required_threads_per_mesh_threadgroup()` |
| `requiredThreadsPerObjectThreadgroup()` | `required_threads_per_object_threadgroup()` |
| `reset()` | `reset()` |
| `setAlphaToCoverageEnabled()` | `set_alpha_to_coverage_enabled()` |
| `setAlphaToOneEnabled()` | `set_alpha_to_one_enabled()` |
| `setBinaryArchives()` | `set_binary_archives_raw()` |
| `setDepthAttachmentPixelFormat()` | `set_depth_attachment_pixel_format()` |
| `setFragmentFunction()` | `set_fragment_function()` |
| `setFragmentLinkedFunctions()` | `set_fragment_linked_functions()` |
| `setLabel()` | `set_label()` |
| `setMaxTotalThreadgroupsPerMeshGrid()` | `set_max_total_threadgroups_per_mesh_grid()` |
| `setMaxTotalThreadsPerMeshThreadgroup()` | `set_max_total_threads_per_mesh_threadgroup()` |
| `setMaxTotalThreadsPerObjectThreadgroup()` | `set_max_total_threads_per_object_threadgroup()` |
| `setMaxVertexAmplificationCount()` | `set_max_vertex_amplification_count()` |
| `setMeshFunction()` | `set_mesh_function()` |
| `setMeshLinkedFunctions()` | `set_mesh_linked_functions()` |
| `setMeshThreadgroupSizeIsMultipleOfThreadExecutionWidth()` | `set_mesh_threadgroup_size_is_multiple_of_thread_execution_width()` |
| `setObjectFunction()` | `set_object_function()` |
| `setObjectLinkedFunctions()` | `set_object_linked_functions()` |
| `setObjectThreadgroupSizeIsMultipleOfThreadExecutionWidth()` | `set_object_threadgroup_size_is_multiple_of_thread_execution_width()` |
| `setPayloadMemoryLength()` | `set_payload_memory_length()` |
| `setRasterSampleCount()` | `set_raster_sample_count()` |
| `setRasterizationEnabled()` | `set_rasterization_enabled()` |
| `setRequiredThreadsPerMeshThreadgroup()` | `set_required_threads_per_mesh_threadgroup()` |
| `setRequiredThreadsPerObjectThreadgroup()` | `set_required_threads_per_object_threadgroup()` |
| `setShaderValidation()` | `set_shader_validation()` |
| `setStencilAttachmentPixelFormat()` | `set_stencil_attachment_pixel_format()` |
| `setSupportIndirectCommandBuffers()` | `set_support_indirect_command_buffers()` |
| `shaderValidation()` | `shader_validation()` |
| `stencilAttachmentPixelFormat()` | `stencil_attachment_pixel_format()` |
| `supportIndirectCommandBuffers()` | `support_indirect_command_buffers()` |

### MetalDrawable

**Rust:** `MetalDrawable` in `quartz-core::src::metal_drawable`

| C++ | Rust |
|-----|------|
| `layer()` | `layer()` |
| `texture()` | `texture()` |

### MetalLayer

**Rust:** `MetalLayer` in `quartz-core::src::metal_layer`

| C++ | Rust |
|-----|------|
| `allowsNextDrawableTimeout()` | `allows_next_drawable_timeout()` |
| `colorspace()` | `colorspace()` |
| `device()` | `device()` |
| `displaySyncEnabled()` | `display_sync_enabled()` |
| `drawableSize()` | `drawable_size()` |
| `framebufferOnly()` | `is_framebuffer_only()` |
| `layer()` | `layer()` |
| `maximumDrawableCount()` | `maximum_drawable_count()` |
| `nextDrawable()` | `next_drawable()` |
| `pixelFormat()` | `pixel_format()` |
| `residencySet()` | `residency_set()` |
| `setAllowsNextDrawableTimeout()` | `set_allows_next_drawable_timeout()` |
| `setColorspace()` | `set_colorspace()` |
| `setDevice()` | `set_device()` |
| `setDisplaySyncEnabled()` | `set_display_sync_enabled()` |
| `setDrawableSize()` | `set_drawable_size()` |
| `setFramebufferOnly()` | `set_framebuffer_only()` |
| `setMaximumDrawableCount()` | `set_maximum_drawable_count()` |
| `setPixelFormat()` | `set_pixel_format()` |

### MotionKeyframeData

**Rust:** `MotionKeyframeData` in `metal::src::acceleration::motion_keyframe`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `buffer()` | `buffer()` |
| `data()` | `data()` |
| `init()` | `new()` |
| `offset()` | `offset()` |
| `setBuffer()` | `set_buffer()` |
| `setOffset()` | `set_offset()` |

### Notification

**Rust:** `Notification` in `metal-foundation::src::notification`

| C++ | Rust |
|-----|------|
| `name()` | `name()` |
| `object()` | `object()` |
| `userInfo()` | `user_info()` |

### NotificationCenter

**Rust:** `NotificationCenter` in `metal-foundation::src::notification`

| C++ | Rust |
|-----|------|
| `addObserver()` | `add_observer()` |
| `addObserver()` | `add_observer()` |
| `defaultCenter()` | `default_center()` |
| `removeObserver()` | `remove_observer()` |

### ObjectPayloadBinding

**Rust:** `ObjectPayloadBinding` in `metal::src::argument::object_payload_binding`

| C++ | Rust |
|-----|------|
| `objectPayloadAlignment()` | `object_payload_alignment()` |
| `objectPayloadDataSize()` | `object_payload_data_size()` |

### ParallelRenderCommandEncoder

**Rust:** `ParallelRenderCommandEncoder` in `metal::src::encoder::parallel_render_encoder`

| C++ | Rust |
|-----|------|
| `renderCommandEncoder()` | `render_command_encoder()` |
| `setColorStoreAction()` | `set_color_store_action()` |
| `setColorStoreActionOptions()` | `set_color_store_action_options()` |
| `setDepthStoreAction()` | `set_depth_store_action()` |
| `setDepthStoreActionOptions()` | `set_depth_store_action_options()` |
| `setStencilStoreAction()` | `set_stencil_store_action()` |
| `setStencilStoreActionOptions()` | `set_stencil_store_action_options()` |

### PipelineBufferDescriptor

**Rust:** `PipelineBufferDescriptor` in `metal::src::pipeline::buffer_descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `mutability()` | `mutability()` |
| `setMutability()` | `set_mutability()` |

### PipelineBufferDescriptorArray

**Rust:** `PipelineBufferDescriptorArray` in `metal::src::pipeline::buffer_descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `init()` | `n/a()` |
| `object()` | `object()` |
| `setObject()` | `set_object()` |

### PipelineDataSetSerializer

**Rust:** `PipelineDataSetSerializer` in `metal::src::mtl4::pipeline_data_set_serializer`

| C++ | Rust |
|-----|------|
| `serializeAsArchiveAndFlushToURL()` | `serialize_as_archive_and_flush_to_url()` |
| `serializeAsPipelinesScript()` | `serialize_as_pipelines_script()` |

### PipelineDataSetSerializerDescriptor

**Rust:** `PipelineDataSetSerializerDescriptor` in `metal::src::mtl4::pipeline_data_set_serializer`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `configuration()` | `configuration()` |
| `init()` | `new()` |
| `setConfiguration()` | `set_configuration()` |

### PipelineDescriptor

**Rust:** `PipelineDescriptor` in `metal::src::mtl4::pipeline_state`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `label()` | `label()` |
| `options()` | `options()` |
| `setLabel()` | `set_label()` |
| `setOptions()` | `set_options()` |

### PipelineOptions

**Rust:** `PipelineOptions` in `metal::src::mtl4::pipeline_state`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `setShaderReflection()` | `set_shader_reflection()` |
| `setShaderValidation()` | `set_shader_validation()` |
| `shaderReflection()` | `shader_reflection()` |
| `shaderValidation()` | `shader_validation()` |

### PipelineStageDynamicLinkingDescriptor

**Rust:** `PipelineStageDynamicLinkingDescriptor` in `metal::src::mtl4::linking_descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `binaryLinkedFunctions()` | `binary_linked_functions_raw()` |
| `init()` | `new()` |
| `maxCallStackDepth()` | `max_call_stack_depth()` |
| `preloadedLibraries()` | `preloaded_libraries_raw()` |
| `setBinaryLinkedFunctions()` | `set_binary_linked_functions_raw()` |
| `setMaxCallStackDepth()` | `set_max_call_stack_depth()` |
| `setPreloadedLibraries()` | `set_preloaded_libraries_raw()` |

### PointerType

**Rust:** `PointerType` in `metal::src::argument::pointer_type`

| C++ | Rust |
|-----|------|
| `access()` | `access()` |
| `alignment()` | `alignment()` |
| `alloc()` | `n/a()` |
| `dataSize()` | `data_size()` |
| `elementArrayType()` | `element_array_type()` |
| `elementIsArgumentBuffer()` | `element_is_argument_buffer()` |
| `elementStructType()` | `element_struct_type()` |
| `elementType()` | `element_type()` |
| `init()` | `n/a()` |

### PrimitiveAccelerationStructureDescriptor

**Rust:** `PrimitiveAccelerationStructureDescriptor` in `metal::src::acceleration::descriptors`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `geometryDescriptors()` | `geometry_descriptors_raw()` |
| `init()` | `new()` |
| `motionEndBorderMode()` | `motion_end_border_mode()` |
| `motionEndTime()` | `motion_end_time()` |
| `motionKeyframeCount()` | `motion_keyframe_count()` |
| `motionStartBorderMode()` | `motion_start_border_mode()` |
| `motionStartTime()` | `motion_start_time()` |
| `setGeometryDescriptors()` | `set_geometry_descriptors_raw()` |
| `setMotionEndBorderMode()` | `set_motion_end_border_mode()` |
| `setMotionEndTime()` | `set_motion_end_time()` |
| `setMotionKeyframeCount()` | `set_motion_keyframe_count()` |
| `setMotionStartBorderMode()` | `set_motion_start_border_mode()` |
| `setMotionStartTime()` | `set_motion_start_time()` |

### PrimitiveAccelerationStructureDescriptor

**Rust:** `PrimitiveAccelerationStructureDescriptor` in `metal::src::acceleration::descriptors`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `descriptor()` | `new()` |
| `geometryDescriptors()` | `geometry_descriptors_raw()` |
| `init()` | `new()` |
| `motionEndBorderMode()` | `motion_end_border_mode()` |
| `motionEndTime()` | `motion_end_time()` |
| `motionKeyframeCount()` | `motion_keyframe_count()` |
| `motionStartBorderMode()` | `motion_start_border_mode()` |
| `motionStartTime()` | `motion_start_time()` |
| `setGeometryDescriptors()` | `set_geometry_descriptors_raw()` |
| `setMotionEndBorderMode()` | `set_motion_end_border_mode()` |
| `setMotionEndTime()` | `set_motion_end_time()` |
| `setMotionKeyframeCount()` | `set_motion_keyframe_count()` |
| `setMotionStartBorderMode()` | `set_motion_start_border_mode()` |
| `setMotionStartTime()` | `set_motion_start_time()` |

### RasterizationRateLayerArray

**Rust:** `RasterizationRateLayerArray` in `metal::src::rasterization_rate`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `init()` | `n/a()` |
| `object()` | `object()` |
| `setObject()` | `set_object()` |

### RasterizationRateLayerDescriptor

**Rust:** `RasterizationRateLayerDescriptor` in `metal::src::rasterization_rate`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `horizontal()` | `horizontal()` |
| `horizontalSampleStorage()` | `horizontal_sample_storage()` |
| `init()` | `new()` |
| `init()` | `new()` |
| `init()` | `new()` |
| `maxSampleCount()` | `max_sample_count()` |
| `sampleCount()` | `sample_count()` |
| `setSampleCount()` | `set_sample_count()` |
| `vertical()` | `vertical()` |
| `verticalSampleStorage()` | `vertical_sample_storage()` |

### RasterizationRateMap

**Rust:** `RasterizationRateMap` in `metal::src::rasterization_rate`

| C++ | Rust |
|-----|------|
| `copyParameterDataToBuffer()` | `copy_parameter_data_to_buffer()` |
| `device()` | `device()` |
| `label()` | `label()` |
| `layerCount()` | `layer_count()` |
| `mapPhysicalToScreenCoordinates()` | `map_physical_to_screen_coordinates()` |
| `mapScreenToPhysicalCoordinates()` | `map_screen_to_physical_coordinates()` |
| `parameterBufferSizeAndAlign()` | `parameter_buffer_size_and_align()` |
| `physicalGranularity()` | `physical_granularity()` |
| `physicalSize()` | `physical_size()` |
| `screenSize()` | `screen_size()` |

### RasterizationRateMapDescriptor

**Rust:** `RasterizationRateMapDescriptor` in `metal::src::rasterization_rate`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `label()` | `label()` |
| `layer()` | `layer()` |
| `layerCount()` | `layer_count()` |
| `layers()` | `layers()` |
| `rasterizationRateMapDescriptor()` | `new()` |
| `rasterizationRateMapDescriptor()` | `new()` |
| `rasterizationRateMapDescriptor()` | `new()` |
| `screenSize()` | `screen_size()` |
| `setLabel()` | `set_label()` |
| `setLayer()` | `set_layer()` |
| `setScreenSize()` | `set_screen_size()` |

### RasterizationRateSampleArray

**Rust:** `RasterizationRateSampleArray` in `metal::src::rasterization_rate`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `init()` | `n/a()` |
| `object()` | `object_raw()` |
| `setObject()` | `set_object_raw()` |

### RenderCommandEncoder

**Rust:** `RenderCommandEncoder` in `metal::src::encoder::render_encoder`

| C++ | Rust |
|-----|------|
| `dispatchThreadsPerTile()` | `dispatch_threads_per_tile()` |
| `drawIndexedPrimitives()` | `draw_indexed_primitives()` |
| `drawIndexedPrimitives()` | `draw_indexed_primitives()` |
| `drawIndexedPrimitives()` | `draw_indexed_primitives()` |
| `drawIndexedPrimitives()` | `draw_indexed_primitives()` |
| `drawMeshThreadgroups()` | `draw_mesh_threadgroups()` |
| `drawMeshThreadgroups()` | `draw_mesh_threadgroups()` |
| `drawMeshThreads()` | `draw_mesh_threads()` |
| `drawPrimitives()` | `draw_primitives()` |
| `drawPrimitives()` | `draw_primitives()` |
| `drawPrimitives()` | `draw_primitives()` |
| `drawPrimitives()` | `draw_primitives()` |
| `executeCommandsInBuffer()` | `execute_commands_in_buffer_ptr()` |
| `executeCommandsInBuffer()` | `execute_commands_in_buffer_ptr()` |
| `setArgumentTable()` | `set_tile_argument_table()` |
| `setBlendColor()` | `set_blend_color()` |
| `setColorAttachmentMap()` | `set_color_attachment_map_ptr()` |
| `setColorStoreAction()` | `set_color_store_action()` |
| `setCullMode()` | `set_cull_mode()` |
| `setDepthBias()` | `set_depth_bias()` |
| `setDepthClipMode()` | `set_depth_clip_mode()` |
| `setDepthStencilState()` | `set_depth_stencil_state()` |
| `setDepthStoreAction()` | `set_depth_store_action()` |
| `setDepthTestBounds()` | `set_depth_test_bounds()` |
| `setFrontFacingWinding()` | `set_front_facing_winding()` |
| `setObjectThreadgroupMemoryLength()` | `set_object_threadgroup_memory_length()` |
| `setRenderPipelineState()` | `set_render_pipeline_state()` |
| `setScissorRect()` | `set_scissor_rect()` |
| `setScissorRects()` | `set_scissor_rects()` |
| `setStencilReferenceValue()` | `set_stencil_reference_value()` |
| `setStencilReferenceValues()` | `set_stencil_reference_values()` |
| `setStencilStoreAction()` | `set_stencil_store_action()` |
| `setThreadgroupMemoryLength()` | `set_threadgroup_memory_length()` |
| `setTriangleFillMode()` | `set_triangle_fill_mode()` |
| `setVertexAmplificationCount()` | `set_vertex_amplification_count()` |
| `setViewport()` | `set_viewport()` |
| `setViewports()` | `set_viewports()` |
| `setVisibilityResultMode()` | `set_visibility_result_mode()` |
| `tileHeight()` | `tile_height()` |
| `tileWidth()` | `tile_width()` |
| `writeTimestamp()` | `write_timestamp()` |

### RenderCommandEncoder

**Rust:** `RenderCommandEncoder` in `metal::src::encoder::render_encoder`

| C++ | Rust |
|-----|------|
| `dispatchThreadsPerTile()` | `dispatch_threads_per_tile()` |
| `drawIndexedPatches()` | `draw_indexed_patches()` |
| `drawIndexedPatches()` | `draw_indexed_patches()` |
| `drawIndexedPrimitives()` | `draw_indexed_primitives()` |
| `drawIndexedPrimitives()` | `draw_indexed_primitives()` |
| `drawIndexedPrimitives()` | `draw_indexed_primitives()` |
| `drawIndexedPrimitives()` | `draw_indexed_primitives()` |
| `drawMeshThreadgroups()` | `draw_mesh_threadgroups()` |
| `drawMeshThreadgroups()` | `draw_mesh_threadgroups()` |
| `drawMeshThreads()` | `draw_mesh_threads()` |
| `drawPatches()` | `draw_patches()` |
| `drawPatches()` | `draw_patches()` |
| `drawPrimitives()` | `draw_primitives()` |
| `drawPrimitives()` | `draw_primitives()` |
| `drawPrimitives()` | `draw_primitives()` |
| `drawPrimitives()` | `draw_primitives()` |
| `executeCommandsInBuffer()` | `execute_commands_in_buffer_ptr()` |
| `executeCommandsInBuffer()` | `execute_commands_in_buffer_ptr()` |
| `memoryBarrier()` | `memory_barrier_with_scope()` |
| `memoryBarrier()` | `memory_barrier_with_scope()` |
| `sampleCountersInBuffer()` | `sample_counters_in_buffer_ptr()` |
| `setBlendColor()` | `set_blend_color()` |
| `setColorAttachmentMap()` | `set_color_attachment_map_ptr()` |
| `setColorStoreAction()` | `set_color_store_action()` |
| `setColorStoreActionOptions()` | `set_color_store_action_options()` |
| `setCullMode()` | `set_cull_mode()` |
| `setDepthBias()` | `set_depth_bias()` |
| `setDepthClipMode()` | `set_depth_clip_mode()` |
| `setDepthStencilState()` | `set_depth_stencil_state()` |
| `setDepthStoreAction()` | `set_depth_store_action()` |
| `setDepthStoreActionOptions()` | `set_depth_store_action_options()` |
| `setDepthTestBounds()` | `set_depth_test_bounds()` |
| `setFragmentAccelerationStructure()` | `set_fragment_acceleration_structure()` |
| `setFragmentBuffer()` | `set_fragment_buffer()` |
| `setFragmentBufferOffset()` | `set_fragment_buffer_offset()` |
| `setFragmentBuffers()` | `set_fragment_buffer()` |
| `setFragmentBytes()` | `set_fragment_bytes()` |
| `setFragmentIntersectionFunctionTable()` | `set_fragment_intersection_function_table_ptr()` |
| `setFragmentIntersectionFunctionTables()` | `set_fragment_intersection_function_table_ptr()` |
| `setFragmentSamplerState()` | `set_fragment_sampler_state()` |
| `setFragmentSamplerState()` | `set_fragment_sampler_state()` |
| `setFragmentSamplerStates()` | `set_fragment_sampler_state()` |
| `setFragmentSamplerStates()` | `set_fragment_sampler_states_ptr()` |
| `setFragmentTexture()` | `set_fragment_texture()` |
| `setFragmentTextures()` | `set_fragment_texture()` |
| `setFragmentVisibleFunctionTable()` | `set_fragment_visible_function_table_ptr()` |
| `setFragmentVisibleFunctionTables()` | `set_fragment_visible_function_table_ptr()` |
| `setFrontFacingWinding()` | `set_front_facing_winding()` |
| `setMeshBuffer()` | `set_mesh_buffer()` |
| `setMeshBufferOffset()` | `set_mesh_buffer_offset()` |
| `setMeshBuffers()` | `set_mesh_buffer()` |
| `setMeshBytes()` | `set_mesh_bytes()` |
| `setMeshSamplerState()` | `set_mesh_sampler_state()` |
| `setMeshSamplerState()` | `set_mesh_sampler_state()` |
| `setMeshSamplerStates()` | `set_mesh_sampler_state()` |
| `setMeshSamplerStates()` | `set_mesh_sampler_states_ptr()` |
| `setMeshTexture()` | `set_mesh_texture()` |
| `setMeshTextures()` | `set_mesh_texture()` |
| `setObjectBuffer()` | `set_object_buffer()` |
| `setObjectBufferOffset()` | `set_object_buffer_offset()` |
| `setObjectBuffers()` | `set_object_buffer()` |
| `setObjectBytes()` | `set_object_bytes()` |
| `setObjectSamplerState()` | `set_object_sampler_state()` |
| `setObjectSamplerState()` | `set_object_sampler_state()` |
| `setObjectSamplerStates()` | `set_object_sampler_state()` |
| `setObjectSamplerStates()` | `set_object_sampler_states_ptr()` |
| `setObjectTexture()` | `set_object_texture()` |
| `setObjectTextures()` | `set_object_texture()` |
| `setObjectThreadgroupMemoryLength()` | `set_object_threadgroup_memory_length()` |
| `setRenderPipelineState()` | `set_render_pipeline_state()` |
| `setScissorRect()` | `set_scissor_rect()` |
| `setScissorRects()` | `set_scissor_rects()` |
| `setStencilReferenceValue()` | `set_stencil_reference_value()` |
| `setStencilReferenceValues()` | `set_stencil_reference_values()` |
| `setStencilStoreAction()` | `set_stencil_store_action()` |
| `setStencilStoreActionOptions()` | `set_stencil_store_action_options()` |
| `setTessellationFactorBuffer()` | `set_tessellation_factor_buffer()` |
| `setTessellationFactorScale()` | `set_tessellation_factor_scale()` |
| `setThreadgroupMemoryLength()` | `set_threadgroup_memory_length()` |
| `setTileAccelerationStructure()` | `set_tile_acceleration_structure()` |
| `setTileBuffer()` | `set_tile_buffer()` |
| `setTileBufferOffset()` | `set_tile_buffer_offset()` |
| `setTileBuffers()` | `set_tile_buffer()` |
| `setTileBytes()` | `set_tile_bytes()` |
| `setTileIntersectionFunctionTable()` | `set_tile_intersection_function_table_ptr()` |
| `setTileIntersectionFunctionTables()` | `set_tile_intersection_function_table_ptr()` |
| `setTileSamplerState()` | `set_tile_sampler_state()` |
| `setTileSamplerState()` | `set_tile_sampler_state()` |
| `setTileSamplerStates()` | `set_tile_sampler_state()` |
| `setTileSamplerStates()` | `set_tile_sampler_states_ptr()` |
| `setTileTexture()` | `set_tile_texture()` |
| `setTileTextures()` | `set_tile_texture()` |
| `setTileVisibleFunctionTable()` | `set_tile_visible_function_table_ptr()` |
| `setTileVisibleFunctionTables()` | `set_tile_visible_function_table_ptr()` |
| `setTriangleFillMode()` | `set_triangle_fill_mode()` |
| `setVertexAccelerationStructure()` | `set_vertex_acceleration_structure()` |
| `setVertexAmplificationCount()` | `set_vertex_amplification_count()` |
| `setVertexBuffer()` | `set_vertex_buffer()` |
| `setVertexBuffer()` | `set_vertex_buffer()` |
| `setVertexBufferOffset()` | `set_vertex_buffer_offset()` |
| `setVertexBufferOffset()` | `set_vertex_buffer_offset()` |
| `setVertexBuffers()` | `set_vertex_buffer()` |
| `setVertexBuffers()` | `set_vertex_buffer()` |
| `setVertexBytes()` | `set_vertex_bytes()` |
| `setVertexBytes()` | `set_vertex_bytes()` |
| `setVertexIntersectionFunctionTable()` | `set_vertex_intersection_function_table_ptr()` |
| `setVertexIntersectionFunctionTables()` | `set_vertex_intersection_function_table_ptr()` |
| `setVertexSamplerState()` | `set_vertex_sampler_state()` |
| `setVertexSamplerState()` | `set_vertex_sampler_state()` |
| `setVertexSamplerStates()` | `set_vertex_sampler_state()` |
| `setVertexSamplerStates()` | `set_vertex_sampler_states_ptr()` |
| `setVertexTexture()` | `set_vertex_texture()` |
| `setVertexTextures()` | `set_vertex_texture()` |
| `setVertexVisibleFunctionTable()` | `set_vertex_visible_function_table_ptr()` |
| `setVertexVisibleFunctionTables()` | `set_vertex_visible_function_table_ptr()` |
| `setViewport()` | `set_viewport()` |
| `setViewports()` | `set_viewports()` |
| `setVisibilityResultMode()` | `set_visibility_result_mode()` |
| `textureBarrier()` | `texture_barrier()` |
| `tileHeight()` | `tile_height()` |
| `tileWidth()` | `tile_width()` |
| `updateFence()` | `update_fence()` |
| `useHeap()` | `use_heap()` |
| `useHeap()` | `use_heap()` |
| `useHeaps()` | `use_heap()` |
| `useHeaps()` | `use_heap()` |
| `useResource()` | `use_resource()` |
| `useResource()` | `use_resource()` |
| `useResources()` | `use_resource()` |
| `useResources()` | `use_resources_ptr()` |
| `waitForFence()` | `wait_for_fence()` |

### RenderPassAttachmentDescriptor

**Rust:** `RenderPassAttachmentDescriptor` in `metal::src::pass::attachment`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `depthPlane()` | `depth_plane()` |
| `init()` | `n/a()` |
| `level()` | `level()` |
| `loadAction()` | `load_action()` |
| `resolveDepthPlane()` | `resolve_depth_plane()` |
| `resolveLevel()` | `resolve_level()` |
| `resolveSlice()` | `resolve_slice()` |
| `resolveTexture()` | `resolve_texture()` |
| `setDepthPlane()` | `set_depth_plane()` |
| `setLevel()` | `set_level()` |
| `setLoadAction()` | `set_load_action()` |
| `setResolveDepthPlane()` | `set_resolve_depth_plane()` |
| `setResolveLevel()` | `set_resolve_level()` |
| `setResolveSlice()` | `set_resolve_slice()` |
| `setResolveTexture()` | `set_resolve_texture()` |
| `setSlice()` | `set_slice()` |
| `setStoreAction()` | `set_store_action()` |
| `setStoreActionOptions()` | `set_store_action_options()` |
| `setTexture()` | `set_texture()` |
| `slice()` | `slice()` |
| `storeAction()` | `store_action()` |
| `storeActionOptions()` | `store_action_options()` |
| `texture()` | `texture()` |

### RenderPassColorAttachmentDescriptor

**Rust:** `RenderPassColorAttachmentDescriptor` in `metal::src::pass::color_attachment`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `clearColor()` | `clear_color()` |
| `init()` | `n/a()` |
| `setClearColor()` | `set_clear_color()` |

### RenderPassColorAttachmentDescriptorArray

**Rust:** `RenderPassColorAttachmentDescriptorArray` in `metal::src::pass::color_attachment`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `init()` | `n/a()` |
| `object()` | `object_at()` |
| `setObject()` | `set_object_at()` |

### RenderPassDepthAttachmentDescriptor

**Rust:** `RenderPassDepthAttachmentDescriptor` in `metal::src::pass::depth_attachment`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `clearDepth()` | `clear_depth()` |
| `depthResolveFilter()` | `depth_resolve_filter()` |
| `init()` | `n/a()` |
| `setClearDepth()` | `set_clear_depth()` |
| `setDepthResolveFilter()` | `set_depth_resolve_filter()` |

### RenderPassDescriptor

**Rust:** `RenderPassDescriptor` in `metal::src::mtl4::render_pass`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `colorAttachments()` | `color_attachments()` |
| `defaultRasterSampleCount()` | `default_raster_sample_count()` |
| `depthAttachment()` | `depth_attachment()` |
| `getSamplePositions()` | `get_sample_positions()` |
| `imageblockSampleLength()` | `imageblock_sample_length()` |
| `init()` | `new()` |
| `rasterizationRateMap()` | `rasterization_rate_map()` |
| `renderTargetArrayLength()` | `render_target_array_length()` |
| `renderTargetHeight()` | `render_target_height()` |
| `renderTargetWidth()` | `render_target_width()` |
| `setDefaultRasterSampleCount()` | `set_default_raster_sample_count()` |
| `setDepthAttachment()` | `set_depth_attachment()` |
| `setImageblockSampleLength()` | `set_imageblock_sample_length()` |
| `setRasterizationRateMap()` | `set_rasterization_rate_map()` |
| `setRenderTargetArrayLength()` | `set_render_target_array_length()` |
| `setRenderTargetHeight()` | `set_render_target_height()` |
| `setRenderTargetWidth()` | `set_render_target_width()` |
| `setSamplePositions()` | `set_sample_positions()` |
| `setStencilAttachment()` | `set_stencil_attachment()` |
| `setSupportColorAttachmentMapping()` | `set_support_color_attachment_mapping()` |
| `setThreadgroupMemoryLength()` | `set_threadgroup_memory_length()` |
| `setTileHeight()` | `set_tile_height()` |
| `setTileWidth()` | `set_tile_width()` |
| `setVisibilityResultBuffer()` | `set_visibility_result_buffer()` |
| `setVisibilityResultType()` | `set_visibility_result_type()` |
| `stencilAttachment()` | `stencil_attachment()` |
| `supportColorAttachmentMapping()` | `support_color_attachment_mapping()` |
| `threadgroupMemoryLength()` | `threadgroup_memory_length()` |
| `tileHeight()` | `tile_height()` |
| `tileWidth()` | `tile_width()` |
| `visibilityResultBuffer()` | `visibility_result_buffer()` |
| `visibilityResultType()` | `visibility_result_type()` |

### RenderPassDescriptor

**Rust:** `RenderPassDescriptor` in `metal::src::mtl4::render_pass`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `colorAttachments()` | `color_attachments()` |
| `defaultRasterSampleCount()` | `default_raster_sample_count()` |
| `depthAttachment()` | `depth_attachment()` |
| `getSamplePositions()` | `get_sample_positions()` |
| `imageblockSampleLength()` | `imageblock_sample_length()` |
| `init()` | `new()` |
| `rasterizationRateMap()` | `rasterization_rate_map()` |
| `renderPassDescriptor()` | `new()` |
| `renderTargetArrayLength()` | `render_target_array_length()` |
| `renderTargetHeight()` | `render_target_height()` |
| `renderTargetWidth()` | `render_target_width()` |
| `sampleBufferAttachments()` | `color_attachments()` |
| `setDefaultRasterSampleCount()` | `set_default_raster_sample_count()` |
| `setDepthAttachment()` | `set_depth_attachment()` |
| `setImageblockSampleLength()` | `set_imageblock_sample_length()` |
| `setRasterizationRateMap()` | `set_rasterization_rate_map()` |
| `setRenderTargetArrayLength()` | `set_render_target_array_length()` |
| `setRenderTargetHeight()` | `set_render_target_height()` |
| `setRenderTargetWidth()` | `set_render_target_width()` |
| `setSamplePositions()` | `set_sample_positions()` |
| `setStencilAttachment()` | `set_stencil_attachment()` |
| `setSupportColorAttachmentMapping()` | `set_support_color_attachment_mapping()` |
| `setThreadgroupMemoryLength()` | `set_threadgroup_memory_length()` |
| `setTileHeight()` | `set_tile_height()` |
| `setTileWidth()` | `set_tile_width()` |
| `setVisibilityResultBuffer()` | `set_visibility_result_buffer()` |
| `setVisibilityResultType()` | `set_visibility_result_type()` |
| `stencilAttachment()` | `stencil_attachment()` |
| `supportColorAttachmentMapping()` | `support_color_attachment_mapping()` |
| `threadgroupMemoryLength()` | `threadgroup_memory_length()` |
| `tileHeight()` | `tile_height()` |
| `tileWidth()` | `tile_width()` |
| `visibilityResultBuffer()` | `visibility_result_buffer()` |
| `visibilityResultType()` | `visibility_result_type()` |

### RenderPassSampleBufferAttachmentDescriptor

**Rust:** `RenderPassSampleBufferAttachmentDescriptor` in `metal::src::pass::render_sample_buffer`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `endOfFragmentSampleIndex()` | `end_of_fragment_sample_index()` |
| `endOfVertexSampleIndex()` | `end_of_vertex_sample_index()` |
| `init()` | `new()` |
| `sampleBuffer()` | `sample_buffer()` |
| `setEndOfFragmentSampleIndex()` | `set_end_of_fragment_sample_index()` |
| `setEndOfVertexSampleIndex()` | `set_end_of_vertex_sample_index()` |
| `setSampleBuffer()` | `set_sample_buffer()` |
| `setStartOfFragmentSampleIndex()` | `set_start_of_fragment_sample_index()` |
| `setStartOfVertexSampleIndex()` | `set_start_of_vertex_sample_index()` |
| `startOfFragmentSampleIndex()` | `start_of_fragment_sample_index()` |
| `startOfVertexSampleIndex()` | `start_of_vertex_sample_index()` |

### RenderPassSampleBufferAttachmentDescriptorArray

**Rust:** `RenderPassSampleBufferAttachmentDescriptorArray` in `metal::src::pass::render_sample_buffer`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `object()` | `object()` |
| `setObject()` | `set_object()` |

### RenderPassStencilAttachmentDescriptor

**Rust:** `RenderPassStencilAttachmentDescriptor` in `metal::src::pass::stencil_attachment`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `clearStencil()` | `clear_stencil()` |
| `init()` | `n/a()` |
| `setClearStencil()` | `set_clear_stencil()` |
| `setStencilResolveFilter()` | `set_stencil_resolve_filter()` |
| `stencilResolveFilter()` | `stencil_resolve_filter()` |

### RenderPipelineBinaryFunctionsDescriptor

**Rust:** `RenderPipelineBinaryFunctionsDescriptor` in `metal::src::mtl4::render_pipeline`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `fragmentAdditionalBinaryFunctions()` | `fragment_additional_binary_functions_raw()` |
| `init()` | `new()` |
| `meshAdditionalBinaryFunctions()` | `mesh_additional_binary_functions_raw()` |
| `objectAdditionalBinaryFunctions()` | `object_additional_binary_functions_raw()` |
| `reset()` | `reset()` |
| `setFragmentAdditionalBinaryFunctions()` | `set_fragment_additional_binary_functions_raw()` |
| `setMeshAdditionalBinaryFunctions()` | `set_mesh_additional_binary_functions_raw()` |
| `setObjectAdditionalBinaryFunctions()` | `set_object_additional_binary_functions_raw()` |
| `setTileAdditionalBinaryFunctions()` | `set_tile_additional_binary_functions_raw()` |
| `setVertexAdditionalBinaryFunctions()` | `set_vertex_additional_binary_functions_raw()` |
| `tileAdditionalBinaryFunctions()` | `tile_additional_binary_functions_raw()` |
| `vertexAdditionalBinaryFunctions()` | `vertex_additional_binary_functions_raw()` |

### RenderPipelineColorAttachmentDescriptor

**Rust:** `RenderPipelineColorAttachmentDescriptor` in `metal::src::mtl4::render_pipeline`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `alphaBlendOperation()` | `alpha_blend_operation()` |
| `blendingState()` | `blending_state()` |
| `destinationAlphaBlendFactor()` | `destination_alpha_blend_factor()` |
| `destinationRGBBlendFactor()` | `destination_rgb_blend_factor()` |
| `init()` | `new()` |
| `pixelFormat()` | `pixel_format()` |
| `reset()` | `reset()` |
| `rgbBlendOperation()` | `rgb_blend_operation()` |
| `setAlphaBlendOperation()` | `set_alpha_blend_operation()` |
| `setBlendingState()` | `set_blending_state()` |
| `setDestinationAlphaBlendFactor()` | `set_destination_alpha_blend_factor()` |
| `setDestinationRGBBlendFactor()` | `set_destination_rgb_blend_factor()` |
| `setPixelFormat()` | `set_pixel_format()` |
| `setRgbBlendOperation()` | `set_rgb_blend_operation()` |
| `setSourceAlphaBlendFactor()` | `set_source_alpha_blend_factor()` |
| `setSourceRGBBlendFactor()` | `set_source_rgb_blend_factor()` |
| `setWriteMask()` | `set_write_mask()` |
| `sourceAlphaBlendFactor()` | `source_alpha_blend_factor()` |
| `sourceRGBBlendFactor()` | `source_rgb_blend_factor()` |
| `writeMask()` | `write_mask()` |

### RenderPipelineColorAttachmentDescriptor

**Rust:** `RenderPipelineColorAttachmentDescriptor` in `metal::src::mtl4::render_pipeline`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `alphaBlendOperation()` | `alpha_blend_operation()` |
| `blendingEnabled()` | `is_blending_enabled()` |
| `destinationAlphaBlendFactor()` | `destination_alpha_blend_factor()` |
| `destinationRGBBlendFactor()` | `destination_rgb_blend_factor()` |
| `init()` | `new()` |
| `isBlendingEnabled()` | `is_blending_enabled()` |
| `pixelFormat()` | `pixel_format()` |
| `rgbBlendOperation()` | `rgb_blend_operation()` |
| `setAlphaBlendOperation()` | `set_alpha_blend_operation()` |
| `setBlendingEnabled()` | `set_blending_enabled()` |
| `setDestinationAlphaBlendFactor()` | `set_destination_alpha_blend_factor()` |
| `setDestinationRGBBlendFactor()` | `set_destination_rgb_blend_factor()` |
| `setPixelFormat()` | `set_pixel_format()` |
| `setRgbBlendOperation()` | `set_rgb_blend_operation()` |
| `setSourceAlphaBlendFactor()` | `set_source_alpha_blend_factor()` |
| `setSourceRGBBlendFactor()` | `set_source_rgb_blend_factor()` |
| `setWriteMask()` | `set_write_mask()` |
| `sourceAlphaBlendFactor()` | `source_alpha_blend_factor()` |
| `sourceRGBBlendFactor()` | `source_rgb_blend_factor()` |
| `writeMask()` | `write_mask()` |

### RenderPipelineColorAttachmentDescriptorArray

**Rust:** `RenderPipelineColorAttachmentDescriptorArray` in `metal::src::mtl4::render_pipeline`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `object()` | `object()` |
| `reset()` | `reset()` |
| `setObject()` | `set_object()` |

### RenderPipelineColorAttachmentDescriptorArray

**Rust:** `RenderPipelineColorAttachmentDescriptorArray` in `metal::src::mtl4::render_pipeline`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `object()` | `object()` |
| `setObject()` | `set_object()` |

### RenderPipelineDescriptor

**Rust:** `RenderPipelineDescriptor` in `metal::src::mtl4::render_pipeline`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `alphaToCoverageState()` | `alpha_to_coverage_state()` |
| `alphaToOneState()` | `alpha_to_one_state()` |
| `colorAttachmentMappingState()` | `color_attachment_mapping_state()` |
| `colorAttachments()` | `color_attachments()` |
| `fragmentFunctionDescriptor()` | `fragment_function_descriptor()` |
| `fragmentStaticLinkingDescriptor()` | `fragment_static_linking_descriptor()` |
| `init()` | `new()` |
| `inputPrimitiveTopology()` | `input_primitive_topology()` |
| `isRasterizationEnabled()` | `is_rasterization_enabled()` |
| `maxVertexAmplificationCount()` | `max_vertex_amplification_count()` |
| `rasterSampleCount()` | `raster_sample_count()` |
| `rasterizationEnabled()` | `is_rasterization_enabled()` |
| `reset()` | `reset()` |
| `setAlphaToCoverageState()` | `set_alpha_to_coverage_state()` |
| `setAlphaToOneState()` | `set_alpha_to_one_state()` |
| `setColorAttachmentMappingState()` | `set_color_attachment_mapping_state()` |
| `setFragmentFunctionDescriptor()` | `set_fragment_function_descriptor()` |
| `setFragmentStaticLinkingDescriptor()` | `set_fragment_static_linking_descriptor()` |
| `setInputPrimitiveTopology()` | `set_input_primitive_topology()` |
| `setMaxVertexAmplificationCount()` | `set_max_vertex_amplification_count()` |
| `setRasterSampleCount()` | `set_raster_sample_count()` |
| `setRasterizationEnabled()` | `set_rasterization_enabled()` |
| `setSupportFragmentBinaryLinking()` | `set_support_fragment_binary_linking()` |
| `setSupportIndirectCommandBuffers()` | `set_support_indirect_command_buffers()` |
| `setSupportVertexBinaryLinking()` | `set_support_vertex_binary_linking()` |
| `setVertexDescriptor()` | `set_vertex_descriptor()` |
| `setVertexFunctionDescriptor()` | `set_vertex_function_descriptor()` |
| `setVertexStaticLinkingDescriptor()` | `set_vertex_static_linking_descriptor()` |
| `supportFragmentBinaryLinking()` | `support_fragment_binary_linking()` |
| `supportIndirectCommandBuffers()` | `support_indirect_command_buffers()` |
| `supportVertexBinaryLinking()` | `support_vertex_binary_linking()` |
| `vertexDescriptor()` | `vertex_descriptor()` |
| `vertexFunctionDescriptor()` | `vertex_function_descriptor()` |
| `vertexStaticLinkingDescriptor()` | `vertex_static_linking_descriptor()` |

### RenderPipelineDescriptor

**Rust:** `RenderPipelineDescriptor` in `metal::src::mtl4::render_pipeline`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `alphaToCoverageEnabled()` | `is_alpha_to_coverage_enabled()` |
| `alphaToOneEnabled()` | `is_alpha_to_one_enabled()` |
| `binaryArchives()` | `binary_archives_raw()` |
| `colorAttachments()` | `color_attachments()` |
| `depthAttachmentPixelFormat()` | `depth_attachment_pixel_format()` |
| `fragmentBuffers()` | `fragment_buffers()` |
| `fragmentFunction()` | `fragment_function()` |
| `fragmentLinkedFunctions()` | `fragment_linked_functions()` |
| `fragmentPreloadedLibraries()` | `fragment_preloaded_libraries_raw()` |
| `init()` | `new()` |
| `inputPrimitiveTopology()` | `input_primitive_topology()` |
| `isAlphaToCoverageEnabled()` | `is_alpha_to_coverage_enabled()` |
| `isAlphaToOneEnabled()` | `is_alpha_to_one_enabled()` |
| `isRasterizationEnabled()` | `is_rasterization_enabled()` |
| `isTessellationFactorScaleEnabled()` | `is_tessellation_factor_scale_enabled()` |
| `label()` | `label()` |
| `maxFragmentCallStackDepth()` | `max_fragment_call_stack_depth()` |
| `maxTessellationFactor()` | `max_tessellation_factor()` |
| `maxVertexAmplificationCount()` | `max_vertex_amplification_count()` |
| `maxVertexCallStackDepth()` | `max_vertex_call_stack_depth()` |
| `rasterSampleCount()` | `raster_sample_count()` |
| `rasterizationEnabled()` | `is_rasterization_enabled()` |
| `reset()` | `reset()` |
| `sampleCount()` | `sample_count()` |
| `setAlphaToCoverageEnabled()` | `set_alpha_to_coverage_enabled()` |
| `setAlphaToOneEnabled()` | `set_alpha_to_one_enabled()` |
| `setBinaryArchives()` | `set_binary_archives_raw()` |
| `setDepthAttachmentPixelFormat()` | `set_depth_attachment_pixel_format()` |
| `setFragmentFunction()` | `set_fragment_function()` |
| `setFragmentLinkedFunctions()` | `set_fragment_linked_functions()` |
| `setFragmentPreloadedLibraries()` | `set_fragment_preloaded_libraries_raw()` |
| `setInputPrimitiveTopology()` | `set_input_primitive_topology()` |
| `setLabel()` | `set_label()` |
| `setMaxFragmentCallStackDepth()` | `set_max_fragment_call_stack_depth()` |
| `setMaxTessellationFactor()` | `set_max_tessellation_factor()` |
| `setMaxVertexAmplificationCount()` | `set_max_vertex_amplification_count()` |
| `setMaxVertexCallStackDepth()` | `set_max_vertex_call_stack_depth()` |
| `setRasterSampleCount()` | `set_raster_sample_count()` |
| `setRasterizationEnabled()` | `set_rasterization_enabled()` |
| `setSampleCount()` | `set_sample_count()` |
| `setShaderValidation()` | `set_shader_validation()` |
| `setStencilAttachmentPixelFormat()` | `set_stencil_attachment_pixel_format()` |
| `setSupportAddingFragmentBinaryFunctions()` | `set_support_adding_fragment_binary_functions()` |
| `setSupportAddingVertexBinaryFunctions()` | `set_support_adding_vertex_binary_functions()` |
| `setSupportIndirectCommandBuffers()` | `set_support_indirect_command_buffers()` |
| `setTessellationControlPointIndexType()` | `set_tessellation_control_point_index_type()` |
| `setTessellationFactorFormat()` | `set_tessellation_factor_format()` |
| `setTessellationFactorScaleEnabled()` | `set_tessellation_factor_scale_enabled()` |
| `setTessellationFactorStepFunction()` | `set_tessellation_factor_step_function()` |
| `setTessellationOutputWindingOrder()` | `set_tessellation_output_winding_order()` |
| `setTessellationPartitionMode()` | `set_tessellation_partition_mode()` |
| `setVertexDescriptor()` | `set_vertex_descriptor()` |
| `setVertexFunction()` | `set_vertex_function()` |
| `setVertexLinkedFunctions()` | `set_vertex_linked_functions()` |
| `setVertexPreloadedLibraries()` | `set_vertex_preloaded_libraries_raw()` |
| `shaderValidation()` | `shader_validation()` |
| `stencilAttachmentPixelFormat()` | `stencil_attachment_pixel_format()` |
| `supportAddingFragmentBinaryFunctions()` | `support_adding_fragment_binary_functions()` |
| `supportAddingVertexBinaryFunctions()` | `support_adding_vertex_binary_functions()` |
| `supportIndirectCommandBuffers()` | `support_indirect_command_buffers()` |
| `tessellationControlPointIndexType()` | `tessellation_control_point_index_type()` |
| `tessellationFactorFormat()` | `tessellation_factor_format()` |
| `tessellationFactorScaleEnabled()` | `is_tessellation_factor_scale_enabled()` |
| `tessellationFactorStepFunction()` | `tessellation_factor_step_function()` |
| `tessellationOutputWindingOrder()` | `tessellation_output_winding_order()` |
| `tessellationPartitionMode()` | `tessellation_partition_mode()` |
| `vertexBuffers()` | `vertex_buffers()` |
| `vertexDescriptor()` | `vertex_descriptor()` |
| `vertexFunction()` | `vertex_function()` |
| `vertexLinkedFunctions()` | `vertex_linked_functions()` |
| `vertexPreloadedLibraries()` | `vertex_preloaded_libraries_raw()` |

### RenderPipelineDynamicLinkingDescriptor

**Rust:** `RenderPipelineDynamicLinkingDescriptor` in `metal::src::mtl4::linking_descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `fragmentLinkingDescriptor()` | `fragment_linking_descriptor()` |
| `init()` | `new()` |
| `meshLinkingDescriptor()` | `mesh_linking_descriptor()` |
| `objectLinkingDescriptor()` | `object_linking_descriptor()` |
| `tileLinkingDescriptor()` | `tile_linking_descriptor()` |
| `vertexLinkingDescriptor()` | `vertex_linking_descriptor()` |

### RenderPipelineFunctionsDescriptor

**Rust:** `RenderPipelineFunctionsDescriptor` in `metal::src::pipeline::functions_descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `fragmentAdditionalBinaryFunctions()` | `fragment_additional_binary_functions_raw()` |
| `init()` | `new()` |
| `setFragmentAdditionalBinaryFunctions()` | `set_fragment_additional_binary_functions_raw()` |
| `setTileAdditionalBinaryFunctions()` | `set_tile_additional_binary_functions_raw()` |
| `setVertexAdditionalBinaryFunctions()` | `set_vertex_additional_binary_functions_raw()` |
| `tileAdditionalBinaryFunctions()` | `tile_additional_binary_functions_raw()` |
| `vertexAdditionalBinaryFunctions()` | `vertex_additional_binary_functions_raw()` |

### RenderPipelineReflection

**Rust:** `RenderPipelineReflection` in `metal::src::pipeline::reflection`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `fragmentArguments()` | `fragment_arguments_raw()` |
| `fragmentBindings()` | `fragment_bindings_raw()` |
| `init()` | `new()` |
| `meshBindings()` | `mesh_bindings_raw()` |
| `objectBindings()` | `object_bindings_raw()` |
| `tileArguments()` | `tile_arguments_raw()` |
| `tileBindings()` | `tile_bindings_raw()` |
| `vertexArguments()` | `vertex_arguments_raw()` |
| `vertexBindings()` | `vertex_bindings_raw()` |

### RenderPipelineState

**Rust:** `RenderPipelineState` in `metal::src::pipeline::render_state`

| C++ | Rust |
|-----|------|
| `device()` | `device()` |
| `functionHandle()` | `function_handle_with_name()` |
| `functionHandle()` | `function_handle_with_name()` |
| `functionHandle()` | `function_handle_with_name()` |
| `gpuResourceID()` | `gpu_resource_id()` |
| `imageblockMemoryLength()` | `imageblock_memory_length()` |
| `imageblockSampleLength()` | `imageblock_sample_length()` |
| `label()` | `label()` |
| `maxTotalThreadgroupsPerMeshGrid()` | `max_total_threadgroups_per_mesh_grid()` |
| `maxTotalThreadsPerMeshThreadgroup()` | `max_total_threads_per_mesh_threadgroup()` |
| `maxTotalThreadsPerObjectThreadgroup()` | `max_total_threads_per_object_threadgroup()` |
| `maxTotalThreadsPerThreadgroup()` | `max_total_threads_per_threadgroup()` |
| `meshThreadExecutionWidth()` | `mesh_thread_execution_width()` |
| `newIntersectionFunctionTable()` | `new_intersection_function_table()` |
| `newRenderPipelineDescriptor()` | `new_render_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newRenderPipelineState()` | `new_render_pipeline_state()` |
| `newVisibleFunctionTable()` | `new_visible_function_table()` |
| `objectThreadExecutionWidth()` | `object_thread_execution_width()` |
| `reflection()` | `reflection()` |
| `requiredThreadsPerMeshThreadgroup()` | `required_threads_per_mesh_threadgroup()` |
| `requiredThreadsPerObjectThreadgroup()` | `required_threads_per_object_threadgroup()` |
| `requiredThreadsPerTileThreadgroup()` | `required_threads_per_tile_threadgroup()` |
| `shaderValidation()` | `shader_validation()` |
| `supportIndirectCommandBuffers()` | `support_indirect_command_buffers()` |
| `threadgroupSizeMatchesTileSize()` | `threadgroup_size_matches_tile_size()` |

### ResidencySet

**Rust:** `ResidencySet` in `metal::src::residency_set`

| C++ | Rust |
|-----|------|
| `addAllocation()` | `add_allocation_ptr()` |
| `addAllocations()` | `add_allocations_ptr()` |
| `allAllocations()` | `all_allocations_ptr()` |
| `allocatedSize()` | `allocated_size()` |
| `allocationCount()` | `allocation_count()` |
| `commit()` | `commit()` |
| `containsAllocation()` | `contains_allocation_ptr()` |
| `device()` | `device()` |
| `endResidency()` | `end_residency()` |
| `label()` | `label()` |
| `removeAllAllocations()` | `remove_all_allocations()` |
| `removeAllocation()` | `remove_allocation_ptr()` |
| `removeAllocations()` | `remove_allocations_ptr()` |
| `requestResidency()` | `request_residency()` |

### ResidencySetDescriptor

**Rust:** `ResidencySetDescriptor` in `metal::src::residency_set`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `initialCapacity()` | `initial_capacity()` |
| `label()` | `label()` |
| `setInitialCapacity()` | `set_initial_capacity()` |
| `setLabel()` | `set_label()` |

### ResourceStateCommandEncoder

**Rust:** `ResourceStateCommandEncoder` in `metal::src::encoder::resource_state_encoder`

| C++ | Rust |
|-----|------|
| `moveTextureMappingsFromTexture()` | `move_texture_mappings_from_texture()` |
| `updateFence()` | `update_fence()` |
| `updateTextureMapping()` | `update_texture_mapping()` |
| `updateTextureMapping()` | `update_texture_mapping()` |
| `updateTextureMappings()` | `update_texture_mappings()` |
| `waitForFence()` | `wait_for_fence()` |

### ResourceStatePassDescriptor

**Rust:** `ResourceStatePassDescriptor` in `metal::src::pass::resource_state`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `resourceStatePassDescriptor()` | `new()` |
| `sampleBufferAttachments()` | `sample_buffer_attachments()` |

### ResourceStatePassSampleBufferAttachmentDescriptor

**Rust:** `ResourceStatePassSampleBufferAttachmentDescriptor` in `metal::src::pass::resource_state`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `endOfEncoderSampleIndex()` | `end_of_encoder_sample_index()` |
| `init()` | `new()` |
| `sampleBuffer()` | `sample_buffer()` |
| `setEndOfEncoderSampleIndex()` | `set_end_of_encoder_sample_index()` |
| `setSampleBuffer()` | `set_sample_buffer()` |
| `setStartOfEncoderSampleIndex()` | `set_start_of_encoder_sample_index()` |
| `startOfEncoderSampleIndex()` | `start_of_encoder_sample_index()` |

### ResourceStatePassSampleBufferAttachmentDescriptorArray

**Rust:** `ResourceStatePassSampleBufferAttachmentDescriptorArray` in `metal::src::pass::resource_state`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `init()` | `n/a()` |
| `object()` | `object()` |
| `setObject()` | `set_object()` |

### ResourceViewPoolDescriptor

**Rust:** `ResourceViewPoolDescriptor` in `metal::src::texture_view_pool`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `label()` | `label()` |
| `resourceViewCount()` | `resource_view_count()` |
| `setLabel()` | `set_label()` |
| `setResourceViewCount()` | `set_resource_view_count()` |

### SamplerDescriptor

**Rust:** `SamplerDescriptor` in `metal::src::sampler`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `borderColor()` | `border_color()` |
| `compareFunction()` | `compare_function()` |
| `init()` | `new()` |
| `label()` | `label()` |
| `lodAverage()` | `lod_average()` |
| `lodBias()` | `lod_bias()` |
| `lodMaxClamp()` | `lod_max_clamp()` |
| `lodMinClamp()` | `lod_min_clamp()` |
| `magFilter()` | `mag_filter()` |
| `maxAnisotropy()` | `max_anisotropy()` |
| `minFilter()` | `min_filter()` |
| `mipFilter()` | `mip_filter()` |
| `normalizedCoordinates()` | `normalized_coordinates()` |
| `rAddressMode()` | `r_address_mode()` |
| `reductionMode()` | `reduction_mode()` |
| `sAddressMode()` | `s_address_mode()` |
| `setBorderColor()` | `set_border_color()` |
| `setCompareFunction()` | `set_compare_function()` |
| `setLabel()` | `set_label()` |
| `setLodAverage()` | `set_lod_average()` |
| `setLodBias()` | `set_lod_bias()` |
| `setLodMaxClamp()` | `set_lod_max_clamp()` |
| `setLodMinClamp()` | `set_lod_min_clamp()` |
| `setMagFilter()` | `set_mag_filter()` |
| `setMaxAnisotropy()` | `set_max_anisotropy()` |
| `setMinFilter()` | `set_min_filter()` |
| `setMipFilter()` | `set_mip_filter()` |
| `setNormalizedCoordinates()` | `set_normalized_coordinates()` |
| `setRAddressMode()` | `set_r_address_mode()` |
| `setReductionMode()` | `set_reduction_mode()` |
| `setSAddressMode()` | `set_s_address_mode()` |
| `setSupportArgumentBuffers()` | `set_support_argument_buffers()` |
| `setTAddressMode()` | `set_t_address_mode()` |
| `supportArgumentBuffers()` | `support_argument_buffers()` |
| `tAddressMode()` | `t_address_mode()` |

### SamplerState

**Rust:** `SamplerState` in `metal::src::sampler`

| C++ | Rust |
|-----|------|
| `device()` | `device()` |
| `gpuResourceID()` | `gpu_resource_id()` |
| `label()` | `label()` |

### Set

**Rust:** `Set` in `metal-foundation::src::set`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `count()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `init()` | `n/a()` |
| `objectEnumerator()` | `n/a()` |

### SharedEvent

**Rust:** `SharedEvent` in `metal::src::sync`

| C++ | Rust |
|-----|------|
| `newSharedEventHandle()` | `new_shared_event_handle()` |
| `notifyListener()` | `notify_listener()` |
| `notifyListener()` | `notify_listener()` |
| `setSignaledValue()` | `set_signaled_value()` |
| `signaledValue()` | `signaled_value()` |
| `waitUntilSignaledValue()` | `set_signaled_value()` |

### SharedEventHandle

**Rust:** `SharedEventHandle` in `metal::src::sync`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `init()` | `n/a()` |
| `label()` | `label()` |

### SharedEventListener

**Rust:** `SharedEventListener` in `metal::src::sync`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `dispatchQueue()` | `dispatch_queue_raw()` |
| `init()` | `new()` |
| `init()` | `new()` |
| `sharedListener()` | `shared()` |

### SharedTextureHandle

**Rust:** `SharedTextureHandle` in `metal::src::texture::shared_handle`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `device()` | `device()` |
| `init()` | `n/a()` |
| `label()` | `label()` |

### SpatialScaler

**Rust:** `SpatialScaler` in `metal-fx::src::spatial_scaler`

| C++ | Rust |
|-----|------|
| `encodeToCommandBuffer()` | `encode_to_command_buffer()` |

### SpatialScaler

**Rust:** `SpatialScaler` in `metal-fx::src::spatial_scaler`

| C++ | Rust |
|-----|------|
| `encodeToCommandBuffer()` | `encode_to_command_buffer()` |

### SpatialScalerDescriptor

**Rust:** `SpatialScalerDescriptor` in `metal-fx::src::spatial_scaler`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `colorProcessingMode()` | `color_processing_mode()` |
| `colorTextureFormat()` | `color_texture_format()` |
| `init()` | `new()` |
| `inputHeight()` | `input_height()` |
| `inputWidth()` | `input_width()` |
| `newSpatialScaler()` | `new_spatial_scaler()` |
| `newSpatialScaler()` | `new_spatial_scaler()` |
| `outputHeight()` | `output_height()` |
| `outputTextureFormat()` | `output_texture_format()` |
| `outputWidth()` | `output_width()` |
| `setColorProcessingMode()` | `set_color_processing_mode()` |
| `setColorTextureFormat()` | `set_color_texture_format()` |
| `setInputHeight()` | `set_input_height()` |
| `setInputWidth()` | `set_input_width()` |
| `setOutputHeight()` | `set_output_height()` |
| `setOutputTextureFormat()` | `set_output_texture_format()` |
| `setOutputWidth()` | `set_output_width()` |
| `supportsDevice()` | `supports_device()` |
| `supportsMetal4FX()` | `supports_device()` |

### SpecializedFunctionDescriptor

**Rust:** `SpecializedFunctionDescriptor` in `metal::src::mtl4::specialized_function_descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `constantValues()` | `constant_values_raw()` |
| `functionDescriptor()` | `function_descriptor()` |
| `init()` | `new()` |
| `setConstantValues()` | `set_constant_values_raw()` |
| `setFunctionDescriptor()` | `set_function_descriptor()` |
| `setSpecializedName()` | `set_specialized_name()` |
| `specializedName()` | `specialized_name()` |

### StageInputOutputDescriptor

**Rust:** `StageInputOutputDescriptor` in `metal::src::stage_input_output`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `attributes()` | `attributes()` |
| `indexBufferIndex()` | `index_buffer_index()` |
| `indexType()` | `index_type()` |
| `init()` | `new()` |
| `layouts()` | `layouts()` |
| `reset()` | `reset()` |
| `setIndexBufferIndex()` | `set_index_buffer_index()` |
| `setIndexType()` | `set_index_type()` |
| `stageInputOutputDescriptor()` | `new()` |

### StaticLinkingDescriptor

**Rust:** `StaticLinkingDescriptor` in `metal::src::mtl4::linking_descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `functionDescriptors()` | `function_descriptors_raw()` |
| `groups()` | `groups_raw()` |
| `init()` | `new()` |
| `privateFunctionDescriptors()` | `private_function_descriptors_raw()` |
| `setFunctionDescriptors()` | `set_function_descriptors_raw()` |
| `setGroups()` | `set_groups_raw()` |
| `setPrivateFunctionDescriptors()` | `set_private_function_descriptors_raw()` |

### StencilDescriptor

**Rust:** `StencilDescriptor` in `metal::src::depth_stencil`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `depthFailureOperation()` | `depth_failure_operation()` |
| `depthStencilPassOperation()` | `depth_stencil_pass_operation()` |
| `init()` | `new()` |
| `readMask()` | `read_mask()` |
| `setDepthFailureOperation()` | `set_depth_failure_operation()` |
| `setDepthStencilPassOperation()` | `set_depth_stencil_pass_operation()` |
| `setReadMask()` | `set_read_mask()` |
| `setStencilCompareFunction()` | `set_stencil_compare_function()` |
| `setStencilFailureOperation()` | `set_stencil_failure_operation()` |
| `setWriteMask()` | `set_write_mask()` |
| `stencilCompareFunction()` | `stencil_compare_function()` |
| `stencilFailureOperation()` | `stencil_failure_operation()` |
| `writeMask()` | `write_mask()` |

### StitchedFunctionDescriptor

**Rust:** `StitchedFunctionDescriptor` in `metal::src::mtl4::stitched_function_descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `functionDescriptors()` | `function_descriptors_raw()` |
| `functionGraph()` | `function_graph()` |
| `init()` | `new()` |
| `setFunctionDescriptors()` | `set_function_descriptors_raw()` |
| `setFunctionGraph()` | `set_function_graph()` |

### StitchedLibraryDescriptor

**Rust:** `StitchedLibraryDescriptor` in `metal::src::function_stitching`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `binaryArchives()` | `binary_archives_ptr()` |
| `functionGraphs()` | `function_graphs_ptr()` |
| `functions()` | `functions_ptr()` |
| `init()` | `new()` |
| `options()` | `options()` |
| `setBinaryArchives()` | `set_binary_archives_ptr()` |
| `setFunctionGraphs()` | `set_function_graphs_ptr()` |
| `setFunctions()` | `set_functions_ptr()` |
| `setOptions()` | `set_options()` |

### StructMember

**Rust:** `StructMember` in `metal::src::argument::struct_member`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `argumentIndex()` | `argument_index()` |
| `arrayType()` | `array_type()` |
| `dataType()` | `data_type()` |
| `init()` | `n/a()` |
| `name()` | `name()` |
| `offset()` | `offset()` |
| `pointerType()` | `pointer_type()` |
| `structType()` | `struct_type()` |
| `tensorReferenceType()` | `tensor_reference_type()` |
| `textureReferenceType()` | `texture_reference_type()` |

### StructType

**Rust:** `StructType` in `metal::src::argument::struct_type`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `init()` | `n/a()` |
| `memberByName()` | `member_by_name()` |
| `members()` | `members_ptr()` |

### TemporalDenoisedScaler

**Rust:** `TemporalDenoisedScaler` in `metal-fx::src::temporal_denoised_scaler`

| C++ | Rust |
|-----|------|
| `encodeToCommandBuffer()` | `encode_to_command_buffer()` |

### TemporalDenoisedScaler

**Rust:** `TemporalDenoisedScaler` in `metal-fx::src::temporal_denoised_scaler`

| C++ | Rust |
|-----|------|
| `encodeToCommandBuffer()` | `encode_to_command_buffer()` |

### TemporalDenoisedScalerDescriptor

**Rust:** `TemporalDenoisedScalerDescriptor` in `metal-fx::src::temporal_denoised_scaler`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `colorTextureFormat()` | `color_texture_format()` |
| `denoiseStrengthMaskTextureFormat()` | `denoise_strength_mask_texture_format()` |
| `depthTextureFormat()` | `depth_texture_format()` |
| `diffuseAlbedoTextureFormat()` | `diffuse_albedo_texture_format()` |
| `init()` | `new()` |
| `inputContentMaxScale()` | `input_content_max_scale()` |
| `inputContentMinScale()` | `input_content_min_scale()` |
| `inputHeight()` | `input_height()` |
| `inputWidth()` | `input_width()` |
| `isAutoExposureEnabled()` | `is_auto_exposure_enabled()` |
| `isDenoiseStrengthMaskTextureEnabled()` | `is_denoise_strength_mask_texture_enabled()` |
| `isInputContentPropertiesEnabled()` | `is_input_content_properties_enabled()` |
| `isReactiveMaskTextureEnabled()` | `is_reactive_mask_texture_enabled()` |
| `isSpecularHitDistanceTextureEnabled()` | `is_specular_hit_distance_texture_enabled()` |
| `isTransparencyOverlayTextureEnabled()` | `is_transparency_overlay_texture_enabled()` |
| `motionTextureFormat()` | `motion_texture_format()` |
| `newTemporalDenoisedScaler()` | `new_temporal_denoised_scaler()` |
| `newTemporalDenoisedScaler()` | `new_temporal_denoised_scaler()` |
| `normalTextureFormat()` | `normal_texture_format()` |
| `outputHeight()` | `output_height()` |
| `outputTextureFormat()` | `output_texture_format()` |
| `outputWidth()` | `output_width()` |
| `reactiveMaskTextureFormat()` | `reactive_mask_texture_format()` |
| `requiresSynchronousInitialization()` | `requires_synchronous_initialization()` |
| `roughnessTextureFormat()` | `roughness_texture_format()` |
| `setAutoExposureEnabled()` | `set_auto_exposure_enabled()` |
| `setColorTextureFormat()` | `set_color_texture_format()` |
| `setDenoiseStrengthMaskTextureEnabled()` | `set_denoise_strength_mask_texture_enabled()` |
| `setDenoiseStrengthMaskTextureFormat()` | `set_denoise_strength_mask_texture_format()` |
| `setDepthTextureFormat()` | `set_depth_texture_format()` |
| `setDiffuseAlbedoTextureFormat()` | `set_diffuse_albedo_texture_format()` |
| `setInputContentMaxScale()` | `set_input_content_max_scale()` |
| `setInputContentMinScale()` | `set_input_content_min_scale()` |
| `setInputContentPropertiesEnabled()` | `set_input_content_properties_enabled()` |
| `setInputHeight()` | `set_input_height()` |
| `setInputWidth()` | `set_input_width()` |
| `setMotionTextureFormat()` | `set_motion_texture_format()` |
| `setNormalTextureFormat()` | `set_normal_texture_format()` |
| `setOutputHeight()` | `set_output_height()` |
| `setOutputTextureFormat()` | `set_output_texture_format()` |
| `setOutputWidth()` | `set_output_width()` |
| `setReactiveMaskTextureEnabled()` | `set_reactive_mask_texture_enabled()` |
| `setReactiveMaskTextureFormat()` | `set_reactive_mask_texture_format()` |
| `setRequiresSynchronousInitialization()` | `set_requires_synchronous_initialization()` |
| `setRoughnessTextureFormat()` | `set_roughness_texture_format()` |
| `setSpecularAlbedoTextureFormat()` | `set_specular_albedo_texture_format()` |
| `setSpecularHitDistanceTextureEnabled()` | `set_specular_hit_distance_texture_enabled()` |
| `setSpecularHitDistanceTextureFormat()` | `set_specular_hit_distance_texture_format()` |
| `setTransparencyOverlayTextureEnabled()` | `set_transparency_overlay_texture_enabled()` |
| `setTransparencyOverlayTextureFormat()` | `set_transparency_overlay_texture_format()` |
| `specularAlbedoTextureFormat()` | `specular_albedo_texture_format()` |
| `specularHitDistanceTextureFormat()` | `specular_hit_distance_texture_format()` |
| `supportedInputContentMaxScale()` | `supported_input_content_max_scale()` |
| `supportedInputContentMinScale()` | `supported_input_content_min_scale()` |
| `supportsDevice()` | `supports_device()` |
| `supportsMetal4FX()` | `supports_metal4_fx()` |
| `transparencyOverlayTextureFormat()` | `transparency_overlay_texture_format()` |

### TemporalScaler

**Rust:** `TemporalScaler` in `metal-fx::src::temporal_scaler`

| C++ | Rust |
|-----|------|
| `encodeToCommandBuffer()` | `encode_to_command_buffer()` |

### TemporalScaler

**Rust:** `TemporalScaler` in `metal-fx::src::temporal_scaler`

| C++ | Rust |
|-----|------|
| `encodeToCommandBuffer()` | `encode_to_command_buffer()` |

### TemporalScalerDescriptor

**Rust:** `TemporalScalerDescriptor` in `metal-fx::src::temporal_scaler`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `colorTextureFormat()` | `color_texture_format()` |
| `depthTextureFormat()` | `depth_texture_format()` |
| `init()` | `new()` |
| `inputContentMaxScale()` | `input_content_max_scale()` |
| `inputContentMinScale()` | `input_content_min_scale()` |
| `inputHeight()` | `input_height()` |
| `inputWidth()` | `input_width()` |
| `isAutoExposureEnabled()` | `is_auto_exposure_enabled()` |
| `isInputContentPropertiesEnabled()` | `is_input_content_properties_enabled()` |
| `isReactiveMaskTextureEnabled()` | `is_reactive_mask_texture_enabled()` |
| `motionTextureFormat()` | `motion_texture_format()` |
| `newTemporalScaler()` | `new_temporal_scaler()` |
| `newTemporalScaler()` | `new_temporal_scaler()` |
| `outputHeight()` | `output_height()` |
| `outputTextureFormat()` | `output_texture_format()` |
| `outputWidth()` | `output_width()` |
| `reactiveMaskTextureFormat()` | `reactive_mask_texture_format()` |
| `requiresSynchronousInitialization()` | `requires_synchronous_initialization()` |
| `setAutoExposureEnabled()` | `set_auto_exposure_enabled()` |
| `setColorTextureFormat()` | `set_color_texture_format()` |
| `setDepthTextureFormat()` | `set_depth_texture_format()` |
| `setInputContentMaxScale()` | `set_input_content_max_scale()` |
| `setInputContentMinScale()` | `set_input_content_min_scale()` |
| `setInputContentPropertiesEnabled()` | `set_input_content_properties_enabled()` |
| `setInputHeight()` | `set_input_height()` |
| `setInputWidth()` | `set_input_width()` |
| `setMotionTextureFormat()` | `set_motion_texture_format()` |
| `setOutputHeight()` | `set_output_height()` |
| `setOutputTextureFormat()` | `set_output_texture_format()` |
| `setOutputWidth()` | `set_output_width()` |
| `setReactiveMaskTextureEnabled()` | `set_reactive_mask_texture_enabled()` |
| `setReactiveMaskTextureFormat()` | `set_reactive_mask_texture_format()` |
| `setRequiresSynchronousInitialization()` | `set_requires_synchronous_initialization()` |
| `supportedInputContentMaxScale()` | `set_input_content_max_scale()` |
| `supportedInputContentMinScale()` | `set_input_content_min_scale()` |
| `supportsDevice()` | `supports_device()` |
| `supportsMetal4FX()` | `supports_device()` |

### Tensor

**Rust:** `Tensor` in `metal::src::tensor`

| C++ | Rust |
|-----|------|
| `buffer()` | `buffer()` |
| `bufferOffset()` | `buffer_offset()` |
| `dataType()` | `data_type()` |
| `dimensions()` | `dimensions()` |
| `getBytes()` | `get_bytes()` |
| `gpuResourceID()` | `gpu_resource_id()` |
| `replaceSliceOrigin()` | `replace_slice_origin()` |
| `strides()` | `strides()` |
| `usage()` | `usage()` |

### TensorBinding

**Rust:** `TensorBinding` in `metal::src::argument::tensor_binding`

| C++ | Rust |
|-----|------|
| `dimensions()` | `dimensions_ptr()` |
| `indexType()` | `index_type()` |
| `tensorDataType()` | `tensor_data_type()` |

### TensorDescriptor

**Rust:** `TensorDescriptor` in `metal::src::tensor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `cpuCacheMode()` | `cpu_cache_mode()` |
| `dataType()` | `data_type()` |
| `dimensions()` | `dimensions()` |
| `hazardTrackingMode()` | `hazard_tracking_mode()` |
| `init()` | `new()` |
| `resourceOptions()` | `resource_options()` |
| `setCpuCacheMode()` | `set_cpu_cache_mode()` |
| `setDataType()` | `set_data_type()` |
| `setDimensions()` | `set_dimensions()` |
| `setHazardTrackingMode()` | `set_hazard_tracking_mode()` |
| `setResourceOptions()` | `set_resource_options()` |
| `setStorageMode()` | `set_storage_mode()` |
| `setStrides()` | `set_strides()` |
| `setUsage()` | `set_usage()` |
| `storageMode()` | `storage_mode()` |
| `strides()` | `strides()` |
| `usage()` | `usage()` |

### TensorExtents

**Rust:** `TensorExtents` in `metal::src::tensor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `extentAtDimensionIndex()` | `extent_at_dimension_index()` |
| `init()` | `new()` |
| `init()` | `new()` |
| `rank()` | `rank()` |

### TensorReferenceType

**Rust:** `TensorReferenceType` in `metal::src::argument::tensor_reference_type`

| C++ | Rust |
|-----|------|
| `access()` | `access()` |
| `alloc()` | `n/a()` |
| `dimensions()` | `dimensions_ptr()` |
| `indexType()` | `index_type()` |
| `init()` | `n/a()` |
| `tensorDataType()` | `tensor_data_type()` |

### Texture

**Rust:** `Texture` in `metal::src::texture::texture`

| C++ | Rust |
|-----|------|
| `allowGPUOptimizedContents()` | `allow_gpu_optimized_contents()` |
| `arrayLength()` | `array_length()` |
| `buffer()` | `buffer()` |
| `bufferBytesPerRow()` | `buffer_bytes_per_row()` |
| `bufferOffset()` | `buffer_offset()` |
| `compressionType()` | `compression_type()` |
| `depth()` | `depth()` |
| `firstMipmapInTail()` | `first_mipmap_in_tail()` |
| `framebufferOnly()` | `is_framebuffer_only()` |
| `getBytes()` | `get_bytes()` |
| `getBytes()` | `get_bytes()` |
| `gpuResourceID()` | `gpu_resource_id()` |
| `height()` | `height()` |
| `iosurface()` | `iosurface()` |
| `iosurfacePlane()` | `iosurface_plane()` |
| `isFramebufferOnly()` | `is_framebuffer_only()` |
| `isShareable()` | `is_shareable()` |
| `isSparse()` | `is_sparse()` |
| `mipmapLevelCount()` | `mipmap_level_count()` |
| `newRemoteTextureViewForDevice()` | `new_remote_texture_view_for_device()` |
| `newSharedTextureHandle()` | `new_shared_texture_handle()` |
| `newTextureView()` | `new_texture_view()` |
| `newTextureView()` | `new_texture_view()` |
| `newTextureView()` | `new_texture_view()` |
| `newTextureView()` | `new_texture_view()` |
| `parentRelativeLevel()` | `parent_relative_level()` |
| `parentRelativeSlice()` | `parent_relative_slice()` |
| `parentTexture()` | `parent_texture()` |
| `pixelFormat()` | `pixel_format()` |
| `remoteStorageTexture()` | `remote_storage_texture()` |
| `replaceRegion()` | `replace_region()` |
| `replaceRegion()` | `replace_region()` |
| `rootResource()` | `gpu_resource_id()` |
| `sampleCount()` | `sample_count()` |
| `shareable()` | `is_shareable()` |
| `sparseTextureTier()` | `sparse_texture_tier()` |
| `swizzle()` | `swizzle()` |
| `tailSizeInBytes()` | `tail_size_in_bytes()` |
| `textureType()` | `texture_type()` |
| `usage()` | `usage()` |
| `width()` | `width()` |

### TextureBinding

**Rust:** `TextureBinding` in `metal::src::argument::texture_binding`

| C++ | Rust |
|-----|------|
| `arrayLength()` | `array_length()` |
| `depthTexture()` | `depth_texture()` |
| `isDepthTexture()` | `is_depth_texture()` |
| `textureDataType()` | `texture_data_type()` |
| `textureType()` | `texture_type()` |

### TextureDescriptor

**Rust:** `TextureDescriptor` in `metal::src::texture::descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `allowGPUOptimizedContents()` | `allow_gpu_optimized_contents()` |
| `arrayLength()` | `array_length()` |
| `compressionType()` | `compression_type()` |
| `cpuCacheMode()` | `cpu_cache_mode()` |
| `depth()` | `depth()` |
| `hazardTrackingMode()` | `hazard_tracking_mode()` |
| `height()` | `height()` |
| `init()` | `new()` |
| `mipmapLevelCount()` | `mipmap_level_count()` |
| `pixelFormat()` | `pixel_format()` |
| `placementSparsePageSize()` | `placement_sparse_page_size()` |
| `resourceOptions()` | `resource_options()` |
| `sampleCount()` | `sample_count()` |
| `setAllowGPUOptimizedContents()` | `set_allow_gpu_optimized_contents()` |
| `setArrayLength()` | `set_array_length()` |
| `setCompressionType()` | `set_compression_type()` |
| `setCpuCacheMode()` | `set_cpu_cache_mode()` |
| `setDepth()` | `set_depth()` |
| `setHazardTrackingMode()` | `set_hazard_tracking_mode()` |
| `setHeight()` | `set_height()` |
| `setMipmapLevelCount()` | `set_mipmap_level_count()` |
| `setPixelFormat()` | `set_pixel_format()` |
| `setPlacementSparsePageSize()` | `set_placement_sparse_page_size()` |
| `setResourceOptions()` | `set_resource_options()` |
| `setSampleCount()` | `set_sample_count()` |
| `setStorageMode()` | `set_storage_mode()` |
| `setSwizzle()` | `set_swizzle()` |
| `setTextureType()` | `set_texture_type()` |
| `setUsage()` | `set_usage()` |
| `setWidth()` | `set_width()` |
| `storageMode()` | `storage_mode()` |
| `swizzle()` | `swizzle()` |
| `texture2DDescriptor()` | `texture_2d_descriptor()` |
| `textureBufferDescriptor()` | `texture_buffer_descriptor()` |
| `textureCubeDescriptor()` | `texture_cube_descriptor()` |
| `textureType()` | `texture_type()` |
| `usage()` | `usage()` |
| `width()` | `width()` |

### TextureReferenceType

**Rust:** `TextureReferenceType` in `metal::src::argument::texture_reference_type`

| C++ | Rust |
|-----|------|
| `access()` | `access()` |
| `alloc()` | `n/a()` |
| `init()` | `n/a()` |
| `isDepthTexture()` | `is_depth_texture()` |
| `textureDataType()` | `texture_data_type()` |
| `textureType()` | `texture_type()` |

### TextureViewDescriptor

**Rust:** `TextureViewDescriptor` in `metal::src::texture::view_descriptor`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `levelRange()` | `level_range()` |
| `pixelFormat()` | `pixel_format()` |
| `setLevelRange()` | `set_level_range()` |
| `setPixelFormat()` | `set_pixel_format()` |
| `setSliceRange()` | `set_slice_range()` |
| `setSwizzle()` | `set_swizzle()` |
| `setTextureType()` | `set_texture_type()` |
| `sliceRange()` | `slice_range()` |
| `swizzle()` | `swizzle()` |
| `textureType()` | `texture_type()` |

### TextureViewPool

**Rust:** `TextureViewPool` in `metal::src::texture_view_pool`

| C++ | Rust |
|-----|------|
| `setTextureView()` | `set_texture_view()` |
| `setTextureView()` | `set_texture_view()` |
| `setTextureViewFromBuffer()` | `set_texture_view_from_buffer()` |

### ThreadgroupBinding

**Rust:** `ThreadgroupBinding` in `metal::src::argument::threadgroup_binding`

| C++ | Rust |
|-----|------|
| `threadgroupMemoryAlignment()` | `threadgroup_memory_alignment()` |
| `threadgroupMemoryDataSize()` | `threadgroup_memory_data_size()` |

### TileRenderPipelineColorAttachmentDescriptor

**Rust:** `TileRenderPipelineColorAttachmentDescriptor` in `metal::src::pipeline::tile_pipeline`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `pixelFormat()` | `pixel_format()` |
| `setPixelFormat()` | `set_pixel_format()` |

### TileRenderPipelineColorAttachmentDescriptorArray

**Rust:** `TileRenderPipelineColorAttachmentDescriptorArray` in `metal::src::pipeline::tile_pipeline`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `init()` | `n/a()` |
| `object()` | `object()` |
| `setObject()` | `set_object()` |

### TileRenderPipelineDescriptor

**Rust:** `TileRenderPipelineDescriptor` in `metal::src::mtl4::tile_render_pipeline`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `colorAttachments()` | `color_attachments()` |
| `init()` | `new()` |
| `maxTotalThreadsPerThreadgroup()` | `max_total_threads_per_threadgroup()` |
| `rasterSampleCount()` | `raster_sample_count()` |
| `requiredThreadsPerThreadgroup()` | `required_threads_per_threadgroup()` |
| `reset()` | `reset()` |
| `setMaxTotalThreadsPerThreadgroup()` | `set_max_total_threads_per_threadgroup()` |
| `setRasterSampleCount()` | `set_raster_sample_count()` |
| `setRequiredThreadsPerThreadgroup()` | `set_required_threads_per_threadgroup()` |
| `setStaticLinkingDescriptor()` | `set_static_linking_descriptor()` |
| `setSupportBinaryLinking()` | `set_support_binary_linking()` |
| `setThreadgroupSizeMatchesTileSize()` | `set_threadgroup_size_matches_tile_size()` |
| `setTileFunctionDescriptor()` | `set_tile_function_descriptor()` |
| `staticLinkingDescriptor()` | `static_linking_descriptor()` |
| `supportBinaryLinking()` | `support_binary_linking()` |
| `threadgroupSizeMatchesTileSize()` | `threadgroup_size_matches_tile_size()` |
| `tileFunctionDescriptor()` | `tile_function_descriptor()` |

### TileRenderPipelineDescriptor

**Rust:** `TileRenderPipelineDescriptor` in `metal::src::mtl4::tile_render_pipeline`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `binaryArchives()` | `binary_archives_raw()` |
| `colorAttachments()` | `color_attachments()` |
| `init()` | `new()` |
| `label()` | `label()` |
| `linkedFunctions()` | `linked_functions()` |
| `maxCallStackDepth()` | `max_call_stack_depth()` |
| `maxTotalThreadsPerThreadgroup()` | `max_total_threads_per_threadgroup()` |
| `preloadedLibraries()` | `preloaded_libraries_raw()` |
| `rasterSampleCount()` | `raster_sample_count()` |
| `requiredThreadsPerThreadgroup()` | `required_threads_per_threadgroup()` |
| `reset()` | `reset()` |
| `setBinaryArchives()` | `set_binary_archives_raw()` |
| `setLabel()` | `set_label()` |
| `setLinkedFunctions()` | `set_linked_functions()` |
| `setMaxCallStackDepth()` | `set_max_call_stack_depth()` |
| `setMaxTotalThreadsPerThreadgroup()` | `set_max_total_threads_per_threadgroup()` |
| `setPreloadedLibraries()` | `set_preloaded_libraries_raw()` |
| `setRasterSampleCount()` | `set_raster_sample_count()` |
| `setRequiredThreadsPerThreadgroup()` | `set_required_threads_per_threadgroup()` |
| `setShaderValidation()` | `set_shader_validation()` |
| `setSupportAddingBinaryFunctions()` | `set_support_adding_binary_functions()` |
| `setThreadgroupSizeMatchesTileSize()` | `set_threadgroup_size_matches_tile_size()` |
| `setTileFunction()` | `set_tile_function()` |
| `shaderValidation()` | `shader_validation()` |
| `supportAddingBinaryFunctions()` | `support_adding_binary_functions()` |
| `threadgroupSizeMatchesTileSize()` | `threadgroup_size_matches_tile_size()` |
| `tileBuffers()` | `tile_buffers()` |
| `tileFunction()` | `tile_function()` |

### Type

**Rust:** `Type` in `metal::src::argument::type_info`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `dataType()` | `data_type()` |
| `init()` | `n/a()` |

### VertexAttribute

**Rust:** `VertexAttribute` in `metal::src::library::vertex_attribute`

| C++ | Rust |
|-----|------|
| `active()` | `is_active()` |
| `alloc()` | `new()` |
| `attributeIndex()` | `attribute_index()` |
| `attributeType()` | `attribute_type()` |
| `init()` | `new()` |
| `isActive()` | `is_active()` |
| `isPatchControlPointData()` | `is_patch_control_point_data()` |
| `isPatchData()` | `is_patch_data()` |
| `name()` | `name()` |
| `patchControlPointData()` | `is_patch_control_point_data()` |
| `patchData()` | `is_patch_data()` |

### VertexAttributeDescriptor

**Rust:** `VertexAttributeDescriptor` in `metal::src::vertex`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `bufferIndex()` | `buffer_index()` |
| `format()` | `format()` |
| `init()` | `new()` |
| `offset()` | `offset()` |
| `setBufferIndex()` | `set_buffer_index()` |
| `setFormat()` | `set_format()` |
| `setOffset()` | `set_offset()` |

### VertexAttributeDescriptorArray

**Rust:** `VertexAttributeDescriptorArray` in `metal::src::vertex`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `init()` | `n/a()` |
| `object()` | `object()` |
| `setObject()` | `set_object()` |

### VertexBufferLayoutDescriptor

**Rust:** `VertexBufferLayoutDescriptor` in `metal::src::vertex`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `init()` | `new()` |
| `setStepFunction()` | `set_step_function()` |
| `setStepRate()` | `set_step_rate()` |
| `setStride()` | `set_stride()` |
| `stepFunction()` | `step_function()` |
| `stepRate()` | `step_rate()` |
| `stride()` | `stride()` |

### VertexBufferLayoutDescriptorArray

**Rust:** `VertexBufferLayoutDescriptorArray` in `metal::src::vertex`

| C++ | Rust |
|-----|------|
| `alloc()` | `n/a()` |
| `init()` | `n/a()` |
| `object()` | `object()` |
| `setObject()` | `set_object()` |

### VertexDescriptor

**Rust:** `VertexDescriptor` in `metal::src::vertex`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `attributes()` | `attributes()` |
| `init()` | `new()` |
| `layouts()` | `layouts()` |
| `reset()` | `reset()` |
| `vertexDescriptor()` | `new()` |

### VisibleFunctionTable

**Rust:** `VisibleFunctionTable` in `metal::src::function_table`

| C++ | Rust |
|-----|------|
| `gpuResourceID()` | `gpu_resource_id()` |
| `setFunction()` | `set_function()` |
| `setFunctions()` | `set_functions()` |

### VisibleFunctionTableDescriptor

**Rust:** `VisibleFunctionTableDescriptor` in `metal::src::function_table`

| C++ | Rust |
|-----|------|
| `alloc()` | `new()` |
| `functionCount()` | `function_count()` |
| `init()` | `new()` |
| `setFunctionCount()` | `set_function_count()` |
| `visibleFunctionTableDescriptor()` | `new()` |
