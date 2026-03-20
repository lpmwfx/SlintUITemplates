# SlintUITemplates — Claude Project Instructions

## Persona: AIDevOps

This project uses the **AIDevOps** persona (see `get_rule("personas/aidevops.md")`).

- **OPS mode**: Planning, PHASES, PROJECT, architecture — no source code
- **DEV mode**: Autonomous execution from TODO, one task at a time, commit per passing test
- Branch: `aidevops/<phase-id>` — never commit to main directly

## Startup Checklist (every session)

```
0. mcp__rulestools__setup(".")          — install hooks (idempotent)
1. get_rule("global/startup.md")        — mandatory session checklist
2. read proj/PROJECT                    — goal, stack, current phase
3. read proj/RULES                      — active rules, load each via get_rule()
4. read proj/PHASES                     — active phase + approach + patterns
5. read proj/UIUX                       — UI/UX source of truth (before any .slint work)
6. read proj/FIXES                      — known problems, avoid repeating
7. read proj/TODO                       — current tasks
```

## Stack

- Language: Rust 2021
- UI: Slint (`.slint` files in `ui/`)
- Platform: Windows
- Repo: https://github.com/lpmwfx/SlintUITemplates

## Slint Live Preview

```bash
# Standalone preview (no recompile)
slint-viewer ui/<file>.slint --auto-reload

# In-app hot-reload
# Cargo.toml: slint = { features = ["live-preview"] }
SLINT_LIVE_PREVIEW=1 cargo run
```

## Rules

- All proj/ files are in `.gitignore` — local only, never committed
- One `.slint` file per component — PascalCase matches filename
- All token values in `ui/tokens.slint` — never literal px/color in components
- Each template self-contained under `ui/templates/<name>/`
- Read `proj/UIUX` before ANY UI work — no exceptions

## Active Rule Files

Load via `get_rule()` at session start:

```
global/topology.md, global/module-tree.md, global/file-limits.md
global/config-driven.md, global/persistent-state.md
rust/README.md
uiux/tokens.md, uiux/components.md, uiux/state-flow.md
uiux/file-structure.md, uiux/help-about.md, uiux/checklist.md
uiux/menus-windows.md, uiux/menus-slint.md, uiux/mother-child.md
slint/README.md, slint/responsive-layout.md
```


---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
