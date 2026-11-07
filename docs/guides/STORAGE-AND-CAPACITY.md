# Projects, storage bundles and capacity that creeps up

Target walkthrough, not a working capacity manager or a report about your disks.
All figures are synthetic **decimal GB** (1 GB = 1,000,000,000 bytes). The generic
model is defined in [the resource contract](../../specs/RESOURCE-MANAGEMENT.md).

## Configure roles once; reuse them across instances

```text
VERSIONED PROJECT POLICY             TRUSTED LOCAL/PROVIDER BINDINGS
  workset rules                       fast-a -> admitted volume/endpoint A
  retained-copy policy                fast-b -> admitted volume/endpoint B
  job resource profiles               retained -> admitted volume/endpoint C
             |                        archive -> admitted volume/endpoint D
             +-------------------+----------------------+
                                 v
                    exact bundles + owned claims
                                 v
                      shared physical placements
```

A project chooses roots, versions, proximity, independent retention and resource
profiles. A machine binds those abstract roles to its available places. Another
user can map them to one SSD and a cloud store; a team can use several servers.
The same primitives work for media, CAD, datasets, software build artifacts or
documents. Basic Git/LFS users do not need the optional typed asset layer.

Tig can show a role/capacity plan without creating caches or copying bytes.
Executable work requires an admitted provider, authority and resource scope.

There is no required number of tiers, local disks or cloud providers. A place can
serve several roles, and a role can admit several places under explicit selection
rules. Multiple directory roots on one disk share its capacity domain. Multiple
projects can use the same place without creating a private cache copy per project.

Start with the questions the project needs answered:

| Configure | Example question |
| --- | --- |
| Working-set rule | Which branch, release, draft or selected dataset needs nearby bytes? |
| Placement constraints | Which eligible places meet latency, region or access requirements? |
| Retention | How many independent retained copies, and for how long? |
| Job profile | What is the peak scratch/output demand, with what uncertainty? |
| Capacity policy | Which shared quota, safety floor and project limit apply? |
| Observation policy | How fresh must measurements be, and what can be sampled cheaply? |

The policy supplies intent; bindings supply real endpoints; observations supply
actual state. Editing a label alone does not prove any physical move occurred.

## Bundles are shared demand, not directories to copy

```text
project alpha                        project beta
  current bundle                       current bundle
  previous bundle                      previous bundle
  draft/worktree claims                draft/worktree claims
            \                            /
             +------------+-------------+
                          v
            union of exact placement obligations
                          |
         one immutable copy per required physical place
                          |
              active use and independent custody
```

An object shared by three branches and two projects need not occupy five copies
on one place. Project quota displays may attribute shared bytes under a declared
charging rule; the physical total still counts actual allocations once.

Different lifecycle rules can retain one object both nearby and in an archive.
When the last active claim ends, only its unnecessary nearby copy becomes a
candidate; retained copies and other jobs stay protected. Previous versions do
not need to remain on every workstation or active tier.

## N worktrees share assets and own their scratch

```text
small Git controls                shared immutable objects
  worktree 1 --- claim ----------> A B C
  worktree 2 --- claim ----------> A B D      A and B are shared
  worktree 3 --- claim ----------> A E
  worktree N --- claim ----------> ...

admitted active jobs              private mutable space
  job 1 ------------------------> bounded scratch + unpublished output
  job 2 ------------------------> bounded scratch + unpublished output
  idle worktree ----------------> no full job-peak reservation
```

Place scratch near the job's intended output where appropriate; do not stage
every project through one default disk. Do not copy a complete asset tree into
each job directory. Explicit materializations and standard LFS local-store copies
are real physical costs and appear in accounting. Private writes cannot mutate
shared immutable objects.

Resource profiles cap output, scratch and any explicit materialization. Active
jobs reserve their remaining peak. A worktree limit does not reserve that entire
maximum while idle. One worker exceeding its budget is handled in its scope;
it must not acquire a global cache lock that pauses unrelated workers.

## A worked capacity plan

These four generic places are synthetic and independent. Each row represents
one capacity domain; aliases of the same volume must share one row/budget.

| Place | Capacity | Managed objects | Scratch allocated | Materialized copies | Other allocated | Free | Safety floor |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| fast-a | 4,000 | 2,600 | 100 | 150 | 150 | 1,000 | 400 |
| fast-b | 2,000 | 1,300 | 60 | 80 | 90 | 470 | 200 |
| retained | 4,000 | 3,000 | 50 | 100 | 150 | 700 | 400 |
| archive | 8,000 | 5,200 | 0 | 0 | 100 | 2,700 | 800 |

Six enrolled worktrees might have four active jobs and two idle sessions:

| Job | Place | Scratch already allocated | Additional scratch reserved | Output still reserved | Remaining reservation |
| --- | --- | ---: | ---: | ---: | ---: |
| alpha-preview | fast-a | 40 | 60 | 20 | 80 |
| alpha-export | fast-a | 20 | 80 | 25 | 105 |
| beta-preview | fast-b | 25 | 35 | 12 | 47 |
| beta-export | fast-b | 10 | 10 | 5 | 15 |

Other observed scratch still counts physically; it may belong to completed,
unpublished or unattributed work. Those categories require ownership checks.

```text
fast-a usable new headroom = 1,000 free - 400 floor - 185 still reserved = 415 GB
fast-b usable new headroom =   470 free - 200 floor -  62 still reserved = 208 GB
```

Already allocated scratch is inside observed free space. Subtracting the original
job maximum again would double-charge it. The runtime must safely reconcile its
allocation/reservation watermarks; these simple sums alone are not a concurrency
implementation.

A proposed alpha batch needs 30 GB missing inputs + 60 GB output + 140 GB extra
scratch + 10 GB publication overhead = **240 GB peak additional demand**. It fits
415 GB. After expected transient cleanup its net retained growth could be 90 GB;
that lower number cannot fund the initial 240 GB peak.

A beta job needs 20 + 15 + 180 + 10 = **225 GB**. It does not fit 208 GB. Tig should
explain the 17 GB shortfall for this request, while alpha jobs can continue.
The operator could choose a lower-peak admitted profile, use another eligible
place, or first reclaim verified eligible copies. Planned reclaim does not count
as free space until it actually happens. Silent overflow to someone else's full
working disk is not a solution.

## Expected size, actual size and uncertainty

Profiles can estimate required bytes from object counts, dimensions, duration,
codec settings or calibrated fixtures. Keep expected/range/observed/unknown
separate. For example, a 1,800-second output at a combined 8.32 Mbit/s is about
1.872 GB before container overhead. That says nothing about temporary raw frames
or producer intermediates; an admitted render may need much more peak workspace.
Do not assume compression/dedup from visual similarity or a small Git diff.

Generic reporting should expose:

```text
logical bundle total             useful membership size
unique bytes required per place  deduplicated placement demand
new missing bytes                acquisition cost
expected output / uncertainty    producer/profile estimate
peak additional allocation       admission basis
observed allocated / free bytes  provider measurement
possible reclaim                 conditional plan, not spendable space
```

## Daily observations explain creep

Here is synthetic fast-b growth with no implied current measurement:

| Day | Managed | Scratch | Materialized | Other | Free |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1 | 1,300 | 60 | 80 | 90 | 470 |
| 2 | 1,312 | 70 | 85 | 93 | 440 |
| 3 | 1,332 | 95 | 95 | 98 | 380 |
| 4 | 1,362 | 155 | 115 | 108 | 260 |

```text
three-day space consumed: 210 GB
  new managed allocation:  62 GB
  scratch growth:          95 GB
  materialization growth:  35 GB
  other/unattributed:      18 GB
```

The report should not blame 210 GB on new final assets. It should identify which
jobs/worktrees/representations explain the 95 GB scratch and 35 GB materialization
growth, which outputs remain unpublished, and which changes are not yet attributed.
For example, day-4 scratch could be 55 GB active + 60 GB completed + 40 GB unknown.
Completed or unknown does not automatically mean disposable.

```text
incremental operation observations
             + configured periodic reconciliation
                              |
           allocated/free + reservation watermarks + coverage
                              |
             growth by project / worktree / job / place
                              |
        warn / explain / replan / admit bounded next actions
```

Show observation time, freshness, provider scope and missing/offline places. A
daily sample can read cheap free-space metadata and reconcile known events without
rehashing every cache. Explicit deeper inventory is a separate bounded operation.
An offline archive remains unknown; no routine recursive scan of cold storage.

Daily observations, detailed ledgers and expanding indexes belong in admitted
runtime/asset storage with chunking, retention and overhead budgets. Git keeps
small authored policy and optional reviewed checkpoint pins. No daily Git commit
or duplicated spreadsheet is required. Catch-up reports can be generated on demand;
a worker/schedule must be explicitly enabled and is not installed on resume.

## Optional mapping to four local volumes

One media project's bindings could be:

```text
episode working role   -> F:
music working role     -> E:
previous-version role  -> D:
archive role           -> M:
```

That is a configuration example. Another project can choose different names,
places, fallback rules and copy counts. Existing legacy consumer episode fallback behavior
is not silently changed by this guide. F/E/D/M letters do not establish independent
failure domains or actual available capacity. Cold M access remains explicitly
scoped; overflow/fallback must fit the destination's own budget and admission.

For a CAD project, fast-a might hold active assemblies, retained hold prior
releases and archive hold independent copies. For an ML dataset project, fast-b
could hold the current training slice while remote places retain full datasets.
The same rules govern shared objects, N jobs, scratch, quotas and observations.

The [worked JSON fixture](../../examples/storage/capacity.design-1.json) keeps
the arithmetic reviewable. It is synthetic data, not a runnable policy schema or
proof that Tig's future runtime admission/measurement engine is implemented.
