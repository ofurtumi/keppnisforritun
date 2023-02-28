use std::io::{self, BufRead};

fn main() {
    let lines: Vec<Vec<i64>> = io::stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .trim()
                .split(" ")
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    println!("{lines:?}");
}

pub struct union_find {
    p: Vec<i64>,
    rank: Vec<i64>,
    size: Vec<i64>,
    num_sets: i64,
}

impl union_find {
    pub fn new() -> union_find {
        union_find { 
            p: Vec<i64>::new(), 
            rank: Vec<i64>::new(), 
            size: Vec<i64>::new(), 
            num_sets: i64
        }
    }
}
