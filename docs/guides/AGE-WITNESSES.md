# Age evidence for historical retirement

Experimental metadata-only operator interface. It does not release claims, approve
final deletion or run payload operations. Read the [contract](../../specs/AGE-WITNESSES.md)
and [bulk retirement guide](BULK-RETIREMENT.md) before composing candidates.

Age belongs to a namespace/source/predecessor occurrence. A trusted capture observes
that the exact version is already historical, then samples the independently
enrolled clock. The latest bound is conservative: a version first observed today
does not acquire last year's age from a Git date, file modification time or label.
Unknown age holds selection until evidence or another separately authorized scope
is available. Other current, recent, draft and job obligations remain independent.

## Explicit existing-root commands

```text
tig-storage age describe
tig-storage age initialize PROFILE
tig-storage age capture PROFILE REQUEST
tig-storage age status PROFILE REQUEST
tig-storage age assess PROFILE REQUEST
tig-storage age inherit PROFILE REQUEST PLAN
```

The strict private `tig.age-profile.design-1` document contains `enrollment`,
`registry` and ordered `observers` (existing root and trusted source minimum),
`metadata` (existing root and metadata minimum), optional `clock` (existing root,
capacity minimum and LocalClock enrollment), optional `required_age_millis`,
optional owned `subset` enrollment, `permissions` and bounded `limits`.
Every root is absolute and machine-local. Enrollment binds metadata/namespace/
policy/authority/epoch/owner/class/clock identities; external current minima and
writer admission remain operator responsibilities. Portable input cannot relax
those private boundaries. Metadata inspection needs the actual admitted provider.

`permissions.initialize`, `permissions.capture` and `permissions.inherit` each
default false. Permission/static admission is checked before root IO. Initialize
only adopts the identity root of an already enrolled empty Retention source; it
does not create a journal, discover stores or reset acknowledged facts. Repeated
initialization preserves existing observations. The registry namespace is distinct
from every observed namespace, so capture never edits the observed journal.

A request has only `namespace`, `source` and `predecessor_revision`. Capture accepts
no timestamp. Its first successful observation uses an explicit clock profile;
acknowledged retries preserve the original evidence and do not open/sample the
clock. Capture may finish bounded metadata, adopt its registry root and retire
superseded registry roots after proving all their bindings survive. Interrupted
cleanup resumes without dropping facts. Unacknowledged metadata may remain after
interruption; this interface does not scan or sweep it.

Status has no writer or clock capability and reports historical facts. Assess uses
the private `required_age_millis` and may append a clock observation. An unknown
fact or missing historical occurrence produces a hold without sampling time.
Missing/corrupt acknowledged evidence produces recovery failure, never zero age.
Both responses deny claim-release and payload-removal authority.

Inherit accepts the existing exact retained-subset plan. It proves the parent
observation, recomputes the unchanged survivor projection and verifies actual
source transition receipts. It retains parent/plan references and preserves the
original observation. A proposed/unexecuted plan or matching hashes alone cannot
grant inherited age. No clock profile is required for inheritance.

Reports distinguish registry, source and metadata checkpoints. Persist newly
acknowledged minima outside this repository under the operator's existing trust
boundary. Unsupported authority, unavailable roots, exhausted budgets or journal
lifetime hold progress; there is no implicit fresh epoch or fallback clock.
One metadata read/write/byte/time budget spans all phases of each call, including
cleanup and persisted readback. Budget exhaustion may leave acknowledged facts or
unfinished cleanup; a later retry preserves those facts and does not reset age.

## Native category and block selection

```text
tig-retirement select-aged PROFILE REQUEST
```

The schema-feature binary accepts a separate strict
`tig.native-retirement-aged-profile.design-1` profile. `selection` is the existing
native select profile; `age` supplies the exact age enrollment, registry
root/minimum, clock root/minimum/enrollment, private `required_age_millis` and
optional age limits. The request remains the existing exact asset/category filter;
it cannot supply an age or timestamp. Plain `select` retains its original contract
and rejects age fields.

The adapter verifies native header/path/owner/predecessor proofs, assesses the same
historical occurrence and reobserves the source. Unknown/recent age yields a null
bulk selection with an explicit assessment. An eligible response supplies an
existing `bulk::Selection` candidate; independently admitted final-plan consent,
complete current protection and provider fences remain necessary. The native
preview itself reports no payload verification or removal authority. Assessment
may write only the enrolled clock journal; source, registry and metadata inspection
remain observational.

## Remaining gates

This is a bounded local protocol, conditional on admitted writers/native OS time
and externally current authority/minima. Real provider/large-file qualification,
live native lifecycle adoption, UI/Action, operational journal continuation and
human acceptance remain separate. Full metadata retirement must preserve all age
record/parent/plan/source dependencies or hold expiry. A bounded registry does not
solve long service lifetime by deleting facts. NB-07's more-than-MAX_ROOTS sharing
requires exact global consent and separately authorized historical-claim settlement;
repeating current bounded final batches cannot retire that whole shared block.
