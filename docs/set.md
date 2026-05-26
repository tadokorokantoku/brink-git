# set

Save a key/value pair on the **current branch**.

## Usage

```bash
brink set <key> <value...>
```

All arguments after `<key>` are joined with a single space into the value.

## Examples

```bash
brink set esa https://esa.io/posts/123
brink set issue https://github.com/org/repo/issues/42
brink set note "contains spaces"
```

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | Saved successfully |
| 1 | Error (not a git repo, detached HEAD, missing value, I/O error) |

## Notes

- Overwrites an existing value for the same key on the same branch.
- Does not validate URLs or key names.
