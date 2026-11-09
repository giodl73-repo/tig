//! Explicit staged-index operator interface. No hook or provider initialization.
use super::profiles::Storage;
use super::*;
use tig_git::lifecycle::index;
use tig_store::{
    existing::{Profile as SourceProfile, ReadCache},
    ledger::CapacityLedger,
    local::LocalStore,
    placement::{self, Cursor, Local},
};
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct IndexProfile {
    schema: String,
    enrollment: index::Enrollment,
    git_executable: PathBuf,
    repository_root: PathBuf,
    source_limits: Limits,
    namespace_root: PathBuf,
    namespace_minimum: source_claims::Checkpoint,
    metadata_root: PathBuf,
    metadata_minimum: metadata::Checkpoint,
    #[serde(default)]
    allow_selection: bool,
}
pub(super) fn run(args: &[std::ffi::OsString]) -> Result<Value, &'static str> {
    if args.len() < 3 {
        return Err("usage");
    }
    let action = args[1].to_str().ok_or("usage")?;
    if !matches!(action, "plan" | "status" | "apply" | "resume" | "reconcile")
        || (action == "plan" && args.len() != 3)
        || (matches!(action, "status" | "apply" | "resume") && args.len() != 4)
        || (action == "reconcile" && !(4..=5).contains(&args.len()))
    {
        return Err("usage");
    }
    let profile: IndexProfile = parse(&read(Path::new(&args[2]))?)?;
    if profile.schema != "tig.git-index-profile.design-1" {
        return Err("invalid_document");
    }
    profile.enrollment.id().map_err(code)?;
    for path in [
        &profile.git_executable,
        &profile.repository_root,
        &profile.namespace_root,
        &profile.metadata_root,
    ] {
        absolute(path)?;
    }
    let mut namespace = Namespace::open(&profile.namespace_root, &profile.namespace_minimum)
        .map_err(|_| "journal_unavailable")?;
    let existing = if matches!(action, "status" | "apply" | "resume") {
        Some(index::Plan::from_json(&read(Path::new(&args[3]))?).map_err(code)?)
    } else {
        None
    };
    if action == "status" {
        return serde_json::to_value(
            index::inspect(
                &profile.enrollment,
                &mut namespace,
                existing.as_ref().ok_or("usage")?,
            )
            .map_err(code)?,
        )
        .map_err(|_| "invalid_document");
    }
    if action != "plan" && !profile.allow_selection {
        return Err("selection_disabled");
    }
    let repository = Repository::new(
        profile.git_executable.clone(),
        profile.repository_root.clone(),
        profile.source_limits,
    )
    .map_err(|_| "git_unavailable")?;
    let mut metadata = MetadataStore::open(&profile.metadata_root, &profile.metadata_minimum)
        .map_err(|_| "metadata_unavailable")?;
    if action == "reconcile" {
        return reconcile(&profile, &repository, namespace, &mut metadata, args);
    }
    let mut execution = index::Execution {
        enrollment: &profile.enrollment,
        repository: &repository,
        namespace: &mut namespace,
        metadata: &mut metadata,
        allow_selection: profile.allow_selection,
    };
    if action == "plan" {
        return serde_json::to_value(execution.plan().map_err(code)?)
            .map_err(|_| "invalid_document");
    }
    serde_json::to_value(
        execution
            .apply(existing.as_ref().ok_or("usage")?)
            .map_err(code)?,
    )
    .map_err(|_| "invalid_document")
}
fn reconcile(
    profile: &IndexProfile,
    repository: &Repository,
    namespace: Namespace,
    metadata: &mut MetadataStore,
    args: &[std::ffi::OsString],
) -> Result<Value, &'static str> {
    let storage: Storage = parse(&read(Path::new(&args[3]))?)?;
    if storage.schema != "tig.placement-profile.design-1"
        || storage.providers.len() != storage.config.providers.len()
        || storage.namespaces.len() != storage.config.namespaces.len()
        || storage.config.owner != profile.enrollment.owner
        || storage.config.authority != profile.enrollment.authority
        || !storage.config.namespaces.iter().any(|n| {
            n.namespace == profile.enrollment.namespace && n.policy == profile.enrollment.policy
        })
    {
        return Err("authority");
    }
    let source_digest = Digest::blob(
        &identity::canonical_json(&storage.byte_source).map_err(|_| "invalid_document")?,
    );
    if storage.config.seed != source_digest {
        return Err("authority");
    }
    let source_profile: SourceProfile =
        serde_json::from_value(storage.byte_source).map_err(|_| "invalid_document")?;
    let mut seed = ReadCache::new(source_profile).map_err(|_| "source_unavailable")?;
    let cursor: Option<Cursor> = if args.len() == 5 {
        Some(parse(&read(Path::new(&args[4]))?)?)
    } else {
        None
    };
    let mut namespaces = storage
        .namespaces
        .iter()
        .map(|n| {
            absolute(&n.root)?;
            Namespace::open(&n.root, &n.minimum).map_err(|_| "journal_unavailable")
        })
        .collect::<Result<Vec<_>, _>>()?;
    let source_index = namespaces
        .iter()
        .position(|n| n.identity() == &profile.enrollment.namespace)
        .ok_or("authority")?;
    if fs::canonicalize(&profile.namespace_root).map_err(|_| "journal_unavailable")?
        != fs::canonicalize(&storage.namespaces[source_index].root)
            .map_err(|_| "journal_unavailable")?
    {
        return Err("authority");
    }
    namespaces[source_index] = namespace;
    let stores = storage
        .providers
        .iter()
        .map(|p| {
            absolute(&p.root)?;
            LocalStore::open(&p.root).map_err(|_| "provider_unavailable")
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut ledgers = storage
        .providers
        .iter()
        .map(|p| {
            absolute(&p.ledger_root)?;
            CapacityLedger::open(&p.ledger_root, &p.minimum).map_err(|_| "capacity_unavailable")
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut places = stores
        .iter()
        .zip(ledgers.iter_mut())
        .zip(storage.config.providers.iter())
        .map(|((store, ledger), p)| Local {
            store,
            ledger,
            job: &p.job,
        })
        .collect::<Vec<_>>();
    let nodes = metadata
        .finished_reader()
        .map_err(|_| "metadata_unavailable")?;
    placement::Execution {
        config: &storage.config,
        namespaces: &mut namespaces,
        nodes: &nodes,
        places: &mut places,
        seed: &mut seed,
        seed_identity: &source_digest,
        allow_copy: false,
        allow_eviction: false,
        allow_reacquisition: false,
        allow_cancellation: false,
    }
    .plan(None)
    .map_err(placement_cli::placement_code)?;
    let mut selection = index::Execution {
        enrollment: &profile.enrollment,
        repository,
        namespace: &mut namespaces[source_index],
        metadata,
        allow_selection: true,
    };
    let admitted = selection.plan().map_err(code)?;
    let selected = selection.apply(&admitted).map_err(code)?;
    if selected.observed_index_current != Some(true) || !selected.desired_current {
        return Ok(
            json!({"schema":"tig.git-index-placement-response.design-1","selection":selected,"placement":null,"storage_converged":false,"held":"stale_index"}),
        );
    }
    // Selection is durable now. Later provider failures must preserve that fact.
    let placement = (|| {
        let nodes = metadata
            .finished_reader()
            .map_err(|_| "metadata_unavailable")?;
        let mut execution = placement::Execution {
            config: &storage.config,
            namespaces: &mut namespaces,
            nodes: &nodes,
            places: &mut places,
            seed: &mut seed,
            seed_identity: &source_digest,
            allow_copy: storage.allow_copy,
            allow_eviction: storage.allow_eviction,
            allow_reacquisition: storage.allow_reacquisition,
            allow_cancellation: storage.allow_cancellation,
        };
        let plan = execution
            .plan(cursor.as_ref())
            .map_err(placement_cli::placement_code)?;
        let placed = execution
            .apply(&plan)
            .map_err(placement_cli::placement_code)?;
        Ok::<_, &'static str>((plan, placed))
    })();
    let current = repository
        .index()
        .ok()
        .map(|s| s.checkpoint().listing == admitted.listing);
    match placement {
        Ok((plan, placed)) => Ok(
            json!({"schema":"tig.git-index-placement-response.design-1","selection":selected,"observed_index_current":current,"storage_converged":if current==Some(true){placed.storage_converged}else{Some(false)},"plan":plan,"placement":placed}),
        ),
        Err(held) => Ok(
            json!({"schema":"tig.git-index-placement-response.design-1","selection":selected,"observed_index_current":current,"placement":null,"storage_converged":false,"held":held}),
        ),
    }
}
