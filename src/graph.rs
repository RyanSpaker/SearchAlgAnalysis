use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};



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

    pub fn vertex_count(&self) -> usize {self.edges.len()}
    pub fn avg_edge_count(&self) -> f32 {
        self.edges.iter().map(|map| map.len() as f32).sum::<f32>() / (self.vertex_count() as f32)
    }
    pub fn is_edge(&self, from: u32, to: u32) -> bool {
        self.edges.len() > from as usize && self.edges[from as usize].contains_key(&to)
    }
    pub fn get_edge(&self, from: u32, to: u32) -> f32{
        if !self.is_edge(from, to) {return 0.0;}
        *self.edges[from as usize].get(&to).unwrap()
    }
    

}