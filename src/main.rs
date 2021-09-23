use std::io;

fn main() {
    let mut numbers = get_input_i64();
    numbers.sort_by(|a,b| b.cmp(a));
    let a = numbers[0];
    let b = numbers[1];
    println!("{}", a+b);

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

