# get

Print the value for a key on the **current branch** to stdout.

## Usage

```bash
brink get <key>
```

## Examples

```bash
url=$(brink get esa)
open "$(brink get esa)"
```

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | Key exists; value written to stdout (no trailing newline added beyond the stored value) |
| 1 | Key not set, or other error |

## Missing key

When the key is not set on the current branch:

- stderr: `brink: key "<key>" is not set on branch "<branch>"`
- stderr: `hint: run \`brink set <key> <value>\``
- exit code: **1**
- stdout: empty

Use `brink has <key>` if you only need an existence check without stderr output.
