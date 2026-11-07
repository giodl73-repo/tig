# Product work packages

Seventeen bounded packages, including one explicit discovery package. These are
product capabilities and their proof obligations, not a process-framework backlog.

Statuses track each full target scope and its admitted entry/exit, not whether
any component code exists. Proposed packages can contain bounded implementations
whose evidence does not close their prerequisites or full acceptance. See
[current architecture](../ARCHITECTURE.md) for implemented slices.

| ID | Objective | Parent IDs | Entry Criteria | Exit Criteria | L0 / L1 / L2 | Status |
| --- | --- | --- | --- | --- | --- | --- |
| WP-000 | Choose a placement backend with demonstrated local/offline fit | REQ-006; SPEC-002/SPEC-003/SPEC-004/SPEC-005/SPEC-009 | Bounded backend questions and synthetic workload agreed | Recorded iRODS/Rucio/minimal-Rust capability, footprint, recovery and portability findings; choose or defer each capability explicitly | L0: DOCS/format; L1: DOCS profile; L2: not required, discovery only | discovery-recorded |
| WP-001 | Resolve exact immutable objects and bounded tree paths | REQ-001/REQ-005/REQ-006; SPEC-002 | Current core regression baseline and hash/encoding decision scope pinned | Canonical/raw identity vectors, path and size negatives, bounded tree lookup and sparse-change cost demonstrated | L0: DOCS/format; L1: RUST profile; L2: synthetic caller integration, no provider claim | verified-pure |
| WP-002 | Open and seal private byte worktree changes | REQ-001/REQ-004/REQ-009; SPEC-006 | Frozen object/tree semantics; overlay/tombstone scope explicit | Shared base, private puts/removals, same-byte reuse, disjoint writes and same-leaf/delete-modify conflicts produce exact candidates without IO claims | L0: DOCS/format; L1: RUST profile; L2: synthetic caller integration, no provider claim | verified-pure |
| WP-003 | Publish completed bytes with recoverable acknowledgement | REQ-003/REQ-004/REQ-009; SPEC-003 | Backend publication/flush and durable operation identity contract accepted | Lost reply/torn log/stale restore and every publication ordering fault retain recoverability; ack levels reflect achieved persistence | L0: DOCS/format; L1: PROVIDER profile; L2: profile-specific isolated end-to-end/failure evidence | in-progress |
| WP-004 | Protect active readers and owned claims during retirement | REQ-002/REQ-003/REQ-004; SPEC-004 | Effective provider fence and clock/renewal assumptions frozen | Granted reader versus retirement, ABA, new claims and partition/expiry cases reject unsafe actions while unrelated publishers progress | L0: DOCS/format; L1: PROVIDER profile; L2: profile-specific isolated end-to-end/failure evidence | in-progress |
| WP-005 | Use standard Git LFS objects from exact repository views | REQ-006/REQ-008/REQ-009; SPEC-007 | Pinned Git/LFS clients and standard extension support; enrolled source/storage scope | Pointer/index extraction, configured store, partial/conflicted staging, transfer retries and external-pruner limits verified in temporary repos | L0: DOCS/format; L1: GIT profile; L2: profile-specific isolated end-to-end/failure evidence | in-progress |
| WP-006 | Plan working proximity and retained replicas across versions | REQ-002/REQ-003/REQ-005/REQ-009; SPEC-008 | Claim owner, source lineage, failure-domain and assignment semantics frozen | Current/previous/draft union, infeasible copy count, unregistered archive policy, unknown coverage and peak-resource plans are explicit | L0: DOCS/format; L1: RUST profile; L2: synthetic caller integration, no provider claim | in-progress |
| WP-007 | Apply safe bounded placement on local stores | REQ-003/REQ-004/REQ-005/REQ-006; SPEC-009 | Local place identity/capability/custody matrix and bounded apply policy frozen | Copy/verify/recheck/release faults preserve bytes; full disk/corruption/shared use and physical reclaim measured in temporary stores | L0: DOCS/format; L1: PROVIDER profile; L2: profile-specific isolated end-to-end/failure evidence | in-progress |
| WP-008 | Find shared cached objects without routine rescans | REQ-005/REQ-007; SPEC-005 | Catalog schema/recovery boundary and warm/cold workload budgets pinned | Miss invalidation, event gaps, source corruption, restore and concurrent publication meet bounded reads/latency with no false custody | L0: DOCS/format; L1: PROVIDER profile; L2: profile-specific isolated end-to-end/failure evidence | in-progress |
| WP-009 | Follow Git commit and merge lifecycle without losing draft protection | REQ-002/REQ-004/REQ-009; SPEC-006/SPEC-007/SPEC-008 | Owned source checkpoint and trusted policy adoption rules frozen | New/staged/committed/pushed/merged/reverted and missed/duplicate events converge; crash handoff and active jobs stay protected; valid source may merge with explicitly stale output | L0: DOCS/format; L1: GIT profile; L2: profile-specific isolated end-to-end/failure evidence | in-progress |
| WP-010 | Resolve optional typed assets with Art parity | REQ-001/REQ-006/REQ-010; SPEC-010 | Art source/lock/licenses and canonical/model/graph/variant vector inventory pinned | Old/new parity on imported pure mechanisms; separate exact definitions/instances, declared mixed-stream merge constraints and standalone low-level use remain intact | L0: DOCS/format; L1: RUST profile; L2: synthetic caller integration, no provider claim | proposed |
| WP-011 | Produce artifacts from typed bindings and profiles | REQ-005/REQ-010; SPEC-011 | Native extraction and trusted producer/consumed-projection contracts accepted | Synthetic non-media producer plus text/translation/variant changes establish exact consumption, reusable results, source-valid/output-stale state and frozen catch-up batches; coalesce only before job admission | L0: DOCS/format; L1: INTEGRATION profile; L2: profile-specific isolated end-to-end/failure evidence | proposed |
| WP-012 | Evaluate and deliver exact selected asset versions | REQ-006/REQ-009/REQ-010; SPEC-012 | Trusted rubric/policy/authority, issue ownership and target identity contracts frozen | Stale scores, withdrawn authority, issue retries and remote drift/lost replies reconcile without private evidence leakage or false delivery; output freshness gates do not unconditionally block valid source merges | L0: DOCS/format; L1: INTEGRATION profile; L2: profile-specific isolated end-to-end/failure evidence | proposed |
| WP-013 | Retain and acquire objects through admitted remote stores | REQ-003/REQ-005/REQ-006/REQ-008; SPEC-013 | Each remote capability, identity, durability/retention and credential scope accepted | Authenticated LFS/object-store adapters pass retry/corruption/partial/denial and retention tests; explicit unsupported deletion/fencing | L0: DOCS/format; L1: PROVIDER profile; L2: profile-specific isolated end-to-end/failure evidence | proposed |
| WP-014 | Distribute reusable immutable shards through optional mature transport | REQ-003/REQ-005/REQ-006; SPEC-013 | Transport client pin, shard partition, privacy/key and reuse budgets accepted | Missing seeds, partial pieces, malicious metainfo, cross-client reuse and bounded scratch verified; no public discovery enabled by default | L0: DOCS/format; L1: PROVIDER profile; L2: profile-specific isolated end-to-end/failure evidence | proposed |
| WP-015 | Inspect exact branch resolutions and placement plans | REQ-006/REQ-007/REQ-010; SPEC-014 | Read-only view/bundle contracts, auth/export and native/WASM boundaries accepted | Synthetic branch/profile/coverage/budget reports share native semantics and show blocked states without private artifacts | L0: DOCS/format; L1: UI profile; L2: profile-specific isolated end-to-end/failure evidence | proposed |
| WP-016 | Adopt a private consumer through reversible format/runtime migration | REQ-006/REQ-010; SPEC-010/SPEC-011/SPEC-012/SPEC-013 | Owner-approved private source pins, exact old/new mapping and rollback/retention plan | Owner-run same-behavior and recoverability acceptance, dry-run byte accounting and reversible scoped cutover; public fixtures remain synthetic | L0: DOCS/format; L1: INTEGRATION profile; L2: profile-specific isolated end-to-end/failure evidence | proposed |

## Common execution and V closure

All packages use CR-001 through CR-008 and their LANGUAGE_PROFILES commands.
L0 is always structural/format/role/trace inspection. L1/L2 follow the named
profile, including explicit non-applicability for discovery prose. New tests must
exist and execute; a zero-match filter is not proof. Integration scope is bounded
and synthetic/temporary; no user cache census, transfer, cleanup or daemon.

Each implementation uses an isolated package branch/worktree at a recorded input
SHA, one writer per scope, and explicit allowed package paths. Stop when entry
contracts change or byte protection/authority/fencing is unresolved. Commit/push
after the relevant checks; PR/merge and consumer/live-operation approval remain
distinct. Active wave: 2026-10-25-placement-foundation; pulse mapping below.

Before closing a package, record its own V: NEED/CON -> REQ/SPEC -> ARCH/IF/DES/CR
-> exact implementation commit -> executed verification -> named VAL outcome
(or scoped accepted deferral) -> EVID/TRACE/REVIEW -> package disposition.
Every row is initially OPEN. No package is complete or independently approved.
Work-package design acceptance does not waive implementation entry criteria.

Required lanes for all implementation packages: systems engineering, traceability,
V&V, software assurance, safety/risk, security/privacy, source custody and
configuration/change control. The six local roles cover these lenses; method is
producer-applied unless an actual independent reviewer is recorded. Provider
packages must show effective recovery/custody evidence; no global-lock shortcut.

### WP-000: Choose a placement backend with demonstrated local/offline fit

- Product capability: Choose a placement backend with demonstrated local/offline fit.
- Need/workflow: CON-003/004; needs and full acceptance in MISSION/CONOPS/VALIDATION.
- Requirements/specs: REQ-006; SPEC-002/SPEC-003/SPEC-004/SPEC-005/SPEC-009.
- Architecture/interfaces/design: ARCH-002/ARCH-003/ARCH-007; IF-003/IF-004/IF-005/IF-012; DES-003/DES-004/DES-005/DES-008; CR-001..CR-008.
- Allowed boundaries: PKG-002/PKG-003/PKG-007/PKG-009; profile DOCS; integration follows INTEGRATION_PLAN.md.
- Product surfaces: docs/BACKEND-DECISION.md and bounded synthetic discovery notes. Other runtime packages/private inputs are out of scope.
- Prerequisites: none. Entry: Bounded backend questions and synthetic workload agreed.
- Exit: Recorded iRODS/Rucio/minimal-Rust capability, footprint, recovery and portability findings; choose or defer each capability explicitly.
- Verification commands: tools/validate.ps1 and git diff --check; no runtime checks required for discovery notes.
- Levels: L0 required; L1 not required (prose discovery); L2 not required (no runtime claim).
- Validation impact: VAL-002/003; no user acceptance claimed before its named run.
- Principal risk: Concrete removal fence, catalog recovery and Windows/offline fit remain unmeasured.
- Assurance: all eight required lanes, scoped to this package; producer review initially, actual independence recorded only when present.
- Git: wp/000-capability at pinned main; one scoped writer; no live configuration changes.
- Wave/pulse: context/waves/2026-10-25-placement-foundation/pulses/pulse-02.md.
- VTRACE-only closeout: EVID/VERIFICATION/VALIDATION/TRACE/REVIEW plus stage/pulse/package status; exact source and remaining gaps.
- V closure now: choose/defer discovery recorded in BACKEND-DECISION.md and EVID-011; local bounded adapter selected, distributed compatibility/footprint not benchmarked. No production backend acceptance.

### WP-001: Resolve exact immutable objects and bounded tree paths

- Product capability: Resolve exact immutable objects and bounded tree paths.
- Need/workflow: CON-001; needs and full acceptance in MISSION/CONOPS/VALIDATION.
- Requirements/specs: REQ-001/REQ-005/REQ-006; SPEC-002.
- Architecture/interfaces/design: ARCH-001; IF-002; DES-001; CR-001..CR-008.
- Allowed boundaries: PKG-001; profile RUST; integration follows INTEGRATION_PLAN.md.
- Product surfaces: crates/tig-core identity/tree modules and tests; specs/MODEL.md. Other runtime packages/private inputs are out of scope.
- Prerequisites: none. Entry: Current core regression baseline and hash/encoding decision scope pinned.
- Exit: Canonical/raw identity vectors, path and size negatives, bounded tree lookup and sparse-change cost demonstrated.
- Verification commands: tools/validate.ps1 and git diff --check; cargo fmt --all -- --check; cargo test --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; exact targeted fixture commands pinned at entry.
- Levels: L0 required; L1 pure vectors/negative tests required; L2 synthetic caller integration required, provider IO not applicable.
- Validation impact: VAL-001; no user acceptance claimed before its named run.
- Principal risk: Do not accidentally change plain LFS blob identity or import a second canonicalizer.
- Assurance: all eight required lanes, scoped to this package; producer review initially, actual independence recorded only when present.
- Git: wp/001-capability at pinned main; one scoped writer; no live configuration changes.
- Wave/pulse: context/waves/2026-10-25-placement-foundation/pulses/pulse-02.md.
- VTRACE-only closeout: EVID/VERIFICATION/VALIDATION/TRACE/REVIEW plus stage/pulse/package status; exact source and remaining gaps.
- V closure now: pure implementation verified in EVID-007/008; 26 new public-API tests plus 13 handoff regressions. Producer role review accepts this scope with risk; full requirements and VAL-001 remain OPEN. See specs/IDENTITY-AND-TREES.md for input, limits and commands.

### WP-002: Open and seal private byte worktree changes

- Product capability: Open and seal private byte worktree changes.
- Need/workflow: CON-002; needs and full acceptance in MISSION/CONOPS/VALIDATION.
- Requirements/specs: REQ-001/REQ-004/REQ-009; SPEC-006.
- Architecture/interfaces/design: ARCH-004; IF-007/IF-008; DES-006; CR-001..CR-008.
- Allowed boundaries: PKG-001/PKG-005; profile RUST; integration follows INTEGRATION_PLAN.md.
- Product surfaces: planned tig-worktree pure model/tests; specs/BYTE-WORKTREE.md. Other runtime packages/private inputs are out of scope.
- Prerequisites: WP-001. Entry: Frozen object/tree semantics; overlay/tombstone scope explicit.
- Exit: Shared base, private puts/removals, same-byte reuse, disjoint writes and same-leaf/delete-modify conflicts produce exact candidates without IO claims.
- Verification commands: tools/validate.ps1 and git diff --check; cargo fmt --all -- --check; cargo test --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; exact targeted fixture commands pinned at entry.
- Levels: L0 required; L1 pure vectors/negative tests required; L2 synthetic caller integration required, provider IO not applicable.
- Validation impact: VAL-001; no user acceptance claimed before its named run.
- Principal risk: A root pointer must not force every independent asset edit through one merge surface.
- Assurance: all eight required lanes, scoped to this package; producer review initially, actual independence recorded only when present.
- Git: wp/002-capability at pinned main; one scoped writer; no live configuration changes.
- Wave/pulse: context/waves/2026-10-25-placement-foundation/pulses/pulse-03.md.
- VTRACE-only closeout: EVID/VERIFICATION/VALIDATION/TRACE/REVIEW plus stage/pulse/package status; exact source and remaining gaps.
- V closure now: pure implementation verified under EVID-009 (22 new public-API tests); full requirements, durable worktrees, Git integration and VAL-001 remain OPEN. Producer eight-lane review accepts this pure scope with risk.

### WP-003: Publish completed bytes with recoverable acknowledgement

- Product capability: Publish completed bytes with recoverable acknowledgement.
- Need/workflow: CON-003; needs and full acceptance in MISSION/CONOPS/VALIDATION.
- Requirements/specs: REQ-003/REQ-004/REQ-009; SPEC-003.
- Architecture/interfaces/design: ARCH-002; IF-003/IF-004; DES-003; CR-001..CR-008.
- Allowed boundaries: PKG-002/PKG-003; profile PROVIDER; integration follows INTEGRATION_PLAN.md.
- Product surfaces: planned tig-store publication and tig-engine journal modules; isolated fixture stores. Other runtime packages/private inputs are out of scope.
- Prerequisites: WP-000/WP-001. Entry: Backend publication/flush and durable operation identity contract accepted.
- Exit: Lost reply/torn log/stale restore and every publication ordering fault retain recoverability; ack levels reflect achieved persistence.
- Verification commands: tools/validate.ps1 and git diff --check; cargo fmt --all -- --check; cargo test --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; exact targeted fixture commands pinned at entry.
- Levels: L0 required; L1 component/fake failures required; L2 isolated profile-specific workflow/recovery required.
- Validation impact: VAL-002; no user acceptance claimed before its named run.
- Principal risk: Durable protection cannot be reconstructed solely from branch heads or disposable catalog rows.
- Assurance: all eight required lanes, scoped to this package; producer review initially, actual independence recorded only when present.
- Git: wp/003-capability at pinned main; one scoped writer; no live configuration changes.
- Wave/pulse: context/waves/2026-10-25-placement-foundation/pulses/pulse-04.md.
- VTRACE-only closeout: EVID/VERIFICATION/VALIDATION/TRACE/REVIEW plus stage/pulse/package status; exact source and remaining gaps.
- V closure now: bounded local protocol subset implemented and exercised under EVID-010 (24 entries); process-restart acknowledgement only. Full power-loss/restore/ownership/admission exit and VAL-002 remain OPEN; package in progress.

### WP-004: Protect active readers and owned claims during retirement

- Product capability: Protect active readers and owned claims during retirement.
- Need/workflow: CON-003/005; needs and full acceptance in MISSION/CONOPS/VALIDATION.
- Requirements/specs: REQ-002/REQ-003/REQ-004; SPEC-004.
- Architecture/interfaces/design: ARCH-002; IF-004/IF-006/IF-012; DES-004; CR-001..CR-008.
- Allowed boundaries: PKG-002/PKG-003; profile PROVIDER; integration follows INTEGRATION_PLAN.md.
- Product surfaces: planned tig-engine claim/fence modules and provider-port fixtures. Other runtime packages/private inputs are out of scope.
- Prerequisites: WP-003. Entry: Effective provider fence and clock/renewal assumptions frozen.
- Exit: Granted reader versus retirement, ABA, new claims and partition/expiry cases reject unsafe actions while unrelated publishers progress.
- Verification commands: tools/validate.ps1 and git diff --check; cargo fmt --all -- --check; cargo test --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; exact targeted fixture commands pinned at entry.
- Levels: L0 required; L1 component/fake failures required; L2 isolated profile-specific workflow/recovery required.
- Validation impact: VAL-002/004; no user acceptance claimed before its named run.
- Principal risk: No timeout-only or open-handle-only safety assumption; unsupported removal must block.
- Assurance: all eight required lanes, scoped to this package; producer review initially, actual independence recorded only when present.
- Git: wp/004-capability at pinned main; one scoped writer; no live configuration changes.
- Wave/pulse: context/waves/2026-10-25-placement-foundation/pulses/pulse-04.md.
- VTRACE-only closeout: EVID/VERIFICATION/VALIDATION/TRACE/REVIEW plus stage/pulse/package status; exact source and remaining gaps.
- V closure now: local OS fences, exact replicas, local retention and owned handoff exercised under EVID-013/018/019/020; full claim/recovery, expiry/renewal, remote-provider, hostile-namespace and payload-retirement authority remain OPEN. Package in progress; WP-003 full prerequisite and VAL-002/004 remain OPEN.

### WP-005: Use standard Git LFS objects from exact repository views

- Product capability: Use standard Git LFS objects from exact repository views.
- Need/workflow: CON-001/005; needs and full acceptance in MISSION/CONOPS/VALIDATION.
- Requirements/specs: REQ-006/REQ-008/REQ-009; SPEC-007.
- Architecture/interfaces/design: ARCH-004; IF-008/IF-009; DES-007; CR-001..CR-008.
- Allowed boundaries: PKG-004; profile GIT; integration follows INTEGRATION_PLAN.md.
- Product surfaces: experimental tig-git exact-source/download agent; synthetic Git/LFS integration fixtures. Other runtime packages/private inputs are out of scope.
- Prerequisites: WP-001/WP-003/WP-004. Entry: Pinned Git/LFS clients and standard extension support; enrolled source/storage scope.
- Exit: Pointer/index extraction, configured store, partial/conflicted staging, transfer retries and external-pruner limits verified in temporary repos.
- Verification commands: tools/validate.ps1 and git diff --check; cargo fmt --all -- --check; cargo test --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; exact targeted fixture commands pinned at entry.
- Levels: L0 required; L1 component/fake failures required; L2 isolated profile-specific workflow/recovery required.
- Validation impact: VAL-001/004; no user acceptance claimed before its named run.
- Principal risk: OID-only requests cannot manufacture branch lifecycle or cloud-upload completion.
- Assurance: all eight required lanes, scoped to this package; producer review initially, actual independence recorded only when present.
- Git: wp/005-capability at pinned main; one scoped writer; no live configuration changes.
- Wave/pulse: context/waves/2026-10-25-placement-foundation/pulses/pulse-04.md.
- VTRACE-only closeout: EVID/VERIFICATION/VALIDATION/TRACE/REVIEW plus stage/pulse/package status; exact source and remaining gaps.
- V closure now: EVID-022/023 cover bounded download compatibility; EVID-024/025 cover exact scoped commit/index reads and source-driven actual transfer authorization. Full attribute-aware extraction, durable claim adoption, upload, accounting and native validation remain OPEN. Package stays in progress.

### WP-006: Plan working proximity and retained replicas across versions

Resource design input: [RESOURCE-MANAGEMENT](../../specs/RESOURCE-MANAGEMENT.md).
Prove shared capacity-domain admission, remaining reservation accounting, bounded
active jobs versus idle worktrees and independent project quotas. The guide's
arithmetic fixture does not satisfy these runtime exit obligations.

- Product capability: Plan working proximity and retained replicas across versions.
- Need/workflow: CON-002/004; needs and full acceptance in MISSION/CONOPS/VALIDATION.
- Requirements/specs: REQ-002/REQ-003/REQ-005/REQ-009; SPEC-008.
- Architecture/interfaces/design: ARCH-001/ARCH-002; IF-004/IF-006; DES-002/DES-008; CR-001..CR-008.
- Allowed boundaries: PKG-001/PKG-003; profile RUST; integration follows INTEGRATION_PLAN.md.
- Product surfaces: crates/tig-core placement extension and planned engine policy; synthetic assigned/domain fixtures. Other runtime packages/private inputs are out of scope.
- Prerequisites: WP-001/WP-003/WP-004. Entry: Claim owner, source lineage, failure-domain and assignment semantics frozen.
- Exit: Current/previous/draft union, infeasible copy count, unregistered archive policy, unknown coverage and peak-resource plans are explicit.
- Verification commands: tools/validate.ps1 and git diff --check; cargo fmt --all -- --check; cargo test --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; exact targeted fixture commands pinned at entry.
- Levels: L0 required; L1 pure vectors/negative tests required; L2 synthetic caller integration required, provider IO not applicable.
- Validation impact: VAL-003; no user acceptance claimed before its named run.
- Principal risk: Copy counts are not independent backup counts; main history follows declared selection lineage.
- Assurance: all eight required lanes, scoped to this package; producer review initially, actual independence recorded only when present.
- Git: wp/006-capability at pinned main; one scoped writer; no live configuration changes.
- Wave/pulse: context/waves/2026-10-25-placement-foundation/pulses/pulse-04.md.
- VTRACE-only closeout: EVID/VERIFICATION/VALIDATION/TRACE/REVIEW plus stage/pulse/package status; exact source and remaining gaps.
- V closure now: pure capacity-domain subset exercised under EVID-012; no durable resource service, full placement/retained-copy solver or physical-space admission integration. Package in progress; upstream prerequisites and VAL-003 remain OPEN.

### WP-007: Apply safe bounded placement on local stores

- Product capability: Apply safe bounded placement on local stores.
- Need/workflow: CON-003/004; needs and full acceptance in MISSION/CONOPS/VALIDATION.
- Requirements/specs: REQ-003/REQ-004/REQ-005/REQ-006; SPEC-009.
- Architecture/interfaces/design: ARCH-002/ARCH-007; IF-003/IF-004/IF-006/IF-012; DES-003/DES-004/DES-008; CR-001..CR-008.
- Allowed boundaries: PKG-002/PKG-003; profile PROVIDER; integration follows INTEGRATION_PLAN.md.
- Product surfaces: planned local provider and bounded executor; no production cache paths. Other runtime packages/private inputs are out of scope.
- Prerequisites: WP-003/WP-004/WP-006. Entry: Local place identity/capability/custody matrix and bounded apply policy frozen.
- Exit: Copy/verify/recheck/release faults preserve bytes; full disk/corruption/shared use and physical reclaim measured in temporary stores.
- Verification commands: tools/validate.ps1 and git diff --check; cargo fmt --all -- --check; cargo test --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; exact targeted fixture commands pinned at entry.
- Levels: L0 required; L1 component/fake failures required; L2 isolated profile-specific workflow/recovery required.
- Validation impact: VAL-002/003; no user acceptance claimed before its named run.
- Principal risk: No final archival deletion by ordinary cache eviction; reservations cannot control external writers.
- Assurance: all eight required lanes, scoped to this package; producer review initially, actual independence recorded only when present.
- Git: wp/007-capability at pinned main; one scoped writer; no live configuration changes.
- Wave/pulse: context/waves/2026-10-25-placement-foundation/pulses/pulse-04.md.
- VTRACE-only closeout: EVID/VERIFICATION/VALIDATION/TRACE/REVIEW plus stage/pulse/package status; exact source and remaining gaps.
- V closure now: admitted local publication/footprint and measured local sampling subsets exercised under EVID-015/016/017, including authorized private-partial discard. Native domain enrollment, complete volume overhead/enforcement, safe retirement/reclaim and VAL-002/003 remain OPEN; upstream prerequisites not waived. Package in progress.

### WP-008: Find shared cached objects without routine rescans

Resource design input: [RESOURCE-MANAGEMENT](../../specs/RESOURCE-MANAGEMENT.md).
Prove incremental attribution and bounded reconciliation of physical allocation,
scratch, materialization and unknown growth; retain observations outside Git
under an explicit metadata budget. Offline coverage stays unknown.

- Product capability: Find shared cached objects without routine rescans.
- Need/workflow: CON-001/005; needs and full acceptance in MISSION/CONOPS/VALIDATION.
- Requirements/specs: REQ-005/REQ-007; SPEC-005.
- Architecture/interfaces/design: ARCH-003; IF-005; DES-005; CR-001..CR-008.
- Allowed boundaries: PKG-003; profile PROVIDER; integration follows INTEGRATION_PLAN.md.
- Product surfaces: tig-store journals and bounded persistent metadata NodeStore; planned full engine catalog; bounded synthetic workload fixtures. Other runtime packages/private inputs are out of scope.
- Prerequisites: WP-003/WP-004. Entry: Catalog schema/recovery boundary and warm/cold workload budgets pinned.
- Exit: Miss invalidation, event gaps, source corruption, restore and concurrent publication meet bounded reads/latency with no false custody.
- Verification commands: tools/validate.ps1 and git diff --check; cargo fmt --all -- --check; cargo test --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; exact targeted fixture commands pinned at entry.
- Levels: L0 required; L1 component/fake failures required; L2 isolated profile-specific workflow/recovery required.
- Validation impact: VAL-001/004; no user acceptance claimed before its named run.
- Principal risk: Catalog availability and integrity evidence are separate; unknown/offline is not empty.
- Assurance: all eight required lanes, scoped to this package; producer review initially, actual independence recorded only when present.
- Git: wp/008-capability at pinned main; one scoped writer; no live configuration changes.
- Wave/pulse: context/waves/2026-10-25-placement-foundation/pulses/pulse-04.md.
- VTRACE-only closeout: EVID/VERIFICATION/VALIDATION/TRACE/REVIEW plus stage/pulse/package status; exact source and remaining gaps.
- V closure now: EVID-014/028/029 verify bounded journals, persistent metadata and a 500,000-binding synthetic sparse-edit workload. EVID-030 connects persisted roots to read-only LFS authorization. Complete catalog/source coverage, global admission, compaction, restore authority and native integration remain OPEN; upstream prerequisites and VAL-001/004 are not waived. Package in progress.

### WP-009: Follow Git commit and merge lifecycle without losing draft protection

- Product capability: Follow Git commit and merge lifecycle without losing draft protection.
- Need/workflow: CON-002/005; needs and full acceptance in MISSION/CONOPS/VALIDATION.
- Requirements/specs: REQ-002/REQ-004/REQ-009; SPEC-006/SPEC-007/SPEC-008.
- Architecture/interfaces/design: ARCH-002/ARCH-004; IF-004/IF-006/IF-007/IF-008; DES-003/DES-006/DES-007; CR-001..CR-008.
- Allowed boundaries: PKG-003/PKG-004/PKG-005; profile GIT; integration follows INTEGRATION_PLAN.md.
- Product surfaces: bounded source-root journal and Git extraction integration; planned full ref/job reconciliation and guarded adoption. Other runtime packages/private inputs are out of scope.
- Prerequisites: WP-002/WP-005/WP-006/WP-007/WP-008. Entry: Owned source checkpoint and trusted policy adoption rules frozen.
- Exit: New/staged/committed/pushed/merged/reverted and missed/duplicate events converge; crash handoff and active jobs stay protected; valid source may merge with explicitly stale output.
- Verification commands: tools/validate.ps1 and git diff --check; cargo fmt --all -- --check; cargo test --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; exact targeted fixture commands pinned at entry.
- Levels: L0 required; L1 component/fake failures required; L2 isolated profile-specific workflow/recovery required.
- Validation impact: VAL-001/004; no user acceptance claimed before its named run.
- Principal risk: A successful Git event is not observed placement success; no second manual content merge.
- Assurance: all eight required lanes, scoped to this package; producer review initially, actual independence recorded only when present.
- Git: wp/009-capability at pinned main; one scoped writer; no live configuration changes.
- Wave/pulse: context/waves/2026-10-25-placement-foundation/pulses/pulse-05.md.
- VTRACE-only closeout: EVID/VERIFICATION/VALIDATION/TRACE/REVIEW plus stage/pulse/package status; exact source and remaining gaps.
- V closure now: EVID-026/027 verify prepared/current/predecessor root persistence, exact owner/revision transitions, crash replay and real Git integration. Full Git/job source coverage, root metadata availability, currentness/retirement fencing, restore authority, compaction and native adoption remain OPEN; prerequisites are not waived. Package in progress.

### WP-010: Resolve optional typed assets with Art parity

- Product capability: Resolve optional typed assets with Art parity.
- Need/workflow: CON-006; needs and full acceptance in MISSION/CONOPS/VALIDATION.
- Requirements/specs: REQ-001/REQ-006/REQ-010; SPEC-010.
- Architecture/interfaces/design: ARCH-005; IF-010; DES-009; CR-001..CR-008.
- Allowed boundaries: PKG-001/PKG-006; profile RUST; integration follows INTEGRATION_PLAN.md.
- Product surfaces: planned tig-assets pure modules/tests and bounded source-intake docs. Other runtime packages/private inputs are out of scope.
- Prerequisites: WP-001. Entry: Art source/lock/licenses and canonical/model/graph/variant vector inventory pinned.
- Exit: Old/new parity on imported pure mechanisms; separate exact definitions/instances, declared mixed-stream merge constraints and standalone low-level use remain intact.
- Verification commands: tools/validate.ps1 and git diff --check; cargo fmt --all -- --check; cargo test --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; exact targeted fixture commands pinned at entry.
- Levels: L0 required; L1 pure vectors/negative tests required; L2 synthetic caller integration required, provider IO not applicable.
- Validation impact: VAL-005; no user acceptance claimed before its named run.
- Principal risk: Do not count Art tests as Tig evidence before port; preserve original hash domains until tested migration.
- Assurance: all eight required lanes, scoped to this package; producer review initially, actual independence recorded only when present.
- Git: wp/010-capability at pinned main; one scoped writer; no live configuration changes.
- Wave/pulse: context/waves/2026-10-25-placement-foundation/pulses/pulse-05.md.
- VTRACE-only closeout: EVID/VERIFICATION/VALIDATION/TRACE/REVIEW plus stage/pulse/package status; exact source and remaining gaps.
- V closure now: native intake and caller-admitted declared-reference queries are component-tested under NATIVE-INTAKE.md / NATIVE-GRAPH.md (EVID-031/032/033). Art model/graph/variant parity, full semantic/native provider extraction and consumer acceptance remain OPEN; this is not full package closure.

### WP-011: Produce artifacts from typed bindings and profiles

- Product capability: Produce artifacts from typed bindings and profiles.
- Need/workflow: CON-006; needs and full acceptance in MISSION/CONOPS/VALIDATION.
- Requirements/specs: REQ-005/REQ-010; SPEC-011.
- Architecture/interfaces/design: ARCH-005; IF-010/IF-011; DES-009; CR-001..CR-008.
- Allowed boundaries: PKG-005/PKG-006; profile INTEGRATION; integration follows INTEGRATION_PLAN.md.
- Product surfaces: planned tig-assets binding/profile/native-adapter APIs and fake producers. Other runtime packages/private inputs are out of scope.
- Prerequisites: WP-002/WP-003/WP-004/WP-010. Entry: Native extraction and trusted producer/consumed-projection contracts accepted.
- Exit: Synthetic non-media producer plus text/translation/variant changes establish exact consumption, reusable results, source-valid/output-stale state and frozen catch-up batches; coalesce only before job admission.
- Verification commands: tools/validate.ps1 and git diff --check; cargo fmt --all -- --check; cargo test --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; exact targeted fixture commands pinned at entry.
- Levels: L0 required; L1 component/fake failures required; L2 isolated profile-specific workflow/recovery required.
- Validation impact: VAL-005; no user acceptance claimed before its named run.
- Principal risk: No unmanaged path fallback, duplicate dependency lists or compiled-output relabelling.
- Assurance: all eight required lanes, scoped to this package; producer review initially, actual independence recorded only when present.
- Git: wp/011-capability at pinned main; one scoped writer; no live configuration changes.
- Wave/pulse: context/waves/2026-10-25-placement-foundation/pulses/pulse-05.md.
- VTRACE-only closeout: EVID/VERIFICATION/VALIDATION/TRACE/REVIEW plus stage/pulse/package status; exact source and remaining gaps.
- V closure now: left-side design proposed; implementation/verification/validation OPEN; review accepts plan with risk only.

### WP-012: Evaluate and deliver exact selected asset versions

- Product capability: Evaluate and deliver exact selected asset versions.
- Need/workflow: CON-006; needs and full acceptance in MISSION/CONOPS/VALIDATION.
- Requirements/specs: REQ-006/REQ-009/REQ-010; SPEC-012.
- Architecture/interfaces/design: ARCH-006; IF-011; DES-010; CR-001..CR-008.
- Allowed boundaries: PKG-006/PKG-007; profile INTEGRATION; integration follows INTEGRATION_PLAN.md.
- Product surfaces: planned scoped gate/triage/delivery modules and fake issue/remote adapters. Other runtime packages/private inputs are out of scope.
- Prerequisites: WP-009/WP-011/WP-013. Entry: Trusted rubric/policy/authority, issue ownership and target identity contracts frozen.
- Exit: Stale scores, withdrawn authority, issue retries and remote drift/lost replies reconcile without private evidence leakage or false delivery; output freshness gates do not unconditionally block valid source merges.
- Verification commands: tools/validate.ps1 and git diff --check; cargo fmt --all -- --check; cargo test --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; exact targeted fixture commands pinned at entry.
- Levels: L0 required; L1 component/fake failures required; L2 isolated profile-specific workflow/recovery required.
- Validation impact: VAL-005; no user acceptance claimed before its named run.
- Principal risk: An issue close, aggregate score or old commit cannot grant present publication authority.
- Assurance: all eight required lanes, scoped to this package; producer review initially, actual independence recorded only when present.
- Git: wp/012-capability at pinned main; one scoped writer; no live configuration changes.
- Wave/pulse: context/waves/2026-10-25-placement-foundation/pulses/pulse-05.md.
- VTRACE-only closeout: EVID/VERIFICATION/VALIDATION/TRACE/REVIEW plus stage/pulse/package status; exact source and remaining gaps.
- V closure now: left-side design proposed; implementation/verification/validation OPEN; review accepts plan with risk only.

### WP-013: Retain and acquire objects through admitted remote stores

- Product capability: Retain and acquire objects through admitted remote stores.
- Need/workflow: CON-003/004/005; needs and full acceptance in MISSION/CONOPS/VALIDATION.
- Requirements/specs: REQ-003/REQ-005/REQ-006/REQ-008; SPEC-013.
- Architecture/interfaces/design: ARCH-007; IF-005/IF-009/IF-012; DES-008/DES-011; CR-001..CR-008.
- Allowed boundaries: PKG-002/PKG-007; profile PROVIDER; integration follows INTEGRATION_PLAN.md.
- Product surfaces: planned remote provider modules and fake/isolated test endpoints. Other runtime packages/private inputs are out of scope.
- Prerequisites: WP-005/WP-007/WP-008. Entry: Each remote capability, identity, durability/retention and credential scope accepted.
- Exit: Authenticated LFS/object-store adapters pass retry/corruption/partial/denial and retention tests; explicit unsupported deletion/fencing.
- Verification commands: tools/validate.ps1 and git diff --check; cargo fmt --all -- --check; cargo test --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; exact targeted fixture commands pinned at entry.
- Levels: L0 required; L1 component/fake failures required; L2 isolated profile-specific workflow/recovery required.
- Validation impact: VAL-002/003/004; no user acceptance claimed before its named run.
- Principal risk: Cloud-provider name or successful API response is not independent retained custody.
- Assurance: all eight required lanes, scoped to this package; producer review initially, actual independence recorded only when present.
- Git: wp/013-capability at pinned main; one scoped writer; no live configuration changes.
- Wave/pulse: context/waves/2026-10-25-placement-foundation/pulses/pulse-05.md.
- VTRACE-only closeout: EVID/VERIFICATION/VALIDATION/TRACE/REVIEW plus stage/pulse/package status; exact source and remaining gaps.
- V closure now: left-side design proposed; implementation/verification/validation OPEN; review accepts plan with risk only.

### WP-014: Distribute reusable immutable shards through optional mature transport

- Product capability: Distribute reusable immutable shards through optional mature transport.
- Need/workflow: CON-004; needs and full acceptance in MISSION/CONOPS/VALIDATION.
- Requirements/specs: REQ-003/REQ-005/REQ-006; SPEC-013.
- Architecture/interfaces/design: ARCH-007; IF-012; DES-011; CR-001..CR-008.
- Allowed boundaries: PKG-007; profile PROVIDER; integration follows INTEGRATION_PLAN.md.
- Product surfaces: planned optional transport provider using existing client/library APIs. Other runtime packages/private inputs are out of scope.
- Prerequisites: WP-013. Entry: Transport client pin, shard partition, privacy/key and reuse budgets accepted.
- Exit: Missing seeds, partial pieces, malicious metainfo, cross-client reuse and bounded scratch verified; no public discovery enabled by default.
- Verification commands: tools/validate.ps1 and git diff --check; cargo fmt --all -- --check; cargo test --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; exact targeted fixture commands pinned at entry.
- Levels: L0 required; L1 component/fake failures required; L2 isolated profile-specific workflow/recovery required.
- Validation impact: VAL-003; no user acceptance claimed before its named run.
- Principal risk: Do not write a torrent stack or assume seed count equals backup or a logical drop frees physical pack bytes.
- Assurance: all eight required lanes, scoped to this package; producer review initially, actual independence recorded only when present.
- Git: wp/014-capability at pinned main; one scoped writer; no live configuration changes.
- Wave/pulse: context/waves/2026-10-25-placement-foundation/pulses/pulse-05.md.
- VTRACE-only closeout: EVID/VERIFICATION/VALIDATION/TRACE/REVIEW plus stage/pulse/package status; exact source and remaining gaps.
- V closure now: left-side design proposed; implementation/verification/validation OPEN; review accepts plan with risk only.

### WP-015: Inspect exact branch resolutions and placement plans

- Product capability: Inspect exact branch resolutions and placement plans.
- Need/workflow: CON-001/006; needs and full acceptance in MISSION/CONOPS/VALIDATION.
- Requirements/specs: REQ-006/REQ-007/REQ-010; SPEC-014.
- Architecture/interfaces/design: ARCH-008; IF-013; DES-012; CR-001..CR-008.
- Allowed boundaries: PKG-008; profile UI; integration follows INTEGRATION_PLAN.md.
- Product surfaces: bounded native Rust CLI/catalogue implemented with synthetic browser fixtures; optional WASM/full native parity remains target scope. Other runtime packages/private inputs are out of scope.
- Prerequisites: WP-008/WP-009/WP-010. Entry: Read-only view/bundle contracts, auth/export and native/WASM boundaries accepted.
- Exit: Synthetic branch/profile/coverage/budget reports share native semantics and show blocked states without private artifacts.
- Verification commands: tools/validate.ps1 and git diff --check; cargo fmt --all -- --check; cargo test --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; exact targeted fixture commands pinned at entry.
- Levels: L0 required; L1 component/fake failures required; L2 isolated profile-specific workflow/recovery required.
- Validation impact: VAL-001/005; no user acceptance claimed before its named run.
- Principal risk: A static browser cannot silently read local drives; no implicit bridge or credentials in exports.
- Assurance: all eight required lanes, scoped to this package; producer review initially, actual independence recorded only when present.
- Git: wp/015-capability at pinned main; one scoped writer; no live configuration changes.
- Wave/pulse: context/waves/2026-10-25-placement-foundation/pulses/pulse-05.md.
- VTRACE-only closeout: EVID/VERIFICATION/VALIDATION/TRACE/REVIEW plus stage/pulse/package status; exact source and remaining gaps.
- V closure now: left-side design proposed; implementation/verification/validation OPEN; review accepts plan with risk only.

### WP-016: Adopt a private consumer through reversible format/runtime migration

- Product capability: Adopt a private consumer through reversible format/runtime migration.
- Need/workflow: CON-002/003/006; needs and full acceptance in MISSION/CONOPS/VALIDATION.
- Requirements/specs: REQ-006/REQ-010; SPEC-010/SPEC-011/SPEC-012/SPEC-013.
- Architecture/interfaces/design: ARCH-004/ARCH-005/ARCH-006/ARCH-007; IF-007/IF-008/IF-010/IF-011/IF-012; DES-006/DES-007/DES-009/DES-010; CR-001..CR-008.
- Allowed boundaries: PKG-004/PKG-005/PKG-006/PKG-007; profile INTEGRATION; integration follows INTEGRATION_PLAN.md.
- Product surfaces: consumer-owned adapter/record changes; Tig generic APIs only where explicitly scoped. Other runtime packages/private inputs are out of scope.
- Prerequisites: WP-009/WP-011/WP-012/WP-013. Entry: Owner-approved private source pins, exact old/new mapping and rollback/retention plan.
- Exit: Owner-run same-behavior and recoverability acceptance, dry-run byte accounting and reversible scoped cutover; public fixtures remain synthetic.
- Verification commands: tools/validate.ps1 and git diff --check; cargo fmt --all -- --check; cargo test --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; exact targeted fixture commands pinned at entry.
- Levels: L0 required; L1 component/fake failures required; L2 isolated profile-specific workflow/recovery required.
- Validation impact: VAL-002/005; no user acceptance claimed before its named run.
- Principal risk: No mass consumer conversion, private media export or live destructive cleanup from a prototype pass.
- Assurance: all eight required lanes, scoped to this package; producer review initially, actual independence recorded only when present.
- Git: wp/016-capability at pinned main; one scoped writer; no live configuration changes.
- Wave/pulse: context/waves/2026-10-25-placement-foundation/pulses/pulse-05.md.
- VTRACE-only closeout: EVID/VERIFICATION/VALIDATION/TRACE/REVIEW plus stage/pulse/package status; exact source and remaining gaps.
- V closure now: left-side design proposed; implementation/verification/validation OPEN; review accepts plan with risk only.

## Orphan and readiness disposition

Every accepted requirement/spec has an allocation in IMPLEMENTATION_PLAN.md.
SPEC-001/015 retain current regression/development scope rather than imaginary
new runtime packages. All target packages are proposed; prerequisites and unknown
contracts constrain entry. Backend discovery and pure identity work can be the
first bounded slices after baseline closeout; live removal remains unavailable.

## Stage review

Producer-applied six-role lenses: pass_with_risk for package design. WP-01:
verification could be a generic Cargo invocation disconnected from the feature.
Resolved by product-specific exits, frozen targeted commands and independent
profile L2 obligations. Exact future test names/results remain entry/closeout
records, never fabricated here. No target package is marked implemented.

WP-02 follow-up review: source merging and rendering are independent policy
transitions. Explicit mixed-stream resolution is validated by the pinned definition;
catch-up builds freeze chosen sources and preserve old production provenance.
Native instance/definition fixtures do not imply a released format or semantic engine.

WP-009 bounded Git handoff progress: EVID-048 supplies exact-base small-file
proposal commits/refs and real Git application/conflict fixtures, independent of
optional typed assets. This does not close missed-event/owner recovery, canonical
native handoff integration, claim convergence, payload protection or full adoption.

EVID-049 adds the standalone verified-cache CLI/shared read-only preflight for
WP-005/008/009. A candidate file can be proposed without REEL-specific Git code.
The new path does not close native schema/gates, managed owner recovery or the
complete source/claim/placement adoption criteria above.
