//! Render Triangle Example
//!
//! This example demonstrates a complete Metal render pipeline workflow:
//! 1. Create render pipeline with vertex and fragment shaders
//! 2. Create a texture to render into
//! 3. Set up a render pass descriptor
//! 4. Draw a triangle
//! 5. Read the texture contents to verify
//!
//! Run with: cargo run --example 03_render_triangle

use mtl::{
    ClearColor, LoadAction, PixelFormat, PrimitiveType, RenderCommandEncoder,
    RenderPipelineDescriptor, RenderPipelineState, StorageMode, StoreAction, TextureDescriptor,
    TextureUsage, device,
};
use mtl_foundation::Referencing;

/// Metal Shading Language (MSL) source code for our render shaders.
const SHADER_SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct VertexOut {
    float4 position [[position]];
    float4 color;
};

vertex VertexOut vertex_main(
    uint vertex_id [[vertex_id]]
) {
    // Define a triangle in clip space
    float2 positions[3] = {
        float2( 0.0,  0.5),   // Top
        float2(-0.5, -0.5),   // Bottom left
        float2( 0.5, -0.5)    // Bottom right
    };

    float4 colors[3] = {
        float4(1.0, 0.0, 0.0, 1.0),  // Red
        float4(0.0, 1.0, 0.0, 1.0),  // Green
        float4(0.0, 0.0, 1.0, 1.0)   // Blue
    };

    VertexOut out;
    out.position = float4(positions[vertex_id], 0.0, 1.0);
    out.color = colors[vertex_id];
    return out;
}

fragment float4 fragment_main(VertexOut in [[stage_in]]) {
    return in.color;
}
"#;

fn main() {
    println!("Metal Render Triangle Example");
    println!("==============================\n");

    // Step 1: Get the Metal device
    let device = match device::system_default() {
        Some(device) => {
            println!("Using device: {}", device.name());
            device
        }
        None => {
            eprintln!("Error: No Metal device found.");
            std::process::exit(1);
        }
    };

    // Step 2: Create texture to render into
    let texture_width = 512usize;
    let texture_height = 512usize;

    let texture_desc = TextureDescriptor::texture_2d_descriptor(
        PixelFormat::BGRA8_UNORM,
        texture_width,
        texture_height,
        false, // no mipmaps
    )
    .expect("Failed to create texture descriptor");

    texture_desc.set_usage(TextureUsage::RENDER_TARGET | TextureUsage::SHADER_READ);
    texture_desc.set_storage_mode(StorageMode::SHARED);

    let texture = unsafe {
        device
            .new_texture(texture_desc.as_ptr())
            .expect("Failed to create texture")
    };

    println!(
        "Created render target: {}x{} {:?}",
        texture.width(),
        texture.height(),
        texture.pixel_format()
    );

    // Step 3: Compile shaders and create pipeline
    println!("\nCompiling shaders...");
    let library = device
        .new_library_with_source(SHADER_SOURCE, None)
        .expect("Failed to compile shaders");

    let vertex_function = library
        .new_function_with_name("vertex_main")
        .expect("Vertex function not found");

    let fragment_function = library
        .new_function_with_name("fragment_main")
        .expect("Fragment function not found");

    println!("  Vertex shader: {:?}", vertex_function.name());
    println!("  Fragment shader: {:?}", fragment_function.name());

    // Create render pipeline descriptor
    let pipeline_desc =
        RenderPipelineDescriptor::new().expect("Failed to create pipeline descriptor");
    pipeline_desc.set_vertex_function(Some(&vertex_function));
    pipeline_desc.set_fragment_function(Some(&fragment_function));

    // Set the pixel format for the color attachment
    let color_attachments = pipeline_desc.color_attachments();
    if let Some(color_attachment) = color_attachments.object(0) {
        color_attachment.set_pixel_format(PixelFormat::BGRA8_UNORM);
    }

    // Create the pipeline state
    let pipeline: RenderPipelineState = unsafe {
        device
            .new_render_pipeline_state(pipeline_desc.as_ptr())
            .expect("Failed to create render pipeline")
    };

    println!("Render pipeline created");

    // Step 4: Create command queue and buffer
    let command_queue = device
        .new_command_queue()
        .expect("Failed to create command queue");
    let command_buffer = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");

    // Step 5: Set up render pass descriptor
    let render_pass_desc =
        mtl::RenderPassDescriptor::new().expect("Failed to create render pass descriptor");

    if let Some(color_attachments) = render_pass_desc.color_attachments() {
        if let Some(color_attachment) = color_attachments.object_at(0) {
            color_attachment.set_texture(Some(&texture));
            color_attachment.set_load_action(LoadAction::CLEAR);
            color_attachment.set_store_action(StoreAction::STORE);
            color_attachment.set_clear_color(ClearColor::new(0.1, 0.1, 0.1, 1.0)); // Dark gray background
        }
    }

    // Step 6: Encode render commands
    let encoder_ptr = command_buffer.render_command_encoder(&render_pass_desc);
    let encoder = unsafe { RenderCommandEncoder::from_raw(encoder_ptr) }
        .expect("Failed to create render encoder");

    encoder.set_render_pipeline_state(&pipeline);

    // Set viewport
    let viewport = mtl::Viewport {
        origin_x: 0.0,
        origin_y: 0.0,
        width: texture_width as f64,
        height: texture_height as f64,
        znear: 0.0,
        zfar: 1.0,
    };
    encoder.set_viewport(viewport);

    // Draw the triangle (3 vertices)
    println!("\nDrawing triangle...");
    encoder.draw_primitives(PrimitiveType::TRIANGLE, 0, 3);

    encoder.end_encoding();
    command_buffer.commit();

    println!("Waiting for GPU...");
    command_buffer.wait_until_completed();

    // Step 7: Verify the render by reading texture contents
    println!("\nVerifying render output...");

    // Read pixel data from texture
    let bytes_per_pixel = 4; // BGRA8
    let bytes_per_row = texture_width * bytes_per_pixel;
    let data_size = texture_height * bytes_per_row;
    let mut pixel_data = vec![0u8; data_size];

    let region = mtl::Region::new_2d(0, 0, texture_width, texture_height);
    unsafe {
        texture.get_bytes_simple(
            pixel_data.as_mut_ptr() as *mut std::ffi::c_void,
            bytes_per_row,
            region,
            0,
        );
    }

    // Analyze the pixels
    let mut non_background_pixels = 0;
    let mut red_pixels = 0;
    let mut green_pixels = 0;
    let mut blue_pixels = 0;

    for y in 0..texture_height {
        for x in 0..texture_width {
            let offset = y * bytes_per_row + x * bytes_per_pixel;
            let b = pixel_data[offset];
            let g = pixel_data[offset + 1];
            let r = pixel_data[offset + 2];
            let _a = pixel_data[offset + 3];

            // Check if this isn't the background color (approximately 0.1, 0.1, 0.1)
            if r > 30 || g > 30 || b > 30 {
                non_background_pixels += 1;

                // Determine dominant color
                if r > g && r > b {
                    red_pixels += 1;
                } else if g > r && g > b {
                    green_pixels += 1;
                } else if b > r && b > g {
                    blue_pixels += 1;
                }
            }
        }
    }

    println!("\nRender Analysis:");
    println!("  Total pixels: {}", texture_width * texture_height);
    println!(
        "  Triangle pixels (non-background): {}",
        non_background_pixels
    );
    println!("  Red-dominant pixels: {}", red_pixels);
    println!("  Green-dominant pixels: {}", green_pixels);
    println!("  Blue-dominant pixels: {}", blue_pixels);

    // Verify we rendered something
    if non_background_pixels > 0 {
        println!("\nSuccess! Triangle was rendered.");

        // A triangle should cover roughly 25% of a 512x512 texture
        // (half width * half height * 0.5 for triangle)
        let coverage = non_background_pixels as f64 / (texture_width * texture_height) as f64;
        println!("  Coverage: {:.1}%", coverage * 100.0);

        // Should have all three vertex colors present (red, green, blue)
        if red_pixels > 0 && green_pixels > 0 && blue_pixels > 0 {
            println!("  All three vertex colors present - gradient working!");
        }
    } else {
        println!("\nError: No triangle pixels found!");
        std::process::exit(1);
    }

    // Print timing
    let gpu_start = command_buffer.gpu_start_time();
    let gpu_end = command_buffer.gpu_end_time();

    println!("\nTiming:");
    println!("  GPU time: {:.6} ms", (gpu_end - gpu_start) * 1000.0);

    println!("\nRender example completed successfully!");
}
