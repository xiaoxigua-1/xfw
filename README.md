# xfw — Xiaoxigua Flash Widget

Xiaoxigua Flash Widget (xfw) is an ultra-lightweight Wayland bar/widget runtime that pairs a high-performance Rust core with hot-reloadable Lua declarative views. The framework targets Linux ricers that want AGS-level ergonomics with the RAM footprint of a small C binary.

## Highlights
- Declarative Lua DSL for describing views, layout metadata, and event bindings.
- Rust runtime with `taffy` flex/grid layout, `tiny-skia` + `cosmic-text` CPU rendering, and smithay layer-shell integration.
- Single RenderObject tree (Flutter-style) with both layout style and render style.
- Observable state graph that invalidates only dirty rectangles; zero redraws when nothing changes.
- Hot-reload Lua modules without restarting the compositor session.
- Optional overflow clipping for containers (`overflow = "hidden"` or `clip = true`).
- Test-time PNG dumps for renderer/runtime inspection.

## Architecture

### RenderObject Tree (Single Tree)
```
RenderObjectTree
├── RenderObject::Container { layout_style, render_style, rect, children }
├── RenderObject::Text { layout_style, render_style, rect, content }
└── RenderObject::Image { layout_style, render_style, rect, path }
```

### Style Separation
- **layout_style**: taffy::Style (flex_direction, justify_content, gap, padding, margin, etc.)
- **render_style**: RenderStyle (color, font_size, background_color, border_color, border_radius, opacity)

### Render Pipeline
```
Lua config → RenderObjectConverter → RenderObjectTree (with layout_style + render_style)
    → LayoutEngine.compute_layout() → rect computed
    → Renderer.render() → DrawCommand list → tiny-skia
```

## Repository Layout
```
docs/ # Architecture notes, roadmap, interface specs
lua/ # Lua DSL, widgets, IPC helpers
crates/
  xfw/ # Binary entrypoint, CLI + logging bootstrap
  xfw-cli/ # CLI parsing + Lua config entrypoint selection
  xfw-model/ # Core data structures (UiNode, StyleSource, StateField)
  xfw-layout/ # taffy-powered layout graph, RenderObjectTree, converter
  xfw-platform/ # Wayland + event loop glue (future smithay integration)
  xfw-render/ # tiny-skia + cosmic-text renderer, DrawCommand
  xfw-runtime/ # Scheduler, Lua bridge, dirty rect orchestration
Cargo.toml # Workspace manifest
README.md
```

## Quick Start
```bash
# install dependencies (Wayland dev libs, LuaJIT) then build
cargo build --workspace

# run with a sample widget set (Lua defines layout + styles + events)
cargo run -p xfw-cli -- --config lua/widgets/status_bar.lua
```

## Testing
```bash
cargo test --workspace
```

### Render Debug Dumps
- **Renderer tests:** `XFW_RENDER_DUMP=1 cargo test -p xfw-render`
  - Dumps PNGs to `target/xfw-render-dumps/`
  - Test assets in `target/xfw-render-test-assets/`
- **Runtime tests:** `XFW_RUNTIME_DUMP=1 cargo test -p xfw-runtime`
  - Dumps PNGs to `target/xfw-runtime-dumps/frame.png`
  - Set `XFW_RUNTIME_DUMP_NAME` to customize the filename

## Configuration Model
- **Lua-first:** Every widget layout, style, and event binding is authored in Lua (`lua/widgets/*.lua`). The runtime loads the Lua tree through `mlua` and reacts without restarting.
- **State + Logic:** Lua modules describe observable stores, IPC handlers, and view trees in one place. Rust stays focused on layout math, rendering, and Wayland glue.
- **Future styling options:** Additional style descriptions (SCSS/CSS translators, theming DSLs) can compile down to the same Lua schema later, but raw Lua definitions remain the priority for now.

## Next Steps
1. Wire `xfw-render` output into Wayland buffers (`xfw-platform`).
2. Flesh out dirty-rectangle tracking and partial rerendering.
3. Extend Lua DSL coverage and validate style schemas (`docs/lua_dsl.md`).

See `docs/configuration.md` for Lua config expectations and `docs/roadmap.md` for the detailed multi-phase plan.
