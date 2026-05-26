use brinkgit::store::{self, Database};
use std::fs;
use std::process::Command;
use tempfile::TempDir;

fn init_repo(root: &std::path::Path) {
    Command::new("git").args(["init"]).current_dir(root).status().unwrap();
    Command::new("git")
        .args(["config", "user.email", "t@e.com"])
        .current_dir(root)
        .status()
        .unwrap();
    Command::new("git")
        .args(["config", "user.name", "t"])
        .current_dir(root)
        .status()
        .unwrap();
    fs::write(root.join("f"), "x").unwrap();
    Command::new("git")
        .args(["add", "f"])
        .current_dir(root)
        .status()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "init"])
        .current_dir(root)
        .status()
        .unwrap();
    Command::new("git")
        .args(["branch", "-M", "feature/test"])
        .current_dir(root)
        .status()
        .unwrap();
}

fn brink_bin() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_BIN_EXE_brink"))
}

#[test]
fn set_get_list_has_roundtrip() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    init_repo(root);

    let bin = brink_bin();
    assert!(
        Command::new(&bin)
            .args(["set", "esa", "https://esa.io/posts/1"])
            .current_dir(root)
            .status()
            .unwrap()
            .success()
    );

    let get = Command::new(&bin)
        .args(["get", "esa"])
        .current_dir(root)
        .output()
        .unwrap();
    assert!(get.status.success());
    assert_eq!(String::from_utf8_lossy(&get.stdout), "https://esa.io/posts/1");

    assert!(
        !Command::new(&bin)
            .args(["has", "figma"])
            .current_dir(root)
            .status()
            .unwrap()
            .success()
    );

    let list = Command::new(&bin)
        .args(["list"])
        .current_dir(root)
        .output()
        .unwrap();
    assert!(list.status.success());
    let out = String::from_utf8_lossy(&list.stdout);
    assert!(out.starts_with("branch: feature/test\n"));
    assert!(out.contains("esa\thttps://esa.io/posts/1"));

    let data_path = root.join(".git").join("brink").join("data.json");
    let db: Database = store::load(&data_path).unwrap();
    assert_eq!(
        db.get("feature/test").unwrap().get("esa").unwrap(),
        "https://esa.io/posts/1"
    );
}

#[test]
fn get_missing_exits_one() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    init_repo(root);
    let out = Command::new(brink_bin())
        .args(["get", "esa"])
        .current_dir(root)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("hint: run `brink set esa"));
}
