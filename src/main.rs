use std::io;

fn main() {
    let nab = get_input_i64();
    let n = nab[0];
    let a = nab[1];
    let b = nab[2];
    println!("{}", n-a+b);
}

fn get_input_i64() -> Vec<i64> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.parse().unwrap()).collect()
}

