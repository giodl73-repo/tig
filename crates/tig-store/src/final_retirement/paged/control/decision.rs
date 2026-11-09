use super::*;
use crate::local::LocalStore;

fn copy_key(c: &bulk::Copy) -> (&Digest, &Digest) {
    (&c.store, &c.publication.object.sha256)
}
pub fn copies(
    mut copies: Vec<bulk::Copy>,
    nodes: &mut impl NodeStore,
    limits: &Limits,
) -> Result<Object> {
    if copies.len() > 65536 {
        return Err(Error::Budget);
    }
    copies.sort_by(|a, b| copy_key(a).cmp(&copy_key(b)));
    if copies
        .windows(2)
        .any(|c| copy_key(&c[0]) >= copy_key(&c[1]))
    {
        return Err(Error::Invalid);
    }
    let mut work = Work::new(nodes, limits)?;
    let mut index = BTreeMap::new();
    for (i, c) in copies.iter().enumerate() {
        index.insert(slot(i)?, work.doc(c)?);
    }
    let root = work.tree(&index, 65536)?;
    Ok(work.doc(&CopyIndex {
        schema: "tig.paged-control-copies.design-1".into(),
        root,
        count: copies.len(),
    })?)
}
fn input(work: &mut Work<'_, impl NodeStore>, root: &Object) -> Result<Vec<bulk::Copy>> {
    let h: CopyIndex = work.read(root)?;
    if h.schema != "tig.paged-control-copies.design-1" || h.count > 65536 {
        return Err(Error::Invalid);
    }
    let rows = work.entries(&h.root, 65536)?;
    if rows.len() != h.count {
        return Err(Error::Invalid);
    }
    let mut copies = Vec::new();
    for (i, (key, o)) in rows.into_iter().enumerate() {
        if key != slot(i)? {
            return Err(Error::Invalid);
        }
        copies.push(work.read::<bulk::Copy>(&o)?);
    }
    if copies
        .windows(2)
        .any(|c| copy_key(&c[0]) >= copy_key(&c[1]))
    {
        return Err(Error::Invalid);
    }
    Ok(copies)
}
fn anchor(e: &Enrollment, r: &Request, prepared: &Object) -> Result<Digest> {
    hash(
        &serde_json::json!({"schema":"tig.paged-control-action-basis.design-1","enrollment":e.id()?,
        "request":r,"prepared":prepared}),
    )
}
fn action(
    e: &Enrollment,
    r: &Request,
    anchor: &Digest,
    c: &bulk::Copy,
) -> Result<provider::Request> {
    let id = hash(
        &serde_json::json!({"schema":"tig.paged-control-copy.design-1","basis":anchor,"copy":c}),
    )?;
    Ok(provider::Request {
        operation: Key::parse(&format!("paged-final-{}", id.as_str()))
            .map_err(|_| Error::Invalid)?,
        owner: e.selection.owner.clone(),
        publication: c.publication.clone(),
        capacity: c.capacity.clone(),
        context: anchor.clone(),
        window: r.window.clone(),
    })
}
/// Freeze the original claim selection, conditional pages, exclusions and actual
/// provider birth/grant plans. This stages metadata only; finish/readback is
/// separate and a candidate grants neither permission.
pub fn plan(
    e: &Enrollment,
    r: Request,
    roster: &claims::Roster,
    namespaces: &mut [Namespace],
    nodes: &mut impl NodeStore,
    stores: &[&LocalStore],
    limits: &Limits,
) -> Result<Decision> {
    if e.age.is_some() {
        return Err(Error::Authority);
    }
    plan_inner(e, r, roster, namespaces, nodes, stores, limits)
}
pub(super) fn plan_inner(
    e: &Enrollment,
    r: Request,
    roster: &claims::Roster,
    namespaces: &mut [Namespace],
    nodes: &mut impl NodeStore,
    stores: &[&LocalStore],
    limits: &Limits,
) -> Result<Decision> {
    e.id()?;
    r.check(e)?;
    if stores.len() > 4
        || stores
            .iter()
            .map(|s| s.identity())
            .collect::<BTreeSet<_>>()
            .len()
            != stores.len()
    {
        return Err(Error::Authority);
    }
    let mut work = Work::new(nodes, limits)?;
    let inventory = super::super::inspect(&e.selection, &r.manifest, &mut work, limits)?;
    let copies = input(&mut work, &r.copies)?;
    let subjects = inventory
        .subjects()
        .map(|s| (s.object.sha256.clone(), s))
        .collect::<BTreeMap<_, _>>();
    for c in &copies {
        if c.publication.owner != e.selection.owner
            || subjects
                .get(&c.publication.object.sha256)
                .is_none_or(|s| s.object != c.publication.object)
        {
            return Err(Error::Invalid);
        }
    }
    let prepared = preparation::prepare(
        &e.selection,
        &r.manifest,
        &e.sources,
        roster,
        namespaces,
        &mut work,
        limits,
    )?;
    let (prepared_header, _) = preparation::read(&e.selection, &prepared, &mut work, limits)?;
    let anchor = anchor(e, &r, &prepared)?;
    let mut subjects_index = BTreeMap::new();
    let mut actions_index = BTreeMap::new();
    for subject in inventory.subjects() {
        let requested = copies
            .iter()
            .filter(|c| c.publication.object == subject.object)
            .collect::<Vec<_>>();
        let mut hold = if subject.held_by_surviving_claims {
            Some(Hold::SurvivingClaims)
        } else if requested.is_empty() {
            Some(Hold::NoCopyScope)
        } else {
            None
        };
        let mut plans = Vec::new();
        if hold.is_none() {
            for c in requested {
                let Some(store) = stores.iter().find(|s| s.identity() == &c.store) else {
                    hold = Some(Hold::ProviderUnavailable);
                    break;
                };
                match provider::plan(store, action(e, &r, &anchor, c)?) {
                    Ok(p) if p.policy.scope == e.selection.scope && p.policy.clock == e.clock => {
                        plans.push(p)
                    }
                    Ok(_)
                    | Err(
                        provider::Error::ProtectedGrant
                        | provider::Error::Invalid
                        | provider::Error::Unsupported,
                    ) => {
                        hold = Some(Hold::ProviderProtection);
                        break;
                    }
                    Err(_) => {
                        hold = Some(Hold::ProviderUnavailable);
                        break;
                    }
                }
            }
        }
        if hold.is_none() {
            for p in plans {
                if actions_index.len() >= 65536 {
                    return Err(Error::Budget);
                }
                let i = actions_index.len();
                actions_index.insert(slot(i)?, work.doc(&p)?);
            }
        }
        subjects_index.insert(
            super::super::path(subject.object.sha256.as_str())?,
            work.doc(&Subject {
                object: subject.object.clone(),
                hold,
            })?,
        );
    }
    // Provider inspection must not turn a prior source vector into a lease.
    for pin in &inventory.header.basis.members {
        let n = namespaces
            .iter_mut()
            .find(|n| n.identity() == &pin.namespace)
            .ok_or(Error::Authority)?;
        n.refresh_minimum(&pin.minimum)?;
        admission::bulk::snapshot(n)?;
        if claims::NamespacePin::current(n)? != *pin {
            return Err(Error::Stale);
        }
    }
    let d = Decision {
        age: None,
        schema: "tig.paged-control-decision.design-1".into(),
        enrollment: e.id()?,
        request: r,
        prepared,
        subjects: work.tree(&subjects_index, limits.objects)?,
        actions: work.tree(&actions_index, 65536)?,
        pages: prepared_header.count,
        bindings: prepared_header.bindings,
        objects: subjects_index.len(),
        action_count: actions_index.len(),
    };
    d.id()?;
    work.timed()?;
    Ok(d)
}
pub(super) fn load(
    e: &Enrollment,
    d: &Decision,
    nodes: &mut impl NodeStore,
    limits: &Limits,
) -> Result<Loaded> {
    e.id()?;
    d.id()?;
    d.request.check(e)?;
    if d.enrollment != e.id()? {
        return Err(Error::Authority);
    }
    if d.age.is_some() != e.age.is_some() {
        return Err(Error::Authority);
    }
    let mut work = Work::new(nodes, limits)?;
    let (prepared, steps) = preparation::read(&e.selection, &d.prepared, &mut work, limits)?;
    if prepared.manifest != d.request.manifest
        || prepared.sources != e.sources
        || prepared.count != d.pages
        || prepared.bindings != d.bindings
    {
        return Err(Error::Invalid);
    }
    let index = work.entries(&d.subjects, limits.objects)?;
    if index.len() != d.objects {
        return Err(Error::Invalid);
    }
    let inv = super::super::inspect(&e.selection, &d.request.manifest, &mut work, limits)?;
    let mut subjects = Vec::new();
    for ((key, o), expected) in index.into_iter().zip(inv.subjects()) {
        let s: Subject = work.read(&o)?;
        if key.as_str() != s.object.sha256.as_str()
            || s.object != expected.object
            || expected.held_by_surviving_claims && s.hold != Some(Hold::SurvivingClaims)
        {
            return Err(Error::Invalid);
        }
        subjects.push(s);
    }
    if subjects.len() != inv.summary().unique_objects {
        return Err(Error::Invalid);
    }
    let index = work.entries(&d.actions, 65536)?;
    if index.len() != d.action_count {
        return Err(Error::Invalid);
    }
    let copies = input(&mut work, &d.request.copies)?;
    let anchor = anchor(e, &d.request, &d.prepared)?;
    let mut actions = Vec::new();
    let mut seen = BTreeSet::new();
    for (i, (key, o)) in index.into_iter().enumerate() {
        let p: provider::Plan = work.read(&o)?;
        p.id()?;
        let c = copies
            .iter()
            .find(|c| {
                &c.store == p.enrollment.store()
                    && c.publication.object == p.request().publication.object
            })
            .ok_or(Error::Invalid)?;
        if key != slot(i)?
            || encode(&action(e, &d.request, &anchor, c)?)? != encode(p.request())?
            || p.policy.scope != e.selection.scope
            || p.policy.clock != e.clock
            || !seen.insert((c.store.clone(), c.publication.object.sha256.clone()))
            || !subjects
                .iter()
                .any(|s| s.object == c.publication.object && s.hold.is_none())
        {
            return Err(Error::Invalid);
        }
        actions.push(p);
    }
    for s in subjects.iter().filter(|s| s.hold.is_none()) {
        let requested = copies
            .iter()
            .filter(|c| c.publication.object == s.object)
            .collect::<Vec<_>>();
        if requested.is_empty()
            || requested
                .iter()
                .any(|c| !seen.contains(&(c.store.clone(), s.object.sha256.clone())))
        {
            return Err(Error::Invalid);
        }
    }
    let loaded = Loaded {
        prepared,
        steps,
        subjects,
        actions,
    };
    if d.age.is_some() {
        aged::read(e, d, &loaded, &mut work, limits)?;
    }
    Ok(loaded)
}

/// Explain the frozen original copy scope and exclusions before approval.
/// This reads metadata only and neither observes current custody nor authenticates
/// an arbitrary supplied header or the caller's provider identity assertions.
pub fn inspect(
    e: &Enrollment,
    d: &Decision,
    nodes: &mut impl NodeStore,
    limits: &Limits,
) -> Result<Inventory> {
    let l = load(e, d, nodes, limits)?;
    Ok(Inventory {
        decision: d.id()?,
        basis: l.prepared.basis,
        subjects: l.subjects,
        actions: l.actions,
    })
}
