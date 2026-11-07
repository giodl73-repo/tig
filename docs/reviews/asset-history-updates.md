# Producer review: explicit history update preparation

Scope and controls: [ASSET-HISTORY-UPDATES](../../specs/ASSET-HISTORY-UPDATES.md).
Input 27846e51261e871502b5848c3fd245147dc18baf. Targets: edits module and
tests, optional export, exact-history membership accessors. No persistence or
provider format change. Six repository role checklists applied by the producer;
not independent review or human acceptance.

| Role | Finding, disposition and remaining scope |
| --- | --- |
| Identity | Stale raw instance refuses before metadata reads; source and candidate bind exact definitions/schema. Deterministic content identities, exact repeat no-op and preserved raw no-op bytes tested. |
| Placement | Registering a revision cannot select it. Selection requires explicit representation, subject, status and mirrored stream names. Partial coverage stays partial; preparation creates no claim. |
| Storage/custody | Only new small metadata is returned; immutable parent pages stay byte-identical. Unclassified selected bytes must be registered/classified before replacement. No retention, deletion or custody authority returned. |
| Concurrency/recovery | Expected source identity is returned for the existing later Git handoff. Reserved-slot collision refuses; retries are deterministic. This in-memory API provides no durable transaction or adoption fence itself. |
| Provider/privacy | Reader cache charges old/new objects together and checks sizes/hashes. Requests/status are bounded before metadata reads. No paths, prompts or private providers enrolled; synthetic tests make no writes through the reader. |
| Git/adoption | Stable asset ID, authored values, rights, unrelated streams/metadata preserved. Historical aliases remain old unless explicitly named; status changes require the caller's complete map. Existing sparse proposals remain the future handoff. |

Executed: ten focused update scenarios and the complete all-feature tig-assets
regression suite pass; package all-target/all-feature locked Clippy passes with
warnings denied. Cases include episode-like and non-media records, A/B retained
revisions, two delivery targets, stale input, run collision, wrong selection,
stream collisions, cumulative budgets and bounded growth. The existing common
history suites cover causal conflicts and partial/missing coverage. L0 and final
combined workspace/hosted qualification are tracked at integration, separately.

Disposition: pass_with_risk for pure preparation only. T2 persistence/CLI/Git
handoff and T3 protection/placement remain open. A prepared candidate does not
prove domain stream rules, production correctness, actual remote publication,
payload availability, retention or complete asset semantics.
