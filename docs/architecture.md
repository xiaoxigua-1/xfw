# Architecture Overview

## High-Level Flow
1. **Lua DSL** (`lua/widgets/*.lua`) declares the widget tree using nested tables; these scripts are the configuration surface.
2. **Runtime Bridge** (`xfw-runtime`) loads the tree via `mlua`, assigns node IDs, and exposes state handles to Lua.
3. **Layout Engine** (`xfw-layout`) maps the tree into `taffy` nodes and computes absolute frames.
4. **Renderer** (`xfw-render`) converts the tree into draw commands and rasterizes them via `tiny-skia` + `cosmic-text`.
5. **Event Loop** blocks on `epoll`/Wayland file descriptors; external IPC injects state changes that invalidates targeted nodes only.
6. **Clipping** is opt-in via `overflow = "hidden"` or `clip = true` and enforced in the renderer.

```
Lua DSL ──IPC────┐
                 │  Hot reload / state patches
xfw-runtime─────┴──▶xfw-layout──▶xfw-render──▶Wayland Layer Surface
        ▲                                ▲
        └───────── dirty rect metadata ──┘
```

## Module Responsibilities

| Module | Purpose |
| --- | --- |
| `xfw-runtime` | Owns lifecycle, hot reload controller, scheduler, and dirty-rect tracking. |
| `xfw-runtime::lua` | Embeds LuaJIT, exposes DSL helpers, serializes node graphs, and maps observable state to Rust channels. |
| `xfw-layout` | Wraps `taffy` to compute flex/grid layouts, caches node styles, and reports frame diffs. |
| `xfw-render` | CPU renderer built on `tiny-skia` for primitives and `cosmic-text` for glyph shaping. |
| `xfw-platform` | Wayland + layer-shell setup, buffer management, input dispatch, epoll-driven wakeups. |
| `xfw-cli` | Parses CLI arguments/environment and points the runtime at a Lua config entrypoint. |

## Data Contracts
- **Node Tree Schema:** Each node carries `id`, `kind`, `props`, `style` tables; IDs stay stable for diffing.
- **Invalidation Messages:** `xfw-runtime::lua` sends `{ node_id, dirty_rect }` events; renderer batches by surface region.
- **Input Events:** Platform module normalizes pointer/button events and forwards them back to Lua with hit-test metadata.
- **Debug Dumps:** Renderer/runtime tests can emit PNGs to `target/xfw-render-dumps/` and `target/xfw-runtime-dumps/` when enabled via env vars.

## Performance Principles
- Prefer stack allocation and small structs; no dynamic trait objects on the hot path.
- Track dirty rectangles per node and collapse overlapping regions before rasterization.
- Sleep the event loop via OS blocking primitives; wake on Wayland fd or IPC pipes only.
- Keep Lua↔Rust FFI chatter coarse-grained (diff bundles, not per-pixel instructions).
