use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok();
    let buff_arr: Vec<char> = buffer.trim().chars().collect();
    println!("{}", find_smallest_bigger(buff_arr));

    assert_eq!(
        find_smallest_bigger("218765".trim().chars().collect()),
        "251678"
    );
    assert_eq!(
        find_smallest_bigger("1234".trim().chars().collect()),
        "1243"
    );
    assert_eq!(find_smallest_bigger("4321".trim().chars().collect()), "0");
    assert_eq!(
        find_smallest_bigger("534976".trim().chars().collect()),
        "536479"
    );
}

fn find_smallest_bigger(mut arr: Vec<char>) -> String {
    let n = arr.len();
    let mut smallest = usize::MAX;
    let mut found = false;

    for i in (1..n).rev() {
        if arr[i] > arr[i - 1] {
            smallest = i;
            found = true;
            break;
        }
    }

    if !found {
        return "0".to_string();
    }

    let (smallest_comp, mut next_small) = (arr[smallest - 1], smallest);

    for i in (smallest + 1)..n {
        if arr[i] > smallest_comp && arr[i] < arr[smallest] {
            next_small = i;
        }
    }

    arr.swap(next_small, smallest - 1);
    arr[smallest..].sort();

    return arr
        .into_iter()
        .collect::<String>()
        .trim_start_matches("0")
        .to_string();
}
