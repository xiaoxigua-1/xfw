use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use mlua::{Lua, RegistryKey, Table, Value};
use parking_lot::RwLock;
use xfw_model::UiNode;

use crate::state::{NodeId, StateChange, StateRegistry};
use crate::tree::ViewTreeBuilder;

pub struct LuaEngine {
    lua: Lua,
    render_tree: Option<RegistryKey>,
    state_registry: Arc<RwLock<StateRegistry>>,
    _state_change_tx: Option<Sender<String>>,
}

impl LuaEngine {
    pub fn new() -> Result<Self> {
        let lua = Lua::new();
        let state_registry = Arc::new(RwLock::new(StateRegistry::new()));
        let engine = Self {
            lua,
            render_tree: None,
            state_registry,
            _state_change_tx: None,
        };
        engine.initialize()?;
        Ok(engine)
    }

    fn initialize(&self) -> Result<()> {
        self.extend_package_path()?;
        self.bootstrap_globals()?;
        self.register_ffi()?;
        Ok(())
    }

    fn register_ffi(&self) -> Result<()> {
        let registry = self.state_registry.clone();

        self.lua.globals().set(
            "__xfw_register_state",
            self.lua
                .create_function(move |_lua, (node_id, path): (usize, String)| {
                    let mut reg = registry.write();
                    reg.register(path.clone().into(), NodeId::new(node_id));
                    tracing::debug!(node_id = node_id, path = %path, "registered state dependency");
                    Ok(())
                })?,
        )?;

        let registry = self.state_registry.clone();
        self.lua.globals().set(
            "__xfw_unregister_node",
            self.lua.create_function(move |_lua, node_id: usize| {
                let mut reg = registry.write();
                reg.unregister_node(NodeId::new(node_id));
                tracing::debug!(node_id = node_id, "unregistered node");
                Ok(())
            })?,
        )?;

        let registry = self.state_registry.clone();
        self.lua.globals().set(
            "__xfw_notify_state_change",
            self.lua
                .create_function(move |_lua, (path, value): (String, String)| {
                    let reg = registry.read();
                    let affected = reg.get_affected_nodes(&path);

                    let json_value: serde_json::Value =
                        serde_json::from_str(&value).unwrap_or(serde_json::Value::Null);

                    let _change = StateChange {
                        path: path.clone().into(),
                        value: json_value,
                    };

                    tracing::debug!(path = %path, affected = affected.len(), "state changed");
                    Ok(())
                })?,
        )?;

        Ok(())
    }

    fn extend_package_path(&self) -> Result<()> {
        let lua_dir = std::env::current_dir()?.join("lua");
        let lua_dir = lua_dir
            .to_str()
            .ok_or_else(|| anyhow!("lua directory path contains invalid UTF-8"))?;
        let chunk = format!(
            r#"
local dir = {dir};
package.path = package.path .. ';' .. dir .. '/?.lua;' .. dir .. '/?/init.lua;' .. dir .. '/?/?.lua';
"#,
            dir = Self::quote_lua(lua_dir)
        );
        let chunk = self.lua.load(&chunk).set_name("xfw::package_path");
        chunk.exec()?;
        Ok(())
    }

    fn quote_lua(value: &str) -> String {
        let escaped = value
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\'', "\\'");
        format!("'{}'", escaped)
    }

    fn bootstrap_globals(&self) -> Result<()> {
        let chunk = self
            .lua
            .load("require('framework.bootstrap')")
            .set_name("xfw::bootstrap");
        chunk.exec()?;
        Ok(())
    }

    pub fn load_entrypoint(&mut self, path: &Path) -> Result<()> {
        let code = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read lua file: {}", path.display()))?;
        let chunk = self.lua.load(&code).set_name(path.to_string_lossy());
        chunk.exec()?;
        self.capture_render_tree()
    }

    fn capture_render_tree(&mut self) -> Result<()> {
        let globals = self.lua.globals();
        let state: Table = globals
            .get("__xfw_state")
            .context("bootstrap state missing; ensure framework bootstrap ran")?;
        let root: Value = state.get("root")?;
        match root {
            Value::Table(table) => {
                let key = self.lua.create_registry_value(table)?;
                self.render_tree = Some(key);
                Ok(())
            }
            Value::Nil => Err(anyhow!("Lua config did not call UI.render(...)")),
            other => Err(anyhow!(
                "UI.render produced unexpected value type: {}",
                other.type_name()
            )),
        }
    }

    pub fn render_tree(&self) -> Result<Option<Table>> {
        if let Some(key) = &self.render_tree {
            let table: Table = self.lua.registry_value(key)?;
            Ok(Some(table))
        } else {
            Ok(None)
        }
    }

    pub fn build_view_tree(&mut self) -> Result<UiNode> {
        let table = self
            .render_tree()?
            .context("Lua config did not provide a UI tree")?;
        let builder = ViewTreeBuilder::new(self.lua.clone());
        builder.build(table)
    }

    pub fn state_registry(&self) -> Arc<RwLock<StateRegistry>> {
        self.state_registry.clone()
    }

    pub fn set_state_change_callback<F>(&self, callback: F)
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        let registry = self.state_registry.clone();
        let cb = Arc::new(callback);

        if let Ok(func) = self
            .lua
            .create_function(move |_lua, (path, value): (String, String)| {
                let reg = registry.read();
                let affected = reg.get_affected_nodes(&path);
                let json_value: serde_json::Value =
                    serde_json::from_str(&value).unwrap_or(serde_json::Value::Null);

                let _change = StateChange {
                    path: path.clone().into(),
                    value: json_value,
                };
                tracing::debug!(path = %path, affected = affected.len(), "state changed");
                cb(path);
                Ok(())
            })
        {
            let _ = self.lua.globals().set("__xfw_notify_state_change", func);
        }
    }
}
