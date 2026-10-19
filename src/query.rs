//! Planned: RDF 1.2-aware graph query and multitemporal snapshots.
//!
//! Graph match targets: GSPO/GPOS/GOSP/SPOG prefix lookups.
//! Temporal match targets: `known_as_of(CommitId)` plus independent axes
//! `at(axis, t)` / `during(axis, interval)`. No synthetic query API yet.
//! SPARQL integration should reuse existing parser/evaluator components.
