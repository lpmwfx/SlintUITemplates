use slint_ui_templates::adapter::AppAdapter;

fn main() -> Result<(), slint::PlatformError> {
    let app = AppAdapter::new()?;
    app.apply_theme();
    app.apply_grid(std::path::Path::new("config/desktop.toml"))
       .expect("grid config");
    app.run()
}
