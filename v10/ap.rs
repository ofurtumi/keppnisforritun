use std::io::{self, BufRead};

fn main() {
    let lines: Vec<i64> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().trim().parse().unwrap())
        .collect();

    for line in lines {
        let mut sum = 1;
        let upper = (line as f64).sqrt().floor() as i64;
        for div in 2..=upper {
            if line % div == 0 {
                if div != (line / div) {
                    sum = sum + div + (line / div);
                } else {
                    sum = sum + div;
                }
            }
        }

        print!("{line} ");

        if sum == line {
            println!("perfect");
        } else if line - sum <= 2 && line - sum >= -2 {
            println!("almost perfect");
        } else {
            println!("not perfect");
        }
    }
}
