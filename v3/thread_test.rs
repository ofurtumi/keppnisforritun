use std::thread;
use std::time::Duration;

fn main() {
    let testvec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    thread::spawn(|| {
        for i in testvec {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in testvec {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
