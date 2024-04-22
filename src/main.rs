mod graph;
fn main() {
    let graph = graph::Graph::new_california();
    print!("Hey{}", graph.edges.len());
}
