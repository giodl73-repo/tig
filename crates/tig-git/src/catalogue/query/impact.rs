//! Reverse traversal of the existing admitted graph, with exact revision witnesses.
use super::*;
use std::collections::VecDeque;

impl Catalogue {
    pub(super) fn impact_query(
        &mut self,
        project: &str,
        id: &str,
        kinds: &BTreeSet<graph::EdgeKind>,
        offset: usize,
    ) -> Result<Value> {
        let p = self.projects.get_mut(project).ok_or("unknown_project")?;
        let target = p
            .snapshot
            .resolve_id(id)
            .map_err(|_| "unknown_asset")?
            .reference()
            .clone();
        let mut missing = BTreeMap::<graph::Reference, usize>::new();
        for asset in p.snapshot.selected_records() {
            if !p.enrollment.graph_profiles.iter().any(|profile| {
                profile.definition_id == asset.definition().id
                    && profile.definition == asset.definition().object
            }) {
                *missing.entry(asset.definition().clone()).or_default() += 1;
            }
        }
        let roots = p
            .snapshot
            .selected_records()
            .map(|a| a.reference().clone())
            .collect::<BTreeSet<_>>();
        if roots.len() > 10_000 {
            return Err("graph_budget_exceeded".into());
        }
        let query = graph::Query {
            checkpoint: p.snapshot.checkpoint().clone(),
            roots,
            kinds: kinds.clone(),
            limits: graph::Limits {
                nodes: 10_000,
                edges: 40_000,
                depth: 64,
                bytes: 16 * 1024 * 1024,
            },
        };
        let g = graph::resolve(&mut p.snapshot, &query, &p.enrollment.graph_profiles)
            .map_err(|_| "graph_refused")?;
        let mut reverse = BTreeMap::<&graph::Reference, Vec<&graph::Edge>>::new();
        for e in g.edges() {
            reverse.entry(&e.target).or_default().push(e);
        }
        let mut distances = BTreeMap::from([(target.clone(), 0usize)]);
        let mut witness = BTreeMap::new();
        let mut pending = VecDeque::from([target.clone()]);
        let mut depth_limited = false;
        while let Some(current) = pending.pop_front() {
            let distance = distances[&current];
            for edge in reverse.get(&current).into_iter().flatten() {
                if distances.contains_key(&edge.from) {
                    continue;
                }
                if distance >= 64 {
                    depth_limited = true;
                    continue;
                }
                distances.insert(edge.from.clone(), distance + 1);
                witness.insert(edge.from.clone(), *edge);
                pending.push_back(edge.from.clone());
            }
        }
        let mut ordered: Vec<_> = distances.iter().filter(|(r, _)| **r != target).collect();
        ordered.sort_by(|(a, da), (b, db)| da.cmp(db).then_with(|| a.cmp(b)));
        let items=ordered.iter().skip(offset).take(100).map(|(r,d)|{
            let mut path=Vec::new();let mut current=(*r).clone();
            while let Some(edge)=witness.get(&current){path.push(*edge);current=edge.target.clone();if current==target{break;}}
            json!({"asset":r,"distance":d,"class":g.nodes().get(*r).map(|n|n.class),"path":path})
        }).collect::<Vec<_>>();
        Ok(
            json!({"target":target,"kinds":kinds,"checkpoint":g.checkpoint(),"admission_sha256":g.admission(),
            "binding_sha256":g.binding_digest(),"registered_roots":query.roots.len(),"nodes":g.nodes().len(),"edges":g.edges().len(),
            "total_known":ordered.len(),"items":items,"gaps":g.gaps().iter().take(100).collect::<Vec<_>>(),"gap_count":g.gaps().len(),
            "unadmitted_definitions":missing.iter().take(100).map(|(definition,count)|json!({"definition":definition,"registered_records":count})).collect::<Vec<_>>(),"unadmitted_definition_count":missing.len(),
            "gaps_truncated":g.gaps().len()>100,"impact_depth_limited":depth_limited,
            "complete_declared_coverage":g.complete_declared_coverage()&&!depth_limited,
            "semantics":"Potential consumers of this exact selected record revision within admitted edge kinds; paths explain reachability, not that a render or publication is necessarily stale. Frozen older revisions are distinct. Cross-project and unindexed consumers remain unknown."}),
        )
    }
}
