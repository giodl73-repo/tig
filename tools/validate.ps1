$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path $PSScriptRoot -Parent
function Require([bool]$condition, [string]$message) {
    if (-not $condition) { throw $message }
}
$required = @('AGENTS.md','CONTEXT.md','README.md','PRODUCT_PLAN.md','LICENSE',
    'context/waves/PHASES.md','specs/MODEL.md','specs/SAFETY.md',
    '.roles/ROLE.md','.pitfall/PITFALL.md','docs/vtrace/TRACE.md',
    'docs/vtrace/REVIEW.md','docs/vtrace/VERIFICATION.md','docs/INTAKE.md',
    'docs/BACKEND-DECISION.md','docs/reviews/foundation-targets.json')
foreach ($path in $required) {
    Require (Test-Path -LiteralPath (Join-Path $repoRoot $path) -PathType Leaf) "Missing $path"
}
$files = @(Get-ChildItem $repoRoot -Recurse -File -Force | Where-Object {
    $_.FullName -notmatch '[\\/](\.git|\.local|target)[\\/]'
})
$bytes = ($files | Measure-Object -Property Length -Sum).Sum
Require ($bytes -lt 4194304) 'Source/docs exceed 4 MiB ordinary-file budget'
foreach ($file in $files) {
    Require ($file.Length -le 131072) "Ordinary file exceeds 128 KiB: $($file.Name)"
    Require ($file.Extension -notin @('.mp4','.mp3','.wav','.flac','.png','.jpg','.jpeg','.pdf','.zip','.exe','.dll','.pdb','.rlib','.sqlite','.db','.jsonl','.log')) "Runtime/media/binary file in source: $($file.Name)"
}
$links = 0
foreach ($file in $files | Where-Object Extension -eq '.md') {
    $body = Get-Content -LiteralPath $file.FullName -Raw
    Require ($body -notmatch '<REPO>|<repo-slug>|<YYYY-MM-DD-wave-slug>') "Unfilled scaffold: $($file.Name)"
    foreach ($match in [regex]::Matches($body, '\[[^\]]+\]\(([^)]+)\)')) {
        $target = $match.Groups[1].Value
        if ($target -match '^[a-zA-Z][a-zA-Z0-9+.-]*:|^#') { continue }
        $target = ($target -split '#',2)[0]
        Require (Test-Path -LiteralPath (Join-Path $file.DirectoryName $target)) "Missing Markdown target: $target"
        $links++
    }
}
$catalog = Get-Content (Join-Path $repoRoot 'tests/scenarios/catalog.json') -Raw | ConvertFrom-Json -AsHashtable
Require ($catalog.schema -eq 'tig.scenarios.design-1' -and $catalog.cases.Count -eq 25) 'Invalid designed-case inventory'
$ids = [Collections.Generic.HashSet[string]]::new()
$scenarioDoc = Get-Content (Join-Path $repoRoot 'docs/scenarios/FOUNDATION.md') -Raw
foreach ($case in $catalog.cases) {
    Require ($case.id -match '^T(0[1-9]|1[0-9]|2[0-5])$' -and $ids.Add($case.id)) 'Invalid/duplicate scenario ID'
    Require ($case.status -eq 'designed' -and $case.evidence.Count -eq 0) 'Foundation cannot claim runtime evidence'
    Require ($scenarioDoc -match ('(?m)^\| ' + $case.id + ' \|')) "Scenario absent from document: $($case.id)"
    Require (-not [string]::IsNullOrWhiteSpace($case.expectation)) 'Missing scenario expectation'
}
# Design arithmetic only: this does not exercise a runtime reservation protocol.
$capacity = Get-Content (Join-Path $repoRoot 'examples/storage/capacity.design-1.json') -Raw | ConvertFrom-Json -AsHashtable
Require ($capacity.schema -eq 'tig.capacity-example.design-1' -and
    $capacity.scope -eq 'synthetic-arithmetic-not-runtime-policy-or-live-observation' -and
    $capacity.unit -eq 'decimal-GB' -and $capacity.bytes_per_gb -eq 1000000000) 'Invalid capacity example scope/units'
Require ($capacity.enrolled_worktrees -eq $capacity.active_jobs.Count + $capacity.idle_worktrees) 'Example worktree accounting drift'
$placesById = @{}
$domains = [Collections.Generic.HashSet[string]]::new()
foreach ($place in $capacity.places) {
    Require (-not $placesById.ContainsKey($place.id)) 'Duplicate example place'
    Require ($domains.Add($place.capacity_domain)) 'This example requires distinct capacity domains; aliases must share a budget'
    $placesById[$place.id] = $place
    foreach ($key in @('capacity_gb','managed_gb','scratch_gb','materialized_gb','other_gb','free_gb','floor_gb')) {
        Require ($place[$key] -is [long] -or $place[$key] -is [int]) 'Capacity example expects integer GB'
        Require ($place[$key] -ge 0) 'Negative capacity example quantity'
    }
    Require ($place.capacity_gb -eq $place.managed_gb + $place.scratch_gb + $place.materialized_gb + $place.other_gb + $place.free_gb) 'Example physical balance drift'
    $jobs = @($capacity.active_jobs | Where-Object place -eq $place.id)
    $allocated = ($jobs | Measure-Object scratch_allocated_gb -Sum).Sum
    Require ($allocated -le $place.scratch_gb) 'Allocated job scratch exceeds physical scratch'
}
foreach ($job in $capacity.active_jobs) {
    Require ($placesById.ContainsKey($job.place)) 'Unknown job place'
    Require ($job.scratch_allocated_gb -ge 0 -and $job.remaining_scratch_gb -ge 0 -and $job.remaining_output_gb -ge 0) 'Negative job quantity'
}
foreach ($proposal in $capacity.proposals) {
    Require ($placesById.ContainsKey($proposal.place)) 'Unknown proposal place'
    $place = $placesById[$proposal.place]
    $reserved = 0
    foreach ($job in $capacity.active_jobs | Where-Object place -eq $place.id) {
        $reserved += $job.remaining_scratch_gb + $job.remaining_output_gb
    }
    $available = [Math]::Max(0, $place.free_gb - $place.floor_gb - $reserved)
    $peak = $proposal.missing_input_gb + $proposal.output_gb + $proposal.additional_scratch_gb + $proposal.publication_gb
    Require ($available -eq $proposal.expected_available_gb -and $peak -eq $proposal.expected_additional_peak_gb) 'Example admission arithmetic drift'
    Require (($peak -le $available) -eq $proposal.expected_admit) 'Example admission result drift'
}
foreach ($day in $capacity.daily_observations) {
    Require ($placesById.ContainsKey($day.place)) 'Unknown observation place'
    Require ($day.managed_gb + $day.scratch_gb + $day.materialized_gb + $day.other_gb + $day.free_gb -eq $placesById[$day.place].capacity_gb) 'Daily physical balance drift'
}
$first = $capacity.daily_observations[0]
$last = $capacity.daily_observations[-1]
$growth = $capacity.expected_three_day_growth_gb
Require ($first.place -eq $last.place -and $last.day - $first.day -eq 3) 'Growth example scope drift'
foreach ($bucket in @('managed','scratch','materialized','other')) {
    Require ($last["${bucket}_gb"] - $first["${bucket}_gb"] -eq $growth[$bucket]) 'Growth attribution drift'
}
Require ($first.free_gb - $last.free_gb -eq $growth.total -and
    $growth.total -eq $growth.managed + $growth.scratch + $growth.materialized + $growth.other) 'Total growth drift'
$scratch = $capacity.day4_scratch_breakdown_gb
Require ($scratch.active + $scratch.completed + $scratch.unknown -eq $last.scratch_gb) 'Scratch attribution drift'
Write-Output 'PASS synthetic capacity arithmetic; no runtime admission or live measurements exercised.'
$pins = Get-Content (Join-Path $repoRoot 'tools/vendor/pins.json') -Raw | ConvertFrom-Json -AsHashtable
$integration = Get-Content (Join-Path $repoRoot 'tests/scenarios/integrations.json') -Raw | ConvertFrom-Json -AsHashtable
Require ($integration.schema -eq 'tig.integration-scenarios.design-1' -and $integration.suites.Count -eq 8) 'Invalid integration design ledger'
$extraCount = 0
foreach ($suite in $integration.suites) {
    $expectedCount = switch ($suite.prefix) {
        'CL' { 10 } 'GW' { 15 } 'CR' { 10 } 'RW' { 10 }
        'LF' { 10 } 'SP' { 10 } 'BN' { 10 } 'LC' { 10 }
        default { throw 'Unknown design suite' }
    }
    Require ($suite.cases.Count -eq $expectedCount) 'Integration case count mismatch'
    $doc = Get-Content -LiteralPath (Join-Path $repoRoot $suite.document) -Raw
    foreach ($case in $suite.cases) {
        Require ($ids.Add($case.id)) "Duplicate design case: $($case.id)"
        Require ($case.status -eq 'designed' -and $case.evidence.Count -eq 0) 'Design ledger cannot claim runtime evidence'
        Require ($doc -match ('(?m)^\| ' + [regex]::Escape($case.id) + ' \|')) "Missing design row: $($case.id)"
        Require (-not [string]::IsNullOrWhiteSpace($case.expectation)) 'Missing integration expectation'
    }
    foreach ($n in 1..$expectedCount) {
        Require ($ids.Contains(('{0}{1:D2}' -f $suite.prefix,$n))) 'Missing integration case ID'
    }
    $extraCount += $suite.cases.Count
}
$learning = Get-Content (Join-Path $repoRoot 'tests/scenarios/learning-links.json') -Raw | ConvertFrom-Json -AsHashtable
Require ($learning.schema -eq 'tig.scenario-learnings.design-1' -and $learning.links.Count -eq $ids.Count -and $ids.Count -eq 110) 'Every scenario needs one learning link'
$principles = Get-Content (Join-Path $repoRoot 'docs/doctrine/PRINCIPLES.md') -Raw
$invariants = Get-Content (Join-Path $repoRoot 'docs/doctrine/INVARIANTS.md') -Raw
$pitfalls = Get-Content (Join-Path $repoRoot 'docs/doctrine/PITFALLS.md') -Raw
$mitigations = Get-Content (Join-Path $repoRoot 'docs/scenarios/MITIGATIONS.md') -Raw
$trace = Get-Content (Join-Path $repoRoot 'docs/vtrace/TRACE.md') -Raw
$linkedIds = [Collections.Generic.HashSet[string]]::new()
foreach ($link in $learning.links) {
    Require ($ids.Contains($link.id) -and $linkedIds.Add($link.id)) 'Unknown/duplicate learning link'
    Require ($link.mitigation_status -eq 'planned' -and $link.execution_evidence.Count -eq 0) 'Design review must not claim mitigation proof'
    Require ($link.pitfalls.Count -gt 0 -and $link.principles.Count -gt 0 -and $link.invariants.Count -gt 0 -and $link.requirements.Count -gt 0) 'Missing doctrine/requirement link'
    foreach ($id in $link.principles) { Require ($principles -match ('(?m)^## ' + [regex]::Escape($id) + ':')) "Missing principle $id" }
    foreach ($id in $link.invariants) { Require ($invariants -match ('(?m)^\| ' + [regex]::Escape($id) + ' \|')) "Missing invariant $id" }
    foreach ($id in $link.pitfalls) { Require ($pitfalls -match ('(?m)^## ' + [regex]::Escape($id) + ':')) "Missing pitfall $id" }
    foreach ($id in $link.requirements) { Require ($trace -match ('(?m)^\| ' + [regex]::Escape($id) + ' \|')) "Missing requirement $id" }
    Require ($mitigations -match ('(?m)^\| ' + [regex]::Escape($link.mitigation) + ' \|')) 'Missing mitigation'
    $mitigationRow = [regex]::Match($mitigations, '(?m)^\| ' + [regex]::Escape($link.mitigation) + ' \|[^\r\n]+').Value
    Require ($mitigationRow.Contains($link.owner)) 'Mitigation owner disagrees with register'
    foreach ($pitfall in $link.pitfalls) { Require ($mitigationRow.Contains($pitfall)) 'Mitigation does not cover linked pitfall' }
    Require ($link.owner -match '^[a-z]+(?:-[a-z]+)*$') 'Invalid role owner'
    Require (Test-Path -LiteralPath (Join-Path $repoRoot ".roles/parliament/$($link.owner).md")) 'Missing mitigation owner role'
}
foreach ($family in @(@('principles',$principles,'TIG-P-'), @('invariants',$invariants,'TIG-I-'), @('pitfalls',$pitfalls,'TIG-F-'))) {
    foreach ($match in [regex]::Matches($family[1], [regex]::Escape($family[2]) + '\d{2}')) {
        $id = $match.Value
        Require (@($learning.links | Where-Object { $_[$family[0]] -contains $id }).Count -gt 0) "Doctrine has no scenario coverage: $id"
    }
}
$vtraceRoot = Join-Path $repoRoot 'docs/vtrace'
foreach ($name in @('MISSION','CONOPS','REQUIREMENTS','SPECIFICATION_BASELINE','ARCHITECTURE',
    'PACKAGE_BOUNDARIES','INTERFACES','DESIGN','CODE_RIGOR','LANGUAGE_PROFILES',
    'IMPLEMENTATION_PLAN','WORK_PACKAGES','INTEGRATION_PLAN','CHANGE_CONTROL',
    'COMMUNICATIONS_STRATEGY','EVIDENCE','VERIFICATION','VALIDATION','TRACE',
    'RISKS','REVIEW','REVIEW_CHECKLISTS','STAGE_EXECUTION','README')) {
    Require (Test-Path -LiteralPath (Join-Path $vtraceRoot "$name.md")) "Missing VTRACE stage: $name"
}
$requirementsText = Get-Content (Join-Path $vtraceRoot 'REQUIREMENTS.md') -Raw
$specificationText = Get-Content (Join-Path $vtraceRoot 'SPECIFICATION_BASELINE.md') -Raw
$workText = Get-Content (Join-Path $vtraceRoot 'WORK_PACKAGES.md') -Raw
$reqRows = [regex]::Matches($requirementsText, '(?m)^\| (REQ-\d{3}) \|')
$specRows = [regex]::Matches($specificationText, '(?m)^\| (SPEC-\d{3}) \| ([^|]+) \|')
$workRows = [regex]::Matches($workText, '(?m)^\| (WP-\d{3}) \|[^\r\n]+')
Require ($reqRows.Count -eq 10 -and $specRows.Count -eq 15 -and $workRows.Count -eq 17) 'Controlled VTRACE baseline inventory drift'
foreach ($row in $reqRows) {
    $id = $row.Groups[1].Value
    Require ($trace.Contains($id) -and $workText.Contains($id)) "Unallocated requirement: $id"
}
foreach ($row in $specRows) {
    $id = $row.Groups[1].Value
    Require ($trace.Contains($id)) "Untraced specification: $id"
    if ($row.Groups[2].Value.Trim() -eq 'target') {
        Require ($workText.Contains($id)) "Unallocated target specification: $id"
    }
}
$packageIds = [Collections.Generic.HashSet[string]]::new()
foreach ($row in $workRows) {
    Require ($packageIds.Add($row.Groups[1].Value)) 'Duplicate work package'
    $cells = @($row.Value.Trim('|').Split('|') | ForEach-Object { $_.Trim() })
    Require ($cells.Count -eq 7 -and @($cells | Where-Object { [string]::IsNullOrWhiteSpace($_) }).Count -eq 0) 'Incomplete work-package table row'
    foreach ($level in @('L0','L1','L2')) { Require ($cells[5].Contains($level)) 'Missing work-package verification level' }
    if ($cells[0] -eq 'WP-001') {
        Require ($cells[6] -eq 'verified-pure') 'WP-001 scope must remain explicitly pure'
        Require (Test-Path (Join-Path $repoRoot 'specs/IDENTITY-AND-TREES.md')) 'Missing WP-001 proof scope'
        $evidenceText = Get-Content (Join-Path $vtraceRoot 'EVIDENCE.md') -Raw
        Require ($evidenceText.Contains('EVID-007') -and $evidenceText.Contains('EVID-008')) 'Missing WP-001 evidence'
    } elseif ($cells[0] -eq 'WP-002') {
        Require ($cells[6] -eq 'verified-pure') 'WP-002 scope must remain explicitly pure'
        $evidenceText = Get-Content (Join-Path $vtraceRoot 'EVIDENCE.md') -Raw
        Require ($evidenceText.Contains('EVID-009')) 'Missing WP-002 evidence'
        Require (Test-Path (Join-Path $repoRoot 'crates/tig-core/tests/worktree.rs')) 'Missing WP-002 public-API tests'
    } elseif ($cells[0] -eq 'WP-000') {
        Require ($cells[6] -eq 'discovery-recorded') 'WP-000 records a scoped choose/defer decision only'
        $evidenceText = Get-Content (Join-Path $vtraceRoot 'EVIDENCE.md') -Raw
        Require ($evidenceText.Contains('EVID-011')) 'Missing backend discovery scope'
    } elseif ($cells[0] -eq 'WP-003') {
        Require ($cells[6] -eq 'in-progress') 'WP-003 full persistence/restore obligations remain open'
        Require (Test-Path (Join-Path $repoRoot 'specs/PUBLICATION.md')) 'Missing publication proof scope'
        $evidenceText = Get-Content (Join-Path $vtraceRoot 'EVIDENCE.md') -Raw
        Require ($evidenceText.Contains('EVID-010')) 'Missing scoped publication evidence'
    } elseif ($cells[0] -in @('WP-004','WP-006')) {
        Require ($cells[6] -eq 'in-progress') 'Coordination subsets cannot close full dependent packages'
        Require (Test-Path (Join-Path $repoRoot 'specs/SCOPED-COORDINATION.md')) 'Missing coordination scope'
        $evidenceText = Get-Content (Join-Path $vtraceRoot 'EVIDENCE.md') -Raw
        Require ($evidenceText.Contains('EVID-012') -and $evidenceText.Contains('EVID-013')) 'Missing coordination evidence'
        if ($cells[0] -eq 'WP-004') {
            Require ($evidenceText.Contains('EVID-018') -and $evidenceText.Contains('EVID-019') -and $evidenceText.Contains('EVID-020') -and $evidenceText.Contains('EVID-021')) 'Missing replica/retention/handoff/adoption evidence'
            Require (Test-Path (Join-Path $repoRoot 'specs/RETENTION-HANDOFF.md')) 'Missing owned handoff scope'
            Require (Test-Path (Join-Path $repoRoot 'docs/CONSUMER-ADOPTION.md')) 'Missing consumer readiness scope'
            Require (Test-Path (Join-Path $repoRoot 'docs/SOURCE-BUDGET.md')) 'Missing source budget scope'
            Require (Test-Path (Join-Path $repoRoot 'specs/DURABLE-RETENTION.md')) 'Missing durable retention scope'
            Require (Test-Path (Join-Path $repoRoot 'specs/REPLICA-VERIFICATION.md')) 'Missing replica verification scope'
        }
    } elseif ($cells[0] -eq 'WP-005') {
        Require ($cells[6] -eq 'in-progress') 'Download compatibility cannot close full Git lifecycle scope'
        Require (Test-Path (Join-Path $repoRoot 'specs/LFS-CACHE-INTEROP.md')) 'Missing LFS compatibility scope'
        $evidenceText = Get-Content (Join-Path $vtraceRoot 'EVIDENCE.md') -Raw
        Require ($evidenceText.Contains('EVID-022') -and $evidenceText.Contains('EVID-023')) 'Missing LFS component/client evidence'
        Require ($evidenceText.Contains('EVID-024') -and $evidenceText.Contains('EVID-025')) 'Missing exact source/transfer integration evidence'
        Require (Test-Path (Join-Path $repoRoot 'specs/GIT-SOURCE-EXTRACTION.md')) 'Missing scoped Git extraction contract'
    } elseif ($cells[0] -eq 'WP-007') {
        Require ($cells[6] -eq 'in-progress') 'Accounted publication cannot close full physical transfer package'
        Require (Test-Path (Join-Path $repoRoot 'specs/ACCOUNTED-PUBLICATION.md')) 'Missing accounted publication scope'
        $evidenceText = Get-Content (Join-Path $vtraceRoot 'EVIDENCE.md') -Raw
        Require ($evidenceText.Contains('EVID-015') -and $evidenceText.Contains('EVID-016') -and $evidenceText.Contains('EVID-017')) 'Missing publication/sampling/cleanup evidence'
        Require (Test-Path (Join-Path $repoRoot 'specs/PRIVATE-ATTEMPT-CLEANUP.md')) 'Missing private cleanup scope'
        Require (Test-Path (Join-Path $repoRoot 'specs/LOCAL-CAPACITY-PROBE.md')) 'Missing local sampling scope'
    } elseif ($cells[0] -eq 'WP-008') {
        Require ($cells[6] -eq 'in-progress') 'Journal subset cannot close full cache catalog/recovery package'
        Require (Test-Path (Join-Path $repoRoot 'specs/RESERVATION-JOURNAL.md')) 'Missing journal scope'
        $evidenceText = Get-Content (Join-Path $vtraceRoot 'EVIDENCE.md') -Raw
        Require ($evidenceText.Contains('EVID-014')) 'Missing journal evidence'
        Require ($evidenceText.Contains('EVID-028') -and $evidenceText.Contains('EVID-029') -and $evidenceText.Contains('EVID-030')) 'Missing persistent metadata/scale/integration evidence'
        Require (Test-Path (Join-Path $repoRoot 'specs/PERSISTENT-METADATA.md')) 'Missing persistent metadata proof scope'
    } elseif ($cells[0] -eq 'WP-009') {
        Require ($cells[6] -eq 'in-progress') 'Root claims cannot close full Git/job lifecycle adoption'
        Require (Test-Path (Join-Path $repoRoot 'specs/SOURCE-CLAIM-JOURNAL.md')) 'Missing source-claim transition scope'
        $evidenceText = Get-Content (Join-Path $vtraceRoot 'EVIDENCE.md') -Raw
        Require ($evidenceText.Contains('EVID-026') -and $evidenceText.Contains('EVID-027')) 'Missing persistent source/Git integration evidence'
    } else {
        Require ($cells[6] -eq 'proposed') 'Unimplemented package cannot silently claim completion'
    }
}
$dependencies = @{}
foreach ($section in [regex]::Matches($workText, '(?ms)^### (WP-\d{3}):.*?(?=^### WP-|\z)')) {
    $id = $section.Groups[1].Value
    $dependencyLine = [regex]::Match($section.Value, '(?m)^- Prerequisites: ([^.]+)\. Entry:')
    Require ($dependencyLine.Success) "Missing explicit package prerequisites: $id"
    $dependencies[$id] = @([regex]::Matches($dependencyLine.Groups[1].Value, 'WP-\d{3}') | ForEach-Object { $_.Value })
    foreach ($dependency in $dependencies[$id]) { Require ($packageIds.Contains($dependency)) "Unknown package dependency: $dependency" }
}
Require ($dependencies.Count -eq $packageIds.Count) 'Each work package needs its detailed scope/entry/exit record'
$remaining = [Collections.Generic.HashSet[string]]::new($packageIds)
while ($remaining.Count -gt 0) {
    $ready = @($remaining | Where-Object { @($dependencies[$_] | Where-Object { $remaining.Contains($_) }).Count -eq 0 })
    Require ($ready.Count -gt 0) 'Work-package dependency cycle'
    foreach ($id in $ready) { [void]$remaining.Remove($id) }
}
foreach ($pin in $pins.files) {
    $hash = (Get-FileHash (Join-Path $repoRoot $pin.path) -Algorithm SHA256).Hash.ToLowerInvariant()
    Require ($hash -eq $pin.sha256) "Vendored checker/notice drift: $($pin.path)"
}
$review = Get-Content (Join-Path $repoRoot 'docs/reviews/foundation-targets.json') -Raw | ConvertFrom-Json -AsHashtable
Require (-not $review.runtime_behavior_proven -and $review.method -eq 'producer-applied-foundation-review') 'Invalid review scope'
foreach ($target in $review.targets) {
    $item = Get-Item -Force -LiteralPath (Join-Path $repoRoot $target.path)
    Require ($item.Length -eq $target.bytes) "Reviewed byte length drift: $($target.path)"
    Require ((Get-FileHash -LiteralPath $item.FullName -Algorithm SHA256).Hash.ToLowerInvariant() -eq $target.sha256) "Reviewed hash drift: $($target.path)"
}
$python = if ($IsWindows) { Get-Command py -ErrorAction SilentlyContinue } else { Get-Command python3 -ErrorAction SilentlyContinue }
if (-not $python) { $python = Get-Command python -ErrorAction SilentlyContinue }
Require ($null -ne $python) 'Python 3 required for development-only standards checkers'
foreach ($checker in @('roles/check_roles.py','pitfall/check_pitfall.py')) {
    & $python.Source (Join-Path $PSScriptRoot "vendor/$checker") $repoRoot
    Require ($LASTEXITCODE -eq 0) "Pinned checker failed: $checker"
}
& git -C $repoRoot diff --check
Require ($LASTEXITCODE -eq 0) 'Working-tree whitespace check failed'
& git -C $repoRoot diff --cached --check
Require ($LASTEXITCODE -eq 0) 'Staged whitespace check failed'
Write-Output "PASS structure: $($files.Count) files, $bytes bytes, $links local links; 25 foundation + $extraCount integration designed cases. Run Cargo separately for pure mechanism tests."
