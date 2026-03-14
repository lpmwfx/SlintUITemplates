use slint::ComponentHandle;
use slint_ui_templates::{dsl::BgStyle, platform};

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
const ALPHA_MAX: u32 = 255;
const DEMO_FRAME_WIDTH: u32 = 300;
const DEMO_FRAME_HEIGHT: u32 = 200;

fn render_demo_frame(width: u32, height: u32) -> Result<slint::Image, Box<dyn std::error::Error>> {
    use tiny_skia::*;

    let mut pixmap = Pixmap::new(width, height).ok_or("pixmap allocation failed")?;

    // Background fill
    pixmap.fill(Color::from_rgba8(BG_COLOR_R, BG_COLOR_G, BG_COLOR_B, BG_COLOR_A));

    // Circle
    let mut paint = Paint::default();
    paint.set_color_rgba8(CIRCLE_COLOR_R, CIRCLE_COLOR_G, CIRCLE_COLOR_B, CIRCLE_COLOR_A);
    paint.anti_alias = true;

    let path = PathBuilder::from_circle(CIRCLE_CENTER_X, CIRCLE_CENTER_Y, CIRCLE_RADIUS)
        .ok_or("circle path failed")?;
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    // Stroked rectangle
    let mut stroke_paint = Paint::default();
    stroke_paint.set_color_rgba8(RECT_COLOR_R, RECT_COLOR_G, RECT_COLOR_B, RECT_COLOR_A);
    stroke_paint.anti_alias = true;

    let rect_path = {
        let mut pb = PathBuilder::new();
        pb.move_to(RECT_TOP_LEFT_X, RECT_TOP_LEFT_Y);
        pb.line_to(RECT_TOP_RIGHT_X, RECT_TOP_RIGHT_Y);
        pb.line_to(RECT_BOTTOM_RIGHT_X, RECT_BOTTOM_RIGHT_Y);
        pb.line_to(RECT_BOTTOM_LEFT_X, RECT_BOTTOM_LEFT_Y);
        pb.close();
        pb.finish().ok_or("rect path failed")?
    };
    let stroke = Stroke { width: RECT_STROKE_WIDTH, ..Default::default() };
    pixmap.stroke_path(&rect_path, &stroke_paint, &stroke, Transform::identity(), None);

    // Diagonal line
    let mut line_paint = Paint::default();
    line_paint.set_color_rgba8(LINE_COLOR_R, LINE_COLOR_G, LINE_COLOR_B, LINE_COLOR_A);
    line_paint.anti_alias = true;

    let line_path = {
        let mut pb = PathBuilder::new();
        pb.move_to(LINE_START_X, LINE_START_Y);
        pb.line_to(LINE_END_X, LINE_END_Y);
        pb.finish().ok_or("line path failed")?
    };
    let line_stroke = Stroke { width: LINE_STROKE_WIDTH, ..Default::default() };
    pixmap.stroke_path(&line_path, &line_paint, &line_stroke, Transform::identity(), None);

    // Convert pixmap → slint::Image via SharedPixelBuffer
    let pixel_data = pixmap.data();
    let mut buf = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::new(width, height);
    let pixels = buf.make_mut_slice();
    for (i, px) in pixels.iter_mut().enumerate() {
        let base = i * PIXEL_STRIDE;
        // tiny-skia premultiplies alpha — demultiply for slint
        let a = pixel_data[base + 3];
        let (r, g, b) = if a == 0 {
            (0, 0, 0)
        } else {
            (
                ((pixel_data[base]     as u32 * ALPHA_MAX) / a as u32) as u8,
                ((pixel_data[base + 1] as u32 * ALPHA_MAX) / a as u32) as u8,
                ((pixel_data[base + 2] as u32 * ALPHA_MAX) / a as u32) as u8,
            )
        };
        *px = slint::Rgba8Pixel { r, g, b, a };
    }
    Ok(slint::Image::from_rgba8(buf))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = slint_ui_templates::FrameworkViewer::new()?;

    // Wire canvas frame — rendered once at startup
    ui.set_canvas_frame(render_demo_frame(300, 200)?);

    // Wire Solid/Mica/Acrylic buttons → OS backdrop + Theme.material
    let weak = ui.as_weak();
    ui.on_request_bg_style(move |style_name| {
        if let Some(handle) = weak.upgrade() {
            let bg = match style_name.as_str() {
                "mica"    => BgStyle::Mica,
                "acrylic" => BgStyle::Acrylic,
                _         => BgStyle::Solid,
            };
            platform::apply_backdrop(handle.window(), bg);
            handle.global::<slint_ui_templates::Theme>().set_material(style_name);
        }
    });

    ui.run()?;
    Ok(())
}
