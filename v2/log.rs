use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines: Vec<String> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .into_iter()
        .rev()
        .collect();

    let x = lines.pop().unwrap();
    let n = x.parse::<i32>().unwrap();
    for _i in 0..n {
        let t = lines.pop().unwrap().parse::<u32>().unwrap();
        let comp_string: String = lines.pop().unwrap();
        println!("{comp_string}:");
        let comp_len = comp_string.replace(" ", "").len();
        let mut song_fits: Vec<String> = Vec::new();
        for _j in 0..t {
            let temp_string: String = lines.pop().unwrap();
            if temp_string.replace(" ", "").len() == comp_len {
                song_fits.push(temp_string);
            }
        }
        song_fits.sort();
        for song in song_fits {
            println!("{song}");
        }
    }
}
