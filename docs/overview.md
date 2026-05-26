# overview

## Purpose

`brink` associates arbitrary key/value pairs with the **current Git branch**. Typical keys: `issue`, `esa`, `figma`, `pr`.

## Storage

- **Path**: `{git-common-dir}/brink/data.json`
- **Resolved via**: `git rev-parse --git-common-dir`
- **Not committed**: files live under `.git`, outside the working tree
- **Shared across worktrees** of the same repository (common git dir)

### JSON shape

```json
{
  "feature/add-login": {
    "esa": "https://esa.io/posts/123",
    "issue": "https://github.com/org/repo/issues/42"
  }
}
```

Branch names are top-level keys. Each branch maps to an object of string keys and string values.

## Requirements

| Condition | Result |
|-----------|--------|
| Outside a git repository | Error |
| Detached HEAD (no branch name) | Error |
| Inside a repo, on a branch | Commands run against that branch |

Subdirectories of the repo are fine; Git discovers the repo from the current working directory.

## Behavior notes

- `set` on an existing key **overwrites** without confirmation.
- No built-in URL validation; any string is accepted.
- v1 has no `unset`; remove keys by editing `data.json` manually if needed.
