slint::include_modules!();

mod layout;

use layout::build;

fn main() {
    let app = AppWindow::new().unwrap();

    // Initial layout — "1:2/1:1:1" = left | center-top/center-bottom(1:1) | right
    let dsl = "1:2/1:1:1";
    let win_w = 1280.0_f32;
    let win_h = 800.0_f32;

    push_layout(&app, dsl, win_w, win_h);

    // Handle drag events — Rust recalculates layout on resize
    let app_weak = app.as_weak();
    app.on_panel_dragged(move |id, dx, dy| {
        let app = app_weak.unwrap();
        // TODO phase 4: update ratio for `id` by delta (dx/win_w or dy/win_h)
        // For now: log the event
        let _ = (id, dx, dy);
        let _ = app;
    });

    app.run().unwrap();
}

fn push_layout(app: &AppWindow, dsl: &str, win_w: f32, win_h: f32) {
    let items = build(dsl, win_w, win_h);

    let model: Vec<PanelItem> = items.iter().map(|i| PanelItem {
        id:    i.id,
        kind:  i.kind.as_str().into(),
        label: i.label.clone().into(),
        x: i.x, y: i.y, w: i.w, h: i.h,
    }).collect();

    app.set_panels(std::rc::Rc::new(slint::VecModel::from(model)).into());
}
