use std::io;

fn main() {
    let n:i64 = get_input()[0].parse().unwrap();
    println!("{}", if n%2 == 1 { "Black" } else { "White" });
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}


