use slint::ComponentHandle;
use slint_ui_templates::{dsl::BgStyle, platform};

fn render_demo_frame(width: u32, height: u32) -> Result<slint::Image, Box<dyn std::error::Error>> {
    use tiny_skia::*;

    let mut pixmap = Pixmap::new(width, height).ok_or("pixmap allocation failed")?;

    // Background fill
    pixmap.fill(Color::from_rgba8(30, 30, 46, 255));

    // Circle
    let mut paint = Paint::default();
    paint.set_color_rgba8(137, 180, 250, 200);
    paint.anti_alias = true;

    let path = PathBuilder::from_circle(150.0, 100.0, 70.0)
        .ok_or("circle path failed")?;
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    // Stroked rectangle
    let mut stroke_paint = Paint::default();
    stroke_paint.set_color_rgba8(166, 227, 161, 220);
    stroke_paint.anti_alias = true;

    let rect_path = {
        let mut pb = PathBuilder::new();
        pb.move_to(220.0, 40.0);
        pb.line_to(280.0, 40.0);
        pb.line_to(280.0, 160.0);
        pb.line_to(220.0, 160.0);
        pb.close();
        pb.finish().ok_or("rect path failed")?
    };
    let stroke = Stroke { width: 3.0, ..Default::default() };
    pixmap.stroke_path(&rect_path, &stroke_paint, &stroke, Transform::identity(), None);

    // Diagonal line
    let mut line_paint = Paint::default();
    line_paint.set_color_rgba8(243, 139, 168, 200);
    line_paint.anti_alias = true;

    let line_path = {
        let mut pb = PathBuilder::new();
        pb.move_to(10.0, 10.0);
        pb.line_to(290.0, 190.0);
        pb.finish().ok_or("line path failed")?
    };
    let line_stroke = Stroke { width: 2.0, ..Default::default() };
    pixmap.stroke_path(&line_path, &line_paint, &line_stroke, Transform::identity(), None);

    // Convert pixmap → slint::Image via SharedPixelBuffer
    let data = pixmap.data();
    let mut buf = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::new(width, height);
    let pixels = buf.make_mut_slice();
    for (i, px) in pixels.iter_mut().enumerate() {
        let base = i * 4;
        // tiny-skia premultiplies alpha — demultiply for slint
        let a = data[base + 3];
        let (r, g, b) = if a == 0 {
            (0, 0, 0)
        } else {
            (
                ((data[base]     as u32 * 255) / a as u32) as u8,
                ((data[base + 1] as u32 * 255) / a as u32) as u8,
                ((data[base + 2] as u32 * 255) / a as u32) as u8,
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
    ui.on_request_bg_style(move |style| {
        if let Some(handle) = weak.upgrade() {
            let bg = match style.as_str() {
                "mica"    => BgStyle::Mica,
                "acrylic" => BgStyle::Acrylic,
                _         => BgStyle::Solid,
            };
            platform::apply_backdrop(handle.window(), bg);
            handle.global::<slint_ui_templates::Theme>().set_material(style);
        }
    });

    ui.run()?;
    Ok(())
}
