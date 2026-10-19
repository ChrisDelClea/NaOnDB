//! redb-native persistence. This initial slice accepts ONLY *pre-interned IDs*.
//! Next: term dictionary -> atomic assertion ledger -> temporal projections.

mod schema;

use std::path::Path;

use redb::{Database, Error, ReadableDatabase, ReadableTableMetadata};

use crate::model::ids::{GraphId, TermId};
use crate::model::quad::{EncodedQuad, IndexOrder};
use crate::storage::schema::{GOSP, GPOS, GSPO, SPOG};

pub struct GraphDb {
    inner: Database,
}

impl GraphDb {
    pub fn create(path: impl AsRef<Path>) -> Result<Self, Error> {
        let inner = Database::create(path)?;
        let txn = inner.begin_write()?;
        {
            txn.open_table(GSPO)?;
            txn.open_table(GPOS)?;
            txn.open_table(GOSP)?;
            txn.open_table(SPOG)?;
        }
        txn.commit()?;
        Ok(Self { inner })
    }

    /// Bootstrap-only operation. It does NOT check RDF types or record temporal history.
    /// Use only with genuine, stable, dictionary-assigned term IDs.
    pub fn insert_quad_ids(&self, quad: EncodedQuad) -> Result<(), Error> {
        let txn = self.inner.begin_write()?;
        {
            let mut table = txn.open_table(GSPO)?;
            table.insert(quad.key(IndexOrder::Gspo), 1_u8)?;
        }
        {
            let mut table = txn.open_table(GPOS)?;
            table.insert(quad.key(IndexOrder::Gpos), 1_u8)?;
        }
        {
            let mut table = txn.open_table(GOSP)?;
            table.insert(quad.key(IndexOrder::Gosp), 1_u8)?;
        }
        {
            let mut table = txn.open_table(SPOG)?;
            table.insert(quad.key(IndexOrder::Spog), 1_u8)?;
        }
        txn.commit()?;
        Ok(())
    }

    pub fn contains_quad_ids(&self, quad: EncodedQuad) -> Result<bool, Error> {
        let txn = self.inner.begin_read()?;
        let table = txn.open_table(GSPO)?;
        let exists = table.get(quad.key(IndexOrder::Gspo))?.is_some();
        Ok(exists)
    }

    pub fn quad_count(&self) -> Result<u64, Error> {
        let txn = self.inner.begin_read()?;
        let table = txn.open_table(GSPO)?;
        Ok(table.len()?)
    }

    /// Outbound (predicate, object) pairs for a graph+subject prefix, ordered by ID.
    /// This is a low-level index demo, not a semantic SPARQL traversal.
    pub fn outgoing_ids(
        &self,
        graph: GraphId,
        subject: TermId,
    ) -> Result<Vec<(TermId, TermId)>, Error> {
        let mut from = [0_u8; 32];
        let mut until = [0xff_u8; 32];
        from[0..8].copy_from_slice(&graph.0.to_be_bytes());
        from[8..16].copy_from_slice(&subject.0.to_be_bytes());
        until[0..8].copy_from_slice(&graph.0.to_be_bytes());
        until[8..16].copy_from_slice(&subject.0.to_be_bytes());

        let txn = self.inner.begin_read()?;
        let table = txn.open_table(GSPO)?;
        let mut neighbors = Vec::new();
        for entry in table.range(from..=until)? {
            let (key, _) = entry?;
            let encoded = key.value();
            let predicate = TermId(u64::from_be_bytes(
                encoded[16..24].try_into().expect("fixed key width"),
            ));
            let object = TermId(u64::from_be_bytes(
                encoded[24..32].try_into().expect("fixed key width"),
            ));
            neighbors.push((predicate, object));
        }
        Ok(neighbors)
    }
}
