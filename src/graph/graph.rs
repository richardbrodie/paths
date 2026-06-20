use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use serde::{Deserialize, Serialize};

use crate::graph::osm::OsmData;

pub const SMALL_RAW_PATH: &str = "data/small.json";
pub const _BIG_RAW_PATH: &str = "data/big.json";
pub const PREPARED_PATH: &str = "data/prepared_graph.json";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn load() -> Self {
        if let Ok(false) = std::fs::exists(PREPARED_PATH) {
            panic!("readable graph json not found");
        }
        let file = File::open(PREPARED_PATH).unwrap();
        let reader = BufReader::new(file);
        simd_json::serde::from_reader(reader).unwrap()
    }

    pub fn prepare() {
        let raw = OsmData::load(SMALL_RAW_PATH);
        let mut edges = vec![];
        let mut nodes = vec![];

        // first extract all the nodes
        for elem in raw.elements.iter() {
            match elem.r#type.as_ref() {
                "node" => {
                    // make a node object
                    let node = Node {
                        lat: elem.lat.unwrap(),
                        lon: elem.lon.unwrap(),
                        id: elem.id,
                    };
                    nodes.push(node);
                }
                _ => (),
            }
        }

        // secondly, extract the ways and generate edges
        for elem in raw.elements.iter() {
            match elem.r#type.as_ref() {
                "way" => {
                    let way_nodes = elem.nodes.as_ref().unwrap();
                    for n in 0..way_nodes.len() - 1 {
                        let this_id = way_nodes[n];
                        let this = nodes.iter().find(|w| w.id == this_id).unwrap();
                        let next_id = way_nodes[n + 1];
                        let next = nodes.iter().find(|w| w.id == next_id).unwrap();

                        let weight = haversine(this.lat, this.lon, next.lat, next.lon);

                        let edge = Edge {
                            a: this.id,
                            b: next.id,
                            weight,
                        };
                        edges.push(edge);
                    }
                }
                _ => (),
            }
        }

        // thirdly remove all the nodes that aren't part of an edge
        let nodes = nodes
            .into_iter()
            .filter(|n| edges.iter().any(|e| e.a == n.id || e.b == n.id))
            .collect();

        let g = Graph { nodes, edges };

        let file = File::create(PREPARED_PATH).unwrap();
        let writer = BufWriter::new(file);
        simd_json::serde::to_writer(writer, &g).unwrap()
    }

    pub fn weight(&self, n1: usize, n2: usize) -> Option<&f64> {
        self.edges
            .iter()
            .find(|e| (e.a == n1 && e.b == n2) || (e.b == n1 && e.a == n2))
            .map(|e| &e.weight)
    }

    pub fn node(&self, id: usize) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Node {
    pub lat: f64,
    pub lon: f64,
    pub id: usize,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Edge {
    pub a: usize,
    pub b: usize,
    pub weight: f64,
}

pub fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6371000.0; // Earth radius in meters
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    R * c
}
