use std::io;
use std::collections::HashSet;

fn main() {
    let n = get_input_i64()[0];
    let a = get_input_i64();
    let ah:HashSet<i64> = a.into_iter().collect();
    if ah.len() == n as usize {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}

fn get_input_i64() -> Vec<i64> {
    let words = get_input();
    words.iter().map(|word| word.parse().unwrap()).collect()
}

