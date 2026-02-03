//! Selector Validation Tests
//!
//! These tests verify that all Objective-C selectors used in the metal crate
//! actually exist in the Metal framework. A typo in a selector name would
//! cause a runtime crash when that method is called.
//!
//! The tests use the Objective-C runtime to verify that Metal protocols and
//! classes respond to the selectors we use.
//!
//! Note: Most Metal types (MTLDevice, MTLBuffer, etc.) are Objective-C
//! *protocols*, not classes. The concrete implementation classes are private.

use metal_sys::{Class, Protocol, Sel};

/// Verify that a protocol declares an instance method selector.
fn check_protocol_instance_selector(protocol_name: &str, selector_name: &str) -> bool {
    let Some(protocol) = Protocol::get(protocol_name) else {
        eprintln!("Protocol not found: {}", protocol_name);
        return false;
    };
    let sel = Sel::register(selector_name);
    protocol.has_instance_method(sel)
}

/// Verify that a class responds to an instance method selector.
#[allow(dead_code)]
fn check_class_instance_selector(class_name: &str, selector_name: &str) -> bool {
    let Some(class) = Class::get(class_name) else {
        eprintln!("Class not found: {}", class_name);
        return false;
    };
    let sel = Sel::register(selector_name);
    class.instances_respond_to(sel)
}

/// Verify that a class responds to a class method selector.
fn check_class_selector(class_name: &str, selector_name: &str) -> bool {
    let Some(class) = Class::get(class_name) else {
        eprintln!("Class not found: {}", class_name);
        return false;
    };
    let sel = Sel::register(selector_name);
    class.responds_to(sel)
}

/// Helper macro to validate multiple protocol selectors and collect failures.
macro_rules! validate_protocol_selectors {
    (instance $protocol:expr => [$($sel:expr),* $(,)?]) => {{
        let mut failures = Vec::new();
        $(
            if !check_protocol_instance_selector($protocol, $sel) {
                failures.push(format!("{}.-{}", $protocol, $sel));
            }
        )*
        failures
    }};
}

/// Helper macro to validate multiple class selectors and collect failures.
#[allow(unused_macros)]
macro_rules! validate_class_selectors {
    (class $class:expr => [$($sel:expr),* $(,)?]) => {{
        let mut failures = Vec::new();
        $(
            if !check_class_selector($class, $sel) {
                failures.push(format!("{}.+{}", $class, $sel));
            }
        )*
        failures
    }};
}

// =============================================================================
// MTLDevice - The entry point for all Metal operations
// =============================================================================

#[test]
fn validate_device_selectors() {
    let mut failures = Vec::new();

    // MTLDevice is a protocol
    failures.extend(validate_protocol_selectors!(instance "MTLDevice" => [
        // Basic properties
        "name",
        "registryID",
        "maxThreadsPerThreadgroup",
        "hasUnifiedMemory",
        "recommendedMaxWorkingSetSize",
        "location",
        "locationNumber",
        "maxTransferRate",
        "isHeadless",
        "isLowPower",
        "isRemovable",
        "peerGroupID",
        "peerIndex",
        "peerCount",
        "currentAllocatedSize",
        "maxBufferLength",

        // Feature queries
        "supportsFamily:",
        "supportsTextureSampleCount:",
        "supportsCounterSampling:",
        "supportsVertexAmplificationCount:",
        "supportsDynamicLibraries",
        "supportsRenderDynamicLibraries",
        "supportsFunctionPointers",
        "supportsFunctionPointersFromRender",
        "supportsPullModelInterpolation",
        "supportsRaytracing",
        "supportsRaytracingFromRender",
        "supports32BitFloatFiltering",
        "supports32BitMSAA",
        "supportsBCTextureCompression",
        "supportsQueryTextureLOD",
        "supportsShaderBarycentricCoordinates",
        "areProgrammableSamplePositionsSupported",
        "areRasterOrderGroupsSupported",

        // Resource creation
        "newCommandQueue",
        "newCommandQueueWithMaxCommandBufferCount:",
        "newBufferWithLength:options:",
        "newBufferWithBytes:length:options:",
        "newBufferWithBytesNoCopy:length:options:deallocator:",
        "newTextureWithDescriptor:",
        "newSamplerStateWithDescriptor:",
        "newDepthStencilStateWithDescriptor:",

        // Pipeline creation
        "newLibraryWithSource:options:error:",
        "newLibraryWithData:error:",
        "newLibraryWithURL:error:",
        "newDefaultLibrary",
        "newDefaultLibraryWithBundle:error:",
        "newRenderPipelineStateWithDescriptor:error:",
        "newRenderPipelineStateWithDescriptor:options:reflection:error:",
        "newComputePipelineStateWithFunction:error:",
        "newComputePipelineStateWithFunction:options:reflection:error:",
        "newComputePipelineStateWithDescriptor:options:reflection:error:",

        // Heap and memory
        "newHeapWithDescriptor:",
        "heapBufferSizeAndAlignWithLength:options:",
        "heapTextureSizeAndAlignWithDescriptor:",

        // Argument encoder
        "newArgumentEncoderWithArguments:",

        // IO command queue
        "newIOCommandQueueWithDescriptor:error:",
        "newIOHandleWithURL:error:",

        // Event
        "newEvent",
        "newSharedEvent",
        "newSharedEventWithHandle:",
        "newFence",

        // Indirect command buffer
        "newIndirectCommandBufferWithDescriptor:maxCommandCount:options:",

        // Acceleration structure
        "newAccelerationStructureWithSize:",
        "newAccelerationStructureWithDescriptor:",
        "accelerationStructureSizesWithDescriptor:",
    ]));

    if !failures.is_empty() {
        panic!(
            "MTLDevice selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLCommandQueue
// =============================================================================

#[test]
fn validate_command_queue_selectors() {
    let mut failures = Vec::new();

    failures.extend(validate_protocol_selectors!(instance "MTLCommandQueue" => [
        "label",
        "setLabel:",
        "device",
        "commandBuffer",
        "commandBufferWithDescriptor:",
        "commandBufferWithUnretainedReferences",
    ]));

    if !failures.is_empty() {
        panic!(
            "MTLCommandQueue selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLCommandBuffer
// =============================================================================

#[test]
fn validate_command_buffer_selectors() {
    let mut failures = Vec::new();

    failures.extend(
        validate_protocol_selectors!(instance "MTLCommandBuffer" => [
            "label",
            "setLabel:",
            "device",
            "commandQueue",
            "retainedReferences",
            "errorOptions",
            "status",
            "error",
            "logs",
            "kernelStartTime",
            "kernelEndTime",
            "GPUStartTime",
            "GPUEndTime",

            // Encoding
            "renderCommandEncoderWithDescriptor:",
            "computeCommandEncoder",
            "computeCommandEncoderWithDescriptor:",
            "computeCommandEncoderWithDispatchType:",
            "blitCommandEncoder",
            "blitCommandEncoderWithDescriptor:",
            "parallelRenderCommandEncoderWithDescriptor:",
            "resourceStateCommandEncoder",
            "resourceStateCommandEncoderWithDescriptor:",
            "accelerationStructureCommandEncoder",
            "accelerationStructureCommandEncoderWithDescriptor:",

            // Execution
            "enqueue",
            "commit",
            "waitUntilScheduled",
            "waitUntilCompleted",
            "addScheduledHandler:",
            "addCompletedHandler:",

            // Debug
            "pushDebugGroup:",
            "popDebugGroup",
            "encodeSignalEvent:value:",
            "encodeWaitForEvent:value:",
        ]),
    );

    if !failures.is_empty() {
        panic!(
            "MTLCommandBuffer selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLBuffer
// =============================================================================

#[test]
fn validate_buffer_selectors() {
    let mut failures = Vec::new();

    failures.extend(validate_protocol_selectors!(instance "MTLBuffer" => [
        "label",
        "setLabel:",
        "device",
        "cpuCacheMode",
        "storageMode",
        "hazardTrackingMode",
        "resourceOptions",
        "allocatedSize",

        // Buffer-specific
        "length",
        "contents",
        "didModifyRange:",
        "newTextureWithDescriptor:offset:bytesPerRow:",
        "addDebugMarker:range:",
        "removeAllDebugMarkers",
        "gpuAddress",
        "newRemoteBufferViewForDevice:",
        "remoteStorageBuffer",
    ]));

    if !failures.is_empty() {
        panic!(
            "MTLBuffer selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLTexture
// =============================================================================

#[test]
fn validate_texture_selectors() {
    let mut failures = Vec::new();

    failures.extend(validate_protocol_selectors!(instance "MTLTexture" => [
        "label",
        "setLabel:",
        "device",
        "cpuCacheMode",
        "storageMode",
        "hazardTrackingMode",
        "resourceOptions",
        "allocatedSize",

        // Texture-specific
        "rootResource",
        "parentTexture",
        "parentRelativeLevel",
        "parentRelativeSlice",
        "buffer",
        "bufferOffset",
        "bufferBytesPerRow",
        "textureType",
        "pixelFormat",
        "width",
        "height",
        "depth",
        "mipmapLevelCount",
        "sampleCount",
        "arrayLength",
        "usage",
        "isShareable",
        "isFramebufferOnly",
        "firstMipmapInTail",
        "tailSizeInBytes",
        "isSparse",
        "allowGPUOptimizedContents",
        "compressionType",
        "gpuResourceID",
        "swizzle",
        "newTextureViewWithPixelFormat:",
        "newTextureViewWithPixelFormat:textureType:levels:slices:",
        "newTextureViewWithPixelFormat:textureType:levels:slices:swizzle:",
        "getBytes:bytesPerRow:bytesPerImage:fromRegion:mipmapLevel:slice:",
        "replaceRegion:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage:",
        "getBytes:bytesPerRow:fromRegion:mipmapLevel:",
        "replaceRegion:mipmapLevel:withBytes:bytesPerRow:",
        "newSharedTextureHandle",
        "remoteStorageTexture",
        "newRemoteTextureViewForDevice:",
    ]));

    if !failures.is_empty() {
        panic!(
            "MTLTexture selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLRenderCommandEncoder
// =============================================================================

#[test]
fn validate_render_command_encoder_selectors() {
    let mut failures = Vec::new();

    failures.extend(validate_protocol_selectors!(instance "MTLRenderCommandEncoder" => [
        "label",
        "setLabel:",
        "device",
        "endEncoding",
        "insertDebugSignpost:",
        "pushDebugGroup:",
        "popDebugGroup",

        // Render state
        "setRenderPipelineState:",
        "setViewport:",
        "setViewports:count:",
        "setFrontFacingWinding:",
        "setCullMode:",
        "setDepthClipMode:",
        "setDepthBias:slopeScale:clamp:",
        "setScissorRect:",
        "setScissorRects:count:",
        "setTriangleFillMode:",
        "setBlendColorRed:green:blue:alpha:",
        "setDepthStencilState:",
        "setStencilReferenceValue:",
        "setStencilFrontReferenceValue:backReferenceValue:",
        "setVisibilityResultMode:offset:",

        // Vertex resources
        "setVertexBytes:length:atIndex:",
        "setVertexBuffer:offset:atIndex:",
        "setVertexBuffers:offsets:withRange:",
        "setVertexBufferOffset:atIndex:",
        "setVertexTexture:atIndex:",
        "setVertexTextures:withRange:",
        "setVertexSamplerState:atIndex:",
        "setVertexSamplerStates:withRange:",
        "setVertexSamplerState:lodMinClamp:lodMaxClamp:atIndex:",

        // Fragment resources
        "setFragmentBytes:length:atIndex:",
        "setFragmentBuffer:offset:atIndex:",
        "setFragmentBuffers:offsets:withRange:",
        "setFragmentBufferOffset:atIndex:",
        "setFragmentTexture:atIndex:",
        "setFragmentTextures:withRange:",
        "setFragmentSamplerState:atIndex:",
        "setFragmentSamplerStates:withRange:",
        "setFragmentSamplerState:lodMinClamp:lodMaxClamp:atIndex:",

        // Draw calls
        "drawPrimitives:vertexStart:vertexCount:instanceCount:",
        "drawPrimitives:vertexStart:vertexCount:",
        "drawPrimitives:vertexStart:vertexCount:instanceCount:baseInstance:",
        "drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:",
        "drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:",
        "drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:baseVertex:baseInstance:",

        // Indirect draw
        "drawPrimitives:indirectBuffer:indirectBufferOffset:",
        "drawIndexedPrimitives:indexType:indexBuffer:indexBufferOffset:indirectBuffer:indirectBufferOffset:",

        // Tile shading
        "tileWidth",
        "tileHeight",
        "setTileBytes:length:atIndex:",
        "setTileBuffer:offset:atIndex:",
        "setTileBuffers:offsets:withRange:",
        "setTileBufferOffset:atIndex:",
        "setTileTexture:atIndex:",
        "setTileTextures:withRange:",
        "setTileSamplerState:atIndex:",
        "setTileSamplerStates:withRange:",

        // Threadgroup memory
        "setThreadgroupMemoryLength:offset:atIndex:",

        // Use resources
        "useResource:usage:",
        "useResources:count:usage:",
        "useResource:usage:stages:",
        "useResources:count:usage:stages:",
        "useHeap:",
        "useHeaps:count:",
        "useHeap:stages:",
        "useHeaps:count:stages:",
    ]));

    if !failures.is_empty() {
        panic!(
            "MTLRenderCommandEncoder selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLComputeCommandEncoder
// =============================================================================

#[test]
fn validate_compute_command_encoder_selectors() {
    let mut failures = Vec::new();

    failures.extend(
        validate_protocol_selectors!(instance "MTLComputeCommandEncoder" => [
            "label",
            "setLabel:",
            "device",
            "endEncoding",
            "insertDebugSignpost:",
            "pushDebugGroup:",
            "popDebugGroup",

            // Compute state
            "setComputePipelineState:",
            "dispatchType",

            // Resources
            "setBytes:length:atIndex:",
            "setBuffer:offset:atIndex:",
            "setBuffers:offsets:withRange:",
            "setBufferOffset:atIndex:",
            "setTexture:atIndex:",
            "setTextures:withRange:",
            "setSamplerState:atIndex:",
            "setSamplerStates:withRange:",
            "setSamplerState:lodMinClamp:lodMaxClamp:atIndex:",
            "setThreadgroupMemoryLength:atIndex:",
            "setImageblockWidth:height:",

            // Dispatch
            "dispatchThreadgroups:threadsPerThreadgroup:",
            "dispatchThreadgroupsWithIndirectBuffer:indirectBufferOffset:threadsPerThreadgroup:",
            "dispatchThreads:threadsPerThreadgroup:",

            // Barriers and memory
            "memoryBarrierWithScope:",
            "memoryBarrierWithResources:count:",

            // Use resources
            "useResource:usage:",
            "useResources:count:usage:",
            "useHeap:",
            "useHeaps:count:",

            // Indirect command buffer
            "executeCommandsInBuffer:withRange:",
            "executeCommandsInBuffer:indirectBuffer:indirectBufferOffset:",

            // Sample counters
            "sampleCountersInBuffer:atSampleIndex:withBarrier:",

            // Stage input descriptor
            "setStageInRegion:",
            "setStageInRegionWithIndirectBuffer:indirectBufferOffset:",

            // Visibility
            "setVisibleFunctionTable:atBufferIndex:",
            "setVisibleFunctionTables:withBufferRange:",
            "setIntersectionFunctionTable:atBufferIndex:",
            "setIntersectionFunctionTables:withBufferRange:",
            "setAccelerationStructure:atBufferIndex:",
        ]),
    );

    if !failures.is_empty() {
        panic!(
            "MTLComputeCommandEncoder selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLBlitCommandEncoder
// =============================================================================

#[test]
fn validate_blit_command_encoder_selectors() {
    let mut failures = Vec::new();

    failures.extend(validate_protocol_selectors!(instance "MTLBlitCommandEncoder" => [
        "label",
        "setLabel:",
        "device",
        "endEncoding",
        "insertDebugSignpost:",
        "pushDebugGroup:",
        "popDebugGroup",

        // Copy operations
        "copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:",
        "copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:",
        "copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:",
        "copyFromBuffer:sourceOffset:toBuffer:destinationOffset:size:",
        "copyFromTexture:sourceSlice:sourceLevel:toTexture:destinationSlice:destinationLevel:sliceCount:levelCount:",
        "copyFromTexture:toTexture:",

        // Generate mipmaps
        "generateMipmapsForTexture:",

        // Fill operations
        "fillBuffer:range:value:",

        // Synchronization (macOS only)
        "synchronizeResource:",
        "synchronizeTexture:slice:level:",

        // Indirect command buffer
        "optimizeContentsForGPUAccess:",
        "optimizeContentsForCPUAccess:",
        "resetCommandsInBuffer:withRange:",
        "copyIndirectCommandBuffer:sourceRange:destination:destinationIndex:",
        "optimizeIndirectCommandBuffer:withRange:",

        // Sample counters
        "sampleCountersInBuffer:atSampleIndex:withBarrier:",
        "resolveCounters:inRange:destinationBuffer:destinationOffset:",

        // Resource state
        "updateFence:",
        "waitForFence:",

        // Get texture access counters
        "getTextureAccessCounters:region:mipLevel:slice:resetCounters:countersBuffer:countersBufferOffset:",
        "resetTextureAccessCounters:region:mipLevel:slice:",
    ]));

    if !failures.is_empty() {
        panic!(
            "MTLBlitCommandEncoder selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLLibrary
// =============================================================================

#[test]
fn validate_library_selectors() {
    let mut failures = Vec::new();

    failures.extend(validate_protocol_selectors!(instance "MTLLibrary" => [
        "label",
        "setLabel:",
        "device",
        "functionNames",
        "type",
        "installName",
        "newFunctionWithName:",
        "newFunctionWithName:constantValues:error:",
        "newFunctionWithDescriptor:error:",
        "newFunctionWithDescriptor:completionHandler:",
        "newIntersectionFunctionWithDescriptor:error:",
        "newIntersectionFunctionWithDescriptor:completionHandler:",
    ]));

    if !failures.is_empty() {
        panic!(
            "MTLLibrary selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLFunction
// =============================================================================

#[test]
fn validate_function_selectors() {
    let mut failures = Vec::new();

    failures.extend(validate_protocol_selectors!(instance "MTLFunction" => [
        "label",
        "setLabel:",
        "device",
        "functionType",
        "patchType",
        "patchControlPointCount",
        "vertexAttributes",
        "stageInputAttributes",
        "name",
        "options",
        "newArgumentEncoderWithBufferIndex:",
        "newArgumentEncoderWithBufferIndex:reflection:",
    ]));

    if !failures.is_empty() {
        panic!(
            "MTLFunction selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLRenderPipelineState
// =============================================================================

#[test]
fn validate_render_pipeline_state_selectors() {
    let mut failures = Vec::new();

    failures.extend(
        validate_protocol_selectors!(instance "MTLRenderPipelineState" => [
            "label",
            "device",
            "maxTotalThreadsPerThreadgroup",
            "threadgroupSizeMatchesTileSize",
            "imageblockSampleLength",
            "imageblockMemoryLengthForDimensions:",
            "supportIndirectCommandBuffers",
            "maxTotalThreadsPerObjectThreadgroup",
            "maxTotalThreadsPerMeshThreadgroup",
            "objectThreadExecutionWidth",
            "meshThreadExecutionWidth",
            "maxTotalThreadgroupsPerMeshGrid",
            "gpuResourceID",
            "shaderValidation",
            "functionHandleWithFunction:stage:",
            "newVisibleFunctionTableWithDescriptor:stage:",
            "newIntersectionFunctionTableWithDescriptor:stage:",
            "newRenderPipelineStateWithAdditionalBinaryFunctions:error:",
        ]),
    );

    if !failures.is_empty() {
        panic!(
            "MTLRenderPipelineState selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLComputePipelineState
// =============================================================================

#[test]
fn validate_compute_pipeline_state_selectors() {
    let mut failures = Vec::new();

    failures.extend(
        validate_protocol_selectors!(instance "MTLComputePipelineState" => [
            "label",
            "device",
            "maxTotalThreadsPerThreadgroup",
            "threadExecutionWidth",
            "staticThreadgroupMemoryLength",
            "supportIndirectCommandBuffers",
            "gpuResourceID",
            "shaderValidation",
            "functionHandleWithFunction:",
            "newVisibleFunctionTableWithDescriptor:",
            "newIntersectionFunctionTableWithDescriptor:",
            "newComputePipelineStateWithAdditionalBinaryFunctions:error:",
            "imageblockMemoryLengthForDimensions:",
        ]),
    );

    if !failures.is_empty() {
        panic!(
            "MTLComputePipelineState selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLSamplerState
// =============================================================================

#[test]
fn validate_sampler_state_selectors() {
    let mut failures = Vec::new();

    failures.extend(validate_protocol_selectors!(instance "MTLSamplerState" => [
        "label",
        "device",
        "gpuResourceID",
    ]));

    if !failures.is_empty() {
        panic!(
            "MTLSamplerState selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLDepthStencilState
// =============================================================================

#[test]
fn validate_depth_stencil_state_selectors() {
    let mut failures = Vec::new();

    failures.extend(
        validate_protocol_selectors!(instance "MTLDepthStencilState" => [
            "label",
            "device",
        ]),
    );

    if !failures.is_empty() {
        panic!(
            "MTLDepthStencilState selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLHeap
// =============================================================================

#[test]
fn validate_heap_selectors() {
    let mut failures = Vec::new();

    failures.extend(validate_protocol_selectors!(instance "MTLHeap" => [
        "label",
        "setLabel:",
        "device",
        "storageMode",
        "cpuCacheMode",
        "hazardTrackingMode",
        "resourceOptions",
        "size",
        "usedSize",
        "currentAllocatedSize",
        "maxAvailableSizeWithAlignment:",
        "type",
        "newBufferWithLength:options:",
        "newBufferWithLength:options:offset:",
        "newTextureWithDescriptor:",
        "newTextureWithDescriptor:offset:",
        "setPurgeableState:",
    ]));

    if !failures.is_empty() {
        panic!(
            "MTLHeap selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLEvent / MTLSharedEvent
// =============================================================================

#[test]
fn validate_event_selectors() {
    let mut failures = Vec::new();

    failures.extend(validate_protocol_selectors!(instance "MTLEvent" => [
        "label",
        "setLabel:",
        "device",
    ]));

    failures.extend(validate_protocol_selectors!(instance "MTLSharedEvent" => [
        "label",
        "setLabel:",
        "device",
        "signaledValue",
        "setSignaledValue:",
        "newSharedEventHandle",
        "notifyListener:atValue:block:",
    ]));

    if !failures.is_empty() {
        panic!(
            "MTLEvent/MTLSharedEvent selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLFence
// =============================================================================

#[test]
fn validate_fence_selectors() {
    let mut failures = Vec::new();

    failures.extend(validate_protocol_selectors!(instance "MTLFence" => [
        "label",
        "setLabel:",
        "device",
    ]));

    if !failures.is_empty() {
        panic!(
            "MTLFence selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLArgumentEncoder
// =============================================================================

#[test]
fn validate_argument_encoder_selectors() {
    let mut failures = Vec::new();

    failures.extend(
        validate_protocol_selectors!(instance "MTLArgumentEncoder" => [
            "label",
            "setLabel:",
            "device",
            "encodedLength",
            "alignment",
            "setArgumentBuffer:offset:",
            "setArgumentBuffer:startOffset:arrayElement:",
            "setBuffer:offset:atIndex:",
            "setBuffers:offsets:withRange:",
            "setTexture:atIndex:",
            "setTextures:withRange:",
            "setSamplerState:atIndex:",
            "setSamplerStates:withRange:",
            "constantDataAtIndex:",
            "newArgumentEncoderForBufferAtIndex:",
            "setRenderPipelineState:atIndex:",
            "setRenderPipelineStates:withRange:",
            "setComputePipelineState:atIndex:",
            "setComputePipelineStates:withRange:",
            "setIndirectCommandBuffer:atIndex:",
            "setIndirectCommandBuffers:withRange:",
            "setAccelerationStructure:atIndex:",
            "setVisibleFunctionTable:atIndex:",
            "setVisibleFunctionTables:withRange:",
            "setIntersectionFunctionTable:atIndex:",
            "setIntersectionFunctionTables:withRange:",
        ]),
    );

    if !failures.is_empty() {
        panic!(
            "MTLArgumentEncoder selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLIndirectCommandBuffer
// =============================================================================

#[test]
fn validate_indirect_command_buffer_selectors() {
    let mut failures = Vec::new();

    failures.extend(
        validate_protocol_selectors!(instance "MTLIndirectCommandBuffer" => [
            "label",
            "setLabel:",
            "device",
            "size",
            "gpuResourceID",
            "indirectRenderCommandAtIndex:",
            "indirectComputeCommandAtIndex:",
            "resetWithRange:",
        ]),
    );

    if !failures.is_empty() {
        panic!(
            "MTLIndirectCommandBuffer selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLAccelerationStructure
// =============================================================================

#[test]
fn validate_acceleration_structure_selectors() {
    let mut failures = Vec::new();

    failures.extend(
        validate_protocol_selectors!(instance "MTLAccelerationStructure" => [
            "label",
            "setLabel:",
            "device",
            "size",
            "gpuResourceID",
        ]),
    );

    if !failures.is_empty() {
        panic!(
            "MTLAccelerationStructure selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// MTLAccelerationStructureCommandEncoder
// =============================================================================

#[test]
fn validate_acceleration_structure_command_encoder_selectors() {
    let mut failures = Vec::new();

    failures.extend(validate_protocol_selectors!(instance "MTLAccelerationStructureCommandEncoder" => [
        "label",
        "setLabel:",
        "device",
        "endEncoding",
        "insertDebugSignpost:",
        "pushDebugGroup:",
        "popDebugGroup",

        "buildAccelerationStructure:descriptor:scratchBuffer:scratchBufferOffset:",
        "refitAccelerationStructure:descriptor:destination:scratchBuffer:scratchBufferOffset:",
        "refitAccelerationStructure:descriptor:destination:scratchBuffer:scratchBufferOffset:options:",
        "copyAccelerationStructure:toAccelerationStructure:",
        "writeCompactedAccelerationStructureSize:toBuffer:offset:",
        "writeCompactedAccelerationStructureSize:toBuffer:offset:sizeDataType:",
        "copyAndCompactAccelerationStructure:toAccelerationStructure:",
        "sampleCountersInBuffer:atSampleIndex:withBarrier:",
        "updateFence:",
        "waitForFence:",
        "useResource:usage:",
        "useResources:count:usage:",
        "useHeap:",
        "useHeaps:count:",
    ]));

    if !failures.is_empty() {
        panic!(
            "MTLAccelerationStructureCommandEncoder selector validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// Descriptor Classes - validate alloc/init class methods
// =============================================================================

#[test]
fn validate_descriptor_class_methods() {
    let mut failures = Vec::new();

    let descriptor_classes = [
        "MTLTextureDescriptor",
        "MTLSamplerDescriptor",
        "MTLDepthStencilDescriptor",
        "MTLStencilDescriptor",
        "MTLRenderPipelineDescriptor",
        "MTLComputePipelineDescriptor",
        "MTLRenderPassDescriptor",
        "MTLCommandBufferDescriptor",
        "MTLHeapDescriptor",
        "MTLIndirectCommandBufferDescriptor",
        "MTLCompileOptions",
        "MTLFunctionDescriptor",
        "MTLIntersectionFunctionDescriptor",
        "MTLVisibleFunctionTableDescriptor",
        "MTLIntersectionFunctionTableDescriptor",
        "MTLAccelerationStructureBoundingBoxGeometryDescriptor",
        "MTLAccelerationStructureTriangleGeometryDescriptor",
        "MTLPrimitiveAccelerationStructureDescriptor",
        "MTLInstanceAccelerationStructureDescriptor",
        "MTLResourceStatePassDescriptor",
        "MTLBlitPassDescriptor",
        "MTLComputePassDescriptor",
        "MTLAccelerationStructurePassDescriptor",
    ];

    for class_name in descriptor_classes {
        // alloc is a class method (inherited from NSObject)
        if !check_class_selector(class_name, "alloc") {
            failures.push(format!("{}.+alloc", class_name));
        }
    }

    if !failures.is_empty() {
        panic!(
            "Descriptor class method validation failed:\n{}",
            failures.join("\n")
        );
    }
}

// =============================================================================
// Test that Metal framework is properly loaded
// =============================================================================

#[test]
fn validate_metal_framework_loaded() {
    // Ensure the Metal framework protocols are available
    assert!(
        Protocol::get("MTLDevice").is_some(),
        "MTLDevice protocol not found - Metal framework may not be loaded"
    );
    assert!(
        Protocol::get("MTLCommandQueue").is_some(),
        "MTLCommandQueue protocol not found"
    );
    assert!(
        Protocol::get("MTLBuffer").is_some(),
        "MTLBuffer protocol not found"
    );
    assert!(
        Protocol::get("MTLTexture").is_some(),
        "MTLTexture protocol not found"
    );

    // Ensure descriptor classes are available (these are actual classes)
    assert!(
        Class::get("MTLTextureDescriptor").is_some(),
        "MTLTextureDescriptor class not found"
    );
    assert!(
        Class::get("MTLRenderPassDescriptor").is_some(),
        "MTLRenderPassDescriptor class not found"
    );
}
