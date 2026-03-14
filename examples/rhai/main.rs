use std::rc::Rc;
use std::cell::RefCell;
use slint_ui_templates::adapter::AppAdapter;
use slint_ui_templates::bindings::rhai::build_engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // REASON: Rhai engine holds Rc clones of adapter for callback access
    let adapter = Rc::new(RefCell::new(AppAdapter::new()?));
    let engine = build_engine(Rc::clone(&adapter));

    let script = std::fs::read_to_string("examples/rhai/demo.rhai")?;
    engine.eval::<()>(&script)?;

    // Drop engine before consuming adapter — releases Rc clones held by closures
    drop(engine);

    Rc::try_unwrap(adapter)
        .map_err(|_| "engine not dropped — leaked Rc reference")?
        .into_inner()
        .run()?;

    Ok(())
}
