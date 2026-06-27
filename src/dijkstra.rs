use crate::Graph;

/// this function should run dijkstra's algorithm to calculate a route between `start` and `end`,
/// using the nodes and edges proviced by `graph`. It should return a sequence of node ids in the form
/// `[start, n1, n2, ... n99, end]`. Or `None` if no route is possible.
///
/// A note on the syntax here: the "<G>... where" part might look a bit scary. But it's basically
/// just a way of specifying that the Type-alias G is any concrete type that implements the `Graph`
/// trait. In this project `AdjacentList` is the graph datastructure, but using this syntax allows
/// future graph implementations to be passed as the `graph: &G` argument without this function
/// needing to be rewritten, saying in effect "anything that behaves like Graph is a valid argument,
/// we don't need to know the Type".
pub fn route<G>(graph: &G, start: usize, end: usize) -> Option<Vec<usize>>
where
    G: Graph,
{
    // put your code here
    // hint: use `graph.neighbours(...)` in a loop

    // replace this with your generated path
    None
}
