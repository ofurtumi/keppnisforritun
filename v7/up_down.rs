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

    print!("{}", up_down(lines[1].to_owned()))
}

fn up_down(data: Vec<i64>) -> String {
    let window = data.windows(3);
    for d in window {
        if d[0] < d[1] && d[1] < d[2] {
            return format!("{} {} {}", d[0], d[1], d[2]);
        }
    }
    return format!("-1");
}
