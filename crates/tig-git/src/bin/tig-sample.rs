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
    existing::{Layout, ReadCache},
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
    seed: ReadCache,
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
            seed: ReadCache::new(serde_json::from_value(byte_source)?)?,
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
    fn checkpoint(&mut self, index: usize, commit: &str) -> Result<()> {
        let stage = &self.fixture["stages"][index];
        let id = stage["id"].as_str().ok_or("stage missing")?.to_owned();
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
        let plan = executor.plan(None)?;
        let receipt = executor.apply(&plan)?;
        if !receipt.page_converged || receipt.next.is_some() || !receipt.holds.is_empty() {
            return Err(format!("{id}: unresolved placement: {:?}", receipt.holds).into());
        }
        let mut observations = BTreeMap::new();
        for (name, object) in &self.objects {
            let mut actual = BTreeSet::new();
            for (role, store) in ["working", "history", "backup"]
                .into_iter()
                .zip(&self.stores)
            {
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
            observations.insert(name, json!({"object":object,"verified_copies":actual}));
        }
        let report = json!({"schema":"tig.teaching-observation.design-1","lesson":id,"source_commit":commit,"source_checkpoint":self.namespaces[0].checkpoint(),"plan":plan,"receipt":receipt,"observations":observations,"declared_fixture_domains":3,"physical_domain_coverage":"not-established; portable fixture directories","independent_hardware_backup_proven":false});
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
        Ok(())
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
    fn run(mut self, exchange: bool) -> Result<Value> {
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
        self.checkpoint(3, &merged)?;
        self.withdraw("job", "finish-job")?;
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
        let exchange_evidence = if exchange {
            let (head, evidence) = self.exchange(&current)?;
            current = head;
            Some(evidence)
        } else {
            None
        };
        let pending = if exchange {
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
        Ok(result)
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
                            return Err("copy changed during Git exchange".into());
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
            json!({"schema":"tig.sample-interface.design-1","operations":["describe","rehearse-ordinary NEW_ABSOLUTE_ROOT","rehearse-exchange NEW_ABSOLUTE_ROOT"],"fixture_only":true,"opens_existing_roots":false,"complete_course":false}),
        );
    }
    if args.len() != 2 || (args[0] != "rehearse-ordinary" && args[0] != "rehearse-exchange") {
        return Err("usage: tig-sample describe | rehearse-ordinary NEW_ABSOLUTE_ROOT | rehearse-exchange NEW_ABSOLUTE_ROOT".into());
    }
    Demo::new(Path::new(&args[1]))?.run(args[0] == "rehearse-exchange")
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
