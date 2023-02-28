use std::io::{self, BufRead};

fn main() {
    let mut lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().to_string())
        .collect();

    let header: Vec<usize> = lines
        .remove(0)
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let (n, _k) = (header[0], header[0]);

    let mut data: Vec<i64> = vec![0; n];
    // let mut outstring = String::new();

    for s in lines {
        let input: Vec<&str> = s.split(" ").collect();
        match input[0] {
            "F" => {
                let f = input[1].parse::<usize>().unwrap();
                data[f - 1] ^= 1;
            }
            "C" => {
                let i = input[1].parse::<usize>().unwrap();
                let j = input[2].parse::<usize>().unwrap();
                // let sum: i64 = data[i - 1..j].join("0");
                // outstring += format!("\n{}", sum).as_str();
                let val = data[i - 1..j]
                    .iter()
                    .fold(0, |acc: i64, x| if *x == 1 { acc + 1 } else { acc });
                println!("{val}");
            }
            _ => return,
        }
    }
    // print!("{}",outstring.strip_prefix("\n").unwrap());
}
