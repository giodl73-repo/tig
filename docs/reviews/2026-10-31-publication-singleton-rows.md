# Singleton receipt object review

EVID-075; WP-010/011/015; REQ-001/006/010; SPEC-010/012/015;
IF-010/011/013; PKG-006/008. Entry is qualified main
`dae494d9bac18456020e90043b2764e9225c1d6e`. RUST and INTEGRATION profiles.
Scope: optional pure publication checker, explicit CLI tests and contract/trace.
No provider or placement code, private fixture, cache or runtime binding changes.

Need: a valid receipt may describe one delivered subject at its root. Rewriting
it into an array would replace the original evidence pin. An explicit
`single-row` policy instead selects exactly that object; fields remain relative
to the selected row. Empty JSON Pointer selects the root. Nested objects and a
member with an empty name retain distinct exact selectors. An authored schema
that forbids an empty selector still refuses; the checker grants no exemption.

The first synthetic root-receipt test failed at policy admission on the entry
implementation. The implemented rule passes that test without changing evidence
bytes, target identity domains or any existing collection rule. Invalid member-key
use refuses because a singleton has no selected dictionary member. Overlapping
roots, unknown rule fields, descendant fallback, nonobject values and bad subject
or target fields refuse. Parsed evidence reuse still applies each policy's own
limits. Existing pointer-string targets never fall back to opaque locators.

L0 requires foundation, format, diff and strict Clippy. L1 focused publication
verification passes 29 cases, including five new singleton cases and all existing
cases. L2 extends the actual compiled CLI/privacy case to a fourth receipt shape;
successful checks and corrupt/extra-argument failures write no unexpected files
or raw private fixture strings. Full workspace, standalone Git/LFS and hosted
qualification are separate checks, recorded when complete.

Full local qualification now passes: locked all-feature workspace tests, strict
all-target/all-feature Clippy (54.30 s), standalone Git/LFS check (14.44 s),
foundation, formatting and whitespace. Reviewed target pins are refreshed only
for changed code/contract/trace files; runtime proof and source budgets are not
relaxed. Hosted and actual consumer gates remain open.

Producer-applied role findings:

- Identity/namespace: exact root equality preserves JSON Pointer semantics;
  equal targets across array and singleton shapes share the target identity while
  distinct evidence policies remain pinned. Disposition: pass for this scope.
- Provider/privacy: opaque addresses remain uninterpreted and redacted; only
  explicit local metadata files are opened by the existing CLI. Pass for this
  scope; no provider identity or current-publication proof is asserted.
- Storage/custody: all custody/eviction/payload assertions remain false. No byte
  movement or recovery policy is changed. Full storage acceptance stays open.
- Git/adoption: preserve original receipts and bounded records; older tools refuse
  the unknown rule. Projects must version any selector-schema change explicitly.
  Actual consumer qualification and production installation remain open.

CR-004 focused justification: the existing bounded `check_parsed` evaluator stays
one unit so all row forms share subject/length/target validation and report flags.
The added rule only changes exact row selection; it introduces no IO, recursion,
new traversal or state transition. All policy roots retain the sixteen-rule bound.
These are producer findings, not independent review or human approval. Owner:
Tig maintainers. No work package, full requirement or intended-use cutover closes
from this component evidence.
