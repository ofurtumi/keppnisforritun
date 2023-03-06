use std::io::{self, BufRead};

fn main() {
    let mut header = String::new();
    io::stdin().read_line(&mut header).ok();
    let n_k: Vec<u32> = header
        .split(" ")
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect();

    let mut lines: Vec<usize> = io::stdin()
        .lock()
        .lines()
        .map(|l| usize::from_str_radix(l.unwrap().trim(), 2).unwrap())
        .collect();

    let base: usize = 2;
    let k = n_k[1];
    let power: usize = base.pow(k);
    let mut distance = vec![21; power];

    for line in lines.iter() {
        distance[*line] = 0;
    }

    let mut line_count = lines.len();

    let mut top = 0;
    while line_count > top {
        let temp = lines[top];
        top += 1;
        for i in 0..k {
            let y = temp ^ (1 << i);
            if distance[temp] + 1 < distance[y] {
                lines.push(y);
                line_count += 1;
                distance[y] = distance[temp] + 1;
            }
        }
    }
    let char_max: usize = *distance.iter().max().unwrap();
    let pos_max = distance.iter().position(|p| p == &char_max).unwrap();

    println!("{:0>1$b}", pos_max, k as usize);
}
