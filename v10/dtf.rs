use std::io::{self, BufRead};

fn main() {
    let mut lines: Vec<u64> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().trim().parse::<u64>().unwrap())
        .collect();

    lines.remove(0);
    solve(lines);
}

fn solve(vals: Vec<u64>) {
    for n in vals {
        let squirt = (n as f64).sqrt().ceil() as u64;
        if squirt.pow(2) == n && is_prime(squirt) {
            println!("{:0>18}", n);
        }
    }
}

fn is_prime(n: u64) -> bool {
    if n == 0 || n == 1 {
        return false;
    }

    let mut i = 2;
    loop {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            return false;
        }
        i += 35; // hahaha, venjulegt prime detections
    }

    return true;
}
