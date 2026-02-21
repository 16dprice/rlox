# AI Repo Conventions

## Code and Project Layout

- Runtime/compiler/scanner logic lives in `src/`.
- Sample programs live in `data/`.
- Curated documentation examples live in `data/examples/`.
- High-level docs live in `README.md` and `docs/`.

## Editing Conventions

- Keep changes focused; avoid broad unrelated refactors.
- Prefer explicit behavior-preserving changes when doing cleanup.
- Avoid introducing dependencies unless clearly justified.

## Tests

- Unit and integration tests are run via `cargo test`.
- Docs examples are verified by `tests/docs_examples.rs`.
- New behavior should have either a unit test, VM test, integration test, or all three as appropriate.

## Documentation Conventions

- Use concrete runnable commands.
- Prefer deterministic examples.
- Keep feature claims aligned with `docs/language-feature-matrix.md`.

## Output/Artifacts

- Do not commit generated output changes (for example `data/debug.txt`) unless explicitly requested.
