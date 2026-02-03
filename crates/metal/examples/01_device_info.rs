//! Device Information Example
//!
//! This example demonstrates how to query Metal device properties and capabilities.
//! It's the simplest Metal program - just getting information about your GPU.
//!
//! Run with: cargo run --example 01_device_info

use metal::device;

fn main() {
    println!("Metal Device Information");
    println!("========================\n");

    // Get the system default Metal device (the primary GPU)
    let device = match device::system_default() {
        Some(device) => device,
        None => {
            eprintln!("Error: No Metal device found. Metal requires macOS or iOS with compatible hardware.");
            std::process::exit(1);
        }
    };

    // Basic Device Info
    println!("Basic Information:");
    println!("  Name:              {}", device.name());
    println!("  Registry ID:       {}", device.registry_id());
    if let Some(arch) = device.architecture() {
        println!("  Architecture:      {}", arch.name());
    }
    println!("  Location:          {:?}", device.location());

    // Device Type
    println!("\nDevice Type:");
    println!("  Unified Memory:    {}", device.has_unified_memory());
    println!("  Low Power:         {}", device.is_low_power());
    println!("  Headless:          {}", device.is_headless());
    println!("  Removable:         {}", device.is_removable());

    // Memory
    println!("\nMemory:");
    println!("  Currently Allocated:     {} bytes", device.current_allocated_size());
    println!("  Recommended Working Set: {} bytes", device.recommended_max_working_set_size());
    println!("  Max Buffer Length:       {} bytes", device.max_buffer_length());

    // Thread Limits
    let max_threads = device.max_threads_per_threadgroup();
    // Copy fields from packed struct to avoid unaligned reference
    let (width, height, depth) = (max_threads.width, max_threads.height, max_threads.depth);
    println!("\nCompute Limits:");
    println!("  Max Threads Per Threadgroup: {} x {} x {}", width, height, depth);
    println!("  Max Threadgroup Memory:      {} bytes", device.max_threadgroup_memory_length());

    // Feature Support
    println!("\nFeature Support:");
    println!("  Ray Tracing:               {}", device.supports_raytracing());
    println!("  Dynamic Libraries:         {}", device.supports_dynamic_libraries());
    println!("  Function Pointers:         {}", device.supports_function_pointers());
    println!("  Pull Model Interpolation:  {}", device.supports_pull_model_interpolation());
    println!("  32-bit Float Filtering:    {}", device.supports_32bit_float_filtering());
    println!("  32-bit MSAA:               {}", device.supports_32bit_msaa());
    println!("  BC Texture Compression:    {}", device.supports_bc_texture_compression());
    println!("  Query Texture LOD:         {}", device.supports_query_texture_lod());
    println!("  Barycentric Coordinates:   {}", device.supports_shader_barycentric_coordinates());
    println!("  Programmable Sample Pos:   {}", device.are_programmable_sample_positions_supported());
    println!("  Raster Order Groups:       {}", device.are_raster_order_groups_supported());

    // Texture Support
    println!("\nTexture Support:");
    println!("  Read/Write Texture Tier:   {:?}", device.read_write_texture_support());
    println!("  Argument Buffers Tier:     {:?}", device.argument_buffers_support());
    println!("  Sparse Tile Size:          {} bytes", device.sparse_tile_size_in_bytes());

    // Sample counts
    println!("\nSupported Sample Counts:");
    for sample_count in [1, 2, 4, 8, 16, 32] {
        if device.supports_texture_sample_count(sample_count) {
            println!("  Sample count {}: Supported", sample_count);
        }
    }

    // GPU Families
    use metal::GPUFamily;
    println!("\nGPU Family Support:");
    let families = [
        (GPUFamily::COMMON1, "Common1"),
        (GPUFamily::COMMON2, "Common2"),
        (GPUFamily::COMMON3, "Common3"),
        (GPUFamily::APPLE1, "Apple1"),
        (GPUFamily::APPLE2, "Apple2"),
        (GPUFamily::APPLE3, "Apple3"),
        (GPUFamily::APPLE4, "Apple4"),
        (GPUFamily::APPLE5, "Apple5"),
        (GPUFamily::APPLE6, "Apple6"),
        (GPUFamily::APPLE7, "Apple7"),
        (GPUFamily::APPLE8, "Apple8"),
        (GPUFamily::APPLE9, "Apple9"),
        (GPUFamily::METAL3, "Metal3"),
    ];
    for (family, name) in families {
        if device.supports_family(family) {
            println!("  {} - Supported", name);
        }
    }

    // Peer GPU Info (multi-GPU configurations)
    println!("\nMulti-GPU Configuration:");
    println!("  Peer Group ID: {}", device.peer_group_id());
    println!("  Peer Index:    {}", device.peer_index());
    println!("  Peer Count:    {}", device.peer_count());

    // Performance
    println!("\nPerformance:");
    println!("  Max Transfer Rate:                {} bytes/sec", device.max_transfer_rate());
    println!("  Max Concurrent Compilation Tasks: {}", device.maximum_concurrent_compilation_task_count());

    // Timestamps
    let (cpu_timestamp, gpu_timestamp) = device.sample_timestamps();
    println!("\nTimestamps:");
    println!("  CPU Timestamp: {}", cpu_timestamp);
    println!("  GPU Timestamp: {}", gpu_timestamp);
    println!("  Timestamp Frequency: {} Hz", device.query_timestamp_frequency());

    // List all available devices (macOS only)
    #[cfg(target_os = "macos")]
    {
        println!("\n\nAll Available Devices:");
        println!("======================");
        let devices = device::copy_all_devices();
        if devices.is_empty() {
            println!("  No devices found");
        } else {
            for (i, dev) in devices.iter().enumerate() {
                println!("  [{}] {} (Registry ID: {})", i, dev.name(), dev.registry_id());
            }
        }
    }

    println!("\n\nDevice information query complete.");
}
