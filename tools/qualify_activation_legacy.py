"""Explicit pinned, timing-instrumented legacy writer qualification fixture."""

import argparse
import difflib
import hashlib
import importlib.util
import json
import os
import subprocess
import time
from contextlib import closing
from pathlib import Path


PIN = "c833add85f9a9c79d3387a78e469bc13a2570403"
LEDGER = "crates/tig-store/src/ledger.rs"
TEST = "crates/tig-store/tests/activation_legacy_writer.rs"
ORIGINAL_SHA = "98a043d49d73d193a07a65cfb06c05f8b672ee8a0e385f30b8f3ef3ffda435ed"
HERE = Path(__file__).resolve().parent
TEMPLATE = HERE / "fixtures/activation_legacy_writer.rs"
PROBE = HERE / "probe_activation_sql.py"
NEEDLE = b"        self.guard()?;\n        let request = Request { operation, command };"
BARRIER = b'''        self.guard()?;
        if let (Ok(activation_ready), Ok(activation_release)) = (
            std::env::var("TIG_ACTIVATION_READY"), std::env::var("TIG_ACTIVATION_RELEASE")
        ) {
            std::fs::write(&activation_ready, b"after-guard-before-transaction")?;
            let activation_started = std::time::Instant::now();
            while !std::path::Path::new(&activation_release).is_file() {
                if activation_started.elapsed() > std::time::Duration::from_secs(20) {
                    return Err(LedgerError::Injected);
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
        let request = Request { operation, command };'''


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8", newline="\n")


def git(checkout, *args):
    return subprocess.run(["git", "-C", str(checkout), *args], check=True, capture_output=True).stdout


def check_pin(checkout):
    if not (checkout / ".git").is_file() or checkout == HERE.parent.resolve():
        raise ValueError("use an explicit detached linked worktree, never the active source checkout")
    attached = subprocess.run(["git", "-C", str(checkout), "symbolic-ref", "--quiet", "HEAD"],
                              capture_output=True)
    if attached.returncode != 1:
        raise ValueError("legacy fixture checkout must be detached")
    if git(checkout, "rev-parse", "HEAD").decode("utf-8").strip() != PIN:
        raise ValueError("checkout is not the pinned legacy source")


def prepare(checkout, evidence):
    check_pin(checkout)
    if git(checkout, "status", "--porcelain").strip():
        raise ValueError("prepare requires a clean owned legacy checkout; never reset it")
    original = git(checkout, "show", f"{PIN}:{LEDGER}")
    if sha(original) != ORIGINAL_SHA or (checkout / LEDGER).read_bytes() != original:
        raise ValueError("original ledger bytes do not match the pin")
    if original.count(NEEDLE) != 1:
        raise ValueError("timing barrier site is not unique")
    test_path = checkout / TEST
    if test_path.exists():
        raise ValueError("legacy fixture test path already exists")
    evidence.mkdir()
    patched = original.replace(NEEDLE, BARRIER)
    template = TEMPLATE.read_bytes()
    (checkout / LEDGER).write_bytes(patched)
    test_path.write_bytes(template)
    patch = "".join(difflib.unified_diff(
        original.decode("utf-8").splitlines(keepends=True),
        patched.decode("utf-8").splitlines(keepends=True),
        fromfile="a/" + LEDGER, tofile="b/" + LEDGER,
    ))
    (evidence / "timing.patch").write_text(patch, encoding="utf-8", newline="\n")
    plan = {
        "schema": "tig.legacy-activation-prepared.design-1",
        "legacy_source": PIN, "checkout": str(checkout),
        "original_ledger_sha256": sha(original), "patched_ledger_sha256": sha(patched),
        "test_sha256": sha(template), "patch_sha256": sha(patch.encode("utf-8")),
        "cargo_lock_sha256": sha((checkout / "Cargo.lock").read_bytes()),
        "harness_sha256": sha(Path(__file__).read_bytes()), "probe_sha256": sha(PROBE.read_bytes()),
        "timing_instrumented": True, "legacy_writer_executed": False,
        "activation_implemented": False,
    }
    write_json(evidence / "prepared.json", plan)
    print(json.dumps({"prepared": True, "legacy_source": PIN, "legacy_writer_executed": False}))


def check_prepared(checkout, evidence):
    plan = json.loads((evidence / "prepared.json").read_text(encoding="utf-8"))
    check_pin(checkout)
    if (plan.get("schema") != "tig.legacy-activation-prepared.design-1"
            or plan.get("checkout") != str(checkout) or plan.get("legacy_source") != PIN):
        raise ValueError("wrong prepared fixture identity")
    expected = {
        checkout / LEDGER: plan["patched_ledger_sha256"], checkout / TEST: plan["test_sha256"],
        checkout / "Cargo.lock": plan["cargo_lock_sha256"],
        evidence / "timing.patch": plan["patch_sha256"],
        Path(__file__): plan["harness_sha256"], PROBE: plan["probe_sha256"],
    }
    for path, expected_sha in expected.items():
        if sha(path.read_bytes()) != expected_sha:
            raise ValueError(f"prepared input changed: {path.name}")
    changed = set(git(checkout, "diff", "--name-only").decode("utf-8").splitlines())
    untracked = set(git(checkout, "ls-files", "--others", "--exclude-standard").decode("utf-8").splitlines())
    if changed != {LEDGER} or untracked != {TEST}:
        raise ValueError("legacy checkout contains changes beyond the timing patch/test")
    return plan


def probe_module():
    spec = importlib.util.spec_from_file_location("activation_probe", PROBE)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def child_env(case, base=None):
    env = (os.environ if base is None else base).copy()
    env.pop("TIG_ACTIVATION_READY", None)
    env.pop("TIG_ACTIVATION_RELEASE", None)
    env["TIG_ACTIVATION_CASE"] = str(case)
    return env


def run_prepared(checkout, evidence, target):
    plan = check_prepared(checkout, evidence)
    if target.is_relative_to(checkout) or target.is_relative_to(HERE.parent):
        raise ValueError("Cargo target must be outside both source checkouts")
    target.mkdir()  # Never reuse or clear another task's target.
    run = evidence / "run"
    run.mkdir()  # A failed run is preserved; no in-place restart.
    env = child_env(run)
    env.update({"TEMP": str(run), "TMP": str(run), "CARGO_TARGET_DIR": str(target),
                "CARGO_INCREMENTAL": "0", "CARGO_PROFILE_DEV_DEBUG": "0", "CARGO_PROFILE_TEST_DEBUG": "0",
                "CARGO_BUILD_JOBS": "1", "RUST_TEST_THREADS": "1", "CARGO_TERM_COLOR": "never"})
    build = subprocess.run(["cargo", "test", "--offline", "--locked", "-p", "tig-store", "--test",
                            "activation_legacy_writer", "--no-run", "--message-format=json"],
                           cwd=checkout, env=env, capture_output=True, text=True, encoding="utf-8", timeout=900)
    (run / "build.jsonl").write_text(build.stdout, encoding="utf-8", newline="\n")
    (run / "build.stderr").write_text(build.stderr, encoding="utf-8", newline="\n")
    if build.returncode != 0:
        raise RuntimeError("pinned fixture build failed; inspect preserved build.stderr")
    executables = []
    for line in build.stdout.splitlines():
        item = json.loads(line)
        if item.get("reason") == "compiler-artifact" and item.get("target", {}).get("name") == "activation_legacy_writer" and item.get("executable"):
            executables.append(Path(item["executable"]).resolve())
    if len(executables) != 1 or not executables[0].is_relative_to(target):
        raise ValueError("unexpected compiled fixture executable")
    executable = executables[0]
    case = run / "case"
    case.mkdir()
    write_json(case / "owner.json", {"schema": "tig.activation-legacy-case.design-1", "legacy_source": PIN})
    prepared = subprocess.run([str(executable), "--exact", "prepare_legacy_source", "--nocapture"],
                              env=child_env(case, env), capture_output=True, text=True, encoding="utf-8", timeout=30)
    (run / "prepare.stdout").write_text(prepared.stdout, encoding="utf-8", newline="\n")
    (run / "prepare.stderr").write_text(prepared.stderr, encoding="utf-8", newline="\n")
    if prepared.returncode != 0:
        raise RuntimeError("legacy Rust preparation failed")
    source = json.loads((case / "prepared-source.json").read_text(encoding="utf-8"))
    module = probe_module()
    raw = module.canonical({"schema": "tig.activation-legacy-probe-prepared.design-1", "synthetic": True})
    (run / "fixture-prepared.json").write_bytes(raw)
    minimum = source["minimum"]
    intent = {"schema": "tig.capacity-activation-intent.design-1", "operation": "legacy-fence-fixture",
              "authority": minimum["genesis"], "generation": 1, "source": minimum,
              "prepared": {"sha256": sha(raw), "bytes": len(raw)}}
    ready, release = case / "ready", case / "release"
    actor_env = child_env(case, env)
    actor_env.update({"TIG_ACTIVATION_READY": str(ready), "TIG_ACTIVATION_RELEASE": str(release)})
    actor = subprocess.Popen([str(executable), "--exact", "delayed_legacy_writer", "--nocapture"],
                             env=actor_env, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                             text=True, encoding="utf-8")
    try:
        deadline = time.monotonic() + 15
        while not ready.exists() or ready.read_bytes() != b"after-guard-before-transaction":
            if actor.poll() is not None or time.monotonic() > deadline:
                raise RuntimeError("pinned Rust writer did not reach its exact timing barrier")
            time.sleep(0.01)
        with closing(module.connect(case / "source")) as db:
            db.execute("BEGIN IMMEDIATE")
            module.seal_sql(db, minimum, intent)
            db.execute("COMMIT")
        release.write_bytes(b"released-after-seal-commit")
        stdout, stderr = actor.communicate(timeout=30)
        (run / "writer.stdout").write_text(stdout, encoding="utf-8", newline="\n")
        (run / "writer.stderr").write_text(stderr, encoding="utf-8", newline="\n")
        if actor.returncode != 0:
            raise RuntimeError("pinned legacy writer qualification failed")
    finally:
        if actor.poll() is None:
            actor.kill()
            actor.wait(timeout=10)
    writer = json.loads((case / "writer-result.json").read_text(encoding="utf-8"))
    with closing(module.connect(case / "source")) as db:
        if module.checkpoint(db) != minimum or db.execute("SELECT count(*) FROM events WHERE operation='after-guard-write'").fetchone()[0] != 0:
            raise AssertionError("old writer changed the acknowledged source")
    report = {"schema": "tig.instrumented-legacy-activation-proof.design-1", "legacy_source": PIN,
              "original_ledger_sha256": plan["original_ledger_sha256"], "patched_ledger_sha256": plan["patched_ledger_sha256"],
              "test_sha256": plan["test_sha256"], "patch_sha256": plan["patch_sha256"],
              "executable_sha256": sha(executable.read_bytes()), "writer": writer,
              "controller": "python-sqlite", "controller_sqlite_version": module.sqlite3.sqlite_version,
              "timing_instrumented": True, "unmodified_legacy_binary_qualified": False,
              "rust_activation_controller_implemented": False, "activation_implemented": False}
    write_json(run / "report.json", report)
    print(json.dumps(report, sort_keys=True))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--checkout", required=True, type=Path)
    parser.add_argument("--evidence", required=True, type=Path)
    parser.add_argument("--target", type=Path)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--prepare-only", action="store_true")
    mode.add_argument("--run-prepared", action="store_true")
    args = parser.parse_args()
    checkout, evidence = args.checkout.resolve(), args.evidence.resolve()
    if evidence.is_relative_to(checkout) or evidence.is_relative_to(HERE.parent.resolve()):
        parser.error("evidence must be outside both source checkouts")
    if args.prepare_only:
        prepare(checkout, evidence)
    elif args.target is None:
        parser.error("--run-prepared requires an explicit new external --target")
    else:
        run_prepared(checkout, evidence, args.target.resolve())
