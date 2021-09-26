use std::io;
use std::collections::HashSet;

fn main() {
    let s = &get_input()[0];
    let v = string_vec_u32(&s);
    let h:HashSet<u32> = v.clone().into_iter().collect();
    if h.len() < 2 {
        println!("Weak");
        std::process::exit(0);
    }
    let next: [u32; 10] = [1,2,3,4,5,6,7,8,9,0];
    let mut prev: usize = 100;
    for i in v.iter() {
        if prev < 10 && *i != next[prev] {
            println!("Strong");
            std::process::exit(0);
        }
        prev = *i as usize;
    }
    println!("Weak");
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}

fn string_vec_u32(s:&String) -> Vec<u32> {
    s.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

