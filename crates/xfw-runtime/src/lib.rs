mod lua;
pub mod state;
mod tree;

use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use parking_lot::RwLock;

use xfw_cli::RuntimeConfig;
use xfw_layout::{LayoutEngine, RenderObject, RenderObjectConverter, RenderObjectTree};
use xfw_platform::PlatformSurface;
use xfw_render::{PixmapRenderer, Renderer};

use crate::state::StateRegistry;

/// Runtime orchestrator for loading Lua configs, computing layout, and rendering.
///
/// # Examples
/// ```rust,no_run
/// use xfw_cli::RuntimeConfig;
/// use xfw_runtime::Runtime;
/// let config = RuntimeConfig { entrypoint: "lua/widgets/status_bar.lua".into() };
/// let _runtime = Runtime::new(config).unwrap();
/// ```
///
/// # Errors
/// See `Runtime::new` and other methods for specific error conditions.
///
/// # Panics
/// None.
pub struct Runtime {
    config: RuntimeConfig,
    lua: lua::LuaEngine,
    layout: LayoutEngine,
    renderer: Renderer,
    pixmap: PixmapRenderer,
    platform: PlatformSurface,
    render_tree: Option<RenderObjectTree>,
    #[allow(dead_code)]
    state_registry: Arc<RwLock<StateRegistry>>,
}

impl Runtime {
    /// Creates a new runtime instance.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use xfw_cli::RuntimeConfig;
    /// use xfw_runtime::Runtime;
    /// let config = RuntimeConfig { entrypoint: "lua/widgets/status_bar.lua".into() };
    /// let _runtime = Runtime::new(config).unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns an error if Lua, renderer, or platform initialization fails.
    ///
    /// # Panics
    /// None.
    pub fn new(config: RuntimeConfig) -> Result<Self> {
        let lua = lua::LuaEngine::new()?;
        let state_registry = lua.state_registry();
        let layout = LayoutEngine::new();
        let renderer = Renderer::with_default_size();
        let (width, height) = renderer.size();
        let pixmap = PixmapRenderer::new(width, height)?;
        let platform = PlatformSurface::new()?;

        let mut runtime = Self {
            config,
            lua,
            layout,
            renderer,
            pixmap,
            platform,
            render_tree: None,
            state_registry,
        };

        runtime.setup_state_callback()?;
        Ok(runtime)
    }

    fn setup_state_callback(&mut self) -> Result<()> {
        let (tx, rx) = std::sync::mpsc::channel::<String>();
        self.lua.set_state_change_callback(move |path| {
            let _ = tx.send(path);
        });

        std::thread::spawn(move || {
            while let Ok(path) = rx.recv() {
                tracing::debug!(path = %path, "state changed");
            }
        });
        Ok(())
    }

    /// Runs the runtime event loop.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use xfw_cli::RuntimeConfig;
    /// use xfw_runtime::Runtime;
    /// let config = RuntimeConfig { entrypoint: "lua/widgets/status_bar.lua".into() };
    /// let mut runtime = Runtime::new(config).unwrap();
    /// // runtime.run().unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns an error when Lua loading, layout, rendering, or platform dispatch fails.
    ///
    /// # Panics
    /// None.
    pub fn run(&mut self) -> Result<()> {
        tracing::info!("entrypoint" = ?self.config.entrypoint, "msg" = "bootstrapping runtime");
        self.load_config()?;
        self.rebuild_render_tree()?;
        self.render_current_tree()?;
        tracing::info!(msg = "render tree built, ready");
        self.renderer.prepare()?;
        self.platform.dispatch_loop()
    }

    fn load_config(&mut self) -> Result<()> {
        let path = Path::new(&self.config.entrypoint);
        self.lua.load_entrypoint(path)?;
        Ok(())
    }

    fn rebuild_render_tree(&mut self) -> Result<()> {
        let ui_node = self.lua.build_view_tree()?;
        let converter = RenderObjectConverter::new();
        let root = converter.convert(&ui_node);
        let mut tree = RenderObjectTree::new(root);
        self.layout.compute_layout(&mut tree)?;
        self.render_tree = Some(tree);
        Ok(())
    }

    fn render_current_tree(&mut self) -> Result<()> {
        let render_tree = self
            .render_tree
            .as_ref()
            .ok_or_else(|| anyhow!("Render tree is not initialized"))?;
        let commands = self.renderer.render(render_tree, None)?;
        if std::env::var_os("XFW_RENDER_DEBUG").is_some() {
            tracing::info!(node_count = render_tree.node_count(), "render tree built");
            Self::log_render_tree(render_tree.root(), 0);
            tracing::info!(command_count = commands.len(), "render commands built");
            for (index, command) in commands.iter().take(20).enumerate() {
                tracing::debug!(index, command = ?command);
            }
            Self::dump_debug(render_tree.root(), &commands)?;
        }
        self.pixmap.clear((0.0, 0.0, 0.0, 0.0));
        self.pixmap.execute(&commands)?;
        if std::env::var_os("XFW_RUNTIME_DUMP").is_some() {
            let dump_name =
                std::env::var("XFW_RUNTIME_DUMP_NAME").unwrap_or_else(|_| "frame.png".to_string());
            std::fs::create_dir_all("target/xfw-runtime-dumps")?;
            let path = format!("target/xfw-runtime-dumps/{dump_name}");
            self.pixmap.pixmap_mut().save_png(&path)?;
        }
        Ok(())
    }

    fn log_render_tree(node: &RenderObject, depth: usize) {
        let render_style = node.render_style();
        tracing::debug!(
            depth,
            kind = ?node.kind(),
            id = node.id(),
            rect = ?node.rect(),
            bg_color = ?render_style.background_color,
            color = ?render_style.color,
            border_color = ?render_style.border_color,
            border_radius = ?render_style.border_radius,
            border_width = ?render_style.border_width,
            opacity = ?render_style.opacity,
            overflow = ?render_style.overflow,
            "render node"
        );
        if let Some(children) = node.children() {
            for child in children {
                Self::log_render_tree(child, depth + 1);
            }
        }
    }

    fn dump_debug(root: &RenderObject, commands: &[xfw_render::DrawCommand]) -> Result<()> {
        let mut lines = Vec::new();
        lines.push("RenderObject tree:".to_string());
        Self::dump_node(root, 0, &mut lines);
        lines.push("".to_string());
        lines.push("DrawCommands:".to_string());
        for (index, command) in commands.iter().enumerate() {
            lines.push(format!("{index}: {command:?}"));
        }
        std::fs::create_dir_all("target/xfw-runtime-dumps")?;
        std::fs::write("target/xfw-runtime-dumps/debug.txt", lines.join("\n"))?;
        Ok(())
    }

    fn dump_node(node: &RenderObject, depth: usize, lines: &mut Vec<String>) {
        let indent = "  ".repeat(depth);
        let render_style = node.render_style();
        lines.push(format!(
            "{indent}{:?} id={:?} rect={:?} bg={:?} color={:?} border={:?} radius={:?} opacity={:?} overflow={:?}",
            node.kind(),
            node.id(),
            node.rect(),
            render_style.background_color,
            render_style.color,
            render_style.border_color,
            render_style.border_radius,
            render_style.opacity,
            render_style.overflow,
        ));
        if let RenderObject::Text { content, .. } = node {
            lines.push(format!("{indent}  text=\"{content}\""));
        }
        if let Some(children) = node.children() {
            for child in children {
                Self::dump_node(child, depth + 1, lines);
            }
        }
    }

    /// Rebuilds layout and renders after a state change.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use xfw_cli::RuntimeConfig;
    /// use xfw_runtime::Runtime;
    /// let config = RuntimeConfig { entrypoint: "lua/widgets/status_bar.lua".into() };
    /// let mut runtime = Runtime::new(config).unwrap();
    /// let _ = runtime.on_state_change("store.battery");
    /// ```
    ///
    /// # Errors
    /// Returns an error if tree rebuild or rendering fails.
    ///
    /// # Panics
    /// None.
    pub fn on_state_change(&mut self, path: &str) -> Result<()> {
        tracing::debug!(path = %path, "state changed, rebuilding tree");
        self.rebuild_render_tree()?;
        self.render_current_tree()?;
        Ok(())
    }

    /// Loads, lays out, and renders the current config once.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use xfw_cli::RuntimeConfig;
    /// use xfw_runtime::Runtime;
    /// let config = RuntimeConfig { entrypoint: "lua/widgets/status_bar.lua".into() };
    /// let mut runtime = Runtime::new(config).unwrap();
    /// let (_w, _h, _data) = runtime.render_once().unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns an error if Lua loading, layout, or rendering fails.
    ///
    /// # Panics
    /// None.
    pub fn render_once(&mut self) -> Result<(u32, u32, Vec<u8>)> {
        self.load_config()?;
        self.rebuild_render_tree()?;
        self.render_current_tree()?;
        Ok((
            self.pixmap.width(),
            self.pixmap.height(),
            self.pixmap.data().to_vec(),
        ))
    }
}
