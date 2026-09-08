# API reference: graphics modules

## GPU backends

All backends expose the same construction shape:

```rust
let backend = VulkanBackend::new(surface, width, height).await;
backend.resize(width, height);
let device = backend.device();
```

Backend selection mirrors the `gpu` module:

| Backend   | Module    | wgpu backend     |
|-----------|-----------|------------------|
| Vulkan    | `gpu::vulkan`  | `Backends::VULKAN`     |
| WebGPU    | `gpu::webgpu`  | `Backends::BROWSER_WEBGPU` |
| OpenGL    | `gpu::opengl`  | `Backends::GL`          |
| Metal     | `gpu::metal`   | `Backends::METAL`       |

## Surfaces

`AndroidSurface::new(window, instance)` builds a wgpu surface from a `ndk`
`NativeWindow`. The desktop surfaces follow the same contract with a
`raw_window_handle::HasRawWindowHandle` window.

## Shaders

`ShaderManager::compile_vertex(source)` and `compile_fragment(source)` accept
WGSL sources and return a `wgpu::ShaderModule`.

## Textures

`TextureManager::create_font_atlas(device, width, height)` allocates an
`R8Unorm` texture labelled `font_atlas` for glyph rasterization.

## Pipelines

`PipelineManager::create_terminal_pipeline(device, layout, vertex, fragment)`
builds the terminal render pipeline with alpha blending over `Bgra8Unorm`.