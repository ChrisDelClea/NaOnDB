# NaOnDB

**NaOnDB** — **Na**tive **On**tology Graph **DB**.

An ontology-first, Rust-native embedded graph database built directly on **redb**.
A general-purpose knowledge graph for enterprise ontologies, observability, data integration,
and agent memory. Agent memory is an optional ontology package, not a second database.

> **Bootstrap / pre-alpha.** The implemented storage slice is an ID-encoded quad index.
> Term interning, native RDF 1.2 persistence, ontology enforcement, temporal history/queries,
> SPARQL, BM25 and vector search remain implementation work. Model types and design documents
> are contracts, not claims that these features already work.

## Design commitments

- **One durable engine:** redb; no required secondary graph, vector or relational database.
- **RDF 1.2 plus quads:** named graphs and triple terms/reifiers are complementary.
- **Ontology-native:** explicit identities, versioned vocabularies, RDFS/OWL semantics,
  and SHACL constraints. Validation does not prove that a claim is true.
- **Multitemporal:** system-managed commit history plus independent valid, event, observed,
  published and custom domain time axes. Not a fixed bitemporal schema.
- **Claims are not facts:** reifying a proposition does not assert it. Separate observations
  retain their own provenance, stance and time coordinates.
- **Unified retrieval:** graph, temporal, BM25 and vector retrieval over the same durable state.
  No mandatory LLM, embedding service or external search database.

## Repository map

| Path | Responsibility |
| --- | --- |
| `src/model/` | RDF vocabulary types, IDs, assertions and temporal coordinates |
| `src/storage/` | redb bootstrap and four ordered quad indexes |
| `src/ontology.rs` | Ontology/validation boundary, not yet implemented |
| `src/query.rs` | Graph, SPARQL and temporal query boundary, not yet implemented |
| `src/search.rs` | Lexical/vector/hybrid retrieval boundary, not yet implemented |
| `ontologies/` | Core vocabulary, optional memory vocabulary and sample SHACL shapes |
| `docs/` | Architecture, ADRs and delivery order |
| `planning/` | Prioritized issues, dependencies and acceptance criteria |
| `tests/`, `examples/` | Bootstrap checks and a minimal embedded example |
| `.github/workflows/` | Formatting, Clippy, tests and example execution |

Read [AGENTS.md](AGENTS.md), [the architecture](docs/architecture.md),
[the roadmap](docs/roadmap.md) and [the implementation backlog](planning/ISSUES.md).
The live tracker is [GitHub Issues](https://github.com/ChrisDelClea/NaOnDB/issues).

## Development

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo run --example embedded
```

The example uses illustrative numeric IDs. It is not yet a public RDF ingestion API.
Start with the term dictionary, then atomic quad operations and the assertion ledger.

## References and inspiration

- [Netflix ontology-driven observability](https://netflixtechblog.com/ontology-driven-observability-building-an-end-to-end-knowledge-graph-at-netflix-scale-421392cf5adf)
- [W3C RDF 1.2 Concepts](https://www.w3.org/TR/rdf12-concepts/)
- [redb](https://github.com/cberner/redb)
- [OxRDF](https://docs.rs/oxrdf/0.3.4/oxrdf/)
- [LightMem](https://github.com/zjunlp/LightMem)

LightMem-family consolidation and event/evidence ideas, Hindsight-style retrieval and
Graphiti-style knowledge evolution are inspiration, not dependencies or promises of feature parity.

## Project status

No project license has been selected. Crate publication is disabled (`publish = false`).
The helper scripts in `scripts/` are optional maintainer tools; they are not needed to use the crate.
