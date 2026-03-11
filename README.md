# SlintUITemplates

Composable Slint UI building blocks — mother-child pattern, token-driven, zero literals.
Three ways to use it:

## 1. Design tool (no Rust needed)

```sh
git clone https://github.com/lpmwfx/SlintUITemplates
slint-viewer ui/desktop/app-window.slint --auto-reload
slint-viewer ui/mobile/app-window.slint  --auto-reload
```

Edit `ui/views/*.slint` — live preview updates instantly. No recompile.

## 2. DSL layout engine

```toml
[dependencies]
slint-ui-templates = "0.1"
```

```rust
use slint_ui_templates::layout;

let panels = layout::build("1:2/1:1:1", 1920.0, 1080.0);
// Returns Vec<SolvedItem> with normalized x/y/w/h
```

DSL: `:` = HSplit (columns), `/` = VSplit (rows), numbers = size ratios.

## 3. App template

```sh
cargo run                      # desktop app with OS dark-mode detection
cargo run --example desktop    # same via examples/
```

Clone, edit `ui/views/*.slint`, ship.

## Architecture

```
ui/tokens/     design tokens (colors auto-follow OS dark mode)
ui/desktop/    desktop mother + child views
ui/mobile/     mobile mother
ui/views/      ← designer edits here (home, list, settings)
ui/shared/     reusable components (Button, Card, Toggle, Avatar, ...)
src/layout/    DSL parser + solver
src/grid/      TOML grid config loader
```

## Components

| Component | Props | Callback |
|-----------|-------|----------|
| Button | `text`, `primary`, `disabled` | `clicked()` |
| Card | `title`, `subtitle` | — |
| Badge | `text`, `variant` | — |
| Toggle | `checked`, `label` | `toggled(bool)` |
| TextInput | `value`, `placeholder`, `error` | `changed(string)` |
| Avatar | `initials`, `size` (sm/md/lg) | — |
| ProgressBar | `value` (0.0..1.0), `show-label` | — |
| SelectField | `options`, `selected`, `placeholder` | `changed(string)` |

## License

MIT
