use anyhow::{anyhow, Result};
use cosmic_text::{FontSystem, SwashCache};
use tiny_skia::{
    FilterQuality, Mask, MaskType, PathBuilder, Pixmap, PixmapPaint, Rect, Stroke, Transform,
};
use xfw_layout::{
    ImageFit, OverflowBehavior, Rect as XfwRect, RenderObject, RenderObjectTree, TextAlign,
};

/// A render instruction emitted by `Renderer` and consumed by `PixmapRenderer`.
///
/// # Examples
/// ```rust
/// use xfw_render::DrawCommand;
/// use xfw_layout::Rect;
///
/// let cmd = DrawCommand::FillRect {
///     rect: Rect { x: 0.0, y: 0.0, width: 10.0, height: 10.0 },
///     color: (1.0, 0.0, 0.0, 1.0),
///     border_radius: None,
///     opacity: 1.0,
/// };
/// ```
///
/// # Errors
/// None.
///
/// # Panics
/// None.
#[derive(Debug, Clone)]
pub enum DrawCommand {
    FillRect {
        rect: XfwRect,
        color: (f32, f32, f32, f32),
        border_radius: Option<f32>,
        opacity: f32,
    },
    StrokeRect {
        rect: XfwRect,
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
        rect: XfwRect,
        image_fit: ImageFit,
    },
    ClipPath {
        rect: XfwRect,
        border_radius: f32,
    },
    PopClip,
}

/// Renders `DrawCommand`s into a tiny-skia pixmap using CPU rasterization.
///
/// # Examples
/// ```rust
/// # use xfw_render::{DrawCommand, PixmapRenderer};
/// # use xfw_layout::Rect;
/// let mut renderer = PixmapRenderer::new(32, 32).unwrap();
/// renderer.clear((0.0, 0.0, 0.0, 0.0));
/// renderer.execute(&[DrawCommand::FillRect {
///     rect: Rect { x: 0.0, y: 0.0, width: 16.0, height: 16.0 },
///     color: (0.2, 0.3, 0.8, 1.0),
///     border_radius: None,
///     opacity: 1.0,
/// }]).unwrap();
/// ```
///
/// # Errors
/// See individual methods.
///
/// # Panics
/// None.
pub struct PixmapRenderer {
    pixmap: Pixmap,
    font_system: FontSystem,
    swash_cache: SwashCache,
    clip_stack: Vec<Mask>,
}

struct TextDrawArgs<'a> {
    text: &'a str,
    x: f32,
    y: f32,
    width: f32,
    color: (f32, f32, f32, f32),
    font_size: f32,
    font_family: Option<&'a str>,
    text_align: TextAlign,
}

impl PixmapRenderer {
    /// Creates a new pixmap renderer with a fixed-size buffer.
    ///
    /// # Examples
    /// ```rust
    /// # use xfw_render::PixmapRenderer;
    /// let renderer = PixmapRenderer::new(128, 64).unwrap();
    /// assert_eq!(renderer.width(), 128);
    /// ```
    ///
    /// # Errors
    /// Returns an error if the pixmap allocation fails.
    ///
    /// # Panics
    /// None.
    pub fn new(width: u32, height: u32) -> Result<Self> {
        let pixmap =
            Pixmap::new(width, height).ok_or_else(|| anyhow!("Failed to create pixmap"))?;
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        Ok(Self {
            pixmap,
            font_system,
            swash_cache,
            clip_stack: Vec::new(),
        })
    }

    /// Returns the pixmap width in pixels.
    ///
    /// # Examples
    /// ```rust
    /// # use xfw_render::PixmapRenderer;
    /// let renderer = PixmapRenderer::new(8, 8).unwrap();
    /// assert_eq!(renderer.width(), 8);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn width(&self) -> u32 {
        self.pixmap.width()
    }
    /// Returns the pixmap height in pixels.
    ///
    /// # Examples
    /// ```rust
    /// # use xfw_render::PixmapRenderer;
    /// let renderer = PixmapRenderer::new(8, 12).unwrap();
    /// assert_eq!(renderer.height(), 12);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn height(&self) -> u32 {
        self.pixmap.height()
    }
    /// Returns a mutable reference to the underlying pixmap.
    ///
    /// # Examples
    /// ```rust
    /// # use xfw_render::PixmapRenderer;
    /// let mut renderer = PixmapRenderer::new(4, 4).unwrap();
    /// let pixmap = renderer.pixmap_mut();
    /// assert_eq!(pixmap.width(), 4);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn pixmap_mut(&mut self) -> &mut Pixmap {
        &mut self.pixmap
    }
    /// Returns raw premultiplied RGBA data of the pixmap.
    ///
    /// # Examples
    /// ```rust
    /// # use xfw_render::PixmapRenderer;
    /// let renderer = PixmapRenderer::new(2, 2).unwrap();
    /// assert_eq!(renderer.data().len(), 2 * 2 * 4);
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn data(&self) -> &[u8] {
        self.pixmap.data()
    }

    /// Clears the pixmap to a solid color.
    ///
    /// # Examples
    /// ```rust
    /// # use xfw_render::PixmapRenderer;
    /// let mut renderer = PixmapRenderer::new(2, 2).unwrap();
    /// renderer.clear((0.0, 0.0, 0.0, 0.0));
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn clear(&mut self, color: (f32, f32, f32, f32)) {
        self.pixmap.fill(self.to_color(color));
    }

    /// Executes a list of draw commands against the pixmap.
    ///
    /// # Examples
    /// ```rust
    /// # use xfw_render::{DrawCommand, PixmapRenderer};
    /// # use xfw_layout::Rect;
    /// let mut renderer = PixmapRenderer::new(8, 8).unwrap();
    /// renderer.execute(&[DrawCommand::FillRect {
    ///     rect: Rect { x: 0.0, y: 0.0, width: 4.0, height: 4.0 },
    ///     color: (1.0, 1.0, 1.0, 1.0),
    ///     border_radius: None,
    ///     opacity: 1.0,
    /// }]).unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns an error when an image cannot be loaded or a clip mask cannot be built.
    ///
    /// # Panics
    /// None.
    pub fn execute(&mut self, commands: &[DrawCommand]) -> Result<()> {
        for cmd in commands {
            self.execute_command(cmd)?;
        }
        Ok(())
    }

    fn execute_command(&mut self, cmd: &DrawCommand) -> Result<()> {
        match cmd {
            DrawCommand::FillRect {
                rect,
                color,
                border_radius,
                opacity: _,
            } => {
                self.fill_rect(*rect, *color, *border_radius);
            }
            DrawCommand::StrokeRect {
                rect,
                color,
                width,
                border_radius,
            } => {
                self.stroke_rect(*rect, *color, *width, *border_radius);
            }
            DrawCommand::DrawText {
                text,
                x,
                y,
                width,
                color,
                font_size,
                font_family,
                text_align,
            } => {
                let args = TextDrawArgs {
                    text,
                    x: *x,
                    y: *y,
                    width: *width,
                    color: *color,
                    font_size: *font_size,
                    font_family: font_family.as_deref(),
                    text_align: *text_align,
                };
                self.draw_text(&args)?;
            }
            DrawCommand::DrawImage {
                path,
                rect,
                image_fit,
            } => {
                self.draw_image(path, *rect, *image_fit)?;
            }
            DrawCommand::ClipPath {
                rect,
                border_radius,
            } => {
                self.push_clip(*rect, *border_radius)?;
            }
            DrawCommand::PopClip => {
                self.pop_clip();
            }
        }
        Ok(())
    }

    fn fill_rect(
        &mut self,
        rect: XfwRect,
        color: (f32, f32, f32, f32),
        border_radius: Option<f32>,
    ) {
        let mut paint = tiny_skia::Paint::default();
        paint.set_color(self.to_color(color));
        paint.anti_alias = true;

        let rect = match Rect::from_ltrb(rect.x, rect.y, rect.x + rect.width, rect.y + rect.height)
        {
            Some(r) => r,
            None => return,
        };

        let mask = self.current_mask();
        if let Some(radius) = border_radius {
            if radius > 0.0 {
                if let Some(path) = rounded_rect_path(rect, radius) {
                    self.pixmap.fill_path(
                        &path,
                        &paint,
                        tiny_skia::FillRule::EvenOdd,
                        Transform::identity(),
                        mask.as_ref(),
                    );
                }
                return;
            }
        }
        self.pixmap
            .fill_rect(rect, &paint, Transform::identity(), mask.as_ref());
    }

    fn stroke_rect(
        &mut self,
        rect: XfwRect,
        color: (f32, f32, f32, f32),
        width: f32,
        border_radius: Option<f32>,
    ) {
        let mut paint = tiny_skia::Paint::default();
        paint.set_color(self.to_color(color));
        paint.anti_alias = true;

        let stroke = Stroke {
            width,
            ..Default::default()
        };

        let rect = match Rect::from_ltrb(rect.x, rect.y, rect.x + rect.width, rect.y + rect.height)
        {
            Some(r) => r,
            None => return,
        };

        let mask = self.current_mask();
        if let Some(radius) = border_radius {
            if radius > 0.0 {
                if let Some(path) = rounded_rect_path(rect, radius) {
                    self.pixmap.stroke_path(
                        &path,
                        &paint,
                        &stroke,
                        Transform::identity(),
                        mask.as_ref(),
                    );
                }
                return;
            }
        }
        let path = PathBuilder::from_rect(rect);
        self.pixmap
            .stroke_path(&path, &paint, &stroke, Transform::identity(), mask.as_ref());
    }

    fn draw_text(&mut self, args: &TextDrawArgs<'_>) -> Result<()> {
        let line_height = args.font_size * 1.2;
        let mut buffer = cosmic_text::Buffer::new(
            &mut self.font_system,
            cosmic_text::Metrics::new(args.font_size, line_height),
        );
        let family = args.font_family.unwrap_or("sans-serif");
        let metrics = cosmic_text::Metrics::new(args.font_size, line_height);
        let attrs = cosmic_text::Attrs::new()
            .family(cosmic_text::Family::Name(family))
            .metrics(metrics);
        let align = match args.text_align {
            TextAlign::Left => cosmic_text::Align::Left,
            TextAlign::Center => cosmic_text::Align::Center,
            TextAlign::Right => cosmic_text::Align::Right,
            TextAlign::Justify => cosmic_text::Align::Justified,
        };
        buffer.set_text(
            &mut self.font_system,
            args.text,
            &attrs,
            cosmic_text::Shaping::Advanced,
            Some(align),
        );
        buffer.set_size(&mut self.font_system, Some(args.width), None);

        let text_color = cosmic_text::Color::rgba(
            (args.color.0.clamp(0.0, 1.0) * 255.0) as u8,
            (args.color.1.clamp(0.0, 1.0) * 255.0) as u8,
            (args.color.2.clamp(0.0, 1.0) * 255.0) as u8,
            (args.color.3.clamp(0.0, 1.0) * 255.0) as u8,
        );

        let pixmap_width = self.pixmap.width() as i32;
        let pixmap_height = self.pixmap.height() as i32;
        let row_stride = self.pixmap.width() as usize;
        let origin_x = args.x as i32;
        let origin_y = (args.y + args.font_size).round() as i32;
        let mask = self.current_mask();
        let (mask_data, mask_width) = match mask.as_ref() {
            Some(mask) => (Some(mask.data()), mask.width() as usize),
            None => (None, 0),
        };
        let pixels = self.pixmap.pixels_mut();

        buffer.draw(
            &mut self.font_system,
            &mut self.swash_cache,
            text_color,
            |gx, gy, w, h, color| {
                if w == 0 || h == 0 {
                    return;
                }
                let gx = gx + origin_x;
                let gy = gy + origin_y;
                let start_x = gx.max(0);
                let start_y = gy.max(0);
                let end_x = (gx + w as i32).min(pixmap_width);
                let end_y = (gy + h as i32).min(pixmap_height);
                if start_x >= end_x || start_y >= end_y {
                    return;
                }

                for yy in start_y..end_y {
                    let row = yy as usize * row_stride;
                    let mask_row = yy as usize * mask_width;
                    for xx in start_x..end_x {
                        let alpha = match mask_data {
                            Some(mask) => mask[mask_row + xx as usize],
                            None => 255,
                        };
                        if alpha == 0 {
                            continue;
                        }
                        let final_alpha = ((u16::from(color.a()) * u16::from(alpha)) / 255) as u8;
                        let src = tiny_skia::ColorU8::from_rgba(
                            color.r(),
                            color.g(),
                            color.b(),
                            final_alpha,
                        )
                        .premultiply();
                        let dst = pixels[row + xx as usize];
                        pixels[row + xx as usize] = blend_premultiplied(src, dst);
                    }
                }
            },
        );
        Ok(())
    }

    fn draw_image(&mut self, _path: &str, _rect: XfwRect, _image_fit: ImageFit) -> Result<()> {
        if _rect.width <= 0.0 || _rect.height <= 0.0 {
            return Ok(());
        }

        let image = Pixmap::load_png(_path).map_err(|e| {
            anyhow!(
                "Failed to load image {path}: {error}",
                path = _path,
                error = e
            )
        })?;
        let src_w = image.width() as f32;
        let src_h = image.height() as f32;
        if src_w <= 0.0 || src_h <= 0.0 {
            return Ok(());
        }

        let dest_w = _rect.width;
        let dest_h = _rect.height;

        let (scale_x, scale_y, origin_x, origin_y) = match _image_fit {
            ImageFit::Fill => (dest_w / src_w, dest_h / src_h, _rect.x, _rect.y),
            ImageFit::Contain => {
                let scale = (dest_w / src_w).min(dest_h / src_h);
                let draw_w = src_w * scale;
                let draw_h = src_h * scale;
                (
                    scale,
                    scale,
                    _rect.x + (dest_w - draw_w) * 0.5,
                    _rect.y + (dest_h - draw_h) * 0.5,
                )
            }
            ImageFit::Cover => {
                let scale = (dest_w / src_w).max(dest_h / src_h);
                let draw_w = src_w * scale;
                let draw_h = src_h * scale;
                (
                    scale,
                    scale,
                    _rect.x + (dest_w - draw_w) * 0.5,
                    _rect.y + (dest_h - draw_h) * 0.5,
                )
            }
            ImageFit::FitWidth => {
                let scale = dest_w / src_w;
                let draw_h = src_h * scale;
                (scale, scale, _rect.x, _rect.y + (dest_h - draw_h) * 0.5)
            }
            ImageFit::FitHeight => {
                let scale = dest_h / src_h;
                let draw_w = src_w * scale;
                (scale, scale, _rect.x + (dest_w - draw_w) * 0.5, _rect.y)
            }
            ImageFit::None => (1.0, 1.0, _rect.x, _rect.y),
        };

        let transform = Transform::from_scale(scale_x, scale_y).post_translate(origin_x, origin_y);
        let paint = PixmapPaint {
            quality: FilterQuality::Bilinear,
            ..Default::default()
        };

        let mask = self.current_mask();
        self.pixmap
            .as_mut()
            .draw_pixmap(0, 0, image.as_ref(), &paint, transform, mask.as_ref());
        Ok(())
    }

    fn push_clip(&mut self, rect: XfwRect, border_radius: f32) -> Result<()> {
        let clip = match Rect::from_ltrb(rect.x, rect.y, rect.x + rect.width, rect.y + rect.height)
        {
            Some(r) => r,
            None => return Ok(()),
        };
        let path = match rounded_rect_path(clip, border_radius) {
            Some(p) => p,
            None => return Ok(()),
        };
        let mut mask_pixmap = Pixmap::new(self.pixmap.width(), self.pixmap.height())
            .ok_or_else(|| anyhow!("Failed to create clip mask pixmap"))?;
        let mut paint = tiny_skia::Paint::default();
        paint.set_color(
            tiny_skia::Color::from_rgba(1.0, 1.0, 1.0, 1.0).unwrap_or(tiny_skia::Color::WHITE),
        );
        paint.anti_alias = true;
        mask_pixmap.fill(tiny_skia::Color::TRANSPARENT);
        mask_pixmap.fill_path(
            &path,
            &paint,
            tiny_skia::FillRule::EvenOdd,
            Transform::identity(),
            None,
        );

        let mut next_mask = Mask::from_pixmap(mask_pixmap.as_ref(), MaskType::Alpha);
        if let Some(current) = self.clip_stack.last() {
            intersect_mask(current, &mut next_mask);
        }
        self.clip_stack.push(next_mask);
        Ok(())
    }

    fn pop_clip(&mut self) {
        self.clip_stack.pop();
    }

    fn current_mask(&self) -> Option<Mask> {
        self.clip_stack.last().cloned()
    }

    fn to_color(&self, color: (f32, f32, f32, f32)) -> tiny_skia::Color {
        tiny_skia::Color::from_rgba(
            color.0.clamp(0.0, 1.0),
            color.1.clamp(0.0, 1.0),
            color.2.clamp(0.0, 1.0),
            color.3.clamp(0.0, 1.0),
        )
        .unwrap_or(tiny_skia::Color::BLACK)
    }
}

fn blend_premultiplied(
    src: tiny_skia::PremultipliedColorU8,
    dst: tiny_skia::PremultipliedColorU8,
) -> tiny_skia::PremultipliedColorU8 {
    let sa = src.alpha() as u16;
    let inv_sa = 255u16.saturating_sub(sa);

    let r = src.red() as u16 + (dst.red() as u16 * inv_sa + 127) / 255;
    let g = src.green() as u16 + (dst.green() as u16 * inv_sa + 127) / 255;
    let b = src.blue() as u16 + (dst.blue() as u16 * inv_sa + 127) / 255;
    let a = src.alpha() as u16 + (dst.alpha() as u16 * inv_sa + 127) / 255;

    tiny_skia::PremultipliedColorU8::from_rgba(
        r.min(255) as u8,
        g.min(255) as u8,
        b.min(255) as u8,
        a.min(255) as u8,
    )
    .unwrap_or(dst)
}

fn intersect_mask(base: &Mask, next: &mut Mask) {
    let base_data = base.data();
    let next_data = next.data_mut();
    let len = base_data.len().min(next_data.len());
    for i in 0..len {
        next_data[i] = next_data[i].min(base_data[i]);
    }
}

fn rounded_rect_path(rect: Rect, radius: f32) -> Option<tiny_skia::Path> {
    let r = radius.min(rect.width() / 2.0).min(rect.height() / 2.0);
    if r <= 0.0 {
        return Some(PathBuilder::from_rect(rect));
    }

    let k = r * 0.552_284_8;
    let left = rect.left();
    let right = rect.right();
    let top = rect.top();
    let bottom = rect.bottom();

    let mut pb = PathBuilder::new();
    pb.move_to(left + r, top);
    pb.line_to(right - r, top);
    pb.cubic_to(right - r + k, top, right, top + r - k, right, top + r);
    pb.line_to(right, bottom - r);
    pb.cubic_to(
        right,
        bottom - r + k,
        right - r + k,
        bottom,
        right - r,
        bottom,
    );
    pb.line_to(left + r, bottom);
    pb.cubic_to(left + r - k, bottom, left, bottom - r + k, left, bottom - r);
    pb.line_to(left, top + r);
    pb.cubic_to(left, top + r - k, left + r - k, top, left + r, top);
    pb.close();
    pb.finish()
}

/// Builds draw commands from a `RenderObjectTree`.
///
/// # Examples
/// ```rust
/// # use xfw_render::Renderer;
/// let renderer = Renderer::new(1920, 1080);
/// let (width, height) = renderer.size();
/// assert_eq!((width, height), (1920, 1080));
/// ```
///
/// # Errors
/// None.
///
/// # Panics
/// None.
pub struct Renderer {
    #[allow(dead_code)]
    width: u32,
    #[allow(dead_code)]
    height: u32,
}

impl Renderer {
    /// Creates a new renderer with a fixed output size.
    ///
    /// # Examples
    /// ```rust
    /// # use xfw_render::Renderer;
    /// let renderer = Renderer::new(128, 64);
    /// assert_eq!(renderer.size(), (128, 64));
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Creates a renderer using the default size (1920x1080).
    ///
    /// # Examples
    /// ```rust
    /// # use xfw_render::Renderer;
    /// let renderer = Renderer::with_default_size();
    /// assert_eq!(renderer.size(), (1920, 1080));
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn with_default_size() -> Self {
        Self::new(1920, 1080)
    }

    /// Returns the configured renderer size.
    ///
    /// # Examples
    /// ```rust
    /// # use xfw_render::Renderer;
    /// let renderer = Renderer::new(10, 20);
    /// assert_eq!(renderer.size(), (10, 20));
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Prepares the renderer for a rendering pass.
    ///
    /// # Examples
    /// ```rust
    /// # use xfw_render::Renderer;
    /// let mut renderer = Renderer::new(1, 1);
    /// renderer.prepare().unwrap();
    /// ```
    ///
    /// # Errors
    /// None.
    ///
    /// # Panics
    /// None.
    pub fn prepare(&mut self) -> Result<()> {
        Ok(())
    }

    /// Converts a render tree into draw commands.
    ///
    /// # Examples
    /// ```rust
    /// # use xfw_render::Renderer;
    /// # use xfw_layout::{RenderObject, RenderObjectTree, Rect};
    /// # use taffy::Style as TaffyStyle;
    /// let root = RenderObject::container(None, TaffyStyle::default(), Default::default(), vec![]);
    /// let tree = RenderObjectTree::new(root);
    /// let mut renderer = Renderer::new(1, 1);
    /// let commands = renderer.render(&tree, None).unwrap();
    /// assert!(commands.is_empty());
    /// ```
    ///
    /// # Errors
    /// Returns an error if rendering fails.
    ///
    /// # Panics
    /// None.
    pub fn render(
        &mut self,
        tree: &RenderObjectTree,
        _dirty_rect: Option<XfwRect>,
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

                let mut pushed_clip = false;
                let overflow_hidden = render_style.overflow == OverflowBehavior::Hidden;
                if overflow_hidden {
                    commands.push(DrawCommand::ClipPath {
                        rect: *rect,
                        border_radius: border_radius.unwrap_or(0.0),
                    });
                    pushed_clip = true;
                } else if let Some(radius) = border_radius {
                    if radius > 0.0
                        && render_style.background_color.is_none()
                        && render_style.border_color.is_none()
                    {
                        commands.push(DrawCommand::ClipPath {
                            rect: *rect,
                            border_radius: radius,
                        });
                        pushed_clip = true;
                    }
                }

                for child in children {
                    self.process_node(child, commands);
                }

                if pushed_clip {
                    commands.push(DrawCommand::PopClip);
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

                if std::env::var_os("XFW_RENDER_DEBUG_BOUNDS").is_some() {
                    commands.push(DrawCommand::StrokeRect {
                        rect: *rect,
                        color: (1.0, 0.0, 1.0, 1.0),
                        width: 1.0,
                        border_radius: None,
                    });
                }
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
