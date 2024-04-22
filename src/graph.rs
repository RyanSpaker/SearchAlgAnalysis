use std::collections::HashMap;



/// Graph structure representing a undirected weighted graph
pub struct Graph{
    /// Adjacency list implementation of a graph
    edges: Vec<HashMap<u32, f32>>
}