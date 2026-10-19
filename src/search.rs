//! Planned: one redb-backed retrieval layer with native BM25, vector and graph search.
//!
//! Persistent lexical postings and vectors belong to redb. ANN may maintain a
//! reconstructible in-memory cache. Hybrid ranking includes temporal, ontology
//! and provenance filters. No external search database is required.
//! LLM-based consolidation and retrieval agents remain optional *applications*.
