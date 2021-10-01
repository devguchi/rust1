use std::io;

fn main() {
    let s = &get_input()[0];
    let sv:Vec<char> = s.chars().rev().collect();
    for i in sv.iter() {
        if *i == '6' {
            print!("9");
        } else if *i == '9' {
            print!("6");
        } else {
            print!("{}", i);
        }
    }
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}

