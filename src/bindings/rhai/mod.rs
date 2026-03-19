// NOTE(mother-child): build_engine is the sole non-test fn; remaining fns are
// #[cfg(test)] helpers — total logic is minimal, no extraction needed.

// Guard: rhai's `sync` feature requires Send+Sync on all closures and captured
// values, but Slint's AppWindow is !Send+!Sync.  The two are fundamentally
// incompatible — even wrapping in Arc<Mutex<T>> would not help because the
// inner Slint handle cannot cross thread boundaries.
#[cfg(feature = "rhai-sync")]
compile_error!(
    "slint-ui-templates rhai bindings are not compatible with rhai's `sync` feature. \
     Slint requires single-threaded access — use the `rhai` feature without `rhai-sync`."
);

/// Rhai-to-AppAdapter function bindings registered on the scripting engine.
///
/// # Limitations
///
/// **Thread-Safety**: The Slint UI framework is single-threaded (`!Send + !Sync`). Rhai's default
/// `Engine` has a `sync` feature that requires all closures and captured values to be `Send + Sync`.
/// This creates a fundamental incompatibility: the AppAdapter holds an `AppWindow` which is not
/// `Send`, yet Rhai's closures capture the adapter to register callbacks.
///
/// **Workaround**: This module disables Rhai's `sync` feature. Rhai engine instances remain
/// thread-local and must not be shared across threads. Use this engine only from the main UI thread.
///
/// **Implication**: Applications using Rhai scripting cannot spawn background threads that
/// interact with the UI. All Rhai scripts execute on the main UI thread.
pub mod api;
/// DSL API — nav, toolbar, window, bg style registration.
pub mod dsl;
pub use api::register_api;

use rhai::Engine;
use std::rc::Rc;
use std::cell::RefCell;
use crate::AppAdapter;

/// Build a Rhai Engine with all AppAdapter API functions registered.
pub fn build_engine(adapter: Rc<RefCell<AppAdapter>>) -> Engine {
    let mut engine = Engine::new();
    api::register_api(&mut engine, adapter);
    engine
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        i_slint_backend_testing::init_no_event_loop();
    }

    #[test]
    fn engine_set_get_active_view() {
        init();
        // REASON: Rhai closures capture adapter — AppWindow is !Sync, single-threaded
        let adapter = Rc::new(RefCell::new(AppAdapter::new().unwrap()));
        let engine = build_engine(Rc::clone(&adapter));
        engine.eval::<()>(r#"set_active_view("settings")"#).unwrap();
        assert_eq!(adapter.borrow().get_active_view(), "settings");
    }

    #[test]
    fn engine_set_dark_mode() {
        init();
        // REASON: Rhai closures capture adapter — AppWindow is !Sync, single-threaded
        let adapter = Rc::new(RefCell::new(AppAdapter::new().unwrap()));
        let engine = build_engine(Rc::clone(&adapter));
        engine.eval::<()>("set_dark_mode(true)").unwrap();
        assert!(adapter.borrow().is_dark_mode());
    }
}
