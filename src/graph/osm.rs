use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
