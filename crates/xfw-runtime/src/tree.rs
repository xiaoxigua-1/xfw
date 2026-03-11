use anyhow::{anyhow, Result};
use mlua::{Lua, Table, Value};
use xfw_model::{
    ContentSource, EventBinding, EventKind, NodeKind, StateField, StateValue, StyleAttribute,
    StyleSource, StyleValue, UiNode,
};

const RESERVED_KEYS: &[&str] = &[
    "id",
    "kind",
    "children",
    "style",
    "value",
    "on_click",
    "on_scroll",
    "on_hover",
    "anchor",
];

pub struct ViewTreeBuilder;

impl ViewTreeBuilder {
    pub fn new(_lua: Lua) -> Self {
        Self
    }

    pub fn build(&self, table: Table) -> Result<UiNode> {
        self.node_from_table(table)
    }

    fn node_from_table(&self, table: Table) -> Result<UiNode> {
        let kind_str: Option<String> = table.get("kind").ok();
        let kind = kind_str
            .as_deref()
            .map(NodeKind::parse)
            .unwrap_or(NodeKind::View);

        let mut node = UiNode::new(kind);
        node.id = table.get("id").ok();
        node.props.style = self.parse_style(&table)?;
        node.props.value = self.parse_value(&table)?;
        node.props.events = self.parse_events(&table)?;
        node.props.state = self.parse_state(&table)?;
        node.children = self.parse_children(&table)?;
        Ok(node)
    }

    fn parse_children(&self, table: &Table) -> Result<Vec<UiNode>> {
        let mut nodes = Vec::new();
        if let Ok(children) = table.get::<Table>("children") {
            for child in children.sequence_values::<Table>() {
                nodes.push(self.node_from_table(child?)?);
            }
        }
        Ok(nodes)
    }

    fn parse_style(&self, table: &Table) -> Result<StyleSource> {
        match table.get::<Value>("style")? {
            Value::Table(style_table) => {
                let mut entries = Vec::new();
                for pair in style_table.pairs::<Value, Value>() {
                    let (key, value) = pair?;
                    let name = match key {
                        Value::String(s) => s.to_str()?.to_string(),
                        _ => continue,
                    };
                    if let Some(style_value) = self.convert_style_value(value)? {
                        entries.push(StyleAttribute {
                            name,
                            value: style_value,
                        });
                    }
                }
                Ok(StyleSource::Static(entries))
            }
            Value::Function(_) => Ok(StyleSource::Dynamic),
            _ => Ok(StyleSource::None),
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn convert_style_value(&self, value: Value) -> Result<Option<StyleValue>> {
        Ok(match value {
            Value::Nil => None,
            Value::String(s) => Some(StyleValue::String(s.to_str()?.to_string())),
            Value::Integer(i) => Some(StyleValue::Integer(i)),
            Value::Number(n) => Some(StyleValue::Number(n as f32)),
            Value::Boolean(b) => Some(StyleValue::Bool(b)),
            Value::Table(table) => {
                let mut items = Vec::new();
                for value in table.sequence_values::<Value>() {
                    if let Some(entry) = self.convert_style_value(value?)? {
                        items.push(entry);
                    }
                }
                Some(StyleValue::Array(items))
            }
            _ => None,
        })
    }

    fn parse_value(&self, table: &Table) -> Result<Option<ContentSource>> {
        match table.get::<Value>("value")? {
            Value::Nil => Ok(None),
            Value::String(s) => Ok(Some(ContentSource::StaticString(s.to_str()?.to_string()))),
            Value::Function(_) => Ok(Some(ContentSource::Dynamic)),
            other => Err(anyhow!(
                "unsupported value type for node: {}",
                other.type_name()
            )),
        }
    }

    fn parse_events(&self, table: &Table) -> Result<Vec<EventBinding>> {
        let mut events = Vec::new();
        for (field, kind) in [
            ("on_click", EventKind::Click),
            ("on_scroll", EventKind::Scroll),
            ("on_hover", EventKind::Hover),
        ] {
            if matches!(table.get::<Value>(field)?, Value::Function(_)) {
                events.push(EventBinding {
                    kind: kind.clone(),
                    dynamic: true,
                });
            }
        }
        Ok(events)
    }

    fn parse_state(&self, table: &Table) -> Result<Vec<StateField>> {
        let mut fields = Vec::new();
        for pair in table.clone().pairs::<Value, Value>() {
            let (key, value) = pair?;
            let name = match key {
                Value::String(s) => s.to_str()?.to_string(),
                _ => continue,
            };
            if RESERVED_KEYS.contains(&name.as_str()) {
                continue;
            }
            if let Some(state_value) = self.convert_state_value(value)? {
                fields.push(StateField {
                    name,
                    value: state_value,
                });
            }
        }
        Ok(fields)
    }

    fn convert_state_value(&self, value: Value) -> Result<Option<StateValue>> {
        Ok(match value {
            Value::String(s) => Some(StateValue::String(s.to_str()?.to_string())),
            Value::Integer(i) => Some(StateValue::Integer(i)),
            Value::Number(n) => Some(StateValue::Number(n as f32)),
            Value::Boolean(b) => Some(StateValue::Bool(b)),
            _ => None,
        })
    }
}
