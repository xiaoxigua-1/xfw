use std::collections::HashMap;
use taffy::prelude::*;

/// RGBA color in linear 0..=1.0 space.
///
/// # Examples
/// ```rust
/// use xfw_layout::Color;
/// let c = Color::from_hex("#ff0000").unwrap();
/// assert_eq!(c.r, 1.0);
/// ```
///
/// # Errors
/// None.
///
/// # Panics
/// None.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Parses a hex color in `#RRGGBB` or `#RRGGBBAA` format.
    ///
    /// # Examples
    /// ```rust
    /// use xfw_layout::Color;
    /// let c = Color::from_hex("#00ff00").unwrap();
    /// assert_eq!(c.g, 1.0);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;
            Some(Self { r, g, b, a: 1.0 })
        } else if hex.len() == 8 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()? as f32 / 255.0;
            Some(Self { r, g, b, a })
        } else {
            None
        }
    }
}

/// Rendering-only style properties for a node.
///
/// # Examples
/// ```rust
/// use xfw_layout::{RenderStyle, Color};
/// let mut style = RenderStyle::default();
/// style.background_color = Color::from_hex("#ff0000");
/// ```
///
/// # Errors
/// None.
///
/// # Panics
/// None.
#[derive(Debug, Clone, Default)]
pub struct RenderStyle {
    pub color: Option<Color>,
    pub font_size: Option<f32>,
    pub font_family: Option<String>,
    pub background_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: Option<f32>,
    pub border_radius: Option<f32>,
    pub opacity: Option<f32>,
    pub text_align: Option<TextAlign>,
    pub image_fit: Option<ImageFit>,
    pub overflow: OverflowBehavior,
}

/// Overflow behavior for rendering.
///
/// # Examples
/// ```rust
/// use xfw_layout::OverflowBehavior;
/// assert_eq!(OverflowBehavior::parse("hidden"), OverflowBehavior::Hidden);
/// ```
///
/// # Errors
/// None.
///
/// # Panics
/// None.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::FromRepr)]
#[repr(u8)]
pub enum OverflowBehavior {
    #[default]
    Visible,
    Hidden,
}

impl OverflowBehavior {
    /// Parses a string into an overflow behavior.
    ///
    /// # Examples
    /// ```rust
    /// use xfw_layout::OverflowBehavior;
    /// assert_eq!(OverflowBehavior::parse("clip"), OverflowBehavior::Hidden);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "hidden" | "clip" => OverflowBehavior::Hidden,
            _ => OverflowBehavior::Visible,
        }
    }
}

/// Horizontal text alignment.
///
/// # Examples
/// ```rust
/// use xfw_layout::TextAlign;
/// assert_eq!(TextAlign::parse("center"), TextAlign::Center);
/// ```
///
/// # Errors
/// None.
///
/// # Panics
/// None.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::FromRepr)]
#[repr(u8)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

impl TextAlign {
    /// Parses a string into a text alignment.
    ///
    /// # Examples
    /// ```rust
    /// use xfw_layout::TextAlign;
    /// assert_eq!(TextAlign::parse("right"), TextAlign::Right);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "center" => TextAlign::Center,
            "right" => TextAlign::Right,
            "justify" => TextAlign::Justify,
            _ => TextAlign::Left,
        }
    }
}

/// How images fit within their bounds.
///
/// # Examples
/// ```rust
/// use xfw_layout::ImageFit;
/// assert_eq!(ImageFit::parse("contain"), ImageFit::Contain);
/// ```
///
/// # Errors
/// None.
///
/// # Panics
/// None.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::FromRepr)]
#[repr(u8)]
pub enum ImageFit {
    #[default]
    Fill,
    Contain,
    Cover,
    FitWidth,
    FitHeight,
    None,
}

impl ImageFit {
    /// Parses a string into an image fit mode.
    ///
    /// # Examples
    /// ```rust
    /// use xfw_layout::ImageFit;
    /// assert_eq!(ImageFit::parse("fit-width"), ImageFit::FitWidth);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().replace('_', "-").as_str() {
            "contain" => ImageFit::Contain,
            "cover" => ImageFit::Cover,
            "fit-width" => ImageFit::FitWidth,
            "fit-height" => ImageFit::FitHeight,
            "none" => ImageFit::None,
            _ => ImageFit::Fill,
        }
    }
}

/// Absolute rectangle used for layout and rendering.
///
/// # Examples
/// ```rust
/// use xfw_layout::Rect;
/// let rect = Rect { x: 0.0, y: 0.0, width: 10.0, height: 20.0 };
/// assert_eq!(rect.width, 10.0);
/// ```
///
/// # Errors
/// None.
///
/// # Panics
/// None.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
}

impl From<&taffy::Layout> for Rect {
    fn from(layout: &taffy::Layout) -> Self {
        Self {
            x: layout.location.x,
            y: layout.location.y,
            width: layout.size.width,
            height: layout.size.height,
        }
    }
}

/// Anchor flags for Wayland layer surfaces.
///
/// # Examples
/// ```rust
/// use xfw_layout::Anchor;
/// let anchor = Anchor::parse("top right");
/// assert!(anchor.top && anchor.right);
/// ```
///
/// # Errors
/// None.
///
/// # Panics
/// None.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
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
    /// use xfw_layout::Anchor;
    /// let anchor = Anchor::parse("bottom left");
    /// assert!(anchor.bottom && anchor.left);
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
}

/// Z-order layer placement for windows.
///
/// # Examples
/// ```rust
/// use xfw_layout::Layer;
/// assert_eq!(Layer::parse("top"), Layer::Top);
/// ```
///
/// # Errors
/// None.
///
/// # Panics
/// None.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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
    /// use xfw_layout::Layer;
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

/// Node kind in the render tree.
///
/// # Examples
/// ```rust
/// use xfw_layout::Kind;
/// let kind = Kind::Text;
/// ```
///
/// # Errors
/// None.
///
/// # Panics
/// None.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Container,
    Text,
    Image,
}

/// Render tree node with layout and render styles.
///
/// # Examples
/// ```rust
/// use taffy::Style as TaffyStyle;
/// use xfw_layout::{RenderObject, RenderStyle};
/// let node = RenderObject::container(None, TaffyStyle::default(), RenderStyle::default(), vec![]);
/// ```
///
/// # Errors
/// None.
///
/// # Panics
/// None.
#[derive(Debug, Clone)]
pub enum RenderObject {
    Container {
        id: Option<String>,
        layout_style: Style,
        render_style: RenderStyle,
        rect: Rect,
        children: Vec<RenderObject>,
    },
    Text {
        id: Option<String>,
        layout_style: Style,
        render_style: RenderStyle,
        rect: Rect,
        content: String,
    },
    Image {
        id: Option<String>,
        layout_style: Style,
        render_style: RenderStyle,
        rect: Rect,
        path: String,
    },
}

impl RenderObject {
    /// Creates a container node.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderStyle};
    /// let node = RenderObject::container(Some("root".to_string()), TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn container(
        id: Option<String>,
        layout_style: Style,
        render_style: RenderStyle,
        children: Vec<RenderObject>,
    ) -> Self {
        Self::Container {
            id,
            layout_style,
            render_style,
            rect: Rect::default(),
            children,
        }
    }

    /// Creates a text node.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderStyle};
    /// let node = RenderObject::text(None, TaffyStyle::default(), RenderStyle::default(), "Hello".to_string());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn text(
        id: Option<String>,
        layout_style: Style,
        render_style: RenderStyle,
        content: String,
    ) -> Self {
        Self::Text {
            id,
            layout_style,
            render_style,
            rect: Rect::default(),
            content,
        }
    }

    /// Creates an image node.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderStyle};
    /// let node = RenderObject::image(None, TaffyStyle::default(), RenderStyle::default(), "/tmp/image.png".to_string());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn image(
        id: Option<String>,
        layout_style: Style,
        render_style: RenderStyle,
        path: String,
    ) -> Self {
        Self::Image {
            id,
            layout_style,
            render_style,
            rect: Rect::default(),
            path,
        }
    }

    /// Returns the node id, if present.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderStyle};
    /// let node = RenderObject::container(Some("root".to_string()), TaffyStyle::default(), RenderStyle::default(), vec![]);
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

    /// Returns the layout style for this node.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderStyle};
    /// let node = RenderObject::container(None, TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// let _style = node.layout_style();
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn layout_style(&self) -> &Style {
        match self {
            Self::Container { layout_style, .. } => layout_style,
            Self::Text { layout_style, .. } => layout_style,
            Self::Image { layout_style, .. } => layout_style,
        }
    }

    /// Returns the mutable layout style for this node.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderStyle};
    /// let mut node = RenderObject::container(None, TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// let _style = node.layout_style_mut();
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn layout_style_mut(&mut self) -> &mut Style {
        match self {
            Self::Container { layout_style, .. } => layout_style,
            Self::Text { layout_style, .. } => layout_style,
            Self::Image { layout_style, .. } => layout_style,
        }
    }

    /// Returns the render style for this node.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderStyle};
    /// let node = RenderObject::container(None, TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// let _style = node.render_style();
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn render_style(&self) -> &RenderStyle {
        match self {
            Self::Container { render_style, .. } => render_style,
            Self::Text { render_style, .. } => render_style,
            Self::Image { render_style, .. } => render_style,
        }
    }

    /// Returns the mutable render style for this node.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderStyle};
    /// let mut node = RenderObject::container(None, TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// let _style = node.render_style_mut();
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn render_style_mut(&mut self) -> &mut RenderStyle {
        match self {
            Self::Container { render_style, .. } => render_style,
            Self::Text { render_style, .. } => render_style,
            Self::Image { render_style, .. } => render_style,
        }
    }

    /// Returns the computed layout rectangle.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderStyle};
    /// let node = RenderObject::container(None, TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// let _rect = node.rect();
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn rect(&self) -> &Rect {
        match self {
            Self::Container { rect, .. } => rect,
            Self::Text { rect, .. } => rect,
            Self::Image { rect, .. } => rect,
        }
    }

    /// Returns the mutable layout rectangle.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderStyle};
    /// let mut node = RenderObject::container(None, TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// let _rect = node.rect_mut();
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn rect_mut(&mut self) -> &mut Rect {
        match self {
            Self::Container { rect, .. } => rect,
            Self::Text { rect, .. } => rect,
            Self::Image { rect, .. } => rect,
        }
    }

    /// Returns children for container nodes.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderStyle};
    /// let node = RenderObject::container(None, TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// assert!(node.children().unwrap().is_empty());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn children(&self) -> Option<&[RenderObject]> {
        match self {
            Self::Container { children, .. } => Some(children),
            _ => None,
        }
    }

    /// Returns mutable children for container nodes.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderStyle};
    /// let mut node = RenderObject::container(None, TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// assert!(node.children_mut().unwrap().is_empty());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn children_mut(&mut self) -> Option<&mut Vec<RenderObject>> {
        match self {
            Self::Container { children, .. } => Some(children),
            _ => None,
        }
    }

    /// Returns the node kind.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderStyle, Kind};
    /// let node = RenderObject::container(None, TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// assert_eq!(node.kind(), Kind::Container);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn kind(&self) -> Kind {
        match self {
            Self::Container { .. } => Kind::Container,
            Self::Text { .. } => Kind::Text,
            Self::Image { .. } => Kind::Image,
        }
    }

    /// Finds a node by id in the subtree.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderStyle};
    /// let node = RenderObject::container(Some("root".to_string()), TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// assert!(node.find_by_id("root").is_some());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn find_by_id(&self, id: &str) -> Option<&RenderObject> {
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
    /// use xfw_layout::{RenderObject, RenderStyle};
    /// let mut node = RenderObject::container(Some("root".to_string()), TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// assert!(node.find_by_id_mut("root").is_some());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn find_by_id_mut(&mut self, id: &str) -> Option<&mut RenderObject> {
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
}

fn find_by_prefix_impl<'a>(
    node: &'a RenderObject,
    prefix: &str,
    results: &mut Vec<&'a RenderObject>,
) {
    if let Some(id) = node.id() && id.starts_with(prefix) {
        results.push(node);
    }
    if let Some(children) = node.children() {
        for child in children.iter() {
            find_by_prefix_impl(child, prefix, results);
        }
    }
}

fn count_nodes_impl(node: &RenderObject) -> usize {
    let mut count = 1;
    if let Some(children) = node.children() {
        for child in children.iter() {
            count += count_nodes_impl(child);
        }
    }
    count
}

/// Rooted render tree with indexed lookup helpers.
///
/// # Examples
/// ```rust
/// use taffy::Style as TaffyStyle;
/// use xfw_layout::{RenderObject, RenderObjectTree, RenderStyle};
/// let root = RenderObject::container(None, TaffyStyle::default(), RenderStyle::default(), vec![]);
/// let tree = RenderObjectTree::new(root);
/// assert_eq!(tree.node_count(), 1);
/// ```
///
/// # Errors
/// None.
///
/// # Panics
/// None.
pub struct RenderObjectTree {
    root: RenderObject,
    node_map: HashMap<String, usize>,
}

impl RenderObjectTree {
    /// Creates a new render tree from a root node.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderObjectTree, RenderStyle};
    /// let root = RenderObject::container(None, TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// let tree = RenderObjectTree::new(root);
    /// assert_eq!(tree.node_count(), 1);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn new(root: RenderObject) -> Self {
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

    /// Returns the root node.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderObjectTree, RenderStyle};
    /// let root = RenderObject::container(None, TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// let tree = RenderObjectTree::new(root);
    /// let _root = tree.root();
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn root(&self) -> &RenderObject {
        &self.root
    }

    /// Returns the mutable root node.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderObjectTree, RenderStyle};
    /// let root = RenderObject::container(None, TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// let mut tree = RenderObjectTree::new(root);
    /// let _root = tree.root_mut();
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn root_mut(&mut self) -> &mut RenderObject {
        &mut self.root
    }

    /// Returns the total number of nodes in the tree.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderObjectTree, RenderStyle};
    /// let root = RenderObject::container(None, TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// let tree = RenderObjectTree::new(root);
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

    /// Finds a node by id.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderObjectTree, RenderStyle};
    /// let root = RenderObject::container(Some("root".to_string()), TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// let tree = RenderObjectTree::new(root);
    /// assert!(tree.find_by_id("root").is_some());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn find_by_id(&self, id: &str) -> Option<&RenderObject> {
        self.root.find_by_id(id)
    }

    /// Finds a node by id (mutable).
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderObjectTree, RenderStyle};
    /// let root = RenderObject::container(Some("root".to_string()), TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// let mut tree = RenderObjectTree::new(root);
    /// assert!(tree.find_by_id_mut("root").is_some());
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn find_by_id_mut(&mut self, id: &str) -> Option<&mut RenderObject> {
        self.root.find_by_id_mut(id)
    }

    /// Finds multiple nodes by id.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderObjectTree, RenderStyle};
    /// let root = RenderObject::container(Some("root".to_string()), TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// let tree = RenderObjectTree::new(root);
    /// let nodes = tree.find_many(&["root".to_string()]);
    /// assert_eq!(nodes.len(), 1);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn find_many(&self, ids: &[String]) -> Vec<&RenderObject> {
        ids.iter().filter_map(|id| self.find_by_id(id)).collect()
    }

    /// Finds nodes whose id starts with a prefix.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderObjectTree, RenderStyle};
    /// let root = RenderObject::container(Some("root".to_string()), TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// let tree = RenderObjectTree::new(root);
    /// let nodes = tree.find_by_prefix("roo");
    /// assert_eq!(nodes.len(), 1);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn find_by_prefix(&self, prefix: &str) -> Vec<&RenderObject> {
        let mut results = Vec::new();
        find_by_prefix_impl(&self.root, prefix, &mut results);
        results
    }

    /// Collects node ids affected by a state path prefix.
    ///
    /// # Examples
    /// ```rust
    /// use taffy::Style as TaffyStyle;
    /// use xfw_layout::{RenderObject, RenderObjectTree, RenderStyle};
    /// let root = RenderObject::container(Some("store.battery".to_string()), TaffyStyle::default(), RenderStyle::default(), vec![]);
    /// let tree = RenderObjectTree::new(root);
    /// let ids = tree.get_affected_ids("store");
    /// assert_eq!(ids.len(), 1);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn get_affected_ids(&self, state_path: &str) -> Vec<String> {
        self.find_by_prefix(state_path)
            .iter()
            .filter_map(|n| n.id().map(String::from))
            .collect()
    }
}

fn build_map_impl(node: &RenderObject, map: &mut HashMap<String, usize>, depth: usize) {
    if let Some(id) = node.id() {
        map.insert(id.to_string(), depth);
    }
    if let Some(children) = node.children() {
        for child in children.iter() {
            build_map_impl(child, map, depth + 1);
        }
    }
}
