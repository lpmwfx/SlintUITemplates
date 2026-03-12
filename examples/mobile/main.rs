// examples/mobile — demonstrates the Material 3 MobileShell in a simulated Android frame.
// Run: cargo run --example mobile

use slint::ComponentHandle;
use slint_ui_templates::FrameworkViewer;

fn main() -> Result<(), slint::PlatformError> {
    let viewer = FrameworkViewer::new()?;
    // Pre-select the App Shell page showing Android layout
    viewer.set_active_view("app-shell".into());
    viewer.run()
}
