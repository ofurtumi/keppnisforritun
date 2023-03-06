use std::collections::VecDeque;
use std::io;

#[derive(Clone)]
struct Graph {
    edges: Vec<Vec<usize>>,
    visited: Vec<i64>,
    mst: Option<usize>,
}

impl Graph {
    pub fn new(v: usize) -> Self {
        Self {
            edges: vec![Vec::with_capacity(v); v],
            visited: vec![-1; v],
            mst: None,
        }
    }

    /// # add undirected edge
    /// does what the name implies,
    /// adds an edge from vertex `u` to vertex `v`
    /// ## params:
    /// - u: vertex connected to edge
    /// - v: vertex connected to edge
    pub fn add_undirected_edge(&mut self, u: usize, v: usize) {
        self.edges[u].push(v);
        self.edges[v].push(u);
    }

    /// # add directed edge
    /// does what the name implies,
    /// adds a directed edge from vertex `u` to vertex `v`
    /// ## params:
    /// - u: vertex from where the edge originates
    /// - v: vertex where the edge will end up
    pub fn add_directed_edge(&mut self, u: usize, v: usize) {
        self.edges[u].push(v);
    }

    /// # breadth first search
    /// simple bfs implimentation, used to calculate minimum spanning tree
    pub fn bfs(&mut self) {
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(1);
        self.visited[1] = 0;

        while !q.is_empty() {
            let branch = q.pop_front().unwrap();
            for i in &self.edges[branch] {
                let leaf = *i;
                if self.visited[leaf] == -1 {
                    q.push_back(leaf);
                    self.visited[leaf] = self.visited[branch] + 1;
                }
            }
        }
    }

    /// # calculate minimum spanning tree
    /// uses visitation list from bfs to calculate number of legs in mst
    ///
    /// ## returns:
    /// - mst: number of legs in the mst
    pub fn calc_mst(&mut self) -> usize {
        if let None = self.mst {
            self.mst = Some(self.visited.iter().filter(|v| **v >= 1).count());
        }
        return self.mst.unwrap();
    }
}

fn get_graph(v: usize, e: usize) -> Graph {
    let mut g = Graph {
        edges: vec![Vec::new(); v],
        visited: vec![-1; v],
        mst: None,
    };
    for _j in 0..e {
        let edge = read_input_line();
        g.edges[edge[0]].push(edge[1]);
        g.edges[edge[1]].push(edge[0]);
    }
    g
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
    let test_cases = read_input_line();

    for _i in 0..test_cases[0] {
        let header = read_input_line();
        let mut g = Graph::new(header[0] + 1);
        for _i in 0..header[1] {
            let edge = read_input_line();
            g.add_undirected_edge(edge[0], edge[1]);
        }

        g.bfs();
        println!("{}", g.calc_mst());
    }
}
