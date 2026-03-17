mod demo_frame;

use slint::ComponentHandle;
use slint_ui_templates::{dsl::BgStyle, pal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = slint_ui_templates::FrameworkViewer::new()?;

    // Wire navigate callback — active-view is `in property`, only Rust can set it
    let nav_ui = ui.clone_strong();
    ui.on_navigate(move |id| { nav_ui.set_active_view(id); });

    // Wire canvas frame — rendered once at startup
    ui.set_canvas_frame(demo_frame::render(demo_frame::DEMO_FRAME_WIDTH, demo_frame::DEMO_FRAME_HEIGHT)?);

    // Wire Solid/Mica/Acrylic buttons → OS backdrop + Theme.material
    let weak = ui.as_weak();
    ui.on_request_bg_style(move |style_name| {
        if let Some(handle) = weak.upgrade() {
            let bg = BgStyle::from_str(style_name.as_str());
            pal::apply_backdrop(handle.window(), bg);
            handle.global::<slint_ui_templates::Theme>().set_material(style_name);
        }
    });

    ui.run()?;
    Ok(())
}
