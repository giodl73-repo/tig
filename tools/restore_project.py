"""Explicit consumer orchestration; byte reads/copies are delegated to Tig CLIs."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path, PurePosixPath
import shutil
import subprocess
import tempfile


def relative(value: str) -> str:
    if not isinstance(value, str) or not value or "\\" in value or ":" in value:
        raise ValueError("expected a portable relative path")
    parts = value.split("/")
    if any(part.lower() in ("", ".", "..", ".git") or part.rstrip(" .") != part for part in parts):
        raise ValueError("unsafe relative path")
    if any(part.split(".")[0].upper() in {"CON", "PRN", "AUX", "NUL", *[f"COM{i}" for i in range(1, 10)], *[f"LPT{i}" for i in range(1, 10)]} for part in parts):
        raise ValueError("reserved path component")
    return PurePosixPath(value).as_posix()


def parents(repo: Path, destination: str, apply: bool) -> Path:
    target = repo / relative(destination)
    current = repo
    for part in Path(destination).parts[:-1]:
        current /= part
        try:
            reparse = bool(getattr(current.lstat(), "st_file_attributes", 0) & 0x400)
        except FileNotFoundError:
            reparse = False
        if current.is_symlink() or reparse:
            raise ValueError("destination parent link refused")
        if apply:
            current.mkdir(exist_ok=True)
        elif current.exists() and not current.is_dir():
            raise ValueError("destination parent is not a directory")
    return target


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--repo", type=Path, default=Path.cwd())
    p.add_argument("--store", type=Path, required=True, help="existing reel-cas root")
    p.add_argument("--bin-dir", type=Path, required=True)
    p.add_argument("--revision", default="HEAD")
    p.add_argument("--apply", action="store_true")
    a = p.parse_args()
    repo = a.repo.resolve(strict=True)
    store = a.store.resolve(strict=True)
    git = shutil.which("git")
    if not git or not store.is_dir():
        raise ValueError("Git and an existing store directory are required")

    def command(args: list[str]) -> bytes:
        return subprocess.run(args, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=360).stdout

    def git_cmd(*args: str) -> bytes:
        return command([git, "-C", str(repo), *args])

    revision = git_cmd("rev-parse", "--verify", "--end-of-options", a.revision + "^{commit}").decode().strip()

    def document(path: str) -> dict:
        spec = f"{revision}:{relative(path)}"
        if int(git_cmd("cat-file", "-s", spec)) > 65536:
            raise ValueError("metadata file exceeds 64 KiB")
        raw = git_cmd("show", spec)
        return json.loads(raw)

    manifest = document("tig/project.json")
    if manifest["schema"] != "tig.project-restore.experimental-1":
        raise ValueError("unsupported project manifest")
    entries = manifest["records"]
    if not isinstance(entries, list) or not 1 <= len(entries) <= 1024 or len(set(entries)) != len(entries):
        raise ValueError("invalid bounded record list")
    prefixes = [relative(s).rstrip("/") + "/" for s in manifest["destination_prefixes"]]
    records = []
    total = 0
    destinations = set()
    for name in entries:
        name = relative(name)
        if not name.startswith("tig/records/") or not name.endswith(".tig"):
            raise ValueError("record outside declared scope")
        record = document(name)
        destination = relative(record["authored"]["restore_path"])
        if not any(destination.startswith(prefix) for prefix in prefixes) or destination in destinations:
            raise ValueError("destination outside scope or duplicated")
        destinations.add(destination)
        if len(record["streams"]) != 1:
            raise ValueError("one stream per restoration record is required")
        stream = record["streams"][0]
        total += stream["object"]["bytes"]
        if not 0 <= total <= 8 * 1024**3:
            raise ValueError("project restore exceeds 8 GiB bound")
        parents(repo, destination, False)
        records.append((record["id"], stream["slot"], destination))

    profile = {"schema": "tig.asset-resolver-profile.experimental-1", "git_executable": str(Path(git).resolve()),
               "repository": str(repo), "commit": revision, "records": ["tig/records"],
               "definitions": ["tig/definitions"], "byte_source": {
                   "places": [{"root": str(store), "layout": "reel-cas"}],
                   "required": "prevent-mutation" if os.name == "nt" else "cooperative-immutable",
                   "limits": {"verify_millis": 300000}}}
    suffix = ".exe" if os.name == "nt" else ""
    resolve = str((a.bin_dir / ("tig-resolve" + suffix)).resolve(strict=True))
    restore = str((a.bin_dir / ("tig-restore" + suffix)).resolve(strict=True))
    with tempfile.TemporaryDirectory(prefix="tig-explicit-restore-") as temporary:
        profile_path = Path(temporary) / "profile.json"
        profile_path.write_text(json.dumps(profile), encoding="utf-8")
        # Verify the complete requested set before making any project directories.
        for asset_id, slot, _ in records:
            command([resolve, "--verify", str(profile_path), asset_id, slot])
        actions = {}
        if a.apply:
            for asset_id, slot, destination in records:
                target = parents(repo, destination, True)
                result = json.loads(command([restore, "--apply", str(profile_path), asset_id, slot, str(target)]))
                action = result["action"]
                actions[action] = actions.get(action, 0) + 1
        print(json.dumps({"schema": "tig.project-restore-result.experimental-1", "commit": revision,
                          "apply": a.apply, "files": len(records), "bytes": total, "actions": actions,
                          "payload_custody": False, "complete_semantic_coverage": False}))


if __name__ == "__main__":
    main()
