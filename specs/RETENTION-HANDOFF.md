# Owned retention handoff

Input f3db4886541fe3abd34470abf422b392be80e508. Bounded WP-003/004/006/007/008:
REQ-002/003/004/005/007/009; SPEC-003/004/005/008/009; ARCH-002/003/007;
IF-003/004/005/006/012; DES-003/004/005/008; PKG-002/003; PROVIDER profile.
Entry: budgeted permanent local retention and verified live replicas.
Exit: exact explicit owner authorization, verified protected successor, durable
handoff before old claim removal, bounded accounting and restart-safe recovery.

```text
exact plan -> explicit owner authorization -> reserve metadata peak
    -> typed bound handoff allocation + exact attempt fence
    -> verify successor grant/bytes, hold successor shared guard
    -> source exclusive guard (nonblocking, same object)
    -> recheck old claim/bytes -> immutable handoff marker
    -> remove ONLY old claim metadata -> measure new metadata -> settle
```

No payload deletion or copy is performed. Only the old grant's owner can authorize
this exact handoff; the request and job owner must match. Successor bytes must be
identical, in a distinct store and distinct explicitly admitted failure-domain
label, with acknowledgement at least as strong as the old publication. Labels
are assertions, not native physical proof. Complete policy/claim authority remains
required before eventual payload eviction.

The successor's live guard prevents its own release while the source is retired.
Simultaneous reciprocal handoffs cannot acquire both source exclusive guards;
they defer nonblockingly. A released grant cannot serve as a successor. Later
handoffs may move custody onward, always protecting another active grant first.

Local manifest design-5 adds a required releases root. Immutable handoff markers
are keyed by object and old owner/operation; retired operations never resurrect.
Only active claim files are enumerated (at most 16 per object); historical markers
use exact lookups and remain outside Git. Marker-before-removal permits a crash
after either step: replay verifies the matching marker, removes only the exact
old claim if still present, and finishes accounting without redoing the decision.
Settled replay is read-only and returns historical disposition, not current custody.

Unknown/malformed metadata blocks action. Missing claim metadata without a matching
handoff marker is an integrity incident, not a retry. An installed marker already
invalidates the old grant even if settlement is pending. Old immutable grant/marker
evidence remains available; ordinary release never expires a claim by timeout.
Source and successor guards remain object scoped; no DB transaction spans IO.

Metadata is admitted/charged under the conservative allocated-file-data basis.
Namespace/ledger overhead and physical reclaim remain separately unproven. A new
successor or changed request needs new authorization. No stale-backup authority,
hostile namespace, native independence, full claims or final archival deletion
is established. Only synthetic temporary roots are used; no live consumer switch.
L0 structure/fmt/VTRACE; L1 locked workspace/Clippy; L2 actual marker/removal,
concurrency and process interruption. Full VAL-002/003/004 remain OPEN.

## Implemented evidence

EVID-020 adds 16 entries including a subprocess helper; workspace total 246.
Commands: cargo test -p tig-store --lib --locked retention::handoff::tests;
locked workspace, fmt and all-target Clippy. Temporary fixtures cover six injected
boundaries, three actual child exits, exact authorization/budget/type checks,
corruption/missing records, changed successor, preserved payloads and onward custody.
A bounded channel rendezvous forces reciprocal successor guards to overlap before
source acquisition; workers are joined before assertions. No deadlock, dual
retirement, physical reclaim or production native-independence claim is inferred.

Released claim metadata frees its active slot while immutable markers remain at
exact lookup paths. Historical evidence never reinstates an old grant, and settled
replay cannot repair a missing marker under spent budget. No native cache action
or final-object eviction is enabled. The source/docs budget change is separately
controlled in [the source budget](../docs/SOURCE-BUDGET.md); growing runtime history is not ordinary Git data.
