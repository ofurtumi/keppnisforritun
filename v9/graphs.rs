use std::cmp::Reverse;
use std::collections::{vec_deque, BinaryHeap, HashSet, VecDeque};
use std::io;

#[derive(Clone)]
struct Graph {
    edges: Vec<Vec<(usize, usize)>>,
    visited: Vec<i64>,
}

impl Graph {
    pub fn new(v: usize) -> Self {
        Self {
            edges: vec![Vec::with_capacity(v); v],
            visited: vec![-1; v],
        }
    }

    /// # add undirected edge
    /// does what the name implies,
    /// adds an edge from vertex `u` to vertex `v`
    /// ## params:
    /// - u: vertex connected to edge
    /// - v: vertex connected to edge
    /// - w: weight of the edge
    pub fn add_undirected_edge(&mut self, u: usize, v: usize, w: usize) {
        self.edges[u].push((v, w));
        self.edges[v].push((u, w));
    }

    /// # add directed edge
    /// does what the name implies,
    /// adds a directed edge from vertex `u` to vertex `v`
    /// ## params:
    /// - u: vertex from where the edge originates
    /// - v: vertex where the edge will end up
    /// - w: weight of the edge
    pub fn add_directed_edge(&mut self, u: usize, v: usize, w: usize) {
        self.edges[u].push((v, w));
    }

    /// # breadth first search
    /// simple bfs implimentation, used to calculate minimum spanning tree
    pub fn bfs(&mut self) {
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(1);
        self.visited[1] = 0;

        while !q.is_empty() {
            let branch = q.pop_front().unwrap();
            let mut available: Vec<(usize, usize)> = self.edges[branch]
                .clone()
                .into_iter()
                .filter(|&edge| self.visited[edge.0].is_negative())
                .collect();

            available.sort_by(|a, b| a.cmp(&b));

            for i in available {
                let leaf = i.0;
                if self.visited[leaf] == -1 {
                    q.push_back(leaf);
                    self.visited[leaf] = self.visited[branch] + i.1 as i64;
                }
            }
        }
    }

    pub fn topological_inc(&self) -> Option<Vec<usize>> {
        let mut in_deg: Vec<i64> = vec![0; self.edges.len()];
        for i in &self.edges {
            for edge in i {
                in_deg[edge.0] += 1;
            }
        }

        let mut q = BinaryHeap::new();
        for (i, deg) in in_deg.iter().enumerate() {
            if *deg == 0 {
                q.push(Reverse(i));
            }
        }

        let mut count: usize = 0;
        let mut top_order: Vec<usize> = Vec::new();

        while !q.is_empty() {
            let u = q.pop().unwrap();
            top_order.push(u.0);

            for (edge, weight) in &self.edges[u.0] {
                in_deg[*edge] -= 1;
                if in_deg[*edge] == 0 {
                    q.push(Reverse(*edge));
                }
            }

            count += 1;
        }
        if count != self.edges.len() {
            return None;
        }
        return Some(top_order);
    }

    pub fn topological_dec(&self) -> Option<Vec<usize>> {
        let mut in_deg: Vec<i64> = vec![0; self.edges.len()];
        for i in &self.edges {
            for edge in i {
                in_deg[edge.0] += 1;
            }
        }

        let mut q = BinaryHeap::new();
        for (i, deg) in in_deg.iter().enumerate() {
            if *deg == 0 {
                q.push(i);
            }
        }

        let mut count: usize = 0;
        let mut top_order: Vec<usize> = Vec::new();

        while !q.is_empty() {
            let u = q.pop().unwrap();
            top_order.push(u);

            for (edge, weight) in &self.edges[u] {
                in_deg[*edge] -= 1;
                if in_deg[*edge] == 0 {
                    q.push(*edge);
                }
            }

            count += 1;
        }
        if count != self.edges.len() {
            return None;
        }
        return Some(top_order);
    }
}

fn read_input_line() -> Vec<usize> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok();
    buffer
        .trim()
        .split(" ")
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn main() {
    let header = read_input_line();
    let mut g = Graph::new(header[0]);
    for _i in 0..header[1] {
        let edge = read_input_line();
        g.add_directed_edge(edge[0], edge[1], 1);
    }

    let top = g.topological_inc().unwrap();
    let bot = g.topological_dec().unwrap();

    if top != bot {
        println!("back to the lab");
    } else {
        for i in top {
            print!("{i} ");
        }
    }
}
