use taffy::style::Style as TaffyStyle;
use xfw_layout::{RenderObject, RenderObjectTree, RenderStyle};
use xfw_render::{DrawCommand, Renderer};

#[test]
fn test_draw_commands_empty_tree() {
    let node = RenderObject::container(
        Some("root".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        vec![],
    );
    let tree = RenderObjectTree::new(node);
    let mut renderer = Renderer::new(1920, 1080);

    let commands = renderer.render(&tree, None).unwrap();

    assert!(commands.is_empty());
}

#[test]
fn test_draw_commands_with_background() {
    use xfw_layout::Color;

    let render_style = RenderStyle {
        background_color: Some(Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }),
        ..Default::default()
    };
    let node = RenderObject::container(
        Some("root".to_string()),
        TaffyStyle::default(),
        render_style,
        vec![],
    );
    let mut tree = RenderObjectTree::new(node);

    let root = tree.root_mut();
    *root.rect_mut() = xfw_layout::Rect {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 50.0,
    };

    let mut renderer = Renderer::new(1920, 1080);
    let commands = renderer.render(&tree, None).unwrap();

    assert_eq!(commands.len(), 1);
    match &commands[0] {
        DrawCommand::FillRect {
            rect,
            color,
            border_radius: _,
            opacity: _,
        } => {
            assert_eq!(rect.width, 100.0);
            assert_eq!(color.0, 1.0);
        }
        _ => panic!("Expected FillRect"),
    }
}

#[test]
fn test_draw_commands_text_node() {
    use xfw_layout::Color;

    let render_style = RenderStyle {
        color: Some(Color {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        }),
        font_size: Some(16.0),
        ..Default::default()
    };
    let node = RenderObject::text(
        Some("text1".to_string()),
        TaffyStyle::default(),
        render_style,
        "Hello".to_string(),
    );
    let mut tree = RenderObjectTree::new(node);

    let root = tree.root_mut();
    *root.rect_mut() = xfw_layout::Rect {
        x: 10.0,
        y: 20.0,
        width: 50.0,
        height: 20.0,
    };

    let mut renderer = Renderer::new(1920, 1080);
    let commands = renderer.render(&tree, None).unwrap();

    assert_eq!(commands.len(), 1);
    match &commands[0] {
        DrawCommand::DrawText {
            text,
            x,
            y,
            width: _,
            color,
            font_size,
            font_family: _,
            text_align: _,
        } => {
            assert_eq!(text, "Hello");
            assert_eq!(*x, 10.0);
            assert_eq!(*y, 20.0);
            assert_eq!(color.2, 1.0);
            assert_eq!(*font_size, 16.0);
        }
        _ => panic!("Expected DrawText"),
    }
}

#[test]
fn test_draw_commands_image_node() {
    let node = RenderObject::image(
        Some("img1".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        "/path/to/image.png".to_string(),
    );
    let mut tree = RenderObjectTree::new(node);

    let root = tree.root_mut();
    *root.rect_mut() = xfw_layout::Rect {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    let mut renderer = Renderer::new(1920, 1080);
    let commands = renderer.render(&tree, None).unwrap();

    assert_eq!(commands.len(), 1);
    match &commands[0] {
        DrawCommand::DrawImage {
            path,
            rect,
            image_fit: _,
        } => {
            assert_eq!(path, "/path/to/image.png");
            assert_eq!(rect.width, 100.0);
        }
        _ => panic!("Expected DrawImage"),
    }
}

#[test]
fn test_draw_commands_nested_container() {
    use xfw_layout::Color;

    let child_style = RenderStyle {
        background_color: Some(Color {
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        }),
        ..Default::default()
    };
    let child = RenderObject::container(
        Some("child".to_string()),
        TaffyStyle::default(),
        child_style,
        vec![],
    );

    let parent_style = RenderStyle {
        background_color: Some(Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }),
        ..Default::default()
    };
    let parent = RenderObject::container(
        Some("parent".to_string()),
        TaffyStyle::default(),
        parent_style,
        vec![child],
    );

    let mut tree = RenderObjectTree::new(parent);

    {
        let root = tree.root_mut();
        *root.rect_mut() = xfw_layout::Rect {
            x: 0.0,
            y: 0.0,
            width: 200.0,
            height: 100.0,
        };
        if let Some(children) = root.children_mut() {
            if let Some(c) = children.get_mut(0) {
                *c.rect_mut() = xfw_layout::Rect {
                    x: 10.0,
                    y: 10.0,
                    width: 50.0,
                    height: 50.0,
                };
            }
        }
    }

    let mut renderer = Renderer::new(1920, 1080);
    let commands = renderer.render(&tree, None).unwrap();

    assert_eq!(commands.len(), 2);
}

#[test]
fn test_draw_commands_skips_zero_size() {
    let node = RenderObject::container(
        Some("root".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        vec![],
    );
    let mut tree = RenderObjectTree::new(node);

    let root = tree.root_mut();
    *root.rect_mut() = xfw_layout::Rect {
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
    };

    let mut renderer = Renderer::new(1920, 1080);
    let commands = renderer.render(&tree, None).unwrap();

    assert!(commands.is_empty());
}
