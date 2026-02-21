# AI Workflows

## 1. Feature Change Workflow

1. Inspect current behavior and tests.
2. Implement focused code changes.
3. Add or update tests.
4. Update docs (`README`, matrix, examples) if behavior changed.
5. Run validation:

```bash
cargo fmt
cargo test --quiet
```

## 2. Bugfix Workflow

1. Reproduce bug with a minimal script or test.
2. Add a failing test when possible.
3. Fix implementation.
4. Ensure new and existing tests pass.
5. Document any user-visible behavior change.

## 3. Docs-Only Workflow

1. Confirm current behavior using runnable commands.
2. Update docs with concrete commands/output.
3. If adding examples, add/update `data/examples/*.rlox` and docs example tests.
4. Run:

```bash
cargo test --quiet
```

## 4. Test-Only Workflow

1. Add regression/coverage tests without changing behavior.
2. Verify tests fail before behavior fix when testing a bug.
3. Run full suite and confirm deterministic results.
