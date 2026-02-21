# Examples

This repo includes two kinds of runnable examples:

- Curated docs examples in `data/examples/`.
- Existing historical/project examples in `data/`.

All commands below run via file mode and print output between the standard wrappers.

## Curated Examples (`data/examples`)

### 1. Arithmetic and Strings

File: `data/examples/01_arithmetic.rlox`

Run:

```bash
cargo run --quiet -- file data/examples/01_arithmetic.rlox
```

Expected program output:

```text
9
lox vm
```

### 2. Control Flow (`while`, `if/else`)

File: `data/examples/02_control_flow.rlox`

Run:

```bash
cargo run --quiet -- file data/examples/02_control_flow.rlox
```

Expected program output:

```text
ok
3
```

### 3. Functions and Returns

File: `data/examples/03_functions.rlox`

Run:

```bash
cargo run --quiet -- file data/examples/03_functions.rlox
```

Expected program output:

```text
25
7
```

### 4. Closures and Captured State

File: `data/examples/04_closures.rlox`

Run:

```bash
cargo run --quiet -- file data/examples/04_closures.rlox
```

Expected program output:

```text
1
2
1
3
```

### 5. Classes and Instance Properties

File: `data/examples/05_classes_properties.rlox`

Run:

```bash
cargo run --quiet -- file data/examples/05_classes_properties.rlox
```

Expected program output:

```text
13
```

## Existing Examples (`data/`)

### `data/test.rlox`

Simple class + properties sanity check.

```bash
cargo run --quiet -- file data/test.rlox
```

Expected program output:

```text
3
```

### `data/elise_first_program.rlox`

Basic global variable and string/number concatenation example.

```bash
cargo run --quiet -- file data/elise_first_program.rlox
```

Expected program output:

```text
I love you 3000
```

### `data/easy_closure_example.rlox`

Basic closure capture example.

```bash
cargo run --quiet -- file data/easy_closure_example.rlox
```

Expected program output:

```text
outside
```

### `data/working_closure_test.rlox`

Nested closure capture + mutation example.

```bash
cargo run --quiet -- file data/working_closure_test.rlox
```

Expected program output:

```text
7
8middleside
9middleside
10middleside
```
