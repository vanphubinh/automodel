---
name: invalidate-generated
description: "Invalidate AutoModel generated code to force full regeneration on next build. USE FOR: forcing code regeneration, clearing stale generated output, resetting generated files, regenerating all queries, rebuilding generated code from scratch. DO NOT USE FOR: deleting source code, cleaning build artifacts (use cargo clean)."
argument-hint: "Optionally describe why regeneration is needed"
---

# Invalidate Generated Code

Remove the generated module index to force AutoModel to regenerate all code on the next `cargo build`.

## Procedure

```bash
rm example-app/src/generated/mod.rs
```

Then rebuild:

```bash
cargo build -p example-app
```

AutoModel's build script detects the missing `mod.rs` and regenerates all files in `example-app/src/generated/`.

**Note:** Invalidation is **not required** when SQL query files are changed. The build script automatically detects `.sql` file changes and regenerates code during `cargo build`.
