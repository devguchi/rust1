use std::io;

fn main() {
    let _ = get_input_i64()[0];
    let a = get_input_i64();
    let mut a2 = a.clone();
    a2.sort_by(|a,b| b.cmp(a));
    let score = a2[1];
    let idx = a.iter().position(|&x| x == score).unwrap();
    println!("{}", idx+1);
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

