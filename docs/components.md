# Component Catalog

All components live under `ui/`. Import only from `../tokens/theme.slint` — never import token files directly.

## Desktop Shell

### AppWindow (`ui/desktop/app-window.slint`)

Root window — owns ALL state. Uses mother-child pattern.

```slint
import { AppWindow } from "slint_ui_templates";  // via lib.slint
```

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `active-view` | string | `"home"` | Current view id |
| `nav-items` | `[NavItem]` | built-in 3 | Sidebar navigation items |
| `sidebar-open` | bool | true | Sidebar visibility |
| `sidebar-collapsed` | bool | false | Collapsed (icons only) vs expanded |
| `status-text` | string | `""` | Status bar text (empty = "View: {id}") |
| `show-toolbar` | bool | false | Show toolbar row |
| `toolbar-items` | `[ShellToolbarItem]` | `[]` | Toolbar buttons |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `navigate(id)` | string | Nav item clicked |
| `menu-action(id)` | string | MenuBar item activated |
| `toolbar-clicked(id)` | string | Toolbar button clicked |

---

### NavItem (`ui/desktop/types.slint`)

```slint
struct NavItem {
    id:    string,
    label: string,
    icon:  string,  // Segoe Fluent Icons codepoint
}
```

---

### SideBar (`ui/desktop/side-bar.slint`)

Vertical nav list. Left column of AppWindow.

| Property | Type | Description |
|----------|------|-------------|
| `items` | `[NavItem]` | Nav items to display |
| `active-item` | string | Highlighted item id |
| `collapsed` | bool | Icons-only mode |

| Callback | Description |
|----------|-------------|
| `navigate(id)` | Item clicked |
| `toggle-collapsed()` | Toggle collapse state |

---

### NavBar (`ui/desktop/nav-bar.slint`)

Top breadcrumb bar.

| Property | Type | Description |
|----------|------|-------------|
| `breadcrumb` | string | Current location label |
| `can-go-back` | bool | Show back button |

---

### StatusBar (`ui/desktop/status-bar.slint`)

Bottom status strip.

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `status-text` | string | `"Ready"` | Status message |
| `progress` | float | 0.0 | 0.0–1.0 progress value |
| `show-progress` | bool | false | Show progress bar |

---

### ContentArea (`ui/desktop/content-area.slint`)

Center pane — renders page components based on `active-view`.

| Property | Type | Description |
|----------|------|-------------|
| `active-view` | string | Current view id |

---

## Shell

### ShellToolbar (`ui/shell/desktop/tool-bar.slint`)

Optional toolbar row below native MenuBar.

```slint
struct ShellToolbarItem {
    id:      string,
    icon:    string,
    tooltip: string,
}
```

| Property | Type | Description |
|----------|------|-------------|
| `items` | `[ShellToolbarItem]` | Buttons to display |

---

## Modules

### DragHandle (`ui/modules/DragHandle.slint`)

Resizable gutter between panels.

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `vertical` | bool | true | Vertical (horizontal drag) or horizontal (vertical drag) |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `dragged(dx, dy)` | float, float | Drag delta in logical px |

---

### PanelContainer (`ui/modules/PanelContainer.slint`)

Grid panel layout driven by solved `[PanelItem]` model. Used by `mother/AppWindow`.

---

### ViewSlot (`ui/modules/ViewSlot.slint`)

Named slot for DSL view routing.

```slint
// usage
ViewSlot { view-id: "settings"; active-view: root.active-view; }
```

---

## Framework Viewer

### FrameworkViewer (`ui/viewer/viewer.slint`)

Live component gallery — shows all desktop components with interactive controls.
Used by `examples/viewer/main.rs`.

Pages:
- Buttons — Button, IconButton, ToggleButton
- Inputs — TextInput, Slider, Checkbox
- Display — Card, Badge, Chip
- Feedback — Toast, ProgressBar, Spinner
- Layout — PanelContainer, DragHandle
- Icons — FluentIcons codepoint reference
- App Shell — live AppWindow preview

---

## Globals (not components)

### Theme / Colors (`ui/state/Theme.slint`)

Unified color global. `Colors` is a barrel alias — both names are the same singleton.
See [theme-system.md](theme-system.md) for full token table.

### Settings (`ui/state/settings.slint`)

Zoom + font + icon settings. Written by `AppAdapter::apply_settings()`.
See [settings.md](settings.md).

### FluentIcons, Spacing, Type, Scale

All exported from `ui/tokens/theme.slint`.
See [tokens.md](tokens.md).
