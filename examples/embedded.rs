use naondb::{EncodedQuad, GraphDb, GraphId, TermId};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let db = GraphDb::create(directory.path().join("graph.redb"))?;
    let quad = EncodedQuad {
        graph: GraphId(1),
        subject: TermId(100),
        predicate: TermId(10),
        object: TermId(200),
    };
    // IDs are illustrative; real RDF terms require the not-yet-implemented dictionary.
    db.insert_quad_ids(quad)?;
    assert!(db.contains_quad_ids(quad)?);
    println!(
        "Quads: {}; Outbound: {:?}",
        db.quad_count()?,
        db.outgoing_ids(quad.graph, quad.subject)?
    );
    Ok(())
}
