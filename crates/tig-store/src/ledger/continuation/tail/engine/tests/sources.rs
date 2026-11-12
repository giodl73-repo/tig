//! Manual synthetic publication exercises admission, not the future controller.
use super::*;

#[test]
fn legacy_preparation_refuses_continued_mode_before_metadata_work() {
    let mut f = fixture();
    let c = activate(&mut f);
    let mut l = reopened(&f, &c, &minimum(&c));
    f.nodes.reset_metrics();
    let result = crate::ledger::continuation::preparation::legacy(
        &mut l,
        key("not-a-legacy-source"),
        &mut f.nodes,
        history::index::build_limits(),
    );
    assert!(matches!(result, Err(Error::Invalid)));
    assert_eq!(f.nodes.metrics().node_gets, 0);
    assert_eq!(f.nodes.metrics().puts, 0);
}

pub(super) fn next(
    f: &mut Fixture,
    current: &ActiveControl,
    l: &mut CapacityLedger,
) -> ActiveControl {
    let m = Minimum {
        checkpoint: l.checkpoint(),
        ..minimum(current)
    };
    let (data, _, _) =
        super::super::super::tests::joined::prepare(f, current, &m, &mut l.conn).unwrap();
    f.data = data;
    let mut c = control(f);
    let operation = key(&format!("rotation-{}", c.preparation.tail.generation));
    c.preparation.operation = operation.clone();
    c.preparation.tail.operation = operation.clone();
    c.seal.intent.operation = operation;
    c.seal.intent.prepared = Object::from_bytes(&c.preparation.to_json().unwrap());
    c.seal.hash = Digest::blob(&c.seal.intent.to_json().unwrap());
    c.validate().unwrap();
    drop(create(f, &c));
    let root = l.root.clone();
    fs::rename(
        f._temp.path().join("tail.sqlite"),
        root.join(c.preparation.tail.relative_name().unwrap()),
    )
    .unwrap();
    activation::seal(l, &c.seal.intent, |_| Ok(())).unwrap();
    fs::write(root.join(c.relative_name().unwrap()), c.to_json().unwrap()).unwrap();
    fs::write(root.join("domain.json"), c.to_json().unwrap()).unwrap();
    c
}

#[test]
fn actual_three_generation_writes_preserve_receipts_inflight_state_and_ancestor_minima() {
    let mut f = fixture();
    let original = f.ledger.reserve(key("job"), key("owner"), 1000).unwrap();
    let c1 = activate(&mut f);
    let mut l1 = reopened(&f, &c1, &minimum(&c1));
    let allocation = reference(&l1, "allocation");
    let first = l1
        .complete_allocation(key("generation1-complete"), &allocation, 50)
        .unwrap();
    let c2 = next(&mut f, &c1, &mut l1);
    assert!(matches!(
        l1.invalidate(key("stale-one")),
        Err(LedgerError::NeedsRecovery)
    ));
    let mut l2 = reopened(&f, &c2, &minimum(&c1));
    assert_receipt(
        &l2.complete_allocation(key("generation1-complete"), &allocation, 50)
            .unwrap(),
        &first,
    );
    let job = reference(&l2, "job");
    let pending = l2
        .begin_allocation(key("generation2-pending"), &job, key("next-event"), 75)
        .unwrap();
    let c3 = next(&mut f, &c2, &mut l2);
    assert!(matches!(
        l2.invalidate(key("stale-two")),
        Err(LedgerError::NeedsRecovery)
    ));
    let mut l3 = reopened(&f, &c3, &minimum(&c1));
    assert_receipt(
        &l3.reserve(key("job"), key("owner"), 1000).unwrap(),
        &original,
    );
    assert_receipt(
        &l3.begin_allocation(key("generation2-pending"), &job, key("next-event"), 75)
            .unwrap(),
        &pending,
    );
    let done = l3
        .complete_allocation(key("generation3-complete"), pending.reference(), 0)
        .unwrap();
    let m = Minimum {
        checkpoint: l3.checkpoint(),
        ..minimum(&c3)
    };
    drop(l3);
    let mut l3 = reopened(&f, &c3, &m);
    assert_receipt(
        &l3.complete_allocation(key("generation3-complete"), pending.reference(), 0)
            .unwrap(),
        &done,
    );
    assert_receipt(
        &l3.complete_allocation(key("generation1-complete"), &allocation, 50)
            .unwrap(),
        &first,
    );
    assert_eq!(l3.outputs.len(), 1);
    assert!(matches!(
        l3.reserve(key("no-freshness"), key("owner"), 1),
        Err(LedgerError::NeedsProbe)
    ));
    fs::rename(
        l3.root.join(c1.relative_name().unwrap()),
        f._temp.path().join("older-control-held-aside.json"),
    )
    .unwrap();
    l3.check_history().unwrap();
    let _ancestor = reopened(&f, &c3, &minimum(&c1));
    let mut forged = minimum(&c3);
    forged.activation.digest = c1.identity().unwrap().digest;
    assert!(open(&l3.root, &c3, &forged, f.nodes.finished_reader().unwrap()).is_err());
}

#[test]
fn second_generation_requires_only_its_immediate_source_not_the_original_file() {
    let mut f = fixture();
    let c1 = activate(&mut f);
    let mut l1 = reopened(&f, &c1, &minimum(&c1));
    l1.invalidate(key("first-tail")).unwrap();
    let c2 = next(&mut f, &c1, &mut l1);
    let root = l1.root.clone();
    drop(l1);
    drop(f.ledger);
    fs::rename(
        root.join("ledger.sqlite"),
        f._temp.path().join("original-held-aside.sqlite"),
    )
    .unwrap();
    let mut l2 = open(
        &root,
        &c2,
        &minimum(&c1),
        f.nodes.finished_reader().unwrap(),
    )
    .unwrap();
    l2.invalidate(key("second-tail")).unwrap();
    assert!(!root.join("ledger.sqlite").exists());
}

#[test]
fn missing_or_rebound_prior_archive_and_missing_source_never_trigger_repair() {
    for case in 0..3 {
        let mut f = fixture();
        let c1 = activate(&mut f);
        let mut l1 = reopened(&f, &c1, &minimum(&c1));
        l1.invalidate(key("first-tail")).unwrap();
        let c2 = next(&mut f, &c1, &mut l1);
        let root = l1.root.clone();
        drop(l1);
        let prior = root.join(c1.relative_name().unwrap());
        match case {
            0 => fs::rename(&prior, f._temp.path().join("held-prior.json")).unwrap(),
            1 => fs::write(&prior, c2.to_json().unwrap()).unwrap(),
            _ => fs::rename(
                root.join(c1.preparation.tail.relative_name().unwrap()),
                f._temp.path().join("held-source.sqlite"),
            )
            .unwrap(),
        }
        assert!(
            open(
                &root,
                &c2,
                &minimum(&c2),
                f.nodes.finished_reader().unwrap()
            )
            .is_err()
        );
        if case == 0 {
            assert!(!prior.exists());
        } else if case == 2 {
            assert!(
                !root
                    .join(c1.preparation.tail.relative_name().unwrap())
                    .exists()
            );
        }
    }
}

#[test]
fn a_matching_seal_is_not_a_complete_header_binding_and_wire_receipts_are_not_seals() {
    for wrong_header in [true, false] {
        let mut f = fixture();
        let c1 = activate(&mut f);
        let mut l1 = reopened(&f, &c1, &minimum(&c1));
        l1.invalidate(key("first-tail")).unwrap();
        let c2 = next(&mut f, &c1, &mut l1);
        let root = l1.root.clone();
        drop(l1);
        let conn = connect(&root.join(c1.preparation.tail.relative_name().unwrap())).unwrap();
        if wrong_header {
            conn.execute_batch("DROP TRIGGER tig_tail_UPDATE").unwrap();
            let mut identity = c1.preparation.tail.clone();
            identity.operation = key("different-source-operation");
            let raw = bytes(&identity).unwrap();
            conn.execute(
                "UPDATE tig_tail SET body=?1,hash=?2",
                params![raw, Digest::blob(&raw).as_str()],
            )
            .unwrap();
            conn.execute(&definitions()["tig_tail_UPDATE"].1, [])
                .unwrap();
            // Same actual seal, authority, generation and head: only the exact
            // checked prior TailIdentity distinguishes this rebound source.
            activation::verify_seal(&conn, &c2.seal.intent.to_json().unwrap(), &c2.seal.hash)
                .unwrap();
        } else {
            for (name, (kind, _)) in activation::definitions() {
                if kind == "trigger" {
                    conn.execute_batch(&format!("DROP TRIGGER {name}")).unwrap();
                }
            }
            conn.execute_batch("DROP TABLE tig_activation").unwrap();
            schema(&conn).unwrap();
        }
        drop(conn);
        assert!(
            open(
                &root,
                &c2,
                &minimum(&c2),
                f.nodes.finished_reader().unwrap()
            )
            .is_err()
        );
    }
}
