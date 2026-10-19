# NaOnDB Architecture v0 — ontology-first, not memory-first

## Contract

A single embedded redb instance is the durable source of truth. RDF 1.2 datasets (quads),
triple terms, reifiers, ontology entities, assertion histories, provenance records and auxiliary
search indices share an ACID transaction boundary. In-memory caches are rebuildable.

```text
Applications / Agents / Analytics (Rust, later Python)
                      |
        Typed Graph + RDF/SPARQL + Search APIs
                      |
              Ontology Registry
           OWL/RDFS semantics | SHACL constraints
                      |
           Query / Temporal / Search Planner
                      |
     RDF Dataset + Assertions + Provenance + Time Axes
                      |
   ID Dictionary / Quad Indices / Time Indices / Search Indices
                      |
                    redb
```

## RDF 1.2 and quads are complementary

- A **quad** `(graph, subject, predicate, object)` records an asserted triple in an RDF dataset.
- A **triple term** is an RDF term representing a proposition, permitted in **object position**
  in the RDF 1.2 abstract model, including nesting. It is not automatically asserted.
- A resource `rdf:reifies <<(s p o)>>` identifies one reification of that proposition. Distinct
  `Observation`s or `Claim`s may each reify the *same* proposition, with separate origins and times.
- A **named graph** supplies dataset membership/context; it is NOT a substitute for an assertion
  ID, provenance, an access-control label, a tenant boundary, or any temporal axis.
- Quad membership has set semantics. Empty named graphs additionally require an explicit graph
  catalog; a quad-only encoding cannot preserve them. This is tracked in the index milestone.
  Multiple sources and times are distinct claim/reifier instances, not duplicate canonical quads.

## Storage layout (target)

| redb table/index | Key | Value | Purpose |
|---|---|---|---|
| `term_to_id` | canonical RDF term encoding | `TermId` | dedup/interner |
| `id_to_term` | `TermId` | encoded RDF term | round-trip decode |
| `graphs` | `GraphId` | graph term | preserve empty named graphs |
| `quads_gspo` | `(g,s,p,o)` | membership | outbound scans |
| `quads_gpos` | `(g,p,o,s)` | membership | predicate-based filters |
| `quads_gosp` | `(g,o,s,p)` | membership | inbound scans |
| `quads_spog` | `(s,p,o,g)` | membership | cross-graph lookups |
| `assertion_versions` | `(AssertionId,CommitId)` | immutable version | status/provenance |
| `commit_ledger` | `CommitId` | commit timestamp/metadata | knowledge-as-of |
| `time_axes` | `TimeAxisId` | label/semantics/scale | axis registry |
| `time_access` | `(AxisId, time bucket/endpoint, ClaimId)` | locator | temporal search |
| `lexical_*` | token/document | postings/stats | native BM25 |
| `vector_*` | model/document | embedding/ANN nodes | vector search |
| `blob_*` | content ID | bytes/media pointer | multimodal evidence |

Only `quads_*` tables and an ID-only demonstration exist today. The others are planned.
The layout is subject to benchmark- and query-driven refinement; do not treat indexes as a
promise of cost-free range filtering for arbitrary axis combinations.

## Native multitemporal model

Two different notions must **not** be conflated:

**System history:** assigned at successful commit; use a strictly monotonic `CommitId` for
`known_as_of`. Store wall-clock commit timestamps as metadata, not ordering authorities.
Prior claim versions must remain queryable. Withdrawal is an appended revision, not history deletion.

**Domain time axes:** an extensible, typed registry: valid/effective, occurred/event, observed,
published, and future domain-specific time dimensions. Axis values may be **instant** or
**half-open interval** `[from,until)` with optional open endpoints. Normalize clock times
as UTC Unix *microseconds*. Each axis has its own meaning; do not silently project
`observed_at` to `valid_at`. The database reserves the transaction axis.

A historical query can combine:

```text
known_as_of(commit=927)     # what the database knew then
at(axis=valid, t=T)         # when it was considered valid
before(axis=observed,t=U)   # observation cut-off
between(axis=event, t0,t1) # event occurrence window
```

Snapshot semantics and the necessary multi-axis indexing must be implemented and tested
*before* advertising temporal query support. Source publication time, ingestion time and
observation time must remain distinguishable. For retroactive corrections, never rewrite the
original observation, even if the effective validity interval is corrected.

## Identity, facts and claims

Proposition identity is an interned triple term, not a quad. Graph context lives on the claim;
otherwise the same proposition would accidentally get different identities in different graphs.
An RDF proposition is not the same as a claim that it is true. Extracted hypotheses and
opinions are not automatically asserted as facts. Sources are preserved; conflicting claims
are allowed. Reifiers identify claim/observation instances and link to evidence, including
raw text, structured events and multimodal artifacts. Identity resolution is an explicit
application-defined policy—never deduplicate because names look similar.

Ontology definitions are ordinary versioned RDF data. OWL open-world inference **does not**
enforce constraints. SHACL checks must be explicit and atomic on ingestion. Provenance and
validation status may themselves be expressed via the same dataset model.

## Query and search

- Query: efficient quad-pattern seeks and traversals; then reuse Rust RDF/SPARQL components
  (`spargebra`, `spareval`, `oxrdf`, `oxrdfio`) where standards support permits.
- Search: lexical BM25, graph expansion and time restrictions; later native vector ANN. Query-time
  rank fusion, optional reranking, explainable source/provenance payloads.
- Relevance, ranking and LLMs **do not decide RDF truth**. Agents/Memory APIs are optional packages.
- Native KV API is useful for documents, metadata, and evidence alongside the graph, but must
  observe the same transactional and namespace isolation guarantees.

## Non-goals for the first release

Distributed consensus/sharding, a custom language replacing SPARQL, an LLM runtime inside the
storage layer, full OWL-DL theorem proving, and an external database requirement.

## Operational invariants

1. Redb is the sole required persistent store, one ACID boundary for all indexes.
2. Numeric term IDs are stable and never confused with RDF vocabulary types.
3. Triple terms are propositions; asserting a quad is a *separate* operation.
4. Each recorded assertion has durable identity and system history, independent of other sources.
5. Domain temporal axes are extendable; the system transaction axis cannot be forged.
6. Queries must not present hypotheses as independently verified facts.
7. Term dictionary, quad indexes and claim ledger update or fail together.
8. Explicit, versioned ontology and validation policy for every namespace.

## Search consistency and retention

Every retrieval branch uses one logical snapshot and the same visibility rules. Current BM25
corpus statistics or ANN candidate sets must not silently leak future knowledge into an as-of
query. Start with exact filtered baselines; optimize only against tested recall/correctness.
A rebuilt ANN cache has a watermark; stale caches require delta search or exact fallback.

History preservation is not an unconditional promise to keep personal data forever. A future
retention/erasure policy must explicitly define the history it invalidates. Never bypass that
policy with untracked cache or blob copies.
