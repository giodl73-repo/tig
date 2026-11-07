# Validation

```powershell
pwsh -NoProfile -File tools/validate.ps1
git diff --check
```

PowerShell 7 and Python 3 are development tools only. The validator checks required
files, local Markdown links, scenario structure, pinned checker hashes and runs
vendored ROLES/PITFALL checkers. It performs no cache/provider/network operations.
Scenario inventories cover 110 cases: 25 foundation, ten cache/LFS, fifteen
Git-workflow and six ten-case adversarial/lifecycle suites. Each case must link
to a known principle, invariant, pitfall, owned mitigation and requirement;
checking their structure does not execute their behavior.
This is structural evidence, not placement behavior, complete VTRACE conformance,
independent review or adoption approval. CI runs on Windows and Linux.

Required Rust checks: cargo fmt --all -- --check; cargo test --workspace --locked;
cargo clippy --workspace --all-targets --locked -- -D warnings. Use an explicitly
configured shared Cargo target; no per-worktree cache/staging tree.

The dependency-free tig-core prototype has 13 synthetic handoff tests. These
exercise pure set/transition mechanisms, not Git/LFS extraction, durable commit,
provider IO, independent custody or a complete designed case. CI executes them
on Windows and Linux separately from structural validation.

The portable validator also checks the full local VTRACE artifact inventory,
ten requirement aliases, fifteen spec rows, seventeen proposed packages, complete
parent allocations and acyclic package prerequisites. It checks design structure,
not implementation readiness. Optional canonical native VTRACE assessment and
its pinned source are documented in docs/vtrace/PROCESS.md.
