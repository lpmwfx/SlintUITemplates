/// OS-level window backdrop / composition effects.
///
/// On Windows 11 (22H2+): Mica (`DWMSBT_MAINWINDOW`) and
/// Acrylic (`DWMSBT_TRANSIENTWINDOW`) are applied via DWM.
/// On older Windows or other platforms the call is a no-op.
///
/// Call `apply_backdrop` AFTER the window is visible (after `show()`),
/// so the HWND is live.

use crate::dsl::BgStyle;

#[cfg(target_os = "windows")]
mod imp;

#[cfg(target_os = "windows")]
mod dark_mode;

#[cfg(target_os = "windows")]
pub(crate) use dark_mode::is_dark_mode;

#[cfg(not(target_os = "windows"))]
/// Fallback dark-mode check for non-Windows platforms (always returns false).
pub(crate) fn is_dark_mode() -> bool { false }

/// Apply OS-level backdrop to a Slint window.
/// No-op when `style == BgStyle::Solid` or on non-Windows platforms.
pub fn apply_backdrop(window: &slint::Window, style: BgStyle) {
    #[cfg(target_os = "windows")]
    imp::apply_backdrop(window, style);

    #[cfg(not(target_os = "windows"))]
    let _ = (window, style);
}
