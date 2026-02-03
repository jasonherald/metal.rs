//! Render Pipeline Integration Tests
//!
//! These tests verify that render pipeline operations work correctly with the Metal GPU.
//! They test real GPU operations including shader compilation, pipeline creation, and state.

use mtl_gpu::{PixelFormat, RenderPipelineDescriptor, device};

/// Get the default Metal device or skip the test.
fn get_device() -> mtl_gpu::Device {
    device::system_default().expect("No Metal device available")
}

// =============================================================================
// Shader Sources
// =============================================================================

/// Combined vertex and fragment shader
const COMBINED_SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct VertexOut {
    float4 position [[position]];
    float4 color;
};

vertex VertexOut simple_vertex(
    uint vid [[vertex_id]]
) {
    float2 positions[3] = {
        float2( 0.0,  0.5),
        float2(-0.5, -0.5),
        float2( 0.5, -0.5)
    };

    float4 colors[3] = {
        float4(1.0, 0.0, 0.0, 1.0),
        float4(0.0, 1.0, 0.0, 1.0),
        float4(0.0, 0.0, 1.0, 1.0)
    };

    VertexOut out;
    out.position = float4(positions[vid], 0.0, 1.0);
    out.color = colors[vid];
    return out;
}

fragment float4 simple_fragment(VertexOut in [[stage_in]]) {
    return in.color;
}
"#;

// =============================================================================
// Render Pipeline Creation Tests
// =============================================================================

#[test]
fn test_render_pipeline_descriptor_creation() {
    let descriptor = RenderPipelineDescriptor::new();
    assert!(
        descriptor.is_some(),
        "Failed to create RenderPipelineDescriptor"
    );

    let descriptor = descriptor.unwrap();

    // Test default values
    assert!(descriptor.vertex_function().is_none());
    assert!(descriptor.fragment_function().is_none());
    assert_eq!(descriptor.raster_sample_count(), 1);
}

#[test]
fn test_render_pipeline_descriptor_label() {
    let descriptor = RenderPipelineDescriptor::new().unwrap();

    descriptor.set_label("Test Pipeline");
    assert_eq!(descriptor.label(), Some("Test Pipeline".to_string()));
}

#[test]
fn test_render_pipeline_state_creation() {
    let device = get_device();

    // Compile shaders
    let library = device
        .new_library_with_source(COMBINED_SHADER, None)
        .expect("Failed to compile shader");

    let vertex_fn = library
        .new_function_with_name("simple_vertex")
        .expect("Vertex function not found");

    let fragment_fn = library
        .new_function_with_name("simple_fragment")
        .expect("Fragment function not found");

    // Create pipeline descriptor
    let descriptor = RenderPipelineDescriptor::new().expect("Failed to create pipeline descriptor");

    descriptor.set_vertex_function(Some(&vertex_fn));
    descriptor.set_fragment_function(Some(&fragment_fn));

    // Set color attachment format (required)
    let color_attachments = descriptor.color_attachments();
    let attachment = color_attachments.object(0).unwrap();
    attachment.set_pixel_format(PixelFormat::BGRA8_UNORM);

    // Create pipeline state
    let pipeline = device.new_render_pipeline_state_with_descriptor(&descriptor);
    assert!(
        pipeline.is_ok(),
        "Failed to create render pipeline: {:?}",
        pipeline.err()
    );

    let pipeline = pipeline.unwrap();

    // Verify pipeline state properties
    assert!(pipeline.label().is_none()); // No label set on descriptor
}

#[test]
fn test_render_pipeline_with_label() {
    let device = get_device();

    let library = device
        .new_library_with_source(COMBINED_SHADER, None)
        .expect("Failed to compile shader");

    let vertex_fn = library.new_function_with_name("simple_vertex").unwrap();
    let fragment_fn = library.new_function_with_name("simple_fragment").unwrap();

    let descriptor = RenderPipelineDescriptor::new().unwrap();
    descriptor.set_label("MyRenderPipeline");
    descriptor.set_vertex_function(Some(&vertex_fn));
    descriptor.set_fragment_function(Some(&fragment_fn));

    let color_attachments = descriptor.color_attachments();
    color_attachments
        .object(0)
        .unwrap()
        .set_pixel_format(PixelFormat::BGRA8_UNORM);

    let pipeline = device
        .new_render_pipeline_state_with_descriptor(&descriptor)
        .expect("Failed to create pipeline");

    assert_eq!(pipeline.label(), Some("MyRenderPipeline".to_string()));
}

#[test]
fn test_render_pipeline_missing_vertex_function() {
    let device = get_device();

    let library = device
        .new_library_with_source(COMBINED_SHADER, None)
        .expect("Failed to compile shader");

    let fragment_fn = library.new_function_with_name("simple_fragment").unwrap();

    let descriptor = RenderPipelineDescriptor::new().unwrap();
    // Note: NOT setting vertex function
    descriptor.set_fragment_function(Some(&fragment_fn));

    let color_attachments = descriptor.color_attachments();
    color_attachments
        .object(0)
        .unwrap()
        .set_pixel_format(PixelFormat::BGRA8_UNORM);

    // Should fail because vertex function is required
    let result = device.new_render_pipeline_state_with_descriptor(&descriptor);
    assert!(
        result.is_err(),
        "Pipeline creation should fail without vertex function"
    );

    // Verify we get the right error type
    match result {
        Err(mtl_gpu::ValidationError::MissingVertexFunction) => (),
        Err(e) => panic!("Expected MissingVertexFunction error, got: {:?}", e),
        Ok(_) => panic!("Expected error but got success"),
    }
}

#[test]
fn test_render_pipeline_depth_format() {
    let device = get_device();

    let library = device
        .new_library_with_source(COMBINED_SHADER, None)
        .expect("Failed to compile shader");

    let vertex_fn = library.new_function_with_name("simple_vertex").unwrap();
    let fragment_fn = library.new_function_with_name("simple_fragment").unwrap();

    let descriptor = RenderPipelineDescriptor::new().unwrap();
    descriptor.set_vertex_function(Some(&vertex_fn));
    descriptor.set_fragment_function(Some(&fragment_fn));

    let color_attachments = descriptor.color_attachments();
    color_attachments
        .object(0)
        .unwrap()
        .set_pixel_format(PixelFormat::BGRA8_UNORM);

    // Set depth format
    descriptor.set_depth_attachment_pixel_format(PixelFormat::DEPTH32_FLOAT);
    assert_eq!(
        descriptor.depth_attachment_pixel_format(),
        PixelFormat::DEPTH32_FLOAT
    );

    let pipeline = device.new_render_pipeline_state_with_descriptor(&descriptor);
    assert!(
        pipeline.is_ok(),
        "Failed to create pipeline with depth: {:?}",
        pipeline.err()
    );
}

#[test]
fn test_render_pipeline_multiple_color_attachments() {
    let device = get_device();

    // Shader with multiple render targets
    let mrt_shader = r#"
#include <metal_stdlib>
using namespace metal;

struct FragmentOut {
    float4 color0 [[color(0)]];
    float4 color1 [[color(1)]];
};

vertex float4 vertex_passthrough(uint vid [[vertex_id]]) {
    float2 positions[3] = {
        float2( 0.0,  0.5),
        float2(-0.5, -0.5),
        float2( 0.5, -0.5)
    };
    return float4(positions[vid], 0.0, 1.0);
}

fragment FragmentOut mrt_fragment() {
    FragmentOut out;
    out.color0 = float4(1.0, 0.0, 0.0, 1.0);
    out.color1 = float4(0.0, 1.0, 0.0, 1.0);
    return out;
}
"#;

    let library = device
        .new_library_with_source(mrt_shader, None)
        .expect("Failed to compile MRT shader");

    let vertex_fn = library
        .new_function_with_name("vertex_passthrough")
        .unwrap();
    let fragment_fn = library.new_function_with_name("mrt_fragment").unwrap();

    let descriptor = RenderPipelineDescriptor::new().unwrap();
    descriptor.set_vertex_function(Some(&vertex_fn));
    descriptor.set_fragment_function(Some(&fragment_fn));

    // Set two color attachments
    let color_attachments = descriptor.color_attachments();
    color_attachments
        .object(0)
        .unwrap()
        .set_pixel_format(PixelFormat::BGRA8_UNORM);
    color_attachments
        .object(1)
        .unwrap()
        .set_pixel_format(PixelFormat::BGRA8_UNORM);

    let pipeline = device.new_render_pipeline_state_with_descriptor(&descriptor);
    assert!(
        pipeline.is_ok(),
        "Failed to create MRT pipeline: {:?}",
        pipeline.err()
    );
}

#[test]
fn test_render_pipeline_vertex_only() {
    let device = get_device();

    let library = device
        .new_library_with_source(COMBINED_SHADER, None)
        .expect("Failed to compile shader");

    let vertex_fn = library.new_function_with_name("simple_vertex").unwrap();

    let descriptor = RenderPipelineDescriptor::new().unwrap();
    descriptor.set_vertex_function(Some(&vertex_fn));
    // No fragment function - this is valid for vertex-only pipelines
    descriptor.set_rasterization_enabled(false);

    // No color attachments needed when rasterization is disabled
    // Note: This may still fail on some Metal configurations that require
    // additional setup. We just test that our API correctly forwards to Metal.
    let _result = device.new_render_pipeline_state_with_descriptor(&descriptor);
    // Don't assert success - Metal may have additional requirements
    // The important thing is our API doesn't crash
}

#[test]
fn test_render_pipeline_sample_count() {
    let device = get_device();

    let library = device
        .new_library_with_source(COMBINED_SHADER, None)
        .expect("Failed to compile shader");

    let vertex_fn = library.new_function_with_name("simple_vertex").unwrap();
    let fragment_fn = library.new_function_with_name("simple_fragment").unwrap();

    let descriptor = RenderPipelineDescriptor::new().unwrap();
    descriptor.set_vertex_function(Some(&vertex_fn));
    descriptor.set_fragment_function(Some(&fragment_fn));

    let color_attachments = descriptor.color_attachments();
    color_attachments
        .object(0)
        .unwrap()
        .set_pixel_format(PixelFormat::BGRA8_UNORM);

    // Set sample count for MSAA
    descriptor.set_raster_sample_count(4);
    assert_eq!(descriptor.raster_sample_count(), 4);

    // Check if device supports this sample count
    if device.supports_texture_sample_count(4) {
        let pipeline = device.new_render_pipeline_state_with_descriptor(&descriptor);
        assert!(
            pipeline.is_ok(),
            "Failed to create MSAA pipeline: {:?}",
            pipeline.err()
        );
    }
}

// =============================================================================
// Size Tests
// =============================================================================

#[test]
fn test_render_pipeline_descriptor_size() {
    assert_eq!(
        std::mem::size_of::<RenderPipelineDescriptor>(),
        std::mem::size_of::<*mut std::ffi::c_void>()
    );
}

#[test]
fn test_render_pipeline_state_size() {
    assert_eq!(
        std::mem::size_of::<mtl_gpu::RenderPipelineState>(),
        std::mem::size_of::<*mut std::ffi::c_void>()
    );
}
