mod falling;

use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    let header_values: Vec<i32> = lines[0].split(" ").map(|c| c.parse::<i32>().unwrap()).collect();
    let w: i32 = header_values[0];
    // let p: i32 = header_values[1];
    let mut output_values: HashSet<i32> = HashSet::new();
    let mut input_values: Vec<i32> = Vec::new();
    input_values.push(0);

    let temp: Vec<i32> = lines[1].split(" ").map(|c| c.parse::<i32>().unwrap()).collect();
    for x in temp {
        input_values.push(x);
    }

    let input_inner = input_values.clone();

    for i in 0..input_values.len() {
        for j in i..input_inner.len() {
            output_values.insert(w - (input_values[i]));
            output_values.insert(input_values[i]);

            output_values.insert(input_values[j]-input_values[i]);
        }
    }
    let mut final_out = output_values.into_iter().collect::<Vec<_>>();
    final_out.sort();
    print!("{}", final_out[1]);
    for out in &final_out[2..] {
        print!(" {out}");
    }
}
