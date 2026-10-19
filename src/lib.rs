//! NaOnDB: ontology-native graph database foundation.
//!
//! Pre-alpha: only the raw quad-index storage slice is functional.
//! See `docs/architecture.md` for the targeted semantic model.

pub mod model;
pub mod ontology;
pub mod query;
pub mod search;
pub mod storage;

pub use model::ids::{GraphId, TermId};
pub use model::quad::EncodedQuad;
pub use storage::GraphDb;
