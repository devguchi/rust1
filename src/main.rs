use std::io;

fn main() {
    let mut words = get_input();
    let a: i32 = words[0].parse().unwrap();
    words = get_input();
    let b: i32 = words[0].parse().unwrap();
    let c: i32 = words[1].parse().unwrap();
    words = get_input();
    let s = &words[0];
    let sum = a + b + c;
    println!("{} {}", sum, s);
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}


