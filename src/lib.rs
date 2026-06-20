mod dijkstra;
mod graph;

pub const START: usize = 9441935581; // richard's house
pub const END: usize = 287674100; // anna & aidin's house

pub use dijkstra::route;
pub use graph::{Edge, Graph, Node};

#[cfg(test)]
mod tests {
    use crate::{END, Graph, START, dijkstra::route};

    #[test]
    fn short_route() {
        let graph = Graph::load();

        let result = route(&graph, START, END);

        assert!(result.is_some());
    }
}
