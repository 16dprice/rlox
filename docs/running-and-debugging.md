# Running and Debugging

## Build and Test

```bash
cargo test --quiet
```

## Run Modes

The binary supports three modes from `src/main.rs`.

### 1. File Mode

Run a specific program:

```bash
cargo run --quiet -- file data/test.rlox
```

If no file path is supplied in file mode, it defaults to `data/test.rlox`.

### 2. REPL Mode

```bash
cargo run --quiet -- repl
```

Type `quit` to exit.

### 3. Debug Mode

Compile and disassemble a source file to `data/debug.txt`:

```bash
cargo run --quiet -- debug data/test.rlox
```

This writes opcode-level output with source line context.

## Program Output Format

`file` mode wraps program output between markers:

- `==== BEGIN PROGRAM OUTPUT ====`
- `==== END PROGRAM OUTPUT ====`

This wrapper is expected and is used by documentation example tests.

## Common Commands

Run curated examples:

```bash
cargo run --quiet -- file data/examples/01_arithmetic.rlox
cargo run --quiet -- file data/examples/02_control_flow.rlox
cargo run --quiet -- file data/examples/03_functions.rlox
cargo run --quiet -- file data/examples/04_closures.rlox
cargo run --quiet -- file data/examples/05_classes_properties.rlox
```
