mod dijkstra;
mod graph;

pub const OSM_START: usize = 9441935581; // richard's house
pub const OSM_END: usize = 287674100; // anna & aidin's house
pub const TEST_START: usize = 0;
pub const TEST_END: usize = 3;

pub use dijkstra::route;
pub use graph::{AdjacentList, Edge, ExtractedOsm, Graph, Node, osm_graph, test_graph};

#[cfg(test)]
mod tests {
    use crate::{OSM_END, OSM_START, TEST_END, TEST_START, dijkstra::route, osm_graph, test_graph};

    #[test]
    fn short_route() {
        let graph = test_graph();
        let path = vec![0, 1, 2, 3];

        let result = route(&graph, TEST_START, TEST_END).unwrap();

        assert_eq!(result, path);
    }

    #[test]
    #[ignore = "delete when implemented"]
    fn long_route() {
        let graph = osm_graph();
        let path_length = 59;

        let result = route(&graph, OSM_START, OSM_END).unwrap();

        assert_eq!(result.len(), path_length);
    }
}
