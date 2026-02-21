# Documentation Maintenance Rules

When implementation changes behavior, update docs in this order.

## 1. User Entry Points

Update `README.md` if any of these changed:
- How to run the project
- Supported modes/commands
- High-level project status

## 2. Feature Truth Source

Update `docs/language-feature-matrix.md` if support status changed:
- `Supported`
- `Partial`
- `Not implemented`

Each changed row should include a concrete note and example reference.

## 3. Examples

If behavior/output changed:
- Update affected file(s) in `data/examples/`
- Update expected output blocks in `docs/examples.md`
- Update or extend `tests/docs_examples.rs`

## 4. Contributor/Agent Process

Update `CONTRIBUTING.md` and `AGENTS.md` when validation requirements,
workflow expectations, or documentation rules change.

## 5. Validation

Before finalizing docs updates:

```bash
cargo fmt
cargo test --quiet
```
