# ADR-0002: RDF 1.2 terms in a quad-backed dataset

**Status:** Accepted as the semantic target; RDF 1.2 is still evolving in W3C.

Store RDF dataset quads with ordered access paths, and recognize RDF 1.2 triple terms as
*objects* via `oxrdf`'s `rdf-12` feature. `rdf:reifies` connects a reifier to a proposition.

- A quoted/triple term is NOT an asserted fact.
- Graph membership, proposition identity, and assertion occurrence have different IDs.
- Two observations can reference one proposition with different source/time/provenance.
- Implement standards-compliant import/export and avoid custom RDF-star syntax shortcuts.
- Keep compatibility behavior under conformance tests against an explicit specification snapshot. The linked Concepts document is the
  W3C Candidate Recommendation Snapshot of 7 April 2026, not a final Recommendation.

References: https://www.w3.org/TR/rdf12-concepts/
