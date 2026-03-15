# Documentation Index

Generated: 2026-03-15T21:41:57.005331500+01:00  
Project: `src`  
Coverage: **176/176** items documented (**100%**)

## Files

| Source File | Items | Undocumented |
|---|---|---|
| [adapter/apply.rs](man/adapter/apply.md) | 3 | ✓ |
| [adapter/callbacks.rs](man/adapter/callbacks.md) | 3 | ✓ |
| [adapter/config.rs](man/adapter/config.md) | 3 | ✓ |
| [adapter/mod.rs](man/adapter/mod.md) | 4 | ✓ |
| [adapter/state.rs](man/adapter/state.md) | 11 | ✓ |
| [adapter/tests.rs](man/adapter/tests.md) | 0 | — |
| [bin/docgen.rs](man/bin/docgen.md) | 0 | — |
| [bin/slintui.rs](man/bin/slintui.md) | 0 | — |
| [bindings/mod.rs](man/bindings/mod.md) | 1 | ✓ |
| [bindings/rhai/api.rs](man/bindings/rhai/api.md) | 1 | ✓ |
| [bindings/rhai/dsl.rs](man/bindings/rhai/dsl.md) | 1 | ✓ |
| [bindings/rhai/mod.rs](man/bindings/rhai/mod.md) | 3 | ✓ |
| [docs/mod.rs](man/docs/mod.md) | 1 | ✓ |
| [docs/parser.rs](man/docs/parser.md) | 1 | ✓ |
| [dsl/apply.rs](man/dsl/apply.md) | 1 | ✓ |
| [dsl/builder.rs](man/dsl/builder.md) | 10 | ✓ |
| [dsl/error.rs](man/dsl/error.md) | 1 | ✓ |
| [dsl/icons.rs](man/dsl/icons.md) | 2 | ✓ |
| [dsl/mod.rs](man/dsl/mod.md) | 9 | ✓ |
| [dsl/tests.rs](man/dsl/tests.md) | 0 | — |
| [dsl/types.rs](man/dsl/types.md) | 6 | ✓ |
| [gateway/grid.rs](man/gateway/grid.md) | 1 | ✓ |
| [gateway/mod.rs](man/gateway/mod.md) | 5 | ✓ |
| [gateway/scaffold.rs](man/gateway/scaffold.md) | 1 | ✓ |
| [gateway/scripts.rs](man/gateway/scripts.md) | 1 | ✓ |
| [gateway/settings.rs](man/gateway/settings.md) | 3 | ✓ |
| [gateway/view_config.rs](man/gateway/view_config.md) | 2 | ✓ |
| [grid/config.rs](man/grid/config.md) | 6 | ✓ |
| [grid/mod.rs](man/grid/mod.md) | 3 | ✓ |
| [grid/zone.rs](man/grid/zone.md) | 10 | ✓ |
| [layout/constraints.rs](man/layout/constraints.md) | 3 | ✓ |
| [layout/dsl_v2.rs](man/layout/dsl_v2.md) | 3 | ✓ |
| [layout/mod.rs](man/layout/mod.md) | 8 | ✓ |
| [layout/named_parser.rs](man/layout/named_parser.md) | 1 | ✓ |
| [layout/parser.rs](man/layout/parser.md) | 4 | ✓ |
| [layout/ratio_solver.rs](man/layout/ratio_solver.md) | 6 | ✓ |
| [layout/solver.rs](man/layout/solver.md) | 6 | ✓ |
| [lib.rs](man/lib.md) | 11 | ✓ |
| [main.rs](man/main.md) | 0 | — |
| [pal/dark_mode.rs](man/pal/dark_mode.md) | 1 | ✓ |
| [pal/imp.rs](man/pal/imp.md) | 1 | ✓ |
| [pal/mod.rs](man/pal/mod.md) | 2 | ✓ |
| [settings/apply.rs](man/settings/apply.md) | 1 | ✓ |
| [settings/defaults.rs](man/settings/defaults.md) | 2 | ✓ |
| [settings/mod.rs](man/settings/mod.md) | 11 | ✓ |
| [shell/mod.rs](man/shell/mod.md) | 8 | ✓ |
| [shell/models.rs](man/shell/models.md) | 2 | ✓ |
| [shell/platform.rs](man/shell/platform.md) | 5 | ✓ |
| [view_config/error.rs](man/view_config/error.md) | 1 | ✓ |
| [view_config/eval.rs](man/view_config/eval.md) | 3 | ✓ |
| [view_config/mod.rs](man/view_config/mod.md) | 4 | ✓ |

## All Items

| Item | Kind | Source | Line | Documented |
|---|---|---|---|---|
| `apply_settings` | fn | adapter/apply.rs | 24 | ✓ |
| `apply_theme` | fn | adapter/apply.rs | 30 | ✓ |
| `apply_grid` | fn | adapter/apply.rs | 35 | ✓ |
| `on_navigate` | fn | adapter/callbacks.rs | 4 | ✓ |
| `on_toolbar_clicked` | fn | adapter/callbacks.rs | 10 | ✓ |
| `on_menu_action` | fn | adapter/callbacks.rs | 16 | ✓ |
| `load_view_configs` | fn | adapter/config.rs | 7 | ✓ |
| `set_window_size` | fn | adapter/config.rs | 14 | ✓ |
| `set_bg_style_str` | fn | adapter/config.rs | 19 | ✓ |
| `AppAdapter_adp` | struct | adapter/mod.rs | 17 | ✓ |
| `new` | fn | adapter/mod.rs | 41 | ✓ |
| `apply_dsl` | fn | adapter/mod.rs | 100 | ✓ |
| `run` | fn | adapter/mod.rs | 107 | ✓ |
| `set_active_view` | fn | adapter/state.rs | 7 | ✓ |
| `get_active_view` | fn | adapter/state.rs | 13 | ✓ |
| `set_dark_mode` | fn | adapter/state.rs | 18 | ✓ |
| `get_dark_mode` | fn | adapter/state.rs | 24 | ✓ |
| `set_status` | fn | adapter/state.rs | 29 | ✓ |
| `get_status` | fn | adapter/state.rs | 35 | ✓ |
| `get_zoom` | fn | adapter/state.rs | 40 | ✓ |
| `get_row_top_ratio` | fn | adapter/state.rs | 45 | ✓ |
| `get_row_main_ratio` | fn | adapter/state.rs | 50 | ✓ |
| `set_platform` | fn | adapter/state.rs | 56 | ✓ |
| `get_platform` | fn | adapter/state.rs | 62 | ✓ |
| `rhai` | mod | bindings/mod.rs | 2 | ✓ |
| `register` | fn | bindings/rhai/api.rs | 7 | ✓ |
| `register` | fn | bindings/rhai/dsl.rs | 11 | ✓ |
| `api` | mod | bindings/rhai/mod.rs | 2 | ✓ |
| `dsl` | mod | bindings/rhai/mod.rs | 4 | ✓ |
| `build_engine` | fn | bindings/rhai/mod.rs | 13 | ✓ |
| `parser` | mod | docs/mod.rs | 10 | ✓ |
| `parse` | fn | docs/parser.rs | 19 | ✓ |
| `apply` | fn | dsl/apply.rs | 8 | ✓ |
| `AppDslBuilder` | struct | dsl/builder.rs | 11 | ✓ |
| `new` | fn | dsl/builder.rs | 58 | ✓ |
| `platform` | fn | dsl/builder.rs | 73 | ✓ |
| `nav` | fn | dsl/builder.rs | 79 | ✓ |
| `status` | fn | dsl/builder.rs | 85 | ✓ |
| `toolbar` | fn | dsl/builder.rs | 91 | ✓ |
| `window_size` | fn | dsl/builder.rs | 97 | ✓ |
| `bg_style` | fn | dsl/builder.rs | 103 | ✓ |
| `views` | fn | dsl/builder.rs | 109 | ✓ |
| `build` | fn | dsl/builder.rs | 115 | ✓ |
| `DslError` | enum | dsl/error.rs | 6 | ✓ |
| `IconName` | enum | dsl/icons.rs | 9 | ✓ |
| `fluent_icon` | fn | dsl/icons.rs | 143 | ✓ |
| `apply` | mod | dsl/mod.rs | 25 | ✓ |
| `builder` | mod | dsl/mod.rs | 27 | ✓ |
| `icons` | mod | dsl/mod.rs | 29 | ✓ |
| `types` | mod | dsl/mod.rs | 31 | ✓ |
| `error` | mod | dsl/mod.rs | 33 | ✓ |
| `ResolvedNav` | struct | dsl/mod.rs | 47 | ✓ |
| `ResolvedToolbar` | struct | dsl/mod.rs | 58 | ✓ |
| `AppDsl` | struct | dsl/mod.rs | 73 | ✓ |
| `builder` | fn | dsl/mod.rs | 85 | ✓ |
| `BgStyle` | enum | dsl/types.rs | 8 | ✓ |
| `from_str` | fn | dsl/types.rs | 20 | ✓ |
| `Nav` | struct | dsl/types.rs | 34 | ✓ |
| `new` | fn | dsl/types.rs | 45 | ✓ |
| `Toolbar` | struct | dsl/types.rs | 57 | ✓ |
| `new` | fn | dsl/types.rs | 68 | ✓ |
| `load` | fn | gateway/grid.rs | 5 | ✓ |
| `settings` | mod | gateway/mod.rs | 2 | ✓ |
| `view_config` | mod | gateway/mod.rs | 4 | ✓ |
| `grid` | mod | gateway/mod.rs | 6 | ✓ |
| `scaffold` | mod | gateway/mod.rs | 8 | ✓ |
| `scripts` | mod | gateway/mod.rs | 10 | ✓ |
| `scaffold` | fn | gateway/scaffold.rs | 11 | ✓ |
| `load_script` | fn | gateway/scripts.rs | 4 | ✓ |
| `from_file` | fn | gateway/settings.rs | 5 | ✓ |
| `from_file_or_default` | fn | gateway/settings.rs | 11 | ✓ |
| `to_file` | fn | gateway/settings.rs | 16 | ✓ |
| `eval_file` | fn | gateway/view_config.rs | 6 | ✓ |
| `load_all` | fn | gateway/view_config.rs | 13 | ✓ |
| `TargetConfig` | struct | grid/config.rs | 6 | ✓ |
| `TargetInfo` | struct | grid/config.rs | 16 | ✓ |
| `GridConfig` | struct | grid/config.rs | 28 | ✓ |
| `RowConfig` | struct | grid/config.rs | 36 | ✓ |
| `ColumnConfig` | struct | grid/config.rs | 51 | ✓ |
| `load` | fn | grid/config.rs | 60 | ✓ |
| `config` | mod | grid/mod.rs | 2 | ✓ |
| `zone` | mod | grid/mod.rs | 4 | ✓ |
| `load_target` | fn | grid/mod.rs | 12 | ✓ |
| `ZoneModel` | struct | grid/zone.rs | 6 | ✓ |
| `RowZone` | struct | grid/zone.rs | 15 | ✓ |
| `RowKind` | enum | grid/zone.rs | 27 | ✓ |
| `ColumnZone` | struct | grid/zone.rs | 36 | ✓ |
| `from_config` | fn | grid/zone.rs | 47 | ✓ |
| `total_row_ratio` | fn | grid/zone.rs | 57 | ✓ |
| `row` | fn | grid/zone.rs | 62 | ✓ |
| `column` | fn | grid/zone.rs | 67 | ✓ |
| `set_module` | fn | grid/zone.rs | 75 | ✓ |
| `total_column_ratio` | fn | grid/zone.rs | 123 | ✓ |
| `Constraint` | struct | layout/constraints.rs | 14 | ✓ |
| `new` | fn | layout/constraints.rs | 23 | ✓ |
| `clamp` | fn | layout/constraints.rs | 31 | ✓ |
| `NamedPanel` | struct | layout/dsl_v2.rs | 19 | ✓ |
| `SplitDir` | enum | layout/dsl_v2.rs | 31 | ✓ |
| `to_panels` | fn | layout/dsl_v2.rs | 34 | ✓ |
| `constraints` | mod | layout/mod.rs | 2 | ✓ |
| `dsl_v2` | mod | layout/mod.rs | 4 | ✓ |
| `named_parser` | mod | layout/mod.rs | 6 | ✓ |
| `parser` | mod | layout/mod.rs | 8 | ✓ |
| `ratio_solver` | mod | layout/mod.rs | 10 | ✓ |
| `solver` | mod | layout/mod.rs | 12 | ✓ |
| `build` | fn | layout/mod.rs | 22 | ✓ |
| `build_v2` | fn | layout/mod.rs | 29 | ✓ |
| `parse_named` | fn | layout/named_parser.rs | 7 | ✓ |
| `SplitDir` | enum | layout/parser.rs | 16 | ✓ |
| `PanelNode` | enum | layout/parser.rs | 21 | ✓ |
| `ratio` | fn | layout/parser.rs | 28 | ✓ |
| `parse` | fn | layout/parser.rs | 38 | ✓ |
| `Panel` | struct | layout/ratio_solver.rs | 17 | ✓ |
| `new` | fn | layout/ratio_solver.rs | 28 | ✓ |
| `with_constraint` | fn | layout/ratio_solver.rs | 33 | ✓ |
| `drag` | fn | layout/ratio_solver.rs | 45 | ✓ |
| `normalize` | fn | layout/ratio_solver.rs | 70 | ✓ |
| `check_sum` | fn | layout/ratio_solver.rs | 80 | ✓ |
| `SolvedItem` | struct | layout/solver.rs | 14 | ✓ |
| `ItemKind` | enum | layout/solver.rs | 34 | ✓ |
| `as_str` | fn | layout/solver.rs | 38 | ✓ |
| `Solver` | struct | layout/solver.rs | 48 | ✓ |
| `new` | fn | layout/solver.rs | 58 | ✓ |
| `solve` | fn | layout/solver.rs | 63 | ✓ |
| `adapter` | mod | lib.rs | 4 | ✓ |
| `bindings` | mod | lib.rs | 6 | ✓ |
| `docs` | mod | lib.rs | 8 | ✓ |
| `dsl` | mod | lib.rs | 10 | ✓ |
| `grid` | mod | lib.rs | 12 | ✓ |
| `layout` | mod | lib.rs | 14 | ✓ |
| `gateway` | mod | lib.rs | 16 | ✓ |
| `pal` | mod | lib.rs | 18 | ✓ |
| `settings` | mod | lib.rs | 20 | ✓ |
| `shell` | mod | lib.rs | 22 | ✓ |
| `view_config` | mod | lib.rs | 24 | ✓ |
| `is_dark_mode` | fn | pal/dark_mode.rs | 3 | ✓ |
| `apply_backdrop` | fn | pal/imp.rs | 16 | ✓ |
| `is_dark_mode` | fn | pal/mod.rs | 23 | ✓ |
| `apply_backdrop` | fn | pal/mod.rs | 27 | ✓ |
| `apply` | fn | settings/apply.rs | 13 | ✓ |
| `from_str` | fn | settings/defaults.rs | 5 | ✓ |
| `style_str` | fn | settings/defaults.rs | 12 | ✓ |
| `apply` | mod | settings/mod.rs | 2 | ✓ |
| `AppSettings` | struct | settings/mod.rs | 11 | ✓ |
| `ZoomSettings` | struct | settings/mod.rs | 29 | ✓ |
| `ThemeMode` | enum | settings/mod.rs | 38 | ✓ |
| `ThemeSettings` | struct | settings/mod.rs | 47 | ✓ |
| `IconStyle` | enum | settings/mod.rs | 58 | ✓ |
| `IconSettings` | struct | settings/mod.rs | 66 | ✓ |
| `FontSettings` | struct | settings/mod.rs | 76 | ✓ |
| `from_file` | fn | settings/mod.rs | 87 | ✓ |
| `from_file_or_default` | fn | settings/mod.rs | 92 | ✓ |
| `to_file` | fn | settings/mod.rs | 97 | ✓ |
| `platform` | mod | shell/mod.rs | 2 | ✓ |
| `models` | mod | shell/mod.rs | 6 | ✓ |
| `NavItemConfig` | struct | shell/mod.rs | 12 | ✓ |
| `ToolbarItemConfig` | struct | shell/mod.rs | 24 | ✓ |
| `ShellConfig` | struct | shell/mod.rs | 37 | ✓ |
| `new` | fn | shell/mod.rs | 52 | ✓ |
| `with_nav` | fn | shell/mod.rs | 61 | ✓ |
| `with_toolbar` | fn | shell/mod.rs | 67 | ✓ |
| `nav_model` | fn | shell/models.rs | 9 | ✓ |
| `toolbar_model` | fn | shell/models.rs | 22 | ✓ |
| `Platform` | enum | shell/platform.rs | 4 | ✓ |
| `as_str` | fn | shell/platform.rs | 18 | ✓ |
| `is_mobile` | fn | shell/platform.rs | 29 | ✓ |
| `is_small` | fn | shell/platform.rs | 35 | ✓ |
| `is_desktop` | fn | shell/platform.rs | 41 | ✓ |
| `ViewConfigError` | enum | view_config/error.rs | 4 | ✓ |
| `eval_script` | fn | view_config/eval.rs | 33 | ✓ |
| `eval_file` | fn | view_config/eval.rs | 63 | ✓ |
| `load_all` | fn | view_config/eval.rs | 68 | ✓ |
| `error` | mod | view_config/mod.rs | 8 | ✓ |
| `eval` | mod | view_config/mod.rs | 10 | ✓ |
| `ViewConfig` | struct | view_config/mod.rs | 27 | ✓ |
| `apply` | fn | view_config/mod.rs | 37 | ✓ |
