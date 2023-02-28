use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    for i in solve(lines) {
        println!("{i}");
    }
}

fn solve(cookies: Vec<String>) -> Vec<i64> {
    let mut max_heap: BinaryHeap<i64> = BinaryHeap::new();
    let mut min_heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new(); // ! þarf min_heap.Reverse(i64)
    let mut out_vec: Vec<i64> = Vec::new();

    for cookie in cookies {
        if cookie == "#" {
            if max_heap.len() == min_heap.len() {
                out_vec.push(min_heap.pop().unwrap().0);
            } else {
                out_vec.push(max_heap.pop().unwrap());
            }
        } else {
            let p = cookie.parse::<i64>().unwrap();
            if max_heap.is_empty() || max_heap.peek() >= Some(&p) {
                max_heap.push(p);
            } else {
                min_heap.push(Reverse(p));
            }

            //if maxHeap has more than 1 element than minHeap
            if max_heap.len() > min_heap.len() + 1 {
                min_heap.push(Reverse(max_heap.pop().unwrap()));
            }
            //if maxHeap has less elements than minHeap
            else if max_heap.len() < min_heap.len() {
                max_heap.push(min_heap.pop().unwrap().0);
            }
        }
    }
    return out_vec;
}
