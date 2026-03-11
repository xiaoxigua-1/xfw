# AGENTS.md - AI Assistant & Contributor Guidelines

## 🤖 Project Identity
**Project:** xiaoxigua flash widget (xfw)  
**Mission:** Deliver a declarative, reactive, HTML/CSS-like developer experience for Linux Wayland ricers, while strictly capping system resource footprint to the absolute minimum (15-25MB RAM, 0.0% idle CPU).  
**Core Philosophy:** "Zero Compromise on Performance vs. Developer Experience."

As an AI Agent or developer contributing to this repository, you must strictly adhere to the rules, architecture, and constraints outlined in this document.

---

## 🛑 Strict Directives for AI Agents

When generating code, refactoring, or suggesting features, you must follow these absolute constraints:

### Directive 1: Event-Driven Sleep (Zero Idle CPU)
* **NEVER** use polling loops (e.g., `std::thread::sleep` in a continuous loop) for updating UI or checking system status.
* All hardware/system status checks (battery, volume, workspaces) must rely on OS-level File Descriptors (FDs), `inotify`, or UNIX Domain Sockets.
* The main event loop must block entirely on `epoll`/`mio` when idle. CPU usage must drop to exactly 0.0% when there is no user interaction or IPC event.

### Directive 2: Fine-Grained Reactivity & Dirty Rectangles
* **NEVER** trigger a full-screen redraw unless the window is newly mapped or explicitly resized by the compositor.
* When a Lua state changes, only recalculate the specific Taffy layout nodes affected.
* Rendering must use the **Dirty Rectangle Algorithm**: only instruct `tiny-skia` to clear and redraw the specific bounding box `(x, y, w, h)` of the mutated component.

### Directive 3: Zero-Copy and FFI Optimization
* Crossing the FFI boundary between Rust and Lua (`mlua`) is relatively expensive.
* **DO NOT** pass large data structures across the FFI boundary on every frame.
* Animations (tweening, interpolations) must be calculated natively in Rust. Lua should only dispatch the animation intent (e.g., `transition = "0.2s ease"`).

---

## 🏗️ Architecture & Tech Stack

This project enforces a strict **Frontend/Backend separation** within a single process. Do not introduce bloated frameworks (e.g., GTK, Qt, Slint, Webview).

* **Windowing/Protocol:** `smithay-client-toolkit` (Direct Wayland Layer Shell protocol; strictly NO X11 fallback).
* **Layout Engine:** `taffy` (Flexbox/Grid calculations).
* **Rendering:** `tiny-skia` + `cosmic-text` (Pure CPU software rendering; zero GPU/OpenGL/Vulkan overhead).
* **Event Loop:** Lightweight `mio` or raw `epoll` for IPC socket handling. Avoid heavy async runtimes like `tokio` unless absolutely necessary to keep the binary small.
* **Scripting Bridge:** `mlua` (Binding to LuaJIT for extreme runtime performance).

---

## 📂 Workspace Directory Structure

**xfw** follows a strict Rust Cargo Workspace architecture. Agents must respect crate boundaries and avoid circular dependencies.

```text
xfw/
├── Cargo.toml                  # Workspace root (defines workspace.members)
├── AGENTS.md                   # This file
└── crates/
    ├── xfw/                    # Main entry point & Facade. Glues everything together.
    │   ├── Cargo.toml
    │   └── src/main.rs         # Initializes CLI, logging, and starts the Runtime loop.
    ├── xfw-cli/                # Command Line Interface.
    │   ├── Cargo.toml
    │   └── src/                # Argument parsing (`clap`), daemon management.
    ├── xfw-layout/             # Layout Engine wrapper.
    │   ├── Cargo.toml
    │   └── src/                # Taffy Flexbox/Grid calculations. Outputs (x, y, w, h).
    ├── xfw-model/              # Core Data Structures (The dependency leaf).
    │   ├── Cargo.toml
    │   └── src/                # VDOM nodes, Enums (Style, Color), IPC message structs.
    ├── xfw-platform/           # OS & Windowing Layer.
    │   ├── Cargo.toml
    │   └── src/                # Wayland (SCTK) Layer Shell, input events, Epoll/mio fds.
    ├── xfw-render/             # CPU Rendering Engine.
    │   ├── Cargo.toml
    │   └── src/                # tiny-skia, cosmic-text, and Dirty Rectangle painting.
    └── xfw-runtime/            # Execution Engine & Scripting Bridge.
        ├── Cargo.toml
        └── src/                # mlua bindings, Reactivity engine, event orchestration.

```

### 🧱 Dependency Rules

1. **`xfw-model`** is the lowest level. It must **never** depend on other internal `xfw-*` crates.
2. Cross-crate FFI or event payloads must be defined in `xfw-model`.
3. The **Dirty Rectangle Algorithm** state is managed by `xfw-runtime`, layout bounding boxes are provided by `xfw-layout`, and actual pixel mutation is strictly executed in `xfw-render`.
4. **`xfw-platform`** is solely responsible for Wayland buffers and OS file descriptors. It should not know about Lua or Taffy.


## 🛠️ Language-Specific Guidelines

### Rust Guidelines 🦀

* **Struct-Oriented UI:** The Virtual DOM representation in Rust should map cleanly to Taffy's `NodeId`.
* **Memory Safety over Speed:** Use idiomatic Rust. Use `Rc`/`RefCell` or `Arc`/`Mutex` only when necessary for bridging with `mlua`.
* **Error Handling:** Never `unwrap()` or `panic!()` in the main loop. Handle Wayland compositor disconnects and IPC socket drops gracefully.
* **Testing Directory:** All unit tests must be uniformly placed under the `tests/` directory of their respective crates. Avoid cluttering the `src/` directory with test modules.
* **No Magic Strings:** Strictly avoid hardcoding string comparisons in your logic (e.g., `if status == "active"`). Instead, uniformly use `enum`s to represent states and variants. This prevents typos, ensures type safety, and maintains high code readability.
* **Eliminating Code Duplication:** When standard generics cannot solve the problem, use macro_rules! to eliminate boilerplate by automatically generating identical code structures across multiple distinct types.
* **Tiered Documentation:** Differentiate comment depth based on visibility. For public APIs (`pub`), provide exhaustive `rustdoc` comments (`///`) detailing usage, along with `# Examples`, `# Errors`, and `# Panics` sections. For internal implementations, keep comments brief: summarize the function/struct's core purpose and explicitly state what data each field stores without unnecessary fluff.

### Lua Guidelines 🌙

* **Functional & Stateless Components:** UI components should be pure functions returning tables, reacting only to the global state.
* **Type Safety:** Every function, component, and table must be fully typed using LuaLS comments (`---@param`, `---@return`).
* **Closures for Reactivity:** State binding is done by passing anonymous functions to properties.

---

## 📄 Expected Lua Configuration Example

To help AI Agents understand the intended Developer Experience (DX), here is a reference implementation of a user's `config.lua`. Agents should design the Rust backend and `mlua` bindings to support this exact style:

```lua
-- 1. GLOBAL STATE (Deep Reactive Proxy)
local Store = UI.state({
    workspaces = { active = 1 },
    battery = { level = 100, is_charging = false },
})

-- 2. STATELESS COMPONENTS (Declarative Layout)
local function BatteryWidget()
    return Row({
        style = S({ align_items = "center", gap = 6, padding = "4px 8px" }),
        children = {
            Text({
                -- Reactive binding via closures. Rust only re-evaluates this node when Store changes.
                value = function()
                    return Store.battery.is_charging and "󰂄" or "󰁹"
                end,
                style = function()
                    local is_critical = (not Store.battery.is_charging) and (Store.battery.level <= 20)
                    return S({ color = is_critical and "#f38ba8" or "#a6e3a1" })
                end,
            }),
            Text({
                value = function() return Store.battery.level .. "%" end,
                style = S({ color = "#cdd6f4", font_size = 14 }),
            }),
        },
    })
end

-- 3. ROOT RENDERER (Wayland Layer Shell mapping)
UI.render(Window({
    anchor = "top",
    style = S({ bg_color = "rgba(30, 30, 46, 0.9)", padding = "5px 15px", border_radius = 12 }),
    children = {
        BatteryWidget(),
    },
}))

-- 4. EVENT-DRIVEN IPC (Zero polling, epoll/mio bindings)
IPC.watch_file("/sys/class/power_supply/BAT0/capacity", "on_change", function(value)
    Store.battery.level = tonumber(value) or 100
end)

```

---

## 🌿 Git Workflow & Branching Policy (Strictly Enforced)

**CRITICAL RULE: NEVER work directly on the `main` or `master` branch. You MUST follow this 3-step workflow.**

### Step 1: Branch Creation (Before Coding)
You MUST create and switch to a new branch BEFORE making any changes.
- **Suggested Format:** `<work-type>/<2-3-word-summary>/<ticket-id>`
- **MUST:** Use hyphens (`-`) to separate words. The summary MUST be short and in imperative present tense.
- **MAY:** Include the work type (e.g., `feature`, `refactor`, `bugfix`, `hotfix`) and a corresponding ticket/story ID (if applicable).
- **Example:** `git checkout -b feature/oauth-migration/ATL-244`

### Step 2: Component-Based Committing (During Coding)
Create a Git commit whenever a small component or sub-feature is completed **AND** compiles/runs successfully without errors. 

1. **Verify:** Run build/syntax checks to prove code validity.
2. **Stage:** Run `git add <specific_files>` (Avoid `git add .`).
3. **Commit (Strict Format):** Your commit message MUST consist of a **Subject** and an optional **Body**, separated by a blank line.
   - **Subject:** `<type>[optional scope]: <Description>`
     - Allowed types: `feat`, `fix`, `chore`, `docs`, `refactor`, `style`, `test`, `perf`.
     - Description MUST be: Max 50 chars, Capitalized, imperative present tense, and NO ending period.
   - **Body:** Explain *what* and *why*. Max 72 chars per line. Capitalize each paragraph. Note breaking changes here if any.
   
   **Format Example:**
    ```text
    feat(api): Implement access right management
    
    Implement proper authorization for each service on development phase to 
    validate during the API call. This prevents unauthorized data access.

    ```

*(Note: Use `git commit -m "<subject>" -m "<body>"` to pass multi-line messages securely in the terminal).*

4. **Continue:** Proceed ONLY AFTER the commit is successful.

### Step 3: End of Task Protocol (After Coding)

You are strictly PROHIBITED from merging into `main` or `master`. When the request is fully completed, explicitly output this message:

> "✅ **Task Completed.** All changes have been committed to the branch `[branch name]`. Please review the code, and manually merge this branch into `main` when you are ready."

---

**Agent Acknowledgment:** If you are an AI reading this, acknowledge these rules by optimizing all code output for maximum memory efficiency, avoiding polling, and prioritizing the Dirty Rectangle rendering pipeline.
Do not use triple backticks for reasoning. Wrap your thought process in  tags.
