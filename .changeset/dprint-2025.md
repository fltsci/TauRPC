---
"@fltsci/taurpc": patch
---

Bump dprint to current 2025 plugin/CLI versions and relax `operatorPosition` to `maintain`.

Plugin/CLI bumps:

- dprint CLI 0.50.2 -> 0.54.0
- dprint-plugin-typescript 0.85.1 -> 0.96.0
- dprint-plugin-json 0.17.4 -> 0.21.3
- dprint-plugin-markdown 0.15.3 -> 0.21.1
- dprint-plugin-exec 0.4.3 -> 0.6.2

Config tweak: `binaryExpression.operatorPosition`, `conditionalExpression.operatorPosition`, and `conditionalType.operatorPosition` are set to `maintain`. The previous default (`nextLine`) forced a specific multi-line style for `&&`, `||`, and ternaries. `maintain` accepts whatever the writer used, which is friendlier for external contributors and removes a category of "works locally, fails CI" drift.

No source files reformat as a result of these changes; running `dprint fmt` on main is a no-op after the bump.
