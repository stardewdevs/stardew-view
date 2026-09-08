# Developer guide: configuration

## Config layout

All runtime configuration is JSON under `config/`:

```
config/
  effects/    # individual effect definitions
  schema/     # JSON schemas for validation
  themes/     # color themes
```

## Themes

A theme declares foreground, background, cursor, and selection colors plus a
16-256 entry ANSI palette. Colors are `#rrggbbaa` hex strings.

```json
{
  "name": "stardew-dark",
  "foreground": "#e0e0e0ff",
  "background": "#1a1c1ecc",
  "cursor": "#00ff66ff",
  "selection": "#264f78ff",
  "palette": ["#000000ff", "..."]
}
```

## Effects

Effects are either `screen` (full-frame pass) or `glyph` (per-glyph pass) and
carry a numeric priority. Higher priority runs later.

## Validation

`config/schema` holds draft-2020-12 JSON schemas. Editors and CI can validate
any theme or effect file against `theme.schema.json` / `effect.schema.json`.