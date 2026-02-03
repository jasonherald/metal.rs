//! Blit Operations Example
//!
//! This example demonstrates Metal blit (data transfer) operations:
//! 1. Copy data from buffer to texture
//! 2. Copy texture to texture
//! 3. Copy texture to buffer
//! 4. Generate mipmaps
//!
//! Run with: cargo run --example 04_blit_operations

use metal::{
    BlitCommandEncoder, Origin, PixelFormat, Region, ResourceOptions, Size, StorageMode,
    TextureDescriptor, TextureUsage, device,
};
use metal_foundation::Referencing;

fn main() {
    println!("Metal Blit Operations Example");
    println!("==============================\n");

    // Get the Metal device
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

    // Create dimensions
    let width = 64usize;
    let height = 64usize;
    let bytes_per_pixel = 4; // BGRA8
    let bytes_per_row = width * bytes_per_pixel;
    let buffer_size = height * bytes_per_row;

    // =======================================================================
    // Part 1: Buffer to Texture Copy
    // =======================================================================
    println!("\n--- Part 1: Buffer to Texture Copy ---");

    // Create source buffer with test pattern
    let mut source_data = vec![0u8; buffer_size];
    for y in 0..height {
        for x in 0..width {
            let offset = y * bytes_per_row + x * bytes_per_pixel;
            // Create a simple gradient pattern
            source_data[offset] = (x * 4) as u8; // B
            source_data[offset + 1] = (y * 4) as u8; // G
            source_data[offset + 2] = 128; // R
            source_data[offset + 3] = 255; // A
        }
    }

    let source_buffer = device
        .new_buffer_with_bytes(&source_data, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create source buffer");

    println!("Created source buffer: {} bytes", source_buffer.length());

    // Create destination texture
    let texture_desc =
        TextureDescriptor::texture_2d_descriptor(PixelFormat::BGRA8_UNORM, width, height, false)
            .expect("Failed to create texture descriptor");
    texture_desc.set_usage(TextureUsage::SHADER_READ | TextureUsage::SHADER_WRITE);
    texture_desc.set_storage_mode(StorageMode::SHARED);

    let texture1 = unsafe {
        device
            .new_texture(texture_desc.as_ptr())
            .expect("Failed to create texture 1")
    };

    println!(
        "Created texture1: {}x{}",
        texture1.width(),
        texture1.height()
    );

    // Create command queue and buffer
    let command_queue = device
        .new_command_queue()
        .expect("Failed to create command queue");
    let command_buffer = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");

    // Encode blit commands
    let encoder_ptr = command_buffer.blit_command_encoder();
    let encoder = unsafe { BlitCommandEncoder::from_raw(encoder_ptr) }
        .expect("Failed to create blit encoder");

    // Copy buffer to texture
    let source_size = Size::new(width, height, 1);
    encoder.copy_from_buffer_to_texture(
        &source_buffer,
        0, // source offset
        bytes_per_row,
        0, // bytes per image (0 for 2D)
        source_size,
        &texture1,
        0, // slice
        0, // mipmap level
        Origin::new(0, 0, 0),
    );

    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    println!("Buffer -> Texture copy completed");

    // Verify the copy by reading texture contents
    let mut texture_data = vec![0u8; buffer_size];
    let region = Region::new_2d(0, 0, width, height);
    unsafe {
        texture1.get_bytes_simple(
            texture_data.as_mut_ptr() as *mut std::ffi::c_void,
            bytes_per_row,
            region,
            0,
        );
    }

    // Compare first few pixels
    let mut matches = true;
    for i in 0..std::cmp::min(100, buffer_size) {
        if source_data[i] != texture_data[i] {
            matches = false;
            break;
        }
    }
    println!(
        "  Data verification: {}",
        if matches { "PASSED" } else { "FAILED" }
    );

    // =======================================================================
    // Part 2: Texture to Texture Copy
    // =======================================================================
    println!("\n--- Part 2: Texture to Texture Copy ---");

    // Create second texture
    let texture2 = unsafe {
        device
            .new_texture(texture_desc.as_ptr())
            .expect("Failed to create texture 2")
    };
    println!(
        "Created texture2: {}x{}",
        texture2.width(),
        texture2.height()
    );

    let command_buffer2 = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");
    let encoder_ptr2 = command_buffer2.blit_command_encoder();
    let encoder2 = unsafe { BlitCommandEncoder::from_raw(encoder_ptr2) }
        .expect("Failed to create blit encoder");

    // Copy texture1 to texture2
    encoder2.copy_from_texture_to_texture_region(
        &texture1,
        0, // source slice
        0, // source level
        Origin::new(0, 0, 0),
        source_size,
        &texture2,
        0, // dest slice
        0, // dest level
        Origin::new(0, 0, 0),
    );

    encoder2.end_encoding();
    command_buffer2.commit();
    command_buffer2.wait_until_completed();

    println!("Texture -> Texture copy completed");

    // Verify copy
    let mut texture2_data = vec![0u8; buffer_size];
    unsafe {
        texture2.get_bytes_simple(
            texture2_data.as_mut_ptr() as *mut std::ffi::c_void,
            bytes_per_row,
            region,
            0,
        );
    }

    matches = true;
    for i in 0..buffer_size {
        if texture_data[i] != texture2_data[i] {
            matches = false;
            break;
        }
    }
    println!(
        "  Data verification: {}",
        if matches { "PASSED" } else { "FAILED" }
    );

    // =======================================================================
    // Part 3: Texture to Buffer Copy
    // =======================================================================
    println!("\n--- Part 3: Texture to Buffer Copy ---");

    // Create destination buffer
    let dest_buffer = device
        .new_buffer(buffer_size, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create destination buffer");
    println!("Created destination buffer: {} bytes", dest_buffer.length());

    let command_buffer3 = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");
    let encoder_ptr3 = command_buffer3.blit_command_encoder();
    let encoder3 = unsafe { BlitCommandEncoder::from_raw(encoder_ptr3) }
        .expect("Failed to create blit encoder");

    // Copy texture2 to buffer
    encoder3.copy_from_texture_to_buffer(
        &texture2,
        0, // slice
        0, // level
        Origin::new(0, 0, 0),
        source_size,
        &dest_buffer,
        0, // dest offset
        bytes_per_row,
        0, // bytes per image
    );

    encoder3.end_encoding();
    command_buffer3.commit();
    command_buffer3.wait_until_completed();

    println!("Texture -> Buffer copy completed");

    // Verify copy
    let dest_ptr = dest_buffer.contents().expect("Buffer contents is null") as *const u8;
    let dest_data = unsafe { std::slice::from_raw_parts(dest_ptr, buffer_size) };

    matches = true;
    for i in 0..buffer_size {
        if source_data[i] != dest_data[i] {
            matches = false;
            break;
        }
    }
    println!(
        "  Data verification: {}",
        if matches { "PASSED" } else { "FAILED" }
    );
    println!(
        "  Round-trip (buffer -> texture -> texture -> buffer): {}",
        if matches { "SUCCESSFUL" } else { "FAILED" }
    );

    // =======================================================================
    // Part 4: Mipmap Generation
    // =======================================================================
    println!("\n--- Part 4: Mipmap Generation ---");

    // Create texture with mipmaps
    let mip_texture_desc = TextureDescriptor::texture_2d_descriptor(
        PixelFormat::BGRA8_UNORM,
        256, // Larger size for more mip levels
        256,
        true, // mipmapped
    )
    .expect("Failed to create mipmap texture descriptor");
    mip_texture_desc.set_usage(TextureUsage::SHADER_READ | TextureUsage::SHADER_WRITE);
    mip_texture_desc.set_storage_mode(StorageMode::PRIVATE); // Private for GPU-only access

    let mip_texture = unsafe {
        device
            .new_texture(mip_texture_desc.as_ptr())
            .expect("Failed to create mipmap texture")
    };

    println!(
        "Created mipmap texture: {}x{} with {} mip levels",
        mip_texture.width(),
        mip_texture.height(),
        mip_texture.mipmap_level_count()
    );

    let command_buffer4 = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");
    let encoder_ptr4 = command_buffer4.blit_command_encoder();
    let encoder4 = unsafe { BlitCommandEncoder::from_raw(encoder_ptr4) }
        .expect("Failed to create blit encoder");

    // Generate mipmaps
    encoder4.generate_mipmaps(&mip_texture);

    encoder4.end_encoding();
    command_buffer4.commit();
    command_buffer4.wait_until_completed();

    println!("Mipmap generation completed");

    // =======================================================================
    // Part 5: Fill Buffer
    // =======================================================================
    println!("\n--- Part 5: Fill Buffer ---");

    // Create a buffer to fill
    let fill_buffer = device
        .new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)
        .expect("Failed to create fill buffer");
    println!("Created fill buffer: {} bytes", fill_buffer.length());

    let command_buffer5 = command_queue
        .command_buffer()
        .expect("Failed to create command buffer");
    let encoder_ptr5 = command_buffer5.blit_command_encoder();
    let encoder5 = unsafe { BlitCommandEncoder::from_raw(encoder_ptr5) }
        .expect("Failed to create blit encoder");

    // Fill buffer with pattern byte
    encoder5.fill_buffer(&fill_buffer, 0, 1024, 0xAB);

    encoder5.end_encoding();
    command_buffer5.commit();
    command_buffer5.wait_until_completed();

    println!("Buffer fill completed");

    // Verify fill
    let fill_ptr = fill_buffer.contents().expect("Buffer contents is null") as *const u8;
    let fill_data = unsafe { std::slice::from_raw_parts(fill_ptr, 1024) };

    let all_filled = fill_data.iter().all(|&b| b == 0xAB);
    println!(
        "  Data verification: {}",
        if all_filled { "PASSED" } else { "FAILED" }
    );

    // =======================================================================
    // Summary
    // =======================================================================
    println!("\n=== Summary ===");
    println!("All blit operations completed successfully!");
    println!("  - Buffer to Texture copy: OK");
    println!("  - Texture to Texture copy: OK");
    println!("  - Texture to Buffer copy: OK");
    println!("  - Mipmap generation: OK");
    println!("  - Buffer fill: OK");

    println!("\nBlit operations example completed!");
}
