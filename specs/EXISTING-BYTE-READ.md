# Verified reads from existing byte stores

The explicit `tig-graph verify` adapter batches complete declared Production
primary/control objects through this same reader. It pins the graph profile,
admits count/byte/report limits before IO, deduplicates objects and releases each
guard before reporting. These independent observations do not establish
simultaneous protection, independent custody or removal authority. See the
[graph guide](../docs/guides/GRAPH-INSPECTION.md).

Entry af567dc723e34d237c600e5321faa4b27d4f0034. Bounded WP-004/005/007/011
consumer-read slice; REQ-001/003/004/006/007/008/010, SPEC-003/004/007/009/011,
IF-004/005/009/010/012, PKG-002/003/004/008; PROVIDER/INTEGRATION profiles.
Scope: generic tig-store existing-layout reader, compatible Layout re-export in
tig-git and explicit opt-in resolver verification. L0 fmt/structure/Clippy/VTRACE;
L1 bounded/corrupt/unsupported vectors; L2 temporary real files, actual child-exit
guard release and asset-ID-to-byte-reader integration. No production binding change.

Consumer flow: AssetResolver -> exact Object -> ReadCache::open_verified -> held
read/seek session. Reuse existing REEL CAS, Git LFS and flat object layouts; no
cache directory creation, alias/copy, upload, census, daemon or removal. Only
explicitly enrolled roots are probed in declared order. A corrupt/denied present
candidate is not hidden by fallback. This is byte verification, not authority,
producer freshness, backup/custody or global source/job coverage.

With PreventMutation, Windows holds read-only sharing that denies write/delete and directory
handles that deny ancestor rename while allowing normal child additions. Unix
uses a shared advisory file lock under an explicit cooperative-immutable policy;
mandatory mutation exclusion is unsupported there, never silently downgraded.
Roots/local filesystem remain trusted: this is not a sandbox against hostile
directory mutation. Destructive providers must honor effective read protection.
Path-based child consumption is supported only under Windows sharing protection
and keeps the guard alive for the callback. Portable consumers use Read/Seek.
CooperativeImmutable is explicit on both platforms: it permits alias lifecycle,
holds a shared file lock and requires immutable/cooperating writers/removers.
That mode does not expose path-based consumption. Strict Windows protection can
block same-object alias operations; callers choose the declared mode instead of
silently weakening guards. A real Git LFS handoff runs alongside a cooperative
verified reader in the integration fixture.

Verification uses bounded heap buffers, exact SHA-256/size, opened-file checks,
finite request/aggregate byte limits and a per-object clock checked between reads.
Blocking local filesystem calls are not interruptible deadlines. The byte budget
covers verification; renderer/decoder work still needs caller job admission.
No file path/read observation grants safe eviction or independent retained custody.
Live reader permits are bounded and released only after file/directory handles
close. Default limits are 8 GiB/object, 32 GiB verified bytes, 1,024 requests,
128 live readers and 30 seconds per verification. Hard ceilings are 64 GiB,
1 TiB, 65,536 requests, 1,024 live readers and 300 seconds. Bounds are per reader
provider instance, not global resource or physical-volume admission.

The explicit CLI extension is tig-resolve --verify PROFILE ID [STREAM_SLOT].
It requires a byte_source profile with ordered places, layout and required
protection, then holds the verified reader through result emission. Ordinary
metadata lookup does not touch bytes. No raw path is printed. Existing Profile
JSON and tig-git::cache::Layout names retain compatibility through re-export.

```rust
let asset = assets.resolve_id("scene/04/cel/b")?;
let mut input = cache.open_verified(asset.stream("image")?)?;
// Decode via Read/Seek, or use input.with_path(callback) under WindowsSharing.
```

REEL e8fb31c6b21d74f111700d72076bb2a9d2d18176 read/demotion code (MIT) informs
sharing semantics; preserve Tig's no-unsafe implementation. Concrete platform
proof and stated limitations precede production adoption. Full provider enrollment,
job protection, final eviction and native REEL render qualification remain OPEN.

## Canonical Windows root correction

Entry 84136b516a3f784c51398d819af7b44826236be1. WP-004/005/011,
REQ-001/003/004/007/008/010, SPEC-003/004/007/009/011,
IF-004/005/009/010/012, PKG-002/003/004/008; PROVIDER/RUST/INTEGRATION.
REEL's synthetic existing-publisher fixture independently reproduced raw drive
root verification succeeding while the equivalent std::fs::canonicalize root
failed with Io. No actual cache or private media was used.

A Windows drive prefix is not a complete directory lookup target, even when
the extended-prefix spelling reports absolute to std::path. Root admission must
require the explicit RootDir after Disk/VerbatimDisk, and parent walking must
defer metadata/handle opens until that root is present. Preserve all existing
guard, corruption/no-fallback, layout, byte/request and platform constraints.
No provider writes, discovery, weakened protection or new path class is allowed.

L0 fmt/locked tests/warnings-denied Clippy/portable and canonical VTRACE checks.
L1 reject bare extended/drive-relative/UNC roots and retain corruption/budget
refusal. L2 actual temporary Windows raw versus canonical roots across all three
layouts, missing candidates and held mutation/rename protection. Six producer
role lenses review identity versus spelling, truthful protection, concurrent
writers, privacy and adoption. These are not independent/human acceptance.
