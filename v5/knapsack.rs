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

    let mut backpacks = 0;
    while backpacks < lines.len() {
        let (c, n) = (lines[backpacks][0], lines[backpacks][1] as usize);
        backpacks += 1;
        for i in 0..n {
            println!("{:?}", lines[i]);
        }
        backpacks += n;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given() {
        assert_eq!(1 > 2, false);
    }
    #[test]
    fn test_gpt() {
        assert_eq!(1 == 1, false);
    }
}
