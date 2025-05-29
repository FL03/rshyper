/*
    appellation: index <test>
    authors: @FL03
*/
extern crate rshyper_core as rshyper;
use rshyper::Error;
use rshyper::id::{EdgeId, Index, VertexId, VertexIndex};

#[test]
fn test_edge_id() -> rshyper::Result<()> {
    let mut edge_id = EdgeId::from_value(1);
    assert_eq!(edge_id.get(), &1);
    let next_edge_id = edge_id.next().ok_or(Error::InvalidIndex)?;
    assert_eq!(next_edge_id.get(), &2);
    Ok(())
}

#[test]
fn test_vertex_id() -> rshyper::Result<()> {
    let vertex_id = VertexId::from_value(1);
    assert_eq!(vertex_id.get(), &1);
    Ok(())
}

#[test]
fn test_index() -> rshyper::Result<()> {
    let mut index = Index::<usize, VertexIndex>::from_value(1);
    assert_eq!(index.get(), &1);
    index.set(2);
    assert_eq!(index.get(), &2);
    Ok(())
}
