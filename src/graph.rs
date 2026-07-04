mod adjacent;
mod coordinates;
mod osm;

use std::time::SystemTime;

pub use adjacent::AdjacentList;
pub use coordinates::haversine;
pub use osm::ExtractedOsm;
use serde::{Deserialize, Serialize};

pub trait Graph {
    fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self;

    /// returns all edges connecting to a given node id
    fn neighbours(&self, id: usize) -> Option<&Vec<Neighbour>>;

    /// an iterator of this graph's nodes
    fn node_ids(&self) -> impl Iterator<Item = usize>;
}

pub fn osm_graph() -> AdjacentList {
    let start_time = SystemTime::now();
    let data = ExtractedOsm::load();
    let end_time = SystemTime::now();
    let duration = end_time.duration_since(start_time).unwrap().as_millis();
    println!("load json: {}", duration,);
    AdjacentList::new(data.nodes, data.edges)
}

pub fn test_graph() -> AdjacentList {
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

#[derive(Debug, Default)]
pub struct Neighbour {
    pub end: usize,
    pub weight: f64,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Node {
    pub lat: f64,
    pub lon: f64,
    pub id: usize,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Edge {
    pub ends: [usize; 2],
    pub weight: f64,
}
impl Edge {
    pub fn has_end(&self, e: usize) -> bool {
        self.ends[0] == e || self.ends[1] == e
    }
    pub fn other_end(&self, e: usize) -> Option<usize> {
        if self.ends[0] == e {
            Some(self.ends[1])
        } else if self.ends[1] == e {
            Some(self.ends[0])
        } else {
            None
        }
    }
}

pub const SMALL_RAW_PATH: &str = "data/small.json";
pub const _BIG_RAW_PATH: &str = "data/big.json";
pub const PREPARED_PATH: &str = "data/prepared_graph.json";
