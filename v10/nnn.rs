fn main() {
    let x: u64 = 10;
    print!("{}", encrypt(x));
}

fn encrypt(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    let temp = (n as f64).log10().floor() + 1.0;
    return n * temp as u64;
}
