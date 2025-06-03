/*
    appellation: macros <module>
    authors: @FL03
*/

#[cfg(feature = "hash_graph")]
#[test]
fn test_macro_for_hash_graph() -> rshyper::Result<()> {
    use rshyper::HashGraph;
    let mut graph = HashGraph::<usize, usize>::undirected();

    rshyper::hypergraph! {
        graph {
            nodes {
                let v0;
                let v1 = 10;
                let v2 = 20;
                let v3 = 30;

            };
            edges {
                let e1: [v0, v1, v2];
                let e2: [v1, v2, v3];
            };
        }
    }

    assert_eq!(graph.get_node(&v0)?.weight(), &0);
    assert_eq!(graph.get_node(&v1)?.weight(), &10);
    assert_eq!(graph.get_node(&v2)?.weight(), &20);
    assert_eq!(graph.get_node(&v3)?.weight(), &30);

    assert_eq!(graph.find_order_of_edge(&e1)?, 3);
    assert_eq!(graph.find_order_of_edge(&e2)?, 3);

    Ok(())
}
