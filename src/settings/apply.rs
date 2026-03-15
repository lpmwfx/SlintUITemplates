use slint::ComponentHandle;
use crate::{AppWindow, Theme, Settings as UiSettings};
use crate::pal::is_dark_mode;
use super::{AppSettings, ThemeMode};

/// Expected length of a hex color string (without '#' prefix).
const HEX_COLOR_LEN: usize = 6;
/// Radix for parsing hex color channel values.
const HEX_RADIX: u32 = 16;

/// Push all settings to the Slint globals of the given AppWindow.
/// Called by AppAdapter::apply_settings().
pub fn apply(ui: &AppWindow, settings: &AppSettings) {
    // --- Theme ---
    let dark = match settings.theme.mode {
        ThemeMode::System => is_dark_mode(),
        ThemeMode::Dark   => true,
        ThemeMode::Light  => false,
    };
    ui.global::<Theme>().set_dark(dark);

    if let Some(hex) = &settings.theme.accent {
        if let Some(color) = hex_to_color(hex) {
            ui.global::<Theme>().set_accent_override(color);
        }
    }

    // --- Settings global (zoom, icon, font) ---
    let s = ui.global::<UiSettings>();
    s.set_zoom(settings.zoom.scale);
    s.set_icon_style(settings.icons.style_str().into());
    s.set_icon_color(settings.icons.color.as_str().into());
    s.set_font_family(settings.font.family.as_deref().unwrap_or("").into());
    s.set_font_scale(settings.font.font_scale);
}

fn hex_to_color(s: &str) -> Option<slint::Color> {
    let s = s.trim_start_matches('#');
    if s.len() == HEX_COLOR_LEN {
        let r = u8::from_str_radix(&s[0..2], HEX_RADIX).ok()?;
        let g = u8::from_str_radix(&s[2..4], HEX_RADIX).ok()?;
        let b = u8::from_str_radix(&s[4..6], HEX_RADIX).ok()?;
        Some(slint::Color::from_rgb_u8(r, g, b))
    } else {
        None
    }
}
