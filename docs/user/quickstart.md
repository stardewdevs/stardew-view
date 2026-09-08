# User guide

## What is Stardew View

Stardew View renders a Stardew-themed terminal using the GPU. It targets
desktop (Vulkan/OpenGL/Metal), the web (WebGPU/WASM), and Android (Vulkan plus
JNI).

## Running

Desktops run the example binaries:

```sh
cargo run --example basic
cargo run --example shader_preview
```

Android integrations launch the `io.stardew.view.StardewViewNative` activity
which drives the native renderer through JNI.

## Customizing appearance

Copy a theme from `config/themes` into your config path and edit colors.
Effects under `config/effects` toggle blur, glow, and shadow:

```sh
stardew-view --theme config/themes/stardew-classic.json
```

## Performance

Frames are presented with FIFO vsync by default. The `renderer.present_mode`
config option can select mailbox or immediate where supported. Use the
`FpsCounter` helper for diagnostics.