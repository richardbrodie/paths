mod dijkstra;
mod graph;

pub const START: usize = 9441935581; // richard's house
pub const END: usize = 287674100; // anna & aidin's house

pub use dijkstra::route;
pub use graph::{AdjacentList, Edge, ExtractedOsm, Graph, Node, load_graph};

#[cfg(test)]
mod tests {
    use crate::{
        Edge, Graph, Node,
        dijkstra::route,
        graph::{AdjacentList, load_graph},
    };

    pub fn test_graph() -> impl Graph {
        let nodes = vec![
            Node {
                id: 0,
                ..Default::default()
            },
            Node {
                id: 1,
                ..Default::default()
            },
            Node {
                id: 2,
                ..Default::default()
            },
            Node {
                id: 3,
                ..Default::default()
            },
            Node {
                id: 4,
                ..Default::default()
            },
        ];
        let edges = vec![
            Edge {
                ends: [0, 1],
                weight: 4.0,
            },
            Edge {
                ends: [0, 2],
                weight: 8.0,
            },
            Edge {
                ends: [1, 2],
                weight: 3.0,
            },
            Edge {
                ends: [1, 4],
                weight: 6.0,
            },
            Edge {
                ends: [2, 3],
                weight: 2.0,
            },
            Edge {
                ends: [4, 3],
                weight: 10.0,
            },
        ];

        AdjacentList::new(nodes, edges)
    }

    #[test]
    fn short_route() {
        let graph = test_graph();
        let start = 0;
        let end = 3;
        let path = vec![0, 1, 2, 3];

        let result = route(&graph, start, end).unwrap();

        assert_eq!(result, path);
    }

    #[test]
    #[ignore = "delete when implemented"]
    fn long_route() {
        let graph = load_graph();
        let start = crate::START;
        let end = crate::END;
        let path_length = 59;

        let result = route(&graph, start, end).unwrap();

        assert_eq!(result.len(), path_length);
    }
}
