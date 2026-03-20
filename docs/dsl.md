# DSL Reference

The `AppDsl` builder is the primary Rust entry point for configuring the shell.
Rules are validated at `build()` — wrong configurations are errors, not silent failures.

## Quick example

```rust
use slint_ui_templates::{
    adapter::AppAdapter,
    dsl::{AppDsl, Nav, Toolbar, BgStyle},
};

let dsl = AppDsl::builder("My App")
    .nav(vec![
        Nav::new("home",     "Home",     "home"),
        Nav::new("list",     "List",     "list"),
        Nav::new("settings", "Settings", "settings"),
    ])
    .toolbar(vec![
        Toolbar::new("save", "save", "Save"),
        Toolbar::new("undo", "undo", "Undo"),
    ])
    .status("Connected")
    .window_size(1280, 800)
    .bg_style(BgStyle::Mica)
    .build()?;

let mut adapter = AppAdapter::new()?;
adapter.apply_dsl(&dsl);
adapter.run()?;
```

## Builder methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.nav(vec![...])` | `Vec<Nav>` | required | Navigation destinations (1–7) |
| `.toolbar(vec![...])` | `Vec<Toolbar>` | empty | Toolbar icon buttons (optional) |
| `.status(text)` | `&str` | `"Ready"` | Initial status bar text |
| `.window_size(w, h)` | `u32, u32` | Slint default | Initial window size in logical px |
| `.bg_style(style)` | `BgStyle` | `Solid` | Window backdrop material |
| `.platform(p)` | `Platform` | `Windows` | Target platform (affects nav limits) |
| `.views(vec![...])` | `Vec<&str>` | unchecked | Validate nav↔view id consistency |

## Nav items

```rust
Nav::new(id, label, icon_name)
```

- `id` — used for routing (`active-view = id`)
- `label` — displayed in SideBar
- `icon_name` — Fluent icon name (resolved to codepoint at `build()`)

Nav limits: desktop max 7, Android max 5. Excess → `DslError::TooManyNavItems`.

## Toolbar items

```rust
Toolbar::new(id, icon_name, tooltip)
```

Toolbar is hidden when empty. Shown below the native MenuBar.

## Backdrop (BgStyle)

| Value | OS API | Fallback |
|-------|--------|---------|
| `BgStyle::Solid` | opaque window | always works |
| `BgStyle::Mica` | `DWMSBT_MAINWINDOW` | solid on Windows < 11 22H2 |
| `BgStyle::Acrylic` | `DWMSBT_TRANSIENTWINDOW` | solid on older OS |

`apply_dsl()` sets both `Theme.material` (Slint token routing) and the OS DWM
backdrop attribute in one call.

## Validation errors

```rust
match dsl_result {
    Ok(dsl)   => adapter.apply_dsl(&dsl),
    Err(errs) => for e in &errs { eprintln!("{e}"); }
}
```

| Error | Cause |
|-------|-------|
| `NoNavItems` | nav() not called or empty |
| `TooManyNavItems` | > 7 desktop / > 5 Android |
| `NavItemMissingId(i)` | nav[i].id is empty |
| `UnknownIcon` | icon name not in FluentIcons registry |
| `NavWithoutView(id)` | nav id has no matching entry in .views() |
| `ViewWithoutNav(id)` | .views() entry has no matching nav item |

All errors are collected — build() never stops at the first one.

## AppAdapter callbacks

```rust
// Navigate callback — fires on nav item click or programmatic navigate()
adapter.on_navigate(|id| {
    println!("navigated to: {id}");
});

// Toolbar click
adapter.on_toolbar_clicked(|id| {
    if id == "save" { save_document(); }
});

// Menu action (id from Slint MenuBar)
adapter.on_menu_action("file.exit", || std::process::exit(0));

// ViewConfig auto-applied on navigate (loads views/*.rhai)
adapter.load_view_configs(Path::new("views/"))?;
```

## Rhai scripting

The same configuration is available at runtime via Rhai:

```js
// examples/rhai/demo.rhai
set_nav(["home:Home:home", "list:List:list", "settings:Settings:settings"]);
set_toolbar(["save:save:Save file", "undo:undo:Undo"]);
set_status("Connected");
set_window_size(1280, 800);
set_bg_style("mica");
set_dark_mode(true);
set_zoom(1.25);
set_icon_style("outlined");
set_font("Segoe UI");
```

## Available icon names

```
home  back  forward  menu  settings  nav-collapse  nav-expand
add  remove  delete  edit  save  copy  paste  cut  undo  redo
refresh  close  search  filter  sort  share  download  upload  print
maximize  minimize  restore
folder  folder-open  file  file-new  cloud  database
play  pause  stop  volume  mute  camera  image
list  grid  layout  apps  panel-left  panel-right  fullscreen
person  people  chat  mail  phone  notification  help  info
warning  error-icon  checkmark  dismiss  device  keyboard  wifi  bluetooth
star  star-filled  pin  unpin
```


---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
