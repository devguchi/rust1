use std::io;

fn main() {
    let _ = get_input_i64()[0];
    let s = &get_input()[0];
    let vc:Vec<char> = s.chars().collect();
    let idx = vc.iter().position(|&x| x == '1').unwrap();
    let mut loser = "Takahashi";
    if idx % 2 == 1 {
        loser = "Aoki";
    }
    println!("{}", loser);
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

