use std::io;

fn main() {
    let words = get_input();
    let s:Vec<char> = words[0].chars().collect();
    println!("{}{}{}", s[1], s[2], s[0]);
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}

