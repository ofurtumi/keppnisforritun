use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok();
    let n: f64 = buffer.trim().parse().unwrap();

    let mut xn = match n {
        0.0..=3.0 => 1.5,
        4.0..=8.0 => 2.0,
        9.0..=26.0 => 2.5,
        27.0..=79.0 => 3.0,
        80.0..=255.0 => 3.5,
        256.0..=868.0 => 4.0,
        869.0..=3124.0 => 4.5,
        3125.0..=11802.0 => 5.0,
        11803.0..=46655.0 => 5.5,
        46656.0..=192280.0 => 6.0,
        192281.0..=823543.0 => 6.5,
        823543.0..=3655605.0 => 7.0,
        3655606.0..=16777215.0 => 7.5,
        16777216.0..=79443956.0 => 8.0,
        79443957.0..=387420488.0 => 8.5,
        387420489.0..=1942559142.0 => 9.0,
        1942559143.0..=9999999999.0 => 9.5,
        _ => 8.5,
    };

    let mut old = 0.0;
    let mut count = 0;
    while check_diff(n, xn) {
        old = xn;
        xn = newton(xn, n);
        if xn == old {
            count += 1;
            if count > 5 {
                break;
            }
        }
    }
    let new = oldton(n, 1e-7);
    println!("{xn}");
    println!("{new}");
}

fn check_diff(n1: f64, n2: f64) -> bool {
    let xx = n2.powf(n2);
    return (n1 - xx).abs() > 1e-7;
}

fn newton(xn: f64, n: f64) -> f64 {
    let top = xn.powf(xn) - n;
    let bot = xn.powf(xn) * (xn.ln() + 1.0);
    return xn - (top / bot);
}

fn oldton(n: f64, eps: f64) -> f64 {
    let mut start = 0.0;
    let mut end = n;
    while true {
        let half = start + (end - start) / 2.0;
        let pow = n.powf(1.0 / half);
        if pow + eps < half {
            end = half;
        } else if half < pow - eps {
            start = half;
        } else {
            return half;
        }
    }
    return 0.0;
}
