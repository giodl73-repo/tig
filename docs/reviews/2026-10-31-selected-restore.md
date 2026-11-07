# Restore a selected-only primary

Entry: qualified `266ef3c7b7b453dc2833455f5e23b6dca50d28f6`, identical to the
tested PR49 head's whole tree. WP-005/011/015, REQ-001/003/006/010,
SPEC-004/007/011, IF-005/010/013, PKG-003/004/008. Scope: explicit selector in
the existing CLI, synthetic fixtures and associated contract/trace records.
L0: portable foundation, fmt, locked focused tests and strict workspace Clippy.
L1/L2: real Git and synthetic bytes; selection/stream distinction, planned and
corrupt/missing refusal, preview/apply/retry, exact commit and argument handling.
Private consumer dry runs stay outside this repository and copy no media.

A consumer's first-class primary can have a selected Object and no streams.
Requiring a stream slot prevents its restoration and forces an unnecessary
duplicate stream declaration. `--selected` now resolves that existing selection
directly. Positional stream calls are unchanged. This is explicit selection,
not a magic stream name or a fallback from a missing slot.

The shared reader and copy/publication implementation remain unchanged. Sources
stay verified and held; destinations still require existing ordinary parents;
apply still syncs and independently rereads a private file before no-replace
publication. There is no provider discovery, quota reservation, cache hydration
on startup, journal transaction, deletion or automatic installation.

Producer review: identity checks distinguish selected bytes from a same-named
stream and use the exact Git commit; concurrency/recovery keeps the existing
no-clobber and partial-file limits; provider/privacy retains explicit stores and
private consumer reports; Git/adoption changes no selected records or runtime
bindings. No finding treats a copied or verified file as approval, complete
closure or backup custody. Disposition: pass with the original restoration
limitations, contingent on the stated checks. This is producer review, not
independent or human acceptance. Tooling owns hosted qualification and consumer
acceptance; full custody/placement/adoption remains open.
