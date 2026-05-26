use anyhow::{bail, Result};

const INDEX: &str = include_str!("../docs/index.md");
const OVERVIEW: &str = include_str!("../docs/overview.md");
const SET: &str = include_str!("../docs/set.md");
const GET: &str = include_str!("../docs/get.md");
const LIST: &str = include_str!("../docs/list.md");
const HAS: &str = include_str!("../docs/has.md");

const TOPICS: &[(&str, &str)] = &[
    ("overview", OVERVIEW),
    ("set", SET),
    ("get", GET),
    ("list", LIST),
    ("has", HAS),
];

/// Print Markdown documentation to stdout (AI-friendly; no git required).
pub fn print(topic: Option<&str>) -> Result<()> {
    let Some(raw) = topic else {
        print!("{INDEX}");
        return Ok(());
    };

    let topic = raw.trim().trim_start_matches('/').to_lowercase();

    if topic.is_empty() || topic == "index" || topic == "doc" || topic == "docs" {
        print!("{INDEX}");
        return Ok(());
    }

    for (name, content) in TOPICS {
        if topic == *name {
            print!("{content}");
            return Ok(());
        }
    }

    let available: Vec<&str> = std::iter::once("index")
        .chain(TOPICS.iter().map(|(n, _)| *n))
        .collect();
    bail!(
        "unknown topic \"{raw}\". Available: {}",
        available.join(", ")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_contains_quick_reference() {
        assert!(INDEX.contains("brink set"));
        assert!(INDEX.contains("brink doc"));
    }

    #[test]
    fn topic_set_documents_usage() {
        let err = print(Some("unknown-topic")).unwrap_err();
        assert!(err.to_string().contains("unknown topic"));

        print(Some("set")).unwrap();
    }
}
