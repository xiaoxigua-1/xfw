use anyhow::Result;
use xfw_layout::{ImageFit, Rect, RenderObject, RenderObjectTree, TextAlign};

#[derive(Debug, Clone)]
pub enum DrawCommand {
    FillRect {
        rect: Rect,
        color: (f32, f32, f32, f32),
        border_radius: Option<f32>,
        opacity: f32,
    },
    StrokeRect {
        rect: Rect,
        color: (f32, f32, f32, f32),
        width: f32,
        border_radius: Option<f32>,
    },
    DrawText {
        text: String,
        x: f32,
        y: f32,
        width: f32,
        color: (f32, f32, f32, f32),
        font_size: f32,
        font_family: Option<String>,
        text_align: TextAlign,
    },
    DrawImage {
        path: String,
        rect: Rect,
        image_fit: ImageFit,
    },
    ClipPath {
        rect: Rect,
        border_radius: f32,
    },
}

pub struct Renderer {
    #[allow(dead_code)]
    width: u32,
    #[allow(dead_code)]
    height: u32,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn with_default_size() -> Self {
        Self::new(1920, 1080)
    }

    pub fn prepare(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn render(
        &mut self,
        tree: &RenderObjectTree,
        _dirty_rect: Option<Rect>,
    ) -> Result<Vec<DrawCommand>> {
        let mut commands = Vec::new();
        self.process_node(tree.root(), &mut commands);
        Ok(commands)
    }

    #[allow(clippy::only_used_in_recursion)]
    fn process_node(&self, node: &RenderObject, commands: &mut Vec<DrawCommand>) {
        let rect = node.rect();
        if rect.width <= 0.0 || rect.height <= 0.0 {
            return;
        }

        let render_style = node.render_style();
        let opacity = render_style.opacity.unwrap_or(1.0);

        match node {
            RenderObject::Container { children, .. } => {
                let border_radius = render_style.border_radius;

                if let Some(bg) = render_style.background_color {
                    let final_opacity = if opacity < 1.0 || bg.a < 1.0 {
                        bg.a * opacity
                    } else {
                        bg.a
                    };
                    commands.push(DrawCommand::FillRect {
                        rect: *rect,
                        color: (bg.r, bg.g, bg.b, final_opacity),
                        border_radius,
                        opacity: 1.0,
                    });
                }

                if let Some(border_color) = render_style.border_color {
                    let border_width = render_style.border_width.unwrap_or(1.0);
                    commands.push(DrawCommand::StrokeRect {
                        rect: *rect,
                        color: (
                            border_color.r,
                            border_color.g,
                            border_color.b,
                            border_color.a * opacity,
                        ),
                        width: border_width,
                        border_radius,
                    });
                }

                if border_radius.is_some()
                    && render_style.background_color.is_none()
                    && render_style.border_color.is_none()
                {
                    commands.push(DrawCommand::ClipPath {
                        rect: *rect,
                        border_radius: border_radius.unwrap_or(0.0),
                    });
                }

                for child in children {
                    self.process_node(child, commands);
                }
            }
            RenderObject::Text { content, .. } => {
                let color = render_style
                    .color
                    .map(|c| (c.r, c.g, c.b, c.a * opacity))
                    .unwrap_or((0.0, 0.0, 0.0, opacity));
                let font_size = render_style.font_size.unwrap_or(14.0);
                let font_family = render_style.font_family.clone();
                let text_align = render_style.text_align.unwrap_or(TextAlign::Left);

                commands.push(DrawCommand::DrawText {
                    text: content.clone(),
                    x: rect.x,
                    y: rect.y,
                    width: rect.width,
                    color,
                    font_size,
                    font_family,
                    text_align,
                });
            }
            RenderObject::Image { path, .. } => {
                let image_fit = render_style.image_fit.unwrap_or(ImageFit::Fill);
                commands.push(DrawCommand::DrawImage {
                    path: path.clone(),
                    rect: *rect,
                    image_fit,
                });
            }
        }
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new(1920, 1080)
    }
}
