use slint_ui_templates::AppAdapter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = AppAdapter::new()?;
    app.apply_theme();
    app.apply_grid(std::path::Path::new("config/desktop.toml"))?;
    app.run()?;
    Ok(())
}
