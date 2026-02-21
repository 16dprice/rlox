# AGENTS

This file defines repository-level expectations for coding agents working in `rlox`.

## Primary Goals

- Keep behavior correct and tested.
- Keep docs synchronized with runtime behavior.
- Prefer small, reviewable, focused changes.

## Guardrails

- Do not claim a feature is supported unless it is exercised by tests or runnable examples.
- Do not leave docs examples unverified after behavior changes.
- Do not modify generated/debug artifacts unless explicitly requested.

## Required Validation Before Finishing

```bash
cargo fmt
cargo test --quiet
```

## Docs Synchronization Rules

If behavior changes, update all relevant docs in the same change:

- `README.md`
- `docs/language-feature-matrix.md`
- `docs/examples.md` and affected files under `data/examples/`
- `CONTRIBUTING.md` when workflow/check expectations change

## Agent Docs

Detailed agent guidance:
- `docs/ai/workflows.md`
- `docs/ai/repo-conventions.md`
- `docs/ai/doc-maintenance.md`
