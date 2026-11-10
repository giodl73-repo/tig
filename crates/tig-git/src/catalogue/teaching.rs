//! Bounded recorded fixture observations. No current custody or execution authority.
use super::*;
use std::io::Read;
use tig_core::{Digest, identity};
use tig_store::{
    existing::{ByteSource, Profile as ReaderProfile, ReadCache},
    placement,
};
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Role {
    pub id: String,
    pub label: String,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub schema: String,
    pub project: String,
    pub source: ReaderProfile,
    pub trace: Object,
    pub roles: Vec<Role>,
}
impl Profile {
    pub fn validate(&self) -> Result<()> {
        if self.schema != "tig.catalogue-teaching-profile.design-1"
            || !token(&self.project)
            || self.trace.bytes == 0
            || self.trace.bytes > 65536
            || !(2..=4).contains(&self.roles.len())
            || self.roles.iter().any(|r| {
                !token(&r.id)
                    || r.label.is_empty()
                    || r.label.len() > 64
                    || r.label.chars().any(char::is_control)
            })
            || self
                .roles
                .iter()
                .map(|r| &r.id)
                .collect::<BTreeSet<_>>()
                .len()
                != self.roles.len()
        {
            return Err("Invalid teaching enrollment".into());
        }
        Ok(())
    }
}
pub struct Request {
    profile: Profile,
    commits: BTreeSet<String>,
}
impl Catalogue {
    pub fn teaching_request(&self) -> Result<Request> {
        let profile = self
            .teaching
            .clone()
            .ok_or("No teaching fixture enrolled")?;
        let project = self
            .projects
            .get(&profile.project)
            .ok_or("Teaching project is not enrolled")?;
        let mut commits = project
            .refs
            .iter()
            .map(|(_, c)| c.clone())
            .collect::<BTreeSet<_>>();
        commits.insert(project.commit.clone());
        Ok(Request { profile, commits })
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Trace {
    schema: String,
    completed: Vec<Entry>,
    pending: Vec<String>,
    fixture_native_scope: String,
    providers: Vec<ProviderRole>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProviderRole {
    instance: Digest,
    id: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    lesson: String,
    report: Object,
    file: String,
    title: String,
    event: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Observation {
    object: Object,
    verified_copies: Vec<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Receipt {
    schema: String,
    plan: Digest,
    observed_basis: Digest,
    basis_current: bool,
    page_converged: bool,
    storage_converged: Option<bool>,
    stream_bytes: u64,
    effects: Vec<Effect>,
    observations: Vec<Value>,
    holds: Vec<String>,
    next: Option<placement::Cursor>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Effect {
    object: Object,
    provider: Digest,
    kind: String,
    logical_bytes: u64,
    file_allocation_before: u64,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Report {
    schema: String,
    lesson: String,
    source_commit: String,
    source_checkpoint: tig_store::source_claims::Checkpoint,
    plan: placement::Plan,
    receipt: Receipt,
    observations: BTreeMap<String, Observation>,
    declared_fixture_domains: usize,
    physical_domain_coverage: String,
    independent_hardware_backup_proven: bool,
}
fn document<T: serde::de::DeserializeOwned>(
    reader: &mut ReadCache,
    object: &Object,
    left: &mut u64,
) -> Result<T> {
    if object.bytes == 0 || object.bytes > 65536 || object.bytes > *left {
        return Err("Teaching read budget exceeded".into());
    }
    *left -= object.bytes;
    let mut source = reader
        .open_verified(object)
        .map_err(|_| "Teaching metadata unavailable or corrupt")?;
    let mut raw = Vec::new();
    source
        .by_ref()
        .take(65537)
        .read_to_end(&mut raw)
        .map_err(|_| "Teaching metadata read failed")?;
    if Object::from_bytes(&raw) != *object {
        return Err("Teaching metadata integrity failed".into());
    }
    let value = parse_json(&raw).map_err(|_| "Invalid teaching JSON")?;
    if identity::canonical_json(&value).map_err(|_| "Invalid teaching JSON")? != raw {
        return Err("Noncanonical teaching JSON".into());
    }
    serde_json::from_value(value).map_err(|_| "Unsupported teaching metadata".into())
}
/// Metadata IO runs after the host releases its catalogue mutex.
pub fn execute(request: Request) -> Result<Value> {
    let p = request.profile;
    p.validate()?;
    let mut reader = ReadCache::new(p.source.clone()).map_err(|_| "Teaching reader unavailable")?;
    let mut left = 2 * 1024 * 1024;
    let trace: Trace = document(&mut reader, &p.trace, &mut left)?;
    if trace.schema != "tig.teaching-trace.design-2"
        || trace.completed.is_empty()
        || trace.completed.len() > 16
        || trace.pending.len() > 32
        || trace
            .pending
            .iter()
            .any(|s| s.len() > 160 || s.chars().any(char::is_control))
        || trace.fixture_native_scope.len() > 512
    {
        return Err("Invalid teaching trace".into());
    }
    if trace.providers.len() != p.roles.len()
        || trace
            .providers
            .iter()
            .zip(&p.roles)
            .any(|(a, b)| a.id != b.id)
        || trace
            .providers
            .iter()
            .map(|r| &r.instance)
            .collect::<BTreeSet<_>>()
            .len()
            != p.roles.len()
    {
        return Err("Teaching provider-role order differs".into());
    }
    let expected_providers = trace
        .providers
        .iter()
        .map(|r| r.instance.clone())
        .collect::<Vec<_>>();
    let roles = p
        .roles
        .iter()
        .map(|r| r.id.as_str())
        .collect::<BTreeSet<_>>();
    let mut lessons = Vec::new();
    let mut names = BTreeSet::new();
    let mut objects = BTreeMap::<Digest, u64>::new();
    let mut identities = BTreeMap::<String, Object>::new();
    let mut providers = None;
    for entry in trace.completed {
        if !token(&entry.lesson)
            || !names.insert(entry.lesson.clone())
            || entry.title.is_empty()
            || entry.title.len() > 120
            || entry.event.len() > 1024
            || entry
                .title
                .chars()
                .chain(entry.event.chars())
                .any(char::is_control)
            || entry.file.len() > 160
        {
            return Err("Invalid teaching lesson".into());
        }
        let report: Report = document(&mut reader, &entry.report, &mut left)?;
        let id = report.plan.id().map_err(|_| "Invalid teaching plan")?;
        let binding = (
            report.plan.enrollment.clone(),
            report.plan.providers.clone(),
        );
        if report.schema != "tig.teaching-observation.design-1"
            || report.lesson != entry.lesson
            || report.independent_hardware_backup_proven
            || report.declared_fixture_domains != p.roles.len()
            || report.physical_domain_coverage.is_empty()
            || report.physical_domain_coverage.len() > 256
            || report.plan.providers.len() != p.roles.len()
            || report.plan.providers != expected_providers
            || report
                .plan
                .rows
                .iter()
                .map(|r| &r.object.sha256)
                .collect::<BTreeSet<_>>()
                .len()
                != report.plan.rows.len()
            || providers.as_ref().is_some_and(|old| old != &binding)
            || report.receipt.schema != "tig.placement-report.design-1"
            || report.receipt.plan != id
            || report.receipt.observed_basis != report.plan.basis
            || report.receipt.effects.len() > 128
            || report.receipt.observations.len() > 128
            || report.receipt.holds.len() > 64
            || report.receipt.holds.iter().any(|s| {
                !matches!(
                    s.as_str(),
                    "invalid"
                        | "wrong_enrollment"
                        | "stale"
                        | "budget"
                        | "disabled"
                        | "busy"
                        | "unavailable"
                        | "integrity"
                        | "recovery"
                        | "protected"
                        | "retained"
                        | "missing_source"
                        | "reacquisition_required"
                        | "final_retirement"
                )
            })
            || report.receipt.page_converged && !report.receipt.basis_current
            || report.receipt.storage_converged == Some(true)
                && (!report.receipt.page_converged
                    || report.receipt.next.is_some()
                    || !report.receipt.holds.is_empty())
            || report.observations.len() > 32
            || report.observations.is_empty()
            || report
                .observations
                .values()
                .map(|o| &o.object.sha256)
                .collect::<BTreeSet<_>>()
                .len()
                != report.observations.len()
            || GitId::parse(&report.source_commit).is_err()
        {
            return Err("Inconsistent teaching report".into());
        }
        providers = Some(binding);
        let mut rows = Vec::new();
        for (name, observation) in report.observations {
            let object = &observation.object;
            if !token(&name)
                || observation
                    .verified_copies
                    .windows(2)
                    .any(|pair| pair[0] >= pair[1])
                || observation
                    .verified_copies
                    .iter()
                    .any(|r| !roles.contains(r.as_str()))
                || objects
                    .get(&object.sha256)
                    .is_some_and(|bytes| *bytes != object.bytes)
                || identities.get(&name).is_some_and(|old| old != object)
            {
                return Err("Conflicting teaching object or role".into());
            }
            objects.insert(object.sha256.clone(), object.bytes);
            identities.insert(name.clone(), object.clone());
            let planned = report.plan.rows.iter().find(|r| r.object == *object);
            if planned.is_some_and(|r| r.required.len() != p.roles.len()) {
                return Err("Invalid teaching role order".into());
            }
            let desired = planned.map(|r| {
                r.required
                    .iter()
                    .zip(&p.roles)
                    .filter(|(yes, _)| **yes)
                    .map(|(_, role)| role.id.clone())
                    .collect::<Vec<_>>()
            });
            rows.push(json!({"name":name,"object":object,"desired":desired,"observed":observation.verified_copies}));
        }
        if identities.len() > 32
            || report
                .plan
                .rows
                .iter()
                .any(|row| !rows.iter().any(|v| v["object"] == json!(row.object)))
        {
            return Err("Incomplete teaching object scope".into());
        }
        let mut effects = Vec::new();
        for effect in report.receipt.effects {
            let provider = report
                .plan
                .providers
                .iter()
                .position(|p| p == &effect.provider)
                .ok_or("Unknown teaching effect provider")?;
            if !matches!(
                effect.kind.as_str(),
                "published"
                    | "retained"
                    | "handoff"
                    | "retired"
                    | "reacquired"
                    | "cancelled"
                    | "recovered"
            ) || !rows.iter().any(|v| v["object"] == json!(effect.object))
            {
                return Err("Unknown teaching effect".into());
            }
            effects.push(json!({"object":effect.object,"role":p.roles[provider].id,"kind":effect.kind,"logical_bytes":effect.logical_bytes,"file_allocation_before":effect.file_allocation_before}));
        }
        lessons.push(json!({"id":entry.lesson,"title":entry.title,"event":entry.event,"report":entry.report,"plan":id,
            "source_commit":report.source_commit,"source_available":request.commits.contains(&report.source_commit),
            "source_sequence":report.source_checkpoint.sequence(),"objects":rows,"stream_bytes":report.receipt.stream_bytes,
            "basis_current":report.receipt.basis_current,"converged":report.receipt.page_converged,"storage_converged":report.receipt.storage_converged,
            "remaining_page":report.receipt.next.is_some(),"holds":report.receipt.holds,"effects":effects}));
    }
    let response = json!({"schema":"tig.catalogue-teaching.design-1","basis":p.trace,"project":p.project,"roles":p.roles,
        "lessons":lessons,"pending":trace.pending,"native_scope":trace.fixture_native_scope,
        "mode":"recorded-rehearsal","synthetic_domains":true,"independent_hardware_backup_proven":false,"authorizes_storage":false});
    if serde_json::to_vec(&response)
        .map_err(|_| "Teaching response refused")?
        .len()
        > 65536
    {
        return Err("Teaching response budget exceeded".into());
    }
    Ok(response)
}
