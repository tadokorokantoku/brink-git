use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct GitContext {
    pub common_dir: PathBuf,
    pub branch: String,
}

pub fn discover() -> Result<GitContext> {
    let common_dir = git_output(&["rev-parse", "--git-common-dir"])
        .context("not inside a git repository")?;
    let common_dir = PathBuf::from(common_dir.trim());
    if !common_dir.is_absolute() {
        let cwd = std::env::current_dir()?;
        let common_dir = cwd.join(common_dir);
        return discover_with_common_dir(common_dir);
    }
    discover_with_common_dir(common_dir)
}

fn discover_with_common_dir(common_dir: PathBuf) -> Result<GitContext> {
    let branch = git_output(&["branch", "--show-current"])
        .context("failed to read current branch")?;
    let branch = branch.trim().to_string();
    if branch.is_empty() {
        bail!("not on a branch (detached HEAD)");
    }
    Ok(GitContext { common_dir, branch })
}

pub fn data_file(common_dir: &Path) -> PathBuf {
    common_dir.join("brink").join("data.json")
}

fn git_output(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .with_context(|| format!("failed to run `git {}`", args.join(" ")))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git {} failed: {}", args.join(" "), stderr.trim());
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command;
    use tempfile::TempDir;

    #[test]
    fn data_file_joins_under_common_dir() {
        let p = data_file(Path::new("/tmp/repo.git"));
        assert_eq!(p, PathBuf::from("/tmp/repo.git/brink/data.json"));
    }

    #[test]
    fn discover_reads_branch_in_temp_repo() -> Result<()> {
        let tmp = TempDir::new()?;
        let root = tmp.path();
        Command::new("git").args(["init"]).current_dir(root).output()?;
        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(root)
            .output()?;
        Command::new("git")
            .args(["config", "user.name", "test"])
            .current_dir(root)
            .output()?;
        fs::write(root.join("f"), "x")?;
        Command::new("git")
            .args(["add", "f"])
            .current_dir(root)
            .output()?;
        Command::new("git")
            .args(["commit", "-m", "init"])
            .current_dir(root)
            .output()?;
        Command::new("git")
            .args(["branch", "-M", "main"])
            .current_dir(root)
            .output()?;

        std::env::set_current_dir(root)?;
        let ctx = discover()?;
        assert_eq!(ctx.branch, "main");
        assert!(ctx.common_dir.ends_with(".git"));
        Ok(())
    }
}
