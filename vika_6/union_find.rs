pub struct UnionFind {
    parts: Vec<usize>,
    part_size: Vec<usize>,
}

impl UnionFind {
    /// Create a new partition table with `n` disjoint
    /// partitions numbered 0..`n`. Running time O(n).
    pub fn new(n: usize) -> UnionFind {
        let parts: Vec<usize> = (0..n + 1).collect();
        let part_size = vec![1; n + 1];

        UnionFind {
            parts: parts,
            part_size: part_size,
        }
    }

    /// Merge the partitions containing `i` and `j`.  This
    /// operation is structured such that the canonical
    /// element of the merged partition will be the
    /// canonical element of `i` in the old
    /// partition. Amortized worst-case running time
    /// O(alpha(n)) + O(alpha(m)) where n and m are the
    /// number of elements in partition `i` and `j`. This is
    /// more-or-less O(1).
    pub fn union(&mut self, i: usize, j: usize) {
        let i_leader = self.find(i);
        let j_leader = self.find(j);
        if i_leader != j_leader {
            self.part_size[i_leader] += self.part_size[j_leader];
            self.parts[j_leader] = self.parts[i_leader];
        }
    }

    /// Return a "canonical element" for the partition
    /// containing `i`. Amortized worst-case running time
    /// O(alpha(n)), which is more-or-less O(1). This running
    /// time is achieved by mutating the data structure to
    /// make future `find()` operations faster.
    pub fn find(&mut self, i: usize) -> usize {
        let mut p = i;
        while self.parts[p] != p {
            p = self.parts[p]
        }
        let mut s = i;
        while s != p {
            let t = self.parts[s];
            self.parts[s] = p;
            s = t
        }
        p
    }

    pub fn get_size(&mut self, i: usize) -> usize {
        let id = self.find(i);
        self.part_size[id]
    }
}

use std::io::{self, BufRead};
fn main() {
    let mut lines: Vec<Vec<String>> = io::stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .trim()
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    let header: Vec<usize> = lines
        .remove(0)
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut parity = UnionFind::new(header[0]);

    for line in lines {
        match line[0].as_str() {
            "t" => {
                let (a, b) = (
                    line[1].parse::<usize>().unwrap(),
                    line[2].parse::<usize>().unwrap(),
                );
                parity.union(a, b);
            }
            _ => {
                let i = line[1].parse::<usize>().unwrap();
                println!("{}", parity.get_size(i));
            }
        }
    }
}
