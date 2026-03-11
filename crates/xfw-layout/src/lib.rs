mod converter;
mod render_object_tree;

use taffy::prelude::*;

pub use converter::RenderObjectConverter;
pub use render_object_tree::{
    Anchor, Color, ImageFit, Kind, Layer, Rect, RenderObject, RenderObjectTree, RenderStyle,
    TextAlign,
};

pub struct LayoutEngine {
    taffy: taffy::TaffyTree,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            taffy: taffy::TaffyTree::new(),
        }
    }

    pub fn compute_layout(&mut self, tree: &mut RenderObjectTree) -> anyhow::Result<()> {
        self.compute_node(tree.root_mut())
    }

    fn compute_node(&mut self, node: &mut RenderObject) -> anyhow::Result<()> {
        let style = node.layout_style().clone();
        let child_count = node.children().map(|c| c.len()).unwrap_or(0);

        let child_ids: Vec<NodeId> = (0..child_count).map(NodeId::from).collect();

        let taffy_node = if child_ids.is_empty() {
            self.taffy.new_leaf(style)?
        } else {
            self.taffy.new_with_children(style, &child_ids)?
        };

        self.taffy.compute_layout(taffy_node, Size::MAX_CONTENT)?;

let layout = self.taffy.layout(taffy_node)?;
    *node.rect_mut() = Rect::from(layout);

        if let Some(children) = node.children_mut() {
            for child in children.iter_mut() {
                self.compute_node(child)?;
            }
        }

        Ok(())
    }
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}
