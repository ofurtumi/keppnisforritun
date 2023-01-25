use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    let n = lines[0].parse::<usize>().unwrap();
    let mut ints:Vec<i32> = lines[1]
        .split(" ")
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
    ints.sort();
    ints.reverse();

    let (mut a, mut b) = (0, 0);
    for i in 0..n {
        if i % 2 == 0 {
            a += ints[i];
        } else {
            b += ints[i];
        }
    }

    println!("{a} {b}");
}
