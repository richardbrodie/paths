use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
};

use serde::{Deserialize, Serialize};

use crate::{
    Edge, Node,
    graph::{PREPARED_PATH, SMALL_RAW_PATH, haversine},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct OsmElement {
    pub r#type: String,
    pub id: usize,
    #[serde(default)]
    pub lat: Option<f64>,
    #[serde(default)]
    pub lon: Option<f64>,
    #[serde(default)]
    pub nodes: Option<Vec<usize>>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsmData {
    pub elements: Vec<OsmElement>,
}

impl OsmData {
    pub fn load(path: &str) -> Self {
        let json_str = std::fs::read_to_string(path).unwrap();
        serde_json::from_str(&json_str).unwrap()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExtractedOsm {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}
impl ExtractedOsm {
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
            if elem.r#type == "node" {
                // make a node object
                let node = Node {
                    lat: elem.lat.unwrap(),
                    lon: elem.lon.unwrap(),
                    id: elem.id,
                };
                nodes.push(node);
            }
        }

        // secondly, extract the ways and generate edges
        for elem in raw.elements.iter() {
            if elem.r#type == "way" {
                let way_nodes = elem.nodes.as_ref().unwrap();
                for n in 0..way_nodes.len() - 1 {
                    let this_id = way_nodes[n];
                    let this = nodes.iter().find(|w| w.id == this_id).unwrap();
                    let next_id = way_nodes[n + 1];
                    let next = nodes.iter().find(|w| w.id == next_id).unwrap();

                    let weight = haversine(this.lat, this.lon, next.lat, next.lon);

                    let edge = Edge {
                        ends: [this.id, next.id],
                        weight,
                    };
                    edges.push(edge);
                }
            }
        }

        // thirdly remove all the nodes that aren't part of an edge
        let nodes = nodes
            .into_iter()
            .filter(|n| edges.iter().any(|e| e.has_end(n.id)))
            .collect();

        let g = ExtractedOsm { nodes, edges };

        let file = File::create(PREPARED_PATH).unwrap();
        let writer = BufWriter::new(file);
        simd_json::serde::to_writer(writer, &g).unwrap()
    }
}
