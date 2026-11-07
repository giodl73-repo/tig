# Tig maintainer context

Tig is generic experimental Rust infrastructure. Machine-local enrollment,
cache paths, credentials, producer bindings and private acceptance evidence belong
outside this repository. docs/ARCHITECTURE.md maps the implementation;
PRODUCT_PLAN.md tracks remaining work. Historical test totals and target diagrams
are not current implementation claims.

The workspace contains tig-core, tig-store, tig-git and optional tig-assets.
Basic Git/LFS does not depend on typed assets. Captured Git metadata and admitted
schemas drive optional resolution, graphs, contexts, revisions and deliveries.
The catalogue provides works, search, children, coverage and storage comparison.
Project meaning remains external configuration.

Existing-byte readers and explicit local publication/restore/retention primitives
have bounded fixture evidence. Full live-project custody, active-job/dependency
coverage and payload eviction remain open. Catalogue alignment/older-copy actions
are held previews; there is no removal executor. No cache initialization, scan,
hydration, daemon or hook installation on resume.

Read AGENTS.md and the active wave/pulse. docs/vtrace/README.md routes controls
and evidence. Preserve stable IDs and distinguish historical results, current
code and target behavior. Roles are producer checklists, not independent reviewers.

The public repository began with a clean source snapshot. Private consumer notes,
original history and hosting context remain outside this repository. Historical
evidence SHAs in documentation identify original checkpoints; they are not
promises that those commits exist in public history. Port reviewed generic
patches rather than importing private ancestry. See docs/PUBLIC-RELEASE.md.

Source/docs remain below 3.6 MiB, individual files below 128 KiB. No media, builds,
runtime databases, inventories, raw receipts or private fixtures in Git.
Use explicit build/temp roots. No portfolio/renderer checkout is a runtime
dependency. Vendored checkers retain source pins/licenses; see docs/INTAKE.md
and NOTICE.md. Run tools/validate.ps1 and git diff --check before commits.
Rust changes also require fmt, appropriate locked tests, strict Clippy and
integration cases. Hosted Windows/Linux CI is a separate qualification.
