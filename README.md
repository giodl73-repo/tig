# Tig

**Version references in Git. Manage the bytes with Tig.**

Tig is experimental Rust infrastructure for projects with large assets and
multiple working copies. Git keeps small authored records and Git LFS pointers;
Tig resolves exact bytes, records storage obligations, and helps review where
copies belong. Project schemas, names and storage locations are configuration.

    Git commit / index
           |
    small records or LFS pointers
           |
    exact object: hash + length
           |
           +--------------------------+
           |                          |
    existing byte reader      placement / retention
           |                 desired -> plan -> observed
           |                          |
           +------ admitted places ---+
                   local stores
                optional providers

Objects can belong to several branches or bundles without a copy for each.
Replacing one asset changes its reference; unchanged bytes stay shared.
Another branch, retained version or active job may still need old bytes.
Dropping a reference **does not authorize deletion**.

## What works today

| Capability | Implemented experimental scope |
| --- | --- |
| Identity and metadata | Immutable objects/trees, bounded persistent metadata and exact Git snapshots |
| Existing stores | Guarded verified reads from configured flat, CAS and Git LFS layouts |
| Git LFS | Explicit download-only existing-cache transfer adapter; no initialization or upload |
| Local storage | Bounded publication, capacity admission, source journals, retention and recovery primitives |
| Optional typed assets | Schema admission, declared graphs, sparse variants, production contexts and revision/delivery history |
| Local catalogue | Multi-project works, child navigation, search, versions, metadata coverage and storage comparison |
| Agent queries | Discoverable JSON interface for inspect/admission and profile previews, find, compare, shared bytes, direct bindings and explained transitive impact |
| Storage review | Named copy observations, placement preferences, explicit verification and held dry runs |

These components are not a complete autonomous storage service. Full dependency/job
coverage, end-to-end custody, automatic lifecycle reconciliation, remote transfer
providers and safe payload eviction remain open. The catalogue cannot move or
delete assets. Registration does not prove a remote copy exists.
Read the [safety promise](docs/SAFETY-PROMISE.md).

## Build and try it

Use current stable Rust, Git and Git LFS. Rust 1.98.0 is the recorded local
qualification toolchain; no minimum supported Rust version is fixed yet.
Windows and Linux run in CI. Install Git LFS before integration tests.
No crates.io package or prebuilt installer is published by this repository.

    git clone https://github.com/giodl73-repo/tig.git
    cd tig
    cargo build --workspace --all-features --bins --locked
    cargo test -p tig-git --features schema --test catalogue --locked

The catalogue tests create synthetic Git repositories and temporary stores.
They need no personal project, credentials or existing cache. Building/testing
installs no daemon or Git hooks.

For your project, create a machine-local resolver profile and catalogue enrollment
using [the catalogue contract](specs/CATALOGUE.md) and
[resolver profile](specs/NATIVE-GIT-RESOLUTION.md). Keep local paths and credentials
outside the source repository. Then run:

    target/debug/tig-catalogue check /absolute/path/to/enrollment.json
    target/debug/tig-catalogue serve /absolute/path/to/enrollment.json 8799

Compare recorded selected objects across two enrolled projects without rehashing
media or running the server:

    target/debug/tig-catalogue shared /absolute/path/to/enrollment.json project-a project-b /absolute/path/to/new-report.json

The private report groups shared SHA-256/length identities with their asset IDs
and pinned commits. An optional final category argument narrows the query.
It covers enrolled native selections; unregistered files, nested inventories and
other streams remain outside this comparison. Zero matches is scoped evidence,
not proof that two entire projects share no media. Output never overwrites a file.
For direct selected/named-stream reference lookup and the distinction between
byte equality and dependency changes, see [meta operations](docs/META-OPERATIONS.md).

Agents can use one versioned interface for these operations:

    target/debug/tig-query describe
    target/debug/tig-query run /absolute/path/to/request.json

See [agent queries](docs/AGENT-QUERIES.md) for request shapes, coverage, exit codes,
snapshot/interpretation fencing and the Rust library interface.
Use works to discover admitted scopes, then manifest_references for explicit
inventory byte references with document/field provenance. This reads bounded
named JSON metadata through enrolled readers, never referenced media leaves.
manifest_impact traces potential native consumers if matching owner records
change, keeping inventory evidence separate from semantic dependency paths.

On Windows append .exe to the binary and supply an absolute Windows path.
Open http://127.0.0.1:8799/; stop the foreground process to stop the viewer.
Startup loads configured Git metadata. Nested document expansion, copy observation
and byte verification are separate explicit actions. There is no automatic
project discovery, cache scan, hydration or eviction.

The runtime and CLIs are Rust. Python is used by development checkers and an older
explicit restore helper, not by the Rust viewer. New restore integrations should
use [the Rust project interface](specs/PROJECT-RESTORE.md).

## The model

| Concept | Meaning |
| --- | --- |
| Object | Immutable bytes identified by hash and length |
| Tree | Immutable logical paths mapped to objects or child trees |
| Bundle | Named collection with exact immutable snapshots, not necessarily an archive |
| Place | Configured location with capabilities and a failure domain |
| Claim | Admitted obligation to retain content at suitable places |
| Plan | Proposed operations to satisfy those obligations |
| Observation | Scoped availability evidence, not automatically verified custody |

    main: 1,000 objects       branch replaces 5       merge selects new version
                             reuses 995              history retains old demand
                             draft protects bytes    shared objects stay shared

    Git merge -> desired state -> scoped plan -> verified provider work
                                              -> authorized retirement

The automatic lifecycle arrows describe intended integration. Current components
do not make every Git operation trigger transfers or eviction.

## Learn and contribute

- [Architecture](docs/ARCHITECTURE.md): crates, feature gates, data flow and boundaries.
- [Roadmap](PRODUCT_PLAN.md): implemented slices and remaining milestones.
- [Git-to-Tig guide](docs/guides/GIT-TO-TIG.md), [theory](docs/THEORY.md) and
  [capacity](docs/guides/STORAGE-AND-CAPACITY.md): ASCII walkthroughs.
- [Catalogue](specs/CATALOGUE.md), [Git LFS](specs/LFS-CACHE-INTEROP.md) and
  [existing-byte reads](specs/EXISTING-BYTE-READ.md): implemented contracts.
- [Engineering evidence](docs/vtrace/README.md): scope and limits of verification.
- [Public source review](docs/PUBLIC-RELEASE.md): six-role findings and ongoing release checks.
- [Contributing](CONTRIBUTING.md) and [security](SECURITY.md).

Original Tig code and ordinary technical documentation are MIT licensed.
See [LICENSE](LICENSE) and [third-party provenance](NOTICE.md).
