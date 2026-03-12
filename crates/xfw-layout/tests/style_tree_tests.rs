use taffy::style::Style as TaffyStyle;
use xfw_layout::{RenderObject, RenderObjectTree, RenderStyle};

#[test]
fn test_render_object_tree_new() {
    let node = RenderObject::container(
        Some("root".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        vec![],
    );
    let tree = RenderObjectTree::new(node);
    assert_eq!(tree.node_count(), 1);
}

#[test]
fn test_render_object_tree_with_children() {
    let child1 = RenderObject::container(
        Some("child1".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        vec![],
    );
    let child2 = RenderObject::container(
        Some("child2".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        vec![],
    );
    let root = RenderObject::container(
        Some("root".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        vec![child1, child2],
    );
    let tree = RenderObjectTree::new(root);
    assert_eq!(tree.node_count(), 3);
}

#[test]
fn test_find_by_id() {
    let child = RenderObject::container(
        Some("target".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        vec![],
    );
    let root = RenderObject::container(
        Some("root".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        vec![child],
    );
    let tree = RenderObjectTree::new(root);

    let found = tree.find_by_id("target");
    assert!(found.is_some());

    let not_found = tree.find_by_id("nonexistent");
    assert!(not_found.is_none());
}

#[test]
fn test_find_by_prefix() {
    let node1 = RenderObject::container(
        Some("battery".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        vec![],
    );
    let node2 = RenderObject::container(
        Some("battery_level".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        vec![],
    );
    let node3 = RenderObject::container(
        Some("volume".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        vec![],
    );
    let root = RenderObject::container(
        Some("root".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        vec![node1, node2, node3],
    );
    let tree = RenderObjectTree::new(root);

    let results = tree.find_by_prefix("battery");
    assert_eq!(results.len(), 2);
}

#[test]
fn test_text_node() {
    let node = RenderObject::text(
        Some("text1".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        "Hello".to_string(),
    );
    let tree = RenderObjectTree::new(node);

    let found = tree.find_by_id("text1").unwrap();
    if let RenderObject::Text { content, .. } = found {
        assert_eq!(content, "Hello");
    } else {
        panic!("Expected Text node");
    }
}

#[test]
fn test_image_node() {
    let node = RenderObject::image(
        Some("img1".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        "/path/to/image.png".to_string(),
    );
    let tree = RenderObjectTree::new(node);

    let found = tree.find_by_id("img1").unwrap();
    if let RenderObject::Image { path, .. } = found {
        assert_eq!(path, "/path/to/image.png");
    } else {
        panic!("Expected Image node");
    }
}

#[test]
fn test_dirty_rects_single_node() {
    use xfw_layout::Rect;

    let node = RenderObject::container(
        Some("root".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        vec![],
    );
    let tree = RenderObjectTree::new(node);

    let rects = tree.get_dirty_rects(&["root".to_string()]);
    assert!(!rects.is_empty());
}

#[test]
fn test_dirty_bbox_single_node() {
    use xfw_layout::Rect;

    let node = RenderObject::container(
        Some("root".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        vec![],
    );
    let tree = RenderObjectTree::new(node);

    let bbox = tree.compute_dirty_bbox(&["root".to_string()]);
    assert!(bbox.is_some());
}
