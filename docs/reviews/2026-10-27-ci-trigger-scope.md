# One full matrix per pull request

Entry00a74ef (parentCLI PR31); supporting WP-009, REQ-004/009, SPEC-007,
IF-008, PKG-009. Allowed scope: workflow event filter, this review and reviewed
target pins. DOCS/GIT process change; no runtime contract or validation closure.

Push to an open branch currently starts one full Windows/Linux matrix and the
pull-request event starts another at the same source. Preserve both platforms,
every step/command, permissions and main-push validation. Restrict push events to
main; keep pull_request's normal open/synchronize/reopen behavior. Branches without
a PR have local checks; opening a PR starts full hosted qualification.

Do not cancel or reinterpret pending PR31 checks; its original four-check gate
still applies. This change affects future trigger selection, not acceptance of
existing work. Parent31 must qualify before this package merges. New-head full
Windows/Linux qualification remains mandatory.

Producer Git/adoption and concurrency/recovery review: scoped pass for event
selection. L0 compares the entire permissions/jobs suffix byte-for-byte to the
entry, confirms both platforms/all commands unchanged and runs repository
validation/diff checks. No Rust code changes, so additional local runtime tests
are not needed. L2 is the new PR's actual hosted matrix; no human/independent or
asset-storage acceptance is claimed. Normal main push remains a second validation
of merged state; this is not cancellation, skip logic or reduced test depth.
