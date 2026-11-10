# Record inspector request isolation

Input 57106774f6ff6da78c483ee038ef0a0859629d88; WP-015;
REQ-001/006/007/010; SPEC-014; IF-010/011/013; PKG-004/006/008.
UI/INTEGRATION profiles; identity, provider/privacy, adoption and concurrency
producer checklists. Entry: shipped-script synthetic workflow fixtures.

Scope: exact request tokens for repeated record inspection, immediate clearing
of old tabs/actions, captured project/ID/snapshot response validation and
dependency render tokens. Reuse existing endpoints. No cache IO, new schemas,
historical resolution, live projects or browser automation.

Exit/L0: portable source/hash/link/diff checks. L1: integrated controller cases
and rebuilt native catalogue API tests. L2: same-record out-of-order success and
failure, pending/failed action clearing, wrong response sources, dependency tab
reentry, Escape and return navigation. Replay new failures against input source.
Synthetic controller checks do not establish visual/browser or custody acceptance.

Producer review: identity validates project, logical ID and exact captured commit
before adopting details; unknown enrollment refuses. Provider/privacy uses the
same named metadata routes, session protections and text rendering with no new
discovery, reads or dependencies. Adoption preserves existing card identities and
work/object/consumer return callbacks. Concurrency uses per-inspection and
per-detail-render tokens without locks; pending state clears older action tabs.
No blocking finding in this scope. Owner: catalogue maintainer. Pass_with_risk;
producer checklists are not independent review or human acceptance.

Verification: six new regression cases failed against input source before the
fix, including stale same-card success/error and dependency-tab reentry. All 66
integrated/frontend scenarios now pass. One brittle single-line function-extract
test was replaced by a shipped-script cross-project equivalent. Eight rebuilt
native catalogue comparison/impact API cases pass. No Rust production logic
changed and the full workspace suite was not rerun locally for this slice.
Portable source/hash/link/diff checks and protected hosted checks remain required.
Visual/browser, live-project, complete coverage and custody acceptance remain open.
