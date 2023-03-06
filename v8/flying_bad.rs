use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Edge(u32, u32);

impl Edge {
    fn is_end(&self, v: u32) -> bool {
        self.0 == v || self.1 == v
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.0, self.1) == (other.0, other.1) || (self.0, self.1) == (other.1, other.0)
    }
}

impl Eq for Edge {}

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
type Graph = HashMap<Edge, u32>;

fn is_vertex_in_tree(t: &Tree, v: u32) -> bool {
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
    let mut lines: Vec<Vec<u32>> = io::stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .trim()
                .split(" ")
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let n_tests = lines.remove(0)[0];
    for _i in 0..n_tests as usize {
        let n_m = lines.remove(0);
        let mut count = 0;
        let mut g = Graph::new();
        while count < n_m[1] {
            let e_vec = lines.remove(0);
            let e = Edge(e_vec[0], e_vec[1]);
            g.insert(e, 1);
            count += 1;
        }
        println!("{}", min_spanning_tree(&g).len());
    }
}
