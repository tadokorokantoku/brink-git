use std::process::Command;

fn brink_bin() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_BIN_EXE_brink"))
}

#[test]
fn doc_works_outside_git_repo() {
    let tmp = tempfile::tempdir().unwrap();
    let out = Command::new(brink_bin())
        .args(["doc"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert!(out.status.success(), "stderr: {}", String::from_utf8_lossy(&out.stderr));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("# brink CLI documentation"));
    assert!(stdout.contains("brink doc overview"));
}

#[test]
fn doc_topic_set() {
    let tmp = tempfile::tempdir().unwrap();
    let out = Command::new(brink_bin())
        .args(["doc", "set"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("brink set <key>"));
}

#[test]
fn doc_unknown_topic_fails() {
    let out = Command::new(brink_bin())
        .args(["doc", "nope"])
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("unknown topic"));
}
