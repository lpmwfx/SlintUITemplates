use std::rc::Rc;
use std::cell::RefCell;
use slint_ui_templates::adapter::AppAdapter;
use slint_ui_templates::bindings::rhai::build_engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let adapter = Rc::new(RefCell::new(AppAdapter::new()?));
    let engine = build_engine(Rc::clone(&adapter));

    let script = std::fs::read_to_string("examples/rhai/demo.rhai")?;
    engine.eval::<()>(&script)?;

    // Drop engine before consuming adapter — releases Rc clones held by closures
    drop(engine);

    Rc::try_unwrap(adapter)
        .unwrap_or_else(|_| panic!("engine not dropped"))
        .into_inner()
        .run()?;

    Ok(())
}
