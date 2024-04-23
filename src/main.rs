mod graph;
fn main() {
    let graph = graph::Graph::new_random(2000, 4000);
    println!("{} {} {} {} {}", graph.avg_edge_count(), graph.shortest_path_bfs(0, 1000), graph.shortest_path_bfs_heuristic(0, 1000), graph.shortest_path_dijkstras(0, 1000), graph.shortest_path_a_star(0, 1000));
}
