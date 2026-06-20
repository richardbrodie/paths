use paths::{END, Graph, START, route};

fn main() {
    println!("hello world");

    let graph = Graph::load();
    let result = route(&graph, START, END);
    dbg!(result);
}
