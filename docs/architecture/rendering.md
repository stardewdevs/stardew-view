# Rendering pipeline

## Vertex stage

The terminal shader takes `position` and `tex_coords` attributes and emits a
clip-space position plus texture coordinates. See `shaders/wgsl/terminal_vertex.wgsl`.

## Fragment stage

The fragment shader samples the font atlas, then mixes background and
foreground colors by the sampled coverage:

```
color = mix(background, foreground, texel.r)
```

Uniforms carry background, foreground, and cursor colors.
See `shaders/wgsl/terminal_fragment.wgsl`.

## Texture atlas

Glyphs are rasterized into a single `R8Unorm` atlas (label `font_atlas`)
bound in group 0, sampled with a linear filter.

## Post-processing

Effects are organized by priority in `config/effects`:

- blur - separable Gaussian screen pass.
- glow - additive color glow around bright texels.
- shadow - offset drop shadow with blur.

GLSL sources live in `shaders/glsl` and are compiled to SPIR-V in `shaders/spv`.