# Architecture

Stardew View is a layered GPU terminal renderer.

## Layer overview

```
JNI / FFI exports
       |
       v
StardewView (lib.rs) - frame orchestration
       |
       v
Renderer (sugarloaf) - grid to glyph conversion
       |
       v
Pipeline / Buffer / Texture / Shader - wgpu resources
       |
       v
gpu backends (vulkan / webgpu / opengl / metal)
       |
       v
surface / platform - window and platform integration
```

## Module responsibilities

- `effect` applies post-processing (blur, glow, shadow) to the rendered frame.
- `glyph` and `font` rasterize terminal text into the font atlas.
- `color` maps terminal color indices to RGBA palettes.
- `utils` provides the math, timing, and profiling helpers shared across layers.
- `ffi`/`jni` expose the renderer to C and Android hosts.

## Data flow per frame

1. The host supplies terminal state (grid, cursor, palette).
2. `Renderer::render` converts grid cells into glyph instances.
3. `PipelineManager` binds vertex and fragment shaders.
4. Glyphs are drawn into the font atlas texture.
5. Effects run as post-processing passes.
6. `StardewView::present` submits the command queue and presents the frame.

## Threading model

JNI entry points lock a global `VIEW` mutex; all rendering happens on the
calling thread. Time-based helpers (`Timer`, `FpsCounter`) use
`std::time::Instant` and are not tied to any executor.