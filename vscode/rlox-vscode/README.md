# rlox VS Code Extension (Local)

Local VS Code extension for `.rlox` files.

## Features

- Language registration for `.rlox`
- TextMate syntax highlighting
- Rust LSP client integration (diagnostics + document symbols)
- Command: `Rlox: Restart Language Server`

## Setup

From `vscode/rlox-vscode`:

```bash
npm install
npm run compile
```

Build the language server:

```bash
cargo build --manifest-path tools/rlox-lsp/Cargo.toml
```

Then open this extension folder in VS Code and press `F5` to launch an Extension Development Host.

## Language Server Path

By default, the extension looks for:

`tools/rlox-lsp/target/debug/rlox-lsp`

You can override this in VS Code settings:

```json
{
  "rlox.languageServerPath": "/absolute/path/to/rlox-lsp"
}
```
