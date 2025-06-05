/*
    appellation: macros <module>
    authors: @FL03
*/

#[cfg(feature = "hash_graph")]
#[test]
fn test_hypergraph_on_eg() -> rshyper::Result<()> {
    use rshyper::HashGraph;
    let mut graph = HashGraph::<usize, usize>::undirected();

    rshyper::hypergraph! {
        graph {
            nodes: {
                let v0;
                let v1 = 10;
                let v2 = 20;
                let v3 = 30;

            };
            edges: {
                let e1: [v0, v1, v2];
                let e2: [v1, v2, v3];
            };
        }
    }

    assert_eq!(graph.get_node(&v0)?.weight(), &<usize>::default());
    assert_eq!(graph.get_node(&v1)?.weight(), &10);
    assert_eq!(graph.get_node(&v2)?.weight(), &20);
    assert_eq!(graph.get_node(&v3)?.weight(), &30);

    assert_eq!(graph.get_edge_order(&e1)?, 3);
    assert_eq!(graph.get_edge_order(&e2)?, 3);

    Ok(())
}

#[cfg(feature = "hash_graph")]
#[test]
fn test_hypernode_on_eg() -> rshyper::Result<()> {
    use rshyper::HashGraph;
    let mut graph = HashGraph::<usize, usize>::undirected();

    rshyper::hypernode! {
        graph {
            let v0;
            let v1 = 10;
            let v2 = 20;
            let v3 = 30;
        }
    }

    assert_eq!(graph.get_node(&v0)?.weight(), &<usize>::default());
    assert_eq!(graph.get_node(&v1)?.weight(), &10);
    assert_eq!(graph.get_node(&v2)?.weight(), &20);
    assert_eq!(graph.get_node(&v3)?.weight(), &30);

    Ok(())
}

#[cfg(feature = "hash_graph")]
#[test]
fn test_hyperedge_on_hg() -> rshyper::Result<()> {
    use rshyper::HashGraph;
    let mut graph = HashGraph::<usize, usize>::undirected();

    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;

    rshyper::hyperedge! {
        graph {
            let e1: [v0, v1, v2];
            let e2: [v1, v2, v3];
        }
    }

    assert_eq!(graph.get_edge_order(&e1)?, 3);
    assert_eq!(graph.get_edge_order(&e2)?, 3);

    Ok(())
}
