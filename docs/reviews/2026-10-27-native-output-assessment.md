# Native output assessment producer review

Public summary: consumer identifiers generalized; original notes retained privately.

Entry36286fb12a815e19d51253c9260e2ce4f551967e; depends on PR33 qualification.
Controls, profile and L0/L1/L2 entry/exit are in specs/NATIVE-OUTPUT-ASSESSMENT.md.
Allowed scope: immutable metadata getter, optional native read-only adapter,
synthetic fixtures and bounded docs. No dependency, provider, runtime installation,
executable, cache enrollment, media IO or automatic source/selection change.

Pre-code review is recorded in the specification. Producer-applied post-code
six-role findings (not independent or human approval):

- Identity/namespace: resolve once, require desired checkpoint and expected
  asset/definition before evidence IO; claim subjects cannot replace current ones.
  Selected/named selectors remain distinct, and metadata key segments are literal.
- Git/adoption: evidence pin lives in the current exact record. No second editable
  registry or parent dependency list. Historical approved bytes without a claim
  remain Unproven; they are never assigned a fabricated producer receipt.
- Concurrency/recovery: metadata-only observation at one immutable checkpoint;
  no durable job capability or side effect. Reassess after source changes. Store
  reopen/detached-source fixture tests existing archive/provider integration.
- Placement: unavailable/malformed evidence blocks rather than becoming automatic
  rebuild advice. Admitted old producer semantics can differ from desired producer;
  missing admission is a separate block, not StaleRecipe.
- Storage/custody: use existing NodeStore only, no writes during assessment. No
  byte/media verification, complete global coverage, claim retirement or removal.
- Provider/privacy: pin bytes bounded before lookup, then exact hash/size parse;
  bounded literal locations; Debug prints categories/presence, not IDs, raw
  metadata, pins or provider errors. Explicit private result fields support triage.

Verification: initial six focused cases pass, then nine pass after adding named
selectors, producer separation and early-refusal cases. The tenth fixture initially
failed compilation because its archive restore arguments were reversed; corrected
that test call without changing the archive API. All ten focused tests then pass,
including actual current/original archive/store reopen with Git/records/definitions
offline and a trait-object resolver. No source claim, evidence or media is invented.

Residual risks: current source/admission authority remains caller-owned; parsed
unsigned claims do not prove actual consumption/execution, approval or custody.
The adapter does not run schema/variant rules, discover every output, queue jobs,
change storage placement or qualify private adoption. Generic APIs only; no
legacy consumer/REEL-specific metadata location is built into Tig. Final workspace/hosted
qualification remains required and is recorded separately below.

Independent read-only scope audit (not private acceptance): local legacy consumer
5a10e3e0f20d0be3a52a6c766ccd04a21017861b family index counts and canonical
consumer project c22fcce's51 .tig records were recorded in historical consumer-readiness note at that pinned tree. Existing
native fixture success cannot close other-family/historical intake requirements.
No media/cache read, source edit or production binding change was needed.

Final local verification: workspace --all-features --locked passes502 test entries
(including all ten native assessment fixtures). The500,000-binding metadata suite
passes171.74s. Workspace all-target/all-feature warnings-denied Clippy passes3.60s;
standalone no-feature tig-git check, fmt and structural/source-budget checks pass.
No code changed after these final runs. Logs are private runtime data outside Git.
Hosted exact-head Windows/Linux qualification and parent33 qualification remain
mandatory before merge; full intended-use and private adoption stay OPEN.

Review disposition: scoped component pass_with_risk, owner Tig producer. No
unresolved blocking component findings. Medium residuals (same owner): external
admission authority, unsigned evidence truth, schema/variant completeness and full
native custody/adoption remain open under the original readiness requirements;
passing this adapter must not be used to close those broader requirements.
