use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    let output = lines[0]
        .split("-")
        .map(|c| c.chars().nth(0).unwrap().to_string())
        .collect::<Vec<String>>()
        .join("");

    println!("{output}");
}
