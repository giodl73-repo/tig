# Native instance records and separate definitions

Status: experimental design-1 fixture profile, not a released wire format.
The bounded reader in [native intake](NATIVE-INTAKE.md) now checks envelopes and
exact pins. [Declared-reference queries](NATIVE-GRAPH.md) now use caller-admitted
profiles; full semantic rules and complete native consumer extraction remain open.
The .tig file is instance data on the Git side of the contract.
It is not the type definition or a replacement for standard Git LFS pointers.

```text
definitions/cel.json ---> separately pinned authored JSON Schema / role rules
         |
         v
scene-04/cel-b.tig -----> values + ordered stream bindings + selected result
                                             |
                                             v
                                  immutable byte objects
```

A definition owns fields/defaults, stream grouping/roles/order/cardinality,
reference meaning, permitted operations and status vocabulary. An instance pins
the exact definition and supplies actual values and byte references. Definitions
may use ordinary JSON/JSON Schema plus admitted declarative semantic rules;
they cannot smuggle arbitrary executable code into a manifest.

The proposed generic fixture envelopes are in schemas/. Definitions and authored
field schemas stay separate; JSON shape validation alone does not prove native
reference extraction, authority, production currency or output compatibility.
Each instance's streams can be empty while planned. Missing results remain null,
not invented byte objects. Physical cache paths and credentials are not bindings.

## Exact pins and fixture identity

In design-1 fixtures, sha256/bytes pins identify exact UTF-8 file/object bytes.
An instance or definition does not include its own digest. An instance reference
with id/revision uses the referenced instance file's raw SHA-256 in this profile.
This byte identity is not a released semantic revision/fingerprint algorithm.
Changing metadata can change the file hash without implying a producer rebuild.
Consumed-value projections remain a separate, unimplemented semantic mechanism.

The optional selected binding identifies an immutable result object/manifest.
Each ordered stream has a stable slot, explicit role/group and exact object pin.
The type definition validates applicable stream rules; the generic envelope only
checks wire shape. Source/import lineage records old identities without claiming
that old and new hashes mean the same thing. Definition changes require explicit
instance revalidation/migration and cannot reinterpret frozen historical pins.

## Explicit mixed-stream merge

```text
resolved instance:  slot-1=ours, slot-2=ours, slot-3=ours,
                   slot-4=theirs, slot-5=theirs
```

That is a legitimate new selection. Preserve each stream's provenance and its
declared ordering. Validate the actual result against the pinned definition,
including cross-role constraints and consumed-input/output relationships that
the definition declares. Do not reject unusual mixing or require one production
attempt for all streams unless the definition explicitly imposes that condition.

Merge tooling should expose semantic conflicts before accepting the candidate.
A trusted local merge driver can return an unresolved conflict; protected CI/
merge-queue checks enforce target policy on the actual candidate even if local
hooks are bypassed. A PR cannot weaken its own trusted validation policy.
Missing semantic facts are unknown, not automatically valid.

Structural validity, source selection, output freshness, review/authority,
delivery state and storage redundancy are separate facts. A valid source record
may intentionally retain a stale last-produced result while waiting for render.
Do not forge that result's old provenance to match new source inputs.

## Deferred make-style production

Source fixes can merge today and renders can catch up later. Independent audio
and cel changes yield one combined desired recipe. A ready-output/delivery gate
blocks stale output being represented as current; source merge need not wait for
render unless the project explicitly configures that gate.

An explicit catch-up run freezes chosen roots, source commit, definitions,
profiles, variants and producer pins. It resolves exact consumed dependencies,
reuses current results and builds only affected requested targets. Queue entries
may coalesce before admission; active jobs keep frozen inputs. New fixes during
the run create later demand, not mixed inputs in the active render. Producing
one render instead of two requires combining changes before admitting that render.
No scheduler, background worker or one-render guarantee is implemented yet.

The private legacy consumer recast is a schematic compatibility fixture. Planned instances
and imported metadata are not proof that every primary media input is present or
that the current native renderer can consume these records.

Authored schemas now have an explicit optional Rust execution path under
NATIVE-SCHEMA-VALIDATION.md. This is separate from envelope binding and does not
interpret the definition's stream_rules/semantic_rules maps or native controls.
A borrowing native validated resolver checks every captured current/retained
record; expanding the source requires a new check. It grants no approval/custody.

Explicit local rule execution now uses the versioned declarations in
NATIVE-LOCAL-CONTRACTS.md. It is opt-in and does not reinterpret legacy maps.
Only declared stream/whole-record JSON Schema assertions execute; graph/native
semantics, authority and custody remain separate and full coverage stays false.
