# Trees, bundles and transport

Target walkthrough. Tig has no implemented tree format, hash-tree store or
BitTorrent provider yet. Hash labels below are schematic, not real digests.

## Share structure across versions

```text
bundle snapshot V1                  bundle snapshot V2
       |                                   |
       v                                   v
     root-1                              root-2
     /    \                              /    \
  group-X group-Y                     group-X group-Z
    / \     / \                         / \     / \
   A   B   C   D                       A   B   C   E

Same A, B, C objects. Same group-X manifest. New E and affected manifest nodes.
```

A tree maps logical names to immutable objects or subtrees. A bundle reference
selects an exact tree snapshot. Stable bounded partitioning should let a small
edit replace only affected metadata nodes; a single giant inventory file does
not provide that property just because it has a hash.

Bundle overlap does not require duplicate physical storage. One object can
satisfy current, history and draft claims simultaneously. Its location obligations
are the union of those compatible claims.

## Two different uses of hash trees

```text
LOGICAL INVENTORY                     LARGE FILE BYTES
manifest root                         file tree root
   /      \                              /      \
subtree  subtree                      chunk     chunk
   |        |                          hashes   hashes
objects  objects                         |        |
                                      byte blocks
```

The first shares inventory structure and identifies exact membership. The second
verifies portions of one file. Neither implies that a slightly edited compressed
video retains most byte chunks: encoding can change many bytes. Keep primary
objects independently addressable and measure real cross-version reuse.

Tig object digests, manifest identities and torrent identifiers are separate.
An explicit representation map connects them. A trusted root can support proofs
of membership/integrity; it cannot prove that a disconnected archive still holds
the object or that a consumer has stopped using it.

## Bundle membership versus physical movement

```text
current V1: A B D       current V2: A B E       history: D
       |                       |                   |
       +-----------------------+-------------------+
                               v
                         placement policy
                               |
            working: A B E     history: D     archive: required union
```

If D already has admissible retained copies, moving it into history is a metadata
change plus eligible working-copy retirement. Otherwise acquire and verify those
copies first. Membership changes do not move files by themselves.

## Optional BitTorrent distribution

```text
logical group checkpoint 1 -> bundle V1 -> immutable shards S1 S2
logical group checkpoint 2 -> bundle V2 -> immutable shards S1 S3

receiver: retain S1; acquire missing S3; retain S2 where history still needs it
```

Tig can ask an admitted client/provider to seed or acquire immutable shards.
Changing a label does not mutate an existing torrent's contents. Avoid making a
full new archive for every bundle version. Existing-file/piece reuse depends on
the client integration and must be measured; shared hashes alone do not ensure
zero-copy storage. Local disks normally use a verified local-copy provider.

Stopping a seed is not deletion. Peer availability is not retained independent
custody. Objects inside shared packs can keep an entire pack pinned, so dropping
one object can reclaim zero bytes until separate safe compaction.

See [backend/protocol sources](../BACKEND-DECISION.md) and
[placement theory](../THEORY.md). Transport is optional; logical bundle and claim
semantics must work without BitTorrent or Art.
