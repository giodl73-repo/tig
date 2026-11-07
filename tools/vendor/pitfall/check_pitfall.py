#!/usr/bin/env python3
"""Validate repository-local PITFALL indexes and doctrine documents."""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path


ROW_RE = re.compile(
    r"^\|\s*`?([a-z0-9-]+)`?\s*\|\s*`?(principles|invariants|pitfalls)`?\s*"
    r"\|\s*\[[^\]]+\]\(([^)]+)\)\s*\|\s*([^|]+?)\s*\|$",
    re.IGNORECASE,
)
ENTRY_RE = re.compile(
    r"^#{2,4}\s+([A-Z][A-Z0-9]*(?:-[A-Z][A-Z0-9]*)*-\d+)"
    r"\s*(?::|—|-)\s*(.+)$"
)
LABEL_RE = re.compile(r"^\*\*([^*:]+)(?::\*\*|\*\*:)\s*(.*)$", re.IGNORECASE)
VALID_TYPES = {"principles", "invariants", "pitfalls"}
RETENTION_STATUSES = {"SOLVED", "SUPERSEDED"}


@dataclass(frozen=True)
class RegistryEntry:
    namespace: str
    kind: str
    path: Path


def parse_registry(index: Path) -> list[RegistryEntry]:
    entries: list[RegistryEntry] = []
    for line in index.read_text(encoding="utf-8").splitlines():
        match = ROW_RE.match(line.strip())
        if not match:
            continue
        namespace, kind, raw_path, _owner = match.groups()
        entries.append(
            RegistryEntry(
                namespace=namespace.lower(),
                kind=kind.lower(),
                path=(index.parent / raw_path).resolve(),
            )
        )
    return entries


def parse_entries(path: Path) -> list[tuple[str, dict[str, str]]]:
    parsed: list[tuple[str, dict[str, str]]] = []
    current_id: str | None = None
    labels: dict[str, str] = {}
    current_label: str | None = None
    for line in path.read_text(encoding="utf-8").splitlines():
        heading = ENTRY_RE.match(line)
        if heading:
            if current_id is not None:
                parsed.append((current_id, labels))
            current_id = heading.group(1)
            labels = {}
            current_label = None
            continue
        if current_id is None:
            continue
        label = LABEL_RE.match(line.strip())
        if label:
            current_label = label.group(1).strip().lower()
            labels[current_label] = label.group(2).strip()
        elif not line.strip():
            current_label = None
        elif current_label is not None:
            labels[current_label] = (
                f"{labels[current_label]} {line.strip()}".strip()
            )
    if current_id is not None:
        parsed.append((current_id, labels))
    if parsed:
        return parsed
    return parse_invariant_tables(path.read_text(encoding="utf-8"))


def parse_invariant_tables(text: str) -> list[tuple[str, dict[str, str]]]:
    parsed: list[tuple[str, dict[str, str]]] = []
    lines = text.splitlines()
    index = 0
    while index + 1 < len(lines):
        headers = table_cells(lines[index])
        separator = table_cells(lines[index + 1])
        normalized = [header.strip().lower() for header in headers]
        if (
            not headers
            or len(separator) != len(headers)
            or not all(set(cell.strip()) <= {"-", ":"} for cell in separator)
            or "id" not in normalized
            or "status" not in normalized
            or not ({"invariant", "claim"} & set(normalized))
        ):
            index += 1
            continue
        id_column = normalized.index("id")
        status_column = normalized.index("status")
        statement_column = next(
            normalized.index(name) for name in ("invariant", "claim")
            if name in normalized
        )
        notes_column = normalized.index("notes") if "notes" in normalized else None
        index += 2
        while index < len(lines):
            cells = table_cells(lines[index])
            if len(cells) != len(headers):
                break
            entry_id = cells[id_column].strip().strip("`")
            if re.fullmatch(r"[A-Z][A-Z0-9]*(?:-[A-Z][A-Z0-9]*)*-\d+", entry_id):
                labels = {
                    "_format": "table",
                    "claim": cells[statement_column].strip(),
                    "status": cells[status_column].strip(),
                }
                if notes_column is not None:
                    labels["evidence"] = cells[notes_column].strip()
                parsed.append((entry_id, labels))
            index += 1
    return parsed


def table_cells(line: str) -> list[str]:
    stripped = line.strip()
    if not stripped.startswith("|") or not stripped.endswith("|"):
        return []
    return stripped[1:-1].split("|")


def validate_entry(kind: str, entry_id: str, labels: dict[str, str]) -> list[str]:
    errors: list[str] = []
    present = {key for key, value in labels.items() if value}
    if kind == "invariants" and labels.get("_format") == "table":
        for field in ("claim", "status", "evidence"):
            if field not in present:
                errors.append(
                    f"{entry_id}: compact invariant requires '{field.title()}'"
                )
        return errors
    if kind == "principles":
        if not ({"decision rule", "test"} & present):
            errors.append(
                f"{entry_id}: principle requires 'Decision rule' or 'Test'"
            )
    elif kind == "invariants":
        if not ({"claim", "invariant"} & present):
            errors.append(f"{entry_id}: invariant requires 'Claim' or 'Invariant'")
        if not ({"why it matters", "why it must hold"} & present):
            errors.append(
                f"{entry_id}: invariant requires 'Why it matters' or "
                "'Why it must hold'"
            )
        if "status" not in present:
            errors.append(f"{entry_id}: invariant requires 'Status'")
        if not ({"test", "enforcement"} & present):
            errors.append(f"{entry_id}: invariant requires 'Test' or 'Enforcement'")
    elif kind == "pitfalls":
        if not ({"pattern", "description"} & present):
            errors.append(f"{entry_id}: pitfall requires 'Pattern' or 'Description'")
        if "status" not in present:
            errors.append(f"{entry_id}: pitfall requires 'Status'")
        if not (
            {"structural solution", "structural solution required", "countermeasure"}
            & present
        ):
            errors.append(
                f"{entry_id}: pitfall requires 'Structural solution' or 'Countermeasure'"
            )
    return errors


def parsed_status(labels: dict[str, str]) -> str:
    parts = labels.get("status", "").strip().split(maxsplit=1)
    return parts[0].upper() if parts else ""


def has_any(labels: dict[str, str], fields: set[str]) -> bool:
    return any(labels.get(field, "").strip() for field in fields)


def validate(root: Path) -> tuple[list[str], list[str]]:
    root = root.resolve()
    index = root / ".pitfall" / "PITFALL.md"
    errors: list[str] = []
    warnings: list[str] = []
    if not index.is_file():
        return [f"{root}: missing .pitfall/PITFALL.md"], warnings

    registry = parse_registry(index)
    if not registry:
        return [f"{index}: no doctrine registry rows found"], warnings

    seen_paths: set[Path] = set()
    seen_ids: set[tuple[str, str]] = set()
    for item in registry:
        if item.kind not in VALID_TYPES:
            errors.append(f"{item.path}: unsupported doctrine type '{item.kind}'")
            continue
        try:
            item.path.relative_to(root)
        except ValueError:
            errors.append(f"{item.path}: doctrine path escapes repository root")
            continue
        if item.path in seen_paths:
            errors.append(f"{item.path}: duplicate doctrine registry path")
            continue
        seen_paths.add(item.path)
        if not item.path.is_file():
            errors.append(f"{item.path}: linked doctrine file does not exist")
            continue

        entries = parse_entries(item.path)
        if not entries:
            errors.append(f"{item.path}: no stable doctrine entry IDs found")
            continue
        for entry_id, labels in entries:
            key = (item.namespace, entry_id)
            if key in seen_ids:
                errors.append(
                    f"{item.path}: duplicate entry '{entry_id}' in namespace "
                    f"'{item.namespace}'"
                )
            seen_ids.add(key)
            errors.extend(
                f"{item.path}: {message}"
                for message in validate_entry(item.kind, entry_id, labels)
            )
            if item.kind == "pitfalls":
                present = {key for key, value in labels.items() if value}
                if not ({"domain", "applicability"} & present):
                    warnings.append(
                        f"{item.path}: {entry_id}: no Domain or Applicability field"
                    )
                if not (
                    {
                        "why it is hard to catch",
                        "why it's hard to catch",
                        "detection difficulty",
                    }
                    & present
                ):
                    warnings.append(
                        f"{item.path}: {entry_id}: no detection-difficulty context"
                    )
                if parsed_status(labels) in RETENTION_STATUSES and not has_any(
                    labels,
                    {
                        "evidence",
                        "test",
                        "test required",
                        "superseded by",
                        "translation note",
                        "migration note",
                        "replacement",
                    },
                ):
                    warnings.append(
                        f"{item.path}: {entry_id}: solved or superseded pitfall "
                        "lacks retention evidence, test, or translation note"
                    )
            if not ({"evidence", "test", "test required"} & labels.keys()):
                warnings.append(f"{item.path}: {entry_id}: no Evidence or Test field")
    return errors, warnings


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate PITFALL doctrine")
    parser.add_argument("paths", nargs="+", type=Path, help="repository roots")
    args = parser.parse_args()

    failed = False
    for root in args.paths:
        errors, warnings = validate(root)
        for error in errors:
            print(f"ERROR {error}")
        for warning in warnings:
            print(f"WARN {warning}")
        if errors:
            failed = True
        elif not warnings:
            print(f"OK {root}")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
