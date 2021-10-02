use std::io;

fn main() {
    let x = get_input()[0].parse().unwrap();
    println!("{}", if x >= 0 { x } else { 0 });
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}


