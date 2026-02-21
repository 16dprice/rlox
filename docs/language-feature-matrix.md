# Language Feature Matrix

Status definitions:
- `Supported`: implemented and covered by tests/examples.
- `Partial`: works with important caveats.
- `Not implemented`: tokenized and/or parsed surface is missing runtime/compiler support.

| Feature | Status | Notes | Example File |
| --- | --- | --- | --- |
| Number literals | Supported | Numeric constants compile/execute normally. | `data/examples/01_arithmetic.rlox` |
| String literals | Supported | String constants and string concatenation via `+`. | `data/examples/01_arithmetic.rlox` |
| Booleans and `nil` | Supported | Truthiness and equality comparisons implemented. | `src/vm.rs` tests |
| Arithmetic `+ - * /` | Supported | Numeric arithmetic + string concatenation behavior. | `data/examples/01_arithmetic.rlox` |
| Comparison/equality operators | Supported | `==`, `!=`, `<`, `<=`, `>`, `>=` emitted/executed. | `data/examples/02_control_flow.rlox` |
| Logical `and` / `or` | Supported | Short-circuit bytecode generation implemented. | n/a |
| Global variables | Supported | Declaration, read, and reassignment supported. | `data/elise_first_program.rlox` |
| Local variables / block scope | Partial | Scope works, but block scope currently emits internal `Local { ... }` debug lines from compiler instrumentation. | `data/easy_closure_example.rlox` |
| `if` / `else` | Supported | Conditional jump bytecode path implemented. | `data/examples/02_control_flow.rlox` |
| `while` | Supported | Loop jump emission and runtime execution implemented. | `data/examples/02_control_flow.rlox` |
| `for` | Partial | Loop semantics work; same debug-line caveat as scoped locals. | n/a |
| Functions and `return` | Supported | Function declarations, calls, arity checks, return values. | `data/examples/03_functions.rlox` |
| Closures / upvalues | Supported | Captures and mutation across calls are covered by VM tests. | `data/examples/04_closures.rlox` |
| Native function `clock()` | Supported | Returns epoch milliseconds as a number. | `data/fibonacci_test.rlox` |
| Class declarations | Supported | Empty class declaration and instantiation supported. | `data/examples/05_classes_properties.rlox` |
| Instance properties | Supported | Property set/get on instances supported. | `data/examples/05_classes_properties.rlox` |
| Methods on classes | Not implemented | Class body is currently parsed as empty braces only. | n/a |
| `this` | Not implemented | Tokenized by scanner, no compiler/runtime semantics. | n/a |
| `super` / inheritance | Not implemented | Tokenized by scanner, no inheritance/method dispatch semantics. | n/a |

## Notes for Contributors

If feature behavior changes, update this matrix in the same PR as code changes.
