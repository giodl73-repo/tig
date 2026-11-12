"""Partition the compiled store library inventory without dropping test names."""
import argparse
import json
import os
from pathlib import Path
import re
import subprocess


SHARDS = ("store", "journal", "continuation", "rotations")
ROTATIONS_PREFIX = "ledger::continuation::tail::engine::tests::rotations::"


def partition(listing):
    groups = {name: [] for name in SHARDS}
    seen = set()
    for line in listing.splitlines():
        if not line.strip():
            continue
        match = re.fullmatch(r"([A-Za-z0-9_:]+): test", line)
        if not match or match[1] in seen:
            raise ValueError(f"invalid or duplicate test inventory: {line!r}")
        name = match[1]
        seen.add(name)
        shard = ("rotations" if name.startswith(ROTATIONS_PREFIX)
                 else "continuation" if name.startswith("ledger::continuation::")
                 else "journal" if name.startswith("ledger::") else "store")
        groups[shard].append(name)
    if any(not names for names in groups.values()):
        raise ValueError("empty store library inventory or shard")
    return groups


def commands(executable, names):
    if not names or len(names) != len(set(names)):
        raise ValueError("empty or duplicate selected tests")
    # Bound Windows command-line length as well as argument count.
    batch, size = [], len(str(executable)) + 40
    for name in names:
        if len(name) > 4096:
            raise ValueError("test name exceeds argument bound")
        if batch and (len(batch) == 64 or size + len(name) + 3 > 24000):
            yield [str(executable), "--exact", "--test-threads=1", *batch]
            batch, size = [], len(str(executable)) + 40
        batch.append(name)
        size += len(name) + 3
    yield [str(executable), "--exact", "--test-threads=1", *batch]


def run(shard):
    build = subprocess.run(
        ["cargo", "test", "-p", "tig-store", "--lib", "--all-features",
         "--locked", "--no-run", "--message-format=json"],
        capture_output=True, text=True)
    print(build.stderr, end="", flush=True)
    build.check_returncode()
    executables = set()
    for line in build.stdout.splitlines():
        item = json.loads(line)
        if (item.get("reason") == "compiler-artifact"
                and item.get("target", {}).get("name") == "tig_store"
                and item.get("profile", {}).get("test")
                and item.get("executable")):
            executables.add(item["executable"])
    if len(executables) != 1:
        raise ValueError("expected exactly one store library test executable")
    executable = Path(executables.pop()).resolve(strict=True)
    listing = subprocess.run([str(executable), "--list", "--format", "terse"],
                             check=True, capture_output=True, text=True).stdout
    groups = partition(listing)
    print("Store library inventory: " + ", ".join(
        f"{name}={len(names)}" for name, names in groups.items()), flush=True)
    # Serialize independent resource-heavy fixtures. Their explicit peer/process
    # concurrency is still exercised; native default ignored semantics remain.
    for command in commands(executable, groups[shard]):
        subprocess.run(command, check=True, env=os.environ.copy())


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("shard", choices=SHARDS)
    run(parser.parse_args().shard)
