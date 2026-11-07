# Public baseline: six-role producer review

Input 891c5b196447668deda80e33f1ba198ecacf86b3. WP-015/016,
REQ-006/008/010, SPEC-014, IF-010/013, PKG-008/009; DOCS + INTEGRATION
L0/L1, scoped L2 hosting/anonymous-read inspection. Targets: README, current
architecture/roadmap, doctrine/role wording, release/privacy controls and CI.

Allowed scope: public source preparation, generic documentation/templates and
repository settings. No Rust/runtime, identity domain, schema, provider, asset,
cache, enrollment, daemon or consumer-binding change. This is one producer
applying six checklists, not six independent reviewers or human acceptance.

## Role findings and dispositions

| Role | Finding / severity | Disposition and evidence |
| --- | --- | --- |
| Identity and Namespace Steward | Medium: public/private history and original evidence pins could be conflated | Canonical public identity verified independently of the retained private repository. Public refs contain only clean public ancestry. Historical pins are explicitly historical, not public reachability claims; exact source/tree pins remain in the operator audit. Rust identities and schemas unchanged. |
| Placement Policy Steward | Medium: publication/green CI could be read as lifecycle completion | README/architecture retain explicit desired/observed/authority boundaries. Catalogue movement/removal remains held; full jobs/dependency coverage and retirement remain open. Public source milestone is separated from product milestones. |
| Storage and Custody Steward | Medium: preparation docs still described a pending visibility change | Release/context now record the actual clean-source publication. Original history remains private and backed up. No source copy was retired and no cache action was performed. No custody or physical-reclaim acceptance is inferred. |
| Concurrency and Recovery Adversary | Medium: superseded CI runs and feature-only coverage | CI supersession is scoped by PR/ref, bounded to 60 minutes, with four test threads and debug/incremental settings matching the isolated qualification. All tests remain enabled. Standalone Git/LFS check added; no runtime/global cache lock introduced. |
| Provider and Privacy Steward | High: GitHub secret scanning/push protection disabled; stale historical ignore retained | Both hosting protections enabled. Public source/history scanned without the private-history exception. Private reporting/dependency alerts verified; no open dependency alerts observed. Private project names/paths/assets and credentials remain excluded; fixture example.invalid addresses are synthetic. |
| Git and Adoption Maintainer | Medium: main unprotected; onboarding and role/scenario wording stale | Main requires current PR/Actions checks and conversation resolution, including admins; no force push/deletion. Zero mandatory approvals avoids fictional independent review. README build/standalone CI and redacted synthetic issue/PR templates are aligned. Role cards and design-ledger wording no longer imply no component tests exist. |

Maintainer owns all dispositions. Closed means corrected for this public
experimental baseline; it does not mean full system/provider acceptance.

## Verification, validation, trace and review

Verification: run tools/validate.ps1, git diff --check and Rust fmt; verify
upstream pins/license notices and source budgets. Inspect changes to ensure no
Rust/Cargo dependency or runtime configuration changed. Scan the exact public
history/current tree without a broad allowlist. CI runs the README all-bin build,
full locked all-feature tests, strict Clippy and standalone no-default-features
check on Windows/Linux. Raw reports and settings payloads remain operator-side.

Validation: anonymous clone/source identity and generic synthetic integration
qualification apply to source usability. Public baseline qualification already
ran the all-feature suite, including the 500,000-binding workload, plus Clippy,
standalone and helper checks. This change repeats checks appropriate to docs/CI;
exact-head hosted results remain required before merge. Earlier UI task testing
does not establish full consumer or eviction acceptance.

Trace: this change closes public-facing documentation/hosting findings only.
Work-package states explicitly refer to full target scope; bounded catalogue and
asset implementations do not silently close proposed packages or prerequisites.
Stable requirements/invariants and the 110-case design ledger remain open at
their declared full-system scope; no scenario row is silently promoted to tested.
No independent validation or acceptance is claimed.

CI annotations also exposed deprecated Node 20 checkout/Python actions. Replace
them with official stable Node 24 releases pinned to verified commits; hosted
validation must pass those exact pins before merge. This is build tooling, not
a runtime/dependency update to Tig.

Review: producer-applied identity, placement, custody, concurrency, privacy and
adoption checks completed. No open high finding is accepted for public exposure.
Remaining product limits: full active-job/consumption coverage, independent
retained custody, measured reclaim/authorized payload retirement, remote transfer,
and stable package/version compatibility. Governance choices such as a community
conduct policy and automatic dependency-update PRs are maintainer policy, not
component safety proof or a requirement to claim a completed storage service.

Hosted checks, settings and scan results are verified at closeout; later setting
changes or new commits require fresh inspection. Do not treat this review as a
permanent security certification.
