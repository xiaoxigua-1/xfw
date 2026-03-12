use std::fs::create_dir_all;
use std::path::Path;
use taffy::style::Style as TaffyStyle;
use xfw_layout::{Color, RenderObject, RenderObjectTree, RenderStyle};
use xfw_render::{DrawCommand, PixmapRenderer, Renderer};

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

    if std::env::var_os("XFW_RENDER_DUMP").is_some() {
        let mut pixmap = PixmapRenderer::new(100, 50).unwrap();
        pixmap.clear((0.0, 0.0, 0.0, 0.0));
        pixmap.execute(&commands).unwrap();
        maybe_dump_png("background_fill", &mut pixmap);
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

    if std::env::var_os("XFW_RENDER_DUMP").is_some() {
        let mut pixmap = PixmapRenderer::new(120, 40).unwrap();
        pixmap.clear((0.0, 0.0, 0.0, 0.0));
        pixmap.execute(&commands).unwrap();
        maybe_dump_png("text_draw", &mut pixmap);
    }
}

#[test]
fn test_draw_commands_image_node() {
    let image_path = create_test_png("sample_image", 4, 4);
    let node = RenderObject::image(
        Some("img1".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        image_path.to_string_lossy().to_string(),
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
            assert_eq!(path, image_path.to_string_lossy().as_ref());
            assert_eq!(rect.width, 100.0);
        }
        _ => panic!("Expected DrawImage"),
    }

    if std::env::var_os("XFW_RENDER_DUMP").is_some() {
        let mut pixmap = PixmapRenderer::new(100, 100).unwrap();
        pixmap.clear((0.0, 0.0, 0.0, 0.0));
        pixmap.execute(&commands).unwrap();
        maybe_dump_png("image_draw", &mut pixmap);
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

    if std::env::var_os("XFW_RENDER_DUMP").is_some() {
        let mut pixmap = PixmapRenderer::new(200, 100).unwrap();
        pixmap.clear((0.0, 0.0, 0.0, 0.0));
        pixmap.execute(&commands).unwrap();
        maybe_dump_png("nested_container", &mut pixmap);
    }
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

fn pixel_rgba(renderer: &PixmapRenderer, x: u32, y: u32) -> (u8, u8, u8, u8) {
    let width = renderer.width() as usize;
    let idx = ((y as usize * width) + x as usize) * 4;
    let data = renderer.data();
    (data[idx], data[idx + 1], data[idx + 2], data[idx + 3])
}

fn region_has_non_background(
    renderer: &PixmapRenderer,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    bg: (u8, u8, u8, u8),
) -> bool {
    let width_px = renderer.width();
    for yy in y..(y + height) {
        if yy >= renderer.height() {
            break;
        }
        for xx in x..(x + width) {
            if xx >= width_px {
                break;
            }
            let pixel = pixel_rgba(renderer, xx, yy);
            if pixel != bg {
                return true;
            }
        }
    }
    false
}

fn maybe_dump_png(name: &str, renderer: &mut PixmapRenderer) {
    if std::env::var_os("XFW_RENDER_DUMP").is_none() {
        return;
    }
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("target")
        .join("xfw-render-dumps");
    if let Err(err) = create_dir_all(&dir) {
        eprintln!("Failed to create dump dir: {err}");
        return;
    }
    let path = dir.join(format!("{name}.png"));
    if let Err(err) = renderer.pixmap_mut().save_png(&path) {
        eprintln!("Failed to save png {path:?}: {err}");
    }
}

fn create_test_png(name: &str, width: u32, height: u32) -> std::path::PathBuf {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("target")
        .join("xfw-render-test-assets");
    if let Err(err) = create_dir_all(&dir) {
        panic!("Failed to create test assets dir: {err}");
    }
    let path = dir.join(format!("{name}.png"));
    let mut pixmap = PixmapRenderer::new(width, height).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 0.0));
    let commands = vec![DrawCommand::FillRect {
        rect: xfw_layout::Rect {
            x: 0.0,
            y: 0.0,
            width: width as f32,
            height: height as f32,
        },
        color: (0.0, 0.0, 1.0, 1.0),
        border_radius: None,
        opacity: 1.0,
    }];
    pixmap.execute(&commands).unwrap();
    pixmap.pixmap_mut().save_png(&path).unwrap();
    path
}

#[test]
fn test_pixmap_renderer_clip_path_masks_fill() {
    let mut pixmap = PixmapRenderer::new(10, 10).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 0.0));

    let commands = vec![
        DrawCommand::ClipPath {
            rect: xfw_layout::Rect {
                x: 0.0,
                y: 0.0,
                width: 5.0,
                height: 5.0,
            },
            border_radius: 0.0,
        },
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 10.0,
            },
            color: (1.0, 0.0, 0.0, 1.0),
            border_radius: None,
            opacity: 1.0,
        },
        DrawCommand::PopClip,
    ];

    pixmap.execute(&commands).unwrap();
    maybe_dump_png("clip_masks_fill", &mut pixmap);

    let inside = pixel_rgba(&pixmap, 2, 2);
    let outside = pixel_rgba(&pixmap, 7, 7);
    assert_eq!(inside.3, 255);
    assert_eq!(outside.3, 0);
}

#[test]
fn test_pixmap_renderer_pop_clip_restores_drawing() {
    let mut pixmap = PixmapRenderer::new(10, 10).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 0.0));

    let commands = vec![
        DrawCommand::ClipPath {
            rect: xfw_layout::Rect {
                x: 0.0,
                y: 0.0,
                width: 5.0,
                height: 5.0,
            },
            border_radius: 0.0,
        },
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 10.0,
            },
            color: (1.0, 0.0, 0.0, 1.0),
            border_radius: None,
            opacity: 1.0,
        },
        DrawCommand::PopClip,
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 10.0,
            },
            color: (0.0, 1.0, 0.0, 1.0),
            border_radius: None,
            opacity: 1.0,
        },
    ];

    pixmap.execute(&commands).unwrap();
    maybe_dump_png("clip_pop_restores", &mut pixmap);

    let outside = pixel_rgba(&pixmap, 7, 7);
    assert_eq!(outside.0, 0);
    assert_eq!(outside.1, 255);
    assert_eq!(outside.3, 255);
}

#[test]
fn test_pixmap_renderer_rounded_rect_fill() {
    let mut pixmap = PixmapRenderer::new(10, 10).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 0.0));

    let commands = vec![DrawCommand::FillRect {
        rect: xfw_layout::Rect {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        },
        color: (1.0, 0.0, 0.0, 1.0),
        border_radius: Some(4.0),
        opacity: 1.0,
    }];

    pixmap.execute(&commands).unwrap();
    maybe_dump_png("rounded_rect_fill", &mut pixmap);

    let corner = pixel_rgba(&pixmap, 0, 0);
    let inside = pixel_rgba(&pixmap, 5, 5);
    assert_eq!(corner.3, 0);
    assert_eq!(inside.3, 255);
}

#[test]
fn test_pixmap_renderer_combined_clip_round_image() {
    let image_path = create_test_png("combo_image", 6, 6);
    let mut pixmap = PixmapRenderer::new(20, 20).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 0.0));

    let commands = vec![
        DrawCommand::ClipPath {
            rect: xfw_layout::Rect {
                x: 2.0,
                y: 2.0,
                width: 16.0,
                height: 16.0,
            },
            border_radius: 6.0,
        },
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 2.0,
                y: 2.0,
                width: 16.0,
                height: 16.0,
            },
            color: (1.0, 0.0, 0.0, 1.0),
            border_radius: Some(6.0),
            opacity: 1.0,
        },
        DrawCommand::DrawImage {
            path: image_path.to_string_lossy().to_string(),
            rect: xfw_layout::Rect {
                x: 4.0,
                y: 4.0,
                width: 12.0,
                height: 12.0,
            },
            image_fit: xfw_layout::ImageFit::Contain,
        },
        DrawCommand::PopClip,
        DrawCommand::StrokeRect {
            rect: xfw_layout::Rect {
                x: 2.0,
                y: 2.0,
                width: 16.0,
                height: 16.0,
            },
            color: (0.0, 1.0, 0.0, 1.0),
            width: 1.0,
            border_radius: Some(6.0),
        },
    ];

    pixmap.execute(&commands).unwrap();
    maybe_dump_png("combined_clip_round_image", &mut pixmap);

    let outside = pixel_rgba(&pixmap, 1, 1);
    let inside = pixel_rgba(&pixmap, 10, 10);
    assert_eq!(outside.3, 0);
    assert!(inside.3 > 0);
}

#[test]
fn test_pixmap_renderer_clip_round_text() {
    let mut pixmap = PixmapRenderer::new(120, 40).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 0.0));

    let commands = vec![
        DrawCommand::ClipPath {
            rect: xfw_layout::Rect {
                x: 4.0,
                y: 4.0,
                width: 112.0,
                height: 32.0,
            },
            border_radius: 8.0,
        },
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 4.0,
                y: 4.0,
                width: 112.0,
                height: 32.0,
            },
            color: (0.2, 0.2, 0.2, 1.0),
            border_radius: Some(8.0),
            opacity: 1.0,
        },
        DrawCommand::DrawText {
            text: "Rounded Clip".to_string(),
            x: 10.0,
            y: 10.0,
            width: 100.0,
            color: (1.0, 1.0, 1.0, 1.0),
            font_size: 14.0,
            font_family: None,
            text_align: xfw_layout::TextAlign::Left,
        },
        DrawCommand::PopClip,
    ];

    pixmap.execute(&commands).unwrap();
    maybe_dump_png("clip_round_text", &mut pixmap);

    let outside = pixel_rgba(&pixmap, 1, 1);
    let inside = pixel_rgba(&pixmap, 10, 10);
    assert_eq!(outside.3, 0);
    assert!(inside.3 > 0);
}

#[test]
fn test_pixmap_renderer_round_text_image_combo() {
    let image_path = create_test_png("combo_text_image", 8, 8);
    let mut pixmap = PixmapRenderer::new(140, 60).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 0.0));

    let commands = vec![
        DrawCommand::ClipPath {
            rect: xfw_layout::Rect {
                x: 6.0,
                y: 6.0,
                width: 128.0,
                height: 48.0,
            },
            border_radius: 10.0,
        },
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 6.0,
                y: 6.0,
                width: 128.0,
                height: 48.0,
            },
            color: (0.1, 0.1, 0.3, 1.0),
            border_radius: Some(10.0),
            opacity: 1.0,
        },
        DrawCommand::DrawImage {
            path: image_path.to_string_lossy().to_string(),
            rect: xfw_layout::Rect {
                x: 12.0,
                y: 12.0,
                width: 36.0,
                height: 36.0,
            },
            image_fit: xfw_layout::ImageFit::Contain,
        },
        DrawCommand::DrawText {
            text: "Image + Text".to_string(),
            x: 54.0,
            y: 20.0,
            width: 76.0,
            color: (1.0, 1.0, 1.0, 1.0),
            font_size: 14.0,
            font_family: None,
            text_align: xfw_layout::TextAlign::Left,
        },
        DrawCommand::PopClip,
        DrawCommand::StrokeRect {
            rect: xfw_layout::Rect {
                x: 6.0,
                y: 6.0,
                width: 128.0,
                height: 48.0,
            },
            color: (0.0, 1.0, 0.0, 1.0),
            width: 1.0,
            border_radius: Some(10.0),
        },
    ];

    pixmap.execute(&commands).unwrap();
    maybe_dump_png("round_text_image_combo", &mut pixmap);

    let outside = pixel_rgba(&pixmap, 2, 2);
    let inside = pixel_rgba(&pixmap, 20, 20);
    assert_eq!(outside.3, 0);
    assert!(inside.3 > 0);
}

#[test]
fn test_pixmap_renderer_nested_clip_stack() {
    let mut pixmap = PixmapRenderer::new(30, 30).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 0.0));

    let commands = vec![
        DrawCommand::ClipPath {
            rect: xfw_layout::Rect {
                x: 2.0,
                y: 2.0,
                width: 26.0,
                height: 26.0,
            },
            border_radius: 4.0,
        },
        DrawCommand::ClipPath {
            rect: xfw_layout::Rect {
                x: 8.0,
                y: 8.0,
                width: 14.0,
                height: 14.0,
            },
            border_radius: 4.0,
        },
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 0.0,
                y: 0.0,
                width: 30.0,
                height: 30.0,
            },
            color: (0.0, 0.5, 1.0, 1.0),
            border_radius: None,
            opacity: 1.0,
        },
        DrawCommand::PopClip,
        DrawCommand::PopClip,
    ];

    pixmap.execute(&commands).unwrap();
    maybe_dump_png("nested_clip_stack", &mut pixmap);

    let outside = pixel_rgba(&pixmap, 4, 4);
    let inside = pixel_rgba(&pixmap, 15, 15);
    assert_eq!(outside.3, 0);
    assert!(inside.3 > 0);
}

#[test]
fn test_pixmap_renderer_image_fit_variants() {
    let image_path = create_test_png("fit_variants", 8, 6);
    let mut pixmap = PixmapRenderer::new(180, 40).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 0.0));

    let fits = [
        xfw_layout::ImageFit::Fill,
        xfw_layout::ImageFit::Contain,
        xfw_layout::ImageFit::Cover,
        xfw_layout::ImageFit::FitWidth,
        xfw_layout::ImageFit::FitHeight,
    ];

    let mut commands = Vec::new();
    for (idx, fit) in fits.iter().enumerate() {
        let x = 4.0 + (idx as f32 * 34.0);
        commands.push(DrawCommand::StrokeRect {
            rect: xfw_layout::Rect {
                x,
                y: 4.0,
                width: 30.0,
                height: 30.0,
            },
            color: (0.2, 0.2, 0.2, 1.0),
            width: 1.0,
            border_radius: Some(4.0),
        });
        commands.push(DrawCommand::DrawImage {
            path: image_path.to_string_lossy().to_string(),
            rect: xfw_layout::Rect {
                x: x + 2.0,
                y: 6.0,
                width: 26.0,
                height: 26.0,
            },
            image_fit: *fit,
        });
    }

    pixmap.execute(&commands).unwrap();
    maybe_dump_png("image_fit_variants", &mut pixmap);

    let sample = pixel_rgba(&pixmap, 10, 10);
    assert!(sample.3 > 0);
}

#[test]
fn test_pixmap_renderer_layered_combo() {
    let image_path = create_test_png("layered_combo", 12, 12);
    let mut pixmap = PixmapRenderer::new(160, 80).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 0.0));

    let commands = vec![
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 0.0,
                y: 0.0,
                width: 160.0,
                height: 80.0,
            },
            color: (0.08, 0.08, 0.1, 1.0),
            border_radius: None,
            opacity: 1.0,
        },
        DrawCommand::ClipPath {
            rect: xfw_layout::Rect {
                x: 8.0,
                y: 8.0,
                width: 144.0,
                height: 64.0,
            },
            border_radius: 12.0,
        },
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 8.0,
                y: 8.0,
                width: 144.0,
                height: 64.0,
            },
            color: (0.2, 0.2, 0.25, 1.0),
            border_radius: Some(12.0),
            opacity: 1.0,
        },
        DrawCommand::DrawImage {
            path: image_path.to_string_lossy().to_string(),
            rect: xfw_layout::Rect {
                x: 16.0,
                y: 16.0,
                width: 48.0,
                height: 48.0,
            },
            image_fit: xfw_layout::ImageFit::Cover,
        },
        DrawCommand::DrawText {
            text: "Layered Combo".to_string(),
            x: 72.0,
            y: 28.0,
            width: 72.0,
            color: (1.0, 1.0, 1.0, 1.0),
            font_size: 16.0,
            font_family: None,
            text_align: xfw_layout::TextAlign::Center,
        },
        DrawCommand::PopClip,
        DrawCommand::StrokeRect {
            rect: xfw_layout::Rect {
                x: 8.0,
                y: 8.0,
                width: 144.0,
                height: 64.0,
            },
            color: (0.0, 0.7, 0.3, 1.0),
            width: 2.0,
            border_radius: Some(12.0),
        },
    ];

    pixmap.execute(&commands).unwrap();
    maybe_dump_png("layered_combo", &mut pixmap);

    let outside = pixel_rgba(&pixmap, 2, 2);
    let inside = pixel_rgba(&pixmap, 20, 20);
    assert_eq!(outside.3, 255);
    assert!(inside.3 > 0);
}

#[test]
fn test_pixmap_renderer_nested_clip_text_image_stroke() {
    let image_path = create_test_png("nested_clip_combo", 10, 10);
    let mut pixmap = PixmapRenderer::new(180, 90).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 0.0));

    let commands = vec![
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 0.0,
                y: 0.0,
                width: 180.0,
                height: 90.0,
            },
            color: (0.05, 0.05, 0.07, 1.0),
            border_radius: None,
            opacity: 1.0,
        },
        DrawCommand::ClipPath {
            rect: xfw_layout::Rect {
                x: 10.0,
                y: 10.0,
                width: 160.0,
                height: 70.0,
            },
            border_radius: 12.0,
        },
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 10.0,
                y: 10.0,
                width: 160.0,
                height: 70.0,
            },
            color: (0.16, 0.16, 0.2, 1.0),
            border_radius: Some(12.0),
            opacity: 1.0,
        },
        DrawCommand::ClipPath {
            rect: xfw_layout::Rect {
                x: 16.0,
                y: 16.0,
                width: 60.0,
                height: 58.0,
            },
            border_radius: 10.0,
        },
        DrawCommand::DrawImage {
            path: image_path.to_string_lossy().to_string(),
            rect: xfw_layout::Rect {
                x: 16.0,
                y: 16.0,
                width: 60.0,
                height: 58.0,
            },
            image_fit: xfw_layout::ImageFit::Cover,
        },
        DrawCommand::PopClip,
        DrawCommand::DrawText {
            text: "Nested Clip + Image".to_string(),
            x: 84.0,
            y: 30.0,
            width: 80.0,
            color: (1.0, 1.0, 1.0, 1.0),
            font_size: 14.0,
            font_family: None,
            text_align: xfw_layout::TextAlign::Left,
        },
        DrawCommand::DrawText {
            text: "Right aligned".to_string(),
            x: 84.0,
            y: 50.0,
            width: 80.0,
            color: (0.7, 0.9, 1.0, 1.0),
            font_size: 12.0,
            font_family: None,
            text_align: xfw_layout::TextAlign::Right,
        },
        DrawCommand::PopClip,
        DrawCommand::StrokeRect {
            rect: xfw_layout::Rect {
                x: 10.0,
                y: 10.0,
                width: 160.0,
                height: 70.0,
            },
            color: (0.0, 0.7, 0.3, 1.0),
            width: 2.0,
            border_radius: Some(12.0),
        },
    ];

    pixmap.execute(&commands).unwrap();
    maybe_dump_png("nested_clip_text_image_stroke", &mut pixmap);

    let outside = pixel_rgba(&pixmap, 4, 4);
    let inside = pixel_rgba(&pixmap, 20, 20);
    assert_eq!(outside.3, 255);
    assert!(inside.3 > 0);
}

#[test]
fn test_pixmap_renderer_stacked_clips_and_strokes() {
    let mut pixmap = PixmapRenderer::new(120, 120).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 0.0));

    let commands = vec![
        DrawCommand::ClipPath {
            rect: xfw_layout::Rect {
                x: 6.0,
                y: 6.0,
                width: 108.0,
                height: 108.0,
            },
            border_radius: 14.0,
        },
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 6.0,
                y: 6.0,
                width: 108.0,
                height: 108.0,
            },
            color: (0.15, 0.18, 0.2, 1.0),
            border_radius: Some(14.0),
            opacity: 1.0,
        },
        DrawCommand::ClipPath {
            rect: xfw_layout::Rect {
                x: 20.0,
                y: 20.0,
                width: 80.0,
                height: 80.0,
            },
            border_radius: 10.0,
        },
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 20.0,
                y: 20.0,
                width: 80.0,
                height: 80.0,
            },
            color: (0.2, 0.25, 0.3, 1.0),
            border_radius: Some(10.0),
            opacity: 1.0,
        },
        DrawCommand::ClipPath {
            rect: xfw_layout::Rect {
                x: 34.0,
                y: 34.0,
                width: 52.0,
                height: 52.0,
            },
            border_radius: 8.0,
        },
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 34.0,
                y: 34.0,
                width: 52.0,
                height: 52.0,
            },
            color: (0.3, 0.4, 0.5, 1.0),
            border_radius: Some(8.0),
            opacity: 1.0,
        },
        DrawCommand::PopClip,
        DrawCommand::StrokeRect {
            rect: xfw_layout::Rect {
                x: 34.0,
                y: 34.0,
                width: 52.0,
                height: 52.0,
            },
            color: (1.0, 0.6, 0.2, 1.0),
            width: 2.0,
            border_radius: Some(8.0),
        },
        DrawCommand::PopClip,
        DrawCommand::StrokeRect {
            rect: xfw_layout::Rect {
                x: 20.0,
                y: 20.0,
                width: 80.0,
                height: 80.0,
            },
            color: (0.4, 0.9, 0.3, 1.0),
            width: 2.0,
            border_radius: Some(10.0),
        },
        DrawCommand::PopClip,
        DrawCommand::StrokeRect {
            rect: xfw_layout::Rect {
                x: 6.0,
                y: 6.0,
                width: 108.0,
                height: 108.0,
            },
            color: (0.2, 0.8, 1.0, 1.0),
            width: 2.0,
            border_radius: Some(14.0),
        },
    ];

    pixmap.execute(&commands).unwrap();
    maybe_dump_png("stacked_clips_and_strokes", &mut pixmap);

    let outer = pixel_rgba(&pixmap, 10, 10);
    let inner = pixel_rgba(&pixmap, 60, 60);
    assert!(outer.3 > 0);
    assert!(inner.3 > 0);
}

#[test]
fn test_pixmap_renderer_multiline_text_wrap() {
    let mut pixmap = PixmapRenderer::new(200, 80).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 0.0));

    let commands = vec![
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 6.0,
                y: 6.0,
                width: 188.0,
                height: 68.0,
            },
            color: (0.12, 0.12, 0.14, 1.0),
            border_radius: Some(8.0),
            opacity: 1.0,
        },
        DrawCommand::DrawText {
            text: "This is a long line that should wrap into multiple lines in the text buffer.\nSecond line here.".to_string(),
            x: 12.0,
            y: 12.0,
            width: 176.0,
            color: (1.0, 1.0, 1.0, 1.0),
            font_size: 12.0,
            font_family: None,
            text_align: xfw_layout::TextAlign::Left,
        },
    ];

    pixmap.execute(&commands).unwrap();
    maybe_dump_png("multiline_text_wrap", &mut pixmap);

    let sample = pixel_rgba(&pixmap, 20, 20);
    assert!(sample.3 > 0);
}

#[test]
fn test_pixmap_renderer_centered_text_in_clip() {
    let mut pixmap = PixmapRenderer::new(160, 60).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 0.0));

    let commands = vec![
        DrawCommand::ClipPath {
            rect: xfw_layout::Rect {
                x: 8.0,
                y: 8.0,
                width: 144.0,
                height: 44.0,
            },
            border_radius: 10.0,
        },
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 8.0,
                y: 8.0,
                width: 144.0,
                height: 44.0,
            },
            color: (0.18, 0.18, 0.22, 1.0),
            border_radius: Some(10.0),
            opacity: 1.0,
        },
        DrawCommand::DrawText {
            text: "Centered".to_string(),
            x: 8.0,
            y: 20.0,
            width: 144.0,
            color: (1.0, 1.0, 1.0, 1.0),
            font_size: 16.0,
            font_family: None,
            text_align: xfw_layout::TextAlign::Center,
        },
        DrawCommand::PopClip,
        DrawCommand::StrokeRect {
            rect: xfw_layout::Rect {
                x: 8.0,
                y: 8.0,
                width: 144.0,
                height: 44.0,
            },
            color: (0.2, 0.8, 1.0, 1.0),
            width: 2.0,
            border_radius: Some(10.0),
        },
    ];

    pixmap.execute(&commands).unwrap();
    maybe_dump_png("centered_text_in_clip", &mut pixmap);

    let outside = pixel_rgba(&pixmap, 4, 4);
    let inside = pixel_rgba(&pixmap, 80, 30);
    assert_eq!(outside.3, 0);
    assert!(inside.3 > 0);
}

#[test]
fn test_pixmap_renderer_mixed_text_sizes_and_alignment() {
    let mut pixmap = PixmapRenderer::new(220, 100).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 0.0));

    let commands = vec![
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 8.0,
                y: 8.0,
                width: 204.0,
                height: 84.0,
            },
            color: (0.14, 0.14, 0.18, 1.0),
            border_radius: Some(8.0),
            opacity: 1.0,
        },
        DrawCommand::DrawText {
            text: "Left large".to_string(),
            x: 14.0,
            y: 18.0,
            width: 192.0,
            color: (1.0, 1.0, 1.0, 1.0),
            font_size: 18.0,
            font_family: None,
            text_align: xfw_layout::TextAlign::Left,
        },
        DrawCommand::DrawText {
            text: "Centered medium".to_string(),
            x: 14.0,
            y: 40.0,
            width: 192.0,
            color: (0.9, 0.9, 1.0, 1.0),
            font_size: 14.0,
            font_family: None,
            text_align: xfw_layout::TextAlign::Center,
        },
        DrawCommand::DrawText {
            text: "Right small".to_string(),
            x: 14.0,
            y: 60.0,
            width: 192.0,
            color: (0.7, 0.9, 1.0, 1.0),
            font_size: 12.0,
            font_family: None,
            text_align: xfw_layout::TextAlign::Right,
        },
    ];

    pixmap.execute(&commands).unwrap();
    maybe_dump_png("mixed_text_sizes_alignment", &mut pixmap);

    let background = (
        (0.14 * 255.0) as u8,
        (0.14 * 255.0) as u8,
        (0.18 * 255.0) as u8,
        255,
    );
    assert!(region_has_non_background(
        &pixmap, 14, 18, 192, 20, background
    ));
}

#[test]
fn test_pixmap_renderer_image_opacity_overlay() {
    let image_path = create_test_png("opacity_overlay", 10, 10);
    let mut pixmap = PixmapRenderer::new(120, 60).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 0.0));

    let commands = vec![
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 0.0,
                y: 0.0,
                width: 120.0,
                height: 60.0,
            },
            color: (0.05, 0.05, 0.08, 1.0),
            border_radius: None,
            opacity: 1.0,
        },
        DrawCommand::DrawImage {
            path: image_path.to_string_lossy().to_string(),
            rect: xfw_layout::Rect {
                x: 10.0,
                y: 10.0,
                width: 40.0,
                height: 40.0,
            },
            image_fit: xfw_layout::ImageFit::Cover,
        },
        DrawCommand::FillRect {
            rect: xfw_layout::Rect {
                x: 10.0,
                y: 10.0,
                width: 40.0,
                height: 40.0,
            },
            color: (1.0, 0.0, 0.0, 0.4),
            border_radius: Some(6.0),
            opacity: 0.4,
        },
        DrawCommand::StrokeRect {
            rect: xfw_layout::Rect {
                x: 10.0,
                y: 10.0,
                width: 40.0,
                height: 40.0,
            },
            color: (1.0, 1.0, 1.0, 1.0),
            width: 1.0,
            border_radius: Some(6.0),
        },
    ];

    pixmap.execute(&commands).unwrap();
    maybe_dump_png("image_opacity_overlay", &mut pixmap);

    let inside = pixel_rgba(&pixmap, 20, 20);
    assert!(inside.3 > 0);
}

#[test]
fn test_dirty_rect_filtering() {
    let root = RenderObject::container(
        Some("root".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        vec![
            RenderObject::container(
                Some("red".to_string()),
                TaffyStyle::default(),
                RenderStyle {
                    background_color: Some(Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    ..Default::default()
                },
                vec![],
            ),
            RenderObject::container(
                Some("green".to_string()),
                TaffyStyle::default(),
                RenderStyle {
                    background_color: Some(Color {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    ..Default::default()
                },
                vec![],
            ),
        ],
    );
    let mut tree = RenderObjectTree::new(root);

    {
        let root_node = tree.root_mut();
        *root_node.rect_mut() = xfw_layout::Rect {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };
    }
    if let Some(children) = tree.root_mut().children_mut() {
        if let Some(child) = children.first_mut() {
            *child.rect_mut() = xfw_layout::Rect {
                x: 10.0,
                y: 10.0,
                width: 30.0,
                height: 30.0,
            };
        }
        if let Some(child) = children.get_mut(1) {
            *child.rect_mut() = xfw_layout::Rect {
                x: 50.0,
                y: 50.0,
                width: 30.0,
                height: 30.0,
            };
        }
    }

    let mut renderer = Renderer::new(100, 100);

    let commands_full = renderer.render(&tree, None).unwrap();
    let mut pixmap = PixmapRenderer::new(100, 100).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 1.0));
    pixmap.execute(&commands_full).unwrap();
    maybe_dump_png("dirty_rect_full", &mut pixmap);

    let dirty_rect = xfw_layout::Rect {
        x: 55.0,
        y: 55.0,
        width: 20.0,
        height: 20.0,
    };
    let commands_partial = renderer.render(&tree, Some(dirty_rect)).unwrap();
    let mut pixmap2 = PixmapRenderer::new(100, 100).unwrap();
    pixmap2.clear((0.0, 0.0, 0.0, 1.0));
    pixmap2
        .execute_with_dirty_rect(&commands_partial, Some(dirty_rect), (0.0, 0.0, 0.0, 1.0))
        .unwrap();
    maybe_dump_png("dirty_rect_partial", &mut pixmap2);

    assert!(commands_partial.len() < commands_full.len());
}

#[test]
fn test_dirty_rect_with_nested_clip() {
    let root = RenderObject::container(
        Some("root".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        vec![RenderObject::container(
            Some("clipper".to_string()),
            TaffyStyle::default(),
            RenderStyle {
                background_color: Some(Color {
                    r: 0.2,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }),
                overflow: xfw_layout::OverflowBehavior::Hidden,
                ..Default::default()
            },
            vec![
                RenderObject::container(
                    Some("child1".to_string()),
                    TaffyStyle::default(),
                    RenderStyle {
                        background_color: Some(Color {
                            r: 1.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        ..Default::default()
                    },
                    vec![],
                ),
                RenderObject::container(
                    Some("child2".to_string()),
                    TaffyStyle::default(),
                    RenderStyle {
                        background_color: Some(Color {
                            r: 0.0,
                            g: 1.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        ..Default::default()
                    },
                    vec![],
                ),
            ],
        )],
    );
    let mut tree = RenderObjectTree::new(root);

    {
        let root_node = tree.root_mut();
        *root_node.rect_mut() = xfw_layout::Rect {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };
    }
    if let Some(children) = tree.root_mut().children_mut() {
        if let Some(clipper) = children.first_mut() {
            *clipper.rect_mut() = xfw_layout::Rect {
                x: 10.0,
                y: 10.0,
                width: 50.0,
                height: 50.0,
            };
            if let Some(grandchildren) = clipper.children_mut() {
                if let Some(child1) = grandchildren.first_mut() {
                    *child1.rect_mut() = xfw_layout::Rect {
                        x: 5.0,
                        y: 5.0,
                        width: 30.0,
                        height: 30.0,
                    };
                }
                if let Some(child2) = grandchildren.get_mut(1) {
                    *child2.rect_mut() = xfw_layout::Rect {
                        x: 25.0,
                        y: 25.0,
                        width: 30.0,
                        height: 30.0,
                    };
                }
            }
        }
    }

    let mut renderer = Renderer::new(100, 100);

    let commands_full = renderer.render(&tree, None).unwrap();
    let mut pixmap = PixmapRenderer::new(100, 100).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 1.0));
    pixmap.execute(&commands_full).unwrap();
    maybe_dump_png("dirty_clip_full", &mut pixmap);

    let dirty_rect = xfw_layout::Rect {
        x: 15.0,
        y: 15.0,
        width: 20.0,
        height: 20.0,
    };
    let commands_partial = renderer.render(&tree, Some(dirty_rect)).unwrap();
    let mut pixmap2 = PixmapRenderer::new(100, 100).unwrap();
    pixmap2.clear((0.0, 0.0, 0.0, 1.0));
    pixmap2
        .execute_with_dirty_rect(&commands_partial, Some(dirty_rect), (0.0, 0.0, 0.0, 1.0))
        .unwrap();
    maybe_dump_png("dirty_clip_partial", &mut pixmap2);

    let has_clip = commands_partial
        .iter()
        .any(|c| matches!(c, DrawCommand::ClipPath { .. }));
    assert!(has_clip, "Should have clip path for overflow:hidden");
}

#[test]
fn test_dirty_rect_multiple_areas() {
    let root = RenderObject::container(
        Some("root".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        vec![
            RenderObject::container(
                Some("red".to_string()),
                TaffyStyle::default(),
                RenderStyle {
                    background_color: Some(Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    ..Default::default()
                },
                vec![],
            ),
            RenderObject::container(
                Some("green".to_string()),
                TaffyStyle::default(),
                RenderStyle {
                    background_color: Some(Color {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    ..Default::default()
                },
                vec![],
            ),
            RenderObject::container(
                Some("blue".to_string()),
                TaffyStyle::default(),
                RenderStyle {
                    background_color: Some(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    ..Default::default()
                },
                vec![],
            ),
        ],
    );
    let mut tree = RenderObjectTree::new(root);

    {
        let root_node = tree.root_mut();
        *root_node.rect_mut() = xfw_layout::Rect {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };
    }
    if let Some(children) = tree.root_mut().children_mut() {
        if let Some(red) = children.first_mut() {
            *red.rect_mut() = xfw_layout::Rect {
                x: 5.0,
                y: 5.0,
                width: 20.0,
                height: 20.0,
            };
        }
        if let Some(green) = children.get_mut(1) {
            *green.rect_mut() = xfw_layout::Rect {
                x: 40.0,
                y: 40.0,
                width: 20.0,
                height: 20.0,
            };
        }
        if let Some(blue) = children.get_mut(2) {
            *blue.rect_mut() = xfw_layout::Rect {
                x: 75.0,
                y: 75.0,
                width: 20.0,
                height: 20.0,
            };
        }
    }

    let mut renderer = Renderer::new(100, 100);

    let commands_full = renderer.render(&tree, None).unwrap();
    let mut pixmap = PixmapRenderer::new(100, 100).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 1.0));
    pixmap.execute(&commands_full).unwrap();
    maybe_dump_png("dirty_multi_full", &mut pixmap);

    let dirty_rect = xfw_layout::Rect {
        x: 5.0,
        y: 5.0,
        width: 20.0,
        height: 20.0,
    };
    let commands_partial = renderer.render(&tree, Some(dirty_rect)).unwrap();
    let mut pixmap2 = PixmapRenderer::new(100, 100).unwrap();
    pixmap2.clear((0.0, 0.0, 0.0, 1.0));
    pixmap2
        .execute_with_dirty_rect(&commands_partial, Some(dirty_rect), (0.0, 0.0, 0.0, 1.0))
        .unwrap();
    maybe_dump_png("dirty_multi_partial", &mut pixmap2);

    assert!(commands_partial.len() < commands_full.len());
}

#[test]
fn test_dirty_rect_opacity_change() {
    let root = RenderObject::container(
        Some("root".to_string()),
        TaffyStyle::default(),
        RenderStyle::default(),
        vec![
            RenderObject::container(
                Some("solid".to_string()),
                TaffyStyle::default(),
                RenderStyle {
                    background_color: Some(Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    ..Default::default()
                },
                vec![],
            ),
            RenderObject::container(
                Some("transparent".to_string()),
                TaffyStyle::default(),
                RenderStyle {
                    background_color: Some(Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.5,
                    }),
                    ..Default::default()
                },
                vec![],
            ),
        ],
    );
    let mut tree = RenderObjectTree::new(root);

    {
        let root_node = tree.root_mut();
        *root_node.rect_mut() = xfw_layout::Rect {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };
    }
    if let Some(children) = tree.root_mut().children_mut() {
        if let Some(solid) = children.first_mut() {
            *solid.rect_mut() = xfw_layout::Rect {
                x: 10.0,
                y: 10.0,
                width: 30.0,
                height: 30.0,
            };
        }
        if let Some(transparent) = children.get_mut(1) {
            *transparent.rect_mut() = xfw_layout::Rect {
                x: 20.0,
                y: 20.0,
                width: 30.0,
                height: 30.0,
            };
        }
    }

    let mut renderer = Renderer::new(100, 100);

    let commands_full = renderer.render(&tree, None).unwrap();
    let mut pixmap = PixmapRenderer::new(100, 100).unwrap();
    pixmap.clear((0.0, 0.0, 0.0, 1.0));
    pixmap.execute(&commands_full).unwrap();
    maybe_dump_png("dirty_opacity_full", &mut pixmap);

    let dirty_rect = xfw_layout::Rect {
        x: 15.0,
        y: 15.0,
        width: 20.0,
        height: 20.0,
    };
    let commands_partial = renderer.render(&tree, Some(dirty_rect)).unwrap();
    let mut pixmap2 = PixmapRenderer::new(100, 100).unwrap();
    pixmap2.clear((0.0, 0.0, 0.0, 1.0));
    pixmap2
        .execute_with_dirty_rect(&commands_partial, Some(dirty_rect), (0.0, 0.0, 0.0, 1.0))
        .unwrap();
    maybe_dump_png("dirty_opacity_partial", &mut pixmap2);

    let has_transparent = commands_partial.iter().any(|c| {
        if let DrawCommand::FillRect {
            color: (_, _, _, a),
            ..
        } = c
        {
            return *a < 1.0;
        }
        false
    });
    assert!(has_transparent, "Should render transparent element");
}
