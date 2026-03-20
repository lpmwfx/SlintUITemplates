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
<!-- AUTOGEN:shared START -->

## Shared Components

#### `AccordionItem` struct

| Field | Type |
|-------|------|
| `id` | `string` |
| `title` | `string` |
| `content` | `string` |

### Accordion (`ui/widgets/accordion.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `items` | `[AccordionItem]` | `[]` |  |
| `expanded-id` *(in-out)* | `string` | `""` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `toggled(string)` | |  |

---

### Alert (`ui/widgets/alert.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `message` | `string` | `""` |  |
| `title` | `string` | `""` |  |
| `variant` | `string` | `"info"` | info \| success \| warning \| error |
| `closable` | `bool` | `false` |  |
| `show` | `bool` | `true` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `closed()` | |  |

---

### Avatar (`ui/widgets/avatar.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `initials` | `string` | `""` |  |
| `size` | `string` | `"md"` | sm \| md \| lg |

---

### Badge (`ui/widgets/badge.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `text` | `string` | `""` |  |
| `variant` | `string` | `"info"` | info \| success \| warning \| error |

---

#### `BreadcrumbItem` struct

| Field | Type |
|-------|------|
| `id` | `string` |
| `label` | `string` |

### Breadcrumb (`ui/widgets/breadcrumb.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `items` | `[BreadcrumbItem]` | `[]` |  |
| `separator` | `string` | `"›"` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `navigate(string)` | |  |

---

### Button (`ui/widgets/button.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `label` | `string` | `"Button"` |  |
| `variant` | `string` | `"primary"` | primary \| secondary \| ghost \| danger |
| `enabled` | `bool` | `true` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `clicked()` | |  |

---

### CanvasImage (`ui/widgets/canvas-image.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `frame` | `image` | `` |  |
| `show-placeholder` | `bool` | `true` |  |

---

### CanvasPath (`ui/widgets/canvas-path.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `stroke-color` | `color` | `Colors.accent` |  |
| `fill-color` | `color` | `Colors.accent.with-alpha(0.15)` |  |
| `stroke-w` | `length` | `2px` |  |

---

### Card (`ui/widgets/card.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `title` | `string` | `""` |  |
| `subtitle` | `string` | `""` |  |

---

### Checkbox (`ui/widgets/checkbox.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `checked` | `bool` | `false` |  |
| `indeterminate` | `bool` | `false` |  |
| `label` | `string` | `""` |  |
| `enabled` | `bool` | `true` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `toggled(bool)` | |  |

---

### Chip (`ui/widgets/chip.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `label` | `string` | `""` |  |
| `closable` | `bool` | `true` |  |
| `icon` | `string` | `""` |  |
| `selected` | `bool` | `false` |  |
| `enabled` | `bool` | `true` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `closed()` | |  |
| `clicked()` | |  |

---

#### `CbfAction` struct

| Field | Type |
|-------|------|
| `id` | `string` |
| `icon` | `string` |
| `label` | `string` |
| `enabled` | `bool` |

### CommandBarFlyout (`ui/widgets/command-bar-flyout.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `primary` | `[CbfAction]` | `[]` |  |
| `secondary` | `[CbfAction]` | `[]` |  |
| `window-height` | `length` | `` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `action(string)` | |  |

---

#### `ContextMenuItem` struct

| Field | Type |
|-------|------|
| `id` | `string` |
| `label` | `string` |
| `icon` | `string` |
| `disabled` | `bool` |
| `separator` | `bool` |
| `keybinding` | `string` |

### ContextMenu (`ui/widgets/context-menu.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `items` | `[ContextMenuItem]` | `[]` |  |
| `separator-count` | `int` | `` |  |
| `window-height` | `length` | `` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `activated(string)` | |  |

---

#### `TableColumn` struct

| Field | Type |
|-------|------|
| `key` | `string` |
| `label` | `string` |
| `width` | `length` |

#### `TableRow` struct

| Field | Type |
|-------|------|
| `id` | `string` |
| `cells` | `[string]` |

### DataTable (`ui/widgets/data-table.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `columns` | `[TableColumn]` | `[]` |  |
| `rows` | `[TableRow]` | `[]` |  |
| `sort-key` *(in-out)* | `string` | `""` |  |
| `sort-asc` *(in-out)* | `bool` | `true` |  |
| `selected` *(in-out)* | `string` | `""` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `sort-changed(string, bool)` | |  |
| `row-selected(string)` | |  |

---

### DatePicker (`ui/widgets/date-picker.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `value` *(in-out)* | `string` | `""` |  |
| `label` | `string` | `""` |  |
| `placeholder` | `string` | `"Select date"` |  |
| `error` | `string` | `""` |  |
| `enabled` | `bool` | `true` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `changed(string)` | |  |

---

### Dialog (`ui/widgets/dialog.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `title` | `string` | `"Dialog"` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `confirmed()` | |  |
| `cancelled()` | |  |

---

### Divider (`ui/widgets/divider.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `orientation` | `string` | `"horizontal"` | horizontal \| vertical |
| `label` | `string` | `""` |  |
| `line-color` | `color` | `Colors.border` |  |

---

### Drawer (`ui/widgets/drawer.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `open` | `bool` | `false` |  |
| `placement` | `string` | `"left"` | left \| right |
| `drawer-width` | `length` | `280px` |  |
| `title` | `string` | `""` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `close-requested()` | |  |

---

#### `DropdownMenuItem` struct

| Field | Type |
|-------|------|
| `id` | `string` |
| `label` | `string` |
| `icon` | `string` |
| `disabled` | `bool` |
| `separator` | `bool` |

### DropdownMenu (`ui/widgets/dropdown-menu.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `items` | `[DropdownMenuItem]` | `[]` |  |
| `window-height` | `length` | `` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `activated(string)` | |  |

---

### EmptyState (`ui/widgets/empty-state.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `icon` | `string` | `"\u{E8FD}"` |  |
| `title` | `string` | `"No items"` |  |
| `description` | `string` | `""` |  |
| `action-label` | `string` | `""` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `action()` | |  |

---

### Flyout (`ui/widgets/flyout.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `window-height` | `length` | `` |  |
| `popup-width` | `length` | `220px` |  |
| `popup-height` | `length` | `160px` |  |
| `title` | `string` | `""` |  |
| `body` | `string` | `""` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `closed()` | |  |

---

### FormField (`ui/widgets/form-field.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `label` | `string` | `""` |  |
| `required` | `bool` | `false` |  |
| `error` | `string` | `""` |  |
| `hint` | `string` | `""` |  |

---

### Label (`ui/widgets/label.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `text` | `string` | `""` |  |
| `required` | `bool` | `false` |  |
| `disabled` | `bool` | `false` |  |

---

### ListItem (`ui/widgets/list-item.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `title` | `string` | `""` |  |
| `subtitle` | `string` | `""` |  |
| `selected` | `bool` | `false` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `clicked()` | |  |

---

### MessageBar (`ui/widgets/message-bar.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `message` | `string` | `""` |  |
| `variant` | `string` | `"info"` | info \| warning \| error \| success |
| `action-label` | `string` | `""` |  |
| `closable` | `bool` | `true` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `action()` | |  |
| `closed()` | |  |

---

#### `MultiSelectOption` struct

| Field | Type |
|-------|------|
| `label` | `string` |
| `value` | `string` |

### MultiSelect (`ui/widgets/multi-select.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `options` | `[MultiSelectOption]` | `[]` |  |
| `selected` *(in-out)* | `[string]` | `[]` |  |
| `placeholder` | `string` | `"Select..."` |  |
| `enabled` | `bool` | `true` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `changed([string])` | |  |

---

### Pagination (`ui/widgets/pagination.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `current` | `int` | `1` |  |
| `total` | `int` | `1` |  |
| `pages` | `[int]` | `[]` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `changed(int)` | |  |

---

### PasswordInput (`ui/widgets/password-input.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `label` | `string` | `""` |  |
| `placeholder` | `string` | `""` |  |
| `error` | `string` | `""` |  |
| `enabled` | `bool` | `true` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `changed(string)` | |  |

---

### Popover (`ui/widgets/popover.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `open` | `bool` | `false` |  |
| `popup-width` | `length` | `220px` |  |
| `popup-height` | `length` | `140px` |  |
| `title` | `string` | `""` |  |
| `body` | `string` | `""` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `close-requested()` | |  |

---

### ProgressBar (`ui/widgets/progress-bar.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `value` | `float` | `0.0` | 0.0..1.0 |
| `show-label` | `bool` | `false` |  |

---

### RadioButton (`ui/widgets/radio.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `checked` | `bool` | `false` |  |
| `label` | `string` | `""` |  |
| `enabled` | `bool` | `true` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `clicked()` | |  |

---

### RadioGroup (`ui/widgets/radio.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `options` | `[string]` | `[]` |  |
| `value` *(in-out)* | `string` | `""` |  |
| `enabled` | `bool` | `true` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `changed(string)` | |  |

---

### Rating (`ui/widgets/rating.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `value` *(in-out)* | `int` | `0` |  |
| `max` | `int` | `5` |  |
| `readonly` | `bool` | `false` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `changed(int)` | |  |

---

### ScrollView (`ui/widgets/scroll-view.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `interactive` | `bool` | `true` |  |
| `show-scrollbar` | `bool` | `true` |  |
| `viewport-width  <=> fl.viewport-width` *(out)* | `length` | `` |  |
| `viewport-height <=> fl.viewport-height` *(out)* | `length` | `` |  |

---

### SearchInput (`ui/widgets/search-input.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `value` *(in-out)* | `string` | `""` |  |
| `placeholder` | `string` | `"Search..."` |  |
| `enabled` | `bool` | `true` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `changed(string)` | |  |
| `submitted(string)` | |  |
| `cleared()` | |  |

---

#### `SegmentItem` struct

| Field | Type |
|-------|------|
| `id` | `string` |
| `label` | `string` |
| `icon` | `string` |

### Segmented (`ui/widgets/segmented.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `items` | `[SegmentItem]` | `[]` |  |
| `selected` *(in-out)* | `string` | `""` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `changed(string)` | |  |

---

### SelectField (`ui/widgets/select-field.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `options` | `[string]` | `[]` |  |
| `selected` | `string` | `""` |  |
| `placeholder` | `string` | `"Select..."` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `changed(string)` | |  |

---

### Skeleton (`ui/widgets/skeleton.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `variant` | `string` | `"rect"` | text \| circle \| rect |
| `animate` | `bool` | `true` |  |

---

### Slider (`ui/widgets/slider.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `value` *(in-out)* | `float` | `0.0` |  |
| `min` | `float` | `0.0` |  |
| `max` | `float` | `1.0` |  |
| `step` | `float` | `0.0` |  |
| `label` | `string` | `""` |  |
| `show-value` | `bool` | `false` |  |
| `enabled` | `bool` | `true` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `changed(float)` | |  |

---

### SpinBox (`ui/widgets/spin-box.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `value` *(in-out)* | `int` | `0` |  |
| `min` | `int` | `0` |  |
| `max` | `int` | `100` |  |
| `step` | `int` | `1` |  |
| `label` | `string` | `""` |  |
| `enabled` | `bool` | `true` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `changed(int)` | |  |

---

### Spinner (`ui/widgets/spinner.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `size` | `string` | `"md"` |  |
| `spinner-color` | `color` | `Colors.accent` |  |
| `active` | `bool` | `true` |  |

---

#### `StepItem` struct

| Field | Type |
|-------|------|
| `label` | `string` |
| `completed` | `bool` |

### Stepper (`ui/widgets/stepper.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `steps` | `[StepItem]` | `[]` |  |
| `active` *(in-out)* | `int` | `0` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `step-changed(int)` | |  |

---

#### `TabItem` struct

| Field | Type |
|-------|------|
| `id` | `string` |
| `label` | `string` |
| `icon` | `string` |

### TabBar (`ui/widgets/tab-bar.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `tabs` | `[TabItem]` | `[]` |  |
| `active` *(in-out)* | `string` | `""` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `changed(string)` | |  |

---

### TeachingTip (`ui/widgets/teaching-tip.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `window-height` | `length` | `` |  |
| `title` | `string` | `""` |  |
| `body` | `string` | `""` |  |
| `action-label` | `string` | `"Got it"` |  |
| `dismiss-label` | `string` | `""` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `action()` | |  |
| `dismissed()` | |  |

---

### TextArea (`ui/widgets/text-area.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `label` | `string` | `""` |  |
| `value` *(in-out)* | `string` | `""` |  |
| `placeholder` | `string` | `""` |  |
| `error` | `string` | `""` |  |
| `rows` | `int` | `4` |  |
| `enabled` | `bool` | `true` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `changed(string)` | |  |

---

### TextInput (`ui/widgets/text-input.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `label` | `string` | `""` |  |
| `value` | `string` | `""` |  |
| `placeholder` | `string` | `""` |  |
| `error` | `string` | `""` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `changed(string)` | |  |
| `submitted(string)` | |  |

---

### Toast (`ui/widgets/toast.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `message` | `string` | `""` |  |
| `variant` | `string` | `"info"` | info \| success \| warning \| error |

---

### Toggle (`ui/widgets/toggle.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `checked` | `bool` | `false` |  |
| `label` | `string` | `""` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `toggled(bool)` | |  |

---

#### `ToolbarItem` struct

| Field | Type |
|-------|------|
| `id` | `string` |
| `icon` | `string` |
| `tooltip` | `string` |
| `enabled` | `bool` |

### Toolbar (`ui/widgets/toolbar.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `items` | `[ToolbarItem]` | `[]` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `clicked(string)` | |  |

---

### Tooltip (`ui/widgets/tooltip.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `text` | `string` | `""` |  |
| `placement` | `string` | `"top"` | top \| bottom \| left \| right |
| `enabled` | `bool` | `true` |  |

---

#### `TreeNode` struct

| Field | Type |
|-------|------|
| `id` | `string` |
| `label` | `string` |
| `icon` | `string` |
| `level` | `int` |
| `expanded` | `bool` |
| `leaf` | `bool` |

### TreeView (`ui/widgets/tree-view.slint`)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `nodes` | `[TreeNode]` | `[]` |  |
| `selected` *(in-out)* | `string` | `""` |  |

| Callback | Signature | Description |
|----------|-----------|-------------|
| `selected-changed(string)` | |  |
| `toggle-expanded(string)` | |  |

---

<!-- AUTOGEN:shared END -->


---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
