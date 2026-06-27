use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Run rustfmt on all `.rs` files under `output_dir` so generated code matches `cargo fmt`.
pub fn rustfmt_generated_files(output_dir: &Path) -> Result<()> {
    let mut files = Vec::new();
    collect_rs_files(output_dir, &mut files)?;
    if files.is_empty() {
        return Ok(());
    }

    let output = Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .args(&files)
        .output()
        .context("failed to spawn rustfmt (is it installed?)")?;

    if !output.status.success() {
        anyhow::bail!(
            "rustfmt failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out)?;
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_rs_files_on_empty_dir_returns_nothing() {
        let dir = std::env::temp_dir().join(format!("automodel-rustfmt-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let mut files = Vec::new();
        collect_rs_files(&dir, &mut files).unwrap();
        assert!(files.is_empty());

        let _ = std::fs::remove_dir_all(&dir);
    }
}
