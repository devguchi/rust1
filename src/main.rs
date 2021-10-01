use std::io;

fn main() {
    let ab = get_input_i64();
    let cd = get_input_i64();
    let ans = ab[0]*cd[1]-ab[1]*cd[0];
    println!("{}", ans);
}

fn get_input_i64() -> Vec<i64> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.parse().unwrap()).collect()
}

