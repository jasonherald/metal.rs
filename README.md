# metal.rs

[![crates.io](https://img.shields.io/crates/v/mtl-gpu.svg)](https://crates.io/crates/mtl-gpu)
[![Documentation](https://img.shields.io/badge/docs-github.io-blue)](https://jasonherald.github.io/metal.rs/mtl_gpu/)

Rust bindings to Apple's Metal framework. This is a direct conversion of [metal-cpp](https://developer.apple.com/metal/cpp/) to Rust, preserving the same API structure and naming conventions where possible.

94% of this codebase was converted from C++ to Rust by [Claude](https://claude.ai), Anthropic's AI assistant.

## Requirements

- macOS or iOS with Metal-capable hardware
- Rust 1.85+

## Crates

| Crate | Description |
|-------|-------------|
| [`mtl-gpu`](https://crates.io/crates/mtl-gpu) | Safe Metal bindings - devices, resources, commands, pipelines, encoders |
| [`mtl-sys`](https://crates.io/crates/mtl-sys) | Low-level Objective-C FFI (zero external dependencies) |
| [`mtl-foundation`](https://crates.io/crates/mtl-foundation) | Foundation framework bindings (NSObject, NSString, NSArray, etc.) |
| [`mtl-fx`](https://crates.io/crates/mtl-fx) | MetalFX bindings (SpatialScaler, TemporalScaler, FrameInterpolator) |
| [`mtl-quartz-core`](https://crates.io/crates/mtl-quartz-core) | CAMetalLayer/CAMetalDrawable for display integration |

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
mtl-gpu = "1.0"
```

### Example: Query device info

```rust
use mtl_gpu::device;

fn main() {
    let device = device::system_default().expect("No Metal device");

    println!("Device: {}", device.name());
    println!("Unified memory: {}", device.has_unified_memory());
    println!("Max buffer length: {} bytes", device.max_buffer_length());
}
```

### Example: Run a compute shader

```rust
use mtl_gpu::{device, ComputeCommandEncoder, ResourceOptions, Size};

const SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void double_values(device float* data [[buffer(0)]], uint id [[thread_position_in_grid]]) {
    data[id] = data[id] * 2.0;
}
"#;

fn main() {
    let device = device::system_default().expect("No Metal device");

    // Create buffer with data
    let data: Vec<f32> = (0..16).map(|i| i as f32).collect();
    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 4)
    };
    let buffer = device.new_buffer_with_bytes(bytes, ResourceOptions::STORAGE_MODE_SHARED).unwrap();

    // Compile shader and create pipeline
    let library = device.new_library_with_source(SHADER, None).unwrap();
    let function = library.new_function_with_name("double_values").unwrap();
    let pipeline = device.new_compute_pipeline_state_with_function(&function).unwrap();

    // Encode and dispatch
    let queue = device.new_command_queue().unwrap();
    let cmd_buffer = queue.command_buffer().unwrap();
    let encoder = unsafe { ComputeCommandEncoder::from_raw(cmd_buffer.compute_command_encoder()) }.unwrap();

    encoder.set_compute_pipeline_state(&pipeline);
    encoder.set_buffer(&buffer, 0, 0);
    encoder.dispatch_threadgroups(Size::new(1, 1, 1), Size::new(16, 1, 1));
    encoder.end_encoding();

    cmd_buffer.commit();
    cmd_buffer.wait_until_completed();

    // Read results
    let result = unsafe { std::slice::from_raw_parts(buffer.contents().unwrap() as *const f32, 16) };
    println!("{:?}", result); // [0.0, 2.0, 4.0, 6.0, ...]
}
```

## Running Examples

```bash
cargo run --example 01_device_info
cargo run --example 02_buffer_compute
cargo run --example 03_render_triangle
# ... examples 04-10 cover blit operations, async completion, and Metal 4 features
```

## API Coverage

- 253/253 classes (100%)
- 2963/3175 methods (93%)
- 125/125 enums (100%)

See `docs/API_COVERAGE.md` for details.

## License

MIT OR Apache-2.0
