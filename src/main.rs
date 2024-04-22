mod graph;
fn main() {
    let graph = graph::Graph::new_california();
    println!("{} {} {} {}", graph.avg_edge_count(), graph.shortest_path_bfs(0, 1000), graph.shortest_path_bfs_heuristic(0, 1000), graph.shortest_path_dijkstras(0, 1000));
}
