# Paths homework

## basic structure

`src/dijkstra.rs` is the file you will be working in, I've created a function called `route` that you should write your code in.

`src/lib.rs` is the main top-level file that defines this program. It's where `dijkstra.rs` is imported, and defines a couple of consts that are usable. It also includes one test that will call the `route` function above.

`src/main.rs` has the main function. Generally if a `lib.rs` file isn't included this serves the same purpose instead. It also includes the `main` entrypoint function. With the test I wrote this isn't necessarym but I've set this up so you can run your code more easily as you write it.

### other files you don't need to care too much about
`src/bin/build_graph.rs` is an executable to regenerate the graph data located at `data/prepared_graph.json`. Ignore this.

`src/graph.rs` and `src/graph` is where the code I wrote to generate and prepare the graph lives. You can largely ignore this for now, unless you're curious. Feel free to edit it if you need though.

## tips

You'll want to mostly be using `graph.neighbours(node_id)` to get all the edges connecting to a given node. If you begin with `START` and call that function you'll get two edges, meaning this node connects to two other nodes. 

An Edge looks like this:

```rust
Edge {
    a: 9441935581,
    b: 2099675083,
    weight: 5.326402878962706,
}
```

So if `current_node` is 9441935581 then in the next iteration we'll want to call `graph.neighbours(2099675083)` to see the next edges, and so on.
