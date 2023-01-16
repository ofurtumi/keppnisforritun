use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines: Vec<(i32, i32)> = 
        stdin
        .lock()
        .lines()
        .map(|l| l
            .unwrap()

            .collect());

    println!("{lines}");
}
 
