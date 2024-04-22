use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

use rand::Rng;



/// Graph structure representing a undirected weighted graph
pub struct Graph{
    /// Adjacency list implementation of a graph
    edges: Vec<HashMap<u32, f32>>,
    /// Vertex positions. empty when created with new_random, otherwise stores the 2d plane position
    vertices: Vec<(f32, f32)>
}
impl Graph{
    /// Reads a dataset file and returns the graph built from it
    pub fn from_files(path: String, pos_path: String) -> std::io::Result<Self>{
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
        let file = File::open(pos_path)?;
        let vertices = BufReader::new(file).lines().flatten().map(|line| {
            let mut parts = line.split_whitespace().into_iter(); parts.next();
            let x = parts.next().unwrap().parse::<f32>().unwrap();
            let y = parts.next().unwrap().parse::<f32>().unwrap();
            (x, y)
        }).collect::<Vec<(f32, f32)>>();
        Ok(Self{edges, vertices})
    }
    pub fn new_california() -> Self {Self::from_files("datasets/cal.txt".to_string(), "datasets/cal_pos.txt".to_string()).unwrap()}
    pub fn new_north_america() -> Self {Self::from_files("datasets/na.txt".to_string(), "datasets/na_pos.txt".to_string()).unwrap()}
    pub fn new_san_francisco() -> Self {Self::from_files("datasets/sf.txt".to_string(), "datasets/sf_pos.txt".to_string()).unwrap()}
    pub fn new_oldenburg() -> Self {Self::from_files("datasets/old.txt".to_string(), "datasets/old_pos.txt".to_string()).unwrap()}
    pub fn new_san_joaquin() -> Self {Self::from_files("datasets/sjc.txt".to_string(), "datasets/sjc_pos.txt".to_string()).unwrap()}

    pub fn new_random(vertices: u32, edge_count: u32) -> Self{
        let mut edges = vec![HashMap::new(); vertices as usize];
        let mut unconnected_vertices = Vec::from_iter((1..vertices).into_iter());
        let mut connected_vertices = Vec::with_capacity(vertices as usize);
        connected_vertices.push(0);
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
        Self{edges, vertices: vec![]}
    }

    pub fn new_random_2d(vertice_count: u32, edge_count: u32) -> Self{
        let mut rng = rand::thread_rng();
        let vertices = (0..vertice_count).into_iter().map(|_| (rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0))).collect::<Vec<(f32, f32)>>();

        let mut edges = vec![HashMap::new(); vertice_count as usize];
        let mut unconnected_vertices = Vec::from_iter((1..vertice_count).into_iter());
        let mut connected_vertices = Vec::with_capacity(vertice_count as usize);
        connected_vertices.push(0);
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
        Self{edges, vertices}
    }

    pub fn new_random_2d_taxicab(vertice_count: u32, edge_count: u32) -> Self{
        let mut rng = rand::thread_rng();
        let vertices = (0..vertice_count).into_iter().map(|_| (rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0))).collect::<Vec<(f32, f32)>>();

        let mut edges = vec![HashMap::new(); vertice_count as usize];
        let mut unconnected_vertices = Vec::from_iter((1..vertice_count).into_iter());
        let mut connected_vertices = Vec::with_capacity(vertice_count as usize);
        connected_vertices.push(0);
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
        Self{edges, vertices}
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

    pub fn shortest_path_bfs(&self, from: u32, to: u32) -> f32 {
        // Record nodes we dont need to revisit
        let mut visited_nodes = HashSet::new();
        // Record the currently queued nodes
        let mut queued_nodes = HashSet::new(); queued_nodes.insert(from);
        // Min heap of (vertex, dist from start node) sorted by dist
        let mut queue: Vec<(u32, f32)> = vec![(from, 0.0)];
        while !queue.is_empty(){
            let cur_node = pop_heap(&mut queue);
            visited_nodes.insert(cur_node.0);
            // Go through each edge, adding it to the queue
            for (edge_vert, edge_len) in self.edges[cur_node.0 as usize].iter(){
                // Return if we have found the target node
                if *edge_vert == to {return *edge_len + cur_node.1;}
                // Continue if we already searched this node
                if visited_nodes.contains(edge_vert) {continue;}
                let cur_dist = cur_node.1 + *edge_len;
                // If we haven't queued the node yet, add it
                if !queued_nodes.contains(edge_vert) {
                    queue.push((*edge_vert, cur_dist));
                    let start_index = queue.len()-1;
                    heapify_up(&mut queue, start_index);
                    queued_nodes.insert(*edge_vert);
                }
                // Search through the queue to see if we need to update the queued value
                let mut index = 0;
                loop{
                    if queue[index].0 != *edge_vert {index += 1; continue;}
                    if queue[index].1 < cur_dist {break;}
                    queue[index].1 = cur_dist;
                    heapify_up(&mut queue, index);
                    break;
                }
            }
        }
        return -1.0;
    }
}

pub fn heapify_up(vec: &mut Vec<(u32, f32)>, mut index: usize){
    if index == 0 {return;}
    let mut parent;
    while index > 0 {
        parent = (index-1)/2;
        if vec[parent].1 < vec[index].1 {return;}
        vec.swap(parent, index);
        index = parent;
    }
}
pub fn pop_heap(vec: &mut Vec<(u32, f32)>) -> (u32, f32){
    if vec.len() == 1 {return vec.pop().unwrap();}
    let return_value = vec.swap_remove(0);
    let mut i = 0;
    loop{
        if vec.len() > i*2+2 {
            if vec[i].1 > vec[i*2+1].1 && vec[i*2+1].1 < vec[i*2+2].1{
                vec.swap(i, i*2+1);
                i = i*2+1;
                continue;
            }
            if vec[i].1 > vec[i*2+2].1 && vec[i*2+2].1 < vec[i*2+1].1{
                vec.swap(i, i*2+2);
                i = i*2+2;
                continue;
            }
        }
        if vec.len() > i*2+1 {
            if vec[i].1 > vec[i*2+1].1 {
                vec.swap(i, i*2+1);
                i = i*2+1;
                continue;
            }
        }
        break;
    }
    return return_value;
}