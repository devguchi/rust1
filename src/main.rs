use std::io;

fn main() {
    let ab = get_input_i64();
    let a = ab[0];
    let b = ab[1];
    let ab = a+b;
    if ab >= 15 && b >= 8 {
        println!("1");
    } else if ab >= 10 && b >= 3 {
        println!("2");
    } else if ab >= 3 {
        println!("3");
    } else {
        println!("4");
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


