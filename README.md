# Stardew View

GPU-accelerated renderer for a Stardew-themed terminal.

Stardew View renders terminal state (grid, cursor, colors) on the GPU via
[wgpu](https://wgpu.rs) and [Sugarloaf](https://github.com/edfloreshz/sugarloaf).
It targets desktop (Vulkan, OpenGL, Metal), the web (WebGPU / WASM), and
Android (Vulkan + JNI).

## Features

- GPU-accelerated grid rendering with a font-atlas text pipeline
- Cross-platform backends: Vulkan, WebGPU, OpenGL, Metal
- Android JNI bridge (`io.stardew.view.StardewViewNative`)
- C and Kotlin FFI exports
- Configurable themes and screen/glyph effects (blur, glow, shadow)
- Criterion benchmarks and a full CI suite

## Building

Requires Rust 1.75+ and a wgpu-compatible GPU.

```sh
cargo build --release
```

Android cross-build (requires `cargo-ndk` and the NDK):

```sh
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86 -t x86_64 build --release
```

## Examples

```sh
cargo run --example basic
cargo run --example shader_preview
```

## Testing

```sh
cargo test
cargo test --test integration
cargo bench
```

## Configuration

Themes and effects live under `config/`:

- `config/themes` - color themes
- `config/effects` - blur, glow, shadow definitions
- `config/schema` - JSON schemas for validation

See `docs/user/` for usage and `docs/architecture/` for design details.

## License

Apache-2.0