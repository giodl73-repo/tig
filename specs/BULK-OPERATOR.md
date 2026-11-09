# Bulk operator inspection

Input36fc91bdd4161917e7ab5c1e9f2a078041c7033d.
WP-003/004/007/008/009/015; REQ-001/003/004/005/007/009;
SPEC-003/004/005/008/009/014; IF-003/004/005/006/008/012;
PKG-002/003/008/009. RUST/PROVIDER profiles.
Allowed: bounded explicit CLI plan/peak/status and a writer-free library
Inspection port over existing bulk primitives. Synthetic fixtures only; no
provider initialization, copy, source settlement, grants, deletion or clock
fallback. Entry: independently admitted enrollment/roster, exact historical block
request and existing provider/metadata/ledger roots. Exit: actual CLI reproduces
the library plan, shows held/shared scope and reports interrupted/completed batches
without command/journal changes, including source metadata offline.

## Interface and private admission

`tig-storage bulk describe` performs no IO beyond stdout. Operations are
`plan PROFILE REQUEST`, `peak PROFILE PLAN`, `status PROFILE PLAN`. There is no
apply/resume/cancel/reacquire permission in this first operator surface. Complete
operator execution, independently admitted native clock/authority, native age/
category selection and UI integration remain required product work. Inspection
is a prerequisite, never a substitute for those requirements.

The separate [execution interface](BULK-EXECUTION.md) adds explicit
tig-storage bulk execute apply/resume/cancel under a distinct private profile.
It does not change inspection permissions or acquire a writer for status.

The strict private profile schema is tig.bulk-inspection-profile.design-1:
enrollment, complete roster, admitted_context, metadata {root, minimum}, namespace
{root, minimum} entries, at most four stores and one witness. Each store names
root, ledger_root, ledger minimum, job and portable Place. Roots must be absolute,
provider identities distinct, and namespace IDs exactly match the roster. The
witness matches the enrolled non-removable store and capacity. Copy requests
and action job bindings must reference these independently supplied store/capacity
identities. No portable request or plan supplies a physical path or lowers minima.
Each namespace entry uses exactly its admitted roster minimum; conflicting floors
refuse rather than choosing a weaker one. Plans record a desired witness, not an
availability acknowledgement; peak/status validate the actual witness enrollment.
All documents and responses are bounded to 65,536 bytes; plans retain 48 KiB and
the existing root/path/object/action limits. Growing reports remain asset-side.

Plan opens only an existing read-only MetadataReader and existing named namespaces
and stores. Unknown/missing provider copies remain held rather than inferred absent;
unknown source/metadata closure cannot produce a partial executable plan. No
initialization, inventory scan, hydration or repair is attempted.

## Writer-free historical observation

bulk::Inspection takes the privately admitted metadata identity, namespace
observations, exact per-action jobs and witness/provider handles. It has no
MetadataStore, Clock, deletion cookie or mutation method. Existing Execution
status delegates to the same inspection/control validation, retaining wire bytes
and outcomes. An identity is not byte integrity or availability evidence.

Status/peak do not open source metadata at all. Missing source/provider handles
become explicit per-action/source holds; the acknowledged witness must be available
to distinguish not-started from missing historical evidence. Wrong available
provider identity refuses. Historical completed/deleted evidence stays historical
and current observed presence is unverified. Status never authorizes removal,
repairs spent missing intent, recreates providers or publishes control records.
Ledger/source synchronization may refresh existing derived views; no command is
added and no content, source obligation, grant or deletion witness changes.

L0: fmt, reviewed hashes, portable validator, whitespace and six producer lenses.
L1: focused bulk regressions, strict profile/response/discovery tests, locked
workspace default/all-feature, strict Clippy and standalone Git. L2: actual fresh
CLI artifact qualification over complete/injected-boundary batches, offline source
metadata/provider, journal/payload preservation and redacted static errors. The CLI
artifact qualifier is explicitly ignored/exact and separately run locally/CI;
ordinary ignored suite entries are not proof. Full native/provider/clock/authority,
large-file/lifetime, operator execution and human acceptance remain open.
