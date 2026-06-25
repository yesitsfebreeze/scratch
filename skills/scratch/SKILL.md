---
name: scratch
description: Use the scratch MCP tools instead of Read/Grep to explore code. Fn-level index — open_source for a function map, read_body for one function, search_bodies to grep, read_scratch/write_scratch for durable per-file memory. Edit the source file normally; the watcher re-splits.
---

# scratch

Fn-level code index over MCP. Each source file is split into per-function `.fs` bodies under `.scratch/`, and gets a `*.scratch.md` note for durable memory. Source is truth; `.scratch/` is a derived cache a one-way watcher rebuilds. Edit the source with normal tools — never the `.fs` files.

## Tools

- `index_dir(src_dir)` — bootstrap: split a whole tree. Run once if `.scratch/` is empty.
- `open_source(source_path)` — function list by size (⚠ over `SCRATCH_MAX_LOC`) + the file's scratch note.
- `read_body(path)` — one function body. First line is `§head <src>:<start>-<end> <name>`.
- `search_bodies(query)` — grep across every indexed function.
- `list_bodies(dir)` — functions in a dir, by size.
- `find_large()` — functions over `SCRATCH_MAX_LOC`.
- `read_scratch(source_path)` / `write_scratch(source_path, content, append)` — per-file memory.
- `list_languages()` — installed extensions (builtin `rs`, `py`; drop a WASM module for more).

## Use instead of

- `Read file` → `open_source`, then `read_body` for the parts you need.
- `Grep` → `search_bodies`.
- Editing → `read_body` for the `§head` source line range, then `Edit`/`Write` the **source** file; the watcher re-splits.
- Anything you learn about a file → `write_scratch`. Check `read_scratch` before exploring.

## Config (env vars or `scratch.ini`)

`SCRATCH_SRC_DIR=src` · `SCRATCH_EXT=rs` · `SCRATCH_MAX_LOC=256` · `SCRATCH_DEBOUNCE_MS=500`
