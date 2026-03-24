---
name: cargo-release
description: "Publish automodel crates to crates.io. USE FOR: releasing new versions, bumping version numbers, publishing automodel (lib) and automodel-cli to crates.io, preparing a release. DO NOT USE FOR: building locally, running tests, CI/CD pipeline setup."
argument-hint: "Specify new version number (e.g. 0.6.0) or 'patch'/'minor'/'major'"
---

# Cargo Release — Publishing to crates.io

## Crates

| Crate | Directory | crates.io name | Role |
|-------|-----------|----------------|------|
| automodel | `automodel-lib/` | `automodel` | Core library |
| automodel-cli | `automodel-cli/` | `automodel-cli` | CLI binary |

The CLI depends on the lib, so **the lib must be published first**.

## Prerequisites

- `cargo login` must have been run with a valid crates.io API token
- All changes committed and pushed
- Tests passing (use the **testing** skill to verify)

## Procedure

### 1. Decide the new version

Follow semver. Current version is in `automodel-lib/Cargo.toml` and `automodel-cli/Cargo.toml` (should match).

### 2. Bump versions

Update the version in **both** crate Cargo.toml files to the same value:

- `automodel-lib/Cargo.toml` → `version = "X.Y.Z"`
- `automodel-cli/Cargo.toml` → `version = "X.Y.Z"`

Also update the CLI's dependency on the lib if needed:

- `automodel-cli/Cargo.toml` → `automodel = { path = "../automodel-lib", version = "X" }` (the `version` field must be compatible with the new version)

### 3. Pre-publish checks

```bash
# Ensure everything compiles
cargo check --workspace

# Dry-run package both crates to catch issues before publishing
cargo package -p automodel --allow-dirty
cargo package -p automodel-cli --allow-dirty
```

Review any warnings. Fix before proceeding.

### 4. Commit the version bump

Commit all changes **before** publishing so `cargo publish` works without `--allow-dirty`:

```bash
git add -A
git commit -m "release: vX.Y.Z"
```

### 5. Temporarily comment out `[patch.crates-io]`

The workspace `Cargo.toml` has a `[patch.crates-io]` section that redirects `automodel` to the local path. **`cargo publish` ignores `[patch]`**, so this does not block publishing. However, if you see resolution errors, temporarily comment it out:

```toml
# [patch.crates-io]
# automodel = { path = "automodel-lib" }
```

Restore it after publishing.

### 6. Publish the lib first

```bash
cargo publish -p automodel
```

### 7. Publish the CLI

```bash
cargo publish -p automodel-cli
```

### 8. Tag and push

If you commented out `[patch.crates-io]`, restore and amend the commit.

```bash
git tag vX.Y.Z
git push && git push --tags
```

## Quick Reference (copy-paste)

Replace `X.Y.Z` with the actual version:

```bash
# 1. Verify
cargo check --workspace

# 2. Dry-run
cargo package -p automodel --allow-dirty
cargo package -p automodel-cli --allow-dirty

# 3. Commit
git add -A && git commit -m "release: vX.Y.Z"

# 4. Publish (lib first!)
cargo publish -p automodel
cargo publish -p automodel-cli

# 5. Tag & push
git tag vX.Y.Z
git push && git push --tags
```

## Troubleshooting

- **"crate version X.Y.Z is already uploaded"**: The version already exists on crates.io. Bump to a new version.
- **"not logged in"**: Run `cargo login` with your crates.io API token.
- **Packaging warnings about missing files**: Ensure `README.md` exists at the path specified in `readme = "../README.md"` relative to each crate.
