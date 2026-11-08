# Source-preserving S3 producer review

Input 3fd9918c30295e9a8ca0cf943e076408bde5e3b1;
implementation 4fc3060c9c6ecb262cd1b964146ea5087af9a397.
WP-003/005/006/007/014/015; REQ-001/003/004/005/006/007/009;
SPEC-002/003/004/007/008/009/013; IF-002/003/004/005/006/012/013;
PKG-002/003/004/008. RUST/PROVIDER profiles. Scope: optional conditional S3
PUT/full readback, existing accounted metadata witnesses, explicit CLI and isolated
protocol/process fixtures. Entry/exit and L0/L1/L2 are in
[the contract](../../specs/S3-PUBLICATION.md).

Method: one producer applies all six .roles parliament checklists against
model/safety, publication/accounted/ledger/orchestration contracts and foundation
T01/07/08/09/10/12/13/14/15/16/21/25. Owner of each finding: Tig maintainer using
the named lens. No delegation, independent review or human approval is implied.

| Finding / lens | Severity, disposition and remaining qualification |
| --- | --- |
| S3-01 Identity | High: excluding every query string from endpoint identity could hide a versionId/multipart selector. The parser now admits only recognized authentication/operation annotations, rejects duplicate/unknown query keys and binds exact scheme/authority/path plus target instance/generation. Credential refresh preserves identity while representation changes refuse. Portable plans have no URLs/paths/credentials. Actual AWS signature/IAM qualification remains separate. |
| S3-02 Placement | High: a portable request must not broaden private admission. CLI profiles independently bind target, source/witness Place, owner/context and finite budgets; upload defaults disabled. Existing source publication is freshly verified under its real held reader. Root absence and unsupported retention fields refuse without provider initialization. Source protection/removal policy is unchanged; this adapter offers no placement optimizer or deletion action. |
| S3-03 Custody | High: upload acknowledgement/checksum/ETag alone is insufficient. A complete fresh GET verifies actual length/hash; source remains protected. Conditional writes preserve the existing winner; wrong content/readback and acknowledged remote loss refuse silent overwrite/re-upload. Receipts deny retained custody and removal authority. Object Lock, independent failure domains, provider durability and remote quota are not qualified here. |
| S3-04 Recovery | High: remote effects need bounded prior intent and later completion. Both small witness allocations/bindings are admitted first through existing accounted publication; intent precedes remote IO, completion follows readback. Lost reply resumes with GET before another PUT. Known settled but missing witness evidence holds, never repairs. Exact changed-plan reuse refuses before HTTP. Status adds no commands and separates history, availability and accounting settlement. Whole-authority restore/continuation retains its existing gate. |
| S3-05 Provider/privacy | High: private access must not follow redirects or ambient proxies, leak in errors, or silently change representations. HTTPS is required except explicit literal-loopback fixtures; redirect/content-encoding/partial-response paths refuse. Errors are static and Client Debug is redacted; the CLI installs no transport logger. Library hosts must disable transport trace logging, which can expose query credentials. Header/body/document/request/time bounds are explicit; payload IO counters are not NIC traffic, billing or remote quota. |
| S3-06 Git/adoption | Medium: optional network support must not become a standalone Git requirement or activate from opening a repository. S3 is feature-gated and uses explicit existing roots/jobs/private profiles. Original copy/retire interfaces and records stay compatible. No service, cloud account, runner, hook, consumer or real store is configured/deployed. LFS upload, Git lifecycle reconciliation and live provider adoption remain separate deliverables. |

Disposition: pass_with_risk for the bounded source-preserving protocol interface,
subject to separately recorded verification. This does not close the full cloud/
LFS milestone, complete requirements, independent review, actual AWS account/IAM/
TLS/custody, large-file/native qualification or operational lifetime gates. No
retained-backup or source-eviction authority is inferred from this result.

Failure memory: the synthetic TCP listener was nonblocking and Windows accepted
sockets inherited that mode. Reading before request bytes arrived closed the
connection and surfaced intermittent host aborts. Accepted fixture sockets now
explicitly use blocking mode with finite timeouts; no production retry loop was
added to hide the fixture defect. Review also required explicit witness/settlement
checks before remote IO, so missing acknowledged local metadata cannot look like
a never-started upload.

The actual two-process fixture exposed stale provider/ledger sampling and an
insufficient operation-guard lifetime. Provider evidence now precedes current
ledger observation, with re-read after settlement. The existing admitted
completion-attempt guard spans remote IO; peers acquire completion before intent,
then release it for the ordinary accounted completion publisher. Same-operation
contention reports busy, while another job uploads under its own scope. Fifteen
focused cases pass, including actual peer contention and unrelated-job progress;
final workspace qualification is recorded separately after the corrected source.

EVID-090 records final corrected-source default/all-feature workspace suites,
strict Clippy, standalone Git, fmt/L0 and the separate retirement CLI qualification.
Both broad suites were rerun after the concurrency fix. Hosted checks and
intended-use/operator acceptance remain distinct from these local results.
