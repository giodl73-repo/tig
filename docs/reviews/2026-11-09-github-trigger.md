# Trusted GitHub intake producer review

Implementation input503adca, based on1c253ea; subsequent fixture diagnostics
and exact-source qualification are recorded separately. Controls/scope/entry/exit
and GIT/PROVIDER L0/L1/L2 are in specs/GITHUB-TRIGGER.md. Targets: optional
tig-git::github, CommittedSource, tig-trigger, shared private profile DTOs,
the inert Action client/example, qualification workflow and focused tests.
Method: one producer applying all six .roles checklists, not independent people,
human acceptance or deployment authority. Owner of each row: Tig maintainer using
the named lens. No private cache or runtime consumer is attached.

| Finding / role | Severity and disposition | Remaining proof / limitation |
| --- | --- | --- |
| GT-01 Identity / namespace | High: event fields cannot authenticate repository or choose roots. Verify original RS256 JWS under fixed-issuer keys, exact immutable IDs/ref/commit/workflow and private enrollment digests; strict duplicate-free bounded JSON rejects substitution. | Actual issuer/foreground compatibility must pass explicitly; exact local source listing/root/context compatibility is a separate fixture. Remote commit/tree association trusts admitted GitHub TLS/API authority rather than reconstructing raw Git objects. |
| GT-02 Placement policy | High: authenticated request cannot expand private copy/removal permissions. Default-false selection and four existing placement permissions remain independent. Partial pages and resumed suffixes cannot claim complete convergence. | Full same-basis nonempty synthetic paging, finite-budget holds and duplicate no-op observation are focused tests; full native/branch/draft/index coverage and final-deletion policy remain separate work. |
| GT-03 Storage / custody | Critical retained boundary: signing proves provenance only. All payload work reuses existing accounted copy, retained ingest/handoff and all-namespace/keeper removal fences. Receiver grants no final-copy authority. | Synthetic three-store reads verify actual copies; real issuer empty-scope qualification cannot prove payload custody. Large files, provider durability, actual free-space reclaim and real failure-domain independence remain open. |
| GT-04 Concurrency / recovery | High: late source drift or expiry cannot become success. Check request validity/current head between bounded pages, retain partial progress on later failures and deduplicate hold reasons as they arrive. Existing journals preserve original effects/retries; no new event journal or global cache lock. | Signed expiry, observed clock rollback, outage, duplicate retry and post-copy head drift fixtures; inherited provider process-restart tests remain separately scoped. No instantaneous revocation, dynamic epoch handoff, hard call preemption or unattended partial cleanup is promised. |
| GT-05 Provider / privacy | High: untrusted request must not open roots or forward bearer credentials. Bounded loopback HTTP authenticates before dispatch, fixed-origin HTTPS separates API credentials from issuer keys, and pointer capture never opens excluded payloads. Profiles/credentials stay private; static errors omit raw bodies/paths/tokens. | External TLS ingress, OS/provider isolation, rate admission, real GitHub API compatibility and actual service operation require their own qualification. Malformed HTTP negative tests prove no dispatch; no service is installed. |
| GT-06 Git / adoption | High: workflow completion is not storage convergence. Inert client reports authenticated acceptance separately, skips stale events explicitly and leaves pending/unknown convergence nonzero. Current source comes from independent immutable-repository/ref checks. | Action example is not enabled for a consumer. Missed-event catch-up requires another admitted invocation. Local Git integration remains feature-independent; broader user acceptance and remaining roadmap milestones stay open. |

Review also retained two final-deletion entry requirements from the roadmap pass:
cancellation after historical-claim settlement must restore admitted protection
or truthfully expose retired-but-present state; approval validity must be rechecked
against changed selection/use/policy/authority before each pending removal. Neither
requirement is implemented by this ordinary-placement receiver.

Hosted issuer review corrected an invalid not-before/issued-at coupling: each
claim is now checked independently against current time and the admitted lifetime.
Signed backdated-not-before regression vectors pass without relaxing issuance,
future-not-before or expiry checks. Actual issuer run42546575041 passes atb3aced0
through the foreground binary with an empty fixture scope and zero payload IO.
Earlier hosted line-ending drift was corrected in canonical reviewed pins.

Disposition: pass_with_risk for bounded implementation and local qualification
under EVID-094's explicit broad/final-focused timing. Hosted issuer/Linux/secrets
pass atb3aced0; Windows and later current-head checks remain pending. Do not infer full
priority-6 completion, independent approval or live-operation acceptance. Exact
verification, intended-use validation, trace and final review disposition must be
recorded separately after current-source checks. Hosted failures and corrections
remain evidence; an ignored issuer test is never an executed pass.
