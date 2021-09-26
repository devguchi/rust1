use std::io;
use std::collections::HashSet;

fn main() {
    let w = get_input_lines(4);
    let h:HashSet<String> = w.into_iter().collect();
    if h.len() == 4 {
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

fn get_input_lines(line_len:u32) -> Vec<String> {
    let mut vec:Vec<String> = vec![];
    let mut input:Vec<String>;
    for _ in 0..line_len {
        input = get_input();
        vec.append(&mut input);
    }
    vec
}
