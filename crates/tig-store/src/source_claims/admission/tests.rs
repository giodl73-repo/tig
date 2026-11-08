use super::super::Selection;
use super::*;
use std::{cell::Cell, path::PathBuf, process::Command as Process};
use tig_core::{
    LogicalPath,
    tree::{MemoryNodes, TreeResult},
};
fn key(s: &str) -> Key {
    Key::parse(s).unwrap()
}
fn limits() -> Limits {
    Limits {
        millis: 60_000,
        roots: 256,
        bindings: 10000,
        associations: 100000,
        reads: 100000,
        read_bytes: 32 * 1024 * 1024,
    }
}
fn policy(prefer_active: bool) -> Policy {
    let places = Places {
        current: vec![key("working"), key("backup")],
        pending: vec![key("working"), key("backup")],
        predecessor: vec![key("history"), key("backup")],
    };
    let retained = Places {
        current: vec![key("backup")],
        pending: vec![key("backup")],
        predecessor: vec![key("backup")],
    };
    Policy {
        prefer_active,
        classes: [
            (Kind::Branch, places.clone()),
            (Kind::Draft, places.clone()),
            (Kind::Job, places),
            (Kind::Retention, retained),
        ]
        .into_iter()
        .collect(),
    }
}
fn tree(nodes: &mut impl NodeStore, entries: &[(&str, Object)]) -> Selection {
    let entries = entries
        .iter()
        .map(|(name, object)| (LogicalPath::parse(name).unwrap(), object.clone()))
        .collect();
    let (tree, _) = Tree::build(&entries, nodes, TreeLimits::default()).unwrap();
    Selection {
        root: tree.root().clone(),
        context: Digest::blob(b"context"),
    }
}
struct Fixture {
    temp: tempfile::TempDir,
    namespace: Namespace,
    nodes: MemoryNodes,
    a: Object,
    b: Object,
    common: Object,
    old: Selection,
    new: Selection,
}
impl Fixture {
    fn new(prefer_active: bool) -> Self {
        let temp = tempfile::tempdir().unwrap();
        let namespace = Namespace::create(
            &temp.path().join("scope"),
            key("namespace"),
            key("epoch"),
            policy(prefer_active),
        )
        .unwrap();
        let mut nodes = MemoryNodes::default();
        let a = Object::from_bytes(b"old");
        let b = Object::from_bytes(b"new");
        let common = Object::from_bytes(b"common");
        let old = tree(
            &mut nodes,
            &[("changed", a.clone()), ("unchanged", common.clone())],
        );
        let new = tree(
            &mut nodes,
            &[("changed", b.clone()), ("unchanged", common.clone())],
        );
        Self {
            temp,
            namespace,
            nodes,
            a,
            b,
            common,
            old,
            new,
        }
    }
    fn root(&self) -> PathBuf {
        self.temp.path().join("scope")
    }
    fn enroll(&mut self, source: &str, class: Kind) {
        self.namespace
            .apply(
                key(&format!("enroll-{source}")),
                Command::Enroll {
                    source: key(source),
                    owner: key("owner"),
                    class,
                },
                &self.nodes,
                &limits(),
            )
            .unwrap();
    }
    fn selected(&mut self, source: &str, selection: Selection) {
        let revision = self
            .namespace
            .source(&key(source))
            .unwrap()
            .unwrap()
            .revision;
        let receipt = self
            .namespace
            .apply(
                key(&format!("prepare-{source}-{revision}")),
                Command::Prepare {
                    source: key(source),
                    owner: key("owner"),
                    expected_revision: revision,
                    selection,
                },
                &self.nodes,
                &limits(),
            )
            .unwrap();
        self.namespace
            .apply(
                key(&format!("adopt-{source}-{revision}")),
                Command::Adopt {
                    source: key(source),
                    owner: key("owner"),
                    expected_revision: revision + 1,
                    prepared: receipt.reference,
                },
                &self.nodes,
                &limits(),
            )
            .unwrap();
    }
    fn absent(&mut self, object: &Object, place: &str) -> bool {
        let fence = self.namespace.fence(object).unwrap();
        self.namespace
            .absence(&fence, &key(place), &self.nodes, &limits())
            .unwrap()
            .is_some()
    }
}
#[test]
fn guarded_marker_and_legacy_encoding_are_distinct_and_direct_bypass_refuses() {
    let mut f = Fixture::new(true);
    let minimum = f.namespace.checkpoint();
    let marker = fs::read(f.root().join("claims.json")).unwrap();
    let value: serde_json::Value = serde_json::from_slice(&marker).unwrap();
    assert_eq!(value["schema"], "tig.source-claims.design-2");
    assert!(value.get("admission").is_some());
    assert_eq!(
        fs::read_dir(f.root().join("admission-fences"))
            .unwrap()
            .count(),
        256
    );
    let command = Command::Enroll {
        source: key("main"),
        owner: key("owner"),
        class: Kind::Branch,
    };
    let mut direct = SourceJournal::open(&f.root(), &minimum).unwrap();
    assert_eq!(
        direct.apply(key("direct"), command.clone()),
        Err(super::super::Error::AdmissionRequired)
    );
    assert_eq!(
        direct.apply_hook(key("hook"), command.clone(), |_| Ok(())),
        Err(super::super::Error::AdmissionRequired)
    );
    assert_eq!(
        SourceJournal::preview(&f.root(), &minimum, key("preview"), command.clone()),
        Err(super::super::Error::AdmissionRequired)
    );
    assert_eq!(direct.checkpoint(), minimum);
    let receipt = f
        .namespace
        .apply(key("enrolled"), command.clone(), &f.nodes, &limits())
        .unwrap();
    let recovered =
        SourceJournal::acknowledge(&f.root(), &minimum, key("enrolled"), command).unwrap();
    assert!(recovered.replayed);
    assert_eq!(receipt.checkpoint, recovered.checkpoint);
    let legacy_root = f.temp.path().join("legacy");
    let legacy = SourceJournal::create(&legacy_root, key("project"), key("epoch")).unwrap();
    let raw = fs::read(legacy_root.join("claims.json")).unwrap();
    let legacy_value: serde_json::Value = serde_json::from_slice(&raw).unwrap();
    assert_eq!(legacy_value["schema"], "tig.source-claims.design-1");
    assert!(legacy_value.get("admission").is_none());
    assert_eq!(fs::read_dir(&legacy_root).unwrap().count(), 2);
    assert!(Namespace::open(&legacy_root, &legacy.checkpoint()).is_err());
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct OldConfig {
        schema: String,
        project: Key,
        epoch: Key,
        instance: Digest,
    }
    assert!(serde_json::from_slice::<OldConfig>(&marker).is_err());
    let old: OldConfig = serde_json::from_slice(&raw).unwrap();
    assert_eq!(old.schema, "tig.source-claims.design-1");
    let _ = (old.project, old.epoch, old.instance);
}
#[test]
fn source_enrollment_is_mandatory_and_class_owner_and_operation_are_immutable() {
    let mut f = Fixture::new(true);
    let before = f.namespace.checkpoint();
    let request = Command::Prepare {
        source: key("main"),
        owner: key("owner"),
        expected_revision: 0,
        selection: f.old.clone(),
    };
    assert!(matches!(
        f.namespace
            .apply(key("unregistered"), request, &f.nodes, &limits()),
        Err(Error::Journal(super::super::Error::AdmissionRequired))
    ));
    assert_eq!(f.namespace.checkpoint(), before);
    f.enroll("main", Kind::Branch);
    assert_eq!(
        f.namespace.source(&key("main")).unwrap().unwrap().kind,
        Some(Kind::Branch)
    );
    assert!(
        f.namespace
            .apply(
                key("steal"),
                Command::Enroll {
                    source: key("main"),
                    owner: key("foreign"),
                    class: Kind::Job
                },
                &f.nodes,
                &limits()
            )
            .is_err()
    );
    assert!(
        f.namespace
            .apply(
                key("reclassify"),
                Command::Enroll {
                    source: key("main"),
                    owner: key("owner"),
                    class: Kind::Job
                },
                &f.nodes,
                &limits()
            )
            .is_err()
    );
    assert!(
        f.namespace
            .apply(
                key("enroll-main"),
                Command::Enroll {
                    source: key("other"),
                    owner: key("owner"),
                    class: Kind::Branch
                },
                &f.nodes,
                &limits()
            )
            .is_err()
    );
    let receipt = f
        .namespace
        .apply(
            key("enroll-main"),
            Command::Enroll {
                source: key("main"),
                owner: key("owner"),
                class: Kind::Branch,
            },
            &f.nodes,
            &limits(),
        )
        .unwrap();
    assert!(receipt.replayed);
}
#[test]
fn exclusive_subject_blocks_late_claim_and_allows_it_only_after_release() {
    let mut f = Fixture::new(true);
    f.enroll("main", Kind::Branch);
    let before = f.namespace.checkpoint();
    let fence = f.namespace.fence(&f.a).unwrap();
    let request = Command::Prepare {
        source: key("main"),
        owner: key("owner"),
        expected_revision: 1,
        selection: f.old.clone(),
    };
    assert_eq!(
        f.namespace
            .apply(key("late"), request.clone(), &f.nodes, &limits()),
        Err(Error::Busy)
    );
    assert_eq!(f.namespace.checkpoint(), before);
    let fact = f
        .namespace
        .absence(&fence, &key("working"), &f.nodes, &limits())
        .unwrap()
        .unwrap();
    assert!(!fact.authorizes_removal());
    assert_eq!(fact.object(), &f.a);
    assert_eq!(fact.authority(), f.namespace.identity());
    drop(fact);
    drop(fence);
    f.namespace
        .apply(key("late"), request, &f.nodes, &limits())
        .unwrap();
    let fence = f.namespace.fence(&f.a).unwrap();
    assert!(
        f.namespace
            .absence(&fence, &key("working"), &f.nodes, &limits())
            .unwrap()
            .is_none()
    );
}
#[test]
fn unknown_source_can_enroll_but_cannot_add_conflicting_payload_under_subject_fence() {
    let mut f = Fixture::new(true);
    let fence = f.namespace.fence(&f.a).unwrap();
    f.enroll("new-source", Kind::Job);
    let before = f.namespace.checkpoint();
    assert_eq!(
        f.namespace.apply(
            key("new-payload"),
            Command::Prepare {
                source: key("new-source"),
                owner: key("owner"),
                expected_revision: 1,
                selection: f.old.clone()
            },
            &f.nodes,
            &limits()
        ),
        Err(Error::Busy)
    );
    assert_eq!(f.namespace.checkpoint(), before);
    assert!(
        f.namespace
            .absence(&fence, &key("working"), &f.nodes, &limits())
            .unwrap()
            .is_some()
    );
}
#[test]
fn active_history_precedence_does_not_duplicate_unchanged_or_job_used_bytes() {
    let mut f = Fixture::new(true);
    f.enroll("main", Kind::Branch);
    f.selected("main", f.old.clone());
    f.selected("main", f.new.clone());
    let a = f.a.clone();
    let b = f.b.clone();
    let common = f.common.clone();
    assert!(f.absent(&a, "working"));
    assert!(!f.absent(&a, "history"));
    assert!(!f.absent(&a, "backup"));
    assert!(!f.absent(&b, "working"));
    assert!(f.absent(&common, "history"));
    f.enroll("job", Kind::Job);
    f.selected("job", f.old.clone());
    assert!(!f.absent(&a, "working"));
    assert!(f.absent(&a, "history"));
    let revision = f.namespace.source(&key("job")).unwrap().unwrap().revision;
    f.namespace
        .apply(
            key("withdraw-job"),
            Command::Withdraw {
                source: key("job"),
                owner: key("owner"),
                expected_revision: revision,
            },
            &f.nodes,
            &limits(),
        )
        .unwrap();
    assert!(f.absent(&a, "working"));
    assert!(!f.absent(&a, "history"));
}
#[test]
fn retained_class_is_additive_and_precedence_can_be_explicitly_disabled() {
    let mut p = policy(true);
    let history = Places {
        current: vec![key("history")],
        pending: vec![key("history")],
        predecessor: vec![key("history")],
    };
    p.classes.insert(Kind::Retention, history);
    let mut f = Fixture::new(true);
    let path = f.temp.path().join("hard-retention");
    f.namespace = Namespace::create(&path, key("project"), key("epoch"), p).unwrap();
    f.enroll("main", Kind::Branch);
    f.selected("main", f.old.clone());
    f.enroll("archive", Kind::Retention);
    f.selected("archive", f.old.clone());
    let a = f.a.clone();
    assert!(!f.absent(&a, "history"));
    assert!(!f.absent(&a, "working"));
    let mut f = Fixture::new(false);
    f.enroll("main", Kind::Branch);
    f.selected("main", f.old.clone());
    f.selected("main", f.new.clone());
    let common = f.common.clone();
    assert!(!f.absent(&common, "history"));
}
#[test]
fn role_changes_are_fenced_even_when_the_object_identity_is_unchanged() {
    let mut f = Fixture::new(true);
    f.enroll("main", Kind::Branch);
    f.selected("main", f.old.clone());
    let before = f.namespace.source(&key("main")).unwrap().unwrap();
    let prepared = f
        .namespace
        .apply(
            key("same-prepare"),
            Command::Prepare {
                source: key("main"),
                owner: key("owner"),
                expected_revision: before.revision,
                selection: f.old.clone(),
            },
            &f.nodes,
            &limits(),
        )
        .unwrap();
    let fence = f.namespace.fence(&f.a).unwrap();
    let before = f.namespace.checkpoint();
    assert_eq!(
        f.namespace.apply(
            key("same-adopt"),
            Command::Adopt {
                source: key("main"),
                owner: key("owner"),
                expected_revision: 4,
                prepared: prepared.reference
            },
            &f.nodes,
            &limits()
        ),
        Err(Error::Busy)
    );
    assert_eq!(f.namespace.checkpoint(), before);
    drop(fence);
}
fn unrelated(target: &Object) -> Object {
    (0..1024)
        .map(|n| Object::from_bytes(format!("unrelated-{n}").as_bytes()))
        .find(|o| partition(&o.sha256).unwrap() != partition(&target.sha256).unwrap())
        .unwrap()
}
#[test]
fn unrelated_partition_admission_proceeds_and_same_partition_contention_is_honest() {
    let mut f = Fixture::new(true);
    f.enroll("other", Kind::Draft);
    let other = unrelated(&f.a);
    let selection = tree(&mut f.nodes, &[("other", other)]);
    let fence = f.namespace.fence(&f.a).unwrap();
    f.namespace
        .apply(
            key("other-prepare"),
            Command::Prepare {
                source: key("other"),
                owner: key("owner"),
                expected_revision: 1,
                selection,
            },
            &f.nodes,
            &limits(),
        )
        .unwrap();
    assert!(
        f.namespace
            .absence(&fence, &key("working"), &f.nodes, &limits())
            .unwrap()
            .is_some()
    );
    assert!(matches!(f.namespace.fence(&f.a), Err(Error::Busy)));
}
struct Mutating<'a> {
    inner: &'a MemoryNodes,
    path: PathBuf,
    witness: Checkpoint,
    selection: Selection,
    fired: Cell<bool>,
}
impl NodeStore for Mutating<'_> {
    fn get(&self, o: &Object) -> TreeResult<Option<Vec<u8>>> {
        if !self.fired.replace(true) {
            let mut peer = Namespace::open(&self.path, &self.witness).unwrap();
            peer.apply(
                key("late-enroll"),
                Command::Enroll {
                    source: key("late"),
                    owner: key("owner"),
                    class: Kind::Job,
                },
                self.inner,
                &limits(),
            )
            .unwrap();
            peer.apply(
                key("late-prepare"),
                Command::Prepare {
                    source: key("late"),
                    owner: key("owner"),
                    expected_revision: 1,
                    selection: self.selection.clone(),
                },
                self.inner,
                &limits(),
            )
            .unwrap();
        }
        self.inner.get(o)
    }
    fn put(&mut self, _: &[u8]) -> TreeResult<Object> {
        panic!("assessment is read-only")
    }
}
#[test]
fn subject_predicate_remains_valid_while_unrelated_heads_advance_during_io() {
    let mut f = Fixture::new(true);
    f.enroll("main", Kind::Branch);
    f.selected("main", f.old.clone());
    let target = Object::from_bytes(b"unclaimed");
    let other = unrelated(&target);
    let selection = tree(&mut f.nodes, &[("other", other)]);
    let fence = f.namespace.fence(&target).unwrap();
    let witness = f.namespace.checkpoint();
    let path = f.root();
    let nodes = Mutating {
        inner: &f.nodes,
        path,
        witness: witness.clone(),
        selection,
        fired: Cell::new(false),
    };
    let fact = f
        .namespace
        .absence(&fence, &key("working"), &nodes, &limits())
        .unwrap()
        .unwrap();
    assert!(nodes.fired.get());
    assert_eq!(fact.checkpoint(), &witness);
    drop(fact);
    let current = SourceJournal::observe_all(&f.root(), &witness, 1024).unwrap();
    assert!(current.checkpoint.sequence() > witness.sequence());
}
struct Missing;
impl NodeStore for Missing {
    fn get(&self, _: &Object) -> TreeResult<Option<Vec<u8>>> {
        Ok(None)
    }
    fn put(&mut self, _: &[u8]) -> TreeResult<Object> {
        panic!("no writes")
    }
}
#[test]
fn missing_closure_and_budgets_refuse_before_mutation_or_absence() {
    let mut f = Fixture::new(true);
    f.enroll("main", Kind::Branch);
    let before = f.namespace.checkpoint();
    let command = Command::Prepare {
        source: key("main"),
        owner: key("owner"),
        expected_revision: 1,
        selection: f.old.clone(),
    };
    assert_eq!(
        f.namespace
            .apply(key("missing"), command.clone(), &Missing, &limits()),
        Err(Error::IncompleteClosure)
    );
    assert_eq!(f.namespace.checkpoint(), before);
    let mut tiny = limits();
    tiny.reads = 1;
    assert_eq!(
        f.namespace
            .apply(key("tiny"), command.clone(), &f.nodes, &tiny),
        Err(Error::Budget)
    );
    assert_eq!(f.namespace.checkpoint(), before);
    f.namespace
        .apply(key("good"), command, &f.nodes, &limits())
        .unwrap();
    let fence = f.namespace.fence(&f.a).unwrap();
    assert!(matches!(
        f.namespace
            .absence(&fence, &key("working"), &Missing, &limits()),
        Err(Error::IncompleteClosure)
    ));
    assert!(matches!(f.namespace.fence(&f.a), Err(Error::Busy)));
}
#[test]
fn foreign_fence_size_mismatch_and_unknown_place_cannot_mint_absence() {
    let mut f = Fixture::new(true);
    f.enroll("main", Kind::Branch);
    f.selected("main", f.old.clone());
    let other = Fixture::new(true);
    let fence = other.namespace.fence(&f.a).unwrap();
    assert!(matches!(
        f.namespace
            .absence(&fence, &key("working"), &f.nodes, &limits()),
        Err(Error::WrongAuthority)
    ));
    drop(fence);
    let mut wrong = f.a.clone();
    wrong.bytes += 1;
    let fence = f.namespace.fence(&wrong).unwrap();
    assert!(matches!(
        f.namespace
            .absence(&fence, &key("working"), &f.nodes, &limits()),
        Err(Error::ConflictingSize)
    ));
    assert!(matches!(
        f.namespace
            .absence(&fence, &key("unknown"), &f.nodes, &limits()),
        Err(Error::InvalidPolicy)
    ));
}
#[test]
fn exact_replay_does_not_need_old_nodes_and_changed_operation_refuses() {
    let mut f = Fixture::new(true);
    f.enroll("main", Kind::Branch);
    let command = Command::Prepare {
        source: key("main"),
        owner: key("owner"),
        expected_revision: 1,
        selection: f.old.clone(),
    };
    let one = f
        .namespace
        .apply(key("prepare"), command.clone(), &f.nodes, &limits())
        .unwrap();
    let before = f.namespace.checkpoint();
    let two = f
        .namespace
        .apply(key("prepare"), command.clone(), &Missing, &limits())
        .unwrap();
    assert!(two.replayed);
    assert_eq!(one.checkpoint, two.checkpoint);
    assert_eq!(before, f.namespace.checkpoint());
    let changed = Command::Prepare {
        source: key("main"),
        owner: key("owner"),
        expected_revision: 1,
        selection: f.new.clone(),
    };
    assert!(matches!(
        f.namespace
            .apply(key("prepare"), changed, &Missing, &limits()),
        Err(Error::Journal(super::super::Error::Conflict))
    ));
}
#[test]
fn source_change_during_compilation_refuses_stale_permit_without_overwriting_peer() {
    struct Racing<'a> {
        nodes: &'a MemoryNodes,
        root: PathBuf,
        minimum: Checkpoint,
        command: Command,
        fired: Cell<bool>,
    }
    impl NodeStore for Racing<'_> {
        fn get(&self, object: &Object) -> TreeResult<Option<Vec<u8>>> {
            if !self.fired.replace(true) {
                Namespace::open(&self.root, &self.minimum)
                    .unwrap()
                    .apply(key("peer"), self.command.clone(), self.nodes, &limits())
                    .unwrap();
            }
            self.nodes.get(object)
        }
        fn put(&mut self, _: &[u8]) -> TreeResult<Object> {
            panic!("compilation is read-only")
        }
    }
    let mut f = Fixture::new(true);
    f.enroll("main", Kind::Branch);
    let command = |selection| Command::Prepare {
        source: key("main"),
        owner: key("owner"),
        expected_revision: 1,
        selection,
    };
    let nodes = Racing {
        nodes: &f.nodes,
        root: f.root(),
        minimum: f.namespace.checkpoint(),
        command: command(f.new.clone()),
        fired: Cell::new(false),
    };
    assert_eq!(
        f.namespace
            .apply(key("stale"), command(f.old.clone()), &nodes, &limits()),
        Err(Error::Journal(super::super::Error::Stale))
    );
    assert!(nodes.fired.get());
    let state = f.namespace.source(&key("main")).unwrap().unwrap();
    assert_eq!(state.pending.unwrap().selection, f.new);
    assert_eq!(state.revision, 2);
    drop(f.namespace.fence(&f.a).unwrap());
}
#[test]
fn duplicate_handles_do_not_keep_a_dropped_subject_fence_live() {
    let f = Fixture::new(true);
    let fence = f.namespace.fence(&f.a).unwrap();
    let inherited = fence._locks.0[0].try_clone().unwrap();
    drop(fence);
    let _again = f.namespace.fence(&f.a).unwrap();
    drop(inherited);
}
#[test]
fn protection_projection_cannot_reclassify_a_guarded_source() {
    use crate::protection::{self, Format, Gap, Journal, Location, Profile};
    let mut f = Fixture::new(true);
    f.enroll("main", Kind::Branch);
    f.selected("main", f.old.clone());
    let mut profile = Profile {
        authority: Digest::blob(b"profile-authority"),
        generation: 1,
        kinds: [Kind::Branch, Kind::Draft, Kind::Job, Kind::Retention]
            .into_iter()
            .collect(),
        unmanaged_boundary: Some(Digest::blob(b"admitted-boundary")),
        journals: vec![Journal {
            identity: f.namespace.identity().clone(),
            sources: vec![protection::Source {
                key: key("main"),
                owner: key("owner"),
                kind: Kind::Job,
                format: Format::FlatBindings,
                places: policy(true).classes[&Kind::Branch].clone(),
            }],
        }],
        limits: limits(),
    };
    let locations = [Location {
        root: f.root(),
        minimum: f.namespace.checkpoint(),
    }];
    let bad = protection::capture(&profile, &locations, &f.nodes).unwrap();
    assert!(
        bad.summary()
            .gap_samples
            .iter()
            .any(|gap| matches!(gap, Gap::ConflictingKind { .. }))
    );
    assert!(!bad.summary().observed_scope_resolved);
    profile.journals[0].sources[0].kind = Kind::Branch;
    let good = protection::capture(&profile, &locations, &f.nodes).unwrap();
    assert!(good.summary().observed_scope_resolved);
}
#[test]
fn replaced_marker_and_missing_anchor_refuse_without_repair() {
    let mut f = Fixture::new(true);
    let marker = f.root().join("claims.json");
    let raw = fs::read(&marker).unwrap();
    let mut changed: serde_json::Value = serde_json::from_slice(&raw).unwrap();
    changed["epoch"] = "changed".into();
    fs::write(
        &marker,
        tig_core::identity::canonical_json(&changed).unwrap(),
    )
    .unwrap();
    assert!(f.namespace.fence(&f.a).is_err());
    assert!(
        f.namespace
            .apply(
                key("none"),
                Command::Enroll {
                    source: key("main"),
                    owner: key("owner"),
                    class: Kind::Branch
                },
                &f.nodes,
                &limits()
            )
            .is_err()
    );
    fs::write(&marker, raw).unwrap();
    let anchor = f
        .root()
        .join("admission-fences")
        .join(format!("{:02x}", partition(&f.a.sha256).unwrap()));
    fs::remove_file(&anchor).unwrap();
    assert!(f.namespace.fence(&f.a).is_err());
    assert!(!anchor.exists());
}
#[test]
fn invalid_policy_never_initializes_and_epoch_identity_is_distinct() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("must-not-create");
    let mut bad = policy(true);
    bad.classes.remove(&Kind::Job);
    assert!(Namespace::create(&path, key("project"), key("epoch"), bad).is_err());
    assert!(!path.exists());
    let f = Fixture::new(true);
    let other = Fixture::new(true);
    assert_ne!(f.namespace.identity(), other.namespace.identity());
}

#[derive(Serialize, Deserialize)]
struct ChildInput {
    minimum: Checkpoint,
    command: Command,
    object: Object,
}
struct Child(std::process::Child);
impl Drop for Child {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
fn spawn(root: &Path, mode: &str) -> Child {
    Child(
        Process::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "source_claims::admission::tests::admission_child",
                "--nocapture",
            ])
            .env("TIG_ADMISSION_CHILD_ROOT", root)
            .env("TIG_ADMISSION_CHILD_MODE", mode)
            .spawn()
            .unwrap(),
    )
}
fn child_input(f: &Fixture) -> Command {
    let command = Command::Prepare {
        source: key("main"),
        owner: key("owner"),
        expected_revision: 1,
        selection: f.old.clone(),
    };
    fs::write(
        f.root().join("child-input.json"),
        serde_json::to_vec(&ChildInput {
            minimum: f.namespace.checkpoint(),
            command: command.clone(),
            object: f.a.clone(),
        })
        .unwrap(),
    )
    .unwrap();
    command
}
#[test]
fn admission_child() {
    let Some(root) = std::env::var_os("TIG_ADMISSION_CHILD_ROOT") else {
        return;
    };
    let root = PathBuf::from(root);
    let input: ChildInput =
        serde_json::from_slice(&fs::read(root.join("child-input.json")).unwrap()).unwrap();
    let mut namespace = Namespace::open(&root, &input.minimum).unwrap();
    let mode = std::env::var("TIG_ADMISSION_CHILD_MODE").unwrap();
    if mode == "hold" {
        let _fence = namespace.fence(&input.object).unwrap();
        fs::write(root.join("child-ready"), b"ready").unwrap();
        // Only the owned test parent terminates this helper. Deadline bounds a
        // lost parent's lifetime; expiry itself grants no mutation authority.
        let start = Instant::now();
        while start.elapsed().as_secs() < 20 {
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        panic!("test parent did not terminate helper");
    }
    let mut nodes = MemoryNodes::default();
    tree(
        &mut nodes,
        &[
            ("changed", Object::from_bytes(b"old")),
            ("unchanged", Object::from_bytes(b"common")),
        ],
    );
    let stop = match mode.as_str() {
        "before" => Stage::BeforeCommit,
        "after" => Stage::AfterCommit,
        _ => panic!("unknown child mode"),
    };
    namespace
        .apply_hook(
            key("child-prepare"),
            input.command,
            &nodes,
            &limits(),
            |stage| {
                if stage == stop {
                    std::process::exit(75);
                }
                Ok(())
            },
        )
        .unwrap();
    panic!("child did not reach injected boundary");
}
#[test]
fn process_exit_before_and_after_commit_recovers_exact_receipt_and_releases_locks() {
    for mode in ["before", "after"] {
        let mut f = Fixture::new(true);
        f.enroll("main", Kind::Branch);
        let command = child_input(&f);
        let minimum = f.namespace.checkpoint();
        let mut child = spawn(&f.root(), mode);
        assert_eq!(child.0.wait().unwrap().code(), Some(75));
        let mut reopened = Namespace::open(&f.root(), &minimum).unwrap();
        let state = reopened.source(&key("main")).unwrap().unwrap();
        assert_eq!(state.pending.is_some(), mode == "after");
        // OS process exit releases admission locks even without Rust Drop.
        drop(reopened.fence(&f.a).unwrap());
        let receipt = reopened
            .apply(key("child-prepare"), command.clone(), &f.nodes, &limits())
            .unwrap();
        assert_eq!(receipt.replayed, mode == "after");
        assert_eq!(reopened.source(&key("main")).unwrap().unwrap().revision, 2);
        let saved = reopened.checkpoint();
        drop(reopened);
        let mut restarted = Namespace::open(&f.root(), &saved).unwrap();
        let replay = restarted
            .apply(key("child-prepare"), command, &Missing, &limits())
            .unwrap();
        assert!(replay.replayed);
        assert_eq!(replay.checkpoint, receipt.checkpoint);
        assert_eq!(restarted.checkpoint(), saved);
    }
}
#[test]
fn another_process_blocks_overlapping_admission_until_actual_exit() {
    let mut f = Fixture::new(true);
    f.enroll("main", Kind::Branch);
    let command = child_input(&f);
    let mut child = spawn(&f.root(), "hold");
    let start = Instant::now();
    while !f.root().join("child-ready").exists() {
        assert!(child.0.try_wait().unwrap().is_none(), "helper exited early");
        assert!(
            start.elapsed().as_secs() < 10,
            "helper failed to acquire fence"
        );
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    let before = f.namespace.checkpoint();
    assert_eq!(
        f.namespace
            .apply(key("overlap"), command.clone(), &f.nodes, &limits()),
        Err(Error::Busy)
    );
    assert_eq!(f.namespace.checkpoint(), before);
    // Registry-only enrollment takes no object partition and still progresses.
    f.enroll("unrelated", Kind::Draft);
    assert!(child.0.try_wait().unwrap().is_none());
    child.0.kill().unwrap();
    child.0.wait().unwrap();
    f.namespace
        .apply(key("overlap"), command, &f.nodes, &limits())
        .unwrap();
    assert!(
        f.namespace
            .source(&key("main"))
            .unwrap()
            .unwrap()
            .pending
            .is_some()
    );
}
