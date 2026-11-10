//! Explicit public synthetic fixtures. Never opens existing fixture/provider roots.
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::{Command as Git, ExitCode},
};
use tig_core::{
    Digest, LogicalPath, Object, identity,
    tree::{Tree, TreeLimits},
};
use tig_git::source::{GitId, Limits, Repository, Scope, native};
use tig_store::{
    Key,
    existing::{ByteSource, Layout, ReadCache},
    ledger::CapacityLedger,
    local::LocalStore,
    local_capacity::LocalCapacityProbe,
    metadata::{Limits as MetadataLimits, MetadataStore},
    orchestration::Place,
    placement::{self, Authority, Mapping, Provider, Role},
    source_claims::{
        self, Command, Kind, Places,
        admission::{Namespace, Policy, Select},
    },
    store_protection::{Binding, Member},
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Clone, Copy)]
enum Mode {
    Ordinary,
    Exchange,
    Worktree,
    BackupOutage,
    StalePlan,
    Rename,
}
struct SampleSeed {
    reader: ReadCache,
    disabled: bool,
    attempts: u64,
}
impl ByteSource for SampleSeed {
    type Reader = <ReadCache as ByteSource>::Reader;
    fn open_verified(&mut self, object: &Object) -> tig_store::existing::Result<Self::Reader> {
        self.attempts += 1;
        if self.disabled {
            return Err(tig_store::existing::Error::Denied);
        }
        self.reader.open_verified(object)
    }
}
const FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../examples/catalogue/lifecycle.design-1.json"
));
fn key(s: &str) -> Key {
    Key::parse(s).expect("fixed fixture key")
}
fn hash(v: &impl serde::Serialize) -> Result<Digest> {
    Ok(Digest::blob(&identity::canonical_json(
        &serde_json::to_value(v)?,
    )?))
}
fn save(path: &Path, value: &impl serde::Serialize) -> Result<Object> {
    use std::io::Write;
    let raw = identity::canonical_json(&serde_json::to_value(value)?)?;
    if raw.len() > 65536 {
        return Err("fixture document budget".into());
    }
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)?;
    file.write_all(&raw)?;
    file.sync_all()?;
    Ok(Object::from_bytes(&raw))
}
fn git(exe: &Path, repo: &Path, args: &[&str]) -> Result<String> {
    git_transport(exe, repo, args, false)
}
// Only the fixed exchange fixture calls this with file transport enabled.
fn git_transport(exe: &Path, repo: &Path, args: &[&str], file: bool) -> Result<String> {
    let null = if cfg!(windows) { "NUL" } else { "/dev/null" };
    let mut command = Git::new(exe);
    for (name, _) in std::env::vars_os() {
        if name
            .to_string_lossy()
            .to_ascii_uppercase()
            .starts_with("GIT_")
        {
            command.env_remove(name);
        }
    }
    let result = command
        .args([
            "--no-replace-objects",
            "-c",
            "core.hooksPath=NUL",
            "-c",
            "commit.gpgSign=false",
            "-c",
            "tag.gpgSign=false",
            "-c",
            "core.autocrlf=false",
            "-c",
            "gc.auto=0",
            "-c",
            "maintenance.auto=false",
        ])
        .args([
            "-c",
            &format!("core.attributesFile={null}"),
            "-c",
            &format!("core.excludesFile={null}"),
        ])
        .args(args)
        .current_dir(repo)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", null)
        .env("GIT_ATTR_NOSYSTEM", "1")
        .env("GIT_NO_LAZY_FETCH", "1")
        .env("GIT_ALLOW_PROTOCOL", if file { "file" } else { "" })
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("LC_ALL", "C")
        .env("LANG", "C")
        .env("GIT_LFS_SKIP_SMUDGE", "1")
        .stdin(std::process::Stdio::null())
        .output()?;
    if !result.status.success() {
        return Err(format!(
            "fixture git failed: {}",
            String::from_utf8_lossy(&result.stderr)
        )
        .into());
    }
    Ok(String::from_utf8(result.stdout)?.trim().to_owned())
}
fn git_executable() -> Result<PathBuf> {
    for dir in std::env::split_paths(&std::env::var_os("PATH").ok_or("PATH unavailable")?) {
        let path = dir.join(if cfg!(windows) { "git.exe" } else { "git" });
        if path.is_file() {
            return Ok(fs::canonicalize(path)?);
        }
    }
    Err("Git executable unavailable".into())
}
fn policy() -> Policy {
    let normal = Places {
        current: vec![key("working"), key("backup")],
        pending: vec![key("working"), key("backup")],
        predecessor: vec![key("history"), key("backup")],
    };
    let classes = [Kind::Branch, Kind::Draft, Kind::Job, Kind::Retention]
        .into_iter()
        .map(|kind| (kind, normal.clone()))
        .collect::<BTreeMap<_, _>>();
    Policy {
        prefer_active: true,
        classes,
    }
}
fn limits() -> tig_store::protection::Limits {
    tig_store::protection::Limits {
        millis: 60000,
        roots: 64,
        bindings: 1024,
        associations: 4096,
        reads: 100000,
        read_bytes: 64 * 1024 * 1024,
    }
}
struct Demo {
    root: PathBuf,
    git: PathBuf,
    repository: Repository,
    metadata: MetadataStore,
    namespaces: Vec<Namespace>,
    stores: Vec<LocalStore>,
    ledgers: Vec<CapacityLedger>,
    config: placement::Config,
    seed: SampleSeed,
    seed_identity: Digest,
    fixture: Value,
    objects: BTreeMap<String, Object>,
    definition: Value,
    reports: Vec<Value>,
}
impl Demo {
    fn new(requested: &Path) -> Result<Self> {
        if !requested.is_absolute() || requested.exists() {
            return Err("sample requires a fresh absolute root".into());
        }
        let parent = fs::canonicalize(requested.parent().ok_or("sample parent missing")?)?;
        if parent.ancestors().any(|p| p.join(".git").exists()) {
            return Err("sample output must be outside a Git checkout".into());
        }
        let root = parent.join(requested.file_name().ok_or("sample root name missing")?);
        fs::create_dir(&root)?;
        let fixture: Value = serde_json::from_str(FIXTURE)?;
        let git_exe = git_executable()?;
        let repo = root.join("project");
        fs::create_dir(&repo)?;
        let template = root.join("git-template");
        fs::create_dir(&template)?;
        let template_arg = format!(
            "--template={}",
            template.to_str().ok_or("sample path must be UTF-8")?
        );
        git(&git_exe, &repo, &["init", &template_arg, "-b", "main"])?;
        git(
            &git_exe,
            &repo,
            &["config", "user.name", "Tig public sample"],
        )?;
        git(
            &git_exe,
            &repo,
            &["config", "user.email", "sample@example.invalid"],
        )?;
        for name in ["records", "definitions"] {
            fs::create_dir(repo.join(name))?;
        }
        fs::create_dir(root.join("reports"))?;
        fs::create_dir(root.join("lesson-evidence"))?;
        fs::create_dir(root.join("lesson-evidence/objects"))?;
        fs::create_dir(root.join("seed"))?;
        fs::create_dir(root.join("seed/objects"))?;
        let mut objects = BTreeMap::new();
        for name in ["shared", "original", "fixed"] {
            let v = &fixture["objects"][name];
            let raw = v["text"].as_str().ok_or("seed text missing")?.as_bytes();
            let object = Object::from_bytes(raw);
            if object.bytes != v["bytes"].as_u64().ok_or("seed length missing")?
                || object.sha256.as_str() != v["sha256"].as_str().ok_or("seed digest missing")?
            {
                return Err("independent seed pin mismatch".into());
            }
            fs::write(
                Layout::Flat.object_path(&root.join("seed"), &object.sha256),
                raw,
            )?;
            objects.insert(name.into(), object);
        }
        let authored = save(
            &repo.join("definitions/authored.json"),
            &json!({"type":"object","required":["title","version"],"properties":{"title":{"type":"string"},"version":{"type":"string"}},"additionalProperties":false}),
        )?;
        let definition = save(
            &repo.join("definitions/item.json"),
            &json!({"schema":"tig.definition.design-1","id":"type/study","authored_schema":authored,"authoring":{"properties":["title","version"],"required":["title","version"],"defaults":{}},"stream_rules":{},"semantic_rules":{}}),
        )?;
        let mut definition = serde_json::to_value(definition)?;
        definition["id"] = json!("type/study");
        let mut namespace = Namespace::create(
            &root.join("namespace"),
            key("sample"),
            key("sample-epoch"),
            policy(),
        )?;
        let metadata = MetadataStore::create(
            &root.join("metadata"),
            MetadataLimits {
                nodes: 10000,
                bytes: 16 * 1024 * 1024,
                database_bytes: 32 * 1024 * 1024,
                min_free_bytes: 65536,
            },
        )?;
        for (name, class) in [
            ("main", Kind::Branch),
            ("draft", Kind::Draft),
            ("job", Kind::Job),
        ] {
            namespace.apply(
                key(&format!("enroll-{name}")),
                Command::Enroll {
                    source: key(name),
                    owner: key("sample"),
                    class,
                },
                &metadata,
                &limits(),
            )?;
        }
        let mut stores = Vec::new();
        let mut ledgers = Vec::new();
        let mut providers = Vec::new();
        for (name, role) in [
            ("working", Role::Cache),
            ("history", Role::Cache),
            ("backup", Role::Retained),
        ] {
            let store = if role == Role::Cache {
                LocalStore::create_managed(
                    &root.join(name),
                    key(name),
                    Binding::new(
                        1,
                        Digest::blob(b"public sample scoped clients"),
                        vec![Member {
                            namespace: namespace.identity().clone(),
                            places: vec![key(name)],
                        }],
                    )?,
                )?
            } else {
                LocalStore::create(&root.join(name), key(name))?
            };
            let mut ledger = CapacityLedger::create(
                &root.join(format!("ledger-{name}")),
                key(name),
                key("sample-epoch"),
                0,
            )?;
            LocalCapacityProbe::bind(&store, &mut ledger)?.observe(
                &store,
                &mut ledger,
                key("probe"),
                key("sample"),
            )?;
            let job = ledger
                .reserve(key("sample-controller"), key("sample"), 16 * 1024 * 1024)?
                .reference()
                .clone();
            providers.push(Provider {
                place: Place::local(&store, 1)?,
                role,
                // Synthetic admitted domains, as in the provider fixture suite.
                // Reports explicitly deny physical independence qualification.
                failure_domain: key(&format!("fixture-domain-{name}")),
                capacity: ledger.identity().clone(),
                job,
                assignments: vec![Mapping {
                    namespace: namespace.identity().clone(),
                    place: key(name),
                }],
            });
            stores.push(store);
            ledgers.push(ledger);
        }
        let byte_source = json!({"places":[{"root":root.join("seed"),"layout":"flat"}],"required":"cooperative-immutable"});
        let seed_identity = hash(&byte_source)?;
        let config = placement::Config {
            schema: "tig.placement-enrollment.design-1".into(),
            owner: key("sample"),
            authority: Digest::blob(b"explicit public fixture authority"),
            seed: seed_identity.clone(),
            providers,
            namespaces: vec![Authority {
                namespace: namespace.identity().clone(),
                policy: hash(namespace.policy())?,
            }],
            minimum_domains: 1,
            budget: placement::Budget {
                objects: 32,
                max_object_bytes: 1024,
                max_stream_bytes: 1024 * 1024,
                max_peak_bytes: 16 * 1024 * 1024,
                limits: limits(),
            },
        };
        let repository = Repository::new(git_exe.clone(), repo, Limits::default())?;
        Ok(Self {
            root,
            git: git_exe,
            repository,
            metadata,
            namespaces: vec![namespace],
            stores,
            ledgers,
            config,
            seed: SampleSeed {
                reader: ReadCache::new(serde_json::from_value(byte_source)?)?,
                disabled: false,
                attempts: 0,
            },
            seed_identity,
            fixture,
            objects,
            definition,
            reports: Vec::new(),
        })
    }
    fn write_version(&self, version: &str) -> Result<()> {
        for (name, title, object) in [
            (
                "series",
                "Series study",
                if version == "v1" { "original" } else { "fixed" },
            ),
            ("reference", "Reference study", "shared"),
        ] {
            let mut streams = Vec::new();
            if name == "series" {
                streams.push(json!({"slot":"reference-grid","role":"input","group":"primary","object":self.objects["shared"]}));
                if version == "v2" {
                    streams.push(json!({"slot":"history/v1","role":"result","group":"history","object":self.objects["original"]}));
                }
            }
            let record = json!({"schema":"tig.instance.design-1","id":format!("work/{name}"),"definition":self.definition,"authored":{"title":title,"version":if name=="reference" { "v1" } else { version }},"selected":self.objects[object],"streams":streams,"status":{"lifecycle":"released"},"metadata":{}});
            fs::write(
                self.root.join(format!("project/records/{name}.tig")),
                identity::canonical_json(&record)?,
            )?;
        }
        Ok(())
    }
    fn commit(&self, message: &str) -> Result<String> {
        let repo = self.root.join("project");
        git(&self.git, &repo, &["add", "records", "definitions"])?;
        git(&self.git, &repo, &["commit", "-m", message])?;
        git(&self.git, &repo, &["rev-parse", "HEAD"])
    }
    fn select(&mut self, source: &str, commit: &str, operation: &str) -> Result<()> {
        let view = self.repository.commit(GitId::parse(commit)?)?;
        let records = Scope::new([LogicalPath::parse("records")?])?;
        let definitions = Scope::new([LogicalPath::parse("definitions")?])?;
        let snapshot =
            self.repository
                .native(&view, records, definitions, native::Limits::default())?;
        self.select_snapshot(source, &snapshot, operation)
    }
    fn select_snapshot(
        &mut self,
        source: &str,
        snapshot: &native::AssetSnapshot,
        operation: &str,
    ) -> Result<()> {
        snapshot.validate_authored_schemas(tig_assets::schema::Dialect::Draft202012)?;
        let mut entries = BTreeMap::new();
        for record in snapshot.selected_records() {
            entries.insert(
                LogicalPath::parse(&format!("{}/selected", record.reference().id))?,
                record.selected()?.clone(),
            );
            for stream in record.stream_bindings() {
                // The admitted public fixture distinguishes primary inputs from
                // history; this is not a general native closure implementation.
                if stream.group == "history" && stream.role == "result" {
                    continue;
                }
                if stream.group != "primary" || stream.role != "input" {
                    return Err("unsupported fixture stream meaning".into());
                }
                entries.insert(
                    LogicalPath::parse(&format!("{}/{}", record.reference().id, stream.slot))?,
                    stream.object.clone(),
                );
            }
        }
        let root = Tree::build(&entries, &mut self.metadata, TreeLimits::default())?
            .0
            .root()
            .clone();
        self.metadata.finish()?;
        let revision = self.namespaces[0]
            .source(&key(source))?
            .ok_or("source missing")?
            .revision;
        self.namespaces[0].select(
            &Select {
                prepare: key(&format!("{operation}-prepare")),
                adopt: key(&format!("{operation}-adopt")),
                source: key(source),
                owner: key("sample"),
                expected_revision: revision,
                selection: source_claims::Selection {
                    root,
                    context: snapshot.checkpoint().clone(),
                },
            },
            &self.metadata.finished_reader()?,
            &self.config.budget.limits,
        )?;
        Ok(())
    }
    fn withdraw(&mut self, source: &str, operation: &str) -> Result<()> {
        let revision = self.namespaces[0]
            .source(&key(source))?
            .ok_or("source missing")?
            .revision;
        self.namespaces[0].apply(
            key(operation),
            Command::Withdraw {
                source: key(source),
                owner: key("sample"),
                expected_revision: revision,
            },
            &self.metadata.finished_reader()?,
            &self.config.budget.limits,
        )?;
        Ok(())
    }
    fn with_placement<T>(
        &mut self,
        action: impl FnOnce(
            &mut placement::Execution<'_, '_, SampleSeed, tig_store::metadata::MetadataReader>,
        ) -> placement::Result<T>,
    ) -> Result<T> {
        let mut places = self
            .stores
            .iter()
            .zip(self.ledgers.iter_mut())
            .zip(&self.config.providers)
            .map(|((store, ledger), pin)| placement::Local {
                store,
                ledger,
                job: &pin.job,
            })
            .collect::<Vec<_>>();
        let mut reader = self.metadata.finished_reader()?;
        let mut executor = placement::Execution {
            config: &self.config,
            namespaces: &mut self.namespaces,
            nodes: &mut reader,
            places: &mut places,
            seed: &mut self.seed,
            seed_identity: &self.seed_identity,
            allow_copy: true,
            allow_eviction: true,
            allow_reacquisition: true,
            allow_cancellation: true,
        };
        Ok(action(&mut executor)?)
    }
    fn checkpoint(&mut self, index: usize, commit: &str) -> Result<()> {
        self.checkpoint_coverage(index, commit, false, false)
            .map(|_| ())
    }
    fn checkpoint_coverage(
        &mut self,
        index: usize,
        commit: &str,
        offline: bool,
        held: bool,
    ) -> Result<Digest> {
        let (plan, receipt) = self.with_placement(|executor| {
            let plan = executor.plan(None)?;
            let receipt = executor.apply(&plan)?;
            Ok((plan, receipt))
        })?;
        if (held
            && (receipt.page_converged
                || receipt.storage_converged != Some(false)
                || receipt.holds.is_empty()))
            || (!held
                && (!receipt.page_converged || receipt.next.is_some() || !receipt.holds.is_empty()))
        {
            return Err(format!("stage {index}: unresolved placement: {:?}", receipt.holds).into());
        }
        self.record_checkpoint(index, commit, &plan, &receipt, offline, None)
    }
    fn record_checkpoint(
        &mut self,
        index: usize,
        commit: &str,
        plan: &placement::Plan,
        receipt: &placement::Report,
        offline: bool,
        rejection: Option<Value>,
    ) -> Result<Digest> {
        let stage = &self.fixture["stages"][index];
        let id = stage["id"].as_str().ok_or("stage missing")?.to_owned();
        let mut observations = BTreeMap::new();
        for (name, object) in &self.objects {
            let mut actual = BTreeSet::new();
            let mut unknown = BTreeSet::new();
            for (role, store) in ["working", "history", "backup"]
                .into_iter()
                .zip(&self.stores)
            {
                if offline && role == "backup" {
                    if self.root.join("backup").exists() {
                        return Err("offline backup unexpectedly accessible".into());
                    }
                    unknown.insert(role.to_owned());
                    continue;
                }
                Place::local(store, 1)?;
                let path = Layout::Flat.object_path(&self.root.join(role), &object.sha256);
                match fs::symlink_metadata(&path) {
                    Ok(_) => {
                        store.read_object(object)?;
                        actual.insert(role.to_owned());
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
                    Err(e) => return Err(e.into()),
                }
            }
            let expected = stage["expected_copies"][name]
                .as_array()
                .ok_or("expected copies missing")?
                .iter()
                .map(|v| v.as_str().ok_or("expected role missing").map(str::to_owned))
                .collect::<std::result::Result<BTreeSet<_>, _>>()?;
            if actual != expected {
                return Err(
                    format!("{id}/{name}: expected {expected:?}, observed {actual:?}").into(),
                );
            }
            let expected_unknown = stage
                .get("unknown_copies")
                .and_then(|v| v.get(name))
                .and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .map(|s| s.as_str().ok_or("unknown role missing").map(str::to_owned))
                        .collect::<std::result::Result<BTreeSet<_>, _>>()
                })
                .transpose()?
                .unwrap_or_default();
            if unknown != expected_unknown {
                return Err("unexpected unknown coverage".into());
            }
            let mut observation = json!({"object":object,"verified_copies":actual});
            if offline || rejection.is_some() {
                observation["unknown_copies"] = json!(unknown);
            }
            observations.insert(name, observation);
        }
        let plan_id = plan.id()?;
        let mut report = json!({"schema":if rejection.is_some(){"tig.teaching-observation.design-3"}else if offline {"tig.teaching-observation.design-2"}else{"tig.teaching-observation.design-1"},"lesson":id,"source_commit":commit,"source_checkpoint":self.namespaces[0].checkpoint(),"plan":plan,"receipt":receipt,"observations":observations,"declared_fixture_domains":3,"physical_domain_coverage":"not-established; portable fixture directories","independent_hardware_backup_proven":false});
        if let Some(rejected) = rejection {
            report["rejection"] = rejected;
        }
        let object = save(&self.root.join(format!("reports/{id}.json")), &report)?;
        fs::copy(
            self.root.join(format!("reports/{id}.json")),
            Layout::Flat.object_path(&self.root.join("lesson-evidence"), &object.sha256),
        )?;
        self.reports
            .push(json!({"lesson":id,"report":object,"file":format!("reports/{id}.json"),"title":id.replace('-'," "),"event":stage["event"]}));
        git(
            &self.git,
            &self.root.join("project"),
            &["tag", &format!("lesson-{id}"), commit],
        )?;
        Ok(plan_id)
    }
    fn catalogue(&self, current: &str, trace: &Object) -> Result<()> {
        let profile = self.root.join("resolver.json");
        save(
            &profile,
            &json!({"schema":"tig.asset-resolver-profile.experimental-1","git_executable":self.git,"repository":self.root.join("project"),"commit":current,"records":["records"],"definitions":["definitions"],"byte_source":{"places":[{"root":self.root.join("working"),"layout":"flat"},{"root":self.root.join("history"),"layout":"flat"},{"root":self.root.join("backup"),"layout":"flat"}],"required":"cooperative-immutable"}}),
        )?;
        let enrollment = self.root.join("catalogue.json");
        save(
            &enrollment,
            &json!({"schema":"tig.catalogue-enrollment.experimental-1","teaching":{"schema":"tig.catalogue-teaching-profile.design-1","project":"sample","trace":trace,"roles":[{"id":"working","label":"Working / F"},{"id":"history","label":"History / D"},{"id":"backup","label":"Backup / M"}],"source":{"places":[{"root":self.root.join("lesson-evidence"),"layout":"flat"}],"required":"cooperative-immutable","limits":{"object_bytes":65536,"total_bytes":2097152,"requests":32,"verify_millis":30000,"live_readers":1}}},"default_project":"sample","projects":[{"id":"sample","label":"Tig public teaching sample","profile":profile,"display":[{"category":"Studies","title_pointers":["/authored/title"]}],"fields":[{"label":"Version","pointer":"/authored/version","section":"source"}],"collections":[{"id":"studies","label":"Studies","icon":"folder","roots":[{"pattern":"^work/(.+)$","key":"${1}"}],"stages":[{"label":"Released","pointer":"/status/lifecycle","pattern":"^released$"}],"count_state":"Released","version_pointer":"/authored/version","version_role":"result","version_slot_prefix":"history/"}],"storage":{"schema":"tig.catalogue-storage.experimental-1","inventory":null,"encoding":"json","max_decoded_bytes":1048576,"places":[{"label":"Working (F)","place_index":0,"report_tier":null,"live_probe":true},{"label":"History (D)","place_index":1,"report_tier":null,"live_probe":true},{"label":"Backup (M)","place_index":2,"report_tier":null,"live_probe":true}],"default_family":"Studies","default_cache":"Working (F)"}}]}),
        )?;
        let mut catalogue =
            tig_git::catalogue::Catalogue::load(tig_git::catalogue::Config::load(&enrollment)?)?;
        tig_git::catalogue::teaching::execute(catalogue.teaching_request()?)?;
        catalogue.observe_registered("sample", current, "")?;
        let inventory =
            catalogue.object_inventory(&tig_git::catalogue::storage::objects::Query {
                project: "sample",
                kind: "",
                usage: "",
                anomaly: "",
                primary: "",
                observed: "",
                q: "",
                sort: "bytes",
                offset: 0,
                sha: "",
            })?;
        save(&self.root.join("catalogue-observations.json"), &inventory)?;
        Ok(())
    }
    fn run(mut self, mode: Mode) -> Result<Value> {
        self.write_version("v1")?;
        let first = self.commit("Sample: release v1")?;
        self.checkpoint(0, &first)?;
        self.select("main", &first, "release-v1")?;
        self.checkpoint(1, &first)?;
        self.select("job", &first, "start-old-job")?;
        git(
            &self.git,
            &self.root.join("project"),
            &["switch", "-c", "fix-series"],
        )?;
        self.write_version("v2")?;
        let second = self.commit("Sample: fix the measurement series")?;
        self.select("draft", &second, "stage-v2")?;
        self.checkpoint(2, &second)?;
        git(&self.git, &self.root.join("project"), &["switch", "main"])?;
        git(
            &self.git,
            &self.root.join("project"),
            &[
                "merge",
                "--no-ff",
                "fix-series",
                "-m",
                "Sample: release fixed version",
            ],
        )?;
        let merged = git(
            &self.git,
            &self.root.join("project"),
            &["rev-parse", "HEAD"],
        )?;
        self.select("main", &merged, "merge-v2")?;
        // End the merged branch before releasing its draft obligation. Otherwise
        // the later revert would still have another live branch using v2.
        git(
            &self.git,
            &self.root.join("project"),
            &["branch", "-d", "fix-series"],
        )?;
        self.withdraw("draft", "settle-draft")?;
        let mut stale_evidence = None;
        let backup_evidence = if matches!(mode, Mode::BackupOutage) {
            Some(self.backup_outage(&merged)?)
        } else {
            self.checkpoint(3, &merged)?;
            self.withdraw("job", "finish-job")?;
            if matches!(mode, Mode::StalePlan) {
                stale_evidence = Some(self.stale_plan(&merged, &first)?);
            }
            None
        };
        self.checkpoint(4, &merged)?;
        git(
            &self.git,
            &self.root.join("project"),
            &["revert", "-m", "1", "--no-edit", &merged],
        )?;
        let reverted = git(
            &self.git,
            &self.root.join("project"),
            &["rev-parse", "HEAD"],
        )?;
        self.select("main", &reverted, "revert-v1")?;
        self.checkpoint(5, &reverted)?;
        git(
            &self.git,
            &self.root.join("project"),
            &["revert", "--no-edit", &reverted],
        )?;
        let mut current = git(
            &self.git,
            &self.root.join("project"),
            &["rev-parse", "HEAD"],
        )?;
        self.select("main", &current, "return-v2")?;
        self.checkpoint(6, &current)?;
        let rename_evidence = if matches!(mode, Mode::Rename) {
            let (head, evidence) = self.rename(&current)?;
            current = head;
            Some(evidence)
        } else {
            None
        };
        let exchange_evidence = if matches!(mode, Mode::Exchange) {
            let (head, evidence) = self.exchange(&current)?;
            current = head;
            Some(evidence)
        } else {
            None
        };
        let worktree_evidence = if matches!(mode, Mode::Worktree) {
            Some(self.worktree(&current, &first)?)
        } else {
            None
        };
        let pending = if matches!(mode, Mode::Exchange) {
            json!([
                "full command-by-command Git curriculum",
                "remote byte publication and offline backup",
                "failure rehearsals",
                "final retirement",
                "metadata lifetime"
            ])
        } else {
            json!([
                "full command-by-command Git curriculum",
                "publish/fetch/pull",
                "failure rehearsals",
                "final retirement",
                "metadata lifetime"
            ])
        };
        let trace = save(
            &self.root.join("trace.json"),
            &json!({"schema":"tig.teaching-trace.design-2","providers":self.config.providers.iter().map(|p|json!({"instance":p.place.instance,"id":p.assignments[0].place.as_str()})).collect::<Vec<_>>(),"completed":self.reports,"pending":pending,"fixture_native_scope":"selected objects and fixture primary inputs; historical streams retain historical source obligations"}),
        )?;
        fs::copy(
            self.root.join("trace.json"),
            Layout::Flat.object_path(&self.root.join("lesson-evidence"), &trace.sha256),
        )?;
        self.catalogue(&current, &trace)?;
        let mut result = json!({"schema":"tig.sample-result.design-1","root":self.root,"completed_checkpoints":self.reports.len(),"trace":"trace.json","catalogue":"catalogue.json","complete_course":false});
        if let Some(evidence) = exchange_evidence {
            result["exchange_evidence"] = serde_json::to_value(evidence)?;
        }
        if let Some(evidence) = worktree_evidence {
            result["worktree_evidence"] = serde_json::to_value(evidence)?;
        }
        if let Some(evidence) = backup_evidence {
            result["backup_evidence"] = serde_json::to_value(evidence)?;
        }
        if let Some(evidence) = stale_evidence {
            result["stale_evidence"] = serde_json::to_value(evidence)?;
        }
        if let Some(evidence) = rename_evidence {
            result["rename_evidence"] = serde_json::to_value(evidence)?;
        }
        Ok(result)
    }

    fn rename(&mut self, before_commit: &str) -> Result<(String, Object)> {
        let repo = self.root.join("project");
        let old = "records/series.tig";
        let new = "records/renamed-series.tig";
        let bytes = fs::read(repo.join(old))?;
        let occupied = fs::read(repo.join("records/reference.tig"))?;
        let copies = self.copy_snapshot()?;
        let index = git(&self.git, &repo, &["write-tree"])?;
        if git(&self.git, &repo, &["mv", old, "records/reference.tig"]).is_ok()
            || fs::read(repo.join(old))? != bytes
            || fs::read(repo.join("records/reference.tig"))? != occupied
            || git(&self.git, &repo, &["rev-parse", "HEAD"])? != before_commit
            || git(&self.git, &repo, &["write-tree"])? != index
            || self.copy_snapshot()? != copies
        {
            return Err("conflicting rename did not preserve exact state".into());
        }
        let blob = git(&self.git, &repo, &["rev-parse", &format!("HEAD:{old}")])?;
        git(&self.git, &repo, &["mv", old, new])?;
        let head = self.commit("Sample: rename a record without changing its bytes")?;
        if fs::read(repo.join(new))? != bytes
            || repo.join(old).exists()
            || git(&self.git, &repo, &["rev-parse", &format!("HEAD:{new}")])? != blob
        {
            return Err("rename changed authored bytes".into());
        }
        self.select("main", &head, "same-byte-rename")?;
        let (plan, receipt) = self.with_placement(|e| {
            let plan = e.plan(None)?;
            let receipt = e.apply(&plan)?;
            Ok((plan, receipt))
        })?;
        if !receipt.page_converged
            || !receipt.basis_current
            || receipt.next.is_some()
            || !receipt.holds.is_empty()
            || !receipt.effects.is_empty()
            || receipt.stream_bytes != 0
            || self.copy_snapshot()? != copies
        {
            return Err("same-byte rename changed placement".into());
        }
        self.record_checkpoint(22, &head, &plan, &receipt, false, None)?;
        let evidence = save(
            &self.root.join("rename.json"),
            &json!({
                "schema":"tig.sample-rename.design-1","before_commit":before_commit,
                "after_commit":head,"old_path":old,"new_path":new,"git_blob":blob,
                "record_object":Object::from_bytes(&bytes),"conflicting_destination_refused":true,
                "copies_before":copies,"copies_after":self.copy_snapshot()?,
                "stream_bytes":receipt.stream_bytes,"effects":receipt.effects
            }),
        )?;
        Ok((head, evidence))
    }

    fn stale_plan(&mut self, commit: &str, first: &str) -> Result<Object> {
        let old = self.with_placement(|e| e.plan(None))?;
        let planning_source = self.namespaces[0].checkpoint();
        self.select("job", first, "stale-plan-new-job")?;
        let protected_source = self.namespaces[0].checkpoint();
        let before = self.copy_snapshot()?;
        let rejected = self.with_placement(|e| Ok(e.apply(&old)))?;
        let after = self.copy_snapshot()?;
        if !matches!(rejected, Err(placement::Error::Stale))
            || after != before
            || self.namespaces[0].checkpoint() != protected_source
            || protected_source == planning_source
        {
            return Err("stale plan did not refuse before effects".into());
        }
        let status = self.with_placement(|e| e.status(&old))?;
        if status.basis_current
            || status.page_converged
            || status.storage_converged != Some(false)
            || !status.effects.is_empty()
            || status.stream_bytes != 0
            || status.next.is_some()
            || status.observed_basis == old.basis
        {
            return Err("stale status did not preserve the rejected basis".into());
        }
        let old_id = self.record_checkpoint(
            19,
            commit,
            &old,
            &status,
            false,
            Some(json!({"error":"stale","observed_basis":status.observed_basis})),
        )?;
        let fresh = self.checkpoint_coverage(20, commit, false, false)?;
        if fresh == old_id {
            return Err("fresh protection reused the stale plan".into());
        }
        self.withdraw("job", "stale-plan-job-ended")?;
        let ended = self.checkpoint_coverage(21, commit, false, false)?;
        if ended == old_id || ended == fresh {
            return Err("job end revived old plan authority".into());
        }
        save(
            &self.root.join("stale.json"),
            &json!({"schema":"tig.sample-stale-plan.design-1","error":"stale","rejected_plan":old_id,"planning_basis":old.basis,"observed_basis":status.observed_basis,"planning_source":planning_source,"protected_source":protected_source,"copies_before_rejection":before,"copies_after_rejection":after,"fresh_job_plan":fresh,"fresh_job_ended_plan":ended,"status_stream_bytes":status.stream_bytes,"status_effects":status.effects,"new_job_admission_is_explicit_fixture":true}),
        )
    }

    fn backup_outage(&mut self, commit: &str) -> Result<Object> {
        let backup = self.root.join("backup");
        let offline = self.root.join("offline-backup");
        let verified = fs::canonicalize(&backup)?;
        if verified.parent() != Some(self.root.as_path())
            || verified.file_name() != Some(std::ffi::OsStr::new("backup"))
            || offline.parent() != Some(self.root.as_path())
            || offline.exists()
        {
            return Err("outage move outside fresh root".into());
        }
        let before = self.copy_snapshot()?;
        fs::rename(&backup, &offline)?;
        self.checkpoint_coverage(16, commit, true, true)?;
        self.withdraw("job", "finish-job")?;
        let source = self.namespaces[0].checkpoint();
        let held = self.checkpoint_coverage(17, commit, true, true)?;
        let offline_verified = fs::canonicalize(&offline)?;
        if backup.exists()
            || offline_verified.parent() != Some(self.root.as_path())
            || offline_verified.file_name() != Some(std::ffi::OsStr::new("offline-backup"))
            || backup.parent() != Some(self.root.as_path())
        {
            return Err("outage restore outside fresh root".into());
        }
        fs::rename(&offline, &backup)?;
        if fs::canonicalize(&backup)? != verified {
            return Err("restored a different backup path".into());
        }
        let attempts = self.seed.attempts;
        self.seed.disabled = true;
        let recovered = self.checkpoint_coverage(18, commit, false, false)?;
        if recovered != held
            || self.namespaces[0].checkpoint() != source
            || self.seed.attempts != attempts
        {
            return Err("catchup changed source/plan or used the seed".into());
        }
        self.seed.disabled = false;
        save(
            &self.root.join("backup.json"),
            &json!({"schema":"tig.sample-backup-outage.design-1","held_plan":held,"retry_plan":recovered,"source_checkpoint":source,"seed_attempts_during_retry":self.seed.attempts-attempts,"configured_path_restored":true,"before_outage":before,"physical_hardware_fault_qualified":false}),
        )
    }

    fn worktree(&mut self, current: &str, first: &str) -> Result<Object> {
        let project = self.root.join("project");
        let study = self.root.join("study");
        let primary_index = self.repository.index()?.checkpoint().listing.clone();
        git(
            &self.git,
            &project,
            &["worktree", "add", "--detach", "../study", first],
        )?;
        let verified = fs::canonicalize(&study)?;
        if verified.parent() != Some(self.root.as_path())
            || verified.file_name() != Some(std::ffi::OsStr::new("study"))
        {
            return Err("worktree is outside the fresh fixture root".into());
        }
        if git(&self.git, &study, &["rev-parse", "HEAD"])? != first {
            return Err("worktree selected a different commit".into());
        }
        self.select("draft", first, "worktree-open")?;
        self.select("job", first, "worktree-start-job")?;
        self.checkpoint(11, current)?;

        let repo = Repository::new(self.git.clone(), study.clone(), Limits::default())?;
        let capture = |view: &tig_git::source::Snapshot| -> Result<native::AssetSnapshot> {
            Ok(repo.native(
                view,
                Scope::new([LogicalPath::parse("records")?])?,
                Scope::new([LogicalPath::parse("definitions")?])?,
                native::Limits::default(),
            )?)
        };
        let series = |snapshot: &native::AssetSnapshot| -> Result<Object> {
            Ok(snapshot
                .selected_records()
                .find(|r| r.reference().id == "work/series")
                .ok_or("series missing")?
                .selected()?
                .clone())
        };
        let record = study.join("records/series.tig");
        let old_record = fs::read(&record)?;
        fs::write(&record, fs::read(project.join("records/series.tig"))?)?;
        git(&self.git, &study, &["add", "records/series.tig"])?;
        let staged_view = repo.index()?;
        let staged = capture(&staged_view)?;
        if series(&staged)? != self.objects["fixed"] {
            return Err("index did not stage fixed bytes".into());
        }
        let cached_diff = git(&self.git, &study, &["diff", "--cached", "--name-only"])?;
        fs::write(&record, &old_record)?;
        let working_diff = git(&self.git, &study, &["diff", "--name-only"])?;
        let dirty_status = git(&self.git, &study, &["status", "--porcelain=v1"])?;
        if cached_diff != "records/series.tig"
            || working_diff != cached_diff
            || dirty_status != "MM records/series.tig"
            || series(&capture(&staged_view)?)? != self.objects["fixed"]
        {
            return Err("captured index was replaced by working-file state".into());
        }
        let copies_before = self.copy_snapshot()?;
        let dirty_remove_refused = git(&self.git, &project, &["worktree", "remove", "../study"])
            .err()
            .is_some_and(|e| {
                e.to_string()
                    .contains("contains modified or untracked files")
            });
        if !dirty_remove_refused
            || fs::read(&record)? != old_record
            || self.copy_snapshot()? != copies_before
        {
            return Err("dirty worktree removal did not preserve fixture state".into());
        }
        self.select_snapshot("draft", &staged, "worktree-stage-index")?;
        self.checkpoint(12, current)?;

        git(
            &self.git,
            &study,
            &[
                "restore",
                "--staged",
                "--source=HEAD",
                "--",
                "records/series.tig",
            ],
        )?;
        let unstaged_view = repo.index()?;
        let unstaged = capture(&unstaged_view)?;
        if series(&unstaged)? != self.objects["original"]
            || series(&staged)? != self.objects["fixed"]
            || series(&capture(&staged_view)?)? != self.objects["fixed"]
            || !git(&self.git, &study, &["status", "--porcelain=v1"])?.is_empty()
            || staged_view.checkpoint().listing == unstaged_view.checkpoint().listing
        {
            return Err("unstaging changed an old snapshot or selected wrong bytes".into());
        }
        self.select_snapshot("draft", &unstaged, "worktree-unstage-index")?;
        self.checkpoint(13, current)?;
        if self.repository.index()?.checkpoint().listing != primary_index
            || git(&self.git, &project, &["rev-parse", "HEAD"])? != current
            || git(&self.git, &study, &["rev-parse", "HEAD"])? != first
        {
            return Err("linked worktree changed the primary HEAD or index".into());
        }
        // Verify the canonical target immediately before normal, non-forced removal.
        if fs::canonicalize(&study)? != verified || verified.parent() != Some(self.root.as_path()) {
            return Err("worktree target changed".into());
        }
        git(&self.git, &project, &["worktree", "remove", "../study"])?;
        if study.exists() {
            return Err("worktree was not removed".into());
        }
        self.withdraw("draft", "worktree-close-draft")?;
        self.checkpoint(14, current)?;
        self.withdraw("job", "worktree-end-job")?;
        self.checkpoint(15, current)?;
        save(
            &self.root.join("worktree.json"),
            &json!({"schema":"tig.sample-worktree.design-1","main_commit":current,"worktree_commit":first,"primary_index":primary_index,"staged_index":staged_view.checkpoint().listing,"unstaged_index":unstaged_view.checkpoint().listing,"staged_native":staged.checkpoint(),"unstaged_native":unstaged.checkpoint(),"staged_selected":series(&staged)?,"unstaged_selected":series(&unstaged)?,"cached_diff":cached_diff,"working_diff":working_diff,"dirty_status":dirty_status,"dirty_remove_refused":dirty_remove_refused,"copy_snapshot_during_refusal":copies_before,"worktree_removed":true,"job_ended_after_worktree":true,"automatic_writer_enrollment":false}),
        )
    }

    fn exchange(&mut self, first: &str) -> Result<(String, Object)> {
        let project = self.root.join("project");
        let observer = self.root.join("observer");
        // These paths are generated under a fresh root, never caller-supplied URLs.
        git(
            &self.git,
            &self.root,
            &[
                "init",
                "--bare",
                "--template=git-template",
                "-b",
                "main",
                "exchange.git",
            ],
        )?;
        git(
            &self.git,
            &project,
            &["remote", "add", "sample", "../exchange.git"],
        )?;
        git_transport(
            &self.git,
            &project,
            &["push", "sample", "refs/heads/main:refs/heads/main"],
            true,
        )?;
        git_transport(
            &self.git,
            &self.root,
            &[
                "clone",
                "--no-local",
                "--no-hardlinks",
                "--template=git-template",
                "--branch",
                "main",
                "exchange.git",
                "observer",
            ],
            true,
        )?;
        if git(&self.git, &observer, &["rev-parse", "HEAD"])? != first {
            return Err("clone selected a different commit".into());
        }
        // Native payload seeds must not have been smuggled into the Git tree.
        let files = git(
            &self.git,
            &observer,
            &["ls-tree", "-r", "--name-only", "HEAD"],
        )?;
        for file in files.lines() {
            let bytes = fs::read(observer.join(file))?;
            if self
                .objects
                .values()
                .any(|object| *object == Object::from_bytes(&bytes))
            {
                return Err("payload was transported as a Git blob".into());
            }
        }
        self.checkpoint(8, first)?;

        git(&self.git, &project, &["revert", "--no-edit", first])?;
        let old = git(&self.git, &project, &["rev-parse", "HEAD"])?;
        git_transport(
            &self.git,
            &project,
            &["push", "sample", "refs/heads/main:refs/heads/main"],
            true,
        )?;
        git_transport(
            &self.git,
            &observer,
            &[
                "fetch",
                "--no-tags",
                "origin",
                "refs/heads/main:refs/remotes/origin/main",
            ],
            true,
        )?;
        let after_fetch = git(&self.git, &observer, &["rev-parse", "HEAD"])?;
        if after_fetch != first
            || git(
                &self.git,
                &observer,
                &["rev-parse", "refs/remotes/origin/main"],
            )? != old
        {
            return Err("fetch changed the checkout or selected a different ref".into());
        }
        let record_path = observer.join("records/series.tig");
        let clean = fs::read(&record_path)?;
        let mut dirty: Value = serde_json::from_slice(&clean)?;
        dirty["authored"]["version"] = "uncommitted-learner-edit".into();
        let dirty = identity::canonical_json(&dirty)?;
        fs::write(&record_path, &dirty)?;
        let before = self.copy_snapshot()?;
        let refused = git_transport(
            &self.git,
            &observer,
            &["pull", "--ff-only", "origin", "main"],
            true,
        )
        .err()
        .is_some_and(|error| error.to_string().contains("would be overwritten by merge"));
        if !refused
            || fs::read(&record_path)? != dirty
            || git(&self.git, &observer, &["rev-parse", "HEAD"])? != first
            || self.copy_snapshot()? != before
        {
            return Err("dirty pull did not preserve the checkout and managed copies".into());
        }
        // Repair only the exact synthetic edit made above; no user checkout is admitted.
        fs::write(&record_path, clean)?;
        git_transport(
            &self.git,
            &observer,
            &["pull", "--ff-only", "origin", "main"],
            true,
        )?;
        self.check_observer(&observer, &old, "original")?;
        self.select("main", &old, "exchange-v1")?;
        self.checkpoint(9, &old)?;

        git(&self.git, &project, &["revert", "--no-edit", &old])?;
        let current = git(&self.git, &project, &["rev-parse", "HEAD"])?;
        git_transport(
            &self.git,
            &project,
            &["push", "sample", "refs/heads/main:refs/heads/main"],
            true,
        )?;
        git_transport(
            &self.git,
            &observer,
            &[
                "fetch",
                "--no-tags",
                "origin",
                "refs/heads/main:refs/remotes/origin/main",
            ],
            true,
        )?;
        if git(&self.git, &observer, &["rev-parse", "HEAD"])? != old {
            return Err("fetch advanced observer HEAD".into());
        }
        git_transport(
            &self.git,
            &observer,
            &["pull", "--ff-only", "origin", "main"],
            true,
        )?;
        self.check_observer(&observer, &current, "fixed")?;
        self.select("main", &current, "exchange-v2")?;
        self.checkpoint(10, &current)?;
        let evidence = save(
            &self.root.join("exchange.json"),
            &json!({"schema":"tig.sample-exchange.design-1","published":first,"after_fetch":after_fetch,"pulled_v1":old,"returned_v2":current,"dirty_pull_refused":refused,"dirty_pull_reason":"local changes would be overwritten by merge","dirty_copy_snapshot":before,"selected_payloads_in_cloned_tree":false,"observer_is_enrolled_writer":false,"remote_transport":"fresh-root-local-file-only"}),
        )?;
        Ok((current, evidence))
    }
    fn check_observer(&self, observer: &Path, commit: &str, name: &str) -> Result<()> {
        let record: Value =
            serde_json::from_slice(&fs::read(observer.join("records/series.tig"))?)?;
        if git(&self.git, observer, &["rev-parse", "HEAD"])? != commit
            || record["selected"] != serde_json::to_value(&self.objects[name])?
            || !git(&self.git, observer, &["status", "--porcelain=v1"])?.is_empty()
        {
            return Err("observer did not select exact clean records".into());
        }
        Ok(())
    }
    fn copy_snapshot(&self) -> Result<Value> {
        let mut observed = BTreeMap::new();
        for role in ["working", "history", "backup"] {
            let mut copies = BTreeMap::new();
            for (name, object) in &self.objects {
                let path = Layout::Flat.object_path(&self.root.join(role), &object.sha256);
                match fs::read(path) {
                    Ok(bytes) => {
                        if Object::from_bytes(&bytes) != *object {
                            return Err("copy changed during sample rehearsal".into());
                        }
                        copies.insert(name, object);
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
                    Err(e) => return Err(e.into()),
                }
            }
            observed.insert(role, copies);
        }
        Ok(serde_json::to_value(observed)?)
    }
}
fn run() -> Result<Value> {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();
    if args.len() == 1 && args[0] == "describe" {
        return Ok(
            json!({"schema":"tig.sample-interface.design-1","operations":["describe","rehearse-ordinary NEW_ABSOLUTE_ROOT","rehearse-exchange NEW_ABSOLUTE_ROOT","rehearse-worktree NEW_ABSOLUTE_ROOT","rehearse-backup-outage NEW_ABSOLUTE_ROOT","rehearse-stale-plan NEW_ABSOLUTE_ROOT","rehearse-rename NEW_ABSOLUTE_ROOT"],"fixture_only":true,"opens_existing_roots":false,"complete_course":false}),
        );
    }
    if args.len() != 2 {
        return Err("usage: tig-sample describe | OPERATION NEW_ABSOLUTE_ROOT (see describe for operations)".into());
    }
    let mode = match args[0].to_str() {
        Some("rehearse-ordinary") => Mode::Ordinary,
        Some("rehearse-exchange") => Mode::Exchange,
        Some("rehearse-worktree") => Mode::Worktree,
        Some("rehearse-backup-outage") => Mode::BackupOutage,
        Some("rehearse-stale-plan") => Mode::StalePlan,
        Some("rehearse-rename") => Mode::Rename,
        _ => return Err("unknown sample operation".into()),
    };
    Demo::new(Path::new(&args[1]))?.run(mode)
}
fn main() -> ExitCode {
    match run() {
        Ok(v) => {
            println!("{v}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!(
                "{}",
                json!({"schema":"tig.sample-error.design-1","error":e.to_string()})
            );
            ExitCode::FAILURE
        }
    }
}
