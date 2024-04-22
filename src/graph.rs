use std::{collections::HashMap, fs::File, hash::Hash, io::{BufRead, BufReader}};



/// Graph structure representing a undirected weighted graph
pub struct Graph{
    /// Adjacency list implementation of a graph
    edges: Vec<HashMap<u32, f32>>
}
impl Graph{
    /// Reads a file and returns the graph built from it
    pub fn from_file(path: String) -> std::io::Result<Self>{
        let file = File::open(path)?;
        let lines = BufReader::new(file).lines().flatten();
        let mut edges = vec![];
        for line in lines{
            let mut parts = line.split_whitespace().into_iter(); parts.next();
            let vert_a = parts.next().unwrap().parse::<u32>().unwrap();
            let vert_b = parts.next().unwrap().parse::<u32>().unwrap();
            let dist = parts.next().unwrap().parse::<f32>().unwrap();
            edges.resize(edges.len().max(vert_a.max(vert_b) as usize + 1), HashMap::new());
            edges[vert_a as usize].insert(vert_b, dist);
            edges[vert_b as usize].insert(vert_a, dist);
        }
        Ok(Self{edges})
    }
    pub fn new_california() -> Self {Self::from_file("datasets/cal.txt".to_string()).unwrap()}
    pub fn new_north_america() -> Self {Self::from_file("datasets/na.txt".to_string()).unwrap()}
    pub fn new_san_francisco() -> Self {Self::from_file("datasets/sf.txt".to_string()).unwrap()}
    pub fn new_oldenburg() -> Self {Self::from_file("datasets/old.txt".to_string()).unwrap()}
    pub fn new_san_joaquin() -> Self {Self::from_file("datasets/sjc.txt".to_string()).unwrap()}
}