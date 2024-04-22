mod graph;
fn main() {
    let graph = graph::Graph::new_random(100000, 200000);
    println!("{} {}", graph.avg_edge_count(), graph.shortest_path_bfs(0, 10000));
}
