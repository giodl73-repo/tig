//! Explicit metadata-only native historical binding and selection adapter.
#![forbid(unsafe_code)]
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::{
    collections::BTreeSet,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::ExitCode,
};
use tig_assets::graph::{Profile, Reference};
use tig_core::{Digest, Object, identity};
use tig_git::source::native::retirement::{self, Categories, Filter, Limits};
use tig_store::{
    Key,
    final_retirement::bulk,
    metadata::{Checkpoint, MetadataReader, MetadataStore},
    source_claims::{self, Kind, admission::Namespace},
};
type Result<T> = std::result::Result<T, &'static str>;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Metadata {
    root: PathBuf,
    minimum: Checkpoint,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Source {
    root: PathBuf,
    minimum: source_claims::Checkpoint,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Bind {
    schema: String,
    metadata: Metadata,
    snapshot: Object,
    roots: BTreeSet<Reference>,
    profiles: Vec<Profile>,
    #[serde(default)]
    allow_bind: bool,
    #[serde(default)]
    limits: Limits,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Select {
    schema: String,
    metadata: Metadata,
    namespace: Source,
    source: Key,
    owner: Key,
    predecessor_revision: u64,
    header: Digest,
    categories: Categories,
    #[serde(default)]
    limits: Limits,
}
fn document<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let meta = std::fs::symlink_metadata(path).map_err(|_| "document_unavailable")?;
    if !meta.is_file() || meta.len() > 65_536 {
        return Err("invalid_document");
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        if meta.file_attributes() & 0x400 != 0 {
            return Err("invalid_document");
        }
    }
    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        options.share_mode(1);
    }
    let file = options.open(path).map_err(|_| "document_unavailable")?;
    let mut bytes = Vec::new();
    file.take(65_537)
        .read_to_end(&mut bytes)
        .map_err(|_| "document_unavailable")?;
    if bytes.len() > 65_536 {
        return Err("invalid_document");
    }
    serde_json::from_value(identity::parse_json(&bytes).map_err(|_| "invalid_document")?)
        .map_err(|_| "invalid_document")
}
fn code(e: retirement::Error) -> &'static str {
    match e {
        retirement::Error::Invalid => "invalid_selection",
        retirement::Error::Budget => "budget",
        retirement::Error::Incomplete => "incomplete_scope",
        retirement::Error::Integrity => "wrong_binding",
        retirement::Error::Metadata => "metadata_unavailable",
    }
}
fn run(args: &[String]) -> Result<Value> {
    if args == ["describe"] {
        return Ok(json!({"schema":"tig.native-retirement-interface.design-1",
            "operations":["bind PROFILE","select PROFILE REQUEST"],
            "bind_profile_schema":"tig.native-retirement-bind-profile.design-1",
            "select_profile_schema":"tig.native-retirement-select-profile.design-1",
            "max_document_bytes":65536,"max_response_bytes":65536,
            "initializes_stores":false,"bind_default_enabled":false,
            "select_read_only":true,"age_filter_supported":false,"authorizes_removal":false}));
    }
    if args.len() == 2 && args[0] == "bind" {
        let p: Bind = document(Path::new(&args[1]))?;
        if p.schema != "tig.native-retirement-bind-profile.design-1"
            || !p.metadata.root.is_absolute()
        {
            return Err("invalid_profile");
        }
        if !p.allow_bind {
            return Err("binding_disabled");
        }
        p.limits.validate().map_err(code)?;
        let mut metadata = MetadataStore::open(&p.metadata.root, &p.metadata.minimum)
            .map_err(|_| "metadata_unavailable")?;
        let selection = retirement::build_from_archive(
            &p.snapshot,
            p.roots,
            p.profiles,
            &mut metadata,
            p.limits,
        )
        .map_err(code)?;
        let minimum = metadata.finish().map_err(|_| "metadata_unavailable")?;
        drop(metadata);
        let mut reader =
            MetadataReader::open(&p.metadata.root, &minimum).map_err(|_| "metadata_unavailable")?;
        let inventory = retirement::inspect(&selection, &selection.context, &mut reader, p.limits)
            .map_err(code)?;
        return Ok(
            json!({"schema":"tig.native-retirement-binding-result.design-1",
            "selection":selection,"metadata_checkpoint":minimum,"assets":inventory.assets(),
            "header":inventory.header(),"source_checkpoint":inventory.checkpoint(),
            "authorizes_source_adoption":false,"authorizes_removal":false}),
        );
    }
    if args.len() != 3 || args[0] != "select" {
        return Err("usage");
    }
    let filter: Filter = document(Path::new(&args[2]))?;
    let p: Select = document(Path::new(&args[1]))?;
    if p.schema != "tig.native-retirement-select-profile.design-1"
        || !p.metadata.root.is_absolute()
        || !p.namespace.root.is_absolute()
    {
        return Err("invalid_profile");
    }
    p.limits.validate().map_err(code)?;
    let mut namespace = Namespace::open(&p.namespace.root, &p.namespace.minimum)
        .map_err(|_| "source_unavailable")?;
    let source = namespace
        .source(&p.source)
        .map_err(|_| "source_unavailable")?
        .ok_or("unknown_source")?;
    if source.owner != p.owner
        || source.pending.is_some()
        || !matches!(source.kind, Some(Kind::Branch | Kind::Draft))
    {
        return Err("wrong_authority");
    }
    let version = source
        .predecessors
        .iter()
        .find(|v| v.revision == p.predecessor_revision)
        .ok_or("unknown_predecessor")?;
    let mut reader = MetadataReader::open(&p.metadata.root, &p.metadata.minimum)
        .map_err(|_| "metadata_unavailable")?;
    let inventory =
        retirement::inspect(&version.selection, &p.header, &mut reader, p.limits).map_err(code)?;
    let preview = retirement::select(&inventory, &p.categories, &filter).map_err(code)?;
    // A second observation rejects concurrent source changes; this is no lease.
    let before = namespace.checkpoint();
    let current = namespace
        .source(&p.source)
        .map_err(|_| "source_unavailable")?;
    if namespace.checkpoint() != before || current.as_ref() != Some(&source) {
        return Err("stale_basis");
    }
    let selection = if preview.paths.is_empty() {
        None
    } else {
        Some(bulk::Selection {
            namespace: namespace.identity().clone(),
            request: preview
                .request(p.source, p.predecessor_revision)
                .map_err(code)?,
        })
    };
    Ok(
        json!({"schema":"tig.native-retirement-preview.design-1","source_checkpoint":namespace.checkpoint(),
        "selection":selection,"preview":preview,"authorizes_removal":false}),
    )
}
fn main() -> ExitCode {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let (value, success) = match run(&args) {
        Ok(value) => (value, true),
        Err(code) => (
            json!({"schema":"tig.native-retirement-error.design-1","code":code,"ok":false}),
            false,
        ),
    };
    let (bytes,success)=match serde_json::to_vec(&value) { Ok(raw) if raw.len()<=65_536=>(raw,success),
        _=>(br#"{"schema":"tig.native-retirement-error.design-1","code":"response_budget","ok":false}"#.to_vec(),false) };
    if std::io::stdout().lock().write_all(&bytes).is_err() {
        return ExitCode::FAILURE;
    }
    if success {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
