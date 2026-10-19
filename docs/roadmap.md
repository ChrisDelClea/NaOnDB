# Execution order / vertical milestones

**M0 — Correct storage substrate:** term interning, 4 atomic quad indexes, typed RDF term
validation, idempotent durable reads and traversals, corruption and recovery tests.

**M1 — RDF 1.2 + ontology:** distinguish quoted propositions from asserted statements,
reification identities, ontology registry, controlled identity resolution, SHACL validation,
RDF round trips.

**M2 — Native multitemporal:** immutable claim versions, system-managed commit ledger,
domain axis registry, snapshot/temporal queries and index design. This is a release blocker,
not a cosmetic metadata field.

**M3 — Query & retrieval:** SPARQL access paths, graph traversals, BM25, vectors, and rank
fusion with time and ontology filters.

**M4 — Domain packages:** memory ontology, LightMem-inspired consolidation and EM²Mem-style
event/evidence linkage, Python API, reference enterprise/observability examples.

The order of RDF 1.2 and temporal work may overlap; neither should become a separate database.
Implement one slice at a time and measure demonstrable capabilities, not issue counts.
