//! ABI Validation Tests
//!
//! These tests verify that Rust struct layouts exactly match the Metal framework's
//! expected memory layout. Mismatched struct sizes or field offsets would cause
//! memory corruption when passing data to Metal APIs.
//!
//! The tests validate:
//! - Struct sizes match Metal's expectations
//! - Field offsets are correct (for packed structs)
//! - Alignment requirements are met

use std::mem::{offset_of, size_of};

use mtl::{
    ClearColor, DispatchThreadgroupsIndirectArguments, DispatchThreadsIndirectArguments,
    DrawIndexedPrimitivesIndirectArguments, DrawPatchIndirectArguments,
    DrawPrimitivesIndirectArguments, Origin, Region, ResourceID, SamplePosition, ScissorRect, Size,
    SizeAndAlign, StageInRegionIndirectArguments, Viewport,
};

// =============================================================================
// Core Geometry Types
// =============================================================================

#[test]
fn test_origin_layout() {
    // Origin has 3 UInteger (usize) fields = 3 * 8 = 24 bytes on 64-bit
    assert_eq!(
        size_of::<Origin>(),
        24,
        "Origin should be 24 bytes on 64-bit"
    );

    // Verify field offsets (packed struct, no padding)
    assert_eq!(offset_of!(Origin, x), 0);
    assert_eq!(offset_of!(Origin, y), 8);
    assert_eq!(offset_of!(Origin, z), 16);
}

#[test]
fn test_size_layout() {
    // Size has 3 UInteger (usize) fields = 3 * 8 = 24 bytes on 64-bit
    assert_eq!(size_of::<Size>(), 24, "Size should be 24 bytes on 64-bit");

    // Verify field offsets
    assert_eq!(offset_of!(Size, width), 0);
    assert_eq!(offset_of!(Size, height), 8);
    assert_eq!(offset_of!(Size, depth), 16);
}

#[test]
fn test_region_layout() {
    // Region contains Origin (24) + Size (24) = 48 bytes
    assert_eq!(size_of::<Region>(), 48, "Region should be 48 bytes");

    // Verify field offsets
    assert_eq!(offset_of!(Region, origin), 0);
    assert_eq!(offset_of!(Region, size), 24);
}

#[test]
fn test_sample_position_layout() {
    // SamplePosition has 2 f32 fields = 2 * 4 = 8 bytes
    assert_eq!(
        size_of::<SamplePosition>(),
        8,
        "SamplePosition should be 8 bytes"
    );

    // Verify field offsets
    assert_eq!(offset_of!(SamplePosition, x), 0);
    assert_eq!(offset_of!(SamplePosition, y), 4);
}

// =============================================================================
// Render State Types
// =============================================================================

#[test]
fn test_viewport_layout() {
    // Viewport has 6 f64 fields = 6 * 8 = 48 bytes
    assert_eq!(size_of::<Viewport>(), 48, "Viewport should be 48 bytes");

    // Verify field offsets
    assert_eq!(offset_of!(Viewport, origin_x), 0);
    assert_eq!(offset_of!(Viewport, origin_y), 8);
    assert_eq!(offset_of!(Viewport, width), 16);
    assert_eq!(offset_of!(Viewport, height), 24);
    assert_eq!(offset_of!(Viewport, znear), 32);
    assert_eq!(offset_of!(Viewport, zfar), 40);
}

#[test]
fn test_clear_color_layout() {
    // ClearColor has 4 f64 fields = 4 * 8 = 32 bytes
    assert_eq!(size_of::<ClearColor>(), 32, "ClearColor should be 32 bytes");

    // Verify field offsets
    assert_eq!(offset_of!(ClearColor, red), 0);
    assert_eq!(offset_of!(ClearColor, green), 8);
    assert_eq!(offset_of!(ClearColor, blue), 16);
    assert_eq!(offset_of!(ClearColor, alpha), 24);
}

#[test]
fn test_scissor_rect_layout() {
    // ScissorRect has 4 UInteger fields = 4 * 8 = 32 bytes on 64-bit
    assert_eq!(
        size_of::<ScissorRect>(),
        32,
        "ScissorRect should be 32 bytes on 64-bit"
    );

    // Verify field offsets
    assert_eq!(offset_of!(ScissorRect, x), 0);
    assert_eq!(offset_of!(ScissorRect, y), 8);
    assert_eq!(offset_of!(ScissorRect, width), 16);
    assert_eq!(offset_of!(ScissorRect, height), 24);
}

// =============================================================================
// Indirect Argument Types
// =============================================================================

#[test]
fn test_draw_primitives_indirect_arguments_layout() {
    // 4 u32 fields = 4 * 4 = 16 bytes
    assert_eq!(
        size_of::<DrawPrimitivesIndirectArguments>(),
        16,
        "DrawPrimitivesIndirectArguments should be 16 bytes"
    );

    assert_eq!(offset_of!(DrawPrimitivesIndirectArguments, vertex_count), 0);
    assert_eq!(
        offset_of!(DrawPrimitivesIndirectArguments, instance_count),
        4
    );
    assert_eq!(offset_of!(DrawPrimitivesIndirectArguments, vertex_start), 8);
    assert_eq!(
        offset_of!(DrawPrimitivesIndirectArguments, base_instance),
        12
    );
}

#[test]
fn test_draw_indexed_primitives_indirect_arguments_layout() {
    // 5 u32 fields = 5 * 4 = 20 bytes
    assert_eq!(
        size_of::<DrawIndexedPrimitivesIndirectArguments>(),
        20,
        "DrawIndexedPrimitivesIndirectArguments should be 20 bytes"
    );

    assert_eq!(
        offset_of!(DrawIndexedPrimitivesIndirectArguments, index_count),
        0
    );
    assert_eq!(
        offset_of!(DrawIndexedPrimitivesIndirectArguments, instance_count),
        4
    );
    assert_eq!(
        offset_of!(DrawIndexedPrimitivesIndirectArguments, index_start),
        8
    );
    assert_eq!(
        offset_of!(DrawIndexedPrimitivesIndirectArguments, base_vertex),
        12
    );
    assert_eq!(
        offset_of!(DrawIndexedPrimitivesIndirectArguments, base_instance),
        16
    );
}

#[test]
fn test_draw_patch_indirect_arguments_layout() {
    // 4 u32 fields = 4 * 4 = 16 bytes
    assert_eq!(
        size_of::<DrawPatchIndirectArguments>(),
        16,
        "DrawPatchIndirectArguments should be 16 bytes"
    );

    assert_eq!(offset_of!(DrawPatchIndirectArguments, patch_count), 0);
    assert_eq!(offset_of!(DrawPatchIndirectArguments, instance_count), 4);
    assert_eq!(offset_of!(DrawPatchIndirectArguments, patch_start), 8);
    assert_eq!(offset_of!(DrawPatchIndirectArguments, base_instance), 12);
}

#[test]
fn test_dispatch_threadgroups_indirect_arguments_layout() {
    // 3 u32 fields = 3 * 4 = 12 bytes
    assert_eq!(
        size_of::<DispatchThreadgroupsIndirectArguments>(),
        12,
        "DispatchThreadgroupsIndirectArguments should be 12 bytes"
    );

    assert_eq!(
        offset_of!(DispatchThreadgroupsIndirectArguments, threadgroups_per_grid),
        0
    );
}

#[test]
fn test_dispatch_threads_indirect_arguments_layout() {
    // 2 arrays of [u32; 3] = 2 * 3 * 4 = 24 bytes
    assert_eq!(
        size_of::<DispatchThreadsIndirectArguments>(),
        24,
        "DispatchThreadsIndirectArguments should be 24 bytes"
    );

    assert_eq!(
        offset_of!(DispatchThreadsIndirectArguments, threads_per_grid),
        0
    );
    assert_eq!(
        offset_of!(DispatchThreadsIndirectArguments, threads_per_threadgroup),
        12
    );
}

#[test]
fn test_stage_in_region_indirect_arguments_layout() {
    // 6 u32 fields (origin x/y/z + size w/h/d) = 6 * 4 = 24 bytes
    assert_eq!(
        size_of::<StageInRegionIndirectArguments>(),
        24,
        "StageInRegionIndirectArguments should be 24 bytes"
    );

    assert_eq!(
        offset_of!(StageInRegionIndirectArguments, stage_in_origin),
        0
    );
    assert_eq!(
        offset_of!(StageInRegionIndirectArguments, stage_in_size),
        12
    );
}

// =============================================================================
// Resource Types
// =============================================================================

#[test]
fn test_resource_id_layout() {
    // ResourceID contains a single u64 = 8 bytes
    assert_eq!(size_of::<ResourceID>(), 8, "ResourceID should be 8 bytes");

    assert_eq!(offset_of!(ResourceID, _impl), 0);
}

#[test]
fn test_size_and_align_layout() {
    // SizeAndAlign has 2 UInteger fields = 2 * 8 = 16 bytes on 64-bit
    assert_eq!(
        size_of::<SizeAndAlign>(),
        16,
        "SizeAndAlign should be 16 bytes on 64-bit"
    );

    assert_eq!(offset_of!(SizeAndAlign, size), 0);
    assert_eq!(offset_of!(SizeAndAlign, align), 8);
}

// =============================================================================
// Verify packed structs have no padding
// =============================================================================

#[test]
fn test_no_padding_in_packed_structs() {
    // These structs are repr(C, packed) so should have no padding

    // Origin: 3 * sizeof(usize) with no padding
    assert_eq!(size_of::<Origin>(), 3 * size_of::<usize>());

    // Size: 3 * sizeof(usize) with no padding
    assert_eq!(size_of::<Size>(), 3 * size_of::<usize>());

    // Region: Origin + Size with no padding between them
    assert_eq!(size_of::<Region>(), size_of::<Origin>() + size_of::<Size>());

    // SamplePosition: 2 * sizeof(f32) with no padding
    assert_eq!(size_of::<SamplePosition>(), 2 * size_of::<f32>());

    // Viewport: 6 * sizeof(f64) with no padding
    assert_eq!(size_of::<Viewport>(), 6 * size_of::<f64>());

    // ClearColor: 4 * sizeof(f64) with no padding
    assert_eq!(size_of::<ClearColor>(), 4 * size_of::<f64>());
}

// =============================================================================
// Verify types match expected Metal conventions
// =============================================================================

#[test]
fn test_metal_type_conventions() {
    // UInteger should be 8 bytes on 64-bit (NS::UInteger is size_t)
    assert_eq!(size_of::<usize>(), 8, "Expected 64-bit platform");

    // f32 should be 4 bytes (matches float in Metal)
    assert_eq!(size_of::<f32>(), 4);

    // f64 should be 8 bytes (matches double in Metal)
    assert_eq!(size_of::<f64>(), 8);

    // u32 should be 4 bytes (matches uint32_t in Metal)
    assert_eq!(size_of::<u32>(), 4);

    // u64 should be 8 bytes (matches uint64_t in Metal)
    assert_eq!(size_of::<u64>(), 8);
}
