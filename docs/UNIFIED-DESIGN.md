# One Tig product, optional asset semantics

Decision: incorporate Art's generic asset-management value into Tig's design.
This supersedes the earlier proposed separate Art-product/adapter boundary.
Pure identity/variant slices and optional native/graph/history facilities now
exist. Broader tables below describe target capabilities; see the implemented
[architecture](ARCHITECTURE.md) and roadmap for current scope.

## One user workflow, layered capabilities

```text
Git versions small authored records and standard LFS pointers
                              |
              +---------------+----------------+
              |                                |
        direct object/tree              optional typed asset
        references                      definitions and records
              |                                |
              |                       resolve context / variants
              |                       dependencies / recipes
              +---------------+----------------+
                              |
                  exact snapshot + placement claims
                              |
                  Tig byte worktree and storage
                  lookup / ingest / reconcile
                              |
                  admitted providers and producers
```

Basic Git/LFS users need no asset schema, semantic graph or renderer. Advanced
users enable typed records, production fingerprints, variants, evaluation gates
and delivery bindings within the same product. The low-level byte/placement
layer must remain usable independently through a small Rust API.

## What to bring from Art

| Capability | Value inside Tig | Boundary to preserve |
| --- | --- | --- |
| Canonical records and contracts | Bounded meaningful Git manifests with exact locks | Separate raw blob hash from domain-separated semantic identity |
| Asset graph and coverage | Explain dependencies, impact and incomplete native extraction | Evidence/authority/build/storage edges have different purposes |
| Variant selection | Share identities across languages, styles and contexts | Sparse choices; no implicit Cartesian expansion or silent fallback |
| Consumed-value fingerprints | Rebuild only when consumed inputs/rules change | Provenance, result selection, status and production identity stay distinct |
| Profiles and budgets | Requirements for components, renditions and planning | Unknown estimates are not measured bytes or hard capacity admission |
| Review and delivery lifecycle | Scoped rubric/gates, triage and desired/observed delivery | Technical success is not human authority or verified remote publication |

These are generic capabilities. Media meaning, native scene extraction and
rendering remain adapter/producer responsibilities; Tig must not hard-code cels,
cues, language names, drive letters or media approval vocabulary.

## Git pointers and file granularity

Standard LFS pointers remain a supported direct path. Native Tig manifests can
also bind exact immutable objects/trees; typed records add useful authoring fields
and selected results. Asset semantics do not require a second handwritten byte
inventory. Derive bundle membership from the single admitted source of references.

Keep one small record per independently edited logical asset when that helps
parallel Git edits. A generated scene/release root can seal an aggregate snapshot,
but one shared root-pointer file must not silently become the only merge surface
for every cel. Independent leaf edits should merge independently; changing the
same leaf needs explicit reconciliation. A root-only bundle workflow needs its
own tested three-way tree merge before claiming the same experience.

Wire syntax, extension choice and .art compatibility/import remain explicit
format decisions. Do not mass-rename .art files, schema identifiers or hash
domains as a branding change. Preserve exact source identities until migration
vectors and consumer compatibility tests establish the replacement mapping.

## Byte worktree model

```text
immutable base tree             private change overlay
  cel-a -> A1                    cel-b -> B2
  cel-b -> B1                          |
  cue   -> Q1                          |
         +-----------------------------+
                        |
                   effective view
                  A1 + B2 + Q1
                        |
                seal immutable new tree
                        |
              update corresponding Git record/pointer
```

Unchanged objects stay shared; writes are private until sealed. Files removed
from a draft need explicit removal entries so they do not reappear from its base.
Renames change logical bindings, not immutable object identity. Dirty working
views are not exact published snapshots. Same-name output jobs use bounded
staging and expected-selection checks before adoption; no branch per render is
required. A mount adapter is optional later work; explicit resolved inputs and
private output areas provide the first implementation path.

Sealing bytes and updating a Git selection are separate operations with durable
protection between them. Do not silently commit, approve or publish just because
a job finished. Git merges continue to drive current/history/draft claims under
admitted policy, with independently verified placement and retention.

## Adoption sequence

1. Inventory Art's source pins, licenses, pure tests and design-only claims.
2. Preserve canonical/hash-domain and variant/graph vectors before extraction.
3. Define one shared identity/error/coverage boundary without making placement
   depend on semantic assets. Avoid two canonicalizers or two cache engines.
4. Port a bounded pure slice with old/new parity fixtures, then its documentation
   and applicable role/pitfall/trace evidence. Do not claim the Art test suite
   already runs in Tig.
5. Rehearse .art/native record and consumer migration privately. Keep unrelated
   media and existing cache safeguards out of the product repo.

No Art repo deletion, history rewrite, runtime pin switch or consumer conversion
is implied. Its source remains available as provenance during consolidation.
Donor tests and planned features are not additional Tig implementation evidence.
The current evidence register records each extracted slice separately.
