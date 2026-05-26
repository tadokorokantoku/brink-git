# has

Check whether a key exists on the **current branch**. Produces **no output** on stdout or stderr.

## Usage

```bash
brink has <key>
```

## Examples

```bash
if brink has esa; then
  open "$(brink get esa)"
fi
```

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | Key is set |
| 1 | Key is not set, or other error (e.g. not a git repo, detached HEAD) |

## Comparison with `get`

| | `has` | `get` |
|---|-------|-------|
| Output | none | value on stdout |
| Missing key | exit 1, silent | exit 1, stderr + hint |
| Use when | shell conditionals | you need the value |
