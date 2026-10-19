#!/usr/bin/env bash
# Publish the initial NaOnDB repository, then create its tracked issues.
# Defaults to PRIVATE to avoid making an unlicensed scaffold public unintentionally.
set -euo pipefail
cd "$(dirname "$0")/.."
repo="${GH_REPO:-ChrisDelClea/NaOnDB}"
visibility="${NAONDB_VISIBILITY:-private}"

if [[ "$visibility" != "private" && "$visibility" != "public" ]]; then
  echo "NAONDB_VISIBILITY must be private or public" >&2
  exit 1
fi
if ! command -v gh >/dev/null 2>&1; then
  echo 'Install GitHub CLI (gh) and run gh auth login first.' >&2
  exit 1
fi
gh auth status >/dev/null
if ! git rev-parse --git-dir >/dev/null 2>&1; then
  git init -b main
  git add .
  git commit -m 'Bootstrap ontology-native multitemporal redb graph engine'
fi
if gh repo view "$repo" >/dev/null 2>&1; then
  echo "Repository already exists: $repo"
  if ! git remote get-url origin >/dev/null 2>&1; then
    git remote add origin "https://github.com/$repo.git"
  fi
  git push -u origin main  # no force: refuse divergent remote history
else
  gh repo create "$repo" "--$visibility"     --description 'Ontology-native multitemporal RDF 1.2 graph database on redb'     --source=. --remote=origin --push
fi
export GH_REPO="$repo"
python3 scripts/create_issues.py
