# Validation profiles

Profiles tailor package verification; they are not commands implemented by Tig.
All commands run in the package's isolated checkout with explicitly configured
shared Cargo target and admitted temporary scopes. CI uses Windows and Linux.

| Profile ID | Applicability | L0 | L1 | L2 |
| --- | --- | --- | --- | --- |
| PROFILE-001 (DOCS) | Authored baseline/discovery only | tools/validate.ps1; git diff --check; role/trace inspection | Not required for prose; source claims inspected | Not required; no runtime/acceptance claim |
| PROFILE-002 (RUST) | Pure core/typed algorithms | DOCS plus cargo fmt --all -- --check | cargo test --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; targeted negatives/vectors | Synthetic caller integration required at API boundary; provider IO not applicable |
| PROFILE-003 (PROVIDER) | Durable store/engine/worktree implementation | RUST L0 plus safety/capability review | RUST L1 plus fault-injected provider/journal/race tests | Isolated temporary stores, crash/replay/corruption/capacity/physical-byte assertions on declared platforms |
| PROFILE-004 (GIT) | Exact source extraction/LFS/ref lifecycle | RUST L0 plus upstream client/spec pins | RUST L1 plus pointer/index/source parsing and unsupported/malformed negatives | Temporary real Git/LFS repos, concurrency/hook bypass and explicit transfer endpoint fixtures; no user stores |
| PROFILE-005 (INTEGRATION) | Optional typed/build/delivery and consumer adapters | RUST L0 plus current source/authority/privacy review | RUST L1 plus deterministic fake producer/remote tests | End-to-end synthetic generic consumer and private owner-run acceptance before real adoption |
| PROFILE-006 (UI) | Inspector/WASM/provider-bound UI | RUST L0 plus no-private-data artifact checks | RUST L1 plus native/browser semantic parity and bounded reader tests | Read-only branch/auth/missing-object flows on synthetic data; no implicit local bridge |

Future test command names must be added to the package before implementation
closes; a desired test filter that matches zero tests is not passing evidence.
Record exact executed tests, source, toolchain, workload and limits. Do not use
wildcard production cache access to make an integration test pass.

## Stage review

Producer-applied six-role lenses: pass_with_risk. LP-01: a generic Cargo command
alone could hide missing provider tests. Resolved by explicit L2 obligations,
zero-test rejection and isolated scopes; numerical workloads and exact new test
names are required before each implementation package is closed.

Native optional-feature slice additionally requires cargo test --workspace
--all-features --locked and cargo clippy --workspace --all-features --all-targets
--locked -- -D warnings. Check cargo check -p tig-git --no-default-features --locked
to preserve standalone Git/LFS. Default workspace tests alone skip native adapter
fixtures and cannot qualify this package.

LP-02 compatibility follow-up: canonical VTRACE discovery requires Profile ID /
PROFILE identifiers. Added stable tooling IDs while retaining the package-facing
short aliases; no test requirements or claimed results were weakened.
