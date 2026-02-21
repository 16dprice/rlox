# Architecture

This project follows the bytecode VM architecture from the C implementation in
Crafting Interpreters, adapted to Rust.

## End-to-End Flow

1. Source text is tokenized by the scanner.
2. The compiler parses tokens and emits bytecode opcodes/constants into a `Chunk`.
3. The VM executes the chunk with a value stack, call frames, globals, and closures.

## Core Components

- `src/scanner.rs`
  - Produces `Token`s from source text.
  - Handles comments, whitespace, literals, identifiers, and keywords.

- `src/compiler.rs`
  - Pratt parser + bytecode emitter.
  - Handles declarations/statements/expressions.
  - Builds function chunks and closure metadata.

- `src/chunk.rs`
  - Defines `Chunk` and `OpCode`.
  - Stores `code`, `lines`, and `constants`.

- `src/vm.rs`
  - Executes bytecode instructions.
  - Manages globals, call frames, native functions, closure upvalues, classes/instances.

- `src/value.rs`
  - Runtime value model (`Number`, `String`, `Boolean`, `Function`, `Closure`, `Class`, `Instance`, etc.).

- `src/debug.rs`
  - Human-readable bytecode disassembly.
  - Debug file output used by `debug` CLI mode.
- `tools/rlox-lsp/`
  - Rust LSP server implementation for editor diagnostics/symbols.
- `vscode/rlox-vscode/`
  - Local VS Code extension (language registration, grammar, LSP client).

## Runtime Model

- Value stack: operand storage and expression evaluation.
- Call frames: active function calls with instruction pointers and stack slots.
- Globals table: global variable and native function storage.
- Open upvalues: tracks captured locals used by closures.
- Instances: per-object field maps keyed by property name.

## Entry Points

`src/main.rs` supports three modes:

- `file`: compile + run a `.rlox` source file.
- `repl`: interactive input loop.
- `debug`: compile and write disassembly to `data/debug.txt`.

See [`docs/running-and-debugging.md`](running-and-debugging.md) for commands.
