use std::time::SystemTime;

use paths::{OSM_END, OSM_START, TEST_END, TEST_START, osm_graph, route, test_graph};

fn main() {
    // did we call the program with the "long" flag?
    let args: Vec<String> = std::env::args().collect();

    // here's something readable but not very idiomatic
    let is_long = if let Some(arg) = args.get(1) {
        arg == "long"
    } else {
        false
    };
    let graph = if is_long { osm_graph() } else { test_graph() };
    let (start, end) = if is_long {
        (OSM_START, OSM_END)
    } else {
        (TEST_START, TEST_END)
    };

    // what's more idiomatic is this:
    // let (graph, start, end) = match args.get(1) {
    //     Some(s) if s == "long" => (osm_graph(), OSM_START, OSM_END),
    //     _ => (test_graph(), TEST_START, TEST_END),
    // };

    println!("long path={}, start={}, end={}", is_long, start, end);

    // call the function you will write, passing a reference to graph
    let start_time = SystemTime::now();
    route(&graph, start, end).unwrap();
    let end_time = SystemTime::now();

    let duration = end_time.duration_since(start_time).unwrap().as_millis();
    println!("calculate route: {}", duration)
}
