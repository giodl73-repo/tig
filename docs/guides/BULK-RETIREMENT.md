# Inspect an exact historical block

The experimental operator commands inspect the existing bounded bulk controller.
They do not retire source claims, release grants, delete bytes or approve a plan.
Final-deletion execution and native age/category selection remain separate work.
See [the operator contract](../../specs/BULK-OPERATOR.md) and
[bulk protocol](../../specs/BULK-RETIREMENT.md).

```text
tig-storage bulk describe
tig-storage bulk plan /absolute/private-profile.json /absolute/block-request.json
tig-storage bulk peak /absolute/private-profile.json /absolute/exact-plan.json
tig-storage bulk status /absolute/private-profile.json /absolute/exact-plan.json
```

Capture the plan's stdout outside source Git and pass that exact canonical JSON
to peak/status. Commands emit one JSON document and static error codes; no normal
stderr diagnostics or private filesystem paths appear. Documents/responses are
limited to 64 KiB; plans to 48 KiB. Limits hold rather than silently truncate.

An independently admitted private profile supplies the enrollment, complete
source roster, owner/context, actual provider/capacity identities and jobs, metadata
checkpoint and explicit existing roots. It must be prepared by the enrolled
application/operator. Do not manufacture a lower minimum or infer complete coverage
from a catalogue or a branch head. Paths, credentials and runtime evidence stay
outside project Git. No command here creates a store, ledger or namespace.

The block request uses exact namespace/source/predecessor/path tuples. A displayed
name, filename prefix, file modification time or Git date cannot substitute for
that identity. Copy entries identify exact original publications and enrolled
capacity authorities. The preview binds resolved objects, held reasons, filtered
historical subsets and eligible provider actions. Shared current/job use and
unselected aliases remain protected; an empty copy scope produces holds.

Read the plan before interpreting counts. Selected bindings and unique objects
are different, and one object's copies may occupy several stores. Status separates
selected/eligible unique bytes, planned copy bytes, newly removed logical copy
bytes, historical deletion, prior allocation and metadata peak. It grants no
removal authority or volume-free-space guarantee. A completed historical batch
does not prove current bytes or current protection; read each action's observation.

Status and peak need the enrolled metadata identity but never open its root or a
metadata writer. Source metadata can be offline. Missing namespace/provider
handles appear as explicit holds; absence is not reported as deletion. The witness
must be available to inspect historical intent/terminal evidence. Missing spent
evidence cannot be repaired by status. A successful plan alone does not acknowledge
witness availability; peak/status validate that existing enrollment separately.

This surface intentionally has no apply/resume/cancel/reacquire operation. It does
not make the full retirement workflow complete. Execution needs separately admitted
clock/current authority, exact consent and enabled policy; native selectors,
operator/UI integration, real providers/large files, journal/metadata lifetime and
human acceptance remain required before sustained use.
