# Lua DSL Cheatsheet

```lua
UI.render(Window({
  id = "status-bar",
  style = {
    flex_direction = "row",
    gap = 8,
    padding = "12px 16px",
    bg_color = "#0f1117dd",
    border_radius = 12,
  },
  children = {
    Text({ value = "12:34" }),
    View({ style = { flex_grow = 1 } }),
    Text({ value = "Battery: 98%" }),
  },
}))
```

## Core Concepts
- **Elements:** `View`, `Text`, `Image`, `Row`, `Column`, `Window`, `Button`. Each returns a Lua table with metadata.
- **Styles:** Map directly to layout + render properties (via `taffy` and render style).
- **State:** `UI.state(...)` creates reactive stores; runtime invalidates affected nodes when stores change.
- **Overflow:** `overflow = "hidden"` or `clip = true` clips children to the parent bounds.

## File Layout
- `lua/framework/*.lua` — Helpers, DSL constructors, runtime bridge.
- `lua/widgets/*.lua` — User-authored bars/widgets (status bars, side panels, notification lists).

## Style Keys (Common)
- Layout: `flex_direction`, `flex_wrap`, `flex_grow`, `align_items`, `justify_content`, `gap`, `padding`, `margin`, `width`, `height`.
- Render: `bg_color`, `color`, `font_size`, `border_color`, `border_width`, `border_radius`, `opacity`.
- Text/Image: `text_align`, `image_fit`.
- Clipping: `overflow`, `clip`.
