# Stardew View API

This document describes the public API surface of `stardew-view`.

## Crate overview

`stardew-view` is a GPU-accelerated terminal renderer library. It exposes a
`cdylib` (for FFI/JNI consumers) and an `rlib` (for Rust consumers).

## Core type

`StardewView` holds the renderer, device, surface, and surface configuration.

- `StardewView::new(device, surface, config)` builds a view from existing wgpu state.
- `render(&Term)` renders an `alacritty_terminal::Term`.
- `resize(width, height)` reconfigures the surface.
- `present()` submits the current frame.

## Modules

- `renderer` - wraps `sugarloaf::Sugarloaf`; converts grid/cursor/colors.
- `gpu` - backends: `vulkan`, `webgpu`, `opengl`, `metal`.
- `surface` - per-platform surface creation: `android`, `linux`, `macos`, `windows`, `web`.
- `platform` - per-platform runtime state: `android`, `linux`, `macos`, `windows`.
- `pipeline` - GPU pipeline factories.
- `buffer` - GPU vertex buffers.
- `texture` - GPU textures and font atlases.
- `shader` - WGSL shader compilation.
- `font` - font loading via `crossfont`.
- `glyph` - glyph caching via `swash`/`glyphon`.
- `color` - `Color` and `Palette` types.
- `effect` - blur, glow, and shadow effects.
- `utils` - math, vector, matrix, timer, fps, profiler.
- `ffi` - C and Kotlin exports.
- `jni` - Android JNI bridge (`io.stardew.view.StardewViewNative`).
- `error` - `ViewError` and `Result` aliases.
- `logger` - `env_logger`-based logging setup.

## Error handling

`ViewError` is a `thiserror` enum with `GpuInit`, `SurfaceCreation`, `FontLoad`,
and `Jni` variants, each carrying a `String` message.

## Feature flags

- `default = ["jni"]` - enables the JNI bridge.
- `jni` - JNI dependencies.
- `android` - NDK and Android activity dependencies.

## FFI surface

The crate exports `stardew_view_init`, `stardew_view_create`,
`stardew_view_destroy`, `stardew_view_render`, `stardew_view_resize`, and
`stardew_view_get_version` as C ABI functions for embedding in native hosts.