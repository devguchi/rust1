use std::io;

fn main() {
    let numbers = get_input_i64();
    let n = numbers[0] as f64;
    let price = (1.08*n) as i64;
    if price < 206 {
        println!("Yay!");
    } else if price == 206 {
        println!("so-so");
    } else {
        println!(":(");
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

