mod dijkstra;
mod graph;

pub const START: usize = 9441935581; // richard's house
pub const END: usize = 287674100; // anna & aidin's house

pub use dijkstra::route;
pub use graph::{AdjacentList, Edge, ExtractedOsm, Graph, Node, load_graph, test_graph};

#[cfg(test)]
mod tests {
    use crate::{dijkstra::route, graph::load_graph, test_graph};

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
