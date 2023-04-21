use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok();
    let m: u64 = buffer.trim().parse().unwrap();
    println!("{}", meow_factor(m));
}

fn meow_factor(n: u64) -> u64 {
    let mut out: u64 = 0;
    for i in 1..=1000 {
        let mut temp = n;
        for u in 0..9 {
            if temp % i != 0 {
                break;
            } else if u == 8 {
                out = i;
                break;
            }
            temp /= i;
        }
    }

    return out;
}
