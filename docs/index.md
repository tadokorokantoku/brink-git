# brink CLI documentation

Branch-scoped storage for issue URLs, esa, Figma, and other links.  
Data lives at `{git-common-dir}/brink/data.json` (not committed).

## Topics

Read a topic with `brink doc <topic>`:

| Topic | Command |
|-------|---------|
| Overview (storage, constraints) | `brink doc overview` |
| set | `brink doc set` |
| get | `brink doc get` |
| list | `brink doc list` |
| has | `brink doc has` |
| doc (this index) | `brink doc` |

## Quick reference

```bash
brink set <key> <value...>   # save on current branch
brink get <key>              # print value (exit 1 if missing)
brink has <key>              # exit 0 if set, 1 if not (no output)
brink list                   # human-readable table
brink list --json            # {"branch":"...","entries":{...}}
brink doc [topic]            # print Markdown docs for AI/agents
```

## For AI coding agents

1. Run `brink doc` or `brink doc overview` before using brink in a repository.
2. All subcommands except `doc` require a git repository and a checked-out branch (not detached HEAD).
3. Keys are arbitrary strings; values are opaque strings (often URLs).
4. Use `brink has <key>` for existence checks; use `brink get <key>` only when you need the value.
5. Re-run `brink doc <command>` when you need exact exit codes or examples for one command.
