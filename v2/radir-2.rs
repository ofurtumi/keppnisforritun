use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<(u32, u32)> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().chars())
        // .collect::<Vec<char>>();
        .map(|c| {
            (
                c.next().unwrap().to_digit(10).unwrap(),
                c.next().unwrap().to_digit(10).unwrap(),
            )
        })
        .collect();

    let mut hands: Vec<(i32, i32)> = Vec::new();

    for i in lines {
        println!("{i:?}");
    }

    println!("{hands:?}");
}
