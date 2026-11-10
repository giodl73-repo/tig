# Source settlement producer review

Input9c2a29f, implementation51c5acc. Controls/scope/entry/exit and GIT/PROVIDER
L0/L1/L2 are in specs/SOURCE-SETTLEMENT.md. Targets: conditional source admission,
unchanged Withdraw journal path, optional exact ref lookup, settlement library/CLI,
shared private local placement, and committed/index/GitHub convergence checks.
Method: one producer applying all six .roles checklists, not independent reviewers
or human approval. Owner of every row: Tig maintainer using the named lens.

| Finding / role | Severity and disposition | Evidence / remaining gate |
| --- | --- | --- |
| SS-01 Identity / namespace | High: Git failure is not ref absence, and an old settlement cannot withdraw a reactivated source. Require exact successful ref enumeration, private source/owner/class enrollment, canonical plan and expected source state/revision. | Prefix neighbor, blob ref, unavailable repository, source reactivation and old retry cases pass. Durable rename binding, old-controller revocation and authority/restore migration remain open. |
| SS-02 Placement policy | High: a commit existing is insufficient. Draft must be adopted/conflict-free; current adopted successor must cover every exact draft byte. Withdraw only current into predecessor; never release another owner or history. | Staged/unadopted/missing metadata/conflict cases hold; actual staged/committed/branch-end/job-end flow preserves active jobs and retained history. Native/private closure and final-retirement settlement remain separate. |
| SS-03 Custody | Critical retained boundary: source settlement grants no payload deletion authority. Reuse ordinary placement/keeper fences and independent default-false permissions. | Actual temporary three-store CLI demotion preserves/read-verifies old retained bytes; no final backup removal. Real providers/domains, large files and measured free-space reclaim remain open. |
| SS-04 Concurrency / recovery | High: successor may change after inspection. Check up to eight sorted source revisions inside the existing short transaction after catch-up; preserve historical receipts without regranting conditions. | Changed witness during tree IO refuses; unrelated source progresses. Two settlement child exits and legacy index exits recover. Git/conditions/current settlement are separate observations; no Git/SQL transaction or global cache lock. |
| SS-05 Provider / privacy | High: shared preflight pins private owner/authority/policy/namespace root and existing providers before mutation. Later failures preserve acknowledgement. | Index/settlement suites cover missing/bad/default-disabled profiles and offline status. No paths, credentials or executable hooks enter portable plans. Live ingress/OS/hostile-filesystem and provider adoption remain unqualified. |
| SS-06 Git / adoption | High: unchanged Git ref does not establish current admitted selection. Recheck committed/index/GitHub source state before convergence. | Signed request with independently withdrawn source stays pending; Git drift and condition drift remain distinct. No hook/event router is installed; full automatic/native lifecycle and user acceptance remain open. |

Source/docs budget review (PKG-009): implementation checkpoint is 5,240,121 bytes,
leaving only 2,759 bytes under the former 5 MiB ceiling before required evidence/
review closure. Admit a bounded 6 MiB source/docs ceiling for generic controllers,
synthetic tests and trace. Preserve 128 KiB/file and all runtime/media/credential
exclusions; no provider object, journal, reservation or runtime budget is raised.
Do not omit engineering evidence to fit the former source-only limit.

Disposition: pass_with_risk for bounded implementation and focused qualification.
EVID-096 separately records broader local/hosted verification, synthetic validation
and trace. No final deletion, metadata expiry, full requirement/VAL, independent
review or live-operation acceptance is closed. Conditions are ephemeral predicates;
unchanged event bodies do not become durable proofs of present successor custody.
