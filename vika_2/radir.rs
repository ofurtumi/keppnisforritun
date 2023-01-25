use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<(usize, usize)> = stdin
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .split(" ")
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<usize>>>()
        .iter()
        .map(|v| (v[0], v[1]))
        .collect();

    let mut hendi: [[char; 13]; 4] = [['-'; 13]; 4];
    let n: usize = lines[0].0;
    let p: usize = lines[0].1;
    let mut cnt = 0;
    let mut found = false;
    let mut out = 0;

    for h in &lines[1..] {
        cnt += 1;
        hendi[h.0 - 1][h.1 - 1] = 'x';
        let tempstr = hendi[h.0 - 1].iter().collect::<String>().contains("xxx");
        if !found && tempstr {
            out = cnt;
            found = true;
        }
        if cnt == n {
            break;
        }
    }

    if found {
        let temp_out = out as i32;
        let temp_p = p as i32;
        println!("{}", if temp_out - temp_p > 0 { out - p } else { 1 })
    } else {
        println!("neibb")
    }
}
