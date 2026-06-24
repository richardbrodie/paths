use std::time::{SystemTime, UNIX_EPOCH};

use paths::{END, START, load_graph, route};

fn main() {
    // initialise the graph
    let graph = load_graph();

    // call the function you will write, passing a reference to graph
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let iterations = 10;
    for _ in 0..iterations {
        route(&graph, START, END).unwrap();
    }
    let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!(
        "avg time for dijkstra: {}",
        (end_time - start_time).as_millis() / iterations
    )
}
