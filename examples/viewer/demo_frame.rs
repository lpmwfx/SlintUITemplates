// Demo frame rendering constants
const BG_COLOR_R: u8 = 30;
const BG_COLOR_G: u8 = 30;
const BG_COLOR_B: u8 = 46;
const BG_COLOR_A: u8 = 255;
const CIRCLE_COLOR_R: u8 = 137;
const CIRCLE_COLOR_G: u8 = 180;
const CIRCLE_COLOR_B: u8 = 250;
const CIRCLE_COLOR_A: u8 = 200;
const CIRCLE_CENTER_X: f32 = 150.0;
const CIRCLE_CENTER_Y: f32 = 100.0;
const CIRCLE_RADIUS: f32 = 70.0;
const RECT_COLOR_R: u8 = 166;
const RECT_COLOR_G: u8 = 227;
const RECT_COLOR_B: u8 = 161;
const RECT_COLOR_A: u8 = 220;
const RECT_TOP_LEFT_X: f32 = 220.0;
const RECT_TOP_LEFT_Y: f32 = 40.0;
const RECT_TOP_RIGHT_X: f32 = 280.0;
const RECT_TOP_RIGHT_Y: f32 = 40.0;
const RECT_BOTTOM_RIGHT_X: f32 = 280.0;
const RECT_BOTTOM_RIGHT_Y: f32 = 160.0;
const RECT_BOTTOM_LEFT_X: f32 = 220.0;
const RECT_BOTTOM_LEFT_Y: f32 = 160.0;
const RECT_STROKE_WIDTH: f32 = 3.0;
const LINE_COLOR_R: u8 = 243;
const LINE_COLOR_G: u8 = 139;
const LINE_COLOR_B: u8 = 168;
const LINE_COLOR_A: u8 = 200;
const LINE_START_X: f32 = 10.0;
const LINE_START_Y: f32 = 10.0;
const LINE_END_X: f32 = 290.0;
const LINE_END_Y: f32 = 190.0;
const LINE_STROKE_WIDTH: f32 = 2.0;
const PIXEL_STRIDE: usize = 4;
const RGBA_G_OFFSET: usize = 1;
const RGBA_B_OFFSET: usize = 2;
const RGBA_A_OFFSET: usize = 3;
const ALPHA_MAX: u32 = 255;

/// Width of the demo canvas frame in pixels.
pub const DEMO_FRAME_WIDTH: u32 = 300;
/// Height of the demo canvas frame in pixels.
pub const DEMO_FRAME_HEIGHT: u32 = 200;

/// Stateful canvas builder — drawing ops are methods so render() stays lean.
struct DemoRenderer {
    pixmap: tiny_skia::Pixmap,
    width:  u32,
    height: u32,
}

impl DemoRenderer {
    fn new(width: u32, height: u32) -> Result<Self, Box<dyn std::error::Error>> {
        use tiny_skia::*;
        let mut pixmap = Pixmap::new(width, height).ok_or("pixmap allocation failed")?;
        pixmap.fill(Color::from_rgba8(BG_COLOR_R, BG_COLOR_G, BG_COLOR_B, BG_COLOR_A));
        Ok(Self { pixmap, width, height })
    }

    fn draw_circle(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        use tiny_skia::*;
        let mut paint = Paint::default();
        paint.set_color_rgba8(CIRCLE_COLOR_R, CIRCLE_COLOR_G, CIRCLE_COLOR_B, CIRCLE_COLOR_A);
        paint.anti_alias = true;
        let path = PathBuilder::from_circle(CIRCLE_CENTER_X, CIRCLE_CENTER_Y, CIRCLE_RADIUS)
            .ok_or("circle path failed")?;
        self.pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
        Ok(())
    }

    fn draw_rect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        use tiny_skia::*;
        let mut paint = Paint::default();
        paint.set_color_rgba8(RECT_COLOR_R, RECT_COLOR_G, RECT_COLOR_B, RECT_COLOR_A);
        paint.anti_alias = true;
        let path = {
            let mut pb = PathBuilder::new();
            pb.move_to(RECT_TOP_LEFT_X,     RECT_TOP_LEFT_Y);
            pb.line_to(RECT_TOP_RIGHT_X,    RECT_TOP_RIGHT_Y);
            pb.line_to(RECT_BOTTOM_RIGHT_X, RECT_BOTTOM_RIGHT_Y);
            pb.line_to(RECT_BOTTOM_LEFT_X,  RECT_BOTTOM_LEFT_Y);
            pb.close();
            pb.finish().ok_or("rect path failed")?
        };
        let stroke = Stroke { width: RECT_STROKE_WIDTH, ..Default::default() };
        self.pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        Ok(())
    }

    fn draw_line(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        use tiny_skia::*;
        let mut paint = Paint::default();
        paint.set_color_rgba8(LINE_COLOR_R, LINE_COLOR_G, LINE_COLOR_B, LINE_COLOR_A);
        paint.anti_alias = true;
        let path = {
            let mut pb = PathBuilder::new();
            pb.move_to(LINE_START_X, LINE_START_Y);
            pb.line_to(LINE_END_X,   LINE_END_Y);
            pb.finish().ok_or("line path failed")?
        };
        let stroke = Stroke { width: LINE_STROKE_WIDTH, ..Default::default() };
        self.pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        Ok(())
    }

    fn into_image(self) -> slint::Image {
        let pixel_data = self.pixmap.data().to_vec();
        let mut buf = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::new(self.width, self.height);
        let pixels = buf.make_mut_slice();
        for (i, px) in pixels.iter_mut().enumerate() {
            let base = i * PIXEL_STRIDE;
            let a = pixel_data[base + RGBA_A_OFFSET];
            let (r, g, b) = if a == 0 {
                (0, 0, 0)
            } else {
                (
                    ((pixel_data[base]                 as u32 * ALPHA_MAX) / a as u32) as u8,
                    ((pixel_data[base + RGBA_G_OFFSET] as u32 * ALPHA_MAX) / a as u32) as u8,
                    ((pixel_data[base + RGBA_B_OFFSET] as u32 * ALPHA_MAX) / a as u32) as u8,
                )
            };
            *px = slint::Rgba8Pixel { r, g, b, a };
        }
        slint::Image::from_rgba8(buf)
    }
}

/// Render a demo canvas frame with a circle, stroked rectangle, and diagonal line.
pub fn render(width: u32, height: u32) -> Result<slint::Image, Box<dyn std::error::Error>> {
    let mut r = DemoRenderer::new(width, height)?;
    r.draw_circle()?;
    r.draw_rect()?;
    r.draw_line()?;
    Ok(r.into_image())
}
