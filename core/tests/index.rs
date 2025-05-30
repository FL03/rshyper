/*
    appellation: index <test>
    authors: @FL03
*/
extern crate rshyper_core as rshyper;
use rshyper::index::{EdgeId, Index, Position, VertexId, VertexIndex};

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
fn test_index() -> rshyper::Result<()> {
    let mut index = Index::<usize, VertexIndex>::new(1);
    assert_eq!(index.get(), &1);
    index.set(2);
    assert_eq!(index.get(), &2);
    Ok(())
}

#[test]
fn test_position() -> rshyper::Result<()> {
    let mut index = Position::<usize>::zero();
    let e0 = index.next_edge()?;
    let v0 = index.next_vertex()?;
    assert_eq!(e0.get(), v0.get());
    Ok(())
}
