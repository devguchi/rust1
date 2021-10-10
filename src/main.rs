use std::io;

fn main() {
    let x = get_input_i64()[0];
    if x >= 30 {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn get_input_i64() -> Vec<i64> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.parse().unwrap()).collect()
}
