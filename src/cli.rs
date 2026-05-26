use brinkgit::doc;
use brinkgit::git;
use brinkgit::store::{self, Database, ListJson};
use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use std::collections::BTreeMap;

#[derive(Parser)]
#[command(
    name = "brink",
    about = "Manage per-branch links to issues and documents",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set a key on the current branch (value is all remaining arguments)
    Set {
        key: String,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        value: Vec<String>,
    },
    /// Print the value for a key on the current branch
    Get { key: String },
    /// List all keys on the current branch
    List {
        /// Output JSON: { "branch": "...", "entries": { ... } }
        #[arg(long)]
        json: bool,
    },
    /// Exit 0 if the key exists on the current branch, else 1 (no output)
    Has { key: String },
    /// Print CLI documentation as Markdown (for humans and AI agents)
    Doc {
        /// Topic: overview, set, get, list, has (omit for index)
        #[arg(value_name = "TOPIC")]
        topic: Option<String>,
    },
}

pub fn run(cli: Cli) -> Result<()> {
    if let Commands::Doc { topic } = &cli.command {
        doc::print(topic.as_deref())?;
        return Ok(());
    }

    let ctx = git::discover()?;
    let path = git::data_file(&ctx.common_dir);
    let mut db = store::load(&path)?;

    match cli.command {
        Commands::Doc { .. } => unreachable!(),
        Commands::Set { key, value } => {
            if value.is_empty() {
                bail!("value is required: brink set <key> <value>");
            }
            let value = value.join(" ");
            let branch = db.entry(ctx.branch.clone()).or_default();
            branch.insert(key, value);
            store::save(&path, &db)?;
        }
        Commands::Get { key } => {
            match get_entry(&db, &ctx.branch, &key) {
                Some(v) => {
                    print!("{v}");
                }
                None => {
                    eprintln!(
                        "brink: key \"{key}\" is not set on branch \"{}\"",
                        ctx.branch
                    );
                    eprintln!("hint: run `brink set {key} <value>`");
                    std::process::exit(1);
                }
            }
        }
        Commands::List { json } => {
            let empty = BTreeMap::new();
            let entries = branch_entries(&db, &ctx.branch, &empty);
            if json {
                let out = ListJson {
                    branch: &ctx.branch,
                    entries,
                };
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else {
                println!("branch: {}", ctx.branch);
                for (k, v) in entries {
                    println!("{k}\t{v}");
                }
            }
        }
        Commands::Has { key } => {
            if get_entry(&db, &ctx.branch, &key).is_some() {
                return Ok(());
            }
            std::process::exit(1);
        }
    }

    Ok(())
}

fn branch_entries<'a>(db: &'a Database, branch: &str, empty: &'a BTreeMap<String, String>) -> &'a BTreeMap<String, String> {
    db.get(branch).unwrap_or(empty)
}

fn get_entry<'a>(db: &'a Database, branch: &str, key: &str) -> Option<&'a String> {
    db.get(branch).and_then(|m| m.get(key))
}
