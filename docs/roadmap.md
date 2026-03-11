# Development Roadmap

## Phase 1 — The Canvas
- [ ] Bootstrap `platform::wayland` to create a layer-shell surface with transparent background.
- [x] Render a static rectangle via `tiny-skia` to validate CPU raster path.
- [x] Add CLI flag validation + logging/tracing plumbing.

## Phase 2 — The Bridge & Layout
- [x] Embed Lua runtime, parse a sample widget tree (`lua/widgets/status_bar.lua`).
- [x] Convert Lua tables into `xfw-layout` nodes and compute frames via `taffy`.
- [ ] Serialize layout output back to Lua for debugging (inspection overlay flag).

## Phase 3 — Reactivity & Input
- [x] Implement observable stores in Lua and state diff channels in Rust.
- [ ] Add hit-testing + pointer event dispatch.
- [ ] Introduce dirty-rectangle tracking + partial rerendering.

## Phase 4 — Polish
- [x] Integrate `cosmic-text` for multilingual + emoji typography.
- [ ] Wire IPC adapters (DBus scripts, shell commands, user pipes).
- [ ] Provide tween helpers + easing curves for lightweight animations.

## Stretch Goals
- Benchmark suite comparing RAM/CPU usage with Waybar + AGS.
- Packaging for major distros and `nix` flake recipe.
- Config schema validator + language-server-style autocomplete for the Lua DSL.
