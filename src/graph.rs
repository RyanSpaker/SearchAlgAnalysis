use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

use rand::Rng;



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

    pub fn new_random(vertices: u32, edge_count: u32) -> Self{
        let mut edges = vec![HashMap::new(); vertices as usize];
        let mut unconnected_vertices = Vec::from_iter((0..vertices).into_iter());
        let mut connected_vertices = Vec::with_capacity(vertices as usize);
        let mut rng = rand::thread_rng();
        let mut next_vert; let mut connected_vert;
        let mut dist;
        while !unconnected_vertices.is_empty() {
            next_vert = unconnected_vertices.remove(rng.gen_range(0..unconnected_vertices.len()));
            connected_vert = connected_vertices[rng.gen_range(0..connected_vertices.len())];
            dist = rng.gen_range(0.0..1.0);
            edges[next_vert as usize].insert(connected_vert, dist);
            edges[connected_vert as usize].insert(next_vert, dist);
            connected_vertices.push(next_vert);
        }
        let mut remainin_edges = edge_count-vertices;
        while remainin_edges > 0 {
            next_vert = rng.gen_range(0..vertices);
            connected_vert = rng.gen_range(0..vertices);
            dist = rng.gen_range(0.0..1.0);
            if !edges[next_vert as usize].contains_key(&connected_vert) {
                edges[next_vert as usize].insert(connected_vert, dist);
                edges[connected_vert as usize].insert(next_vert, dist);
                remainin_edges -= 1;
            }
        }
        Self{edges}
    }

    pub fn new_random_2d(vertice_count: u32, edge_count: u32) -> Self{
        let mut rng = rand::thread_rng();
        let vertices = (0..vertice_count).into_iter().map(|_| (rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0))).collect::<Vec<(f32, f32)>>();

        let mut edges = vec![HashMap::new(); vertice_count as usize];
        let mut unconnected_vertices = Vec::from_iter((0..vertice_count).into_iter());
        let mut connected_vertices = Vec::with_capacity(vertice_count as usize);
        let mut next_vert; let mut connected_vert;
        let mut dist;
        while !unconnected_vertices.is_empty() {
            next_vert = unconnected_vertices.remove(rng.gen_range(0..unconnected_vertices.len()));
            connected_vert = connected_vertices[rng.gen_range(0..connected_vertices.len())];
            dist = 
                ((vertices[next_vert as usize].0 - vertices[connected_vert as usize].0).powi(2) + 
                (vertices[next_vert as usize].1 - vertices[connected_vert as usize].1).powi(2)).sqrt();
            edges[next_vert as usize].insert(connected_vert, dist);
            edges[connected_vert as usize].insert(next_vert, dist);
            connected_vertices.push(next_vert);
        }
        let mut remainin_edges = edge_count-vertice_count;
        while remainin_edges > 0 {
            next_vert = rng.gen_range(0..vertice_count);
            connected_vert = rng.gen_range(0..vertice_count);
            dist = 
                ((vertices[next_vert as usize].0 - vertices[connected_vert as usize].0).powi(2) + 
                (vertices[next_vert as usize].1 - vertices[connected_vert as usize].1).powi(2)).sqrt();
            if !edges[next_vert as usize].contains_key(&connected_vert) {
                edges[next_vert as usize].insert(connected_vert, dist);
                edges[connected_vert as usize].insert(next_vert, dist);
                remainin_edges -= 1;
            }
        }
        Self{edges}
    }

    pub fn new_random_2d_taxicab(vertice_count: u32, edge_count: u32) -> Self{
        let mut rng = rand::thread_rng();
        let vertices = (0..vertice_count).into_iter().map(|_| (rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0))).collect::<Vec<(f32, f32)>>();

        let mut edges = vec![HashMap::new(); vertice_count as usize];
        let mut unconnected_vertices = Vec::from_iter((0..vertice_count).into_iter());
        let mut connected_vertices = Vec::with_capacity(vertice_count as usize);
        let mut next_vert; let mut connected_vert;
        let mut dist;
        while !unconnected_vertices.is_empty() {
            next_vert = unconnected_vertices.remove(rng.gen_range(0..unconnected_vertices.len()));
            connected_vert = connected_vertices[rng.gen_range(0..connected_vertices.len())];
            dist = 
                (vertices[next_vert as usize].0 - vertices[connected_vert as usize].0).abs() + 
                (vertices[next_vert as usize].1 - vertices[connected_vert as usize].1).abs();
            edges[next_vert as usize].insert(connected_vert, dist);
            edges[connected_vert as usize].insert(next_vert, dist);
            connected_vertices.push(next_vert);
        }
        let mut remainin_edges = edge_count-vertice_count;
        while remainin_edges > 0 {
            next_vert = rng.gen_range(0..vertice_count);
            connected_vert = rng.gen_range(0..vertice_count);
            dist = 
                (vertices[next_vert as usize].0 - vertices[connected_vert as usize].0).abs() + 
                (vertices[next_vert as usize].1 - vertices[connected_vert as usize].1).abs();
            if !edges[next_vert as usize].contains_key(&connected_vert) {
                edges[next_vert as usize].insert(connected_vert, dist);
                edges[connected_vert as usize].insert(next_vert, dist);
                remainin_edges -= 1;
            }
        }
        Self{edges}
    }

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