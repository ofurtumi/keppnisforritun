use std::io::{self, BufRead};

fn main() {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    let mut iarr: Vec<i64> = lines[1]
        .trim()
        .split(" ")
        .map(|c| c.parse::<i64>().unwrap())
        .collect();

    iarr.sort();

    let diff: Vec<i64> = iarr
        .windows(2)
        .map(|i| i64::pow(i[0] - i[1], 2))
        .collect::<Vec<i64>>();

    println!("{}", diff.iter().sum::<i64>());
}
