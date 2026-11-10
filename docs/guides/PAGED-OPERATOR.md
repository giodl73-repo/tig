# Paged retirement operator

Experimental, explicitly enabled operation over existing private roots. This is
the operator surface for the [paged controller](../../specs/PAGED-EXECUTION.md),
including its admitted age proofs. It does not enroll stores, discover consumers,
install an Action or accept live-provider use. Use synthetic fixtures while the
remaining authority/provider/adoption gates are open.

```text
tig-storage paged describe
tig-storage paged copies PROFILE REQUEST
tig-storage paged plan PROFILE REQUEST
tig-storage paged inspect PROFILE DECISION
tig-storage paged subjects PROFILE DECISION [CURSOR]
tig-storage paged actions PROFILE DECISION [CURSOR]
tig-storage paged peak PROFILE DECISION
tig-storage paged status PROFILE DECISION
tig-storage paged apply PROFILE DECISION
tig-storage paged resume PROFILE DECISION
tig-storage paged cancel PROFILE DECISION
```

The strict private `tig.paged-operator-profile.design-1` supplies `enrollment`,
`roster`, `admitted_context`, `metadata`, `namespaces`, `stores`, `witness`,
optional `clock` and `age_registry`, `limits`, `run_limits`, `permissions`,
`claim_release` and `final_loss`. Roots are absolute existing paths. Metadata and
source bindings contain `root` and `minimum`; store/witness bindings additionally
contain `ledger_root`, `job` and `place`. Clock bindings contain `root`, `minimum`
and the enrolled clock `enrollment`. Registry bindings contain `root`, `minimum`
and optional age `limits`. Keep these bindings outside source control.

All permission flags (`stage`, `apply`, `cancel`) default false. Staging grants
neither claim release nor deletion. For execution, the independently admitted
`claim_release` and `final_loss` must each equal the exact decision digest. The
operator never generates its own approvals or treats a hash as authority.
`resume` requires apply permission. Cancellation requires cancel permission and
both exact approvals, but needs neither clock nor age registry.

Copy requests use schema `tig.paged-copy-request.design-1`, optional `base` index
object and a bounded `copies` array with the existing bulk copy request fields.
The response returns `copies`, `count` and a finished/read-back metadata `minimum`.
Submit further chunks against that exact index. Repeating an identical entry is
idempotent; a conflicting request for the same store/object refuses before writes.
The total index can exceed the portable document limit while remaining bounded.

Plan requests use schema `tig.paged-plan-request.design-1`, `operation`, exact
indexed `selection` object, `copies` index object and `window`. A native indexed
candidate's `request` object is a valid selection input after separate native
admission. The response returns `decision`, `decision_id` and the new metadata
`minimum`. A caller must preserve the decision as canonical JSON, independently
admit its scope, and update its private metadata minimum. No source claim or
provider payload changes during staging. Provider observations can freeze held
copies; later availability cannot expand this decision's eligible set.

Inspect/subjects/actions require metadata only. Each page returns original rows,
total count and an optional next cursor tied to the decision and row kind. Supply
that cursor as a separate bounded JSON document. Pages contain at most64 rows and
bounded serialized bytes; repeat until next is null. These historical explanations
are not a current protection check. All documents/responses are at most64KiB.

Peak reports the required witness metadata reservation, not available disk space
or a complete provider capacity forecast. Status uses a metadata reader and no
clock. An available admitted registry can clarify inherited-age debt; unavailable
metadata/providers/sources remain unknown. Missing counts are null, not zero.

Apply/resume use the actual enrolled clock and the existing controller's source,
provider, grant and age fences. Run limits default to16 source pages and16 provider
actions per call (hard caps64/32); a pending result requires another explicit call
with the same decision and freshly valid admission. Completed effects are durable
history, not permission to repeat them or expand selection.

Cancel opens the witness first and acknowledges the global stop before opening
sources, metadata or providers for cleanup. Cleanup failure retains that stop,
reports unknown debt and can add `cleanup_error`. Cancellation cannot undo
retired history or deleted bytes. Subsequent resume cannot restart cancelled work.

After the operation is sealed, originally held copies can use a
[fresh held-copy follow-up](HELD-COPY-FOLLOWUP.md) with new approval and current
protection checks. It reads retained original scope without reconstructing the
retired predecessor or expanding the original action set.

Exact source/current-authority enrollment, native writer coverage, real-provider
and large-file qualification, journal/metadata lifetime, agent/UI/Action integration
and intended-use acceptance remain separate delivery obligations.
