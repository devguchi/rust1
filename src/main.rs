use std::io;

fn main() {
    let n = get_input_i64()[0];
    println!("{}", n+n.pow(2)+n.pow(3));
} 

fn get_input_i64() -> Vec<i64> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.parse().unwrap()).collect()
}
