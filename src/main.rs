use std::io;
use std::collections::HashSet;

fn main() {
    let n:u32 = get_input()[0].parse().unwrap();
    let words = get_input_lines(n);
    let a_vec:Vec<u32> = words.iter()
        .map(|v| v.parse().unwrap()).collect();
    let a: HashSet<u32> = a_vec.into_iter().collect();
    println!("{}", a.len());
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


