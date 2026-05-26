# list

List all keys and values stored on the **current branch**.

## Usage

```bash
brink list
brink list --json
```

## Human output (default)

First line is the branch name, then one entry per line (tab-separated):

```text
branch: feature/add-login
esa     https://esa.io/posts/123
issue   https://github.com/org/repo/issues/42
```

If nothing is set on the branch, only the `branch:` line is printed.

## JSON output (`--json`)

```json
{
  "branch": "feature/add-login",
  "entries": {
    "esa": "https://esa.io/posts/123",
    "issue": "https://github.com/org/repo/issues/42"
  }
}
```

`entries` is an empty object when no keys are set.

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error (not a git repo, detached HEAD, I/O error) |
