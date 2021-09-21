use std::io;

fn main() {
    let numbers:Vec<i64> = get_input_i64();
    let n = numbers[0];
    let a = numbers[1];
    let x = numbers[2];
    let y = numbers[3];
    let mut _ans = 0;
    if n > a {
        _ans = x*a+y*(n-a);
    } else {
        _ans = x*n;
    }
    println!("{}", _ans);
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
