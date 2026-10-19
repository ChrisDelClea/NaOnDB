//! Multitemporal coordinates for assertion instances, independent of RDF named graphs.
//!
//! UTC Unix timestamps in microseconds. Time intervals are half-open [from, until).
//! CommitId is separate from timestamps; time axes are independently defined.

use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeAxisId(pub u64);

/// System-managed; applications may not supply this axis as an ordinary observation.
pub const TRANSACTION_AXIS: TimeAxisId = TimeAxisId(0);
pub const VALID_AXIS: TimeAxisId = TimeAxisId(1);
pub const EVENT_AXIS: TimeAxisId = TimeAxisId(2);
pub const OBSERVED_AXIS: TimeAxisId = TimeAxisId(3);
pub const PUBLISHED_AXIS: TimeAxisId = TimeAxisId(4);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimestampMicros(pub i64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeInterval {
    from: Option<TimestampMicros>,
    until: Option<TimestampMicros>,
}

impl TimeInterval {
    pub fn new(
        from: Option<TimestampMicros>,
        until: Option<TimestampMicros>,
    ) -> Result<Self, TemporalError> {
        if matches!((from, until), (Some(start), Some(end)) if start >= end) {
            return Err(TemporalError::EmptyOrReversedInterval);
        }
        Ok(Self { from, until })
    }

    pub fn from(&self) -> Option<TimestampMicros> {
        self.from
    }

    pub fn until(&self) -> Option<TimestampMicros> {
        self.until
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeValue {
    Instant(TimestampMicros),
    Interval(TimeInterval),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TemporalCoordinates {
    values: BTreeMap<TimeAxisId, TimeValue>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemporalError {
    EmptyOrReversedInterval,
    SystemAxisIsManagedByDatabase,
    DuplicateAxis(TimeAxisId),
}

impl std::fmt::Display for TemporalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for TemporalError {}

impl TemporalCoordinates {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, axis: TimeAxisId, value: TimeValue) -> Result<(), TemporalError> {
        if axis == TRANSACTION_AXIS {
            return Err(TemporalError::SystemAxisIsManagedByDatabase);
        }
        if self.values.contains_key(&axis) {
            return Err(TemporalError::DuplicateAxis(axis));
        }
        self.values.insert(axis, value);
        Ok(())
    }

    pub fn get(&self, axis: TimeAxisId) -> Option<TimeValue> {
        self.values.get(&axis).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (TimeAxisId, TimeValue)> + '_ {
        self.values.iter().map(|(axis, value)| (*axis, *value))
    }
}
