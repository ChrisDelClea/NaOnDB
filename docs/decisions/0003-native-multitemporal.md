# ADR-0003: Native multitemporal assertion history

**Status:** Accepted as the semantic target.

Do not hard-code the data model to only valid and transaction times. Keep a DB-managed logical
commit history for "what did we know then", and a versioned, typed registry of independently
queryable domain axes (valid, occurred, observed, published, and user extensions).

An assertion carries its own source/reifier/time coordinates. RDF named graphs are orthogonal.
Axes can be instants or half-open intervals; normalized physical storage uses UTC microseconds.
The transaction axis is system-assigned and cannot be written by an application.

Key scenarios: late-arriving observations, retroactive validity fixes, multi-source conflicting
claims, retracted hypotheses, correction while preserving original knowledge state.

Before implementing temporal search, specify edge semantics, clock assumptions, overlapping
interval behavior, index layouts and snapshot isolation. Establish history preservation tests.
