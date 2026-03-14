mod gateway;
mod pages;

use slint_ui_templates::{platform, dsl::BgStyle};
use slint::ComponentHandle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = pages::build()?;
    ui.show()?;
    platform::apply_backdrop(ui.window(), BgStyle::Solid);
    ui.run()?;
    Ok(())
}
