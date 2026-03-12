use rhai::Engine;
use std::rc::Rc;
use std::cell::RefCell;
use crate::adapter::AppAdapter;

/// Register all AppAdapter API functions into the Rhai engine.
pub fn register(engine: &mut Engine, adapter: Rc<RefCell<AppAdapter>>) {
    let a = Rc::clone(&adapter);
    engine.register_fn("set_active_view", move |name: String| {
        a.borrow().set_active_view(&name);
    });

    let a = Rc::clone(&adapter);
    engine.register_fn("get_active_view", move || -> String {
        a.borrow().get_active_view()
    });

    let a = Rc::clone(&adapter);
    engine.register_fn("set_dark_mode", move |on: bool| {
        a.borrow().set_dark_mode(on);
    });

    let a = Rc::clone(&adapter);
    engine.register_fn("set_status", move |text: String| {
        a.borrow().set_status(&text);
    });
}
