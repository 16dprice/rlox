# VS Code Language Support

This repository includes a local VS Code extension and a Rust LSP server for `.rlox` files.

## Components

- VS Code extension: `vscode/rlox-vscode/`
- Rust language server: `tools/rlox-lsp/`

Current capabilities:
- TextMate syntax highlighting
- Basic diagnostics from scanner error tokens
- Document symbols for top-level `fun`, `class`, and `var`

## 1. Build the Language Server

```bash
cargo build --manifest-path tools/rlox-lsp/Cargo.toml
```

This produces:

- `tools/rlox-lsp/target/debug/rlox-lsp` (or `.exe` on Windows)

## 2. Build the VS Code Extension

```bash
cd vscode/rlox-vscode
npm install
npm run compile
```

## 3. Run in Extension Development Host

1. Open `vscode/rlox-vscode` in VS Code.
2. Press `F5` to launch an Extension Development Host.
3. Open any `.rlox` file to activate the extension.

## 4. Configure Server Path (Optional)

By default, the extension resolves the language server at:

`tools/rlox-lsp/target/debug/rlox-lsp`

You can override it in VS Code settings:

```json
{
  "rlox.languageServerPath": "/absolute/path/to/rlox-lsp"
}
```

## Troubleshooting

- If diagnostics/symbols do not appear, run command:
  - `Rlox: Restart Language Server`
- If server fails to launch, verify binary path and executable permissions.
- Rebuild extension after source changes:

```bash
npm run compile
```
