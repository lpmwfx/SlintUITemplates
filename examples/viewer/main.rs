use slint::ComponentHandle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = slint_ui_templates::FrameworkViewer::new()?;
    ui.run()?;
    Ok(())
}
