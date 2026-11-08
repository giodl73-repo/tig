//! Mandatory guarded source mutation and live subject-scoped coordination.
//! A scoped absence fact is not independent custody or removal authority.
use super::{
    Checkpoint, Command, Kind, Places, PreviewDisposition, Receipt, Source, SourceJournal, Stage,
};
use crate::{
    Key, local,
    protection::{BudgetNodes, Limits},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    path::Path,
    time::Instant,
};
use tig_core::{
    Digest, Object,
    tree::{NodeStore, Tree, TreeError, TreeLimits},
};

pub const PARTITIONS: usize = 256;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Journal(super::Error),
    InvalidPolicy,
    Budget,
    Busy,
    WrongAuthority,
    IncompleteClosure,
    ConflictingSize,
    Io,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}
impl From<super::Error> for Error {
    fn from(e: super::Error) -> Self {
        Self::Journal(e)
    }
}
impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::Io
    }
}
impl From<crate::Error> for Error {
    fn from(_: crate::Error) -> Self {
        Self::Io
    }
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    /// Active branch/draft/job demands suppress inactive history proximity.
    /// Explicit retention-class demands are always additive.
    pub prefer_active: bool,
    pub classes: BTreeMap<Kind, Places>,
}
impl Policy {
    fn normalized(&self) -> Result<Self> {
        if self.classes.len() != 4
            || [Kind::Branch, Kind::Draft, Kind::Job, Kind::Retention]
                .iter()
                .any(|k| !self.classes.contains_key(k))
        {
            return Err(Error::InvalidPolicy);
        }
        for p in self.classes.values() {
            for places in [&p.current, &p.pending, &p.predecessor] {
                if places.is_empty() || places.len() > 16 {
                    return Err(Error::InvalidPolicy);
                }
            }
        }
        let mut policy = self.clone();
        for p in policy.classes.values_mut() {
            for places in [&mut p.current, &mut p.pending, &mut p.predecessor] {
                places.sort_by(|a, b| a.as_str().cmp(b.as_str()));
                if places.windows(2).any(|p| p[0] == p[1]) {
                    return Err(Error::InvalidPolicy);
                }
            }
        }
        Ok(policy)
    }
    pub(super) fn valid(&self) -> bool {
        self.normalized().is_ok_and(|p| p == *self)
    }
}

pub(super) fn create_anchors(root: &Path) -> super::Result<()> {
    fs::create_dir(root.join("admission-fences"))?;
    for partition in 0..PARTITIONS {
        OpenOptions::new().write(true).create_new(true).open(
            root.join("admission-fences")
                .join(format!("{partition:02x}")),
        )?;
    }
    Ok(())
}
struct Locks(Vec<File>);
impl Drop for Locks {
    fn drop(&mut self) {
        for file in &self.0 {
            let _ = file.unlock();
        }
    }
}
fn partition(object: &Digest) -> Result<usize> {
    usize::from_str_radix(&object.as_str()[..2], 16).map_err(|_| Error::WrongAuthority)
}
fn locks(root: &Path, parts: &BTreeSet<usize>, exclusive: bool) -> Result<Locks> {
    let directory = root.join("admission-fences");
    local::directory(&directory)?;
    let mut held = Locks(Vec::new());
    for part in parts {
        if *part >= PARTITIONS {
            return Err(Error::WrongAuthority);
        }
        let path = directory.join(format!("{part:02x}"));
        local::regular(&path)?;
        let file = OpenOptions::new().read(true).write(true).open(&path)?;
        if file.metadata()?.len() != 0 {
            return Err(Error::WrongAuthority);
        }
        let result = if exclusive {
            file.try_lock()
        } else {
            file.try_lock_shared()
        };
        result.map_err(|e| match e {
            fs::TryLockError::WouldBlock => Error::Busy,
            fs::TryLockError::Error(_) => Error::Io,
        })?;
        held.0.push(file);
    }
    Ok(held)
}
/// Only this module constructs a permit, after resolving exact before/after roots.
pub(super) struct Permit {
    authority: Digest,
    request: Digest,
    _locks: Locks,
}
impl Permit {
    pub(super) fn matches(
        &self,
        authority: &Digest,
        request: &super::Request,
    ) -> super::Result<bool> {
        Ok(authority == &self.authority && Digest::blob(&super::encode(request)?) == self.request)
    }
}
pub struct SubjectFence {
    authority: Digest,
    object: Object,
    _locks: Locks,
}
impl SubjectFence {
    pub fn authority(&self) -> &Digest {
        &self.authority
    }
    pub fn object(&self) -> &Object {
        &self.object
    }
}
/// One predicate under a live namespace fence. Not Clone/Serialize and not a
/// deletion grant; provider scope, custody and retirement authority are separate.
pub struct ScopedAbsence<'a> {
    fence: &'a SubjectFence,
    place: Key,
    checkpoint: Checkpoint,
}
impl ScopedAbsence<'_> {
    pub fn object(&self) -> &Object {
        &self.fence.object
    }
    pub fn place(&self) -> &Key {
        &self.place
    }
    pub fn authority(&self) -> &Digest {
        &self.fence.authority
    }
    pub fn checkpoint(&self) -> &Checkpoint {
        &self.checkpoint
    }
    pub fn authorizes_removal(&self) -> bool {
        false
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Demand {
    bytes: u64,
    active: BTreeSet<String>,
    history: BTreeSet<String>,
    retained: BTreeSet<String>,
}
impl Demand {
    fn extend(&mut self, other: &Self) -> Result<()> {
        if self.bytes != other.bytes {
            return Err(Error::ConflictingSize);
        }
        self.active.extend(other.active.iter().cloned());
        self.history.extend(other.history.iter().cloned());
        self.retained.extend(other.retained.iter().cloned());
        Ok(())
    }
    fn required(&self, policy: &Policy, place: &str) -> bool {
        self.retained.contains(place)
            || self.active.contains(place)
            || ((!policy.prefer_active || self.active.is_empty()) && self.history.contains(place))
    }
}
struct Work<'a, S> {
    nodes: BudgetNodes<'a, S>,
    limits: &'a Limits,
    roots: usize,
    bindings: usize,
    associations: usize,
}
impl<'a, S: NodeStore> Work<'a, S> {
    fn new(nodes: &'a S, limits: &'a Limits, start: Instant) -> Result<Self> {
        limits.validate().map_err(|_| Error::Budget)?;
        Ok(Self {
            nodes: BudgetNodes::new(nodes, limits, start),
            limits,
            roots: 0,
            bindings: 0,
            associations: 0,
        })
    }
    fn timed(&self) -> Result<()> {
        self.nodes.timed().map_err(|_| Error::Budget)
    }
    fn source(&mut self, source: &Source, policy: &Policy) -> Result<BTreeMap<Digest, Demand>> {
        let kind = source.kind.as_ref().ok_or(Error::WrongAuthority)?;
        let places = policy.classes.get(kind).ok_or(Error::InvalidPolicy)?;
        let mut roots = Vec::new();
        if let Some(current) = &source.current {
            roots.push((&current.selection, &places.current, false));
        }
        if let Some(pending) = &source.pending {
            roots.push((&pending.selection, &places.pending, false));
        }
        for old in &source.predecessors {
            roots.push((&old.selection, &places.predecessor, true));
        }
        let mut result = BTreeMap::<Digest, Demand>::new();
        for (selection, places, history) in roots {
            self.timed()?;
            self.roots += 1;
            if self.roots > self.limits.roots {
                return Err(Error::Budget);
            }
            let limits = TreeLimits {
                max_entries: self.limits.bindings - self.bindings,
                max_reads: self.limits.reads,
                max_read_bytes: self.limits.read_bytes,
                ..TreeLimits::default()
            };
            let (entries, _) = Tree::from_root(selection.root.clone())
                .entries(&mut self.nodes, limits)
                .map_err(|e| {
                    if e == TreeError::Budget {
                        Error::Budget
                    } else {
                        Error::IncompleteClosure
                    }
                })?;
            self.bindings += entries.len();
            self.associations = self
                .associations
                .checked_add(
                    entries
                        .len()
                        .checked_mul(places.len())
                        .ok_or(Error::Budget)?,
                )
                .ok_or(Error::Budget)?;
            if self.associations > self.limits.associations {
                return Err(Error::Budget);
            }
            for object in entries.values() {
                let demand = result
                    .entry(object.sha256.clone())
                    .or_insert_with(|| Demand {
                        bytes: object.bytes,
                        ..Demand::default()
                    });
                if demand.bytes != object.bytes {
                    return Err(Error::ConflictingSize);
                }
                let bucket = if *kind == Kind::Retention {
                    &mut demand.retained
                } else if history {
                    &mut demand.history
                } else {
                    &mut demand.active
                };
                bucket.extend(places.iter().map(|p| p.as_str().to_owned()));
            }
        }
        self.timed()?;
        Ok(result)
    }
}

pub struct Namespace {
    journal: SourceJournal,
}
impl Namespace {
    pub fn create(root: &Path, project: Key, epoch: Key, policy: Policy) -> Result<Self> {
        let policy = policy.normalized()?;
        Ok(Self {
            journal: SourceJournal::create_mode(root, project, epoch, Some(policy))?,
        })
    }
    pub fn open(root: &Path, minimum: &Checkpoint) -> Result<Self> {
        let journal = SourceJournal::open(root, minimum)?;
        if journal.config.admission.is_none() {
            return Err(Error::WrongAuthority);
        }
        journal.check_guarded_marker()?;
        Ok(Self { journal })
    }
    pub fn checkpoint(&self) -> Checkpoint {
        self.journal.checkpoint()
    }
    pub fn identity(&self) -> &Digest {
        self.journal.identity()
    }
    pub fn source(&mut self, key: &Key) -> Result<Option<Source>> {
        self.journal.check_guarded_marker()?;
        Ok(self.journal.source(key)?)
    }
    /// Fixed policy changes require explicit creation/adoption of another epoch.
    pub fn policy(&self) -> &Policy {
        self.journal
            .config
            .admission
            .as_ref()
            .expect("guarded namespace")
    }
    pub fn apply(
        &mut self,
        operation: Key,
        command: Command,
        nodes: &impl NodeStore,
        limits: &Limits,
    ) -> Result<Receipt> {
        self.apply_hook(operation, command, nodes, limits, |_| Ok(()))
    }
    fn apply_hook(
        &mut self,
        operation: Key,
        command: Command,
        nodes: &impl NodeStore,
        limits: &Limits,
        hook: impl FnMut(Stage) -> super::Result<()>,
    ) -> Result<Receipt> {
        let start = Instant::now();
        limits.validate().map_err(|_| Error::Budget)?;
        self.journal.check_guarded_marker()?;
        self.journal.sync()?;
        let preview = SourceJournal::preview_mode(
            &self.journal.root,
            &self.journal.checkpoint(),
            operation.clone(),
            command.clone(),
            true,
        )?;
        let (before, after) = match preview.disposition {
            PreviewDisposition::Recorded { receipt } => return Ok(receipt),
            PreviewDisposition::Proposed { before, after } => (before, after),
        };
        let mut work = Work::new(nodes, limits, start)?;
        let policy = self.policy();
        let before = match &before {
            Some(source) => work.source(source, policy)?,
            None => BTreeMap::new(),
        };
        let after = work.source(&after, policy)?;
        let mut parts = BTreeSet::new();
        for id in before.keys().chain(after.keys()) {
            if before.get(id) != after.get(id) {
                parts.insert(partition(id)?);
            }
        }
        work.timed()?;
        let held = locks(&self.journal.root, &parts, false)?;
        work.timed()?;
        self.journal.check_guarded_marker()?;
        let request = super::Request {
            operation: operation.clone(),
            command: command.clone(),
        };
        let permit = Permit {
            authority: self.identity().clone(),
            request: Digest::blob(&super::encode(&request)?),
            _locks: held,
        };
        Ok(self
            .journal
            .apply_admitted(operation, command, &permit, hook)?)
    }
    /// Mutual exclusion only. This neither initializes a byte store nor deletes.
    pub fn fence(&self, object: &Object) -> Result<SubjectFence> {
        self.journal.check_guarded_marker()?;
        let held = locks(
            &self.journal.root,
            &[partition(&object.sha256)?].into_iter().collect(),
            true,
        )?;
        self.journal.check_guarded_marker()?;
        Ok(SubjectFence {
            authority: self.identity().clone(),
            object: object.clone(),
            _locks: held,
        })
    }
    /// No global head recheck: mandatory mutation admission keeps this subject's
    /// predicate stable under the held exclusive fence while other heads advance.
    pub fn absence<'a>(
        &mut self,
        fence: &'a SubjectFence,
        place: &Key,
        nodes: &impl NodeStore,
        limits: &Limits,
    ) -> Result<Option<ScopedAbsence<'a>>> {
        let start = Instant::now();
        self.journal.check_guarded_marker()?;
        if fence.authority != *self.identity() {
            return Err(Error::WrongAuthority);
        }
        if !self.policy().classes.values().any(|p| {
            p.current
                .iter()
                .chain(&p.pending)
                .chain(&p.predecessor)
                .any(|k| k == place)
        }) {
            return Err(Error::InvalidPolicy);
        }
        self.journal.sync()?;
        let state = self.journal.state.clone();
        let policy = self.policy();
        let mut work = Work::new(nodes, limits, start)?;
        let mut subject = Demand {
            bytes: fence.object.bytes,
            ..Demand::default()
        };
        for source in state.sources.values() {
            let demands = work.source(source, policy)?;
            if let Some(demand) = demands.get(&fence.object.sha256) {
                subject.extend(demand)?;
            }
        }
        work.timed()?;
        self.journal.check_guarded_marker()?;
        if subject.required(policy, place.as_str()) {
            return Ok(None);
        }
        Ok(Some(ScopedAbsence {
            fence,
            place: place.clone(),
            checkpoint: state.head,
        }))
    }
}
#[cfg(test)]
mod tests;
