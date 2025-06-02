/*
    appellation: index <test>
    authors: @FL03
*/
extern crate rshyper_core as rshyper;
use rshyper::index::{EdgeId, IndexBase, IndexCursor, VertexId, VertexIndex};

#[test]
fn test_index() -> rshyper::Result<()> {
    let mut idx = IndexBase::<usize, VertexIndex>::new(1);
    assert_eq!(idx.get(), &1);
    idx.set(2);
    assert_eq!(idx.get(), &2);
    Ok(())
}

#[test]
fn test_edge_id() -> rshyper::Result<()> {
    let mut edge_id = EdgeId::<usize>::default();
    let e0 = edge_id.step()?;
    let e1 = edge_id.step()?;
    let e2 = edge_id.step()?;
    assert_eq!(e0.get(), &0);
    assert_eq!(e1.get(), &1);
    assert_eq!(e2.get(), &2);
    Ok(())
}

#[test]
fn test_vertex_id() -> rshyper::Result<()> {
    let vertex_id = VertexId::new(1);
    assert_eq!(vertex_id.get(), &1);
    Ok(())
}

#[test]
fn test_position() -> rshyper::Result<()> {
    let mut index = IndexCursor::<usize>::zero();
    // create some edge indices
    let e0 = index.next_edge()?;
    let e1 = index.next_edge()?;
    let e2 = index.next_edge()?;
    // check the edge indices
    assert_eq!(e0.get(), &0);
    assert_eq!(e1.get(), &1);
    assert_eq!(e2.get(), &2);
    // create some vertex indices
    let v0 = index.next_vertex()?;
    let v1 = index.next_vertex()?;
    let v2 = index.next_vertex()?;
    // check the vertex indices
    assert_eq!(e0.get(), v0.get());
    assert_eq!(e1.get(), v1.get());
    assert_eq!(e2.get(), v2.get());
    Ok(())
}
