use std::time::{SystemTime, UNIX_EPOCH};

use paths::{END, START, load_graph, route, test_graph};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // initialise the graph
    let graph = match args.get(1) {
        Some(s) if s == "long" => load_graph(),
        _ => test_graph(),
    };

    // call the function you will write, passing a reference to graph
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    route(&graph, START, END).unwrap();
    let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("time for dijkstra: {}", (end_time - start_time).as_millis())
}
