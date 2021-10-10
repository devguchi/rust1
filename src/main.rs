use std::io;

fn main() {
    let n = get_input_i64()[0] as f64;
    println!("{}", (n/1000.0).ceil()*1000.0-n);
}

fn get_input_i64() -> Vec<i64> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.parse().unwrap()).collect()
}
