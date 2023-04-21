use std::io;

fn main() {
    let n = read_input_line();

    let max: u64 = 1000;
    let mut out = 1;
    for base in 2..max {
        for pow in 2..max {
            let num: u64 = base.pow(pow);
            if num > n {
                break;
            };
            if num == n && pow > out {
                eprintln!("debug: {num}^{pow}");
                out = pow;
            }
        }
    }

    print!("{out}");
}

fn read_input_line() -> u64 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok();
    buffer.trim().parse::<u64>().unwrap()
}
