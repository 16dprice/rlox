# Contributing

Thanks for contributing to `rlox`.

## Prerequisites

- Rust stable toolchain
- `cargo` available on your shell path

## Local Setup

```bash
cargo fetch
cargo test --quiet
```

## Standard Workflow

1. Create a branch from `main`.
2. Make focused changes.
3. Run required checks:

```bash
cargo fmt
cargo test --quiet
```

4. Update docs when behavior changes.
5. Open a PR with a clear summary and validation notes.

## Required Validation

Before opening or updating a PR:

```bash
cargo fmt
cargo test --quiet
```

If your changes affect examples or docs output claims, also run:

```bash
cargo run --quiet -- file data/examples/01_arithmetic.rlox
cargo run --quiet -- file data/examples/02_control_flow.rlox
cargo run --quiet -- file data/examples/03_functions.rlox
cargo run --quiet -- file data/examples/04_closures.rlox
cargo run --quiet -- file data/examples/05_classes_properties.rlox
```

## Documentation Update Rules

When code changes behavior, update all relevant docs in the same PR:

- `README.md` for onboarding/usage changes
- `docs/language-feature-matrix.md` for feature support changes
- `docs/examples.md` and `data/examples/*.rlox` for example behavior
- `AGENTS.md` and `docs/ai/*` if workflow/policy for agents changes

## Adding or Updating Examples

Use `data/examples/` for curated docs examples.

Guidelines:
- Keep examples deterministic and minimal.
- Each file should demonstrate one primary concept.
- Include expected output in `docs/examples.md`.
- Ensure `tests/docs_examples.rs` validates new example output.

## PR Checklist

- [ ] Code compiles and tests pass (`cargo test --quiet`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Docs updated for behavior changes
- [ ] Feature matrix updated when support status changed
- [ ] New/changed docs examples validated by tests
