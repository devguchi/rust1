use std::io;

fn main() {
    let ab = get_input_i64();
    let cd = get_input_i64();
    // let a = ab[0];
    let b = ab[1];
    let c = cd[0];
    // let d = cd[1];
    println!("{}", b-c);
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


