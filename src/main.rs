use paths::{END, Graph, START, route};

fn main() {
    println!("hello world");

    // initialise the graph
    let graph = Graph::load();

    // call the function you will write, passing a reference to graph
    let result = route(&graph, START, END);

    // print the result
    // we're expecting something like Some(Vec<1, 2, 3, 4>)
    dbg!(result);
}
