use std::fmt;

#[derive(Debug, Clone)]
pub struct UiNode {
    pub id: Option<String>,
    pub kind: NodeKind,
    pub props: NodeProps,
    pub children: Vec<UiNode>,
}

impl UiNode {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            id: None,
            kind,
            props: NodeProps::default(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, strum::FromRepr)]
#[repr(u8)]
pub enum NodeKind {
    View,
    Text,
    Image,
    Window,
    Button,
    Row,
    Column,
    Custom(String),
}

impl NodeKind {
    pub fn parse(value: &str) -> Self {
        match value {
            "view" => Self::View,
            "text" => Self::Text,
            "image" => Self::Image,
            "window" => Self::Window,
            "button" => Self::Button,
            "row" => Self::Row,
            "column" => Self::Column,
            other => Self::Custom(other.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeProps {
    pub style: StyleSource,
    pub value: Option<ContentSource>,
    pub events: Vec<EventBinding>,
    pub state: Vec<StateField>,
}

impl Default for NodeProps {
    fn default() -> Self {
        Self {
            style: StyleSource::None,
            value: None,
            events: Vec::new(),
            state: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum StyleSource {
    #[default]
    None,
    Static(StyleMap),
    Dynamic,
}

pub type StyleMap = Vec<StyleAttribute>;

#[derive(Debug, Clone)]
pub struct StyleAttribute {
    pub name: String,
    pub value: StyleValue,
}

#[derive(Debug, Clone)]
pub enum StyleValue {
    String(String),
    Number(f32),
    Integer(i64),
    Bool(bool),
    Array(Vec<StyleValue>),
}

impl fmt::Display for StyleValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StyleValue::String(s) => write!(f, "{}", s),
            StyleValue::Number(n) => write!(f, "{}", n),
            StyleValue::Integer(n) => write!(f, "{}", n),
            StyleValue::Bool(b) => write!(f, "{}", b),
            StyleValue::Array(values) => {
                write!(f, "[")?;
                for (idx, value) in values.iter().enumerate() {
                    if idx > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ContentSource {
    StaticString(String),
    Dynamic,
}

#[derive(Debug, Clone)]
pub struct EventBinding {
    pub kind: EventKind,
    pub dynamic: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    Click,
    Scroll,
    Hover,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct StateField {
    pub name: String,
    pub value: StateValue,
}

#[derive(Debug, Clone)]
pub enum StateValue {
    String(String),
    Number(f32),
    Integer(i64),
    Bool(bool),
}

impl NodeProps {
    pub fn get_style(&self, name: &str) -> Option<&StyleValue> {
        match &self.style {
            StyleSource::Static(entries) => entries
                .iter()
                .find(|attr| attr.name == name)
                .map(|attr| &attr.value),
            _ => None,
        }
    }
}
