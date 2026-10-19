use naondb::model::temporal::{
    OBSERVED_AXIS, TRANSACTION_AXIS, TemporalCoordinates, TemporalError, TimeAxisId, TimeInterval,
    TimeValue, TimestampMicros, VALID_AXIS,
};

#[test]
fn domain_axes_are_independent_and_transaction_axis_is_reserved() {
    let mut context = TemporalCoordinates::new();
    let validity = TimeInterval::new(Some(TimestampMicros(10)), None).unwrap();
    context
        .insert(VALID_AXIS, TimeValue::Interval(validity))
        .unwrap();
    context
        .insert(OBSERVED_AXIS, TimeValue::Instant(TimestampMicros(20)))
        .unwrap();
    assert_eq!(
        context.get(OBSERVED_AXIS),
        Some(TimeValue::Instant(TimestampMicros(20)))
    );
    assert_eq!(
        context.insert(TRANSACTION_AXIS, TimeValue::Instant(TimestampMicros(20))),
        Err(TemporalError::SystemAxisIsManagedByDatabase)
    );
    assert_eq!(
        context.insert(VALID_AXIS, TimeValue::Instant(TimestampMicros(20))),
        Err(TemporalError::DuplicateAxis(VALID_AXIS))
    );
    assert_eq!(context.get(VALID_AXIS), Some(TimeValue::Interval(validity)));
}

#[test]
fn intervals_reject_empty_and_reversed_bounds() {
    for (start, end) in [(5, 5), (6, 5)] {
        assert_eq!(
            TimeInterval::new(Some(TimestampMicros(start)), Some(TimestampMicros(end))),
            Err(TemporalError::EmptyOrReversedInterval)
        );
    }
    let unbounded = TimeInterval::new(None, None).unwrap();
    assert_eq!(unbounded.from(), None);
    assert_eq!(unbounded.until(), None);
}

#[test]
fn custom_axes_are_not_limited_to_two_timestamps() {
    let mut context = TemporalCoordinates::new();
    let custom_axis = TimeAxisId(100);
    context
        .insert(custom_axis, TimeValue::Instant(TimestampMicros(30)))
        .unwrap();
    assert_eq!(
        context.iter().collect::<Vec<_>>(),
        vec![(custom_axis, TimeValue::Instant(TimestampMicros(30)))]
    );
}
