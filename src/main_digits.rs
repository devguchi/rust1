use std::io;

fn main() {
    let words = get_input();
    let a_sum = digits_sum(&words[0]);
    let b_sum = digits_sum(&words[1]);
    if a_sum > b_sum {
        println!("{}", a_sum);
    } else {
        println!("{}", b_sum);
    }
}

fn digits_sum(s:&String) -> u32 {
    s.chars().map(|c| c.to_digit(10).unwrap()).sum()
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}

