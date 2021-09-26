use std::io;
use std::collections::HashSet;

fn main() {
    let n = get_input_i64()[0] as usize;
    let mut v:Vec<String> = vec![];
    for _ in 0..n {
        let words = get_input();
        let name = words[0].clone()+"+"+&words[1];
        v.push(name);
    }
    let h:HashSet<String> = v.into_iter().collect();
    if h.len() < n {
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

fn get_input_i64() -> Vec<i64> {
    let words = get_input();
    words.iter().map(|word| word.parse().unwrap()).collect()
}
