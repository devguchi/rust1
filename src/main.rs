use std::io;

fn main() {
    let n = get_input_i64()[0] as usize;
    let a = get_input_i64();
    let mut total = 0;
    for i in 0..n {
        if a[i] > 10 {
            total += a[i] - 10;
        }
    }
    println!("{}", total);
}

fn get_input_i64() -> Vec<i64> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.parse().unwrap()).collect()
}

