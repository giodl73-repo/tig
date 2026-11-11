//! Bounded transient overlay; persists only the final metadata closure.
use super::*;
use std::collections::BTreeSet;

/// Logical work includes discarded intermediate nodes. Published counts are
/// provider calls, not allocation, durability, custody or root adoption.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StagedEditMetrics {
    pub tree: TreeMetrics,
    pub published_nodes: usize,
    pub published_bytes: usize,
}

struct Overlay<'a, S> {
    base: &'a mut S,
    staged: BTreeMap<Digest, Vec<u8>>,
}

impl<S: NodeStore> NodeStore for Overlay<'_, S> {
    fn get(&self, object: &Object) -> TreeResult<Option<Vec<u8>>> {
        match self.staged.get(&object.sha256) {
            Some(bytes) => Ok(Some(bytes.clone())),
            None => self.base.get(object),
        }
    }

    fn put(&mut self, bytes: &[u8]) -> TreeResult<Object> {
        let object = Object::from_bytes(bytes);
        if let Some(previous) = self.staged.get(&object.sha256) {
            if previous != bytes {
                return Err(TreeError::Integrity);
            }
        } else {
            // Context::write checks cumulative writes/bytes before every call.
            self.staged.insert(object.sha256.clone(), bytes.to_vec());
        }
        Ok(object)
    }
}

#[derive(Clone, Copy)]
enum Kind {
    Directory,
    Map,
}

impl<S: NodeStore> Overlay<'_, S> {
    fn publish(&mut self, root: &Object, tree: TreeMetrics) -> TreeResult<StagedEditMetrics> {
        let mut pending = vec![(root.clone(), Kind::Directory, false)];
        let mut seen = BTreeSet::new();
        let mut order = Vec::new();
        while let Some((object, kind, expanded)) = pending.pop() {
            let Some(bytes) = self.staged.get(&object.sha256) else {
                // An unchanged subtree: neither read nor enumerate the base.
                continue;
            };
            if expanded {
                order.push(object);
                continue;
            }
            if !seen.insert(object.sha256.clone()) {
                continue;
            }
            if Object::from_bytes(bytes) != object {
                return Err(TreeError::Integrity);
            }
            pending.push((object, kind, true));
            match kind {
                Kind::Directory => {
                    let dir: Directory =
                        serde_json::from_slice(bytes).map_err(|_| TreeError::InvalidNode)?;
                    if let Some(link) = dir.root {
                        pending.push((link.object, Kind::Map, false));
                    }
                }
                Kind::Map => {
                    let node: MapNode =
                        serde_json::from_slice(bytes).map_err(|_| TreeError::InvalidNode)?;
                    for link in [node.left, node.right].into_iter().flatten() {
                        pending.push((link.object, Kind::Map, false));
                    }
                    if let Entry::Directory { object } = node.value {
                        pending.push((object, Kind::Directory, false));
                    }
                }
            }
        }
        let mut metrics = StagedEditMetrics {
            tree,
            ..StagedEditMetrics::default()
        };
        for object in order {
            let bytes = &self.staged[&object.sha256];
            if self.base.put(bytes)? != object {
                return Err(TreeError::Integrity);
            }
            metrics.published_nodes += 1;
            metrics.published_bytes += bytes.len();
        }
        Ok(metrics)
    }
}

impl Tree {
    /// Run `edit` within its original cumulative limits, then persist each
    /// reachable staged metadata node once. Unchanged base subtrees and file
    /// values are opaque. No provider puts occur on a staging error.
    ///
    /// A flush failure can leave immutable children; it returns no new root.
    /// Caller owns physical admission, finish/readback and atomic root adoption.
    pub fn edit_staged(
        &self,
        expected_root: &Object,
        changes: &BTreeMap<LogicalPath, Option<Object>>,
        store: &mut impl NodeStore,
        limits: TreeLimits,
    ) -> TreeResult<(Self, StagedEditMetrics)> {
        let mut overlay = Overlay {
            base: store,
            staged: BTreeMap::new(),
        };
        let (next, tree) = self.edit(expected_root, changes, &mut overlay, limits)?;
        let metrics = overlay.publish(next.root(), tree)?;
        Ok((next, metrics))
    }
}
