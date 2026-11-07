# Tig evidence process

Design -> exact contract -> synthetic positive/negative fixtures -> fake-provider
failure runs -> reviewed live adoption. L0 structural checks do not prove L1 pure
semantics or L2 provider safety. No complete VTRACE conformance claim is made.

The full local baseline now includes mission/CONOPS, controlled requirements and
current/target specs, architecture/package/interface/design/code rigor, profiles,
implementation/work packages, integration/change/communications, evidence,
verification/validation/trace and scoped review. Its existence is not certification.

Evidence records source identity, inputs, policy/provider/tool pins, scope,
coverage, result and limitations. Raw logs and growing observations stay outside
ordinary Git; compact evidence summaries remain reviewable here.

Scenario feedback is mandatory: each case links to stable principle, invariant,
pitfall and requirement IDs plus a mitigation and accountable role. A new learning
can refine doctrine or add an entry; preserve previous failure memory and IDs.
Mitigation closure needs executable counterexamples and scoped positive/recovery
evidence, not merely a reviewed expected outcome. See docs/scenarios/LEARNINGS.md
and docs/scenarios/MITIGATIONS.md; all current full-system mitigations are planned.

## Adopted source technique and stage execution

TRACKER's portfolio-vtrace-scale skill requires one repo-local deliverable at a
time. Each stage is reviewed against the local roles, actionable design findings
are corrected in that deliverable, and remaining runtime proof is assigned to
named later work. STAGE_EXECUTION.md records that sequence and superseding reviews.
No separate agents or independent people were used for producer-applied checks.

VTRACE source: 7d4f7d25b07769f806e6ecfb973fd0461d43065f, framework process,
review-process, package-boundaries, implementation-management and adopt/assess/gate
skills. Read locally from the standards checkout; no Cargo/runtime dependency
on TRACKER is created. Its optional native structural assessment is:

```text
cargo run --locked --manifest-path <vtrace-checkout>/Cargo.toml -- validate <tig>
```

Portable tools/validate.ps1 remains the repo-owned development entry point and
does not require that checkout. Six local roles cover eight required design
review lanes, with method/limits stated in REVIEW.md. They do not grant upload,
deletion, publication authority or permission to substitute status for proof.

## Implementation discipline

Each package begins from a recorded main SHA on a bounded branch/worktree with
named parent IDs, allowed modules/dependencies and exact entry/exit/profile tests.
Close its own V: need -> requirement/spec -> design -> code -> verification ->
validation -> evidence/trace/review. Record exceptions and open risks explicitly.
Full-system acceptance cannot be inferred from a pure test, provider name,
reachable hash, Git merge, completed checklist or old authority snapshot.

## Stage review

Producer-applied six-role lenses: pass_with_risk. PR-01: full process adoption
could introduce a runtime orchestration framework or imply compliance. Resolved
by development-only records/commands, local source pins and explicit scope; no
new product management CLI or automatic operation is added.
