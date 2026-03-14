/// Rhai-to-AppAdapter function bindings registered on the scripting engine.
pub mod api;
pub use api::register;

use rhai::Engine;
use std::rc::Rc;
use std::cell::RefCell;
use crate::adapter::AppAdapter;

/// Build a Rhai Engine with all AppAdapter API functions registered.
pub fn build_engine(adapter: Rc<RefCell<AppAdapter>>) -> Engine {
    let mut engine = Engine::new();
    api::register(&mut engine, adapter);
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
        let adapter = Rc::new(RefCell::new(AppAdapter::new().unwrap()));
        let engine = build_engine(Rc::clone(&adapter));
        engine.eval::<()>(r#"set_active_view("settings")"#).unwrap();
        assert_eq!(adapter.borrow().get_active_view(), "settings");
    }

    #[test]
    fn engine_set_dark_mode() {
        init();
        let adapter = Rc::new(RefCell::new(AppAdapter::new().unwrap()));
        let engine = build_engine(Rc::clone(&adapter));
        engine.eval::<()>("set_dark_mode(true)").unwrap();
        assert!(adapter.borrow().get_dark_mode());
    }
}
