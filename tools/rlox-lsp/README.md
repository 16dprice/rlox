# rlox-lsp

Rust-based Language Server Protocol implementation for `rlox`.

## Current Capabilities

- Scanner-based diagnostics (`TokenType::Error`)
- Document symbols for top-level `fun`, `class`, and `var` declarations

## Build

```bash
cargo build --manifest-path tools/rlox-lsp/Cargo.toml
```

## Run

The server communicates over stdio and is intended to be launched by an editor client.
