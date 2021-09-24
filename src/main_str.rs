use std::io;
use std::collections::HashSet;

fn main() {
    let words = get_input();
    let s:Vec<char> = words[0].chars().collect();
    let unique: HashSet<char> = s.into_iter().collect();
    let ans = if unique.len() == 1 {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}

