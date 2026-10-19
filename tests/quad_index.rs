use naondb::{EncodedQuad, GraphDb, GraphId, TermId};

#[test]
fn quad_insertion_is_idempotent_and_reopenable() -> Result<(), redb::Error> {
    let directory = tempfile::tempdir().expect("temporary database directory");
    let path = directory.path().join("graph.redb");
    let quad = EncodedQuad {
        graph: GraphId(1),
        subject: TermId(2),
        predicate: TermId(3),
        object: TermId(4),
    };
    {
        let db = GraphDb::create(&path)?;
        assert_eq!(db.quad_count()?, 0);
        db.insert_quad_ids(quad)?;
        db.insert_quad_ids(quad)?;
        assert_eq!(db.quad_count()?, 1);
        assert_eq!(
            db.outgoing_ids(quad.graph, quad.subject)?,
            vec![(quad.predicate, quad.object)]
        );
    }
    let reopened = GraphDb::create(&path)?;
    assert!(reopened.contains_quad_ids(quad)?);
    Ok(())
}

#[test]
fn prefix_scan_respects_named_graph_and_subject() -> Result<(), redb::Error> {
    let directory = tempfile::tempdir().expect("temporary database directory");
    let db = GraphDb::create(directory.path().join("graph.redb"))?;
    for (graph, subject, object) in [(1, 2, 11), (1, 2, 12), (2, 2, 13), (1, 3, 14)] {
        db.insert_quad_ids(EncodedQuad {
            graph: GraphId(graph),
            subject: TermId(subject),
            predicate: TermId(9),
            object: TermId(object),
        })?;
    }
    assert_eq!(
        db.outgoing_ids(GraphId(1), TermId(2))?,
        vec![(TermId(9), TermId(11)), (TermId(9), TermId(12))]
    );
    assert!(db.outgoing_ids(GraphId(3), TermId(2))?.is_empty());
    Ok(())
}
