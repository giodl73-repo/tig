# Finding bytes, with or without Art

Target walkthrough. The LFS adapter and shared catalog remain planned.

## Start with exact content

```text
Git LFS pointer       Tig bundle snapshot + path        Art asset + context
       |                        |                              |
       |                 Tig tree lookup                Art resolves semantics
       |                        |                              |
       +------------------------+------------------------------+
                                v
                       object digest + size
                                |
                    admitted shared location index
                                |
                    choose eligible available copy
                                |
               revalidate as required + protect active use
                                |
                      readable bytes / materialization
```

The index should answer ordinary lookups without walking every cache. It can
remember immutable bundle nodes and scoped presence observations. Unknown/offline
places are distinct from verified absence. A stale cached miss cannot hide a
newly published copy indefinitely. Integrity checks depend on the provider and
evidence contract; a filename is not proof of bytes.

## Acquire only when needed

```text
local valid hit -------------------------------> protected read
       |
       +-- no eligible copy
                  |
        permitted source + destination + budget?
                  |
          transfer/resume -> verify -> publish -> protected read
                  |
       unavailable / unauthorized / over budget -> explain blocked request
```

Place preference is policy-owned. A remote source can be LFS, another managed
store or an admitted peer. The returned object must match the exact requested
identity. Reads and eviction coordinate so a valid consumer is not interrupted.

## Keep standard LFS pointers

```text
Git tree stores standard pointer
              |
Git LFS client requests exact OID/size
              |
Tig transfer adapter -> shared caches / configured remote provider
              |
Git LFS local store -> ordinary hydrated worktree
```

Tig should not require rewriting pointers into a proprietary format. Supported
plain SHA-256 OIDs identify the same exact blob bytes; extensions/transforms need
explicit support. Upload success means the configured destination contract was
satisfied, not automatically that a cloud service received it.

The LFS local store and hydrated worktree may add physical copies. An adapter
alone does not remove those costs. Account for them explicitly. A thin worktree
can retain pointers while a Tig-aware consumer opens immutable objects directly;
this is an optional consumer path, not a requirement to use Art.

## Keep editable files separate from immutable copies

```text
immutable shared object --------> protected read
           |
           +-- explicit editable materialization -> user's working file
                                                      |
                                             changed bytes -> new object
```

Do not hard-link an editable file to shared immutable content without proven
mutation isolation. A changed working file must not corrupt other branches or
the archive. See [cache/LFS contract](../../specs/CACHE-AND-LFS.md) for bounds,
protocol sources and ten designed acceptance cases.
