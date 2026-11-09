use super::*;
use std::cell::Cell;
use tig_core::tree::{NodeStore, TreeResult};

#[test]
fn archive_acquisition_and_binding_share_one_read_budget_before_any_write() {
    struct Counted<'a> {
        nodes: &'a mut MemoryNodes,
        reads: Cell<usize>,
        writes: usize,
    }
    impl NodeStore for Counted<'_> {
        fn get(&self, o: &Object) -> TreeResult<Option<Vec<u8>>> {
            self.reads.set(self.reads.get() + 1);
            self.nodes.get(o)
        }
        fn put(&mut self, b: &[u8]) -> TreeResult<Object> {
            self.writes += 1;
            self.nodes.put(b)
        }
    }
    let (f, snapshot) = fixture();
    let mut nodes = MemoryNodes::default();
    let archive = snapshot
        .archive_metadata(&mut nodes, TreeLimits::default())
        .unwrap();
    let mut counted = Counted {
        nodes: &mut nodes,
        reads: Cell::new(0),
        writes: 0,
    };
    let limits = BindingLimits {
        reads: 1,
        ..BindingLimits::default()
    };
    assert!(matches!(
        retirement::build_from_archive(
            &archive,
            roots(&snapshot),
            profiles(&f),
            &mut counted,
            limits
        ),
        Err(retirement::Error::Budget)
    ));
    assert_eq!(counted.reads.get(), 1);
    assert_eq!(counted.writes, 0);
}

#[test]
fn one_thousand_native_assets_keep_independent_bindings_and_oversize_category_selection_refuses() {
    let f = Fixture::new();
    for n in 0..1000 {
        f.record(
            &format!("leaf-{n}"),
            &format!("asset/{n}"),
            true,
            json!([]),
            Some(b"shared"),
        );
    }
    let mut snapshot = capture(&reader(f.root()), commit(f.root()));
    let all = roots(&snapshot);
    let mut nodes = MemoryNodes::default();
    let root = retirement::build(
        &mut snapshot,
        all,
        profiles(&f),
        &mut nodes,
        BindingLimits::default(),
    )
    .unwrap();
    let inventory =
        retirement::inspect(&root, &root.context, &mut nodes, BindingLimits::default()).unwrap();
    assert_eq!(inventory.assets(), 1000);
    let definitions =
        BTreeSet::from([snapshot.resolve_id("asset/0").unwrap().definition().clone()]);
    let categories = BTreeMap::from([("drawing".into(), definitions)]);
    let all = Filter {
        assets: BTreeSet::new(),
        categories: BTreeSet::from(["drawing".into()]),
    };
    assert!(matches!(
        retirement::select(&inventory, &categories, &all),
        Err(retirement::Error::Budget)
    ));
    let block = Filter {
        assets: (0..10)
            .map(|n| {
                snapshot
                    .resolve_id(&format!("asset/{n}"))
                    .unwrap()
                    .reference()
                    .clone()
            })
            .collect(),
        categories: all.categories,
    };
    let preview = retirement::select(&inventory, &categories, &block).unwrap();
    assert_eq!(preview.assets.len(), 10);
    assert_eq!(preview.paths.len(), 40);
    assert_eq!(
        preview
            .objects
            .iter()
            .filter(|o| **o == Object::from_bytes(b"shared"))
            .count(),
        1
    );
}

#[test]
fn conflicting_payload_sizes_across_separate_native_queries_refuse_before_metadata_publication() {
    let f = Fixture::new();
    f.record("a", "asset/a", true, json!([]), Some(b"shared"));
    let raw = json!({"schema":"tig.instance.design-1","id":"asset/b","definition":f.primary,
        "authored":{"deps":[],"text":"synthetic"},"streams":[],"selected":{"sha256":Object::from_bytes(b"shared").sha256,"bytes":7},"status":{},"metadata":{}});
    write(f.root(), "records/b.tig", &raw);
    let mut snapshot = capture(&reader(f.root()), commit(f.root()));
    let all = roots(&snapshot);
    let mut nodes = MemoryNodes::default();
    assert!(matches!(
        retirement::build(
            &mut snapshot,
            all,
            profiles(&f),
            &mut nodes,
            BindingLimits::default()
        ),
        Err(retirement::Error::Integrity)
    ));
    assert_eq!(nodes.objects().count(), 0);
}
