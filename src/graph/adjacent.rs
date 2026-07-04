use std::collections::BTreeMap;
use std::time::SystemTime;

use crate::graph::Neighbour;
use crate::{Edge, Graph, Node};

pub struct AdjacentList {
    nodes: Vec<Node>,
    adj: BTreeMap<usize, Vec<Neighbour>>,
}

impl Graph for AdjacentList {
    fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        let start_time = SystemTime::now();
        let mut adj = BTreeMap::new();
        for e in edges.iter() {
            let [i1, i2] = e.ends;
            adj.entry(i1).or_insert(Vec::new()).push(Neighbour {
                end: i2,
                weight: e.weight,
            });
            adj.entry(i2).or_insert(Vec::new()).push(Neighbour {
                end: i1,
                weight: e.weight,
            });
        }
        let end_time = SystemTime::now();
        let duration = end_time.duration_since(start_time).unwrap().as_millis();
        println!("build graph: {}", duration,);
        Self { adj, nodes }
    }

    fn neighbours(&self, id: usize) -> Option<&Vec<Neighbour>> {
        self.adj.get(&id)
    }
    fn node_ids(&self) -> impl Iterator<Item = usize> {
        self.nodes.iter().map(|n| n.id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Graph, test_graph};

    const START: usize = 0;
    #[test]
    fn finds_neighbours() {
        let graph = test_graph();

        let result = graph.neighbours(START).unwrap();
        assert_eq!(result.len(), 2);
    }
}
