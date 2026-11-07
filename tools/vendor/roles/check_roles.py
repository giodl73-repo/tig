#!/usr/bin/env python3
"""Validate .roles directories with no third-party dependencies."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path


SLUG_RE = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")


def parse_frontmatter(path: Path) -> dict[str, object]:
    lines = path.read_text(encoding="utf-8").splitlines()
    if not lines or lines[0].strip() != "---":
        return {}

    data: dict[str, object] = {}
    for line in lines[1:]:
        if line.strip() == "---":
            return data
        if ":" not in line:
            continue
        key, raw_value = line.split(":", 1)
        key = key.strip()
        value = raw_value.strip()
        if value.startswith("[") and value.endswith("]"):
            items = [item.strip().strip("\"'") for item in value[1:-1].split(",")]
            data[key] = [item for item in items if item]
        else:
            data[key] = value.strip("\"'")
    return {}


def validate(root: Path) -> tuple[list[str], list[str]]:
    roles_dir = root / ".roles"
    errors: list[str] = []
    warnings: list[str] = []

    if not roles_dir.is_dir():
        return [f"{root}: missing .roles directory"], warnings

    index = roles_dir / "ROLE.md"
    if not index.is_file():
        errors.append(f"{root}: missing .roles/ROLE.md")

    role_files = sorted(
        path
        for path in roles_dir.rglob("*.md")
        if path.name.lower() != "role.md"
    )
    if not role_files:
        warnings.append(f"{root}: no role markdown files found below .roles")

    for path in role_files:
        rel = path.relative_to(root)
        meta = parse_frontmatter(path)
        if not meta:
            warnings.append(f"{rel}: missing frontmatter")
            continue

        for field in ("name", "slug", "tier"):
            if not str(meta.get(field, "")).strip():
                warnings.append(f"{rel}: missing frontmatter field '{field}'")

        slug = str(meta.get("slug", "")).strip()
        if slug and not SLUG_RE.fullmatch(slug):
            errors.append(f"{rel}: slug must be lowercase kebab-case")

        tier = str(meta.get("tier", "")).strip()
        if tier:
            parent = path.parent.name
            if parent != ".roles" and tier != parent:
                warnings.append(f"{rel}: tier '{tier}' does not match folder '{parent}'")

    return errors, warnings


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate .roles directories")
    parser.add_argument("paths", nargs="+", type=Path, help="repo roots to check")
    args = parser.parse_args()

    any_errors = False
    for root in args.paths:
        errors, warnings = validate(root)
        if errors:
            any_errors = True
            for error in errors:
                print(f"ERROR {error}")
        for warning in warnings:
            print(f"WARN {warning}")
        if not errors and not warnings:
            print(f"OK {root}")

    return 1 if any_errors else 0


if __name__ == "__main__":
    sys.exit(main())
