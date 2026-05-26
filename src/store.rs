use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// branch name -> key -> value
pub type Database = BTreeMap<String, BTreeMap<String, String>>;

pub fn load(path: &Path) -> Result<Database> {
    if !path.exists() {
        return Ok(Database::new());
    }
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    if raw.trim().is_empty() {
        return Ok(Database::new());
    }
    let db: Database = serde_json::from_str(&raw)
        .with_context(|| format!("invalid JSON in {}", path.display()))?;
    Ok(db)
}

pub fn save(path: &Path, db: &Database) -> Result<()> {
    let dir = path.parent().context("data file has no parent directory")?;
    fs::create_dir_all(dir).with_context(|| format!("failed to create {}", dir.display()))?;
    let json = serde_json::to_string_pretty(db).context("failed to serialize data")?;
    fs::write(path, format!("{json}\n")).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

#[derive(Debug, Serialize)]
pub struct ListJson<'a> {
    pub branch: &'a str,
    pub entries: &'a BTreeMap<String, String>,
}
