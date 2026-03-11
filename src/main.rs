mod grid;

slint::include_modules!();

fn is_dark_mode() -> bool {
    std::process::Command::new("reg")
        .args([
            "query",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            "/v",
            "AppsUseLightTheme",
        ])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.contains("0x0"))
        .unwrap_or(false)
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // Theme
    ui.global::<Colors>().set_dark_mode(is_dark_mode());

    // Grid engine: load config → build zones → push ratios to Mother
    let zones = grid::load_target(std::path::Path::new("config/desktop.toml"))
        .expect("Failed to load desktop config");

    for row in &zones.rows {
        match row.name.as_str() {
            "top" => ui.set_row_top_ratio(row.ratio as f32),
            "main" => {
                ui.set_row_main_ratio(row.ratio as f32);
                if let grid::zone::RowKind::Columns(cols) = &row.kind {
                    for col in cols {
                        match col.name.as_str() {
                            "left" => ui.set_col_left_ratio(col.ratio as f32),
                            "center" => ui.set_col_center_ratio(col.ratio as f32),
                            "right" => ui.set_col_right_ratio(col.ratio as f32),
                            _ => {}
                        }
                    }
                }
            }
            "bottom" => ui.set_row_bottom_ratio(row.ratio as f32),
            _ => {}
        }
    }

    ui.run()
}
