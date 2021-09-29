use std::io;

fn main() {
    let n = get_input_i64()[0];
    let mut day = 0;
    let mut total = 0;
    loop {
        day += 1;
        total += day;
        if total >= n {
            println!("{}", day);
            std::process::exit(0);
        }
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

