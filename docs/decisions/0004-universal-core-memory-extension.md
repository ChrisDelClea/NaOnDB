# ADR-0004: Memory/search built atop a universal ontology-native core

**Status:** Accepted as the domain boundary.

Experiences, Observations, Opinions, Decisions, Outcomes and MemorySummaries are ontological
application types, not mandatory storage primitives. Memory routines (retain, reflect,
consolidate) remain optional clients/extensions. No LLM or embedding service is required to
open/query the database.

Search is a native capability of the engine: BM25 postings, vector payloads, graph patterns,
time filters and hybrid ranking ultimately share redb-backed durable state. Embedding creation,
LLM enrichment and deep agentic retrieval can be composed externally.

Inspiration: LightMem / StructMem / FluxMem / EM²Mem, Hindsight, Graphiti, HydraDB.
