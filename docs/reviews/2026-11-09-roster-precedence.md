# Roster-wide advisory precedence review

Input106f6f524259fa15ecb487f9e808665fad723d1f;
implementationada8b2176556e77eaee8ceda7a48cf66195fb901.
Controls and RUST/PROVIDER L0/L1/L2 entry/exit:
[ROSTER-PRECEDENCE](../../specs/ROSTER-PRECEDENCE.md).
WP-004/006/007/009; REQ-002/003/004/005/009; SPEC-004/008/009;
IF-004/006/012; PKG-002/003/008.
Targets: store_protection Binding/assessment, admission::roster, placement
enrollment/compiler, focused fixtures and pinned pre-change CLI qualification.
Reviewed source hashes are recorded in foundation-targets.json.

## Method and findings

One producer applies all six [.roles checklists](../../.roles/ROLE.md).
No delegation, independent review, human acceptance or deployment authority.
Owner of each finding: Tig maintainer using the named lens. This closes PL-01
only for the explicit fresh roster profile; legacy namespace-local enrollments
retain their original semantics and need deliberate future migration/adoption.

| Finding / lens | Severity | Disposition and evidence | Remaining obligation |
| --- | --- | --- | --- |
| RP-01 Identity and Namespace | High | New history_scope:roster binding and design-2 placement enrollment distinguish the group decision. Two independent Python ASCII canonical/hashlib outputs pin both legacy and roster binding identities. Namespace aliases map to physical providers after group precedence; mixed profiles refuse. | No automatic live binding/epoch migration or latest-authority guarantee. |
| RP-02 Placement Policy | High, PL-01 corrected in profile | Raw active demand across the exact group suppresses only advisory history in prefer_active namespaces. Actual F/D/M fixture starts a job in another namespace, reacquires F and evicts D, then reverses on job withdrawal. Explicit additive history and retention-class demand keep D. Legacy cross-namespace union remains separately tested. | Native semantic coverage and independently admitted group policy adoption. |
| RP-03 Storage and Custody | High | Same rule drives planning and ordinary live store assessment. Existing retirement verifies retained keepers and holds required cache readers; M survives both transitions. Final content-wide clearance and last-copy authority are unchanged. | Actual providers, large files, native custody and volume reclaim remain unqualified. |
| RP-04 Concurrency and Recovery | High | All group subject guards and provider exclusion stay held through assessment. A real child cannot withdraw the remote active use until guards release, then succeeds; D is required again. Direct peer contention, unknown closure, missing member and failed-guard release cases pass. Raw-bucket admission already fences role changes. | Full provider/authority restoration, live migration and operational lifetime. |
| RP-05 Provider and Privacy | High | Binding defaults preserve old canonical bytes. Pinned pre-change CLI accepts legacy stores and refuses both final-capable and new roster bindings without source/ledger effects. Aggregate work limits do not reset per namespace. Missing metadata and conflicting sizes refuse before seed IO. | Already-open old writers, unmanaged actors and fresh current authority remain adoption gates. |
| RP-06 Git and Adoption | Medium | Existing source events, receipts, provider removal protocol and namespace APIs stay unchanged. Explicit design-2 enrollment cannot run against namespace-local bindings, nor design-1 against roster bindings. No hook, runner, store upgrade or service is installed. | Operator/native profiles, deployment and human intended-use acceptance. |

## Verification

Twenty-five focused placement entries pass, including the child helper; sixteen
store-protection entries pass, including the independent binding framing case.
The real peer has explicit Busy77 and success78 outcomes, not simulated crash
coverage. Existing crash recovery mechanisms are inherited, not new crash claims.
Strict all-feature/all-target workspace Clippy passes. Fmt, portable validation,
reviewed hashes and diff checks pass for implementationada8b21.

Explicit ignored/exact pre-change writer qualification passes using the pinned
273e639 tig-storage artifact: legacy planning succeeds; final-capable and roster
binding planning refuse; source/ledger/payloads remain. The existing hosted step
builds that exact old artifact on each platform and runs this expanded test.
An ordinary ignored test entry is not executed proof.

Full locked default and all-feature workspace suites pass in separate external
targets, including both 500,000-binding metadata fixtures. Standalone Git without
optional features passes. A fresh all-feature CLI passes the explicitly executed
ordinary-retirement pending-cancellation qualifier; the expanded pinned pre-change
writer qualifier passes again. No implementation changed afterada8b21. No frontend
or source event format changed; pre-pair reader, frontend/restore and native
evidence remain historical unless explicitly rerun. Exact commands:

```text
cargo test -p tig-store --lib placement::tests --locked
cargo test -p tig-store --lib store_protection::tests --locked
cargo test --workspace --locked
cargo build --workspace --all-features --bins --locked
cargo test --workspace --all-features --locked
cargo clippy --workspace --all-features --all-targets --locked -- -D warnings
cargo check -p tig-git --no-default-features --locked
cargo test -p tig-store --lib --locked retirement::tests::explicit_cli_pending_cancellation_qualification -- --ignored --exact
cargo test -p tig-store --test retirement_cli --locked pre_final_writer_refuses_new_manifest_and_accepts_legacy_control -- --ignored --exact
```

TIG_RETIREMENT_CLI names the fresh external all-feature binary; TIG_PRE_FINAL_CLI
names the pinned273e639 external artifact. Hosted exact-head qualification remains
separate from these local results and must be checked on the eventual PR head.

The first broad library attempt failed the size-conflict fixture during source
admission: it had already put the same hash at the original size in that job's
own predecessor. Source admission correctly rejected the local conflict before
the intended cross-namespace assertion. The corrected test uses separate fresh
namespaces with internally consistent but mutually conflicting sizes, and the
roster compiler refuses them. No product safety check was weakened for this test.

## Synthetic validation and scope

This rehearses VAL-002/003/004: with a shared immutable old object, a remote job
changes desired proximity and actual managed copies without losing M. Ending that
job cannot race the absence predicate used to remove D. Hard/additive requirements
and exact complete enrollment survive the advisory optimization. Unknown closure
releases all acquired guards and preserves bytes. This is not full native F/D/M
acceptance, latest clock/authority, real provider durability or user UX evidence.

Disposition: pass_with_risk for bounded synthetic qualification; hosted exact-head
qualification remains pending. The full PRODUCT_PLAN goal remains
incomplete, including bulk operator/native age/category integration, real large
files/providers, metadata lifetime/continuation and human acceptance.
