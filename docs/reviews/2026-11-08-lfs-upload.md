# Standalone LFS upload producer review

Input 9b8a6890e0074a58851a8d313dcb9b841fd7d613; implementation
2f1173bd2991af429911b251ceaa173916077ee3. Controls and allowed scope:
[LFS-UPLOAD](../../specs/LFS-UPLOAD.md), WP-003/005/006/007/014/015;
REQ-001/003/004/005/006/007/008/009; SPEC-002/003/004/005/007/008/009/013;
IF-002/003/004/005/006/008/009/012/013; PKG-002/003/004/007/008.
GIT/PROVIDER profiles. Targets: optional tig-git upload/CLI/protocol, shared
upload_witness composition and S3 compatibility, contracts and synthetic fixtures.
Entry: exact authorized OIDs, existing verified readers and accounted publication.
Exit: source-preserving Basic upload/readback and honest bounded recovery/admission.

Method: one producer applying all six .roles parliament checklists against model,
safety, foundation scenarios and current provider/publication contracts. These are
review lenses, not six independent reviewers or human/live-operation approval.
Owner of every finding: Tig maintainer using the named lens.

| Finding / role | Severity | Disposition / evidence | Remaining qualification |
| --- | --- | --- | --- |
| LU-01 Identity and Namespace | High, resolved for this slice | Canonical plans bind exact OID/size, source authorization root, owner/context, remote instance/generation/Batch/ref/scope, witness and capacity/budgets. Credentials are excluded; refreshed credentials preserve identity. Wrong OID/path, stale context and noncanonical plans refuse. Existing S3 schemas/body/key/context formulas are preserved by extraction review and all 15 regressions. | Real-server enrollment and latest-authority/full-restore recovery; new wires remain experimental. |
| LU-02 Placement Policy | High, retained boundary | Init remote and exact authorized source path cannot become arbitrary upload authority. Writes default disabled, budgets are finite, and no source/witness/ledger is initialized. Missing-root fixtures and source-preservation assertions pass. Ref metadata does not infer lifecycle placement. | Trusted Git lifecycle selection, complete native/job admission, quota and sustained operation. |
| LU-03 Storage and Custody | High, corrected | Readable remote bytes alone could skip a failed verify notification. Unfinished operations now perform upload Batch admission and any requested verify before full final readback/completion. Lost PUT/verify replies, denied verify, corrupt/missing readback and acknowledged remote loss are tested. Receipts explicitly deny custody/removal authority. | Actual provider retention/durability, independent backup custody, large files and final deletion. |
| LU-04 Concurrency and Recovery | High, resolved for bounded protocol | Existing completion-attempt guard spans remote IO. A killed actual uploader leaves recoverable intent, blocks its peer and allows another job through the same ledger/store; resume sends no second PUT. Settled missing evidence holds without repair; status adds no commands. | Full authority/epoch handoff, journal continuation, metadata expiry and noncooperative/power-loss behavior. |
| LU-05 Provider and Privacy | High, corrected | Explicit origin/decoded-prefix/header scopes admit action credentials without implicit Batch-token forwarding. Ambiguous scopes, duplicate/conflicting headers, routing/framing overrides and incompatible fixed media types refuse. Relative/absolute expiry and deadlines are checked. Plans/errors contain no credentials, redirects/proxies/helpers are disabled, and TLS is required outside literal-loopback fixtures. | Actual TLS/authentication/server capability qualification. Library hosts must disable sensitive transport trace logging. |
| LU-06 Git and Adoption | Medium, resolved for client seam | Real Git LFS push uploads two exact source objects, verifies registration, reuses existing bytes and preserves source. Standard upload completion has no path; per-object errors permit the next request. Existing download clients regress successfully and the default normal dependency graph excludes HTTP/TLS/typed assets. | Explicit operator installation, trusted merge automation, real consumer/UX acceptance and release. |

L0: fmt, portable validator, target hashes, source budget and diff checks pass.
L1: locked default/all-feature workspace suites and strict all-feature/all-target
Clippy pass. L2: 13 upload integration tests, actual Git LFS 3.4.0 push, real CLI
and killed-process recovery; 15 shared-witness S3 regressions and separately invoked
retirement CLI qualification pass locally on Windows. The final missing-root
assertions were added after the broad-suite build, then all 13 upload cases and
strict Clippy were rerun successfully. Product implementation did not change.
Hosted current-head Windows/Linux qualification is a separate pending check.

Disposition: pass_with_risk for bounded source-preserving upload only. EVID-091
separates verification, synthetic intended-use rehearsal and trace. No full
requirement/workflow, independent review, production custody, native/provider
adoption, automatic merge action, bulk final deletion or metadata lifetime closes.
