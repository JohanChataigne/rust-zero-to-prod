use std::collections::HashMap;
use std::ops::Div;

fn main() {
    let empty: Vec<i32> = Vec::new();
    let single = vec![1];
    let pair_length = vec![2, 5, 8, -5, 0, -1, 2, 8, 2, 1];
    let impair_length = vec![2, 8, -5, 0, -1, -5, 0, 0, 4];

    display_median(&empty); // None
    display_mode(&empty); // None

    display_median(&single); // 1
    display_mode(&single); // 1

    display_median(&pair_length); // 2
    display_mode(&pair_length); // 2

    display_median(&impair_length); // 0
    display_mode(&impair_length); // 0
}

fn median(vec: &Vec<i32>) -> Option<i32> {
    if vec.is_empty() {
        return None;
    }
    if vec.len() == 1 {
        return Option::from(vec[0]);
    }

    let mut sorted_vec = Vec::new();
    sorted_vec.clone_from(vec);
    sorted_vec.sort();

    let len = sorted_vec.len();
    let median_index = len.div(2);

    match len % 2 {
        0 => Option::from((sorted_vec[median_index] + sorted_vec[median_index - 1]) / 2),
        _ => sorted_vec.get(median_index).copied(),
    }
}

fn mode(vec: &Vec<i32>) -> Option<i32> {
    if vec.is_empty() {
        return None;
    }
    if vec.len() == 1 {
        return Option::from(vec[0]);
    }

    let mut counts = HashMap::new();

    for v in vec {
        let count = counts.entry(*v).or_insert(0);
        *count += 1;
    }

    counts
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|item| item.0)
        .copied()
}

fn display_median(vec: &Vec<i32>) {
    match median(vec) {
        None => println!("No median value"),
        Some(value) => println!("Median of {:?} is {value}", vec),
    }
}
fn display_mode(vec: &Vec<i32>) {
    match mode(vec) {
        None => println!("No mode value"),
        Some(value) => println!("Mode of {:?} is {value}", vec),
    }
}
