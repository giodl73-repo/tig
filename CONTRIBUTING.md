# Contributing

Tig is experimental. Discuss public API/schema changes before relying on them;
no compatibility or MSRV policy is promised yet. Use a focused branch and PR.
Read AGENTS.md for repository controls and docs/ARCHITECTURE.md for boundaries.

## Development setup

Install stable Rust with rustfmt/Clippy, Git, Git LFS, PowerShell 7 and Python
3.10+. Rust 1.98.0 is the recorded local qualification toolchain. Python runs
the pinned development checkers and legacy helper tests, not the Rust runtime.
Vendored checkers need no adjacent portfolio repository.

```sh
cargo fmt --all -- --check
cargo test --workspace --all-features --locked
cargo clippy --workspace --all-features --all-targets --locked -- -D warnings
cargo check -p tig-git --no-default-features --locked
python -B tools/test_restore_project.py
pwsh -NoProfile -File tools/validate.ps1
git diff --check
```

CI exercises Windows and Linux. Tests use synthetic temporary stores and real
temporary Git/LFS repositories. Never point a test at personal media or production
stores. Select external target/temp locations when needed; keep build output
ignored. Full tests include a 500,000-binding workload and can take several minutes.

## Changes and review

Preserve identity domains, strict parsing, budgets, scoped concurrency and stale
operation fencing. Keep small authored state in Git; growing indexes and receipts
belong asset-side. Do not infer deletion from missing references or observations.
No provider operation, daemon or hooks should run merely on checkout/startup.

Tests should reproduce meaningful failures. Record exact verification scope;
producer-applied role checklists are not independent acceptance. Update affected
contracts, current architecture/roadmap and reviewed-file hashes. Run the validator
before committing; retain source/per-file budgets and vendored checker pins.

Share a minimal synthetic reproduction, affected version/platform, expected and
actual behavior, and relevant test output. Remove local paths, credentials, raw
enrollment, private prompts/media and consumer receipts from issues and PRs.
Keep project-specific adapters/profiles with their owners.

Original contributions use the repository MIT license. Retain upstream attribution
and license notices for copied/adapted code; dependencies require intake review.
See NOTICE.md. Report sensitive findings using SECURITY.md.
