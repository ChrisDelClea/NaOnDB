#!/usr/bin/env python3
"""Import NaOnDB GitHub issues exactly once, preserving dependencies and labels."""

import json
import os
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REPO = os.environ.get("GH_REPO", "ChrisDelClea/NaOnDB")


def gh(*args: str) -> str:
    return subprocess.check_output(["gh", *args], cwd=ROOT, text=True).strip()


def main() -> None:
    issues = json.loads((ROOT / "planning/issues.json").read_text(encoding="utf-8"))
    existing = json.loads(gh("issue", "list", "--repo", REPO, "--state", "all", "--limit", "500", "--json", "title,number"))
    by_title = {issue["title"]: issue["number"] for issue in existing}
    titles = {issue["key"]: issue["title"] for issue in issues}
    numbers = {}

    areas = sorted({issue["area"] for issue in issues})
    for name, color in [("priority:P0", "b60205"), ("priority:P1", "d93f0b"), ("priority:P2", "fbca04")]:
        gh("label", "create", name, "--repo", REPO, "--color", color, "--force")
    for area in areas:
        gh("label", "create", f"area:{area}", "--repo", REPO, "--color", "6f42c1", "--force")

    for issue in issues:
        dependencies = issue["depends"]
        missing = [key for key in dependencies if key not in numbers]
        if missing:
            raise RuntimeError(f"Dependencies not created or resolved: {missing}")
        if issue["title"] in by_title:
            numbers[issue["key"]] = by_title[issue["title"]]
            print(f"Already exists: #{numbers[issue['key']]} {issue['title']}")
            continue
        lines = ["## Objective", "", issue["goal"], "", "## Acceptance criteria", ""]
        lines += [f"- [ ] {criterion}" for criterion in issue["criteria"]]
        lines += ["", "## Dependencies", ""]
        lines += ([f"- #{numbers[key]} — {titles[key]}" for key in dependencies] if dependencies else ["None; can start immediately."])
        lines += ["", "*Generated from `planning/issues.json`; update source and issue together when scope changes.*"]
        priority = issue["title"].split("]", 1)[0].removeprefix("[")
        url = gh(
            "issue", "create", "--repo", REPO,
            "--title", issue["title"], "--body", "\n".join(lines),
            "--label", f"priority:{priority}", "--label", f"area:{issue['area']}",
        )
        numbers[issue["key"]] = int(url.rsplit("/", 1)[-1])
        print(f"Created: {url}")

    print(f"Processed {len(issues)} issues in {REPO}.")


if __name__ == "__main__":
    main()
