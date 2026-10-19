//! Internal, persistent numeric identities. They are not public RDF term identifiers.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TermId(pub u64);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GraphId(pub u64);

/// Reserved for the default RDF graph; named graphs must have assigned term IDs.
pub const DEFAULT_GRAPH: GraphId = GraphId(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AssertionId(pub u64);

/// Durable, monotonically increasing logical commit version, not wall-clock time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CommitId(pub u64);
