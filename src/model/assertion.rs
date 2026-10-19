//! Assertion identities carry provenance and independent time axes.
//! This is a domain contract; persistence and snapshot semantics remain planned.

use crate::model::ids::{AssertionId, CommitId, GraphId, TermId};
use crate::model::temporal::TemporalCoordinates;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssertionStance {
    /// The proposition is asserted in the selected named/default graph.
    Asserted,
    /// A proposition is referenced but not asserted as true.
    Proposed,
    /// A claim about a proposition, with no automatic truth implication.
    Disputed,
}

/// Draft record shape, not an accepted-write API or a persisted ledger record yet.
#[derive(Clone, Debug)]
pub struct AssertionRecord {
    pub id: AssertionId,
    /// Dictionary ID of an RDF 1.2 triple term, independent of graph membership.
    pub proposition: TermId,
    /// Context in which the claim is recorded; not part of proposition identity.
    pub graph: GraphId,
    /// The RDF resource that reifies this particular assertion/observation.
    pub reifier: TermId,
    pub source: Option<TermId>,
    pub stance: AssertionStance,
    pub temporal: TemporalCoordinates,
    /// Future ledger output only; eventual write APIs must not accept this from clients.
    pub first_recorded_in: Option<CommitId>,
}
