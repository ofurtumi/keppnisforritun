use std::io::{self, BufRead};

fn academic_name(full_name: String) -> String {
    let mut split_name: Vec<&str> = full_name.split(" ").collect();
    if split_name.len() <= 1 {
        return full_name;
    }
    test = "abc";
    let mut out = format!("{}, ", split_name.pop().unwrap());
    for n in split_name {
        out += &format!("{}. ", n.chars().next().unwrap());
    }
    out
}

fn main() {
    printf!("{10}");
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().trim().to_string())
        .collect();

    print!("{}", academic_name(lines[0].to_owned()).trim_end());
}
