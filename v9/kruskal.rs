use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    io,
};

#[derive(Debug, Clone, Eq, Ord, PartialOrd)]
struct Edge(i64, i64);

impl Edge {
    fn is_end(&self, v: i64) -> bool {
        self.0 == v || self.1 == v
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.0, self.1) == (other.0, other.1) || (self.0, self.1) == (other.1, other.0)
    }
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.0 < self.1 {
            self.0.hash(state);
            self.1.hash(state);
        } else {
            self.1.hash(state);
            self.0.hash(state);
        }
    }
}

type Tree = HashSet<Edge>;
type Graph = HashMap<Edge, i64>;

fn is_vertex_in_tree(t: &Tree, v: i64) -> bool {
    t.iter().any(|e| e.is_end(v))
}

fn combine_trees(mut trees: Vec<Tree>, Edge(u, v): &Edge) -> Vec<Tree> {
    let mut res = vec![];
    let mut combined = Tree::from([Edge(*u, *v)]);
    loop {
        if let Some(tree) = trees.pop() {
            if is_vertex_in_tree(&tree, *v) || is_vertex_in_tree(&tree, *u) {
                combined.extend(tree.into_iter());
            } else {
                res.push(tree);
            }
        } else {
            res.push(combined);
            return res;
        }
    }
}

fn min_spanning_tree(g: &Graph) -> Tree {
    let mut trees: Vec<Tree> = vec![];
    loop {
        if let Some((e, ..)) = g
            .iter()
            .filter(|(Edge(v, u), ..)| {
                trees
                    .iter()
                    .all(|t| !is_vertex_in_tree(t, *v) || !is_vertex_in_tree(t, *u))
            })
            .min_by_key(|(_, weight)| *weight)
        {
            trees = combine_trees(trees, e);
        } else {
            return trees.pop().unwrap();
        }
    }
}

fn main() {
    loop {
        let header = read_input_line();
        if header[0] == 0 && header[1] == 0 {
            return;
        }
        solution(header[1] as usize);
    }
}

fn read_input_line() -> Vec<i64> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok();
    buffer
        .trim()
        .split(" ")
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn solution(m: usize) {
    if m == 0 {
        println!("Impossible");
        return;
    }

    let mut g = Graph::new();
    for _i in 0..m {
        let temp_edge = read_input_line();
        let (u, v, w) = (temp_edge[0], temp_edge[1], temp_edge[2]);
        if u > v {
            g.insert(Edge(v, u), w);
        } else {
            g.insert(Edge(u, v), w);
        }
    }

    let mut mst: Vec<Edge> = min_spanning_tree(&g).iter().map(|e| e.to_owned()).collect();
    mst.sort();
    let count: i64 = mst
        .iter()
        .map(|e| *g.get(e).unwrap())
        .collect::<Vec<i64>>()
        .iter()
        .sum();

    println!("{count}");
    for i in mst {
        println!("{} {}", i.0, i.1);
    }
}
