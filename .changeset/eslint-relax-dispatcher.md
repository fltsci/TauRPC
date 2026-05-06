---
"@fltsci/taurpc": patch
---

Disable the `@typescript-eslint/no-unsafe-*` rule family for `src/index.ts`.

The runtime proxy dispatcher receives serialized values, applies user-supplied transforms, and forwards them. Static types cannot describe that flow without defeating the purpose; the typed contract lives in generated Specta bindings, not the dispatcher. The rules stay active for the rest of the codebase.
