# NaOnDB

**NaOnDB** — **Na**tive **On**tology Graph **DB**.

An ontology-first, Rust-native embedded graph database built directly on **redb**.
A general-purpose knowledge graph for enterprise ontologies, observability, data integration,
and agent memory. Agent memory is an optional ontology package, not a second database.

> **Source-available; free for noncommercial use.** Personal projects, noncommercial
> research and teaching, and community contributions are welcome. Commercial use
> requires a separate written license; the current price is **EUR 20,000**.
> See [LICENSE](LICENSE), [commercial licensing](COMMERCIAL-LICENSING.md), and
> [how to contribute](CONTRIBUTING.md).

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

## License and participation

NaOnDB uses the project-specific **NaOnDB Noncommercial License 1.0** in
[LICENSE](LICENSE). You may study, use, modify, and share covered material for
its permitted noncommercial purposes, with attribution and license notices.
Private forks are allowed. Research results may be published, including through
commercial academic publishers, without relicensing independent results.

Commercial products, SaaS/hosting, paid customer work, and internal commercial
operations need a separate agreement with **Christian Klose / @ChrisDelClea**.
The current price is **EUR 20,000**; scope, term, taxes, and payment conditions
are specified in that agreement. Payment alone grants no commercial rights.
See [COMMERCIAL-LICENSING.md](COMMERCIAL-LICENSING.md).

Contributions are welcome under [CONTRIBUTING.md](CONTRIBUTING.md). Contributors
keep ownership and explicitly permit community and commercial distribution under
[Contributor Agreement 1.0](CONTRIBUTOR-AGREEMENT.md); no revenue share is promised.
Dependencies retain their own licenses. These are project-specific legal texts,
not unmodified PolyForm terms or a claim of lawyer-reviewed enforceability.

Because commercial use is restricted, NaOnDB is **source-available**, not
[OSI open source](https://opensource.org/osd). The activity determines whether
use is permitted; institutional status alone is not a commercial exemption.

## Project status

Crate publication remains disabled (`publish = false`). Adding a license does
not publish a package or change the database's pre-alpha implementation status.
The helper scripts in `scripts/` are optional maintainer tools; they are not needed to use the crate.
