# Start or resume Tig development

Open an actual checkout of giodl73-repo/tig as the session workspace. The public
repo assumes no specific drive, consumer repository, cache enrollment or runtime.
Read AGENTS.md, CONTEXT.md, context/RESUME.md and PRODUCT_PLAN.md, then the active
wave/pulse and the applicable specs, roles and engineering controls.

From that checkout, inspect before changing anything:

```powershell
git rev-parse --show-toplevel
git status --short
git branch --show-current
git worktree list
git log -5 --oneline
gh pr list --repo giodl73-repo/tig --state open
```

Check the exact source/CI status of relevant PRs. If gh is unavailable, use the
GitHub PR page. Preserve dirty/other-lane state. On clean main, an explicit fetch
and fast-forward may update source; do not rebase/reset another lane. Source-only
updates should use GIT_LFS_SKIP_SMUDGE=1 and a command-scoped disabled hooks path
when hooks might launch consumer tooling. This does not change global Git config.
Use an owned branch/worktree for implementation. Review dependency PRs before
starting a duplicate or overlapping slice.

Before Cargo or fixture runs, configure CARGO_TARGET_DIR, TEMP and TMP to explicit
operator-chosen development storage outside source. Do not infer free space from
an old report or fall back silently to a full system drive. For bounded local
builds, use CARGO_INCREMENTAL=0 and CARGO_PROFILE_DEV_DEBUG=0 /
CARGO_PROFILE_TEST_DEBUG=0. Keep machine-specific roots outside tracked files.

For documentation-only changes:

```powershell
pwsh -NoProfile -File tools/validate.ps1
git diff --check
```

For catalogue controller work, run the shipped-script synthetic workflows:

```text
node --test crates/tig-git/tests/work-selection.cjs crates/tig-git/tests/impact.cjs crates/tig-git/tests/ux-flows.cjs crates/tig-git/tests/compare-flows.cjs crates/tig-git/tests/object-flows.cjs crates/tig-git/tests/lessons-flows.cjs
```

Run appropriate locked native API tests when bundled scripts change. Rust changes
also require fmt, focused failure/integration tests and strict Clippy under the
applicable language profile. Refresh foundation-target hashes for changed targets;
protected CI remains separate. Do not claim browser acceptance from a DOM double.

Resume never starts a daemon/viewer, scans caches, installs hooks, hydrates assets,
transfers bytes or executes eviction. Preserve shared runtime bindings. Ask before
desktop/browser interaction unless the current session explicitly authorizes it.
Storage executor development uses synthetic, explicitly allocated temporary roots.

At handoff, record source/branch/PR identity, implemented versus tested behavior,
pending checks and the next bounded task in context/RESUME.md when it changes.
Keep large logs and operator state outside Git. A concise new-session prompt is:

> Continue Tig development from this checkout. Read AGENTS.md and
> context/RESUME.md, verify current branches/PRs, then take the next unowned
> PRODUCT_PLAN.md slice. Use synthetic fixtures and explicit external build/temp
> roots. Preserve existing runtimes and caches.
