---
"@fltsci/taurpc": patch
---

Type the per-method callbacks in the generated `TRANSFORM_MAP` against the corresponding method types so the file no longer trips `noImplicitAny` in strict-mode consumers.

Outer parameter stays `unknown` (preserves contravariant assignability to the runtime's `TransformFn = (value: unknown) => unknown`); the body narrows once via `const t = v as <method-type>` and the existing runtime expression uses the typed binding.
