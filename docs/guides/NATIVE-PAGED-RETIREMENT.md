# Select a native block for paged retirement

This experimental metadata adapter selects exact archived native occurrences.
It grants neither claim-release nor final-loss permission and does not execute
payload operations. The separately admitted paged controller supplies those
operations and checks current protection/age/providers independently.

Admit a private tig.native-paged-profile.design-1 containing the existing metadata
root/minimum, source roots/minima, native enrollment (metadata identity, owner,
exact definition categories and optional age policy), finite limits and explicit
allow_stage=true. Staging is disabled by default. Profiles and results remain
private outside project Git; no store is initialized by these commands.

Use tig-retirement index-paged PROFILE REQUEST to stage occurrence metadata.
The tig.native-paged-index-request.design-1 names exact namespace/source/
predecessor/header occurrences. Optional base points to an existing immutable
occurrence index, allowing bounded chunks under the64KiB document limit.
Conflicting header reuse for the same occurrence refuses. The result supplies an
asset-side occurrences pointer and a finished metadata checkpoint.

Use tig-retirement select-paged PROFILE REQUEST with a
tig.native-paged-select-request.design-1 containing that occurrences pointer,
filter (exact asset References and/or admitted category names) and optional
paging limits. Asset and category filters intersect. A requested asset may exist
in only one of several original snapshots; the entire admitted scope must account
for every explicitly requested Reference. Missing scope never silently truncates
the block. The native header/definition/graph archive supplies meaning, not names
or descriptive status fields.

Optional age configuration supplies an enrolled registry root/minimum and explicit
clock/limits. Unknown/recent occurrences remain held in the asset-side row index.
The small candidate points to those rows and an existing generic paged selection
request, or no request when nothing qualifies. A later time does not expand that
frozen request. Staging writes metadata and may append admitted clock observations;
it never changes source claims or payloads.

The CLI finishes the existing metadata provider, opens an independent reader and
recomputes native derivations/request linkage under the same session budget before
returning a bounded preview. Source drift refuses the candidate. Native inspection
is historical metadata, not a source lease or fresh final-deletion approval.
Operator/agent/UI/Action execution and real writer/authority/provider/lifetime
adoption remain required before the complete user workflow is accepted.
