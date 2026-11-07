# Art value intake and Tig specification gaps

Scope: source/design audit and bounded canonicalizer extraction. Art's generic value is folded into
Tig's optional typed layer; media/native production stays adapter-owned. Exact
source pins and licenses are required before code moves. No private consumer
records/media are copied into Tig.

Inspected Art source: 119c66446f2e95129d8d8f143d60ec6220d2813a. Its 19-item
current/target baseline and crate/reuse/variant documents informed this mapping.

| Art capability | Observed source maturity | Tig destination / controlled package |
| --- | --- | --- |
| Strict canonical JSON, digest domains, exact record/contract locks | Implemented experimental pure mechanisms | WP-001/010; preserve hash vectors and distinguish byte pins from semantic revisions |
| Consumed-property recipe fingerprints and produced-result correspondence | Implemented pure claims, not trusted execution | WP-010/011; preserve unrelated-change reuse and separate provenance/freshness/selection |
| Generic revision-bound status vocabulary | Implemented pure subject checks, not full authority | WP-010/012; no inherited approval on changed subjects; withdrawals remain target |
| Typed declared graph, budgets, coverage and reverse candidates | Implemented bounded pure queries | WP-010/011; no unknown-as-empty or automatic defect/custody inference |
| Sparse variant index, exact owner/definition/context pins | Implemented pure resolver | WP-010/011; no silent fallback or Cartesian duplication |
| Native reference extraction and first-class text/translation | Designed; not integrated | WP-011; complete consumed references and staged translation/recording invalidation |
| Profiles, budgets, generic bindings and make-style builds | Designed; no complete producer engine | WP-011; source merges can remain valid with stale outputs and catch up later |
| Evaluation/rubric gates, issue triage and delivery reconciliation | Designed; no live connectors | WP-012; exact subject, trusted policy, desired/observed target and authorized issue writes |
| Read-only project/profile WASM viewer | Planned | WP-015; same pure model, explicit providers and private-data boundaries |
| Registry/index/version/job/demotion extraction from REEL | Audited candidates, not imported infrastructure | WP-000/003-009/013; retain existing safeguards until same-behavior fixtures pass |

The Art pure suite has 73 mechanism tests (34 canonical/model, 25 graph including
two internal guards, 14 variant) plus two development-checker tests. They are not
Tig tests until extraction/parity actually runs here. WP-001 now ports ten
canonical/hash vectors and adds sixteen tree tests alongside thirteen placement
tests (39 total). Graph/model/variant extraction remains future work; the donor
suite is not evidence of a unified runtime. See specs/IDENTITY-AND-TREES.md.

## Is the specification complete?

The engineering baseline now covers needs, requirements, current/target specs,
interfaces, design, packages and proof obligations. It is not a frozen release:

| Still to resolve / prove | Responsible scope |
| --- | --- |
| Native persistent encoding, canonical/tree identity and supported path rules | WP-001/002/010 |
| Backend choice, publication durability and recoverable protection/fencing | WP-000/003/004 |
| Real Git/LFS interoperability, ownership and external pruning coordination | WP-005/009 |
| Independent custody, actual capacity/reclaim and shared lookup budgets | WP-006/007/008/013 |
| Complete semantic/native closures, merge checks and deferred build planning | WP-010/011/012 |
| Real delivery/provider capabilities and reversible private consumer adoption | WP-012/013/016 |

The design-1 schemas are a bounded fixture proposal for separate definition and
instance files. They exercise declared shapes and exact byte bindings without
claiming released identity semantics or execution. A private consumer recast can
validate metadata wiring, but planned records with no selected media cannot prove
full renderability, primary-input custody or production compatibility.

Consumed-input mechanism intake (EVID-044) adapts the donor model's behavioral
separation between exact provenance and consumed-value recipe identity to native
Tig graphs. Annotation/status/unconsumed edits, explicit/default equivalence,
consumed-value/default/producer changes and frozen input behavior now execute in
Tig tests. The hash format is new and explicitly domain-separated; this is neither
legacy Art format import nor complete model/hash/variant parity. Current native
payload/recursive graph bindings remain the single input-reference authority.

Native produced-claim follow-up (EVID-045) adapts the donor's exact original-plan
and selected-result correspondence behavior without importing its record format.
Tig additionally preserves historical bytes without claims as explicitly Unproven,
uses independent current-subject admission and checks native frozen context/opaque
parameters. The matching state is deliberately ClaimedRecipeMatch; full producer
execution, approval/authority, model/variant parity and custody remain unproven.

The optional native variants module and its15 behavior cases adapt Art's pinned
variants.rs/tests/variants.rs to native Object/definition/stream slots, with new
wire/hash identities. Fourteen donor behaviors remain individually exercised;
Tig adds independent validated project context so unused project-space extensions
need not rewrite old neutral records. Initial native Git/recipe/archive fixtures
execute. This is not Art wire/hash compatibility, full model/variant integration,
producer proof, private acceptance or custody. See NATIVE-VARIANTS.md.
