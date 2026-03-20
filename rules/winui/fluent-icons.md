---
name: Fluent Icon Usage Rules
description: When to use which Segoe Fluent Icon, font usage patterns
type: rule
platform: windows
---

# Fluent Icon Usage Rules

## Font

- Font name: `"Segoe Fluent Icons"`
- Pre-installed on all Windows 11 systems — NO packaging required
- Access via `FluentIcons` global in `ui/tokens/theme.slint`

## Usage Pattern

```slint
import { FluentIcons } from "../tokens/theme.slint";

Text {
    text: FluentIcons.settings;
    font-family: "Segoe Fluent Icons";
    font-size: Type.subtitle-size;   // or Scale.icon-md
    color: Colors.text-primary;
}
```

## In NavItem lists

```slint
nav-items: [
    { id: "home",     label: "Home",     icon: FluentIcons.home },
    { id: "settings", label: "Settings", icon: FluentIcons.settings },
];
```

The rendering component (SideBar, BottomNavBar, etc.) must use:
```slint
Text { text: item.icon; font-family: "Segoe Fluent Icons"; }
```

## Icon Size Guidelines

| Context | Size | Token |
|---------|------|-------|
| Navigation item | 20px | `Scale.icon-md` |
| Toolbar button | 20px | `Scale.icon-md` |
| Small status icon | 16px | `Scale.icon-sm` |
| Large / hero icon | 32px | `Scale.icon-lg` |
| Thumbnail | 48px | `Scale.icon-xl` |

## Canonical Icon Assignments

| UI Element | Icon | Rationale |
|------------|------|-----------|
| Home nav item | `FluentIcons.home` | Universal |
| Settings nav item | `FluentIcons.settings` | WinUI3 standard |
| List view | `FluentIcons.list` | Standard list |
| Back button | `FluentIcons.back` | Directional |
| Add / New | `FluentIcons.add` | Standard CUD |
| Delete | `FluentIcons.delete` | Destructive |
| Save | `FluentIcons.save` | Standard |
| Edit / Rename | `FluentIcons.edit` | Standard |
| Search | `FluentIcons.search` | Universal |
| Refresh | `FluentIcons.refresh` | Standard |
| Close / Dismiss | `FluentIcons.close` | Standard |
| User / Account | `FluentIcons.person` | Standard |
| Notification | `FluentIcons.notification` | Standard |
| Help | `FluentIcons.help` | Standard |

## Rules

- NEVER use emoji as icon (🏠 ⚙ 📋) — use FluentIcons codepoints
- NEVER use Unicode symbols (▣ ✎ ◈) as icons — use FluentIcons
- ALWAYS set `font-family: "Segoe Fluent Icons"` when rendering icon strings
- ALWAYS use `FluentIcons.<name>` — never hardcode codepoint strings in components
- Icon-only elements MUST have `accessible-description` property for accessibility
- Toolbar icons SHOULD have tooltips
- Use filled variant for selected/active state, outlined for inactive (when both available)


---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
