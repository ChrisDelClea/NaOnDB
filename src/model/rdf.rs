//! Standards-facing term model. Persistence/interner integration is tracked separately.
//!
//! OxRDF provides RDF 1.2 triple terms behind its `rdf-12` feature.
//! RDF 1.2 permits triple terms in object position (not the subject position).

pub use oxrdf::{BlankNode, GraphName, Literal, NamedNode, Quad, Term, Triple};
