//! 32-byte ordered index keys; this is a physical storage record, not an RDF validator.

use crate::model::ids::{GraphId, TermId};

/// Only use IDs produced by the future RDF term dictionary.
/// A quad is an *asserted* triple in a particular graph, not a quotation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EncodedQuad {
    pub graph: GraphId,
    pub subject: TermId,
    pub predicate: TermId,
    pub object: TermId,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum IndexOrder {
    Gspo,
    Gpos,
    Gosp,
    Spog,
}

impl EncodedQuad {
    pub(crate) fn key(self, order: IndexOrder) -> [u8; 32] {
        let values = match order {
            IndexOrder::Gspo => [self.graph.0, self.subject.0, self.predicate.0, self.object.0],
            IndexOrder::Gpos => [self.graph.0, self.predicate.0, self.object.0, self.subject.0],
            IndexOrder::Gosp => [self.graph.0, self.object.0, self.subject.0, self.predicate.0],
            IndexOrder::Spog => [self.subject.0, self.predicate.0, self.object.0, self.graph.0],
        };
        let mut key = [0_u8; 32];
        for (index, value) in values.into_iter().enumerate() {
            key[index * 8..(index + 1) * 8].copy_from_slice(&value.to_be_bytes());
        }
        key
    }
}
