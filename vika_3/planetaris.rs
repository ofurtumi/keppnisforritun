use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    let header = lines[0]
        .split(" ")
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let (n, mut ships) = (header[0], header[1]);

    let mut atli = lines[1]
        .split(" ")
        .map(|w| w.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    atli.sort();

    let mut wins = 0;
    for fleet in atli {}

    println!("{wins}");
}
