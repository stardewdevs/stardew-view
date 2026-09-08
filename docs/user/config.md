# User guide: configuration reference

## Palette

| Slot       | Field        | Default (stardew-dark) |
|------------|--------------|------------------------|
| Foreground | `foreground` | `#e0e0e0ff` |
| Background | `background` | `#1a1c1ecc` |
| Cursor     | `cursor`     | `#00ff66ff` |
| Selection  | `selection`  | `#264f78ff` |
| Ansi       | `palette`    | 16 entries, `#000000ff`...#ffffffff` |

## Effects

| Effect   | Config file        | Priority | Default |
|----------|--------------------|----------|---------|
| Blur     | `effects/blur.json`    | 10 | disabled |
| Glow     | `effects/glow.json`    | 20 | disabled |
| Shadow   | `effects/shadow.json`  | 30 | disabled |

Each effect is a JSON object with `name`, `type`, `params`, `enabled`, and
`priority`.

## Renderer options

- `backend` - `auto`, `vulkan`, `webgpu`, `opengl`, or `metal`.
- `present_mode` - `fifo` (default), `mailbox`, or `immediate`.
- `fonts.default` - font family name; `fonts.size` - size in points.

Config files are validated against the schemas in `config/schema`.