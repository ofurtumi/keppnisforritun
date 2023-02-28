use std::{
    cmp::max,
    io::{self, BufRead},
};

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

    let (n, k) = (lines[0][0], lines[0][1]);
    let sum: i64 = lines[1].iter().sum();

    let mut i = n.parse();
    let mut lowest = i64::MAX;
    while i >= 1 {
        let sum1 = lines[1][0..i].iter().sum();
        let sum2 = lines[1][i..].iter().sum();
        let temp = max(sum1, sum2);
        if temp < lowest {
            lowest = temp
        };
        i -= 1;
        println!("t1: {}", lowest);
    }
}
