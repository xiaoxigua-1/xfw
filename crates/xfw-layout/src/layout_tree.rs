use std::collections::HashMap;
use std::str::FromStr;
use taffy::style::Style as TaffyStyle;

/// Anchor flags for layout nodes (legacy layout tree).
///
/// # Examples
/// ```rust
/// use xfw_layout::layout_tree::Anchor;
/// let anchor = Anchor::parse("top left");
/// assert!(anchor.top && anchor.left);
/// ```
///
/// # Errors
/// None.
///
/// # Panics
/// None.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Anchor {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

impl Anchor {
    /// Parses a space-separated anchor string.
    ///
    /// # Examples
    /// ```rust
    /// use xfw_layout::layout_tree::Anchor;
    /// let anchor = Anchor::parse("bottom right");
    /// assert!(anchor.bottom && anchor.right);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
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

    /// Returns true if any horizontal anchor is set.
    ///
    /// # Examples
    /// ```rust
    /// use xfw_layout::layout_tree::Anchor;
    /// let anchor = Anchor::parse("left");
    /// assert!(anchor.is_horizontal());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn is_horizontal(&self) -> bool {
        self.left || self.right
    }

    /// Returns true if any vertical anchor is set.
    ///
    /// # Examples
    /// ```rust
    /// use xfw_layout::layout_tree::Anchor;
    /// let anchor = Anchor::parse("top");
    /// assert!(anchor.is_vertical());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn is_vertical(&self) -> bool {
        self.top || self.bottom
    }
}

/// Legacy layer enum used by the layout tree.
///
/// # Examples
/// ```rust
/// use xfw_layout::layout_tree::Layer;
/// assert_eq!(Layer::parse("top"), Layer::Top);
/// ```
///
/// # Errors
/// None.
///
/// # Panics
/// None.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::FromRepr)]
#[repr(u8)]
pub enum Layer {
    #[default]
    Overlay,
    Background,
    Top,
}

impl Layer {
    /// Parses a string into a layer.
    ///
    /// # Examples
    /// ```rust
    /// use xfw_layout::layout_tree::Layer;
    /// assert_eq!(Layer::parse("background"), Layer::Background);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn parse(s: &str) -> Self {
        match s {
            "background" => Layer::Background,
            "top" => Layer::Top,
            _ => Layer::Overlay,
        }
    }
}

/// Legacy layout element kinds.
///
/// # Examples
/// ```rust
/// use xfw_layout::layout_tree::LayoutElementKind;
/// let _kind = LayoutElementKind::Container;
/// ```
///
/// # Errors
/// None.
///
/// # Panics
/// None.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutElementKind {
    Container,
    Text,
    Image,
}

/// Legacy layout node used for intermediate layout experiments.
///
/// # Examples
/// ```rust
/// use taffy::Style as TaffyStyle;
/// use xfw_layout::layout_tree::LayoutNode;
/// let node = LayoutNode::container(None, TaffyStyle::default(), vec![]);
/// ```
///
/// # Errors
/// None.
///
/// # Panics
/// None.
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
    /// Creates a container layout node.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::LayoutNode;
    /// let node = LayoutNode::container(Some("root".to_string()), TaffyStyle::default(), vec![]);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn container(id: Option<String>, style: TaffyStyle, children: Vec<LayoutNode>) -> Self {
        Self::Container {
            id,
            style,
            children,
        }
    }

    /// Creates a text layout node.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::LayoutNode;
    /// let node = LayoutNode::text(None, "Hello".to_string(), TaffyStyle::default());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn text(id: Option<String>, content: String, style: TaffyStyle) -> Self {
        Self::Text { id, content, style }
    }

    /// Creates an image layout node.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::LayoutNode;
    /// let node = LayoutNode::image(None, "/tmp/image.png".to_string(), TaffyStyle::default());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn image(id: Option<String>, path: String, style: TaffyStyle) -> Self {
        Self::Image { id, path, style }
    }

    /// Returns the node id, if present.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::LayoutNode;
    /// let node = LayoutNode::container(Some("root".to_string()), TaffyStyle::default(), vec![]);
    /// assert_eq!(node.id(), Some("root"));
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn id(&self) -> Option<&str> {
        match self {
            Self::Container { id, .. } => id.as_deref(),
            Self::Text { id, .. } => id.as_deref(),
            Self::Image { id, .. } => id.as_deref(),
        }
    }

    /// Returns the node's Taffy style.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::LayoutNode;
    /// let node = LayoutNode::container(None, TaffyStyle::default(), vec![]);
    /// let _style = node.style();
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn style(&self) -> &TaffyStyle {
        match self {
            Self::Container { style, .. } => style,
            Self::Text { style, .. } => style,
            Self::Image { style, .. } => style,
        }
    }

    /// Returns the mutable Taffy style.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::LayoutNode;
    /// let mut node = LayoutNode::container(None, TaffyStyle::default(), vec![]);
    /// let _style = node.style_mut();
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn style_mut(&mut self) -> &mut TaffyStyle {
        match self {
            Self::Container { style, .. } => style,
            Self::Text { style, .. } => style,
            Self::Image { style, .. } => style,
        }
    }

    /// Returns children for container nodes.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::LayoutNode;
    /// let node = LayoutNode::container(None, TaffyStyle::default(), vec![]);
    /// assert!(node.children().unwrap().is_empty());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn children(&self) -> Option<&[LayoutNode]> {
        match self {
            Self::Container { children, .. } => Some(children),
            Self::Text { .. } => None,
            Self::Image { .. } => None,
        }
    }

    /// Returns mutable children for container nodes.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::LayoutNode;
    /// let mut node = LayoutNode::container(None, TaffyStyle::default(), vec![]);
    /// assert!(node.children_mut().unwrap().is_empty());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn children_mut(&mut self) -> Option<&mut Vec<LayoutNode>> {
        match self {
            Self::Container { children, .. } => Some(children),
            Self::Text { .. } => None,
            Self::Image { .. } => None,
        }
    }

    /// Returns true if this node is a container.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::LayoutNode;
    /// let node = LayoutNode::container(None, TaffyStyle::default(), vec![]);
    /// assert!(node.is_container());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn is_container(&self) -> bool {
        matches!(self, Self::Container { .. })
    }

    /// Returns the element kind.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::{LayoutElementKind, LayoutNode};
    /// let node = LayoutNode::container(None, TaffyStyle::default(), vec![]);
    /// assert_eq!(node.element_kind(), LayoutElementKind::Container);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn element_kind(&self) -> LayoutElementKind {
        match self {
            Self::Container { .. } => LayoutElementKind::Container,
            Self::Text { .. } => LayoutElementKind::Text,
            Self::Image { .. } => LayoutElementKind::Image,
        }
    }

    /// Finds a node by id in the subtree.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::LayoutNode;
    /// let node = LayoutNode::container(Some("root".to_string()), TaffyStyle::default(), vec![]);
    /// assert!(node.find_by_id("root").is_some());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
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

    /// Finds a node by id in the subtree (mutable).
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::LayoutNode;
    /// let mut node = LayoutNode::container(Some("root".to_string()), TaffyStyle::default(), vec![]);
    /// assert!(node.find_by_id_mut("root").is_some());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
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

    /// Replaces a node at the given id path.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::LayoutNode;
    /// let mut node = LayoutNode::container(Some("root".to_string()), TaffyStyle::default(), vec![]);
    /// let updated = LayoutNode::container(Some("root".to_string()), TaffyStyle::default(), vec![]);
    /// assert!(node.update_by_path(&["root".to_string()], updated));
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
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

/// Legacy layout tree with helper indexes.
///
/// # Examples
/// ```rust
/// use taffy::Style as TaffyStyle;
/// use xfw_layout::layout_tree::{LayoutNode, LayoutTree};
/// let root = LayoutNode::container(None, TaffyStyle::default(), vec![]);
/// let tree = LayoutTree::new(root);
/// assert_eq!(tree.node_count(), 1);
/// ```
///
/// # Errors
/// None.
///
/// # Panics
/// None.
pub struct LayoutTree {
    root: LayoutNode,
    node_map: HashMap<String, usize>,
}

impl LayoutTree {
    /// Creates a new layout tree.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::{LayoutNode, LayoutTree};
    /// let root = LayoutNode::container(None, TaffyStyle::default(), vec![]);
    /// let tree = LayoutTree::new(root);
    /// assert_eq!(tree.node_count(), 1);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
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

    /// Finds a node by exact path.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::{LayoutNode, LayoutTree};
    /// let root = LayoutNode::container(Some("root".to_string()), TaffyStyle::default(), vec![]);
    /// let tree = LayoutTree::new(root);
    /// assert!(tree.find_by_path("root").is_some());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn find_by_path(&self, path: &str) -> Option<&LayoutNode> {
        find_by_path_impl(&self.root, path)
    }

    /// Finds nodes by prefix.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::{LayoutNode, LayoutTree};
    /// let root = LayoutNode::container(Some("root".to_string()), TaffyStyle::default(), vec![]);
    /// let tree = LayoutTree::new(root);
    /// assert_eq!(tree.find_by_prefix("roo").len(), 1);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn find_by_prefix(&self, prefix: &str) -> Vec<&LayoutNode> {
        let mut results = Vec::new();
        find_by_prefix_impl(&self.root, prefix, &mut results);
        results
    }

    /// Returns the root node.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::{LayoutNode, LayoutTree};
    /// let root = LayoutNode::container(None, TaffyStyle::default(), vec![]);
    /// let tree = LayoutTree::new(root);
    /// let _root = tree.root();
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn root(&self) -> &LayoutNode {
        &self.root
    }

    /// Returns the total node count.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::{LayoutNode, LayoutTree};
    /// let root = LayoutNode::container(None, TaffyStyle::default(), vec![]);
    /// let tree = LayoutTree::new(root);
    /// assert_eq!(tree.node_count(), 1);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn node_count(&self) -> usize {
        count_nodes_impl(&self.root)
    }

    /// Returns mutable root node.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::{LayoutNode, LayoutTree};
    /// let root = LayoutNode::container(None, TaffyStyle::default(), vec![]);
    /// let mut tree = LayoutTree::new(root);
    /// let _root = tree.root_mut();
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn root_mut(&mut self) -> &mut LayoutNode {
        &mut self.root
    }

    /// Finds a node by id.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::{LayoutNode, LayoutTree};
    /// let root = LayoutNode::container(Some("root".to_string()), TaffyStyle::default(), vec![]);
    /// let tree = LayoutTree::new(root);
    /// assert!(tree.find_by_id("root").is_some());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn find_by_id(&self, id: &str) -> Option<&LayoutNode> {
        self.root.find_by_id(id)
    }

    /// Finds a node by id (mutable).
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::{LayoutNode, LayoutTree};
    /// let root = LayoutNode::container(Some("root".to_string()), TaffyStyle::default(), vec![]);
    /// let mut tree = LayoutTree::new(root);
    /// assert!(tree.find_by_id_mut("root").is_some());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn find_by_id_mut(&mut self, id: &str) -> Option<&mut LayoutNode> {
        self.root.find_by_id_mut(id)
    }

    /// Finds multiple nodes by id.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::{LayoutNode, LayoutTree};
    /// let root = LayoutNode::container(Some("root".to_string()), TaffyStyle::default(), vec![]);
    /// let tree = LayoutTree::new(root);
    /// let nodes = tree.find_many(&["root".to_string()]);
    /// assert_eq!(nodes.len(), 1);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn find_many(&self, ids: &[String]) -> Vec<&LayoutNode> {
        ids.iter().filter_map(|id| self.find_by_id(id)).collect()
    }

    /// Finds nodes affected by a state path.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::{LayoutNode, LayoutTree};
    /// let root = LayoutNode::container(Some("store.battery".to_string()), TaffyStyle::default(), vec![]);
    /// let tree = LayoutTree::new(root);
    /// assert_eq!(tree.get_affected_by_state("store").len(), 1);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn get_affected_by_state(&self, state_path: &str) -> Vec<&LayoutNode> {
        self.find_by_prefix(state_path)
    }

    /// Returns affected node ids for a state path.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::{LayoutNode, LayoutTree};
    /// let root = LayoutNode::container(Some("store.battery".to_string()), TaffyStyle::default(), vec![]);
    /// let tree = LayoutTree::new(root);
    /// assert_eq!(tree.get_affected_ids("store").len(), 1);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn get_affected_ids(&self, state_path: &str) -> Vec<String> {
        self.get_affected_by_state(state_path)
            .iter()
            .filter_map(|n| n.id().map(String::from))
            .collect()
    }

    /// Returns the depth for a node id, if tracked.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::{LayoutNode, LayoutTree};
    /// let root = LayoutNode::container(Some("root".to_string()), TaffyStyle::default(), vec![]);
    /// let tree = LayoutTree::new(root);
    /// assert_eq!(tree.get_depth("root"), Some(0));
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn get_depth(&self, id: &str) -> Option<usize> {
        self.node_map.get(id).copied()
    }

    /// Updates a node at the given path.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::layout_tree::{LayoutNode, LayoutTree};
    /// let root = LayoutNode::container(Some("root".to_string()), TaffyStyle::default(), vec![]);
    /// let mut tree = LayoutTree::new(root);
    /// let updated = LayoutNode::container(Some("root".to_string()), TaffyStyle::default(), vec![]);
    /// assert!(tree.update(&["root".to_string()], updated));
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn update(&mut self, path: &[String], new_node: LayoutNode) -> bool {
        self.root.update_by_path(path, new_node)
    }
}
