use std::io;

fn main() {
    let n:i64 = get_input_i64()[0];
    let mut ans = 8;
    if 1 <= n && n <= 125 {
        ans = 4;
    } else if 126 <= n && n <= 211 {
        ans = 6;
    }
    println!("{}", ans);
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words:Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}

fn get_input_i64() -> Vec<i64> {
    let words = get_input();
    words.iter().map(|word| word.parse().unwrap()).collect()
}
