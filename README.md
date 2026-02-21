# rlox

`rlox` is a Rust implementation of the bytecode VM version of Lox from
[Crafting Interpreters](https://craftinginterpreters.com/).

It includes a scanner, Pratt parser/compiler, bytecode chunk format, and VM runtime,
plus sample `.rlox` programs and tests.

## Current Status

The project currently supports a substantial subset of Lox,
including variables, control flow, functions, closures, and basic classes with instance fields.

For detailed support status, see:
- [`docs/language-feature-matrix.md`](docs/language-feature-matrix.md)

## Quick Start

Prerequisites:
- Rust stable toolchain

Install/build dependencies:

```bash
cargo fetch
```

Run default sample program (`data/test.rlox`):

```bash
cargo run --quiet
```

Run a specific file:

```bash
cargo run --quiet -- file data/test.rlox
```

Run REPL mode:

```bash
cargo run --quiet -- repl
```

Generate bytecode debug output in `data/debug.txt`:

```bash
cargo run --quiet -- debug data/test.rlox
```

## Repository Map

- `src/scanner.rs`: tokenization
- `src/compiler.rs`: parsing + bytecode emission
- `src/chunk.rs`: opcodes, code bytes, constants
- `src/vm.rs`: bytecode execution runtime
- `src/value.rs`: runtime value/function/class representations
- `src/debug.rs`: disassembly and debug output helpers
- `data/`: sample programs and debug artifacts
- `data/examples/`: curated docs-focused runnable examples
- `tests/docs_examples.rs`: integration tests for documented examples
- `tools/rlox-lsp/`: Rust language server for editor integration
- `vscode/rlox-vscode/`: local VS Code extension for `.rlox`

## Supported Language Features

See the full matrix:
- [`docs/language-feature-matrix.md`](docs/language-feature-matrix.md)

High-level examples of currently supported behavior:
- arithmetic and string concatenation
- global/local variables and assignment
- `if`, `while`, `for`
- function declaration/call/return
- closures and captured state
- class declarations and instance properties

## Examples

Runnable examples and expected outputs:
- [`docs/examples.md`](docs/examples.md)

## Development Commands

Run tests:

```bash
cargo test --quiet
```

Format code:

```bash
cargo fmt
```

## VS Code Language Support

Local VS Code language support (syntax highlighting + basic LSP) is available in:

- [`vscode/rlox-vscode/`](vscode/rlox-vscode)

Setup and usage guide:

- [`docs/vscode-language-support.md`](docs/vscode-language-support.md)

## Contributing

See:
- [`CONTRIBUTING.md`](CONTRIBUTING.md)

## For Agents

See:
- [`AGENTS.md`](AGENTS.md)
- [`docs/ai/workflows.md`](docs/ai/workflows.md)
- [`docs/ai/repo-conventions.md`](docs/ai/repo-conventions.md)
- [`docs/ai/doc-maintenance.md`](docs/ai/doc-maintenance.md)
