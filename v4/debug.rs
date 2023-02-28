use std::io::{self, BufRead};

fn main() {
    let lines: Vec<i64> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<String>()
        .trim()
        .split(" ")
        .map(|c| c.parse::<i64>().unwrap())
        .collect();

    print!("{}", debug(lines[0], lines[1], lines[2]));
}

fn debug(n: i64, r: i64, p: i64) -> i64 {
    let mut n_mut: i64 = n;
    let mut count = 0;

    if n <= 1 {
        return 0;
    }

    while n_mut - 1 > r && n_mut > 2 {
        count += r + p;
        n_mut /= 2;
    }
    count += ((n_mut - 1) * p) + r;

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given() {
        assert_eq!(debug(1, 100, 20), 0);
        assert_eq!(debug(10, 10, 1), 19);
        assert_eq!(debug(16, 1, 10), 44);
    }

    #[test]
    fn test_gpt() {
        assert_eq!(debug(10, 3, 5), 24);
        assert_eq!(debug(100, 4, 6), 66);
        assert_eq!(debug(8, 2, 7), 27);
    }
}
