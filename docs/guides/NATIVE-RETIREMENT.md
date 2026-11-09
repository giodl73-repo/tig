# Select historical native assets

The optional schema-feature tig-retirement adapter maps native record identities
to exact historical flat bindings. It neither approves final deletion nor opens
payloads. [The contract](../../specs/NATIVE-RETIREMENT-BINDINGS.md) defines the
admitted graph/profile and source boundaries. Categories are configuration; no
cel, cue or golden state is built into the storage engine.

```text
tig-retirement describe
tig-retirement bind /absolute/private-bind-profile.json
tig-retirement select /absolute/private-select-profile.json /absolute/block-request.json
```

An admitted application first captures the exact native source and writes its
metadata archive through AssetSnapshot::archive_metadata. The bind profile uses
schema tig.native-retirement-bind-profile.design-1 and names an existing metadata
root/minimum, that snapshot object, exact native root References, admitted graph
profiles, allow_bind and optional limits. Bind defaults disabled. It compiles
declared Production+Composition storage closure, finishes existing metadata and
independently reads back the candidate. Keep its selection, header digest and
metadata checkpoint outside project Git. Actual source adoption remains a separate
owner-controlled step; building metadata cannot protect or delete any bytes.

Every root asset receives its own logical bindings, even when two assets share
one immutable payload. These are small metadata aliases; physical content is
deduplicated by object identity. Explicit output payloads can belong to storage
closure while remaining forbidden as production inputs. Unsupported or unknown
declared closure refuses construction. This is declared mapping, not complete
native semantics or byte custody.

The select profile uses schema tig.native-retirement-select-profile.design-1:
metadata {root, minimum}, namespace {root, minimum}, source, owner,
predecessor_revision, independently admitted header, categories and optional
limits. It reads the actual historical root under that current admitted namespace
minimum. A source of another owner/class, pending transition, absent predecessor,
wrong header or foreign binding refuses. It does not infer paths from a filename,
an old catalogue row or a content-hash match in another asset's bindings.

The request has assets and categories arrays. Assets contain exact native
References: {id, object}, where object is the raw record's hash and length.
Categories name entries in the private profile's map from labels to exact
definition References. Supply an exact block, categories, or their intersection.
Empty filters and unknown names/ref revisions refuse. Age fields are unsupported
and rejected; Git dates, modification times and descriptive status values are not
retirement ages.

Selection reports original versus remaining bindings and the exact objects/paths.
After a partial owned retirement, retained shared bindings remain inspectable
under the independently admitted original header. Missing members are not silently
reconstructed. A completely retired block produces no new bulk selection. The
header keeps historical interpretation available for later metadata retirement;
its presence does not promise payload recoverability forever.

The response's selection is an existing bulk::Selection. Add it to an independently
prepared bulk Request with exact copy scope and validity window, then use the
[bulk operator](BULK-RETIREMENT.md). That planner still checks all other sources,
jobs, aliases and retained claims. Native category selection grants neither claim
release nor last-copy consent; exact-plan execution admission remains separate.

Profiles/requests/responses are bounded to64 KiB. Inventories cap1024 native roots
and65536 logical bindings, with aggregate graph/provider work budgets.
Archive acquisition shares construction's budget; independent persisted readback
is a second phase under the same limits. Both must succeed before bind returns.
Existing bulk path/object/plan limits still apply. An oversized block/category response
fails rather than becoming partial consent. Larger paged selection, recorded age
evidence, UI/Action integration, real-provider/large-file/lifetime qualification
and human acceptance remain required full-product work. No command initializes a
store, attaches a live cache or installs a service.
