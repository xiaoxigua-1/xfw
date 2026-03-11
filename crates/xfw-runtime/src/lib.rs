mod lua;
pub mod state;
mod tree;

use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use parking_lot::RwLock;

use xfw_cli::RuntimeConfig;
use xfw_layout::{LayoutEngine, RenderObjectConverter, RenderObjectTree};
use xfw_platform::PlatformSurface;
use xfw_render::Renderer;

use crate::state::StateRegistry;

pub struct Runtime {
    config: RuntimeConfig,
    lua: lua::LuaEngine,
    layout: LayoutEngine,
    renderer: Renderer,
    platform: PlatformSurface,
    render_tree: Option<RenderObjectTree>,
    #[allow(dead_code)]
    state_registry: Arc<RwLock<StateRegistry>>,
}

impl Runtime {
    pub fn new(config: RuntimeConfig) -> Result<Self> {
        let lua = lua::LuaEngine::new()?;
        let state_registry = lua.state_registry();
        let layout = LayoutEngine::new();
        let renderer = Renderer::with_default_size();
        let platform = PlatformSurface::new()?;

        let mut runtime = Self {
            config,
            lua,
            layout,
            renderer,
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

    pub fn run(&mut self) -> Result<()> {
        tracing::info!("entrypoint" = ?self.config.entrypoint, "msg" = "bootstrapping runtime");
        self.load_config()?;
        self.rebuild_render_tree()?;
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

    pub fn on_state_change(&mut self, path: &str) -> Result<()> {
        tracing::debug!(path = %path, "state changed, rebuilding tree");
        self.rebuild_render_tree()?;
        Ok(())
    }
}
