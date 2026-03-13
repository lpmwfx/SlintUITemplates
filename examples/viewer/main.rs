use slint::ComponentHandle;
use slint_ui_templates::{dsl::BgStyle, platform};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = slint_ui_templates::FrameworkViewer::new()?;

    // Wire Solid/Mica/Acrylic buttons → OS backdrop + Theme.material
    // Theme.material switches every widget's bg-* to the correct WinUI3 layer-fill
    // alphas for the active material (Mica vs Acrylic use different fills).
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
