from __future__ import annotations

import re
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
SKIP_DIRS = {".git", "target", "__pycache__"}
CORPUS_TOP_LEVEL = {
    "00-meta",
    "01-foundations",
    "02-data-structures",
    "03-performance-engineering",
    "04-algorithms",
    "05-systems",
    "06-tooling",
    "07-research",
    "08-execution",
    "_registry",
    "_templates",
    "_tracking",
}
REQUIRED_FRONT_MATTER_KEYS = {
    "doc_id",
    "title",
    "status",
    "last_verified",
    "source_scope",
    "depends_on",
    "see_also",
}


def iter_markdown_files() -> list[Path]:
    files: list[Path] = []
    for path in REPO_ROOT.rglob("*.md"):
        if any(part in SKIP_DIRS for part in path.parts):
            continue
        files.append(path)
    return sorted(files)


def requires_front_matter(path: Path) -> bool:
    relative = path.relative_to(REPO_ROOT).as_posix()
    if relative == "README.md":
        return True
    top_level = relative.split("/", 1)[0]
    return top_level in CORPUS_TOP_LEVEL


def validate_front_matter(path: Path, errors: list[str]) -> None:
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---"):
        errors.append(f"{path}: missing YAML front matter header")
        return

    lines = text.splitlines()
    if not lines or lines[0].strip() != "---":
        errors.append(f"{path}: malformed YAML front matter opening delimiter")
        return

    closing_index = None
    for index in range(1, len(lines)):
        if lines[index].strip() == "---":
            closing_index = index
            break

    if closing_index is None:
        errors.append(f"{path}: missing YAML front matter closing delimiter")
        return

    front_matter_lines = lines[1:closing_index]
    keys = set()
    last_verified_value = None

    for line in front_matter_lines:
        match = re.match(r"^([A-Za-z_][A-Za-z0-9_-]*):", line)
        if match:
            key = match.group(1)
            keys.add(key)
            if key == "last_verified":
                _, _, value = line.partition(":")
                last_verified_value = value.strip()

    missing = REQUIRED_FRONT_MATTER_KEYS - keys
    if missing:
        errors.append(
            f"{path}: missing front matter keys: {', '.join(sorted(missing))}"
        )

    if last_verified_value and not re.fullmatch(r"\d{4}-\d{2}-\d{2}", last_verified_value):
        errors.append(f"{path}: last_verified must use YYYY-MM-DD")


def extract_local_links(text: str) -> list[str]:
    text_without_fences = re.sub(r"(?s)```.*?```", "", text)
    matches = re.findall(r"\[[^\]]+\]\(([^)]+)\)", text_without_fences)
    results: list[str] = []
    for target in matches:
        candidate = target.strip().strip("<>").strip()
        if not candidate:
            continue
        if candidate.startswith(("#", "http://", "https://", "mailto:", "data:")):
            continue
        if re.match(r"^[A-Za-z]:[\\/]", candidate):
            continue
        results.append(candidate)
    return results


def validate_links(path: Path, errors: list[str]) -> None:
    text = path.read_text(encoding="utf-8")
    for raw_target in extract_local_links(text):
        target_without_anchor = raw_target.split("#", 1)[0]
        candidate = (path.parent / target_without_anchor).resolve()
        if not candidate.exists():
            errors.append(f"{path}: broken relative link -> {raw_target}")


def main() -> int:
    markdown_files = iter_markdown_files()
    errors: list[str] = []

    for path in markdown_files:
        if requires_front_matter(path):
            validate_front_matter(path, errors)
        validate_links(path, errors)

    if errors:
        for error in errors:
            print(error, file=sys.stderr)
        print(f"validation failed: {len(errors)} issue(s)", file=sys.stderr)
        return 1

    print(
        f"validation passed: {len(markdown_files)} markdown files checked from {REPO_ROOT}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
