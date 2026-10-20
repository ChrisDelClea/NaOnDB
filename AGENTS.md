# NaOnDB engineering contract

## Read first

Read `README.md`, `docs/architecture.md`, `docs/decisions/` and the relevant live issue before
editing. New explicit operator decisions supersede stale plans. Check HEAD, worktree and active
branches; preserve unrelated work. Do not duplicate existing implementation or create a second
architecture. Keep the corresponding `planning/issues.json` entry and live issue aligned.

## Non-negotiable boundaries

1. One required durable store: redb. One write transaction for canonical records and affected
   indexes. No required vector/graph/SQL sidecar and no speculative storage-provider framework.
2. The general-purpose core knows RDF terms, graph membership, assertions, provenance,
   ontologies and independent temporal axes. Memory types live in an optional ontology.
3. RDF 1.2 triple terms and dataset quads coexist. Proposition identity, claim/reifier identity,
   graph context and commit history are distinct. Reification never implicitly asserts a fact.
4. Multitemporal means an extensible domain-axis registry plus database-managed system history.
   redb MVCC alone does not provide durable historical queries. Never replace this with two
   hard-coded timestamps or silently infer one axis from another.
5. OWL inference is not SHACL validation; passing validation is not proof of truth. Identity
   resolution must not merge entities merely because their names are similar.
6. Hybrid retrieval is a native database capability. LLM extraction, deep search and
   consolidation are optional callers outside the storage transaction and database core.

## Implementation style

Use a single crate until a real packaging boundary justifies splitting it. Keep modules small
(aim below 500 lines), imports at module scope, and prefer absolute `crate::` imports. Avoid
pass-through wrappers, mock-success APIs and unneeded traits. Mark unimplemented boundaries
honestly; do not return empty success results as a stand-in for implementation.

This is currently a scaffold. Bootstrap raw-ID APIs do not enforce RDF validity or history;
replace them through the term-dictionary work rather than treating them as a production API.
Do not claim full RDF/SHACL/OWL/SPARQL conformance without a stated, tested profile.

## Evidence before completion

Run formatting, strict Clippy, tests and the embedded example. For storage changes add focused
checks for atomicity, reopen and set/index consistency. For temporal/RDF changes verify
historical visibility and unasserted proposition semantics. Keep tests deterministic and offline;
never send repository data to external LLMs or require paid credentials in CI.

Report exact commands, observed results and remaining gaps in a PR. Do not close issues merely
because a skeleton, dependency or document exists. Do not change licensing, visibility or
repository policies without an explicit operator decision.

## Licensing decision

The owner has authorized free noncommercial use and participation, with separate paid
commercial licensing at the current EUR 20,000 price. Read `LICENSE`,
`COMMERCIAL-LICENSING.md`, and `CONTRIBUTING.md`. Preserve the source-available designation;
do not replace the project license with MIT/Apache/CC or mislabel it as standard PolyForm.
External contributions require explicit rights-holder acceptance of the versioned contributor
agreement before merge. Never accept on behalf of another author or infer retroactive rights.
The acceptance process is manual; these documents do not configure automated enforcement.
