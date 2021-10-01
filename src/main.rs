use std::io;
use std::cmp::min;

fn main() {
    let a = get_input_i64();
    let min12 = min(a[0], a[1]);
    let min34 = min(a[2], a[3]);
    println!("{}", min(min12, min34));
}

fn get_input_i64() -> Vec<i64> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.parse().unwrap()).collect()
}

