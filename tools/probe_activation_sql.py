"""Supporting SQLite behavior probe; not legacy Rust or activation qualification."""

import argparse
import hashlib
import json
import queue
import sqlite3
import subprocess
import sys
import threading
from contextlib import closing
from pathlib import Path


TABLES = ("events", "meta", "tig_activation")
ACTIONS = ("INSERT", "UPDATE", "DELETE")
FENCE_TABLE = "CREATE TABLE tig_activation(id INTEGER PRIMARY KEY CHECK(id=1),body BLOB NOT NULL CHECK(length(body)<=65536),hash TEXT NOT NULL)"


def guard_sql(table, action):
    return (f"CREATE TRIGGER tig_activation_{table}_{action} BEFORE {action} ON {table} "
            "BEGIN SELECT RAISE(ABORT,'tig:sealed-source'); END")


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def connect(root, create_file=False):
    path = root / "ledger.sqlite"
    address = path if create_file else path.resolve().as_uri() + "?mode=rw"
    db = sqlite3.connect(address, timeout=0, isolation_level=None, uri=not create_file)
    db.execute("PRAGMA journal_mode=DELETE")
    db.execute("PRAGMA synchronous=FULL")
    return db


def fixture_config(domain):
    return {
        "schema": "tig.capacity-ledger.design-1",
        "domain": domain,
        "epoch": "synthetic-epoch",
        "instance": digest(("public activation SQL probe instance:" + domain).encode("utf-8")),
        "floor": 0,
    }
def create(root, domain="synthetic-volume"):
    root.mkdir()
    raw = canonical(fixture_config(domain))
    (root / "domain.json").write_bytes(raw)
    genesis = digest(raw)
    with closing(connect(root, create_file=True)) as db:
        db.execute("BEGIN IMMEDIATE")
        db.execute("CREATE TABLE meta(id INTEGER PRIMARY KEY CHECK(id=1),genesis TEXT NOT NULL,sequence INTEGER NOT NULL,hash TEXT NOT NULL,bytes INTEGER NOT NULL)")
        db.execute("CREATE TABLE events(sequence INTEGER PRIMARY KEY,operation TEXT NOT NULL UNIQUE,body BLOB NOT NULL,previous TEXT NOT NULL,hash TEXT NOT NULL)")
        db.execute("INSERT INTO meta VALUES(1,?1,0,?1,0)", (genesis,))
        db.execute("COMMIT")
    return {"genesis": genesis, "sequence": 0, "hash": genesis}


def checkpoint(db):
    genesis, sequence, hash_value = db.execute(
        "SELECT genesis,sequence,hash FROM meta WHERE id=1"
    ).fetchone()
    return {"genesis": genesis, "sequence": sequence, "hash": hash_value}


def append(db, operation):
    before = checkpoint(db)
    request = {"operation": operation, "command": {"kind": "invalidate"}}
    body = canonical(request)
    sequence = before["sequence"] + 1
    hash_value = digest(canonical({
        "schema": "tig.capacity-event.design-1",
        "sequence": sequence,
        "previous": before["hash"],
        "request": request,
    }))
    db.execute("INSERT INTO events VALUES(?,?,?,?,?)", (
        sequence, operation, body, before["hash"], hash_value
    ))
    db.execute("UPDATE meta SET sequence=?,hash=?,bytes=bytes+? WHERE id=1", (
        sequence, hash_value, len(body)
    ))


def seal_sql(db, expected, intent):
    if checkpoint(db) != expected:
        raise ValueError("stale source checkpoint")
    if intent["source"] != expected or intent["authority"] != expected["genesis"]:
        raise ValueError("wrong fixture intent source")
    body = canonical(intent)
    if len(body) > 65536:
        raise ValueError("oversize fixture intent")
    db.execute(FENCE_TABLE)
    db.execute("INSERT INTO tig_activation VALUES(1,?,?)", (body, digest(body)))
    for table in TABLES:
        for action in ACTIONS:
            # These identifiers are fixed source literals, never caller input.
            db.execute(guard_sql(table, action))


def delayed_writer(root):
    # This models only timing around SQL admission, not Rust guard semantics.
    config = (root / "domain.json").read_bytes()
    if config != canonical(fixture_config("synthetic-volume")):
        raise ValueError("delayed writer only accepts this public synthetic fixture config")
    with closing(connect(root)) as db:
        before = checkpoint(db)
        print("READY", flush=True)
        if sys.stdin.readline() != "release\n":
            raise RuntimeError("missing explicit fixture release")
        refused = False
        try:
            db.execute("BEGIN IMMEDIATE")
            append(db, "post-guard-writer")
            db.execute("COMMIT")
        except sqlite3.DatabaseError as exc:
            if db.in_transaction:
                db.execute("ROLLBACK")
            refused = "tig:sealed-source" in str(exc)
        print(json.dumps({"refused": refused, "unchanged": checkpoint(db) == before}))


def activation_status(db, minimum):
    present = db.execute("SELECT count(*) FROM sqlite_master WHERE name='tig_activation'").fetchone()[0]
    if not present:
        return "hold"
    observed = dict(db.execute("SELECT name,sql FROM sqlite_master WHERE type='trigger'"))
    expected = {f"tig_activation_{table}_{action}": guard_sql(table, action)
                for table in TABLES for action in ACTIONS}
    if observed != expected:
        return "hold"
    row = db.execute("SELECT body,hash FROM tig_activation WHERE id=1").fetchone()
    if row is None:
        return "hold"
    body, hash_value = row
    intent = json.loads(body)
    if (canonical(intent) != body or digest(body) != hash_value
            or hash_value != minimum["intent"]
            or intent["authority"] != minimum["authority"]
            or intent["generation"] < minimum["generation"]
            or intent["source"] != checkpoint(db)):
        return "hold"
    return "matches-minimum"


def run(root):
    root.mkdir()  # Explicit new synthetic destination; never reuse an old root.
    source = root / "source"
    create(source)
    with closing(connect(source)) as db:
        db.execute("BEGIN IMMEDIATE")
        append(db, "original-acknowledged")
        db.execute("COMMIT")
        original = checkpoint(db)
    prepared = canonical({"schema": "tig.activation-probe-prepared.design-1", "synthetic": True})
    (root / "prepared.json").write_bytes(prepared)
    intent = {
        "schema": "tig.capacity-activation-intent.design-1",
        "operation": "activation-fixture",
        "authority": original["genesis"],
        "generation": 1,
        "source": original,
        "prepared": {"sha256": digest(prepared), "bytes": len(prepared)},
    }
    with closing(connect(source)) as db:
        with closing(sqlite3.connect(root / "pre-fence.sqlite")) as backup:
            db.backup(backup)
    child = subprocess.Popen(
        [sys.executable, "-B", str(Path(__file__).resolve()), "--root", str(source), "--delayed-writer"],
        stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
        text=True, encoding="utf-8",
    )
    ready = queue.Queue()
    threading.Thread(target=lambda: ready.put(child.stdout.readline()), daemon=True).start()
    try:
        if ready.get(timeout=10) != "READY\n":
            raise RuntimeError("supporting SQL writer did not reach its barrier")
        with closing(connect(source)) as db:
            db.execute("BEGIN IMMEDIATE")
            seal_sql(db, original, intent)
            db.execute("COMMIT")
        stdout, stderr = child.communicate("release\n", timeout=10)
        if child.returncode != 0:
            raise RuntimeError(f"supporting writer failed: {stderr}")
        result = json.loads(stdout)
        assert result == {"refused": True, "unchanged": True}
    finally:
        if child.poll() is None:
            child.kill()
            child.wait(timeout=10)
    with closing(connect(source)) as db:
        assert checkpoint(db) == original
        assert db.execute("SELECT count(*) FROM events").fetchone()[0] == 1
        assert db.execute("SELECT count(*) FROM sqlite_master WHERE type='trigger'").fetchone()[0] == 9
        body, fence_hash = db.execute("SELECT body,hash FROM tig_activation WHERE id=1").fetchone()
        assert body == canonical(intent) and fence_hash == digest(body)
        for table in TABLES:
            try:
                db.execute(f"DELETE FROM {table}")
                raise AssertionError("sealed row deletion unexpectedly succeeded")
            except sqlite3.DatabaseError as exc:
                assert "tig:sealed-source" in str(exc)
    other = root / "other-domain"
    create(other, "synthetic-other-domain")
    with closing(connect(other)) as db:
        db.execute("BEGIN IMMEDIATE")
        append(db, "unrelated-domain")
        db.execute("COMMIT")
        assert checkpoint(db)["sequence"] == 1
    rollback = root / "rollback"
    expected = create(rollback)
    with closing(connect(rollback)) as db:
        db.execute("BEGIN IMMEDIATE")
        seal_sql(db, expected, dict(intent, source=expected))
        db.execute("ROLLBACK")
        assert db.execute("SELECT count(*) FROM sqlite_master WHERE name='tig_activation' OR type='trigger'").fetchone()[0] == 0
        db.execute("BEGIN IMMEDIATE")
        append(db, "after-rollback")
        db.execute("COMMIT")
        assert checkpoint(db)["sequence"] == 1
    minimum = {"authority": original["genesis"], "generation": 1, "intent": digest(canonical(intent))}
    (root / "activation-minimum.json").write_bytes(canonical(minimum))
    with closing(connect(source)) as db:
        assert activation_status(db, minimum) == "matches-minimum"
        with closing(sqlite3.connect(root / "partial-fence.sqlite", isolation_level=None)) as partial:
            db.backup(partial)
            partial.execute("DROP TRIGGER tig_activation_events_INSERT")
            assert activation_status(partial, minimum) == "hold"
    with closing(sqlite3.connect((root / "pre-fence.sqlite").resolve().as_uri() + "?mode=rw", uri=True)) as restored:
        assert checkpoint(restored) == original  # Ordinary minimum cannot distinguish it.
        assert restored.execute("SELECT count(*) FROM sqlite_master WHERE name='tig_activation'").fetchone()[0] == 0
        restore_holds = activation_status(restored, minimum) == "hold"
        assert restore_holds
        # Counterexample: a writer that ignores the retained minimum can fork.
        with closing(sqlite3.connect(root / "unchecked-restore.sqlite", isolation_level=None)) as unchecked:
            restored.backup(unchecked)
            unchecked.execute("BEGIN IMMEDIATE")
            append(unchecked, "unchecked-restored-writer")
            unchecked.execute("COMMIT")
            assert checkpoint(unchecked)["sequence"] == original["sequence"] + 1
    report = {
        "schema": "tig.activation-sql-probe.design-1",
        "scope": "supporting-sqlite-behavior-not-legacy-rust-qualification",
        "sqlite_version": sqlite3.sqlite_version,
        "delayed_sql_writer_refused": True,
        "unchanged_ordinary_checkpoint": True,
        "commit_reopen_guarded": True,
        "rollback_keeps_writable": True,
        "independent_domain_progress": True,
        "ordinary_minimum_cannot_detect_pre_fence_restore": True,
        "retained_activation_minimum_holds_restore_model": restore_holds,
        "unchecked_pre_fence_restore_can_fork": True,
        "partial_guard_set_holds_restore_model": True,
        "legacy_rust_qualified": False,
        "activation_implemented": False,
    }
    (root / "report.json").write_bytes(canonical(report))
    print(json.dumps(report, sort_keys=True))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", required=True, type=Path)
    parser.add_argument("--delayed-writer", action="store_true", help=argparse.SUPPRESS)
    args = parser.parse_args()
    if args.delayed_writer:
        delayed_writer(args.root)
    else:
        run(args.root.absolute())
