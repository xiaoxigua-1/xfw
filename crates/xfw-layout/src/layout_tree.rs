use std::collections::HashMap;
use std::str::FromStr;
use taffy::style::Style as TaffyStyle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Anchor {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

impl Anchor {
    pub fn parse(s: &str) -> Self {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let mut anchor = Anchor::default();
        for part in parts {
            match part {
                "top" => anchor.top = true,
                "bottom" => anchor.bottom = true,
                "left" => anchor.left = true,
                "right" => anchor.right = true,
                _ => {}
            }
        }
        anchor
    }

    pub fn is_horizontal(&self) -> bool {
        self.left || self.right
    }

    pub fn is_vertical(&self) -> bool {
        self.top || self.bottom
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::FromRepr)]
#[repr(u8)]
pub enum Layer {
    #[default]
    Overlay,
    Background,
    Top,
}

impl Layer {
    pub fn parse(s: &str) -> Self {
        match s {
            "background" => Layer::Background,
            "top" => Layer::Top,
            _ => Layer::Overlay,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutElementKind {
    Container,
    Text,
    Image,
}

#[derive(Debug, Clone)]
pub enum LayoutNode {
    Container {
        id: Option<String>,
        style: TaffyStyle,
        children: Vec<LayoutNode>,
    },
    Text {
        id: Option<String>,
        content: String,
        style: TaffyStyle,
    },
    Image {
        id: Option<String>,
        path: String,
        style: TaffyStyle,
    },
}

impl LayoutNode {
    pub fn container(id: Option<String>, style: TaffyStyle, children: Vec<LayoutNode>) -> Self {
        Self::Container {
            id,
            style,
            children,
        }
    }

    pub fn text(id: Option<String>, content: String, style: TaffyStyle) -> Self {
        Self::Text { id, content, style }
    }

    pub fn image(id: Option<String>, path: String, style: TaffyStyle) -> Self {
        Self::Image { id, path, style }
    }

    pub fn id(&self) -> Option<&str> {
        match self {
            Self::Container { id, .. } => id.as_deref(),
            Self::Text { id, .. } => id.as_deref(),
            Self::Image { id, .. } => id.as_deref(),
        }
    }

    pub fn style(&self) -> &TaffyStyle {
        match self {
            Self::Container { style, .. } => style,
            Self::Text { style, .. } => style,
            Self::Image { style, .. } => style,
        }
    }

    pub fn style_mut(&mut self) -> &mut TaffyStyle {
        match self {
            Self::Container { style, .. } => style,
            Self::Text { style, .. } => style,
            Self::Image { style, .. } => style,
        }
    }

    pub fn children(&self) -> Option<&[LayoutNode]> {
        match self {
            Self::Container { children, .. } => Some(children),
            Self::Text { .. } => None,
            Self::Image { .. } => None,
        }
    }

    pub fn children_mut(&mut self) -> Option<&mut Vec<LayoutNode>> {
        match self {
            Self::Container { children, .. } => Some(children),
            Self::Text { .. } => None,
            Self::Image { .. } => None,
        }
    }

    pub fn is_container(&self) -> bool {
        matches!(self, Self::Container { .. })
    }

    pub fn element_kind(&self) -> LayoutElementKind {
        match self {
            Self::Container { .. } => LayoutElementKind::Container,
            Self::Text { .. } => LayoutElementKind::Text,
            Self::Image { .. } => LayoutElementKind::Image,
        }
    }

    pub fn find_by_id(&self, id: &str) -> Option<&LayoutNode> {
        if self.id() == Some(id) {
            return Some(self);
        }
        if let Some(children) = self.children() {
            for child in children {
                if let Some(found) = child.find_by_id(id) {
                    return Some(found);
                }
            }
        }
        None
    }

    pub fn find_by_id_mut(&mut self, id: &str) -> Option<&mut LayoutNode> {
        if self.id() == Some(id) {
            return Some(self);
        }
        if let Some(children) = self.children_mut() {
            for child in children.iter_mut() {
                if let Some(found) = child.find_by_id_mut(id) {
                    return Some(found);
                }
            }
        }
        None
    }

    pub fn update_by_path(&mut self, path: &[String], new_node: LayoutNode) -> bool {
        if path.is_empty() {
            *self = new_node;
            return true;
        }

        if let Some(children) = self.children_mut() {
            if let Some(first) = path.first() {
                for child in children.iter_mut() {
                    if child.id() == Some(first.as_str()) {
                        return child.update_by_path(&path[1..], new_node);
                    }
                }
            }
        }
        false
    }
}

fn build_map_impl(node: &LayoutNode, map: &mut HashMap<String, usize>, depth: usize) {
    if let Some(id) = node.id() {
        map.insert(id.to_string(), depth);
    }
    if let Some(children) = node.children() {
        for child in children.iter() {
            build_map_impl(child, map, depth + 1);
        }
    }
}

fn find_by_prefix_impl<'a>(node: &'a LayoutNode, prefix: &str, results: &mut Vec<&'a LayoutNode>) {
    if let Some(id) = node.id() {
        if id.starts_with(prefix) {
            results.push(node);
        }
    }
    if let Some(children) = node.children() {
        for child in children.iter() {
            find_by_prefix_impl(child, prefix, results);
        }
    }
}

fn count_nodes_impl(node: &LayoutNode) -> usize {
    let mut count = 1;
    if let Some(children) = node.children() {
        for child in children.iter() {
            count += count_nodes_impl(child);
        }
    }
    count
}

fn find_by_path_impl<'a>(node: &'a LayoutNode, path: &str) -> Option<&'a LayoutNode> {
    if let Some(id) = node.id() {
        if id == path {
            return Some(node);
        }
    }
    if let Some(children) = node.children() {
        for child in children.iter() {
            if let Some(found) = find_by_path_impl(child, path) {
                return Some(found);
            }
        }
    }
    None
}

pub struct LayoutTree {
    root: LayoutNode,
    node_map: HashMap<String, usize>,
}

impl LayoutTree {
    pub fn new(root: LayoutNode) -> Self {
        let mut tree = Self {
            root,
            node_map: HashMap::new(),
        };
        tree.build_map();
        tree
    }

    fn build_map(&mut self) {
        self.node_map.clear();
        build_map_impl(&self.root, &mut self.node_map, 0);
    }

    pub fn find_by_path(&self, path: &str) -> Option<&LayoutNode> {
        find_by_path_impl(&self.root, path)
    }

    pub fn find_by_prefix(&self, prefix: &str) -> Vec<&LayoutNode> {
        let mut results = Vec::new();
        find_by_prefix_impl(&self.root, prefix, &mut results);
        results
    }

    pub fn root(&self) -> &LayoutNode {
        &self.root
    }

    pub fn node_count(&self) -> usize {
        count_nodes_impl(&self.root)
    }

    pub fn root_mut(&mut self) -> &mut LayoutNode {
        &mut self.root
    }

    pub fn find_by_id(&self, id: &str) -> Option<&LayoutNode> {
        self.root.find_by_id(id)
    }

    pub fn find_by_id_mut(&mut self, id: &str) -> Option<&mut LayoutNode> {
        self.root.find_by_id_mut(id)
    }

    pub fn find_many(&self, ids: &[String]) -> Vec<&LayoutNode> {
        ids.iter().filter_map(|id| self.find_by_id(id)).collect()
    }

    pub fn get_affected_by_state(&self, state_path: &str) -> Vec<&LayoutNode> {
        self.find_by_prefix(state_path)
    }

    pub fn get_affected_ids(&self, state_path: &str) -> Vec<String> {
        self.get_affected_by_state(state_path)
            .iter()
            .filter_map(|n| n.id().map(String::from))
            .collect()
    }

    pub fn get_depth(&self, id: &str) -> Option<usize> {
        self.node_map.get(id).copied()
    }

    pub fn update(&mut self, path: &[String], new_node: LayoutNode) -> bool {
        self.root.update_by_path(path, new_node)
    }
}
