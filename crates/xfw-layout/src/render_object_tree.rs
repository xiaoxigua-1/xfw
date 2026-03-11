use std::collections::HashMap;
use taffy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
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
}

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
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "center" => TextAlign::Center,
            "right" => TextAlign::Right,
            "justify" => TextAlign::Justify,
            _ => TextAlign::Left,
        }
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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
pub enum Kind {
    Container,
    Text,
    Image,
}

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

    pub fn id(&self) -> Option<&str> {
        match self {
            Self::Container { id, .. } => id.as_deref(),
            Self::Text { id, .. } => id.as_deref(),
            Self::Image { id, .. } => id.as_deref(),
        }
    }

    pub fn layout_style(&self) -> &Style {
        match self {
            Self::Container { layout_style, .. } => layout_style,
            Self::Text { layout_style, .. } => layout_style,
            Self::Image { layout_style, .. } => layout_style,
        }
    }

    pub fn layout_style_mut(&mut self) -> &mut Style {
        match self {
            Self::Container { layout_style, .. } => layout_style,
            Self::Text { layout_style, .. } => layout_style,
            Self::Image { layout_style, .. } => layout_style,
        }
    }

    pub fn render_style(&self) -> &RenderStyle {
        match self {
            Self::Container { render_style, .. } => render_style,
            Self::Text { render_style, .. } => render_style,
            Self::Image { render_style, .. } => render_style,
        }
    }

    pub fn render_style_mut(&mut self) -> &mut RenderStyle {
        match self {
            Self::Container { render_style, .. } => render_style,
            Self::Text { render_style, .. } => render_style,
            Self::Image { render_style, .. } => render_style,
        }
    }

    pub fn rect(&self) -> &Rect {
        match self {
            Self::Container { rect, .. } => rect,
            Self::Text { rect, .. } => rect,
            Self::Image { rect, .. } => rect,
        }
    }

    pub fn rect_mut(&mut self) -> &mut Rect {
        match self {
            Self::Container { rect, .. } => rect,
            Self::Text { rect, .. } => rect,
            Self::Image { rect, .. } => rect,
        }
    }

    pub fn children(&self) -> Option<&[RenderObject]> {
        match self {
            Self::Container { children, .. } => Some(children),
            _ => None,
        }
    }

    pub fn children_mut(&mut self) -> Option<&mut Vec<RenderObject>> {
        match self {
            Self::Container { children, .. } => Some(children),
            _ => None,
        }
    }

    pub fn kind(&self) -> Kind {
        match self {
            Self::Container { .. } => Kind::Container,
            Self::Text { .. } => Kind::Text,
            Self::Image { .. } => Kind::Image,
        }
    }

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

pub struct RenderObjectTree {
    root: RenderObject,
    node_map: HashMap<String, usize>,
}

impl RenderObjectTree {
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

    pub fn root(&self) -> &RenderObject {
        &self.root
    }

    pub fn root_mut(&mut self) -> &mut RenderObject {
        &mut self.root
    }

    pub fn node_count(&self) -> usize {
        count_nodes_impl(&self.root)
    }

    pub fn find_by_id(&self, id: &str) -> Option<&RenderObject> {
        self.root.find_by_id(id)
    }

    pub fn find_by_id_mut(&mut self, id: &str) -> Option<&mut RenderObject> {
        self.root.find_by_id_mut(id)
    }

    pub fn find_many(&self, ids: &[String]) -> Vec<&RenderObject> {
        ids.iter().filter_map(|id| self.find_by_id(id)).collect()
    }

    pub fn find_by_prefix(&self, prefix: &str) -> Vec<&RenderObject> {
        let mut results = Vec::new();
        find_by_prefix_impl(&self.root, prefix, &mut results);
        results
    }

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
