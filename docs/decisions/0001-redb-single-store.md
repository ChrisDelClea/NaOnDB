# ADR-0001: redb as the single persistent storage engine

**Status:** Accepted as an initial design decision.

- Rust-native, embedded, ACID, typed KV and ordered B+tree keys.
- Do not adopt Neo4j, Postgres, RocksDB, or Qdrant as required sidecars.
- All source-of-truth records and secondary index updates share a write transaction.
- redb permits concurrent readers but a single active writer; distributed execution is out of scope.
- A cache, ANN accelerator, or ingestion pipeline may be transient, never an independent truth store.
- A robust benchmark may justify revisiting this later, but not via an accidental second storage system.
