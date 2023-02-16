pub mod tildes;

use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok();
    let chars: Vec<char> = buffer.trim().chars().collect();

    println!("{}", erase(chars));
}

fn erase(chars: Vec<char>) -> String {
    let mut out_vec = vec![' '];

    for c in chars {
        if c == '<' {
            out_vec.pop();
        } else {
            out_vec.push(c);
        }
    }
    let out_string: String = out_vec.iter().collect();
    return out_string.trim().to_owned();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(erase(vec!['a', '<', 'b', 'c', '<']), "b");
    }

    #[test]
    fn test_second() {
        let second: Vec<char> = "foss<<rritun".chars().collect();
        assert_eq!(erase(second), "forritun");
    }

    #[test]
    fn test_third() {
        let third: Vec<char> = "a<a<a<aa<<".chars().collect();
        assert_eq!(erase(third), "");
    }
}
