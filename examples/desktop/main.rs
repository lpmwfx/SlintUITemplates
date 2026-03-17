use slint::ComponentHandle;
use slint_ui_templates::AppWindow;

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    // Wire navigate callback — active-view is `in property`, only Rust can set it
    let weak = app.as_weak();
    app.on_navigate(move |id| {
        if let Some(handle) = weak.upgrade() { handle.set_active_view(id); }
    });
    app.run()
}
