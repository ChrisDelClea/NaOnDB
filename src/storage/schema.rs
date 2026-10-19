//! Physical index tables. Indexes are maintained atomically in one redb write transaction.

use redb::TableDefinition;

pub(crate) const GSPO: TableDefinition<[u8; 32], u8> = TableDefinition::new("quads_gspo");
pub(crate) const GPOS: TableDefinition<[u8; 32], u8> = TableDefinition::new("quads_gpos");
pub(crate) const GOSP: TableDefinition<[u8; 32], u8> = TableDefinition::new("quads_gosp");
pub(crate) const SPOG: TableDefinition<[u8; 32], u8> = TableDefinition::new("quads_spog");

// Planned: term_to_id / id_to_term, assertions / ledger, time-axis dictionary,
// temporal access paths, blobs, lexical terms/postings, vector payloads/ANN.
