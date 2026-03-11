mod converter;
mod render_object_tree;

use anyhow::anyhow;
use taffy::prelude::*;

pub use converter::RenderObjectConverter;
pub use render_object_tree::{
    Anchor, Color, ImageFit, Kind, Layer, OverflowBehavior, Rect, RenderObject, RenderObjectTree,
    RenderStyle, TextAlign,
};

#[derive(Clone, Debug)]
struct TextContext {
    text: String,
    font_size: f32,
}

pub struct LayoutEngine {
    taffy: taffy::TaffyTree<TextContext>,
}

struct NodeTree {
    id: NodeId,
    children: Vec<NodeTree>,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            taffy: taffy::TaffyTree::new(),
        }
    }

    pub fn compute_layout(&mut self, tree: &mut RenderObjectTree) -> anyhow::Result<()> {
        self.taffy = taffy::TaffyTree::new();
        let node_tree = self.build_taffy_tree(tree.root())?;
        self.taffy.compute_layout_with_measure(
            node_tree.id,
            Size::MAX_CONTENT,
            |known_dimensions, available_space, _node, context, _style| {
                match context {
                    Some(context) => measure_text(context, known_dimensions, available_space),
                    None => Size::ZERO,
                }
            },
        )?;
        self.apply_layouts(tree.root_mut(), &node_tree, (0.0, 0.0))
    }

    fn build_taffy_tree(&mut self, node: &RenderObject) -> anyhow::Result<NodeTree> {
        let style = node.layout_style().clone();
        let mut children_nodes = Vec::new();

        if let Some(children) = node.children() {
            for child in children.iter() {
                children_nodes.push(self.build_taffy_tree(child)?);
            }
        }

        let child_ids: Vec<NodeId> = children_nodes.iter().map(|c| c.id).collect();
        let taffy_node = if child_ids.is_empty() {
            self.taffy.new_leaf(style)?
        } else {
            self.taffy.new_with_children(style, &child_ids)?
        };

        if let RenderObject::Text {
            content,
            render_style,
            ..
        } = node
        {
            let font_size = render_style.font_size.unwrap_or(14.0);
            let context = TextContext {
                text: content.clone(),
                font_size,
            };
            self.taffy.set_node_context(taffy_node, Some(context))?;
        }

        Ok(NodeTree {
            id: taffy_node,
            children: children_nodes,
        })
    }

    fn apply_layouts(
        &mut self,
        node: &mut RenderObject,
        node_tree: &NodeTree,
        origin: (f32, f32),
    ) -> anyhow::Result<()> {
        let layout = self.taffy.layout(node_tree.id)?;
        let x = origin.0 + layout.location.x;
        let y = origin.1 + layout.location.y;
        *node.rect_mut() = Rect {
            x,
            y,
            width: layout.size.width,
            height: layout.size.height,
        };

        match node.children_mut() {
            Some(children) => {
                if children.len() != node_tree.children.len() {
                    return Err(anyhow!("layout tree mismatch"));
                }
                for (child, child_tree) in children.iter_mut().zip(node_tree.children.iter()) {
                    self.apply_layouts(child, child_tree, (x, y))?;
                }
            }
            None => {
                if !node_tree.children.is_empty() {
                    return Err(anyhow!("layout tree mismatch"));
                }
            }
        }

        Ok(())
    }
}

fn measure_text(
    context: &mut TextContext,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
) -> Size<f32> {
    let len = context.text.chars().count().max(1) as f32;
    let mut width = context.font_size * 0.6 * len;
    let mut height = context.font_size * 1.2;

    if let Some(w) = known_dimensions.width {
        width = w;
    } else if let AvailableSpace::Definite(w) = available_space.width {
        width = width.min(w);
    }

    if let Some(h) = known_dimensions.height {
        height = h;
    } else if let AvailableSpace::Definite(h) = available_space.height {
        height = height.min(h);
    }

    Size { width, height }
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}
