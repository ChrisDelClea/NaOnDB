# NaOnDB implementation backlog

The live issues are the work tracker; `issues.json` holds the initial scope and acceptance criteria.
Keep both aligned when scope changes. Nothing below is completed merely because a skeleton exists.

| Issue | Priority | Area | Depends on |
| --- | --- | --- | --- |
| [#1 RDF 1.2 term dictionary and stable IDs](https://github.com/ChrisDelClea/NaOnDB/issues/1) | P0 | storage | None |
| [#2 Atomic quad indexes and graph pattern scans](https://github.com/ChrisDelClea/NaOnDB/issues/2) | P0 | storage | [#1](https://github.com/ChrisDelClea/NaOnDB/issues/1) |
| [#3 Durable assertion ledger and knowledge-as-of commits](https://github.com/ChrisDelClea/NaOnDB/issues/3) | P0 | temporal | [#1](https://github.com/ChrisDelClea/NaOnDB/issues/1), [#2](https://github.com/ChrisDelClea/NaOnDB/issues/2) |
| [#4 Native multitemporal axis registry and indexed queries](https://github.com/ChrisDelClea/NaOnDB/issues/4) | P0 | temporal | [#3](https://github.com/ChrisDelClea/NaOnDB/issues/3) |
| [#5 Native RDF 1.2 reifiers, quoted propositions and assertion semantics](https://github.com/ChrisDelClea/NaOnDB/issues/5) | P0 | rdf | [#1](https://github.com/ChrisDelClea/NaOnDB/issues/1), [#3](https://github.com/ChrisDelClea/NaOnDB/issues/3) |
| [#6 Ontology registry, identity policies and SHACL ingestion](https://github.com/ChrisDelClea/NaOnDB/issues/6) | P0 | ontology | [#2](https://github.com/ChrisDelClea/NaOnDB/issues/2), [#5](https://github.com/ChrisDelClea/NaOnDB/issues/5) |
| [#7 Standards RDF I/O: Turtle, N-Quads and RDF 1.2](https://github.com/ChrisDelClea/NaOnDB/issues/7) | P1 | rdf | [#2](https://github.com/ChrisDelClea/NaOnDB/issues/2), [#5](https://github.com/ChrisDelClea/NaOnDB/issues/5) |
| [#8 SPARQL and typed graph traversal API over redb](https://github.com/ChrisDelClea/NaOnDB/issues/8) | P1 | query | [#2](https://github.com/ChrisDelClea/NaOnDB/issues/2), [#4](https://github.com/ChrisDelClea/NaOnDB/issues/4), [#7](https://github.com/ChrisDelClea/NaOnDB/issues/7) |
| [#9 Native redb-backed BM25 and lexical filters](https://github.com/ChrisDelClea/NaOnDB/issues/9) | P1 | search | [#1](https://github.com/ChrisDelClea/NaOnDB/issues/1), [#3](https://github.com/ChrisDelClea/NaOnDB/issues/3) |
| [#10 Native embeddings and vector similarity without external DB](https://github.com/ChrisDelClea/NaOnDB/issues/10) | P1 | search | [#1](https://github.com/ChrisDelClea/NaOnDB/issues/1), [#3](https://github.com/ChrisDelClea/NaOnDB/issues/3) |
| [#11 Unified graph + temporal + BM25 + vector retrieval](https://github.com/ChrisDelClea/NaOnDB/issues/11) | P1 | search | [#4](https://github.com/ChrisDelClea/NaOnDB/issues/4), [#8](https://github.com/ChrisDelClea/NaOnDB/issues/8), [#9](https://github.com/ChrisDelClea/NaOnDB/issues/9), [#10](https://github.com/ChrisDelClea/NaOnDB/issues/10) |
| [#12 Versioned memory ontology and experience-to-outcome example](https://github.com/ChrisDelClea/NaOnDB/issues/12) | P1 | memory | [#5](https://github.com/ChrisDelClea/NaOnDB/issues/5), [#6](https://github.com/ChrisDelClea/NaOnDB/issues/6), [#8](https://github.com/ChrisDelClea/NaOnDB/issues/8) |
| [#13 Durable multimodal evidence references and native KV blobs](https://github.com/ChrisDelClea/NaOnDB/issues/13) | P2 | storage | [#1](https://github.com/ChrisDelClea/NaOnDB/issues/1), [#3](https://github.com/ChrisDelClea/NaOnDB/issues/3) |
| [#14 Python bindings and typed API example](https://github.com/ChrisDelClea/NaOnDB/issues/14) | P2 | api | [#8](https://github.com/ChrisDelClea/NaOnDB/issues/8), [#11](https://github.com/ChrisDelClea/NaOnDB/issues/11) |
| [#15 Correctness and recovery gates for an embedded graph store](https://github.com/ChrisDelClea/NaOnDB/issues/15) | P0 | quality | [#2](https://github.com/ChrisDelClea/NaOnDB/issues/2), [#3](https://github.com/ChrisDelClea/NaOnDB/issues/3) |

Start with **#1 → #2 → #3**; develop correctness gates (#15) alongside the storage work.
Multitemporal axes (#4), reifiers (#5) and ontology enforcement (#6) are core requirements.
Search and memory extensions build on this same graph, not a second storage architecture.

Issue numbers were assigned at bootstrap; verify live state before resuming old work.
