use slint::ComponentHandle;
use slint_ui_templates::AppWindow;

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    app.run()
}
