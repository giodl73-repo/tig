# Fixture-backed catalogue workflows

Input f072d1b78a559bbb6259d854db38fd75e970b6e7; WP-015;
REQ-006/007/010; SPEC-014; IF-010/011/013; PKG-004/006/008.
UI/INTEGRATION profiles; producer identity, privacy/provider, adoption and
concurrency lenses.

Entry: native/API impact fixtures and isolated controller scenarios. Scope:
exercise the shipped page scripts together, in their HTML-declared order, with
a small synthetic DOM/fetch fixture. No dependencies, browser control, server
startup or private project/cache access. This proves controller wiring, not
browser layout, accessibility implementation or real provider custody.

Improve impact progress/identity and missing-rule sample explanations; correct
only navigation or stale-reply behavior exposed by these workflows. No new
resolver, storage operations, profiles or semantic inference. All fixture API
responses remain synthetic contracts, paired with existing real Rust API tests.

Exit/L0: source/link/hash/privacy checks. L1: Node controller and integrated
workflow fixtures; targeted native API tests for bundled-script wiring. L2:
work/source/consumer/back, exact record identities, paged progress, independent
projects, incomplete versus complete zero, missing rule samples, stale replies,
filter changes and explicit retry. Keep runtime/browser qualification separate.

Producer review applied the four named lenses to the page controllers and
synthetic harness. Identity: duplicate titles retain visible stable IDs, exact
snapshot and selected-only navigation. Privacy/provider: fixture fetch rejects
unhandled routes, executes only shipped source scripts, and uses no network or
private enrollment. Adoption: built-in Node only; existing Rust API fixtures
remain authoritative for resolver behavior. Concurrency: reproduced stale work,
list, search and refresh error overwrites before adding current-selection/
generation guards; a stale successful refresh is also fenced. No blocking
finding within this controller scope. Owner: catalogue maintainer. Browser
layout/keyboard, real data and custody acceptance remain open.

Verification: 31 frontend scenarios pass (16 integrated, 15 existing). Integrated
fixtures use both real app bootstrap/inspector scripts and real work/impact
controllers in HTML order, including 25/50/63 visible progress, 100-of-128 rule
samples, complete/partial zero, retained pins, filter races, retries and returns.
The DOM double checks visibility/modal ownership and explicit focus behavior;
it does not implement browser layout, native input or rendering. No graph solver
or storage engine is mocked into production. Synthetic response bodies establish
controller expectations only; real native/API qualification remains separate.
Producer review is not independent review or human acceptance.
Three targeted real native HTTP cases pass against the rebuilt bundled scripts;
portable source/link/hash and diff checks pass. Resolver/storage Rust code is
unchanged. Hosted Windows/Linux qualification remains required before merge.
