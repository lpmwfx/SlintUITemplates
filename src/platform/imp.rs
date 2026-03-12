use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::Graphics::Dwm::{DwmExtendFrameIntoClientArea, DwmSetWindowAttribute};
use windows_sys::Win32::UI::Controls::MARGINS;
use i_slint_backend_winit::WinitWindowAccessor;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use crate::dsl::BgStyle;

// Windows 11 DWM backdrop constants (DWMWA_SYSTEMBACKDROP_TYPE / DWM_SYSTEMBACKDROP_TYPE).
// Defined manually — not present in all windows-sys 0.52 feature sets.
const DWMWA_SYSTEMBACKDROP_TYPE: u32 = 38;
const DWMSBT_NONE: i32         = 1; // no backdrop (Solid)
const DWMSBT_MAINWINDOW: i32   = 2; // Mica
const DWMSBT_TRANSIENTWINDOW: i32 = 3; // Acrylic

pub fn apply_backdrop(window: &slint::Window, style: BgStyle) {
    window.with_winit_window(|w| {
        let hwnd: HWND = match w.window_handle().map(|h| h.as_raw()) {
            Ok(RawWindowHandle::Win32(h)) => h.hwnd.get() as HWND,
            _ => return,
        };

        unsafe {
            // Extend DWM non-client frame into the entire client area.
            // Required for client-area backdrop to show through.
            let margins = MARGINS {
                cxLeftWidth:   -1,
                cxRightWidth:  -1,
                cyTopHeight:   -1,
                cyBottomHeight: -1,
            };
            let _ = DwmExtendFrameIntoClientArea(hwnd, &margins);

            // Request system backdrop type.
            let backdrop: i32 = match style {
                BgStyle::Solid   => DWMSBT_NONE,
                BgStyle::Mica    => DWMSBT_MAINWINDOW,
                BgStyle::Acrylic => DWMSBT_TRANSIENTWINDOW,
            };
            let _ = DwmSetWindowAttribute(
                hwnd,
                DWMWA_SYSTEMBACKDROP_TYPE,
                &backdrop as *const i32 as *const core::ffi::c_void,
                core::mem::size_of::<i32>() as u32,
            );
        }
    });
}
